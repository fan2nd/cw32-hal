//! Encoder and sampling timing interfaces.
use embassy_cw32::{
    gpio::Output,
    mode::Blocking,
    spi::{self, Spi},
};

use crate::board_resources::EncoderSpi;
use embassy_cw32::{
    gpio::{Level, Speed},
    time::Hertz,
};
use embedded_hal::spi::MODE_1;
pub const ENCODER_COUNTS_PER_REV: u16 = 16_384;

pub fn connect(pins: EncoderSpi) -> Encoder<'static> {
    let cs = Output::new(pins.cs, Level::High, Speed::Default);
    let mut config = spi::Config::default();
    config.mode = MODE_1;
    config.frequency = Hertz(4_000_000);
    let spi = Spi::new_blocking(pins.spi, pins.sck, pins.mosi, pins.miso, config).unwrap();
    Encoder::new(spi, cs)
}

pub struct Encoder<'d> {
    spi: Spi<'d, Blocking>,
    cs: Output<'d>,
}

impl<'d> Encoder<'d> {
    pub fn new(spi: Spi<'d, Blocking>, cs: Output<'d>) -> Self {
        Self { spi, cs }
    }

    pub fn read_angle(&mut self) -> Result<u16, spi::Error> {
        let mut frame = [0u8; 3];
        self.cs.set_low();
        let result = self.spi.blocking_transfer_in_place_bounded(&mut frame, 256);
        self.cs.set_high();
        result?;
        let raw = (u16::from(frame[0]) << 6) | (u16::from(frame[1]) >> 2);
        Ok((ENCODER_COUNTS_PER_REV - (raw & 0x3fff)) & 0x3fff)
    }
}
