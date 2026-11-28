//! FOC runtime configuration.

pub use crate::param::*;

pub const PWM_HZ: u32 = 16_000;
pub const SPEED_LOOP_HZ: u32 = 2_000;
pub const SPEED_PERIOD_US: u32 = 1_000_000 / SPEED_LOOP_HZ;
pub const PROBE_VOLTAGE_LIMIT_Q15: i16 = 6_000;

pub const IQ_LIMIT_COUNTS: i16 = 200;
pub const CURRENT_TRIP_COUNTS: u16 = 2_000;
