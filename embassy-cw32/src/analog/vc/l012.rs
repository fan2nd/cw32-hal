//! Comparator hardware and input ownership for the l012 VC IP.
//!
//! VC pair reference identity is independent of the shared IRQ13/IRQ24 pairing.

use super::super::{Error, SignalPin, VcInstance};
use super::r#async::{VcIo, WaitKind};
use super::InterruptHandler;
use crate::{gpio::AnyPin, pac, Async, Blocking, Mode, Peri};

use super::super::Bandgap;
#[cfg(vcref_l012)]
use super::super::RefDivider;
#[cfg(dac_l012)]
use super::super::{DacDependency, DacSource, DacSourceInstance};
use embedded_hal::delay::DelayNs;

/// PCLK-based comparator filter encodings from RM 27.7.4. No LSI dependency.
#[derive(Clone, Copy, Debug, Default)]
#[repr(u8)]
pub enum Filter {
    #[default]
    None = 0,
    Div1N2 = 1,
    Div1N4 = 2,
    Div1N8 = 3,
    Div2N6 = 4,
    Div2N8 = 5,
    Div4N6 = 6,
    Div4N8 = 7,
    Div8N6 = 8,
    Div8N8 = 9,
    Div16N5 = 10,
    Div16N6 = 11,
    Div16N8 = 12,
    Div32N5 = 13,
    Div32N6 = 14,
    Div32N8 = 15,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ComparatorConfig {
    pub high_speed: bool,
    /// Approximately 20mV hysteresis when enabled.
    pub hysteresis: bool,
    pub inverted: bool,
    pub filter: Filter,
}
fn comparator_words(
    positive: u8,
    negative: u8,
    config: ComparatorConfig,
) -> Result<(pac::vc::regs::Cr0, pac::vc::regs::Cr1), Error> {
    if positive > 3 || negative > 3 {
        return Err(Error::InvalidSignal);
    }
    let mut cr0 = pac::vc::regs::Cr0(0);
    cr0.set_inp(positive);
    cr0.set_inn(negative);
    cr0.set_resp(config.high_speed);
    cr0.set_hys(config.hysteresis);
    cr0.set_pol(config.inverted);
    cr0.set_en(false);
    let mut cr1 = pac::vc::regs::Cr1(0);
    cr1.set_fltclk(true);
    cr1.set_flttime(config.filter as u8);
    Ok((cr0, cr1))
}
/// Comparator owner, configured but initially disabled.
///
/// `Blocking` reads the output directly; `Async` additionally owns an interrupt
/// binding and provides one-shot event waits. Both modes retain every input pin
/// and the bandgap, divider or DAC borrow until the comparator is dropped.
/// VC pair reference identity is independent of the shared IRQ13/IRQ24 pairing.
pub struct Comp<'d, I: VcInstance, M: Mode> {
    _clock: crate::rcc::ClockGuard,
    _instance: Peri<'d, I>,
    _positive: Peri<'d, AnyPin>,
    _negative: Option<Peri<'d, AnyPin>>,
    #[cfg(vcref_l012)]
    _reference: Option<&'d RefDivider<'d, I::Reference>>,
    #[cfg(dac_l012)]
    _dac: Option<&'d dyn DacDependency>,
    _bandgap: &'d Bandgap<'d>,
    settled: bool,
    _mode: core::marker::PhantomData<M>,
}
impl<'d, I: VcInstance> Comp<'d, I, Blocking> {
    /// Configure external positive and negative inputs. Call `enable` before
    /// sampling the output. Only CH0/CH1 can be used as the negative input.
    pub fn new_blocking<
        P: SignalPin<I, PCH>,
        N: SignalPin<I, NCH>,
        const PCH: u8,
        const NCH: u8,
    >(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        Self::build_external(instance, positive, negative, bandgap, config)
    }
    /// Borrow this VC pair's reference divider. The borrow prevents changing or
    /// dropping the reference until the comparator owner is dropped.
    #[cfg(vcref_l012)]
    pub fn new_blocking_with_reference<P: SignalPin<I, PCH>, const PCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        reference: &'d RefDivider<'d, I::Reference>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        Self::build(
            instance,
            positive.into(),
            PCH,
            None,
            3,
            Some(reference),
            #[cfg(dac_l012)]
            None,
            bandgap,
            config,
        )
    }
    /// Borrow the DAC threshold: VC1/3 use channel 1 and VC2/4 use channel 2.
    /// The borrowed source guard allows threshold updates while keeping its
    /// channel enabled and unavailable for reconfiguration or drop.
    #[cfg(dac_l012)]
    pub fn new_blocking_with_dac<P: SignalPin<I, PCH>, const PCH: u8, const C: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        dac: &'d DacSource<'_, C>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
    ) -> Result<Self, Error>
    where
        I: DacSourceInstance<C>,
    {
        Self::build(
            instance,
            positive.into(),
            PCH,
            None,
            2,
            #[cfg(vcref_l012)]
            None,
            Some(dac),
            bandgap,
            config,
        )
    }
}
impl<'d, I: VcInstance> Comp<'d, I, Async> {
    /// Configure external inputs and install the binding for one-shot event waits.
    /// Call `enable` before waiting. The binding must dispatch this instance's
    /// handler on its physical shared IRQ; sibling handlers remain independent.
    pub fn new<P: SignalPin<I, PCH>, N: SignalPin<I, NCH>, const PCH: u8, const NCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        bandgap: &'d Bandgap<'d>,
        irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        let mut comp = Self::build_external(instance, positive, negative, bandgap, config)?;
        comp.init_interrupt(irq);
        Ok(comp)
    }
    /// Configure a borrowed reference-divider input and an interrupt binding.
    #[cfg(vcref_l012)]
    pub fn new_with_reference<P: SignalPin<I, PCH>, const PCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        reference: &'d RefDivider<'d, I::Reference>,
        bandgap: &'d Bandgap<'d>,
        irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        let mut comp = Self::build(
            instance,
            positive.into(),
            PCH,
            None,
            3,
            Some(reference),
            #[cfg(dac_l012)]
            None,
            bandgap,
            config,
        )?;
        comp.init_interrupt(irq);
        Ok(comp)
    }
    /// Configure a borrowed DAC threshold and an interrupt binding. VC1/3 use
    /// channel 1 and VC2/4 use channel 2. The source remains updatable while its
    /// channel enable, configuration and ownership stay reserved by the guard.
    #[cfg(dac_l012)]
    pub fn new_with_dac<P: SignalPin<I, PCH>, const PCH: u8, const C: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        dac: &'d DacSource<'_, C>,
        bandgap: &'d Bandgap<'d>,
        irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
        config: ComparatorConfig,
    ) -> Result<Self, Error>
    where
        I: DacSourceInstance<C>,
    {
        let mut comp = Self::build(
            instance,
            positive.into(),
            PCH,
            None,
            2,
            #[cfg(vcref_l012)]
            None,
            Some(dac),
            bandgap,
            config,
        )?;
        comp.init_interrupt(irq);
        Ok(comp)
    }
    fn init_interrupt(
        &mut self,
        _irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
    ) {
        use crate::interrupt::typelevel::Interrupt;
        // Construction has disabled/cleared this source and its software state.
        // Never unpend the shared vector: a sibling may already have an event.
        unsafe { I::Interrupt::enable() };
    }
}
impl<'d, I: VcInstance, M: Mode> Comp<'d, I, M> {
    fn build_external<P: SignalPin<I, PCH>, N: SignalPin<I, NCH>, const PCH: u8, const NCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        if NCH > 1 {
            return Err(Error::InvalidSignal);
        }
        Self::build(
            instance,
            positive.into(),
            PCH,
            Some(negative.into()),
            NCH,
            #[cfg(vcref_l012)]
            None,
            #[cfg(dac_l012)]
            None,
            bandgap,
            config,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn build(
        instance: Peri<'d, I>,
        positive: Peri<'d, AnyPin>,
        pch: u8,
        negative: Option<Peri<'d, AnyPin>>,
        nch: u8,
        #[cfg(vcref_l012)] reference: Option<&'d RefDivider<'d, I::Reference>>,
        #[cfg(dac_l012)] dac: Option<&'d dyn DacDependency>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        let (cr0, cr1) = comparator_words(pch, nch, config)?;
        let clock = I::acquire();
        let r = I::regs();
        critical_section::with(|_| {
            // Configure this VC only: no shared reset/reference-register writes.
            r.cr0().write_value(pac::vc::regs::Cr0(0));
            r.cr2().write_value(pac::vc::regs::Cr2(0));
            r.cr1().write_value(cr1);
            r.sr().write_value(pac::vc::regs::Sr(0));
            I::state().reset();
        });
        positive.configure_analog();
        if let Some(pin) = &negative {
            pin.configure_analog();
        }
        r.cr0().write_value(cr0);
        Ok(Self {
            _clock: clock,
            _instance: instance,
            _positive: positive,
            _negative: negative,
            #[cfg(vcref_l012)]
            _reference: reference,
            #[cfg(dac_l012)]
            _dac: dac,
            _bandgap: bandgap,
            settled: false,
            _mode: core::marker::PhantomData,
        })
    }
    /// Enable and allow a conservative 32 us startup delay. This differs from
    /// STM32's bare enable bit so output/waits do not silently skip CW32 settling.
    /// Calling this on an already enabled comparator does not restart it.
    pub fn enable(&mut self, delay: &mut impl DelayNs) {
        if !self.is_enabled() {
            self.settled = false;
            I::regs().cr0().modify(|w| w.set_en(true));
        }
        if !self.settled {
            delay.delay_us(32);
            self.settled = true;
        }
    }
    /// Disable this comparator and its event source, preserving input selection
    /// and every resource borrow. Shared clock/BGR/NVIC state is left untouched.
    pub fn disable(&mut self) {
        self.settled = false;
        critical_section::with(|_| {
            let mut io = VcHardware::<I>(core::marker::PhantomData);
            io.disable();
            I::regs().cr0().modify(|w| w.set_en(false));
            io.clear();
            I::state().reset();
        });
    }
    pub fn is_enabled(&self) -> bool {
        I::regs().cr0().read().en()
    }
    /// Read the filtered, configured-polarity output only while enabled/settled.
    /// An interrupted startup delay cannot make an unsettled output readable.
    pub fn output_level(&self) -> Result<bool, Error> {
        if !self.is_enabled() {
            return Err(Error::Disabled);
        }
        if !self.settled {
            return Err(Error::NotReady);
        }
        Ok(I::regs().sr().read().fltv())
    }
    /// Identifier only; this does not install an automatic hardware motor trip.
    pub const fn number(&self) -> u8 {
        I::NUMBER
    }
}
impl<I: VcInstance, M: Mode> Drop for Comp<'_, I, M> {
    fn drop(&mut self) {
        self.disable();
        I::regs().cr0().write_value(pac::vc::regs::Cr0(0));
        I::regs().cr2().write_value(pac::vc::regs::Cr2(0));
        I::regs().cr1().write_value(pac::vc::regs::Cr1(0x10));
        self._positive.disconnect();
        if let Some(pin) = &self._negative {
            pin.disconnect();
        }
    }
}

impl WaitKind {
    fn bits(self) -> u32 {
        let mut selection = pac::vc::regs::Cr1(0);
        match self {
            Self::Rising => selection.set_riseie(true),
            Self::Falling | Self::Low => selection.set_fallie(true),
            Self::AnyEdge => {
                selection.set_riseie(true);
                selection.set_fallie(true);
            }
            Self::High => selection.set_highie(true),
        }
        selection.0
    }
}
pub(super) struct VcHardware<I: VcInstance>(pub(super) core::marker::PhantomData<I>);
impl<I: VcInstance> VcIo for VcHardware<I> {
    fn disable(&mut self) {
        let r = I::regs();
        r.cr0().modify(|w| w.set_ie(false));
        r.cr1().modify(|w| {
            w.set_highie(false);
            w.set_riseie(false);
            w.set_fallie(false);
        });
    }
    fn clear(&mut self) {
        // INTF is RW0; FLTV is RO. Write zero, never RMW this mixed register.
        I::regs().sr().write_value(pac::vc::regs::Sr(0));
    }
    fn arm(&mut self, kind: WaitKind) {
        let r = I::regs();
        let selection = pac::vc::regs::Cr1(kind.bits());
        r.cr1().modify(|w| {
            w.set_highie(selection.highie());
            w.set_riseie(selection.riseie());
            w.set_fallie(selection.fallie());
        });
        r.cr0().modify(|w| w.set_ie(true));
    }
    fn pending(&mut self) -> bool {
        I::regs().cr0().read().ie() && I::regs().sr().read().intf()
    }
    fn high(&mut self) -> bool {
        I::regs().sr().read().fltv()
    }
}
