//! FOC runtime configuration.

pub use crate::param::*;

pub const PWM_HZ: u32 = 16_000;
pub const SPEED_LOOP_HZ: u32 = 2_000;
pub const SPEED_PERIOD_US: u32 = 1_000_000 / SPEED_LOOP_HZ;
pub const READY_ZERO_SAMPLES: u32 = 1_600;
pub const POT_ZERO_COUNTS: u16 = 150; // C project NMINVD-50.
pub const POT_MIN_COUNTS: u16 = 200; // C project NMINVD.
pub const POT_MAX_COUNTS: u16 = 3_523; // C project NMAXVD.
pub const POT_MIN_RPM: i32 = 0;
pub const POT_MAX_RPM: i32 = 800;

pub const PROBE_VOLTAGE_LIMIT_Q15: i16 = 6_000;
pub const CURRENT_TRIP_COUNTS: i32 = 2_000;
