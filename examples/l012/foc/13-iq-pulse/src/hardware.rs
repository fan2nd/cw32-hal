//! Hardware peripherals and fixed-point conversions for the L012 FOC example.

pub use crate::config::*;
pub use crate::foc::sin_cos_q15_lut;

pub fn bus_voltage_mv(sample: u16) -> u32 {
    u32::from(sample) * ADC_REFERENCE_MV * BUS_DIVIDER_RATIO / ADC_FULL_SCALE
}

pub fn angle_delta(current: u16, previous: u16) -> i32 {
    let delta = i32::from(current) - i32::from(previous);
    if delta > 8_192 {
        delta - 16_384
    } else if delta < -8_192 {
        delta + 16_384
    } else {
        delta
    }
}

/// Minimum single-frame tolerance for 14-bit board-oriented SSI counts.
pub const MAX_SAMPLE_DELTA_COUNTS: i32 = 512;
