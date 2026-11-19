//! FOC runtime configuration.

pub use crate::param::*;

pub const PWM_HZ: u32 = 16_000;
pub const CURRENT_SAMPLE_TICKS: u16 = 250; // BTIM1: 8 MHz / 16 kHz, sample at PWM midpoint.
pub const ALIGNMENT_PERIODS: u32 = PWM_HZ / 2; // C project: T_ALIGNMENT=500 ms.
pub const READY_ZERO_SAMPLES: u32 = 1_600;
pub const POT_ZERO_COUNTS: u16 = 150; // C project NMINVD-50.
pub const POT_MIN_COUNTS: u16 = 200; // C project NMINVD.
pub const POT_MAX_COUNTS: u16 = 3_523; // C project NMAXVD.
pub const POT_MIN_RPM: i32 = 0;
pub const POT_MAX_RPM: i32 = 240;
pub const SPEED_LOOP_DIV: u32 = 16;
pub const DIRECTION_TEST_PERIODS: u32 = PWM_HZ / 10;

pub const PROBE_VOLTAGE_LIMIT_Q15: i16 = 6_000;
pub const CURRENT_TRIP_COUNTS: i32 = 2_000;
