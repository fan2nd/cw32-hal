//! CW32 SPI IP with CR1 enable and power-of-two BR divider.
use super::{BitOrder, Config, ConfigError, Phase, Polarity};
use crate::pac::spi::{regs, Spi};

pub(super) fn divider(clock: u32, requested: u32) -> Result<(u8, u16), ConfigError> {
    if clock == 0 || requested == 0 {
        return Err(ConfigError::ZeroFrequency);
    }
    // RM 19.8.1 reserves BR=7. Do not copy STM32's /256 encoding.
    for br in 0..=6 {
        let div = 2u16 << br;
        if u64::from(clock) <= u64::from(requested) * u64::from(div) {
            return Ok((br, div));
        }
    }
    Err(ConfigError::FrequencyTooLow)
}

pub(super) fn disable(r: Spi) {
    r.cr1().modify(|w| {
        w.set_en(false);
        w.set_dmarx(false);
        w.set_dmatx(false);
    });
}

pub(super) fn configure(r: Spi, config: &Config, br: u8, bits: u8) {
    disable(r);
    // All reserved bits reset to zero. Keep delayed sampling, target and DMA
    // options off. The F030 WIDTH, SSM, CPOL and CPHA locations differ from L012.
    let cr = (u32::from(bits - 1) << 10)
        | (1 << 9) // SSM=1; hardware CS is not routed to a pin.
        | (u32::from(config.bit_order == BitOrder::LsbFirst) << 7)
        | (u32::from(br) << 3)
        | (1 << 2) // MSTR=1, MODE=0 (full duplex).
        | (u32::from(config.mode.polarity == Polarity::IdleHigh) << 1)
        | u32::from(config.mode.phase == Phase::CaptureOnSecondTransition);
    r.cr1().write_value(regs::Cr1(cr));
    r.cr2().write_value(regs::Cr2(0));
    r.ssi().write_value(regs::Ssi(0));
    r.cr1().modify(|w| w.set_en(true));
}
