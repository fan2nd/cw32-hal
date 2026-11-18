//! Encoder and sampling timing interfaces.
use embassy_cw32::{gpio::Flex, mode::Blocking, spi::{self, Spi}};

pub const ENCODER_COUNTS_PER_REV: u16 = 16_384;


pub const PWM_HZ: u32 = 16_000;
pub const CURRENT_SAMPLE_TICKS: u16 = 250; // BTIM1: 8 MHz / 16 kHz, sample at PWM midpoint.
pub const ADC_REFERENCE_MV: u32 = 5_000;
pub const ADC_FULL_SCALE: u32 = 4_095;
pub const CURRENT_BIAS_MV: u32 = 2_500;
pub const CURRENT_SHUNT_MOHM: u32 = 50;
pub const CURRENT_AMPLIFIER_GAIN: u32 = 10;
pub const BUS_DIVIDER_RATIO: u32 = 11; // C project: RV1=1, RV2=10.
pub const ADC_FULL_SCALE_COUNTS: i32 = 2_048;
pub const ALIGNMENT_PERIODS: u32 = PWM_HZ / 2; // C project: T_ALIGNMENT=500 ms.
// The closed-loop controller uses the C reference's A-axis alignment vector,
// so the encoder sample taken after alignment is electrical angle zero.
pub const ALIGNMENT_ELECTRICAL_OFFSET: u16 = 0;
pub const READY_ZERO_SAMPLES: u32 = 1_600;
pub const POT_ZERO_COUNTS: u16 = 150; // C project NMINVD-50.
pub const POT_MIN_COUNTS: u16 = 200; // C project NMINVD.
pub const POT_MAX_COUNTS: u16 = 3_523; // C project NMAXVD.
pub const POT_MIN_RPM: i32 = 0;
pub const POT_MAX_RPM: i32 = 240;
pub const SPEED_LOOP_DIV: u32 = 16;
pub const DIRECTION_TEST_PERIODS: u32 = PWM_HZ / 10;
// The reference C controller uses raw ADC-count current units. Keep the
// direction check in that same unit so it does not overdrive the amplifier.
pub const DIRECTION_TEST_IQ_Q15: i16 = 400;
pub const DIRECTION_MIN_DELTA_COUNTS: i32 = 4;
pub const BUS_MIN_MV: u32 = 8_000;
pub const BUS_MAX_MV: u32 = 16_000;

pub struct Encoder<'d> {
    spi: Spi<'d, Blocking>,
    cs: Flex<'d>,
}

impl<'d> Encoder<'d> {
    pub fn new(spi: Spi<'d, Blocking>, cs: Flex<'d>) -> Self {
        Self { spi, cs }
    }

    pub fn read_angle(&mut self) -> Result<u16, spi::Error> {
        let mut frame = [0u8; 3];
        self.cs.set_low();
        let result = self.spi.blocking_transfer_in_place(&mut frame);
        self.cs.set_high();
        result?;
        let raw = (u16::from(frame[0]) << 6) | (u16::from(frame[1]) >> 2);
        Ok((ENCODER_COUNTS_PER_REV - (raw & 0x3fff)) & 0x3fff)
    }
}
