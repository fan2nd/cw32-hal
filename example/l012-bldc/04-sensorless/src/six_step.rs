#![forbid(unsafe_code)]
//! Stage 03 adds a pure six-step bridge image and one-press sector selection.
//! These values are observations. No hardware output function exists here.

pub const PWM_PERIOD: u16 = 4800;
pub const DEMONSTRATION_DUTY: u16 = PWM_PERIOD / 20;
pub const KEY_DEBOUNCE_MS: u8 = 60;

/// Source-scale counts: later active stages map 4800 counts to 2400 PCLK ticks.
/// Stages 03/04 only display this image; the sampling board leaves gate pins untouched.
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

    /// A+B-, A+C-, B+C-, B+A-, C+A-, C+B-. Invalid sectors fail closed.
    pub const fn commutation(sector: u8, duty: u16) -> Self {
        let duty = if duty > PWM_PERIOD { PWM_PERIOD } else { duty };
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

    pub fn is_valid(&self) -> bool {
        self.pwm_counts.iter().all(|&n| n <= PWM_PERIOD)
            && self.sample_compare < PWM_PERIOD
            && (0..3).all(|n| self.pwm_counts[n] == 0 || !self.low_sides[n])
    }
}

/// Saturating debounce: one new sector per 60 ms press, rearmed on release.
#[derive(Default)]
pub struct SectorSelector {
    sector: u8,
    held_ms: u8,
}
impl SectorSelector {
    pub fn sector(&self) -> u8 {
        self.sector
    }
    pub fn bridge(&self) -> Bridge {
        Bridge::commutation(self.sector, DEMONSTRATION_DUTY)
    }
    pub fn tick_1ms(&mut self, pressed: bool) -> bool {
        self.held_ms = if pressed {
            self.held_ms.saturating_add(1)
        } else {
            0
        };
        if self.held_ms == KEY_DEBOUNCE_MS {
            self.sector = (self.sector + 1) % 6;
            true
        } else {
            false
        }
    }
}
