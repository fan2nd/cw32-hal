//! 中心对齐对称七段SVPWM：零矢量均分，不注入测量脉冲，每控制帧采一次。
use crate::arithmetic::Arithmetic;
use crate::config::{BUS_MAX_MV, CONTROL_TICKS, PWM_HALF_TICKS, PWM_TICKS};
pub const DEAD_TICKS: u16 = 96;
pub const BLANK_TICKS: u16 = 240; // 沿用已用的2.5us消隐，不缩短ADC采样时间。
pub const CONVERSION_TICKS: u16 = 170; // 70+15个ADC时钟，48MHz。
                                       // 转换结果由ADC锁存；CPU不必在有效矢量结束前读出，但必须在下一转换前处理。
                                       // 模拟窗口保持原128 tick额外裕量，独立于允许的中断/读寄存器延迟。
pub const WINDOW_GUARD_TICKS: u16 = 128;
pub const SAMPLE_IRQ_SLACK: u16 = 256; // 2.67us；已测首样本读取响应131 tick。
pub const DUTY_MIN: u16 = PWM_HALF_TICKS * 5 / 100;
pub const DUTY_MAX: u16 = PWM_HALF_TICKS - DUTY_MIN;
pub const UPDATE_DEADLINE: u16 = 700;
// 必须在触发重新连接之后启动ADC，而不只是EOS晚于更新IRQ。
pub const EARLIEST_SAMPLE: u16 = UPDATE_DEADLINE + 100;
pub const CONTROL_DEADLINE: u16 = CONTROL_TICKS - 600;
// sample统一表示距底部UEV的时刻，CCR4=sample，上升半周PWM2上升沿触发。
// 无有效窗口时在接近顶部的000零矢量采偏置，不当作绕组电流。
pub const FALLBACK_SAMPLE: u16 = 2100;

#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub duty: [u16; 3],
    pub sample: u16,
    pub order: [usize; 3],
    pub current_valid: bool,
}
impl Frame {
    pub const fn initial() -> Self {
        Self {
            duty: [PWM_HALF_TICKS / 2; 3],
            sample: FALLBACK_SAMPLE,
            order: [0, 1, 2],
            current_valid: false,
        }
    }
    pub fn valid(&self) -> bool {
        let [a, b, c] = self.order;
        if a >= 3 || b >= 3 || c >= 3 || a == b || a == c || b == c {
            return false;
        }
        if !self.duty.iter().all(|d| (DUTY_MIN..=DUTY_MAX).contains(d))
            || self.duty[a] > self.duty[b]
            || self.duty[b] > self.duty[c]
            || self.sample >= PWM_HALF_TICKS
            || self.sample < EARLIEST_SAMPLE
            || self.sample + CONVERSION_TICKS + SAMPLE_IRQ_SLACK >= PWM_TICKS
            || (self.duty[a] + self.duty[c]).abs_diff(PWM_HALF_TICKS) > 1
        {
            return false;
        }
        if !self.current_valid {
            return self.sample == FALLBACK_SAMPLE
                && self.sample >= self.duty[c] + BLANK_TICKS
                && self.sample + CONVERSION_TICKS + WINDOW_GUARD_TICKS < PWM_TICKS - self.duty[c];
        }
        [(a, b), (b, c)].iter().any(|&(lo, hi)| {
            self.sample == (self.duty[lo] + BLANK_TICKS).max(EARLIEST_SAMPLE)
                && self.sample + CONVERSION_TICKS + WINDOW_GUARD_TICKS < self.duty[hi]
        })
    }
    pub fn accepts(&self, sample: usize, now: u16) -> bool {
        sample == 0
            && now >= self.sample + CONVERSION_TICKS
            && now <= self.sample + CONVERSION_TICKS + SAMPLE_IRQ_SLACK
    }
    pub fn sample_fault_reason(&self, sample: usize, now: u16, reload_pending: bool) -> u32 {
        if reload_pending {
            1
        } else if sample != 0 {
            2
        } else if now < self.sample + CONVERSION_TICKS {
            3
        } else if !self.accepts(sample, now) {
            4
        } else {
            0
        }
    }
}

// 无跨帧误差累积：每次电压请求独立决定占空比，零请求严格等占空比。
pub struct Modulator;
impl Modulator {
    pub const fn new() -> Self {
        Self
    }
    pub fn plan(
        &mut self,
        voltage_mv: [i32; 2],
        bus_mv: i32,
        math: &mut impl Arithmetic,
    ) -> Option<Frame> {
        if !(1000..=BUS_MAX_MV).contains(&bus_mv) {
            return None;
        }
        let [alpha, beta] = voltage_mv;
        if alpha.unsigned_abs() > bus_mv as u32 || beta.unsigned_abs() > bus_mv as u32 {
            return None;
        }
        let b = (-alpha + beta + ((beta * 23988) >> 15)) / 2;
        let phase = [alpha, b, -alpha - b];
        let mut demand = phase.map(|v| math.div(v * PWM_HALF_TICKS as i32, bus_mv));
        let span = *demand.iter().max().unwrap() - *demand.iter().min().unwrap();
        let available = (DUTY_MAX - DUTY_MIN) as i32;
        if span > available {
            for value in &mut demand {
                *value = math.div(*value * available, span);
            }
        }
        let mut order = [0, 1, 2];
        if demand[order[0]] > demand[order[1]] {
            order.swap(0, 1);
        }
        if demand[order[1]] > demand[order[2]] {
            order.swap(1, 2);
        }
        if demand[order[0]] > demand[order[1]] {
            order.swap(0, 1);
        }
        // 中点注入：min(duty)+max(duty)=ARR，两个零矢量总时长相等。
        // 以底部为周期起点：111,V1,V2,000,V2,V1,111，镜像七段。
        let common = (PWM_HALF_TICKS as i32 - demand[order[0]] - demand[order[2]]) / 2;
        let duty = demand.map(|v| (v + common) as u16);
        let [a, b, c] = order;
        let (lo, hi) = if duty[b] - duty[a] >= duty[c] - duty[b] {
            (a, b)
        } else {
            (b, c)
        };
        // 扩大占空比后最早下降沿前移；触发本身须晚于更新/连接截止。
        let trigger = (duty[lo] + BLANK_TICKS).max(EARLIEST_SAMPLE);
        let current_valid = trigger + CONVERSION_TICKS + WINDOW_GUARD_TICKS < duty[hi];
        let f = Frame {
            duty,
            order,
            sample: if current_valid {
                trigger
            } else {
                FALLBACK_SAMPLE
            },
            current_valid,
        };
        f.valid().then_some(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn irq_latency_does_not_enlarge_analog_sampling_window() {
        let mut f = Frame {
            duty: [661, 1200, 1739],
            sample: 1440,
            order: [0, 1, 2],
            current_valid: true,
        };
        assert!(f.valid());
        // 已观察到的131 tick读取响应仍在转换结果锁存期间。
        assert!(f.accepts(0, 1440 + CONVERSION_TICKS + 131));
        assert!(!f.accepts(0, 1440 + CONVERSION_TICKS + SAMPLE_IRQ_SLACK + 1));
        assert!(!f.accepts(1, 1440 + CONVERSION_TICKS));
        f.duty = [662, 1200, 1738];
        assert!(!f.valid()); // 不能因IRQ预算增加而放宽模拟窗口。
        let off = Frame::initial();
        assert!(off.valid());
        assert!(FALLBACK_SAMPLE + CONVERSION_TICKS + SAMPLE_IRQ_SLACK < PWM_TICKS);
        // 停机切换实测更新路径可能超过600 tick。EOS晚于700仍不够，
        // 必须保证触发本身发生在ADC重新接通之后，不能错到下一载波。
        let mut early = Frame {
            duty: [120, 1200, 2280],
            sample: 600,
            order: [0, 1, 2],
            current_valid: true,
        };
        assert!(!early.valid());
        early.sample = EARLIEST_SAMPLE;
        assert!(early.valid());
    }
}
