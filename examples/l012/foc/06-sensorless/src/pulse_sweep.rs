//! Compare opposite voltage pulses on the same unsigned HFI axis.
use crate::foc::{PhaseCurrent, hfi::project};

pub const VOLTS: [i32; 6] = [0, 900, 1800, 2700, 3600, 5400];
pub const REPEATS: u8 = 8;
pub type Response = [[i16; 3]; VOLTS.len()];

pub struct Sweep {
    pub axis: u16,
    phase: u8,
    pair: u8,
    second: bool,
    baseline: i32,
    pub positive: Response,
    pub negative: Response,
}

impl Sweep {
    pub const fn new(axis: u16) -> Self {
        Self {
            axis,
            phase: 0,
            pair: 0,
            second: false,
            baseline: 0,
            positive: [[0; 3]; VOLTS.len()],
            negative: [[0; 3]; VOLTS.len()],
        }
    }

    pub fn sample(&mut self, current: PhaseCurrent) -> (i32, bool) {
        let level = usize::from(self.pair % VOLTS.len() as u8);
        let positive = self.second == (self.pair / VOLTS.len() as u8 % 2 != 0);
        if self.phase == 31 {
            self.baseline = project(current, self.axis).0;
        }
        let tap = match self.phase {
            33 => Some(0),
            35 => Some(1),
            39 => Some(2),
            _ => None,
        };
        if let Some(tap) = tap {
            let delta = project(current, self.axis).0 - self.baseline;
            let (sum, value) = if positive {
                (&mut self.positive[level][tap], delta)
            } else {
                (&mut self.negative[level][tap], -delta)
            };
            *sum = sum.saturating_add(value as i16);
        }
        let voltage = if (31..39).contains(&self.phase) {
            if positive {
                VOLTS[level]
            } else {
                -VOLTS[level]
            }
        } else {
            0
        };
        self.phase += 1;
        if self.phase == 40 {
            self.phase = 0;
            self.second = !self.second;
            if !self.second {
                self.pair += 1;
            }
        }
        (voltage, self.pair == VOLTS.len() as u8 * REPEATS)
    }
}
