//! Owned analog building blocks for CW32L012 (RM v1.4 chapters 25--29).
//!
//! OPA, VC and DAC pins are generated from audited signal routes. A shared reset
//! is never asserted and shared clock gates are never disabled on drop. BGR is
//! enabled once and deliberately left enabled for other analog peripherals.
//! VC additionally offers IRQ-backed one-shot async event waits. OPA calibration
//! and DAC updates remain blocking; there is no analog DMA stream.
//! Values are raw electrical codes, not calibrated volts. Startup delays are
//! conservative software choices, not a guarantee outside datasheet conditions.
//! No board-level signal integrity, calibration accuracy or motor safety has
//! been validated. Disable the power stage before configuring/calibrating OPA.
//!
//! Drivers accept owned peripheral handles or short-lived `Peri::reborrow()`
//! handles. Erasing a pin preserves its borrow; analog source borrows also last
//! until the consuming comparator or OPA is dropped.
//!

use crate::{
    gpio::{AnyPin, Pin},
    pac, peripherals,
    rcc::PeripheralClock,
    Peri, PeripheralType,
};
use embedded_hal::delay::DelayNs;

mod sealed {
    pub(crate) trait OpaInstance {
        fn regs() -> crate::pac::opa::RegisterBlock;
    }
    pub(crate) trait RefInstance {
        fn regs() -> crate::pac::vcref::RegisterBlock;
    }
    pub(crate) trait VcInstance {
        fn regs() -> crate::pac::vc::RegisterBlock;
        fn state() -> &'static crate::async_support::EventState;
    }
    pub trait Pin<I, const S: u8> {}
}
/// Audited bonded signal route. OPA: OUT=0, INP1..3=1..3, INN1..2=11..12;
/// VC: CH0..3=0..3; DAC: OUT1/OUT2=1/2.
pub trait SignalPin<I, const S: u8>: sealed::Pin<I, S> + Pin {}
/// Audited OPA peripheral identity. Register access is internal to the driver.
#[allow(private_bounds)]
pub trait OpaInstance: sealed::OpaInstance + PeripheralClock + PeripheralType {}
#[allow(private_bounds)]
pub trait RefInstance: sealed::RefInstance + PeripheralClock + PeripheralType {}
#[allow(private_bounds)]
pub trait VcInstance: sealed::VcInstance + PeripheralClock + PeripheralType {
    type Interrupt: crate::interrupt::typelevel::Interrupt;
    type Reference: RefInstance;
    const NUMBER: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_analog.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidCode,
    InvalidSignal,
    InvalidDivider,
    InvalidCalibration,
    Busy,
    Timeout,
    ZeroPollBudget,
}

/// Owns the software BGR control. Dropping it NEVER disables BGR: ADC/VC/OPA may
/// have automatically enabled it too. It provides a startup-delay witness.
///
/// Copyable marker values cannot grant hardware access.
pub struct Bandgap<'d> {
    _token: Peri<'d, peripherals::BGR>,
}
impl<'d> Bandgap<'d> {
    pub fn new(token: Peri<'d, peripherals::BGR>, delay: &mut impl DelayNs) -> Self {
        critical_section::with(|_| unsafe {
            // Preserve TSEN; BGR has no peripheral gate/reset. RM 25.12.19.
            let value = pac::BGR.cr().read();
            pac::BGR
                .cr()
                .write(pac::bgr::fields::cr::BGREN.write(value, true));
        });
        delay.delay_us(32); // RM says approximately 30us, including hardware enable.
        Self { _token: token }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    One,
    Two,
}
/// Two internally enabled channels, initially zero. External outputs remain
/// disconnected until explicitly attached with `with_output1/with_output2`.
/// Reference is VDDA. Owning the external PB0/PB1 pin prevents OPA/DAC conflict.
///
/// A driver retains the exclusive borrow of its peripheral.
///
/// Attaching an output retains its borrow even after pin type erasure.
pub struct Dac<'d> {
    _token: Peri<'d, peripherals::DAC>,
    output1: Option<Peri<'d, AnyPin>>,
    output2: Option<Peri<'d, AnyPin>>,
    route: u32,
}
impl<'d> Dac<'d> {
    pub fn new(token: Peri<'d, peripherals::DAC>, delay: &mut impl DelayNs) -> Self {
        <peripherals::DAC as PeripheralClock>::enable_and_reset();
        use pac::dac::fields as f;
        // SAFETY: exclusive whole-DAC token; triggers/DMA/interrupts/waves off.
        unsafe {
            pac::DAC.cr0().write(0);
            pac::DAC.cr1().write(0);
            pac::DAC.dhr12r1().write(0);
            pac::DAC.dhr12r2().write(0);
            pac::DAC
                .cr0()
                .write(f::cr0::EN2.write(f::cr0::EN1.write(0, true), true));
        }
        delay.delay_us(10); // datasheet tSTART typical 3us, not a characterized max.
        Self {
            _token: token,
            output1: None,
            output2: None,
            route: 0,
        }
    }
    pub fn with_output1<P: SignalPin<peripherals::DAC, 1>>(mut self, pin: Peri<'d, P>) -> Self {
        let pin: Peri<'d, AnyPin> = pin.into();
        pin.configure_analog();
        self.route = pac::dac::fields::cr1::C1OUT.write(self.route, true);
        unsafe {
            pac::DAC.cr1().write(self.route);
        }
        self.output1 = Some(pin);
        self
    }
    pub fn with_output2<P: SignalPin<peripherals::DAC, 2>>(mut self, pin: Peri<'d, P>) -> Self {
        let pin: Peri<'d, AnyPin> = pin.into();
        pin.configure_analog();
        self.route = pac::dac::fields::cr1::C2OUT.write(self.route, true);
        unsafe {
            pac::DAC.cr1().write(self.route);
        }
        self.output2 = Some(pin);
        self
    }
    /// Write a 12-bit right-aligned code. TEN=0 transfers it to DOR after one
    /// peripheral clock; analog settling takes additional time. No blocking wait.
    pub fn set(&mut self, channel: Channel, code: u16) -> Result<(), Error> {
        let word = dac_code(code)?;
        unsafe {
            match channel {
                Channel::One => pac::DAC.dhr12r1().write(word),
                Channel::Two => pac::DAC.dhr12r2().write(word),
            }
        }
        Ok(())
    }
    /// Set both holding registers with one 32-bit write.
    pub fn set_pair(&mut self, one: u16, two: u16) -> Result<(), Error> {
        let word = dac_code(one)? | (dac_code(two)? << 16);
        unsafe {
            pac::DAC.dhr12rd().write(word);
        }
        Ok(())
    }
    pub fn output_code(&self, channel: Channel) -> u16 {
        (unsafe {
            match channel {
                Channel::One => pac::DAC.dor1().read(),
                Channel::Two => pac::DAC.dor2().read(),
            }
        } & 0xfff) as u16
    }
}
fn dac_code(code: u16) -> Result<u32, Error> {
    if code > 4095 {
        Err(Error::InvalidCode)
    } else {
        Ok(u32::from(code))
    }
}
impl Drop for Dac<'_> {
    fn drop(&mut self) {
        unsafe {
            pac::DAC.cr1().write(0);
            pac::DAC.cr0().write(0);
        }
        if let Some(pin) = &self.output1 {
            pin.disconnect();
        }
        if let Some(pin) = &self.output2 {
            pin.disconnect();
        }
    }
}

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
fn divider_word(config: DividerConfig) -> Result<u32, Error> {
    if config.step > 7 {
        return Err(Error::InvalidDivider);
    }
    use pac::vcref::fields::r#ref as f;
    Ok(f::EN.write(
        f::VIN.write(
            f::DIV.write(0, config.step.into()),
            config.source == ReferenceSource::Vcore,
        ),
        true,
    ))
}
/// VC1/2 share one divider, VC3/4 another. A reference borrow prevents a live
/// comparator's divider from being reconfigured or dropped by safe Rust.
pub struct RefDivider<'d, I: RefInstance> {
    _instance: Peri<'d, I>,
}
impl<'d, I: RefInstance> RefDivider<'d, I> {
    pub fn new(
        instance: Peri<'d, I>,
        config: DividerConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let word = divider_word(config)?;
        I::enable_and_reset();
        unsafe {
            I::regs().r#ref().write(word);
        }
        delay.delay_us(32);
        Ok(Self {
            _instance: instance,
        })
    }
}
impl<I: RefInstance> Drop for RefDivider<'_, I> {
    fn drop(&mut self) {
        unsafe {
            I::regs().r#ref().write(0);
        }
    }
}

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
) -> Result<(u32, u32), Error> {
    if positive > 3 || negative > 3 {
        return Err(Error::InvalidSignal);
    }
    use pac::vc::fields as f;
    let mut cr0 = f::cr0::INP.write(0, positive.into());
    cr0 = f::cr0::INN.write(cr0, negative.into());
    cr0 = f::cr0::RESP.write(cr0, config.high_speed);
    cr0 = f::cr0::HYS.write(cr0, config.hysteresis);
    cr0 = f::cr0::POL.write(cr0, config.inverted);
    cr0 = f::cr0::EN.write(cr0, true);
    let cr1 = f::cr1::FLTTIME.write(f::cr1::FLTCLK.write(0, true), config.filter as u32);
    Ok((cr0, cr1))
}
/// Polling comparator. All input resources remain owned/borrowed for its life.
/// VC pair reference identity is independent of the shared IRQ13/IRQ24 pairing.
///
/// A live comparator prevents mutation of its DAC threshold.
///
/// VC1 cannot use the divider physically shared by VC3/VC4.
pub struct Comparator<'d, I: VcInstance> {
    _instance: Peri<'d, I>,
    _positive: Peri<'d, AnyPin>,
    _negative: Option<Peri<'d, AnyPin>>,
    _reference: Option<&'d RefDivider<'d, I::Reference>>,
    _dac: Option<&'d Dac<'d>>,
    _bandgap: &'d Bandgap<'d>,
}
impl<'d, I: VcInstance> Comparator<'d, I> {
    pub fn external<P: SignalPin<I, PCH>, N: SignalPin<I, NCH>, const PCH: u8, const NCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        // Only CH0/CH1 are connected to the negative input mux.
        if NCH > 1 {
            return Err(Error::InvalidSignal);
        }
        Self::build(
            instance,
            positive.into(),
            PCH,
            Some(negative.into()),
            NCH,
            None,
            None,
            bandgap,
            config,
            delay,
        )
    }
    pub fn with_reference<P: SignalPin<I, PCH>, const PCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        reference: &'d RefDivider<'d, I::Reference>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        Self::build(
            instance,
            positive.into(),
            PCH,
            None,
            3,
            Some(reference),
            None,
            bandgap,
            config,
            delay,
        )
    }
    /// VC1/3 use internal DAC channel 1, VC2/4 use channel 2. Both DAC channels
    /// are enabled by Dac::new. Set the threshold before borrowing the DAC here.
    pub fn with_dac<P: SignalPin<I, PCH>, const PCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        dac: &'d Dac<'d>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        Self::build(
            instance,
            positive.into(),
            PCH,
            None,
            2,
            None,
            Some(dac),
            bandgap,
            config,
            delay,
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn build(
        instance: Peri<'d, I>,
        positive: Peri<'d, AnyPin>,
        pch: u8,
        negative: Option<Peri<'d, AnyPin>>,
        nch: u8,
        reference: Option<&'d RefDivider<'d, I::Reference>>,
        dac: Option<&'d Dac<'d>>,
        bandgap: &'d Bandgap<'d>,
        config: ComparatorConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let (cr0, cr1) = comparator_words(pch, nch, config)?;
        I::enable_and_reset();
        let r = I::regs();
        // Configure this VC only: no shared resets or reference-register writes.
        unsafe {
            r.cr0().write(0);
            r.cr2().write(0);
            r.cr1().write(cr1);
            r.sr().write(0);
        }
        positive.configure_analog();
        if let Some(pin) = &negative {
            pin.configure_analog();
        }
        unsafe {
            r.cr0().write(cr0);
        }
        delay.delay_us(32);
        Ok(Self {
            _instance: instance,
            _positive: positive,
            _negative: negative,
            _reference: reference,
            _dac: dac,
            _bandgap: bandgap,
        })
    }
    /// Add IRQ-backed one-shot waits, preserving every analog resource borrow.
    /// The binding must dispatch this instance's handler on its physical shared IRQ.
    /// Shared NVIC state is never disabled or unpended by this driver.
    pub fn into_async(
        self,
        _irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
    ) -> AsyncComparator<'d, I> {
        use crate::interrupt::typelevel::Interrupt;
        critical_section::with(|_| {
            let mut io = VcHardware::<I>(core::marker::PhantomData);
            io.disable();
            io.clear();
            I::state().reset();
            // SAFETY: Binding proves our handler is installed in the physical vector.
            unsafe { I::Interrupt::enable() };
        });
        AsyncComparator { inner: self }
    }
    pub fn is_high(&self) -> bool {
        unsafe { pac::vc::fields::sr::FLTV.read(I::regs().sr().read()) }
    }
    /// Identifier only. Installing a VC->ATIM break route belongs to ATIM;
    /// reading this comparator does not itself implement an overcurrent trip.
    pub const fn number(&self) -> u8 {
        I::NUMBER
    }
}
impl<I: VcInstance> Drop for Comparator<'_, I> {
    fn drop(&mut self) {
        unsafe {
            I::regs().cr0().write(0);
            I::regs().cr2().write(0);
            I::regs().cr1().write(0x10);
        }
        self._positive.disconnect();
        if let Some(pin) = &self._negative {
            pin.disconnect();
        }
    }
}

/// One VC instance's share of VC13 or VC24. Bind every async instance used by
/// the application to its actual shared vector; each handler checks only its
/// own IE/INTF and never clears or disables its sibling.
pub struct InterruptHandler<I: VcInstance>(core::marker::PhantomData<I>);
impl<I: VcInstance> crate::interrupt::typelevel::Handler<I::Interrupt> for InterruptHandler<I> {
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|_| {
            service_vc_interrupt(&mut VcHardware::<I>(core::marker::PhantomData), I::state())
        });
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// IRQ-backed, cancel-safe comparator event waits. Edges are relative to the
/// first poll, after stale flags are cleared and interrupt selection is enabled.
/// One hardware INTF coalesces multiple edges; this API is not an edge counter.
/// Rising/falling refer to the filtered, configured-polarity comparator output.
/// There is no busy polling, DMA stream, or automatic hardware motor-trip route.
///
/// Missing a Binding cannot enable interrupts.
/// A live wait exclusively borrows this comparator.
pub struct AsyncComparator<'d, I: VcInstance> {
    inner: Comparator<'d, I>,
}
impl<'d, I: VcInstance> AsyncComparator<'d, I> {
    pub fn is_high(&self) -> bool {
        self.inner.is_high()
    }
    pub const fn number(&self) -> u8 {
        I::NUMBER
    }
    /// Wait for a new rising edge. Earlier/stale INTF is deliberately discarded.
    pub async fn wait_for_rising_edge(&mut self) {
        VcWait::new(&mut self.inner, WaitKind::Rising).await
    }
    /// Wait for a new falling edge. Multiple edges may coalesce in hardware.
    pub async fn wait_for_falling_edge(&mut self) {
        VcWait::new(&mut self.inner, WaitKind::Falling).await
    }
    /// Wait for either edge; INTF does not retain which edge occurred.
    pub async fn wait_for_any_edge(&mut self) {
        VcWait::new(&mut self.inner, WaitKind::AnyEdge).await
    }
    /// Complete immediately if already high, otherwise await a high event.
    /// The level can change again before the awaiting task resumes.
    pub async fn wait_for_high(&mut self) {
        VcWait::new(&mut self.inner, WaitKind::High).await
    }
    /// Complete immediately if already low, otherwise await a falling event.
    /// The level can change again before the awaiting task resumes.
    pub async fn wait_for_low(&mut self) {
        VcWait::new(&mut self.inner, WaitKind::Low).await
    }
}
impl<I: VcInstance> Drop for AsyncComparator<'_, I> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            let mut io = VcHardware::<I>(core::marker::PhantomData);
            io.disable();
            io.clear();
            I::state().reset();
        });
        // The owned Comparator then shuts down and disconnects its own pins.
    }
}

#[derive(Clone, Copy)]
enum WaitKind {
    Rising,
    Falling,
    AnyEdge,
    High,
    Low,
}
impl WaitKind {
    fn bits(self) -> u32 {
        use pac::vc::fields::cr1;
        match self {
            Self::Rising => cr1::RISEIE.mask(),
            Self::Falling | Self::Low => cr1::FALLIE.mask(),
            Self::AnyEdge => cr1::RISEIE.mask() | cr1::FALLIE.mask(),
            Self::High => cr1::HIGHIE.mask(),
        }
    }
    fn satisfied(self, high: bool) -> bool {
        matches!((self, high), (Self::High, true) | (Self::Low, false))
    }
}
const VC_EVENT: u32 = 1;
fn vc_select_mask() -> u32 {
    use pac::vc::fields::cr1;
    cr1::HIGHIE.mask() | cr1::RISEIE.mask() | cr1::FALLIE.mask()
}
trait VcIo {
    fn disable(&mut self);
    fn clear(&mut self);
    fn arm(&mut self, selection: u32);
    fn pending(&mut self) -> bool;
    fn high(&mut self) -> bool;
}
struct VcHardware<I: VcInstance>(core::marker::PhantomData<I>);
impl<I: VcInstance> VcIo for VcHardware<I> {
    fn disable(&mut self) {
        use pac::vc::fields as f;
        unsafe {
            let r = I::regs();
            r.cr0().write(f::cr0::IE.write(r.cr0().read(), false));
            r.cr1().write(r.cr1().read() & !vc_select_mask());
        }
    }
    fn clear(&mut self) {
        // INTF is RW0; FLTV is RO. Write zero, never RMW this mixed register.
        unsafe { I::regs().sr().write(0) };
    }
    fn arm(&mut self, selection: u32) {
        use pac::vc::fields as f;
        unsafe {
            let r = I::regs();
            r.cr1()
                .write((r.cr1().read() & !vc_select_mask()) | selection);
            r.cr0().write(f::cr0::IE.write(r.cr0().read(), true));
        }
    }
    fn pending(&mut self) -> bool {
        use pac::vc::fields as f;
        unsafe {
            f::cr0::IE.read(I::regs().cr0().read()) && f::sr::INTF.read(I::regs().sr().read())
        }
    }
    fn high(&mut self) -> bool {
        unsafe { pac::vc::fields::sr::FLTV.read(I::regs().sr().read()) }
    }
}
fn service_vc_interrupt(
    io: &mut impl VcIo,
    state: &crate::async_support::EventState,
) -> Option<core::task::Waker> {
    if io.pending() {
        // Disable only this source before RW0 acknowledgment, especially for
        // HIGHIE whose level could otherwise immediately reassert INTF.
        io.disable();
        io.clear();
        state.latch(VC_EVENT)
    } else {
        None
    }
}
struct VcWaitCore {
    armed: bool,
    done: bool,
    kind: WaitKind,
}
impl VcWaitCore {
    fn new(kind: WaitKind) -> Self {
        Self {
            armed: false,
            done: false,
            kind,
        }
    }
    fn poll(
        &mut self,
        io: &mut impl VcIo,
        state: &crate::async_support::EventState,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        use core::task::Poll;
        assert!(!self.done, "completed comparator future polled again");
        critical_section::with(|_| {
            if !self.armed {
                io.disable();
                io.clear();
                state.reset();
                state.register(cx.waker());
                // Current-level waits also perform a post-enable check below.
                io.arm(self.kind.bits());
                self.armed = true;
            } else {
                state.register(cx.waker());
            }
            // Registration precedes every latch read. A hardware event between
            // arm and this check is either observed here or remains IRQ-pending.
            let signaled = state.take() & VC_EVENT != 0;
            if signaled || io.pending() || self.kind.satisfied(io.high()) {
                io.disable();
                io.clear();
                state.reset();
                self.armed = false;
                self.done = true;
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
    }
    fn cancel(&mut self, io: &mut impl VcIo, state: &crate::async_support::EventState) {
        if self.armed {
            critical_section::with(|_| {
                io.disable();
                io.clear();
                state.reset();
                self.armed = false;
            });
        }
    }
}
struct VcWait<'a, 'd, I: VcInstance> {
    _driver: &'a mut Comparator<'d, I>,
    core: VcWaitCore,
}
impl<'a, 'd, I: VcInstance> VcWait<'a, 'd, I> {
    fn new(driver: &'a mut Comparator<'d, I>, kind: WaitKind) -> Self {
        Self {
            _driver: driver,
            core: VcWaitCore::new(kind),
        }
    }
}
impl<I: VcInstance> core::future::Future for VcWait<'_, '_, I> {
    type Output = ();
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        self.get_mut().core.poll(
            &mut VcHardware::<I>(core::marker::PhantomData),
            I::state(),
            cx,
        )
    }
}
impl<I: VcInstance> Drop for VcWait<'_, '_, I> {
    fn drop(&mut self) {
        self.core
            .cancel(&mut VcHardware::<I>(core::marker::PhantomData), I::state());
    }
}

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
) -> Result<u32, Error> {
    if !(1..=4).contains(&pch) || config.bias > 7 {
        return Err(Error::InvalidSignal);
    }
    if nch.is_some_and(|n| n != 11 && n != 12) {
        return Err(Error::InvalidSignal);
    }
    use pac::opa::fields::cr as f;
    let mut word = f::BIAS.write(0, config.bias.into());
    word = f::AMP.write(word, gain as u32);
    word = f::MODE.write(word, mode.into());
    word = match pch {
        1 => f::INP1EN.write(word, true),
        2 => f::INP2EN.write(word, true),
        3 => f::INP3EN.write(word, true),
        4 => f::INP4EN.write(word, true),
        _ => unreachable!(),
    };
    word = match nch {
        Some(11) => f::INN1EN.write(word, true),
        Some(12) => f::INN2EN.write(word, true),
        _ => word,
    };
    Ok(f::EN.write(word, true))
}
/// Triggered calibration configuration. Field encoding is explicit because the
/// SDK's period comments are twice the CSR table's values. RM: duration is
/// 8*2^period_code OPACLK periods; AZRUN stays set twice that duration.
#[derive(Clone, Copy, Debug)]
pub struct Calibration {
    pub divider_log2: u8,
    pub period_code: u8,
}
fn calibration_word(config: Calibration) -> Result<u32, Error> {
    if config.divider_log2 > 7 || config.period_code > 15 {
        return Err(Error::InvalidCalibration);
    }
    use pac::opa::fields::cal as f;
    Ok(f::CALEN.write(
        f::CALPERIOD.write(
            f::CLKDIV.write(0, config.divider_log2.into()),
            config.period_code.into(),
        ),
        true,
    ))
}
/// Single OPA with one positive route and, in external mode, one negative route.
/// Output pin is always owned, preventing a simultaneous DAC output on that pad.
///
/// A DAC-backed OPA keeps its source immutable until the OPA is dropped.
pub struct Opa<'d, I: OpaInstance> {
    _instance: Peri<'d, I>,
    _positive: Option<Peri<'d, AnyPin>>,
    _negative: Option<Peri<'d, AnyPin>>,
    _output: Peri<'d, AnyPin>,
    _bandgap: &'d Bandgap<'d>,
    _dac: Option<&'d Dac<'d>>,
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
            None,
            config,
            delay,
        )
    }
    /// Internal DAC1 -> OPA1 / DAC2 -> OPA2, with an externally owned OPA output.
    pub fn dac_follower<O: SignalPin<I, 0>>(
        instance: Peri<'d, I>,
        dac: &'d Dac<'d>,
        output: Peri<'d, O>,
        bandgap: &'d Bandgap<'d>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
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
        dac: Option<&'d Dac<'d>>,
        config: OpaConfig,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let word = opa_word(pch, nch, mode, gain, config)?;
        I::enable_and_reset();
        let r = I::regs();
        unsafe {
            r.cr().write(0xe000);
            r.cal().write(0);
        }
        if let Some(pin) = &positive {
            pin.configure_analog();
        }
        if let Some(pin) = &negative {
            pin.configure_analog();
        }
        output.configure_analog();
        unsafe {
            r.cr().write(word);
        }
        delay.delay_us(40);
        Ok(Self {
            _instance: instance,
            _positive: positive,
            _negative: negative,
            _output: output,
            _bandgap: bandgap,
            _dac: dac,
        })
    }
    /// Software-triggered bounded calibration. Output is invalid during this
    /// call. A timeout leaves calibration running; do not use output until a
    /// subsequent completion or recreate the disabled peripheral after drop.
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
        use pac::opa::fields::cal as f;
        let r = I::regs();
        if unsafe { f::AZRUN.read(r.cal().read()) } {
            return Err(Error::Busy);
        }
        // Do not RMW the trigger/status register: SOFTTRIG is a write-one command.
        unsafe {
            r.cal().write(word);
            r.cal().write(f::SOFTTRIG.write(word, true));
        }
        wait_calibration(poll_budget, || unsafe { f::AZRUN.read(r.cal().read()) })?;
        delay.delay_us(10);
        Ok(())
    }
    pub fn is_calibrating(&self) -> bool {
        unsafe { pac::opa::fields::cal::AZRUN.read(I::regs().cal().read()) }
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
        unsafe {
            I::regs().cr().write(0xe000);
            I::regs().cal().write(0);
        }
        if let Some(pin) = &self._positive {
            pin.disconnect();
        }
        if let Some(pin) = &self._negative {
            pin.disconnect();
        }
        self._output.disconnect();
    }
}
