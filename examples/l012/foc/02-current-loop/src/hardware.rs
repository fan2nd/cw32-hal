//! Hardware peripherals and fixed-point conversions for the L012 FOC example.

pub use crate::config::*;
pub use crate::foc::sin_cos_q15_lut;

pub fn q15_from_adc(sample: u16, offset: u16) -> i16 {
    let signed = i32::from(offset) - i32::from(sample);
    // The C project feeds the calibrated ADC difference directly to Clarke /
    // Park and tunes both PI loops in ADC-count units (M1_IQ_MAX_COUNTS=2000).
    // Do not normalize by 2048 here: that multiplies the measured current by
    // sixteen and makes a normal speed command saturate the power stage.
    signed.clamp(-32_767, 32_767) as i16
}

/// Convert a calibrated ADC difference to mA using the board's 2.5 V bias,
/// 50 mOhm shunt and 10x current amplifier. The bias itself is removed by the
/// runtime offset sample before this function is called; no floating point or
/// 64-bit intermediate is needed.
pub fn current_ma_from_counts(counts: i16) -> i32 {
    let output_mv =
        i32::from(counts).saturating_mul(ADC_REFERENCE_MV as i32) / ADC_FULL_SCALE as i32;
    output_mv.saturating_mul(1_000) / (CURRENT_SHUNT_MOHM * CURRENT_AMPLIFIER_GAIN) as i32
}

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
