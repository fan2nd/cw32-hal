//! CW32F030 VC1/VC2 external-input comparators (RM Rev2.5 chapter 23).
//!
//! Each comparator owns both external input pins. Internal BGR/ADC references,
//! the physically shared DIV circuit, window mode and output pins are deliberately
//! not exposed by this driver until their shared-resource ownership is modeled.
use crate::{
    gpio::{AnyPin, Pin},
    pac, peripherals,
    rcc::PeripheralClock,
    Async, Blocking, Mode, Peri, PeripheralType,
};
mod sealed {
    pub trait Pin<I, const SIGNAL: u8> {}
    pub(crate) trait VcInstance {
        fn regs() -> crate::pac::vc::Vc;
        fn state() -> &'static crate::async_support::EventState;
    }
}
#[allow(private_bounds)]
pub trait VcInstance: sealed::VcInstance + PeripheralType + PeripheralClock {
    type Interrupt: crate::interrupt::typelevel::Interrupt;
    const NUMBER: u8;
}
pub trait SignalPin<I: VcInstance, const SIGNAL: u8>: sealed::Pin<I, SIGNAL> + Pin {}
include!(concat!(env!("OUT_DIR"), "/_generated_analog.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidSignal,
    Disabled,
    InvalidConfig,
    NotReady,
    OutputsEnabled,
    RouteInUse,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Response {
    ExtraLow = 0,
    Low = 1,
    Medium = 2,
    High = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hysteresis {
    Off = 0,
    Mv10 = 1,
    Mv20 = 2,
    Mv30 = 3,
}
/// PCLK clocked digital filter. Values are hardware encodings, not microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filter {
    Cycles1 = 0,
    Cycles3,
    Cycles7,
    Cycles15,
    Cycles63,
    Cycles255,
    Cycles1023,
    Cycles4095,
}
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ComparatorConfig {
    pub response: Response,
    pub hysteresis: Hysteresis,
    pub inverted: bool,
    pub filter: Option<Filter>,
    /// Bounded register polling during startup; not a wall-clock timeout.
    pub ready_poll_limit: u32,
}
impl Default for ComparatorConfig {
    fn default() -> Self {
        Self {
            response: Response::High,
            hysteresis: Hysteresis::Off,
            inverted: false,
            filter: None,
            ready_poll_limit: 0xffff,
        }
    }
}
fn comparator_words(
    positive: u8,
    negative: u8,
    config: ComparatorConfig,
) -> Result<(pac::vc::regs::Cr0, pac::vc::regs::Cr1), Error> {
    if positive > 7 || negative > 7 {
        return Err(Error::InvalidSignal);
    }
    if config.ready_poll_limit == 0 {
        return Err(Error::InvalidConfig);
    }
    let mut cr0 = pac::vc::regs::Cr0(0);
    cr0.set_inp(positive);
    cr0.set_inn(negative);
    cr0.set_resp(config.response as u8);
    cr0.set_hys(config.hysteresis as u8);
    cr0.set_pol(config.inverted);
    cr0.set_en(false);
    let mut cr1 = pac::vc::regs::Cr1(0);
    cr1.set_fltclk(true);
    if let Some(filter) = config.filter {
        cr1.set_flttime(filter as u8);
        cr1.set_flten(true);
    }
    Ok((cr0, cr1))
}
/// External differential comparator, configured but initially disabled.
///
/// Both modes own the same peripheral and input pins. `Async` also proves the
/// interrupt binding and supports one-shot waits. This owner never writes the
/// shared divider, ADC reference or bandgap control.
pub struct Comp<'d, I: VcInstance, M: Mode> {
    _instance: Peri<'d, I>,
    _positive: Peri<'d, AnyPin>,
    _negative: Peri<'d, AnyPin>,
    ready_poll_limit: u32,
    _mode: core::marker::PhantomData<M>,
}
impl<'d, I: VcInstance> Comp<'d, I, Blocking> {
    /// Configure external inputs without enabling the comparator. Call `enable`
    /// to start it and perform bounded hardware READY polling before reading it.
    pub fn new_blocking<
        P: SignalPin<I, PCH>,
        N: SignalPin<I, NCH>,
        const PCH: u8,
        const NCH: u8,
    >(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        Self::build(instance, positive, negative, config)
    }
}
impl<'d, I: VcInstance> Comp<'d, I, Async> {
    /// Configure external inputs and the interrupt binding without enabling the
    /// comparator. Call `enable` before reading or waiting for its output.
    pub fn new<P: SignalPin<I, PCH>, N: SignalPin<I, NCH>, const PCH: u8, const NCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        _irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        use crate::interrupt::typelevel::Interrupt;
        let comp = Self::build(instance, positive, negative, config)?;
        // Construction disabled/cleared this source and its software state.
        // Binding proves our handler is installed. Do not unpend a shared vector.
        unsafe { I::Interrupt::enable() };
        Ok(comp)
    }
}
impl<'d, I: VcInstance, M: Mode> Comp<'d, I, M> {
    fn build<P: SignalPin<I, PCH>, N: SignalPin<I, NCH>, const PCH: u8, const NCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        let (cr0, cr1) = comparator_words(PCH, NCH, config)?;
        I::enable_and_reset(); // Generated clocks never assert shared VC/BGR reset.
        let positive: Peri<'d, AnyPin> = positive.into();
        let negative: Peri<'d, AnyPin> = negative.into();
        positive.configure_analog();
        negative.configure_analog();
        let r = I::regs();
        critical_section::with(|_| {
            r.cr0().write_value(pac::vc::regs::Cr0(0));
            r.cr1().write_value(cr1);
            r.sr().write_value(pac::vc::regs::Sr(0));
            I::state().reset();
        });
        r.cr0().write_value(cr0);
        Ok(Self {
            _instance: instance,
            _positive: positive,
            _negative: negative,
            ready_poll_limit: config.ready_poll_limit,
            _mode: core::marker::PhantomData,
        })
    }
    /// Enable and wait for hardware READY with the configured register-poll
    /// budget. A timeout disables this VC and its event source; its sibling and
    /// the shared reference stay untouched. The same owner can be enabled again.
    pub fn enable(&mut self) -> Result<(), Error> {
        I::regs().cr0().modify(|w| w.set_en(true));
        for _ in 0..self.ready_poll_limit {
            if I::regs().sr().read().ready() {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        self.disable();
        Err(Error::NotReady)
    }
    /// Disable this comparator and its event source, preserving input/filter
    /// configuration. If a brake guard was forgotten, power outputs are disabled
    /// before releasing its route. Shared clock/BGR/NVIC state is left alone.
    pub fn disable(&mut self) {
        critical_section::with(|_| {
            let mut io = VcHardware::<I>(core::marker::PhantomData);
            io.disable();
            shutdown_comparator(&mut ShutdownHardware::<I>(core::marker::PhantomData));
            io.clear();
            I::state().reset();
        });
    }
    pub fn is_enabled(&self) -> bool {
        I::regs().cr0().read().en()
    }
    /// Read the filtered, configured-polarity output only while enabled/ready.
    pub fn output_level(&self) -> Result<bool, Error> {
        if !self.is_enabled() {
            return Err(Error::Disabled);
        }
        let status = I::regs().sr().read();
        if !status.ready() {
            return Err(Error::NotReady);
        }
        Ok(status.fltv())
    }
    pub const fn number(&self) -> u8 {
        I::NUMBER
    }
    /// Route this ready comparator to ATIM hardware brake while borrowing both
    /// resources. Outputs must already be disabled. This is a wiring mechanism,
    /// not a certified motor-protection function; validate polarity and latency.
    pub fn atim_break<'a, 'p>(
        &'a self,
        pwm: &'a mut crate::atim::ThreePhasePwm<'p>,
    ) -> Result<ComparatorBrake<'a, 'd, 'p, I, M>, Error> {
        self.output_level()?;
        critical_section::with(|_| {
            validate_brake_route(
                pwm.outputs_enabled(),
                pac::ATIM.dtr().read().vce(),
                // ATIM reset clears VCE but not a leaked VC's physical route.
                pac::VC1.cr1().read().atimbk() || pac::VC2.cr1().read().atimbk(),
            )?;
            I::regs().cr1().modify(|w| w.set_atimbk(true));
            pwm.set_comparator_brake(true);
            Ok(())
        })?;
        Ok(ComparatorBrake { pwm, _source: self })
    }
}
fn validate_brake_route(
    outputs: bool,
    brake_in_use: bool,
    source_in_use: bool,
) -> Result<(), Error> {
    if outputs {
        return Err(Error::OutputsEnabled);
    }
    if brake_in_use || source_in_use {
        return Err(Error::RouteInUse);
    }
    Ok(())
}
/// Owns the route's lifetime. Dropping it first disables outputs, then removes
/// the comparator route. The counter remains available to the time/ADC logic.
///
/// The protection source cannot be dropped while its route is in use.
/// Direct access to the PWM owner cannot overlap its route guard.
pub struct ComparatorBrake<'a, 'd, 'p, I: VcInstance, M: Mode> {
    pwm: &'a mut crate::atim::ThreePhasePwm<'p>,
    _source: &'a Comp<'d, I, M>,
}
impl<'a, 'd, 'p, I: VcInstance, M: Mode> ComparatorBrake<'a, 'd, 'p, I, M> {
    pub fn pwm(&mut self) -> &mut crate::atim::ThreePhasePwm<'p> {
        self.pwm
    }
}
impl<I: VcInstance, M: Mode> Drop for ComparatorBrake<'_, '_, '_, I, M> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            self.pwm.disable_outputs();
            self.pwm.set_comparator_brake(false);
            I::regs().cr1().modify(|w| w.set_atimbk(false));
        });
    }
}
impl<I: VcInstance, M: Mode> Drop for Comp<'_, I, M> {
    fn drop(&mut self) {
        // A safe caller can mem::forget a route guard. Inspect the physical
        // route as well as normal borrow/Drop ordering before removing its source.
        self.disable();
        I::regs().cr0().write_value(pac::vc::regs::Cr0(0));
        I::regs().cr1().write_value(pac::vc::regs::Cr1(0));
        self._positive.disconnect();
        self._negative.disconnect();
    }
}
trait ShutdownIo {
    fn routed(&mut self) -> bool;
    fn disable_power(&mut self);
    fn disable_comparator(&mut self);
}
fn shutdown_comparator(io: &mut impl ShutdownIo) {
    if io.routed() {
        io.disable_power();
    }
    io.disable_comparator();
}
struct ShutdownHardware<I: VcInstance>(core::marker::PhantomData<I>);
impl<I: VcInstance> ShutdownIo for ShutdownHardware<I> {
    fn routed(&mut self) -> bool {
        I::regs().cr1().read().atimbk()
    }
    fn disable_power(&mut self) {
        // Only one safe route can be installed. Release the leaked route
        // together with its power state so a surviving PWM can be reused.
        pac::ATIM.dtr().modify(|w| {
            w.set_moe(false);
            w.set_aoe(false);
            w.set_vce(false);
        });
    }
    fn disable_comparator(&mut self) {
        // Preserve analog input/filter configuration across explicit disable.
        I::regs().cr1().modify(|w| w.set_atimbk(false));
        I::regs().cr0().modify(|w| w.set_en(false));
    }
}

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

/// IRQ-backed, cancel-safe one-shot waits. Edges are relative to the first poll,
/// after stale flags are cleared and interrupt selection is enabled. Hardware
/// coalesces edges; these methods are notifications, not an edge counter.
///
/// Enable the comparator first. A disabled comparator returns `Error::Disabled`
/// rather than enabling analog circuitry without its required startup checks.
/// Cancellation clears only this VC's event source; it leaves the comparator
/// enabled and never disables or unpends a shared NVIC vector.
impl<I: VcInstance> Comp<'_, I, Async> {
    /// Wait for a new rising edge of the filtered, configured-polarity output.
    pub async fn wait_for_rising_edge(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::Rising).await
    }
    /// Wait for a new falling edge. Multiple edges may coalesce in hardware.
    pub async fn wait_for_falling_edge(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::Falling).await
    }
    /// Wait for either edge; INTF does not retain which edge occurred.
    pub async fn wait_for_any_edge(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::AnyEdge).await
    }
    /// Complete immediately if already high, otherwise await a high event.
    /// The level can change again before the awaiting task resumes.
    pub async fn wait_for_high(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::High).await
    }
    /// Complete immediately if already low, otherwise await a falling event.
    /// The level can change again before the awaiting task resumes.
    pub async fn wait_for_low(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::Low).await
    }
    async fn wait(&mut self, kind: WaitKind) -> Result<(), Error> {
        self.output_level()?;
        VcWait::new(self, kind).await;
        Ok(())
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
    fn satisfied(self, high: bool) -> bool {
        matches!((self, high), (Self::High, true) | (Self::Low, false))
    }
}
const VC_EVENT: u32 = 1;
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
    fn arm(&mut self, selection: u32) {
        let r = I::regs();
        let selection = pac::vc::regs::Cr1(selection);
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
    _driver: &'a mut Comp<'d, I, Async>,
    core: VcWaitCore,
}
impl<'a, 'd, I: VcInstance> VcWait<'a, 'd, I> {
    fn new(driver: &'a mut Comp<'d, I, Async>, kind: WaitKind) -> Self {
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
