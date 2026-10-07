#![deny(unsafe_code)]
//! Blend 起始80帧的观察器重放输入诊断。
//! 复用原 20 ms 缓冲；不打印、不分配、不改变控制/资格。
//! Blend是控制状态，不证明观察器角度等于真实转子角度。

pub const MODEL_TAG: &str = "5a62df7d63f1a6208d002c6b10722d380f653242baf152db8955422a84313c9f";
pub const RECORDS: usize = 80;
pub const START_AGE: u16 = 1;

/// 首帧处理前、末帧控制处理后的完整观察状态。Blend中保留递推初末状态。
/// 保留原布局及 Q8/Q16 小数位，供验证运输用的上一 EMF 和未更新状态。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReplayState {
    pub previous_current: [i32; 2],
    pub emf_q8: [i32; 2],
    pub raw_emf_mv: [i32; 2],
    pub phase: u32,
    pub speed: i32,
    pub magnitude: i32,
    pub error: i32,
    pub open_phase: u32,
    pub corrected_angle: u16,
    pub tracking: u16,
}
impl ReplayState {
    pub const EMPTY: Self = Self {
        previous_current: [0; 2],
        emf_q8: [0; 2],
        raw_emf_mv: [0; 2],
        phase: 0,
        speed: 0,
        magnitude: 0,
        error: 0,
        open_phase: 0,
        corrected_angle: 0,
        tracking: 0,
    };
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Seed {
    pub state: ReplayState,
    pub previous_duty: [u16; 3],
    pub offset: i16,
    pub previous_bus_mv: u16,
    pub age: u16,
}
impl Seed {
    const EMPTY: Self = Self {
        state: ReplayState::EMPTY,
        previous_duty: [0; 3],
        offset: 0,
        previous_bus_mv: 0,
        age: 0,
    };
}

/// duty 是本帧已生效的 CCR。vdda 是电流换算前的旧值，bus 是 ADC2 更新后的值。
/// 时间/order 从严格分离的 duty 与既有采样常数还原；每项必须来自相邻物理帧。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Record {
    pub raw: [u16; 2],
    pub duty: [u16; 3],
    pub vdda_mv: u16,
    pub bus_mv: u16,
    pub forced_angle: u16,
    pub observer_angle: u16,
    pub pll_error: i16,
    pub first_phase_ma: i16,
}
impl Record {
    const EMPTY: Self = Self {
        raw: [0; 2],
        duty: [0; 3],
        vdda_mv: 0,
        bus_mv: 0,
        forced_angle: 0,
        observer_angle: 0,
        pll_error: 0,
        first_phase_ma: 0,
    };
}

pub struct Trace {
    pub seed: Seed,
    pub terminal: ReplayState,
    records: [Record; RECORDS],
    pub len: u16,
    pub complete: bool,
    started: bool,
}
impl Trace {
    pub const fn new() -> Self {
        Self {
            seed: Seed::EMPTY,
            terminal: ReplayState::EMPTY,
            records: [Record::EMPTY; RECORDS],
            len: 0,
            complete: false,
            started: false,
        }
    }
    /// 再次手动启动时仅清元数据，不清大缓冲。窗口外没有数据搬运。
    pub fn wants_input(&mut self, age: u16) -> bool {
        if age < START_AGE {
            return false;
        }
        !self.complete
            && age >= START_AGE
            && age < START_AGE + RECORDS as u16
            && ((!self.started && age == START_AGE)
                || (self.started && age == self.seed.age + self.len))
    }
    /// 只清小header；旧记录不可见，避免新启动过早故障被误配到上一启动。
    pub fn reset(&mut self) {
        self.len = 0;
        self.complete = false;
        self.started = false;
        self.seed.age = 0;
    }
    pub fn needs_seed(&self) -> bool {
        !self.started
    }
    pub fn begin(&mut self, seed: Seed) {
        self.seed = seed;
        self.len = 0;
        self.complete = false;
        self.started = true;
    }
    /// 只有完整且成功处理的输入帧才能提交；缺帧不补算、不拼接另一窗口。
    pub fn push(&mut self, age: u16, record: Record) -> bool {
        if !self.started
            || self.complete
            || usize::from(self.len) >= RECORDS
            || age != self.seed.age + self.len
        {
            return false;
        }
        self.records[usize::from(self.len)] = record;
        self.len += 1;
        usize::from(self.len) == RECORDS
    }
    pub fn finish(&mut self, state: ReplayState) {
        if usize::from(self.len) == RECORDS {
            self.terminal = state;
            self.complete = true;
        }
    }
    pub fn record(&self, index: usize) -> Option<Record> {
        if index < usize::from(self.len) {
            Some(self.records[index])
        } else {
            None
        }
    }
}
const _: () = assert!(crate::config::STARTUP_TIMEOUT_FRAMES > START_AGE as u32 + RECORDS as u32);
const _: () = assert!(crate::config::STARTUP_TIMEOUT_FRAMES <= u16::MAX as u32);
const _: () = assert!(core::mem::size_of::<Record>() == 22);
const _: () = assert!(core::mem::size_of::<ReplayState>() == 48);
const _: () = assert!(core::mem::size_of::<Trace>() <= 1920);
