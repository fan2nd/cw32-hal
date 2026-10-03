//! OPA owner and calibration for the l012 OPA IP.
//!
//! A bandgap startup witness is required independently of DAC availability.
//! Disable the power stage before configuring or calibrating an OPA. There is
//! no validation of board-level calibration accuracy or motor safety.

use super::{Bandgap, Error, OpaInstance, SignalPin};
#[cfg(dac_l012)]
use super::{DacDependency, DacInstance, DacSource, DacSourceInstance};
use crate::{gpio::AnyPin, pac, Peri};
use embedded_hal::delay::DelayNs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gain {
    X2 = 0,
    X4 = 1,
    X8 = 2,
    X16 = 3,
    X32 = 4,
}
#[derive(Clone, Copy, Debug)]
pub struct OpaConfig {
    /// Bias code 0..=7 selects 1..=8uA. Default 7 matches datasheet electrical
    /// characterization; other biases trade speed/settling for consumption.
    pub bias: u8,
}
impl Default for OpaConfig {
    fn default() -> Self {
        Self { bias: 7 }
    }
}
fn opa_word(
    pch: u8,
    nch: Option<u8>,
    mode: u8,
    gain: Gain,
    config: OpaConfig,
) -> Result<pac::opa::regs::Cr, Error> {
    if !(1..=4).contains(&pch) || config.bias > 7 {
        return Err(Error::InvalidSignal);
    }
    if nch.is_some_and(|n| n != 11 && n != 12) {
        return Err(Error::InvalidSignal);
    }
    let mut word = pac::opa::regs::Cr(0);
    word.set_bias(config.bias);
    word.set_amp(gain as u8);
    word.set_mode(mode);
    match pch {
        1 => word.set_inp1en(true),
        2 => word.set_inp2en(true),
        3 => word.set_inp3en(true),
        4 => word.set_inp4en(true),
        _ => unreachable!(),
    }
    match nch {
        Some(11) => word.set_inn1en(true),
        Some(12) => word.set_inn2en(true),
        _ => {}
    }
    word.set_en(true);
    Ok(word)
}
/// Triggered calibration configuration. Field encoding is explicit because the
/// SDK's period comments are twice the CSR table's values. RM: duration is
/// 8*2^period_code OPACLK periods; AZRUN stays set twice that duration.
#[derive(Clone, Copy, Debug)]
pub struct Calibration {
    pub divider_log2: u8,
    pub period_code: u8,
}
fn calibration_word(config: Calibration) -> Result<pac::opa::regs::Cal, Error> {
    if config.divider_log2 > 7 || config.period_code > 15 {
        return Err(Error::InvalidCalibration);
    }
    let mut word = pac::opa::regs::Cal(0);
    word.set_clkdiv(config.divider_log2);
    word.set_calperiod(config.period_code);
    word.set_calen(true);
    Ok(word)
}
/// Single OPA with one positive route and, in external mode, one negative route.
/// Output pin is always owned, preventing a simultaneous DAC output on that pad.
///
/// A DAC-backed OPA retains its source guard, allowing single-channel code
/// updates while keeping the source enabled and its route/pins owned.
pub struct Opa<'d, I: OpaInstance> {
    _clock: crate::rcc::ClockGuard,
    _instance: Peri<'d, I>,
    _positive: Option<Peri<'d, AnyPin>>,
    _negative: Option<Peri<'d, AnyPin>>,
    _output: Peri<'d, AnyPin>,
    _bandgap: &'d Bandgap<'d>,
    #[cfg(dac_l012)]
    _dac: Option<&'d dyn DacDependency>,
    #[cfg(adc_l012)]
    settled: bool,
}
/// Exclusive borrow of an enabled, settled OPA output. Its ADC channel mappings
/// come from the OPA output pad's audited ADC routes; no GPIO token is duplicated.
/// The borrow retains the OPA and all of its pins, BGR and DAC dependencies, and
/// prevents calibration or dropping the owner during ADC operations/sequences.
#[cfg(adc_l012)]
pub struct OpaOutput<'a, I: OpaInstance> {
    _borrow: core::marker::PhantomData<&'a mut I>,
}
impl<'d, I: OpaInstance> Opa<'d, I> {
    pub fn follower<P: SignalPin<I, PCH>, O: SignalPin<I, 0>, const PCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        output: Peri<'d, O>,
        bandgap: &'d Bandgap<'d>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        if !(1..=3).contains(&PCH) {
            return Err(Error::InvalidSignal);
        }
        Self::build(
            instance,
            Some(positive.into()),
            PCH,
            None,
            None,
            output.into(),
            3,
            Gain::X2,
            bandgap,
            #[cfg(dac_l012)]
            None,
            config,
            delay,
        )
    }
    pub fn pga<P: SignalPin<I, PCH>, O: SignalPin<I, 0>, const PCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        output: Peri<'d, O>,
        gain: Gain,
        bandgap: &'d Bandgap<'d>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        if !(1..=3).contains(&PCH) {
            return Err(Error::InvalidSignal);
        }
        Self::build(
            instance,
            Some(positive.into()),
            PCH,
            None,
            None,
            output.into(),
            2,
            gain,
            bandgap,
            #[cfg(dac_l012)]
            None,
            config,
            delay,
        )
    }
    pub fn external<
        P: SignalPin<I, PCH>,
        N: SignalPin<I, NCH>,
        O: SignalPin<I, 0>,
        const PCH: u8,
        const NCH: u8,
    >(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        output: Peri<'d, O>,
        bandgap: &'d Bandgap<'d>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        if !(1..=3).contains(&PCH) {
            return Err(Error::InvalidSignal);
        }
        Self::build(
            instance,
            Some(positive.into()),
            PCH,
            Some(negative.into()),
            Some(NCH),
            output.into(),
            0,
            Gain::X2,
            bandgap,
            #[cfg(dac_l012)]
            None,
            config,
            delay,
        )
    }
    /// Internal DAC1 -> OPA1 / DAC2 -> OPA2, with an externally owned OPA output.
    #[cfg(dac_l012)]
    pub fn dac_follower<D: DacInstance, O: SignalPin<I, 0>, const C: u8>(
        instance: Peri<'d, I>,
        dac: &'d DacSource<'_, D, C>,
        output: Peri<'d, O>,
        bandgap: &'d Bandgap<'d>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error>
    where
        I: DacSourceInstance<D, C>,
    {
        Self::build(
            instance,
            None,
            4,
            None,
            None,
            output.into(),
            3,
            Gain::X2,
            bandgap,
            Some(dac),
            config,
            delay,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn build(
        instance: Peri<'d, I>,
        positive: Option<Peri<'d, AnyPin>>,
        pch: u8,
        negative: Option<Peri<'d, AnyPin>>,
        nch: Option<u8>,
        output: Peri<'d, AnyPin>,
        mode: u8,
        gain: Gain,
        bandgap: &'d Bandgap<'d>,
        #[cfg(dac_l012)] dac: Option<&'d dyn DacDependency>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let word = opa_word(pch, nch, mode, gain, config)?;
        let clock = I::acquire();
        let r = I::regs();
        r.cr().write_value(pac::opa::regs::Cr(0xe000));
        r.cal().write_value(pac::opa::regs::Cal(0));
        if let Some(pin) = &positive {
            pin.configure_analog();
        }
        if let Some(pin) = &negative {
            pin.configure_analog();
        }
        output.configure_analog();
        r.cr().write_value(word);
        delay.delay_us(40);
        Ok(Self {
            _clock: clock,
            _instance: instance,
            _positive: positive,
            _negative: negative,
            _output: output,
            _bandgap: bandgap,
            #[cfg(dac_l012)]
            _dac: dac,
            #[cfg(adc_l012)]
            settled: true,
        })
    }
    /// Borrow the output for ADC conversion without releasing its pin.
    /// A calibration timeout leaves the output unavailable until a successful
    /// calibration (including settling) or reconstruction of the OPA.
    #[cfg(adc_l012)]
    pub fn output(&mut self) -> Result<OpaOutput<'_, I>, Error> {
        if !I::regs().cr().read().en() {
            return Err(Error::Disabled);
        }
        if I::regs().cal().read().azrun() {
            return Err(Error::Busy);
        }
        if !self.settled {
            return Err(Error::NotReady);
        }
        Ok(OpaOutput {
            _borrow: core::marker::PhantomData,
        })
    }
    /// Software-triggered bounded calibration. Output is invalid during this
    /// call. After timeout, calibration may still be running and ADC output
    /// borrowing remains unavailable until a successful subsequent calibration
    /// or reconstruction of the peripheral after drop.
    /// A busy edge must be observed before reporting completion; an operation
    /// whose entire busy pulse is missed returns a conservative timeout.
    pub fn calibrate(
        &mut self,
        config: Calibration,
        poll_budget: u32,
        delay: &mut impl DelayNs,
    ) -> Result<(), Error> {
        let word = calibration_word(config)?;
        if poll_budget == 0 {
            return Err(Error::ZeroPollBudget);
        }
        let r = I::regs();
        if r.cal().read().azrun() {
            return Err(Error::Busy);
        }
        #[cfg(adc_l012)]
        {
            self.settled = false;
        }
        // Do not RMW the trigger/status register: SOFTTRIG is a write-one command.
        r.cal().write_value(word);
        let mut trigger = word;
        trigger.set_softtrig(true);
        r.cal().write_value(trigger);
        wait_calibration(poll_budget, || r.cal().read().azrun())?;
        delay.delay_us(10);
        #[cfg(adc_l012)]
        {
            self.settled = true;
        }
        Ok(())
    }
    pub fn is_calibrating(&self) -> bool {
        I::regs().cal().read().azrun()
    }
}
// SOFTTRIG crosses to OPACLK. Idle immediately after the write may precede
// acceptance, so require an observed busy edge before accepting completion.
// Fast operations whose entire busy pulse is missed conservatively time out.
fn wait_calibration(poll_budget: u32, mut is_busy: impl FnMut() -> bool) -> Result<(), Error> {
    let mut seen_busy = false;
    for _ in 0..poll_budget {
        let busy = is_busy();
        if busy {
            seen_busy = true;
        } else if seen_busy {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(Error::Timeout)
}

impl<I: OpaInstance> Drop for Opa<'_, I> {
    fn drop(&mut self) {
        I::regs().cr().write_value(pac::opa::regs::Cr(0xe000));
        I::regs().cal().write_value(pac::opa::regs::Cal(0));
        if let Some(pin) = &self._positive {
            pin.disconnect();
        }
        if let Some(pin) = &self._negative {
            pin.disconnect();
        }
        self._output.disconnect();
    }
}
