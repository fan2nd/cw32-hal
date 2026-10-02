#![deny(unsafe_code)]
//! Timing, duty scaling and bridge validation for the L012 board driver.
use crate::control::Bridge;

pub const CPU_HZ: u32 = 96_000_000;
pub const PCLK_HZ: u32 = 48_000_000;
pub const PWM_HZ: u32 = 20_000;
pub const PWM_PERIOD: u16 = (PCLK_HZ / PWM_HZ) as u16;
pub const STEP_TIMER_HZ: u32 = 8_000_000;
pub const ADC_HZ: u32 = PCLK_HZ / 8;
pub const ADC1_SAMPLE_CYCLES: u32 = 18;
pub const ADC2_SAMPLE_CYCLES: u32 = 518;
pub const ADC_CONVERSION_CYCLES: u32 = 15;
pub const ADC1_SEQUENCE_NS: u32 =
    4 * (ADC1_SAMPLE_CYCLES + ADC_CONVERSION_CYCLES) * 1_000 / (ADC_HZ / 1_000_000);
pub const ADC2_SEQUENCE_NS: u32 =
    5 * (ADC2_SAMPLE_CYCLES + ADC_CONVERSION_CYCLES) * 1_000 / (ADC_HZ / 1_000_000);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BridgeError {
    InvalidImage,
}

pub const fn pwm_count(source_count: u16) -> u16 {
    source_count / 2
}

/// Reject out-of-range timing and any same-phase high/low overlap.
pub fn validate_bridge(bridge: &Bridge) -> Result<(), BridgeError> {
    if bridge.pwm_counts.iter().any(|&n| n > 4800)
        || bridge.sample_compare >= 4800
        || (0..3).any(|n| bridge.pwm_counts[n] != 0 && bridge.low_sides[n])
    {
        Err(BridgeError::InvalidImage)
    } else {
        Ok(())
    }
}
