//! CW32 ADC1/ADC2 blocking and IRQ-driven sequence sampling (RM 25).
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
    /// ADC1 has its own vector; ADC2 shares ADC2_DAC with the DAC.
    type Interrupt: interrupt::typelevel::Interrupt;
}
pub trait ChannelPin<I: Instance>: sealed::PinSealed<I> + Pin {
    const CHANNEL: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_adc.rs"));
/// Owns the bonded analog input pin. Internal TS/BGR/OPA channels are not fabricated as external pins.
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
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivider {
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SampleTime {
    Cycles6 = 0,
    Cycles7,
    Cycles9,
    Cycles12,
    Cycles18,
    Cycles24,
    Cycles30,
    Cycles42,
    Cycles54,
    Cycles70,
    Cycles102,
    Cycles134,
    Cycles166,
    Cycles198,
    Cycles262,
    Cycles518,
}
impl SampleTime {
    pub const fn cycles(self) -> u16 {
        [
            6, 7, 9, 12, 18, 24, 30, 42, 54, 70, 102, 134, 166, 198, 262, 518,
        ][self as usize]
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub clock_divider: ClockDivider,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            clock_divider: ClockDivider::Div1,
        }
    }
}
fn validate<const N: usize>(pclk: u32, config: Config) -> Result<(), Error> {
    if N == 0 {
        return Err(Error::EmptySequence);
    }
    if N > 8 {
        return Err(Error::SequenceTooLong);
    }
    // Conservative across all supported VDDA, RM 25.4.2. Faster settings need voltage-aware policy.
    if pclk == 0 || pclk > 6_000_000u32 * (1u32 << (config.clock_divider as u8)) {
        return Err(Error::ClockTooFast);
    }
    Ok(())
}
/// Retains instance and all sequence pins. Conversion values are uncalibrated 12-bit codes.
pub struct Adc<'d, I: Instance, const N: usize> {
    _instance: Peri<'d, I>,
    _inputs: [AnalogInput<'d, I>; N],
    cr: u32,
}
impl<'d, I: Instance, const N: usize> Adc<'d, I, N> {
    pub fn new(
        instance: Peri<'d, I>,
        inputs: [AnalogInput<'d, I>; N],
        samples: [SampleTime; N],
        config: Config,
        clocks: &crate::rcc::Clocks,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        validate::<N>(clocks.pclk_hz(), config)?;
        I::enable_and_reset();
        let r = I::regs();
        // RM gives CR reset 0x100 with bit8 reserved: never overwrite reserved
        // readback, including when a shared reset cannot be asserted.
        let reserved = unsafe { r.cr().read() } & !control_mask();
        let cr = control_word(reserved, config.clock_divider, N);
        let mut channels = 0;
        let mut times = 0;
        for n in 0..N {
            channels |= u32::from(inputs[n].channel) << (4 * n);
            times |= (samples[n] as u32) << (4 * n);
        }
        // SAFETY: exclusive instance, shared gate remains enabled. Initialize only this ADC.
        unsafe {
            r.trigger().write_value(0);
            r.start().write_value(0);
            r.cr().write_value(reserved);
            r.ier().write_value(0);
            r.awdcr().write_value(0);
            r.sqrcfr().write_value(channels);
            r.sample().write_value(times);
            r.icr().write_value(!flags());
            r.cr().write_value(cr);
        }
        delay.delay_us(32); // EN needs ~1 us; auto-started BGR needs ~30 us (RM 25.12.19).
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
    /// include [`InterruptHandler<I>`] on this ADC's actual vector. On ADC2_DAC,
    /// bind the DAC handler alongside it if the DAC also uses interrupts.
    /// Shared NVIC enable/pending state is never cleared or disabled here.
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
            I::regs()
                .start()
                .write_value(f::start::START.write(0, true));
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
        core::array::from_fn(|n| unsafe {
            let value = match n {
                0 => r.result0().read(),
                1 => r.result1().read(),
                2 => r.result2().read(),
                3 => r.result3().read(),
                4 => r.result4().read(),
                5 => r.result5().read(),
                6 => r.result6().read(),
                7 => r.result7().read(),
                _ => unreachable!(),
            };
            (value & 0x0fff) as u16
        })
    }
    /// Arm ATIM update TRGO. This only programs the ADC input route, and does not start or change ATIM.
    /// Call capture_triggered to freeze the route and obtain a coherent completed sequence.
    pub fn arm_atim_update(&mut self, _timer: &crate::atim::ThreePhasePwm) -> Result<(), Error> {
        self.prepare()?;
        unsafe {
            I::regs()
                .trigger()
                .write_value(f::trigger::ATIMTRGO.write(0, true));
        }
        Ok(())
    }
    /// Wait for EOS, then disarm further triggers and finish any already-started sequence.
    /// Returns the latest complete sequence, possibly later than the first observed EOS.
    /// This is a bounded one-shot capture, not a lossless/jitter-free periodic sampler.
    pub fn capture_triggered(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        let mut left = poll_limit;
        while left > 0 {
            left -= 1;
            if unsafe { f::isr::EOS.read(I::regs().isr().read()) } {
                unsafe {
                    I::regs().trigger().write_value(0);
                }
                return self.wait_complete(left.saturating_add(1));
            }
            core::hint::spin_loop();
        }
        self.stop();
        Err(Error::Timeout)
    }
    pub fn stop(&mut self) {
        unsafe {
            I::regs().trigger().write_value(0);
            I::regs().start().write_value(0);
        }
    }
    /// Configure a polling analog watchdog over this sequence's external channels.
    /// Disarms external triggers before checking idle; Busy leaves the route disarmed.
    pub fn watchdog(&mut self, low: u16, high: u16) -> Result<(), Error> {
        if low > high || high > 4095 {
            return Err(Error::InvalidThreshold);
        }
        disarm_idle(&mut Hardware::<I>(PhantomData))?;
        let mask = self._inputs.iter().fold(0, |m, i| m | (1u32 << i.channel));
        unsafe {
            I::regs()
                .awdtr()
                .write_value(f::awdtr::VTH.write(f::awdtr::VTL.write(0, low.into()), high.into()));
            I::regs().awdcr().write_value(mask);
        }
        Ok(())
    }
    pub fn watchdog_fault(&self) -> bool {
        unsafe { I::regs().isr().read() & (f::isr::AWDL.mask() | f::isr::AWDH.mask()) != 0 }
    }
    pub fn acknowledge_watchdog(&mut self) {
        unsafe {
            I::regs()
                .icr()
                .write_value(!(f::icr::AWDL.mask() | f::icr::AWDH.mask()));
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

    /// Arm the ADC's ATIM update input and await a coherent completed sequence.
    ///
    /// This does not start or reconfigure ATIM. The ADC input is armed on first
    /// poll, so triggers before polling are not captured. At EOS the interrupt
    /// handler disarms further triggers. If another sequence has already
    /// started, it awaits that sequence's EOS before returning the latest
    /// complete sequence. This is not a first-edge or lossless periodic sampler.
    /// Cancellation also works when no external trigger ever arrives.
    pub async fn sample_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<[u16; N], Error> {
        self.convert(f::trigger::ATIMTRGO.mask()).await
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
/// On ADC2_DAC this services only ADC2's enabled EOS source; it does not touch
/// DAC registers, other ADC flags, or the shared NVIC state.
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
            r.ier()
                .write_value(f::ier::EOS.write(r.ier().read(), enabled));
        }
    }
    fn clear_eos(&mut self) {
        // ADC ICR is R1W0, not W1C (RM 25.12.10). Preserve EOC and AWD flags.
        unsafe { I::regs().icr().write_value(!f::icr::EOS.mask()) };
    }
    fn start(&mut self, enabled: bool) {
        unsafe {
            I::regs()
                .start()
                .write_value(f::start::START.write(0, enabled))
        };
    }
    fn trigger(&mut self, mask: u32) {
        unsafe { I::regs().trigger().write_value(mask) };
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
    // CONT=0: RM 25.5.2 guarantees EOS and automatic START clear at sequence
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
    // RM 25.12.2: START=0 stops conversion and resets the sequence index.
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
    f::icr::EOC.mask() | f::icr::EOS.mask() | f::icr::AWDL.mask() | f::icr::AWDH.mask()
}
fn control_mask() -> u32 {
    f::cr::EN.mask()
        | f::cr::CONT.mask()
        | f::cr::CLK.mask()
        | f::cr::ENS.mask()
        | f::cr::SLAVE.mask()
}
fn control_word(old: u32, divider: ClockDivider, count: usize) -> u32 {
    f::cr::EN.write(
        f::cr::ENS.write(
            f::cr::CLK.write(old & !control_mask(), divider as u32),
            (count - 1) as u32,
        ),
        true,
    )
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
            I::regs().trigger().write_value(0);
        }
    }
    fn busy(&mut self) -> bool {
        unsafe { f::start::START.read(I::regs().start().read()) }
    }
    fn control(&mut self, word: u32) {
        unsafe {
            I::regs().cr().write_value(word);
        }
    }
    fn clear(&mut self) {
        unsafe {
            I::regs().icr().write_value(!flags());
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
            unsafe { I::regs().ier().write_value(0) };
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
            unsafe {
                I::regs()
                    .cr()
                    .write_value(I::regs().cr().read() & !control_mask());
            }
        });
    }
}
/// Both real ADCs start from ADC1 START; ADC2 CR.SLAVE follows ADC1 (RM 25.12.1).
/// Input sequences may have different lengths. Both end flags must complete.
/// On error both stop, and the temporary slave setting is removed.
pub fn sample_pair<const A: usize, const B: usize>(
    master: &mut Adc<'_, peripherals::ADC1, A>,
    slave: &mut Adc<'_, peripherals::ADC2, B>,
    poll_limit: u32,
) -> Result<([u16; A], [u16; B]), Error> {
    let result = (|| {
        master.prepare()?;
        slave.prepare()?;
        unsafe {
            pac::ADC2
                .cr()
                .write_value(f::cr::SLAVE.write(slave.cr, true));
            pac::ADC1
                .start()
                .write_value(f::start::START.write(0, true));
        }
        let a = master.wait_complete(poll_limit)?;
        let b = slave.wait_complete(poll_limit)?;
        Ok((a, b))
    })();
    master.stop();
    slave.stop();
    unsafe {
        pac::ADC2.cr().write_value(slave.cr);
    }
    result
}
