//! 单电阻采样重建与可测量 PWM。时间单位为 96 MHz ATIM 计数。
//! 边沿对齐 PWM1：两桥上管导通时 Idc=-I首降相，一桥上管时 Idc=I末降相。
use crate::arithmetic::Arithmetic;
use crate::config::{BUS_MAX_MV, CPU_HZ, MODEL_PHASE_L_UH, MODEL_PHASE_R_MILLIOHM, PWM_TICKS};

pub const DEAD_TICKS: u16 = 96; // 外加 1 us；不能代替示波器测量驱动传播与 MOS 管关断。
pub const BLANK_TICKS: u16 = 240; // 开关沿后等待 2.5 us，包含死区和模拟建立预算。
pub const ACQUISITION_TICKS: u16 = 140; // 70 个 ADC 时钟结束时的保持时刻。
pub const CONVERSION_TICKS: u16 = 170; // 70+15 个 ADC 时钟，ADC=PCLK/2。
pub const WINDOW_TICKS: u16 = 960; // 每个有效矢量至少 10 us；两次 EOS 间留出完整首样本 ISR。
pub const DUTY_MIN: u16 = PWM_TICKS / 10;
pub const DUTY_MAX: u16 = PWM_TICKS / 2; // 预留第二次采样后的控制计算时间，不追求满调制度。
pub const UPDATE_DEADLINE: u16 = 700;
pub const CONTROL_DEADLINE: u16 = PWM_TICKS - 600;
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
            duty: [
                DUTY_MIN,
                DUTY_MIN + WINDOW_TICKS,
                DUTY_MIN + 2 * WINDOW_TICKS,
            ],
            sample: [
                DUTY_MIN + BLANK_TICKS,
                DUTY_MIN + WINDOW_TICKS + BLANK_TICKS,
            ],
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
    pub fn reconstruct_timed(
        &self,
        dc_ma: [i32; 2],
        bus_mv: i32,
        emf_ab_mv: [i32; 2],
        math: &mut impl Arithmetic,
    ) -> [i32; 3] {
        let mut phase = self.reconstruct(dc_ma);
        let dt = self.hold_tick(1) - self.hold_tick(0);
        let first = self.order[0];
        let [ea, eb] = emf_ab_mv;
        let e_b = (-ea + eb + ((eb * 23_988) >> 15)) / 2;
        let emf = [ea, e_b, -ea - e_b][first];
        let decay = decay_q15(dt);
        // 首降相在中间桥臂下降前为 -2Vbus/3，之后为 -Vbus/3。
        let tail_decay = decay_q15(self.hold_tick(1) - self.duty[self.order[1]]);
        let voltage_q15 =
            -math.div(bus_mv * (32768 + tail_decay - 2 * decay), 3) - emf * (32768 - decay);
        // 先除 R 再乘 1000；配置 R>=1Ω、Vbus<=16V、|Idc|<=2A 保证 i32 范围。
        let response_ma_q15 = math.div(voltage_q15, MODEL_PHASE_R_MILLIOHM) * 1000;
        phase[first] = (decay * phase[first] + response_ma_q15) / 32768;
        phase[self.order[1]] = -phase[first] - phase[self.order[2]];
        phase
    }
    /// 两个第二保持时刻间的 RL 电压响应，包含前帧尾部与本帧头部。
    /// 每个导通区间 [s,e] 的权重为 exp(-R(T-e)/L)-exp(-R(T-s)/L)。
    /// Off/Bootstrap 传全零占空比；两帧分别使用对应实测 Vbus。
    pub fn interval_rl(
        &self,
        previous: &Frame,
        previous_duty: [u16; 3],
        duty: [u16; 3],
        previous_bus_mv: i32,
        bus_mv: i32,
        math: &mut impl Arithmetic,
    ) -> RlInterval {
        let old_t = previous.hold_tick(1);
        let now_t = self.hold_tick(1);
        let dt_ticks = PWM_TICKS - old_t + now_t;
        let decay = decay_q15(dt_ticks);
        let now_decay = decay_q15(now_t);
        let mut pole_q15 = [0i32; 3];
        for phase in 0..3 {
            let old_high = previous_duty[phase].saturating_sub(old_t);
            let new_high = duty[phase].min(now_t);
            pole_q15[phase] = previous_bus_mv * (decay_q15(dt_ticks - old_high) - decay)
                + bus_mv * (decay_q15(now_t - new_high) - now_decay);
        }
        let [a, b, c] = pole_q15;
        RlInterval {
            dt_ticks,
            decay_q15: decay as u16,
            voltage_response_mv: [
                math.div(2 * a - b - c, 3 * 32768),
                (((b - c) / 32768) * 18_919) >> 15,
            ],
        }
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
    pub fn plan(
        &mut self,
        voltage_mv: [i32; 2],
        bus_mv: i32,
        math: &mut impl Arithmetic,
    ) -> Option<Frame> {
        if !(1_000..=BUS_MAX_MV).contains(&bus_mv) {
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
            demand[i] = math.div(phase[i] * i32::from(PWM_TICKS), bus_mv) + self.error_ticks[i];
        }
        let original = demand;
        let span = demand.iter().max().unwrap() - demand.iter().min().unwrap();
        let available = i32::from(DUTY_MAX - DUTY_MIN);
        let saturated = span > available;
        if saturated {
            for x in &mut demand {
                *x = math.div(*x * available, span);
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
        // 公共模平移不改变相间电压，将两次采样提前以增加计算余量。
        let common = i32::from(DUTY_MIN) - low;
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
        let mean = math.div(duty.iter().map(|x| i32::from(*x)).sum::<i32>(), 3);
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

/// 指数加权 RL 电压响应；不是平均电压，禁止混入普通 PWM 占空比平均值。
#[derive(Clone, Copy, Debug)]
pub struct RlInterval {
    pub dt_ticks: u16,
    pub decay_q15: u16,
    pub voltage_response_mv: [i32; 2],
}

impl RlInterval {
    pub fn valid(&self) -> bool {
        (PWM_TICKS / 2..=PWM_TICKS + PWM_TICKS / 2).contains(&self.dt_ticks)
            && i32::from(self.decay_q15) == decay_q15(self.dt_ticks)
            && self.decay_q15 < 32768
            && self
                .voltage_response_mv
                .iter()
                .all(|v| (-BUS_MAX_MV..=BUS_MAX_MV).contains(v))
    }
}

/// 256 tick 网格的线性插值；所有表项从配置 R/L 在编译期计算。
/// 运行时只有查表、i32 乘法和移位，无浮点、幂函数或宽整数除法。
pub fn decay_q15(ticks: u16) -> i32 {
    let index = usize::from(ticks >> 8);
    let fraction = i32::from(ticks & 255);
    let a = i32::from(RL_DECAY_Q15[index]);
    let b = i32::from(RL_DECAY_Q15[index + 1]);
    a - (((a - b) * fraction + 128) >> 8)
}

const RL_DECAY_Q15: [u16; 257] = build_decay_table();
const fn build_decay_table() -> [u16; 257] {
    // 只在编译期使用 i128。先算 exp(-x/16)，再平方四次。
    // 配置范围内 x/16<=1.71；24 项 Q48 Taylor 的误差远小于 Q15 量化。
    assert!(MODEL_PHASE_R_MILLIOHM >= 1000 && MODEL_PHASE_R_MILLIOHM <= 4000);
    assert!(MODEL_PHASE_L_UH >= 100 && MODEL_PHASE_L_UH <= 5000);
    assert!(CPU_HZ == 96_000_000);
    const ONE: i128 = 1i128 << 48;
    let mut table = [0u16; 257];
    let mut index = 0usize;
    while index < table.len() {
        let x = index as i128 * 256 * MODEL_PHASE_R_MILLIOHM as i128 * ONE
            / ((CPU_HZ / 1000) as i128 * MODEL_PHASE_L_UH as i128 * 16);
        let mut term = ONE;
        let mut sum = ONE;
        let mut order = 1i128;
        while order <= 24 {
            term = -term * x / ONE / order;
            sum += term;
            order += 1;
        }
        let mut square = 0;
        while square < 4 {
            sum = (sum * sum + ONE / 2) / ONE;
            square += 1;
        }
        table[index] = ((sum * 32768 + ONE / 2) / ONE) as u16;
        index += 1;
    }
    table
}
