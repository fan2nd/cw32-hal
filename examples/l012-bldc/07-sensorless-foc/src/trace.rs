#![deny(unsafe_code)]
//! 有界停机诊断：每 8 帧记录一次平台数据，不改变任何控制量或资格。
//! 64 × 2 ms = 128 ms 历史；16 个角度箱和 6 个采样顺序箱覆盖整次平台。

pub const RECORDS: usize = 64;
pub const PHASE_BINS: usize = 16;
pub const SECTOR_BINS: usize = 6;
pub const DECIMATION: u16 = 8;

#[derive(Clone, Copy)]
pub struct Observation {
    pub age: u16,
    pub forced: u16,
    pub observer: u16,
    pub pll_error: i16,
    pub emf: [i16; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Record {
    pub age: u16,
    pub forced: u16,
    pub observer: u16,
    pub pll_error: i16,
    pub raw: [u16; 2],
    pub duty: [u16; 3],
    pub transport_ma: i16,
    pub emf: [i16; 2],
}
impl Record {
    const EMPTY: Self = Self {
        age: 0,
        forced: 0,
        observer: 0,
        pll_error: 0,
        raw: [0; 2],
        duty: [0; 3],
        transport_ma: 0,
        emf: [0; 2],
    };
    pub fn delta(&self) -> i16 {
        self.observer.wrapping_sub(self.forced) as i16
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bin {
    pub delta_sum: i32,
    pub count: u16,
    pub delta_min: i16,
    pub delta_max: i16,
    pub transport_peak_ma: u16,
}
impl Bin {
    const EMPTY: Self = Self {
        delta_sum: 0,
        count: 0,
        delta_min: i16::MAX,
        delta_max: i16::MIN,
        transport_peak_ma: 0,
    };
    fn push(&mut self, delta: i16, transport_ma: i16) {
        // 单次平台最多 15000 个抽样点；i16 角差累计仍在 i32 内。
        self.count += 1;
        self.delta_sum += i32::from(delta);
        self.delta_min = self.delta_min.min(delta);
        self.delta_max = self.delta_max.max(delta);
        self.transport_peak_ma = self.transport_peak_ma.max(transport_ma.unsigned_abs());
    }
}

pub struct Trace {
    pub phase: [Bin; PHASE_BINS],
    pub sector: [Bin; SECTOR_BINS],
    records: [Record; RECORDS],
    pub total: u16,
    pub last_age: u16,
    next: u8,
    pub len: u8,
}
impl Trace {
    pub const fn new() -> Self {
        Self {
            phase: [Bin::EMPTY; PHASE_BINS],
            sector: [Bin::EMPTY; SECTOR_BINS],
            records: [Record::EMPTY; RECORDS],
            total: 0,
            last_age: 0,
            next: 0,
            len: 0,
        }
    }
    pub fn push(
        &mut self,
        o: Observation,
        raw: [u16; 2],
        duty: [u16; 3],
        order: [usize; 3],
        transport_ma: i32,
    ) {
        // 新启动的 age 回退时只清空小统计区；不在 ISR 复制整个环形缓冲。
        if o.age <= self.last_age {
            self.phase = [Bin::EMPTY; PHASE_BINS];
            self.sector = [Bin::EMPTY; SECTOR_BINS];
            self.total = 0;
            self.next = 0;
            self.len = 0;
        }
        let record = Record {
            age: o.age,
            forced: o.forced,
            observer: o.observer,
            pll_error: o.pll_error,
            raw,
            duty,
            transport_ma: transport_ma.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
            emf: o.emf,
        };
        self.phase[usize::from(o.forced >> 12)].push(record.delta(), record.transport_ma);
        // [最早下降相, 最晚下降相] 唯一确定六种合法顺序；Frame::valid 已验证。
        let sector = match (order[0], order[2]) {
            (0, 2) => 0, // ABC
            (0, 1) => 1, // ACB
            (1, 2) => 2, // BAC
            (1, 0) => 3, // BCA
            (2, 1) => 4, // CAB
            (2, 0) => 5, // CBA
            _ => return,
        };
        self.sector[sector].push(record.delta(), record.transport_ma);
        self.records[usize::from(self.next)] = record;
        self.next = (self.next + 1) & (RECORDS as u8 - 1);
        self.len = self.len.saturating_add(1).min(RECORDS as u8);
        self.total += 1;
        self.last_age = o.age;
    }
    /// 停机后按最旧到最新读取；索引越界返回 None，不暴露失效的旧启动记录。
    pub fn record(&self, chronological: usize) -> Option<Record> {
        if chronological >= usize::from(self.len) {
            return None;
        }
        let oldest = if usize::from(self.len) == RECORDS {
            usize::from(self.next)
        } else {
            0
        };
        Some(self.records[(oldest + chronological) & (RECORDS - 1)])
    }
}

const _: () = assert!(core::mem::size_of::<Record>() == 24);
const _: () = assert!(core::mem::size_of::<Bin>() == 12);
const _: () = assert!(core::mem::size_of::<Trace>() <= 1824);
