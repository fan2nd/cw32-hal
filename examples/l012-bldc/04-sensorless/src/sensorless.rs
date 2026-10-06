#![forbid(unsafe_code)]
//! 阶段 04 增加经过判定确认的被动反电动势（BEMF）阈值观测。
//! 它绝不执行换相、启动电机、使输出就绪或调度定时器。
//! 退磁消隐和换相延迟由主动驱动阶段负责。

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Edge {
    Rising,
    Falling,
}

/// 原 C 代码使用 `TAB_RFling[1]` 和 ADC 通道 `{3,2,1,3,2,1}`。
pub const EXPECTED_EDGE: [Edge; 6] = [
    Edge::Falling,
    Edge::Rising,
    Edge::Falling,
    Edge::Rising,
    Edge::Falling,
    Edge::Rising,
];
pub const BEMF_CHANNEL: [usize; 6] = [3, 2, 1, 3, 2, 1];

/// 用于 ADC/过零检测教学阶段的独立被动检测器。
/// 它绝不驱动桥臂或启动定时器。应在相应的退磁间隔结束后调用
/// `begin_sector`，随后输入与 PWM 同步的 ADC 样本。
/// 首次满足确认条件的过零事件仅报告一次，直到下一个扇区开始。
pub struct ZeroCrossingDetector {
    sector: u8,
    required: u8,
    consecutive: u8,
    armed: bool,
}

impl Default for ZeroCrossingDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroCrossingDetector {
    pub const fn new() -> Self {
        Self {
            sector: 0,
            required: 2,
            consecutive: 0,
            armed: false,
        }
    }

    /// 扇区无效或确认样本数为零时，禁用检测器。
    pub fn begin_sector(&mut self, sector: u8, required_samples: u8) -> bool {
        self.consecutive = 0;
        self.armed = sector < 6 && required_samples > 0;
        if self.armed {
            self.sector = sector;
            self.required = required_samples;
        }
        self.armed
    }

    pub fn observe(&mut self, sample: [u16; 4], bus_adc: u16) -> bool {
        if !self.armed {
            return false;
        }
        if !crossing_side_matches(self.sector, sample, bus_adc) {
            self.consecutive = 0;
            return false;
        }
        self.consecutive += 1;
        if self.consecutive < self.required {
            return false;
        }
        self.armed = false;
        true
    }
}

fn crossing_side_matches(sector: u8, sample: [u16; 4], bus_adc: u16) -> bool {
    let floating = sample[BEMF_CHANNEL[sector as usize]];
    let threshold = bus_adc >> 1;
    match EXPECTED_EDGE[sector as usize] {
        Edge::Rising => floating > threshold,
        Edge::Falling => floating < threshold,
    }
}
