//! Encoder and sampling timing interfaces.
use embassy_cw32::{gpio::Flex, mode::Blocking, spi::{self, Spi}};

use crate::config::*;

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
