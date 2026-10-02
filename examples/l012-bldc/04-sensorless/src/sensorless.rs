#![forbid(unsafe_code)]
//! Stage 04 adds a passive, qualified BEMF threshold observation.
//! It never commutates, starts a motor, arms an output or schedules a timer.
//! Demagnetization blanking and commutation delay belong to the active stage.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Edge {
    Rising,
    Falling,
}

/// The C uses `TAB_RFling[1]` and ADC channels `{3,2,1,3,2,1}`.
pub const EXPECTED_EDGE: [Edge; 6] = [
    Edge::Falling,
    Edge::Rising,
    Edge::Falling,
    Edge::Rising,
    Edge::Falling,
    Edge::Rising,
];
pub const BEMF_CHANNEL: [usize; 6] = [3, 2, 1, 3, 2, 1];

/// Standalone passive detector for the ADC/zero-crossing teaching stage.
/// It never drives a bridge or starts a timer. Call `begin_sector` after the
/// relevant demagnetization interval, then feed PWM-synchronous ADC samples.
/// The first qualified crossing is reported once until another sector begins.
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

    /// Invalid sector or zero qualification count disarms the detector.
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
