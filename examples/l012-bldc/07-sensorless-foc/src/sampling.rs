//! 单电阻采样重建与可测量 PWM。时间单位为 96 MHz ATIM 计数。
//! 边沿对齐 PWM1：两桥上管导通时 Idc=-I首降相，一桥上管时 Idc=I末降相。
use crate::config::{CPU_HZ, MODEL_PHASE_L_UH, MODEL_PHASE_R_MILLIOHM, PWM_TICKS};

pub const DEAD_TICKS: u16 = 96; // 外加 1 us；不能代替示波器测量驱动传播与 MOS 管关断。
pub const BLANK_TICKS: u16 = 240; // 开关沿后等待 2.5 us，包含死区和模拟建立预算。
pub const ACQUISITION_TICKS: u16 = 140; // 70 个 ADC 时钟结束时的保持时刻。
pub const CONVERSION_TICKS: u16 = 170; // 70+15 个 ADC 时钟，ADC=PCLK/2。
pub const WINDOW_TICKS: u16 = 576; // 每个有效矢量至少 6 us。
pub const DUTY_MIN: u16 = 960;
pub const DUTY_MAX: u16 = 4800; // 预留第二次采样后的控制计算时间，不追求满调制度。
pub const UPDATE_DEADLINE: u16 = 700;
pub const CONTROL_DEADLINE: u16 = 9000;
pub const SAMPLE_IRQ_SLACK: u16 = 128; // 到达过迟即停机，绝不猜测 RESULT0 属于哪一帧。

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub duty: [u16; 3],
    pub sample: [u16; 2],
    pub order: [usize; 3],
}
impl Frame {
    pub const fn initial() -> Self {
        Self {
            duty: [2304, 2880, 3456],
            sample: [2544, 3120],
            order: [0, 1, 2],
        }
    }
    pub fn valid(&self) -> bool {
        let [a, b, c] = self.order;
        a < 3
            && b < 3
            && c < 3
            && a != b
            && b != c
            && a != c
            && self
                .duty
                .iter()
                .all(|&d| (DUTY_MIN..=DUTY_MAX).contains(&d))
            && self.duty[b] >= self.duty[a] + WINDOW_TICKS
            && self.duty[c] >= self.duty[b] + WINDOW_TICKS
            && self.sample[0] == self.duty[a] + BLANK_TICKS
            && self.sample[1] == self.duty[b] + BLANK_TICKS
            && self.sample[0] + CONVERSION_TICKS + SAMPLE_IRQ_SLACK < self.duty[b]
            && self.sample[1] + CONVERSION_TICKS + SAMPLE_IRQ_SLACK < self.duty[c]
    }
    /// ADC 样本属于本帧的证明之一：独立检查第几次转换的硬件完成时间。
    pub fn accepts(&self, sample: usize, now: u16) -> bool {
        sample < 2
            && now >= self.sample[sample] + CONVERSION_TICKS
            && now <= self.sample[sample] + CONVERSION_TICKS + SAMPLE_IRQ_SLACK
    }
    /// 对原时序保护分类，不改变接受区间；先判跨帧，再判样本序号和早/晚。
    pub fn sample_fault_reason(&self, sample: usize, now: u16, reload_pending: bool) -> u32 {
        if reload_pending {
            1
        } else if sample >= 2 {
            2
        } else if now < self.sample[sample] + CONVERSION_TICKS {
            3
        } else if !self.accepts(sample, now) {
            4
        } else {
            0
        }
    }
    pub fn reconstruct(&self, dc_ma: [i32; 2]) -> [i32; 3] {
        let mut phase = [0; 3];
        phase[self.order[0]] = -dc_ma[0];
        phase[self.order[2]] = dc_ma[1];
        phase[self.order[1]] = dc_ma[0] - dc_ma[1];
        phase
    }
    pub fn hold_tick(&self, sample: usize) -> u16 {
        self.sample[sample] + ACQUISITION_TICKS
    }
    /// 将首降相从第一个保持时刻预测到第二个保持时刻，再利用 KCL。
    /// 使用已配置 R/L 和上一轮反电势估计；错误参数/死区/管压降仍会产生误差。
    pub fn reconstruct_timed(&self, dc_ma: [i32; 2], bus_mv: i32, emf_ab_mv: [i32; 2]) -> [i32; 3] {
        let mut phase = self.reconstruct(dc_ma);
        let start = i32::from(self.hold_tick(0));
        let end = i32::from(self.hold_tick(1));
        let dt = end - start;
        let high_ticks = self.duty.map(|d| (i32::from(d) - start).clamp(0, dt));
        let first = self.order[0];
        let sum = high_ticks.iter().sum::<i32>();
        let volt_ticks = (3 * high_ticks[first] - sum) * bus_mv / 3;
        let [ea, eb] = emf_ab_mv;
        let e_b = (-ea + eb + ((eb * 23_988) >> 15)) / 2;
        let emf = [ea, e_b, -ea - e_b][first];
        let resistance_mv = phase[first] * MODEL_PHASE_R_MILLIOHM / 1_000;
        let delta = (volt_ticks - (resistance_mv + emf) * dt)
            / ((CPU_HZ / 1_000_000) as i32 * MODEL_PHASE_L_UH);
        phase[first] += delta;
        phase[self.order[1]] = -phase[first] - phase[self.order[2]];
        phase
    }
    /// 第二保持时刻之间的理想 PWM 电压积分，包含前帧尾部与本帧头部。
    /// bus 可分别取两帧实测值。Off/Bootstrap 传全零占空比（相电压为零）。
    pub fn interval_voltage(
        &self,
        previous: &Frame,
        previous_duty: [u16; 3],
        duty: [u16; 3],
        previous_bus_mv: i32,
        bus_mv: i32,
    ) -> ([i32; 2], u16) {
        let old_t = previous.hold_tick(1);
        let now_t = self.hold_tick(1);
        let dt = PWM_TICKS - old_t + now_t;
        let mut volt_ticks = [0i32; 3];
        for phase in 0..3 {
            let old_high = previous_duty[phase].saturating_sub(old_t);
            let new_high = duty[phase].min(now_t);
            volt_ticks[phase] =
                i32::from(old_high) * previous_bus_mv + i32::from(new_high) * bus_mv;
        }
        let [a, b, c] = volt_ticks;
        (
            [
                (2 * a - b - c) / (3 * i32::from(dt)),
                (((b - c) / i32::from(dt)) * 18_919) >> 15,
            ],
            dt,
        )
    }
}

pub struct Modulator {
    // 缺少可测窗口时，扩展两个有效矢量；把净电压误差带到下一帧。
    // 零请求下交替正负测试矢量，而非长期注入一个固定方向电压。
    error_ticks: [i32; 3],
}
impl Modulator {
    pub const fn new() -> Self {
        Self {
            error_ticks: [0; 3],
        }
    }
    pub fn reset(&mut self) {
        self.error_ticks = [0; 3];
    }
    pub fn plan(&mut self, voltage_mv: [i32; 2], bus_mv: i32) -> Option<Frame> {
        if !(1_000..=55_000).contains(&bus_mv) {
            return None;
        }
        let [alpha, beta] = voltage_mv;
        if alpha.unsigned_abs() > bus_mv as u32 || beta.unsigned_abs() > bus_mv as u32 {
            return None;
        }
        let b = (-alpha + beta + ((beta * 23_988) >> 15)) / 2;
        let phase = [alpha, b, -alpha - b];
        let mut demand = [0; 3];
        for i in 0..3 {
            demand[i] = phase[i] * i32::from(PWM_TICKS) / bus_mv + self.error_ticks[i];
        }
        let original = demand;
        let span = demand.iter().max().unwrap() - demand.iter().min().unwrap();
        let available = i32::from(DUTY_MAX - DUTY_MIN);
        let saturated = span > available;
        if saturated {
            for x in &mut demand {
                *x = *x * available / span;
            }
        }
        let mut order = [0, 1, 2];
        // 三元素插入排序，固定比较次数，无分配。
        if demand[order[0]] > demand[order[1]] {
            order.swap(0, 1);
        }
        if demand[order[1]] > demand[order[2]] {
            order.swap(1, 2);
        }
        if demand[order[0]] > demand[order[1]] {
            order.swap(0, 1);
        }
        let [a, b, c] = order;
        let w = i32::from(WINDOW_TICKS);
        let mut low = demand[a].min(demand[b] - w);
        let high = demand[c].max(demand[b] + w);
        low = low.max(high - available);
        let mid = demand[b].clamp(low + w, high - w);
        let common = (i32::from(DUTY_MIN + DUTY_MAX) - low - high) / 2;
        let mut duty = [0; 3];
        duty[a] = (low + common) as u16;
        duty[b] = (mid + common) as u16;
        duty[c] = (high + common) as u16;
        let frame = Frame {
            duty,
            sample: [duty[a] + BLANK_TICKS, duty[b] + BLANK_TICKS],
            order,
        };
        if !frame.valid() {
            return None;
        }
        let mean = duty.iter().map(|x| i32::from(*x)).sum::<i32>() / 3;
        for i in 0..3 {
            self.error_ticks[i] = if saturated {
                0
            } else {
                (original[i] - (i32::from(duty[i]) - mean)).clamp(-2 * w, 2 * w)
            };
        }
        Some(frame)
    }
}
