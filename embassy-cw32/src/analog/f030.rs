//! CW32F030 VC1/VC2 external-input comparators (RM Rev2.5 chapter 23).
//!
//! Each comparator owns both external input pins. Internal BGR/ADC references,
//! the physically shared DIV circuit, window mode and output pins are deliberately
//! not exposed by this driver until their shared-resource ownership is modeled.
use crate::{
    gpio::{AnyPin, Pin},
    pac, peripherals,
    rcc::PeripheralClock,
    Peri, PeripheralType,
};
use pac::vc::fields as f;
mod sealed {
    pub trait Pin<I, const SIGNAL: u8> {}
    pub(crate) trait VcInstance {
        fn regs() -> crate::pac::vc::RegisterBlock;
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
) -> Result<(u32, u32), Error> {
    if positive > 7 || negative > 7 {
        return Err(Error::InvalidSignal);
    }
    if config.ready_poll_limit == 0 {
        return Err(Error::InvalidConfig);
    }
    let mut cr0 = f::cr0::INP.write(0, positive.into());
    cr0 = f::cr0::INN.write(cr0, negative.into());
    cr0 = f::cr0::RESP.write(cr0, config.response as u32);
    cr0 = f::cr0::HYS.write(cr0, config.hysteresis as u32);
    cr0 = f::cr0::POL.write(cr0, config.inverted);
    cr0 = f::cr0::EN.write(cr0, true);
    let mut cr1 = f::cr1::FLTCLK.write(0, true);
    if let Some(filter) = config.filter {
        cr1 = f::cr1::FLTTIME.write(cr1, filter as u32);
        cr1 = f::cr1::FLTEN.write(cr1, true);
    }
    Ok((cr0, cr1))
}
/// External differential comparator. No shared divider, ADC or bandgap writes.
pub struct Comparator<'d, I: VcInstance> {
    _instance: Peri<'d, I>,
    _positive: Peri<'d, AnyPin>,
    _negative: Peri<'d, AnyPin>,
}
impl<'d, I: VcInstance> Comparator<'d, I> {
    pub fn external<P: SignalPin<I, PCH>, N: SignalPin<I, NCH>, const PCH: u8, const NCH: u8>(
        instance: Peri<'d, I>,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
        config: ComparatorConfig,
    ) -> Result<Self, Error> {
        let (cr0, cr1) = comparator_words(PCH, NCH, config)?;
        I::enable_and_reset(); // Shared VC reset is never asserted by generated clocks.
        let positive: Peri<'d, AnyPin> = positive.into();
        let negative: Peri<'d, AnyPin> = negative.into();
        positive.configure_analog();
        negative.configure_analog();
        let r = I::regs();
        unsafe {
            r.cr0().write(0);
            r.cr1().write(cr1);
            r.sr().write(0);
            r.cr0().write(cr0);
        }
        let comparator = Self {
            _instance: instance,
            _positive: positive,
            _negative: negative,
        };
        for _ in 0..config.ready_poll_limit {
            if unsafe { f::sr::READY.read(r.sr().read()) } {
                return Ok(comparator);
            }
            core::hint::spin_loop();
        }
        // Owned temporary drops and disables this VC, without touching its sibling.
        Err(Error::NotReady)
    }
    pub fn is_high(&self) -> bool {
        unsafe { f::sr::FLTV.read(I::regs().sr().read()) }
    }
    pub const fn number(&self) -> u8 {
        I::NUMBER
    }
    /// Route this live comparator to ATIM hardware brake while borrowing both
    /// resources. Outputs must already be disabled. This is a wiring mechanism,
    /// not a certified motor-protection function; validate polarity and latency.
    pub fn atim_break<'a, 'p>(
        &'a self,
        pwm: &'a mut crate::atim::ThreePhasePwm<'p>,
    ) -> Result<ComparatorBrake<'a, 'd, 'p, I>, Error> {
        critical_section::with(|_| unsafe {
            validate_brake_route(
                pwm.outputs_enabled(),
                pac::atim::fields::dtr::VCE.read(pac::ATIM.dtr().read()),
                // A forgotten guard may outlive an ATIM owner/reinitialization:
                // timer reset clears VCE but not either VC's physical route.
                f::cr1::ATIMBK.read(pac::VC1.cr1().read())
                    || f::cr1::ATIMBK.read(pac::VC2.cr1().read()),
            )?;
            I::regs()
                .cr1()
                .write(f::cr1::ATIMBK.write(I::regs().cr1().read(), true));
            pwm.set_comparator_brake(true);
            Ok(())
        })?;
        Ok(ComparatorBrake { pwm, _source: self })
    }
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
            unsafe { I::Interrupt::enable() };
        });
        AsyncComparator { inner: self }
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
pub struct ComparatorBrake<'a, 'd, 'p, I: VcInstance> {
    pwm: &'a mut crate::atim::ThreePhasePwm<'p>,
    _source: &'a Comparator<'d, I>,
}
impl<'a, 'd, 'p, I: VcInstance> ComparatorBrake<'a, 'd, 'p, I> {
    pub fn pwm(&mut self) -> &mut crate::atim::ThreePhasePwm<'p> {
        self.pwm
    }
}
impl<I: VcInstance> Drop for ComparatorBrake<'_, '_, '_, I> {
    fn drop(&mut self) {
        critical_section::with(|_| unsafe {
            self.pwm.disable_outputs();
            self.pwm.set_comparator_brake(false);
            I::regs()
                .cr1()
                .write(f::cr1::ATIMBK.write(I::regs().cr1().read(), false));
        });
    }
}
impl<I: VcInstance> Drop for Comparator<'_, I> {
    fn drop(&mut self) {
        // A safe caller can mem::forget a route guard. Inspect the physical
        // route as well as normal borrow/Drop ordering before removing its source.
        critical_section::with(|_| {
            shutdown_comparator(&mut ShutdownHardware::<I>(core::marker::PhantomData))
        });
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
        unsafe { f::cr1::ATIMBK.read(I::regs().cr1().read()) }
    }
    fn disable_power(&mut self) {
        use pac::atim::fields::dtr;
        unsafe {
            let r = pac::ATIM.dtr();
            // Only one safe route can be installed. Release the leaked route
            // together with its power state so a surviving PWM can be reused.
            r.write(r.read() & !(dtr::MOE.mask() | dtr::AOE.mask() | dtr::VCE.mask()));
        }
    }
    fn disable_comparator(&mut self) {
        unsafe {
            I::regs().cr0().write(0);
            I::regs().cr1().write(0);
            I::regs().sr().write(0);
        }
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
