//! CW32 ADC1/ADC2 blocking and IRQ-driven sequence sampling (RM 25).
//!
//! [`Adc`] owns the peripheral, with blocking or interrupt-driven mode selected
//! by its constructor. Channels are borrowed per read; [`Sequence`] retains the
//! owner and channel borrows for fixed-length, watchdog and ATIM capture work.
//! Async capture is cancellation-safe, one-shot, and neither DMA nor lossless streaming.
use super::{BorrowedAdcChannel, BorrowedChannel};
use crate::{
    async_support::EventState, gpio::Pin, interrupt, pac, peripherals, rcc::PeripheralClock, Async,
    Blocking, Mode, Peri, PeripheralType,
};
use core::{
    future::poll_fn,
    marker::PhantomData,
    task::{Context, Poll},
};
use embedded_hal::delay::DelayNs;
use interrupt::typelevel::Interrupt as _;
use pac::adc::regs;
mod sealed {
    pub(crate) trait Sealed {
        fn regs() -> crate::pac::adc::Adc;
        fn state() -> &'static crate::async_support::EventState;
    }
    pub trait PinSealed<I> {}
}
/// Generated peripheral identity; shared ADC reset is deliberately never asserted.
#[allow(private_bounds)]
pub trait Instance: sealed::Sealed + PeripheralClock + PeripheralType + 'static {
    /// ADC1 has its own vector; ADC2 shares ADC2_DAC with the DAC.
    type Interrupt: interrupt::typelevel::Interrupt;
}
pub trait ChannelPin<I: Instance>: sealed::PinSealed<I> + Pin {
    const CHANNEL: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_adc.rs"));
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
fn validate_clock(pclk: u32, config: Config) -> Result<(), Error> {
    // Conservative across all supported VDDA, RM 25.4.2. Faster settings need voltage-aware policy.
    if pclk == 0 || pclk > 6_000_000u32 * (1u32 << (config.clock_divider as u8)) {
        return Err(Error::ClockTooFast);
    }
    Ok(())
}
/// Owns one ADC peripheral. Channels are exclusively borrowed for each operation.
/// Conversion values are uncalibrated 12-bit codes.
pub struct Adc<'d, I: Instance, M: Mode> {
    _instance: Peri<'d, I>,
    _mode: PhantomData<M>,
    config: Config,
    clock_hz: u32,
    cr: u32,
}
impl<'d, I: Instance> Adc<'d, I, Blocking> {
    /// Initialize without an IRQ binding. The delay must provide real analog settling time.
    pub fn new_blocking(
        instance: Peri<'d, I>,
        config: Config,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        Self::new_inner(instance, config, delay)
    }
}
impl<'d, I: Instance> Adc<'d, I, Async> {
    /// Initialize with this ADC's checked EOS interrupt binding.
    /// Shared-vector pending state is never cleared and the vector is never disabled.
    pub fn new(
        instance: Peri<'d, I>,
        _irq: impl interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
        config: Config,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let adc = Self::new_inner(instance, config, delay)?;
        // SAFETY: the binding proves the handler is installed on the actual vector.
        unsafe { I::Interrupt::enable() };
        Ok(adc)
    }

    /// Read one borrowed channel using the EOS IRQ. The future retains its exclusive
    /// channel and owner borrows until completion or cancellation. No internal timeout.
    pub async fn read<'ch>(
        &mut self,
        channel: impl BorrowedChannel<'ch, I>,
        sample_time: SampleTime,
    ) -> Result<u16, Error> {
        let mut sequence = self.configure_sequence([(channel.into_channel(), sample_time)])?;
        Ok(sequence.sample().await?[0])
    }
}
impl<'d, I: Instance, M: Mode> Adc<'d, I, M> {
    fn new_inner(
        instance: Peri<'d, I>,
        config: Config,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let pclk = crate::rcc::clocks().pclk_hz();
        validate_clock(pclk, config)?;
        I::enable_and_reset();
        let r = I::regs();
        // RM gives CR reset 0x100 with bit8 reserved: never overwrite reserved
        // readback, including when a shared reset cannot be asserted.
        let reserved = r.cr().read().0 & !control_mask();
        let cr = control_word(reserved, config.clock_divider, 1);
        // Exclusive instance, shared gate remains enabled. Initialize only this ADC.
        r.trigger().write_value(regs::Trigger(0));
        r.start().write_value(regs::Start(0));
        r.cr().write_value(regs::Cr(reserved));
        r.ier().write_value(regs::Ier(0));
        r.awdcr().write_value(regs::Awdcr(0));
        r.sqrcfr().write_value(regs::Sqrcfr(0));
        r.sample().write_value(regs::Sample(0));
        r.icr().write_value(regs::Icr(!flags()));
        r.cr().write_value(regs::Cr(cr));
        delay.delay_us(32); // EN needs ~1 us; auto-started BGR needs ~30 us (RM 25.12.19).
        I::state().reset();
        Ok(Self {
            _instance: instance,
            _mode: PhantomData,
            config,
            clock_hz: pclk / (1u32 << (config.clock_divider as u8)),
            cr,
        })
    }

    /// Validated nominal ADC clock, derived from the active RCC configuration.
    pub fn clock_hz(&self) -> u32 {
        self.clock_hz
    }
    pub fn busy(&self) -> bool {
        I::regs().start().read().start()
    }

    /// Read one borrowed channel, with a bounded number of register observations.
    /// `poll_limit` counts observations, not microseconds. Available in either mode.
    pub fn blocking_read<'ch>(
        &mut self,
        channel: impl BorrowedChannel<'ch, I>,
        sample_time: SampleTime,
        poll_limit: u32,
    ) -> Result<u16, Error> {
        let mut sequence = self.configure_sequence([(channel.into_channel(), sample_time)])?;
        Ok(sequence.blocking_sample(poll_limit)?[0])
    }

    /// Borrow this owner and all channels for a fixed-length scan extension.
    /// Channel tokens can be produced by `AdcChannel::reborrow_adc` or `degrade_adc`.
    /// Dropping the sequence stops conversions and removes its watchdog configuration.
    pub fn configure_sequence<'a, 'ch, const N: usize>(
        &'a mut self,
        channels: [(BorrowedAdcChannel<'ch, I>, SampleTime); N],
    ) -> Result<Sequence<'a, 'd, 'ch, I, M, N>, Error> {
        validate_sequence::<N>()?;
        let cr = control_word(self.cr, self.config.clock_divider, N);
        Ok(Sequence {
            adc: self,
            channels,
            cr,
        })
    }

    /// Disable this instance's EOS source, disarm its trigger and stop conversion.
    /// Leaves shared NVIC and other peripherals untouched.
    pub fn stop(&mut self) {
        critical_section::with(|_| {
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state())
        });
    }
    pub fn watchdog_fault(&self) -> bool {
        let status = I::regs().isr().read();
        status.awdl() || status.awdh()
    }
    pub fn acknowledge_watchdog(&mut self) {
        let mut clear = regs::Icr(u32::MAX);
        clear.set_awdl(false);
        clear.set_awdh(false);
        I::regs().icr().write_value(clear);
    }
}

fn validate_sequence<const N: usize>() -> Result<(), Error> {
    if N == 0 {
        return Err(Error::EmptySequence);
    }
    if N > 8 {
        return Err(Error::SequenceTooLong);
    }
    Ok(())
}

/// Fixed-length scan, watchdog and triggered-capture extension.
///
/// This is an exclusive borrow of an existing ADC owner, not a second peripheral owner.
/// It retains each non-cloneable channel borrow until dropped. Async operations additionally
/// borrow the sequence, so its channels cannot be repurposed while a future is pending.
/// Trigger captures return the latest coherent scan, not a lossless first-edge capture.
pub struct Sequence<'a, 'd, 'ch, I: Instance, M: Mode, const N: usize> {
    adc: &'a mut Adc<'d, I, M>,
    channels: [(BorrowedAdcChannel<'ch, I>, SampleTime); N],
    cr: u32,
}
impl<I: Instance, M: Mode, const N: usize> Sequence<'_, '_, '_, I, M, N> {
    fn prepare(&mut self) -> Result<(), Error> {
        disarm_idle(&mut Hardware::<I>(PhantomData))?;
        // Remove any stale completion from a forgotten earlier operation before reconfiguration.
        self.adc.stop();
        let mut channels = regs::Sqrcfr(0);
        let mut times = regs::Sample(0);
        for (n, (channel, sample)) in self.channels.iter().enumerate() {
            channels.set_sqrch(n, channel.get_hw_channel());
            times.set_sqrch(n, *sample as u8);
        }
        I::regs().sqrcfr().write_value(channels);
        I::regs().sample().write_value(times);
        prepare_sequence(&mut Hardware::<I>(PhantomData), self.cr)
    }
    /// Single software-triggered complete scan; the limit counts register observations.
    pub fn blocking_sample(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        self.prepare()?;
        I::regs().start().write(|w| w.set_start(true));
        self.wait_complete(poll_limit)
    }
    fn wait_complete(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        for _ in 0..poll_limit {
            if I::regs().isr().read().eos() && !self.adc.busy() {
                return Ok(self.results());
            }
            core::hint::spin_loop();
        }
        self.stop();
        Err(Error::Timeout)
    }
    fn results(&self) -> [u16; N] {
        core::array::from_fn(|n| I::regs().result(n).read().result() & 0x0fff)
    }
    /// Arm ATIM update input without starting or reconfiguring ATIM. Use
    /// `capture_triggered` to freeze the route and obtain a coherent complete scan.
    pub fn arm_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<(), Error> {
        self.prepare()?;
        I::regs().trigger().write(|w| w.set_atimtrgo(true));
        Ok(())
    }
    /// Disarm after EOS and finish any already-started final scan. Bounded polling.
    pub fn capture_triggered(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        let mut left = poll_limit;
        while left > 0 {
            left -= 1;
            if I::regs().isr().read().eos() {
                I::regs().trigger().write_value(regs::Trigger(0));
                return self.wait_complete(left.saturating_add(1));
            }
            core::hint::spin_loop();
        }
        self.stop();
        Err(Error::Timeout)
    }
    pub fn stop(&mut self) {
        self.adc.stop();
    }
    /// Configure a polling analog watchdog over this sequence's external channels.
    /// Disarms external triggers before checking idle; Busy leaves the route disarmed.
    pub fn watchdog(&mut self, low: u16, high: u16) -> Result<(), Error> {
        if low > high || high > 4095 {
            return Err(Error::InvalidThreshold);
        }
        disarm_idle(&mut Hardware::<I>(PhantomData))?;
        let mask = self
            .channels
            .iter()
            .fold(0, |m, (i, _)| m | (1u32 << i.get_hw_channel()));
        I::regs().awdtr().write(|w| {
            w.set_vtl(low);
            w.set_vth(high);
        });
        I::regs().awdcr().write_value(regs::Awdcr(mask));
        Ok(())
    }
    pub fn watchdog_fault(&self) -> bool {
        self.adc.watchdog_fault()
    }
    pub fn acknowledge_watchdog(&mut self) {
        self.adc.acknowledge_watchdog();
    }
}
impl<I: Instance, const N: usize> Sequence<'_, '_, '_, I, Async, N> {
    /// Await one software-triggered scan. Dropping the future stops and clears
    /// this instance's EOS source, trigger, conversion and registered waker.
    pub async fn sample(&mut self) -> Result<[u16; N], Error> {
        self.convert(0).await
    }

    /// Arm ATIM on first poll and await the latest coherent complete scan.
    /// Does not start ATIM. Cancellation also works if no trigger arrives.
    pub async fn sample_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<[u16; N], Error> {
        let mut trigger = regs::Trigger(0);
        trigger.set_atimtrgo(true);
        self.convert(trigger.0).await
    }
    async fn convert(&mut self, trigger: u32) -> Result<[u16; N], Error> {
        self.prepare()?;
        let _guard =
            ConversionGuard::start(Hardware::<I>(PhantomData), I::state(), self.cr, trigger)?;
        poll_fn(|cx| poll_sequence(I::state(), cx)).await;
        // The ISR disarmed the trigger and observed START=0 before publication.
        Ok(self.results())
    }
}
impl<I: Instance, M: Mode, const N: usize> Drop for Sequence<'_, '_, '_, I, M, N> {
    fn drop(&mut self) {
        self.adc.stop();
        I::regs().awdcr().write_value(regs::Awdcr(0));
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
        I::regs().ier().read().eos()
    }
    fn eos_pending(&mut self) -> bool {
        I::regs().isr().read().eos()
    }
    fn eos_interrupt(&mut self, enabled: bool) {
        let r = I::regs();
        r.ier().modify(|w| w.set_eos(enabled));
    }
    fn clear_eos(&mut self) {
        // ADC ICR is R1W0, not W1C (RM 25.12.10). Preserve EOC and AWD flags.
        let mut clear = regs::Icr(u32::MAX);
        clear.set_eos(false);
        I::regs().icr().write_value(clear);
    }
    fn start(&mut self, enabled: bool) {
        I::regs().start().write(|w| w.set_start(enabled));
    }
    fn trigger(&mut self, mask: u32) {
        I::regs().trigger().write_value(regs::Trigger(mask));
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
    let mut flags = regs::Icr(0);
    flags.set_eoc(true);
    flags.set_eos(true);
    flags.set_awdl(true);
    flags.set_awdh(true);
    flags.0
}
fn control_mask() -> u32 {
    let mut mask = regs::Cr(0);
    mask.set_en(true);
    mask.set_cont(true);
    mask.set_clk(3);
    mask.set_ens(7);
    mask.set_slave(true);
    mask.0
}
fn control_word(old: u32, divider: ClockDivider, count: usize) -> u32 {
    let mut word = regs::Cr(old & !control_mask());
    word.set_clk(divider as u8);
    word.set_ens((count - 1) as u8);
    word.set_en(true);
    word.0
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
        I::regs().trigger().write_value(regs::Trigger(0));
    }
    fn busy(&mut self) -> bool {
        I::regs().start().read().start()
    }
    fn control(&mut self, word: u32) {
        I::regs().cr().write_value(regs::Cr(word));
    }
    fn clear(&mut self) {
        I::regs().icr().write_value(regs::Icr(!flags()));
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
impl<'d, I: Instance, M: Mode> Drop for Adc<'d, I, M> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            // Also clean up a previously forgotten async future when its owner
            // is eventually dropped. Only this ADC is disabled, never its NVIC.
            I::regs().ier().write_value(regs::Ier(0));
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
            I::regs().cr().modify(|w| w.0 &= !control_mask());
        });
    }
}
/// Both real ADCs start from ADC1 START; ADC2 CR.SLAVE follows ADC1 (RM 25.12.1).
/// Input sequences may have different lengths. Both end flags must complete.
/// On error both stop, and the temporary slave setting is removed.
pub fn sample_pair<M: Mode, S: Mode, const A: usize, const B: usize>(
    master: &mut Sequence<'_, '_, '_, peripherals::ADC1, M, A>,
    slave: &mut Sequence<'_, '_, '_, peripherals::ADC2, S, B>,
    poll_limit: u32,
) -> Result<([u16; A], [u16; B]), Error> {
    let result = (|| {
        master.prepare()?;
        slave.prepare()?;
        let mut control = regs::Cr(slave.cr);
        control.set_slave(true);
        pac::ADC2.cr().write_value(control);
        pac::ADC1.start().write(|w| w.set_start(true));
        let a = master.wait_complete(poll_limit)?;
        let b = slave.wait_complete(poll_limit)?;
        Ok((a, b))
    })();
    master.stop();
    slave.stop();
    pac::ADC2.cr().write_value(regs::Cr(slave.cr));
    result
}
