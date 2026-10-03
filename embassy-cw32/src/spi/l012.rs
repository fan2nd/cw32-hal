//! CW32 SPI IP with separate CR2 enable and arithmetic BR divider.
use super::{BitOrder, Config, ConfigError, Phase, Polarity};
use crate::pac::spi::{regs, Spi};

pub(super) fn divider(clock: u32, requested: u32) -> Result<(u8, u16), ConfigError> {
    if clock == 0 || requested == 0 {
        return Err(ConfigError::ZeroFrequency);
    }
    // ceil(clock / (2 * requested)); use u64 to avoid overflow and rounding
    // above the requested maximum when the nominal rate is fractional.
    let half = u64::from(clock).div_ceil(2 * u64::from(requested));
    if half > 128 {
        return Err(ConfigError::FrequencyTooLow);
    }
    let half = half.max(1) as u16;
    Ok(((half - 1) as u8, half * 2))
}

pub(super) fn disable(r: Spi) {
    // CR2 reset has no reserved ones. Stop SPI and both DMA request sources.
    r.cr2().write_value(regs::Cr2(0));
}

pub(super) fn configure(r: Spi, config: &Config, br: u8, bits: u8) {
    disable(r);
    // RM 22.7.1 permits CR1 changes only while CR2.EN=0. All reserved bits
    // reset to zero; keep filter, delayed sampling, gap and target options off.
    let cr = (u32::from(br) << 24)
        | (u32::from(bits - 1) << 8)
        | (u32::from(config.bit_order == BitOrder::LsbFirst) << 7)
        | (u32::from(config.mode.polarity == Polarity::IdleHigh) << 3)
        | (u32::from(config.mode.phase == Phase::CaptureOnSecondTransition) << 2)
        | 0b11; // SSM=1, MSTR=1, MODE=0 (full duplex).
    r.cr1().write_value(regs::Cr1(cr));
    r.cr3().write_value(regs::Cr3(0));
    r.ssi().write_value(regs::Ssi(0));
    r.cr2().write_value(regs::Cr2(1));
}
