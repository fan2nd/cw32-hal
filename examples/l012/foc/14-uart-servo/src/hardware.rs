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

/// Keep sub-degree precision without overflowing a 32-bit product.
pub fn deci_degrees(counts: i32) -> i32 {
    counts / 16384 * 3600 + counts % 16384 * 3600 / 16384
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_turns_cross_encoder_wrap_in_both_directions() {
        let mut previous = 16000;
        let mut counts = 0;
        for (step, expected) in [(128, 7200), (-128, 0), (-128, -7200)] {
            for _ in 0..256 {
                let next = (i32::from(previous) + step).rem_euclid(16384) as u16;
                counts += angle_delta(next, previous);
                previous = next;
            }
            assert_eq!(deci_degrees(counts), expected);
        }
    }
}
