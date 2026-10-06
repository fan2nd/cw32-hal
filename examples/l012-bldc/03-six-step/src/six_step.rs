#![forbid(unsafe_code)]
//! 阶段 03 增加纯数据形式的六步桥臂状态映像，以及单次按键切换扇区功能。
//! 这些数值仅用于观测，此处不包含任何硬件输出功能。

pub const PWM_PERIOD: u16 = 4800;
pub const DEMONSTRATION_DUTY: u16 = PWM_PERIOD / 20;
pub const KEY_DEBOUNCE_MS: u8 = 60;

/// 沿用原始的 4800 计数 PWM 尺度（96 MHz / 4800 = 20 kHz）。
/// 阶段 03/04 仅展示此状态映像，main 不操作栅极引脚。
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bridge {
    pub pwm_counts: [u16; 3],
    pub low_sides: [bool; 3],
    pub sample_compare: u16,
}
impl Bridge {
    pub const fn off() -> Self {
        Self {
            pwm_counts: [0; 3],
            low_sides: [false; 3],
            sample_compare: PWM_PERIOD - 800,
        }
    }

    /// A+B-、A+C-、B+C-、B+A-、C+A-、C+B-。无效扇区返回全部关闭的安全状态。
    pub const fn commutation(sector: u8, duty: u16) -> Self {
        let (high, low) = match sector {
            0 => (0, 1),
            1 => (0, 2),
            2 => (1, 2),
            3 => (1, 0),
            4 => (2, 0),
            5 => (2, 1),
            _ => return Self::off(),
        };
        let mut result = Self {
            pwm_counts: [0; 3],
            low_sides: [false; 3],
            sample_compare: 300,
        };
        result.pwm_counts[high] = duty;
        result.low_sides[low] = true;
        result
    }
}
