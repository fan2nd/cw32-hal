//! CW32F030 single ADC: blocking and real EOS-IRQ sequence sampling (RM 22).
//! VDDA reference, buffered external inputs, 1–4 slots, one shared sample time.
//! ADCCLK is conservatively limited to 500 kHz across the full VDDA range.
//! No dual-ADC, internal-channel or voltage-qualified high-speed API is implied.
//!
//! [`Adc`] retains the blocking API. [`Adc::into_async`] requires a checked
//! interrupt binding and transfers ownership to [`AsyncAdc`]. Async capture is
//! one-shot and cancellation-safe; it is not a DMA or lossless streaming API.
use crate::{
    async_support::EventState,
    gpio::{AnyPin, Pin},
    interrupt, pac, peripherals,
    rcc::PeripheralClock,
    Peri, PeripheralType,
};
use core::{
    future::poll_fn,
    marker::PhantomData,
    task::{Context, Poll},
};
use embedded_hal::delay::DelayNs;
use interrupt::typelevel::Interrupt as _;
use pac::adc::fields as f;
mod sealed {
    pub(crate) trait Sealed {
        fn regs() -> crate::pac::adc::RegisterBlock;
        fn state() -> &'static crate::async_support::EventState;
    }
    pub trait PinSealed<I> {}
}
/// Generated peripheral identity; shared ADC reset is deliberately never asserted.
#[allow(private_bounds)]
pub trait Instance: sealed::Sealed + PeripheralClock + PeripheralType {
    /// The physical ADC vector of this single-ADC device.
    type Interrupt: interrupt::typelevel::Interrupt;
}
pub trait ChannelPin<I: Instance>: sealed::PinSealed<I> + Pin {
    const CHANNEL: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_adc.rs"));
/// Owns the bonded analog input pin. Internal TS/BGR channels are not fabricated as external pins.
pub struct AnalogInput<'d, I: Instance> {
    pin: Peri<'d, AnyPin>,
    channel: u8,
    _instance: PhantomData<I>,
}
impl<'d, I: Instance> AnalogInput<'d, I> {
    pub fn new<P: ChannelPin<I>>(pin: Peri<'d, P>) -> Self {
        let pin = pin.into();
        pin.configure_analog();
        Self {
            pin,
            channel: P::CHANNEL,
            _instance: PhantomData,
        }
    }
}
impl<'d, I: Instance> Drop for AnalogInput<'d, I> {
    fn drop(&mut self) {
        self.pin.disconnect();
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    EmptySequence,
    SequenceTooLong,
    ClockTooFast,
    Timeout,
    Busy,
    InvalidThreshold,
    MixedSampleTimes,
    NotReady,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivider {
    Div1 = 0,
    Div2,
    Div4,
    Div8,
    Div16,
    Div32,
    Div64,
    Div128,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SampleTime {
    Cycles5 = 0,
    Cycles6,
    Cycles8,
    Cycles10,
}
impl SampleTime {
    pub const fn cycles(self) -> u16 {
        [5, 6, 8, 10][self as usize]
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub clock_divider: ClockDivider,
    /// Maximum READY register observations after the initial 40 us delay.
    pub readiness_poll_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            clock_divider: ClockDivider::Div16,
            readiness_poll_limit: 10_000,
        }
    }
}
fn validate<const N: usize>(pclk: u32, config: Config) -> Result<(), Error> {
    if N == 0 {
        return Err(Error::EmptySequence);
    }
    if N > 4 {
        return Err(Error::SequenceTooLong);
    }
    // RM 22.4.2: VDDA=1.65..1.8 V permits ADCCLK <=500 kHz.
    if pclk == 0 || pclk > 500_000u32 * (1u32 << (config.clock_divider as u8)) {
        return Err(Error::ClockTooFast);
    }
    Ok(())
}
fn shared_sample(samples: &[SampleTime]) -> Result<SampleTime, Error> {
    let first = *samples.first().ok_or(Error::EmptySequence)?;
    if samples.iter().any(|sample| *sample != first) {
        Err(Error::MixedSampleTimes)
    } else {
        Ok(first)
    }
}
fn poll_ready(limit: u32, mut ready: impl FnMut() -> bool) -> bool {
    (0..limit).any(|_| ready())
}
/// Retains instance and all sequence pins. Conversion values are uncalibrated 12-bit codes.
pub struct Adc<'d, I: Instance, const N: usize> {
    _instance: Peri<'d, I>,
    _inputs: [AnalogInput<'d, I>; N],
    cr: u32,
}
impl<'d, I: Instance, const N: usize> Adc<'d, I, N> {
    /// All sample-time entries must match: F030 has one shared SAM field.
    /// Uses VDDA as reference and requires READY within the configured limit.
    pub fn new(
        instance: Peri<'d, I>,
        inputs: [AnalogInput<'d, I>; N],
        samples: [SampleTime; N],
        config: Config,
        clocks: &crate::rcc::Clocks,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        validate::<N>(clocks.pclk_hz(), config)?;
        let sample = shared_sample(&samples)?;
        // The generated clock operation only enables the gate: ADC reset also
        // clears the BGR reference used by VC1/VC2 and must never be asserted.
        I::enable_and_reset();
        let r = I::regs();
        let cr = control_word(0, config.clock_divider, sample);
        let channels = inputs.iter().enumerate().fold(0u32, |word, (n, input)| {
            word | (u32::from(input.channel) << (4 * n))
        });
        critical_section::with(|_| unsafe {
            r.trigger().write(0);
            r.start().write(0);
            r.ier().write(0);
            // Preserve BIAS, reserved bits and shared BGREN, even if VC owns it.
            r.cr0().write(r.cr0().read() & !control_mask());
            r.cr1().write(r.cr1().read() & !0x2fefu32);
            r.cr2().write(r.cr2().read() & !0x3ff); // no accumulation or multi-conversion mode
            r.sqr().write(f::sqr::ENS.write(channels, (N - 1) as u32));
            r.icr().write(0x7f & !flags());
            r.cr0().write((r.cr0().read() & !control_mask()) | cr);
        });
        delay.delay_us(40); // RM 22.4.1: analog startup, then READY must be checked.
        let ready = poll_ready(config.readiness_poll_limit, || unsafe {
            f::isr::READY.read(r.isr().read())
        });
        if !ready {
            critical_section::with(|_| unsafe {
                r.cr0().write(r.cr0().read() & !control_mask());
            });
            return Err(Error::NotReady);
        }
        Ok(Self {
            _instance: instance,
            _inputs: inputs,
            cr,
        })
    }
    pub fn busy(&self) -> bool {
        unsafe { f::start::START.read(I::regs().start().read()) }
    }
    /// Transfer this ADC and all of its pins to an IRQ-driven owner.
    ///
    /// Any previously armed trigger or conversion is stopped. The binding must
    /// include [`InterruptHandler<I>`] on this ADC's actual vector.
    /// NVIC pending state is never cleared or disabled here.
    pub fn into_async(
        self,
        _irq: impl interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>>,
    ) -> AsyncAdc<'d, I, N> {
        critical_section::with(|_| {
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
            // SAFETY: the binding proves that this ADC's handler is installed.
            // Enabling a shared vector preserves every other user's pending IRQ.
            unsafe { I::Interrupt::enable() };
        });
        AsyncAdc { inner: self }
    }
    fn prepare(&mut self) -> Result<(), Error> {
        prepare_sequence(&mut Hardware::<I>(PhantomData), self.cr)
    }
    /// Single software-triggered complete sequence. poll_limit counts register observations, not microseconds.
    pub fn sample(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        self.prepare()?;
        unsafe {
            I::regs().start().write(f::start::START.write(0, true));
        }
        self.wait_complete(poll_limit)
    }
    fn wait_complete(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        for _ in 0..poll_limit {
            if unsafe { f::isr::EOS.read(I::regs().isr().read()) } && !self.busy() {
                return Ok(self.results());
            }
            core::hint::spin_loop();
        }
        self.stop();
        Err(Error::Timeout)
    }
    fn results(&self) -> [u16; N] {
        let r = I::regs();
        let regs = [r.result0(), r.result1(), r.result2(), r.result3()];
        core::array::from_fn(|n| unsafe { (regs[n].read() & 0x0fff) as u16 })
    }
    /// Arm ATIM update triggering without starting or changing ATIM.
    /// The F030 PWM owner configures TRIG.UEVE and TRIG.TRIGE.
    pub fn arm_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<(), Error> {
        self.prepare()?;
        unsafe { I::regs().trigger().write(f::trigger::ATIM.mask()) };
        Ok(())
    }
    /// Disarm after EOS and wait for any already-started final scan.
    /// Returns the latest coherent scan, not a lossless first-edge capture.
    pub fn capture_triggered(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        let mut left = poll_limit;
        while left > 0 {
            left -= 1;
            if unsafe { f::isr::EOS.read(I::regs().isr().read()) } {
                unsafe { I::regs().trigger().write(0) };
                return self.wait_complete(left.saturating_add(1));
            }
            core::hint::spin_loop();
        }
        self.stop();
        Err(Error::Timeout)
    }
    pub fn stop(&mut self) {
        unsafe {
            I::regs().trigger().write(0);
            I::regs().start().write(0);
        }
    }
    /// Configure a polling analog watchdog over this sequence's external channels.
    /// Fault means result < low or result >= high (upper threshold is exclusive).
    /// Disarms external triggers before checking idle; Busy leaves the route disarmed.
    pub fn watchdog(&mut self, low: u16, high: u16) -> Result<(), Error> {
        if low > high || high > 4095 {
            return Err(Error::InvalidThreshold);
        }
        disarm_idle(&mut Hardware::<I>(PhantomData))?;
        unsafe {
            I::regs().vtl().write(u32::from(low));
            I::regs().vth().write(u32::from(high));
            let r = I::regs();
            r.cr1().write(f::cr1::WDTALL.write(r.cr1().read(), true));
        }
        Ok(())
    }
    pub fn watchdog_fault(&self) -> bool {
        unsafe { I::regs().isr().read() & (f::isr::WDTL.mask() | f::isr::WDTH.mask()) != 0 }
    }
    pub fn acknowledge_watchdog(&mut self) {
        unsafe {
            I::regs()
                .icr()
                .write(0x7f & !(f::icr::WDTL.mask() | f::icr::WDTH.mask()));
        }
    }
}

/// IRQ-driven owner of one ADC and its complete, fixed input sequence.
///
/// Each operation exclusively borrows this owner until completion or
/// cancellation. Dropping a pending future disables this ADC's EOS interrupt,
/// disarms its trigger, stops its conversion and removes its registered waker.
/// It never disables a shared vector or clears another peripheral's flags.
pub struct AsyncAdc<'d, I: Instance, const N: usize> {
    inner: Adc<'d, I, N>,
}

impl<'d, I: Instance, const N: usize> AsyncAdc<'d, I, N> {
    /// Start a single software-triggered sequence and sleep until its EOS IRQ.
    ///
    /// There is no internal timeout. An executor's timeout/select can cancel
    /// this future safely by dropping it.
    pub async fn sample(&mut self) -> Result<[u16; N], Error> {
        self.convert(0).await
    }

    /// Await an ATIM update-triggered complete scan. The route is armed on
    /// first poll. EOS disarms it and waits for any already-started final scan.
    /// Does not start the timer; cancellation also works if no trigger arrives.
    pub async fn sample_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<[u16; N], Error> {
        self.convert(f::trigger::ATIM.mask()).await
    }

    async fn convert(&mut self, trigger: u32) -> Result<[u16; N], Error> {
        let _guard = ConversionGuard::start(
            Hardware::<I>(PhantomData),
            I::state(),
            self.inner.cr,
            trigger,
        )?;
        poll_fn(|cx| poll_sequence(I::state(), cx)).await;
        // The ISR has disabled triggers and observed START=0. Results cannot
        // change while they are copied, even if the task was delayed after EOS.
        Ok(self.inner.results())
    }

    /// Return the same ADC and pins to the blocking API.
    pub fn into_blocking(self) -> Adc<'d, I, N> {
        critical_section::with(|_| {
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
        });
        self.inner
    }

    /// Configure the polling analog watchdog while no async conversion is held.
    pub fn watchdog(&mut self, low: u16, high: u16) -> Result<(), Error> {
        self.inner.watchdog(low, high)
    }

    pub fn watchdog_fault(&self) -> bool {
        self.inner.watchdog_fault()
    }

    pub fn acknowledge_watchdog(&mut self) {
        self.inner.acknowledge_watchdog();
    }
}

/// ADC sequence-complete interrupt handler for a verified per-instance binding.
///
/// Services only the enabled EOS source; preserves unrelated ADC flags and NVIC state.
pub struct InterruptHandler<I: Instance> {
    _instance: PhantomData<I>,
}

impl<I: Instance> interrupt::typelevel::Handler<I::Interrupt> for InterruptHandler<I> {
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|_| {
            if service_eos(&mut Hardware::<I>(PhantomData)) {
                // Keep completion publication indivisible with hardware cleanup
                // even if an enclosing executor can run in another interrupt.
                I::state().latch(SEQUENCE_COMPLETE)
            } else {
                None
            }
        });
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

const SEQUENCE_COMPLETE: u32 = 1;

fn poll_sequence(state: &EventState, cx: &mut Context<'_>) -> Poll<()> {
    // Register first, then consume the latch: completion before registration,
    // during registration, or after the check cannot be lost.
    state.register(cx.waker());
    if state.take() & SEQUENCE_COMPLETE != 0 {
        Poll::Ready(())
    } else {
        Poll::Pending
    }
}

trait AsyncSequenceIo: SequenceIo {
    fn eos_interrupt_enabled(&mut self) -> bool;
    fn eos_pending(&mut self) -> bool;
    fn eos_interrupt(&mut self, enabled: bool);
    fn clear_eos(&mut self);
    fn start(&mut self, enabled: bool);
    fn trigger(&mut self, mask: u32);
}

impl<I: Instance> AsyncSequenceIo for Hardware<I> {
    fn eos_interrupt_enabled(&mut self) -> bool {
        unsafe { f::ier::EOS.read(I::regs().ier().read()) }
    }
    fn eos_pending(&mut self) -> bool {
        unsafe { f::isr::EOS.read(I::regs().isr().read()) }
    }
    fn eos_interrupt(&mut self, enabled: bool) {
        unsafe {
            let r = I::regs();
            r.ier().write(f::ier::EOS.write(r.ier().read(), enabled));
        }
    }
    fn clear_eos(&mut self) {
        // ADC ICR is R1W0, not W1C (RM 22.13.11). Preserve EOC and AWD flags.
        unsafe { I::regs().icr().write(0x7f & !f::icr::EOS.mask()) };
    }
    fn start(&mut self, enabled: bool) {
        unsafe { I::regs().start().write(f::start::START.write(0, enabled)) };
    }
    fn trigger(&mut self, mask: u32) {
        unsafe { I::regs().trigger().write(mask) };
    }
}

fn start_async_sequence(
    io: &mut impl AsyncSequenceIo,
    state: &EventState,
    cr: u32,
    trigger: u32,
) -> Result<(), Error> {
    disarm_idle(io)?;
    io.eos_interrupt(false);
    state.reset();
    io.control(cr);
    io.clear_eos();
    io.eos_interrupt(true);
    if trigger == 0 {
        io.start(true);
    } else {
        io.trigger(trigger);
    }
    Ok(())
}

fn service_eos(io: &mut impl AsyncSequenceIo) -> bool {
    if !io.eos_interrupt_enabled() || !io.eos_pending() {
        return false;
    }
    io.disarm();
    // Clear BEFORE checking START. A late sequence can finish while the CPU
    // runs even inside a critical section. Clearing after reading busy could
    // erase its final EOS and leave a sleeping future with no remaining IRQ.
    io.clear_eos();
    // MODE=4: RM 22.5.5 guarantees EOS and automatic START clear at sequence
    // completion. With the route disarmed, at most one sequence remains.
    if io.busy() {
        return false;
    }
    io.eos_interrupt(false);
    true
}

fn cancel_async_sequence(io: &mut impl AsyncSequenceIo, state: &EventState) {
    io.eos_interrupt(false);
    io.disarm();
    // RM 22.13.8: START=0 stops conversion; the next MODE=4 scan starts at SQR0.
    io.start(false);
    io.clear_eos();
    state.reset();
}

struct ConversionGuard<'a, IO: AsyncSequenceIo> {
    io: IO,
    state: &'a EventState,
}

impl<'a, IO: AsyncSequenceIo> ConversionGuard<'a, IO> {
    fn start(mut io: IO, state: &'a EventState, cr: u32, trigger: u32) -> Result<Self, Error> {
        critical_section::with(|_| start_async_sequence(&mut io, state, cr, trigger))?;
        Ok(Self { io, state })
    }
}

impl<IO: AsyncSequenceIo> Drop for ConversionGuard<'_, IO> {
    fn drop(&mut self) {
        critical_section::with(|_| cancel_async_sequence(&mut self.io, self.state));
    }
}

fn flags() -> u32 {
    f::icr::EOC.mask()
        | f::icr::EOS.mask()
        | f::icr::EOA.mask()
        | f::icr::WDTL.mask()
        | f::icr::WDTH.mask()
        | f::icr::WDTR.mask()
        | f::icr::OVW.mask()
}
fn control_mask() -> u32 {
    f::cr0::EN.mask()
        | f::cr0::MODE.mask()
        | f::cr0::TSEN.mask()
        | f::cr0::REF.mask()
        | f::cr0::CLK.mask()
        | f::cr0::SAM.mask()
        | f::cr0::BUF.mask()
}
fn control_word(old: u32, divider: ClockDivider, sample: SampleTime) -> u32 {
    let mut word = old & !control_mask();
    word = f::cr0::MODE.write(word, 4); // single complete sequence scan, not continuous
    word = f::cr0::REF.write(word, 3); // VDDA; no unowned external-reference pin
    word = f::cr0::CLK.write(word, divider as u32);
    word = f::cr0::SAM.write(word, sample as u32);
    word = f::cr0::BUF.write(word, true);
    f::cr0::EN.write(word, true)
}
trait SequenceIo {
    fn disarm(&mut self);
    fn busy(&mut self) -> bool;
    fn control(&mut self, word: u32);
    fn clear(&mut self);
}
struct Hardware<I: Instance>(PhantomData<I>);
impl<I: Instance> SequenceIo for Hardware<I> {
    fn disarm(&mut self) {
        unsafe {
            I::regs().trigger().write(0);
        }
    }
    fn busy(&mut self) -> bool {
        unsafe { f::start::START.read(I::regs().start().read()) }
    }
    fn control(&mut self, word: u32) {
        critical_section::with(|_| unsafe {
            let r = I::regs();
            // Keep VC's current BGREN value, never restore a stale snapshot.
            r.cr0()
                .write((r.cr0().read() & !control_mask()) | (word & control_mask()));
        });
    }
    fn clear(&mut self) {
        unsafe {
            I::regs().icr().write(0x7f & !flags());
        }
    }
}
fn disarm_idle(io: &mut impl SequenceIo) -> Result<(), Error> {
    // Hardware may start a sequence up to the disarm write. Inspect START afterwards.
    io.disarm();
    if io.busy() {
        Err(Error::Busy)
    } else {
        Ok(())
    }
}
fn prepare_sequence(io: &mut impl SequenceIo, cr: u32) -> Result<(), Error> {
    disarm_idle(io)?;
    io.control(cr);
    io.clear();
    Ok(())
}
impl<'d, I: Instance, const N: usize> Drop for Adc<'d, I, N> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            // Also clean up a previously forgotten async future when its owner
            // is eventually dropped. Only this ADC is disabled, never its NVIC.
            unsafe { I::regs().ier().write(0) };
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
            unsafe {
                I::regs()
                    .cr0()
                    .write(I::regs().cr0().read() & !control_mask());
            }
        });
    }
}
