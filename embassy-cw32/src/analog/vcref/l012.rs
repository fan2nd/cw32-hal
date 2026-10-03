//! Pair-owned reference divider for the l012 VCREF IP.

use super::{Error, RefInstance};
use crate::{pac, Peri};
use embedded_hal::delay::DelayNs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceSource {
    Vdda,
    Vcore,
}
#[derive(Clone, Copy, Debug)]
pub struct DividerConfig {
    pub source: ReferenceSource,
    /// 0..=7: output = source_voltage * (step+1) / 8. Vcore is nominally 1.6V.
    pub step: u8,
}
fn divider_word(config: DividerConfig) -> Result<pac::vcref::regs::Ref, Error> {
    if config.step > 7 {
        return Err(Error::InvalidDivider);
    }
    let mut word = pac::vcref::regs::Ref(0);
    word.set_div(config.step);
    word.set_vin(config.source == ReferenceSource::Vcore);
    word.set_en(true);
    Ok(word)
}
/// VC1/2 share one divider, VC3/4 another. A reference borrow prevents a live
/// comparator's divider from being reconfigured or dropped by safe Rust.
pub struct RefDivider<'d, I: RefInstance> {
    _clock: crate::rcc::ClockGuard,
    _instance: Peri<'d, I>,
}
impl<'d, I: RefInstance> RefDivider<'d, I> {
    pub fn new(
        instance: Peri<'d, I>,
        config: DividerConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let word = divider_word(config)?;
        let clock = I::acquire();
        I::regs().r#ref().write_value(word);
        delay.delay_us(32);
        Ok(Self {
            _clock: clock,
            _instance: instance,
        })
    }
}
impl<I: RefInstance> Drop for RefDivider<'_, I> {
    fn drop(&mut self) {
        I::regs().r#ref().write_value(pac::vcref::regs::Ref(0));
    }
}
