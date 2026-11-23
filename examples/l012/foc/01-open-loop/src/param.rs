//! Motor and analog front-end parameters.

pub const ENCODER_COUNTS_PER_REV: u16 = 16_384;
pub const ADC_REFERENCE_MV: u32 = 5_000;
pub const ADC_FULL_SCALE: u32 = 4_095;
pub const CURRENT_BIAS_MV: u32 = 2_500;
pub const CURRENT_SHUNT_MOHM: u32 = 50;
pub const CURRENT_AMPLIFIER_GAIN: u32 = 10;
pub const BUS_DIVIDER_RATIO: u32 = 11; // C project: RV1=1, RV2=10.
pub const ADC_FULL_SCALE_COUNTS: i32 = 2_048;
pub const ALIGNMENT_ELECTRICAL_OFFSET: u16 = 0;
pub const DIRECTION_TEST_IQ_Q15: i16 = 400;
pub const DIRECTION_MIN_DELTA_COUNTS: i32 = 4;
pub const BUS_MIN_MV: u32 = 8_000;
pub const BUS_MAX_MV: u32 = 16_000;
