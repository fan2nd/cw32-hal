//! CW32F030 single ADC: blocking and interrupt-driven sampling (RM 22).
//! VDDA reference, buffered external inputs, 1–4 slots, one shared sample time.
//! ADCCLK is conservatively limited to 500 kHz across the full VDDA range.
//! One channel uses MODE=0/EOC; multi-slot scans use MODE=4/EOS. Hardware
//! watchdog is supported only for a one-channel sequence (RM 22.9).
//! Internal signals use a dedicated MODE=0 path; no dual-ADC or
//! voltage-qualified high-speed API is implied.
//!
//! [`Adc`] owns the peripheral, with blocking or interrupt-driven mode selected
//! by its constructor. Channels are borrowed per read; [`Sequence`] retains the
//! owner and channel borrows for fixed-length, watchdog and ATIM capture work.
//! ADC-IRQ capture is cancellation-safe and one-shot. The separate finite DMA
//! endpoint owns static inputs and destination; neither API promises streaming.
use super::{
    common::{
        cancel_async_sequence, disarm_idle, on_interrupt, poll_sequence, prepare_sequence,
        AsyncSequenceIo, ConversionGuard, SequenceIo,
    },
    BorrowedAdcChannel, BorrowedChannel, InternalSource,
};
use crate::{
    gpio::Pin, interrupt, pac, peripherals, rcc::KernelClock, Async, Blocking, Mode, Peri,
    PeripheralType,
};
use core::{future::poll_fn, marker::PhantomData};
use embedded_hal::delay::DelayNs;
use interrupt::typelevel::Interrupt as _;
use pac::adc::regs;
mod sealed {
    pub(crate) trait Sealed {
        fn regs() -> crate::pac::adc::Adc;
        fn state() -> &'static crate::interrupt::EventState;
        #[cfg(any(dma_l012, dma_f030))]
        fn dma_state() -> &'static crate::adc::common::DmaState;
        #[cfg(any(dma_l012, dma_f030))]
        fn dma_request() -> crate::dma::Request;
    }
    pub trait PinSealed<I> {}
}
/// Generated peripheral identity; shared ADC reset is deliberately never asserted.
#[allow(private_bounds)]
pub trait Instance: sealed::Sealed + KernelClock + PeripheralType + 'static {
    /// The physical ADC vector of this single-ADC device.
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
    MixedSampleTimes,
    NotReady,
    /// F030 hardware watchdog works only in single-channel mode (RM 22.9).
    UnsupportedWatchdogMode,
    /// An interrupted DMA read retains this ADC and its static resources until reset.
    #[cfg(any(dma_l012, dma_f030))]
    DmaPoisoned,
    /// Only MODE=0, one conversion to RESULT0, has an audited DMA endpoint.
    #[cfg(any(dma_l012, dma_f030))]
    UnsupportedDmaSequence,
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
fn validate_clock(pclk: u32, config: Config) -> Result<(), Error> {
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
/// Owns one ADC peripheral. Channels are exclusively borrowed for each operation.
/// Conversion values are uncalibrated 12-bit codes.
pub struct Adc<'d, I: Instance, M: Mode> {
    _clock: crate::rcc::ClockGuard,
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
    /// Initialize with this ADC's checked conversion interrupt binding.
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

    /// Read one borrowed channel using its conversion-complete IRQ. The future retains its exclusive
    /// channel and owner borrows until completion or cancellation. No internal timeout.
    pub async fn read<'ch>(
        &mut self,
        channel: impl BorrowedChannel<'ch, I>,
        sample_time: SampleTime,
    ) -> Result<u16, Error> {
        let mut sequence = self.configure_sequence([(channel.into_channel(), sample_time)])?;
        Ok(sequence.sample().await?[0])
    }

    /// Activate and settle one internal source, then await its raw 12-bit code.
    /// Always uses MODE=0, BUF=1 and the existing VDDA reference. Cancellation
    /// stops the ADC before releasing the source borrow. No scan/DMA is exposed.
    pub async fn read_internal(
        &mut self,
        source: &mut impl InternalSource,
        sample_time: SampleTime,
        delay: &mut impl DelayNs,
    ) -> Result<u16, Error> {
        let mut sequence = self.internal_sequence(source, sample_time, delay)?;
        Ok(sequence.sample().await?[0])
    }
}
impl<'d, I: Instance, M: Mode> Adc<'d, I, M> {
    fn new_inner(
        instance: Peri<'d, I>,
        config: Config,
        delay: &mut impl DelayNs,
    ) -> Result<Self, Error> {
        let pclk = I::frequency();
        validate_clock(pclk, config)?;
        #[cfg(any(dma_l012, dma_f030))]
        I::dma_state().check()?;
        let sample = SampleTime::Cycles5;
        // The generated clock operation only enables the gate: ADC reset also
        // clears the BGR reference used by VC1/VC2 and must never be asserted.
        let clock = I::acquire();
        let r = I::regs();
        let cr = control_word(0, config.clock_divider, sample, 1);
        critical_section::with(|_| {
            r.trigger().write_value(regs::Trigger(0));
            r.start().write_value(regs::Start(0));
            r.ier().write_value(regs::Ier(0));
            // Preserve BIAS, reserved bits and shared BGREN, even if VC owns it.
            r.cr0().modify(|w| w.0 &= !control_mask());
            r.cr1().modify(|w| {
                w.set_wdtall(false);
                w.set_wdtch(0);
                w.set_dmaen(false);
                w.set_align(false);
                w.set_discard(false);
                w.set_chmux(0);
            });
            // CR2 mixes an accumulation-clear command with configuration: keep
            // the explicit read/write and leave ACCRST clear.
            let mut accumulation = r.cr2().read();
            accumulation.set_accrst(false);
            accumulation.set_accen(false);
            accumulation.set_cnt(0);
            r.cr2().write_value(accumulation);
            r.sqr().write_value(regs::Sqr(0));
            r.icr().write_value(regs::Icr(0x7f & !flags()));
            r.cr0().modify(|w| w.0 = (w.0 & !control_mask()) | cr);
        });
        delay.delay_us(40); // RM 22.4.1: analog startup, then READY must be checked.
        let ready = poll_ready(config.readiness_poll_limit, || r.isr().read().ready());
        if !ready {
            critical_section::with(|_| {
                r.cr0().modify(|w| w.0 &= !control_mask());
            });
            return Err(Error::NotReady);
        }
        I::state().reset();
        Ok(Self {
            _clock: clock,
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

    /// Activate and settle one internal source, then return its raw 12-bit code.
    /// MODE=0/BUF=1 meets RM 22.4.3. The current ADCCLK limit of 500 kHz
    /// also keeps the follower below its 200 ksps limit for every sample time.
    /// `poll_limit` counts observations, not microseconds.
    pub fn blocking_read_internal(
        &mut self,
        source: &mut impl InternalSource,
        sample_time: SampleTime,
        delay: &mut impl DelayNs,
        poll_limit: u32,
    ) -> Result<u16, Error> {
        let mut sequence = self.internal_sequence(source, sample_time, delay)?;
        Ok(sequence.blocking_sample(poll_limit)?[0])
    }

    fn internal_sequence<'a, 'ch>(
        &'a mut self,
        source: &'ch mut impl InternalSource,
        sample_time: SampleTime,
        delay: &mut impl DelayNs,
    ) -> Result<Sequence<'a, 'd, 'ch, I, M, 1>, Error> {
        // RM 22.4.1: 19 comparison clocks plus the selected sampling phase.
        // Keep the BUF limit explicit if voltage-aware clock policy is added.
        if u64::from(self.clock_hz) > 200_000 * (u64::from(sample_time.cycles()) + 19) {
            return Err(Error::ClockTooFast);
        }
        #[cfg(any(dma_l012, dma_f030))]
        I::dma_state().check()?;
        self.stop();
        source.setup(delay);
        let temperature = source.channel() == 14;
        let mut sequence =
            self.configure_sequence([(super::channel::internal_channel(source), sample_time)])?;
        // Generic external control words clear TSEN. This private one-element
        // sequence must retain it through prepare and the async start write.
        let mut cr = regs::Cr0(sequence.cr);
        cr.set_tsen(temperature);
        sequence.cr = cr.0;
        Ok(sequence)
    }

    /// Borrow this owner and all channels for a fixed-length scan extension.
    /// Channel tokens can be produced by `AdcChannel::reborrow_adc` or `degrade_adc`.
    /// Dropping the sequence stops conversions and removes its watchdog configuration.
    /// A new sequence discards conversion/watchdog state from a forgotten ordinary
    /// sequence. A forgotten DMA read instead retains a persistent Busy/poisoned
    /// lease. Validation errors leave the existing hardware state unchanged.
    pub fn configure_sequence<'a, 'ch, const N: usize>(
        &'a mut self,
        channels: [(BorrowedAdcChannel<'ch, I>, SampleTime); N],
    ) -> Result<Sequence<'a, 'd, 'ch, I, M, N>, Error> {
        validate_sequence::<N>()?;
        #[cfg(any(dma_l012, dma_f030))]
        I::dma_state().check()?;
        let cr = control_word(
            self.cr,
            self.config.clock_divider,
            shared_sample(&channels.each_ref().map(|(_, sample)| *sample))?,
            N,
        );
        self.stop();
        I::regs().cr1().modify(|w| {
            w.set_wdtall(false);
            w.set_wdtch(0);
        });
        self.acknowledge_watchdog();
        Ok(Sequence {
            adc: self,
            channels,
            cr,
        })
    }

    /// Disable this instance's EOC/EOS sources, disarm its trigger and stop conversion.
    /// Leaves shared NVIC and other peripherals untouched.
    /// Also cancels a forgotten DMA read. Without clean DMA TC, input resources
    /// remain quarantined, this ADC stays enabled, and further reads are rejected.
    pub fn stop(&mut self) {
        critical_section::with(|_| {
            #[cfg(any(dma_l012, dma_f030))]
            I::dma_state().cancel();
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state())
        });
    }
    pub fn watchdog_fault(&self) -> bool {
        let status = I::regs().isr().read();
        status.wdtl() || status.wdth()
    }
    pub fn acknowledge_watchdog(&mut self) {
        let mut clear = regs::Icr(0x7f);
        clear.set_wdtl(false);
        clear.set_wdth(false);
        I::regs().icr().write_value(clear);
    }
}

fn validate_sequence<const N: usize>() -> Result<(), Error> {
    if N == 0 {
        return Err(Error::EmptySequence);
    }
    if N > 4 {
        return Err(Error::SequenceTooLong);
    }
    Ok(())
}

fn validate_watchdog<const N: usize>(low: u16, high: u16) -> Result<(), Error> {
    if N != 1 {
        return Err(Error::UnsupportedWatchdogMode);
    }
    if low > high || high > 4095 {
        return Err(Error::InvalidThreshold);
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
        #[cfg(any(dma_l012, dma_f030))]
        I::dma_state().check()?;
        disarm_idle(&mut Hardware::<I>(PhantomData))?;
        // Remove any stale completion from a forgotten earlier operation before reconfiguration.
        self.adc.stop();
        let mut sequence = regs::Sqr(0);
        for (n, (channel, _)) in self.channels.iter().enumerate() {
            sequence.set_sqr(n, channel.get_hw_channel());
        }
        if N == 1 {
            I::regs()
                .cr1()
                .modify(|w| w.set_chmux(self.channels[0].0.get_hw_channel()));
        }
        sequence.set_ens((N - 1) as u8);
        I::regs().sqr().write_value(sequence);
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
            if self.complete() && !self.adc.busy() {
                return Ok(self.results());
            }
            core::hint::spin_loop();
        }
        self.stop();
        Err(Error::Timeout)
    }
    fn complete(&self) -> bool {
        let status = I::regs().isr().read();
        if N == 1 {
            status.eoc()
        } else {
            status.eos()
        }
    }
    fn results(&self) -> [u16; N] {
        core::array::from_fn(|n| I::regs().result(n).read().result() & 0x0fff)
    }
    /// Arm ATIM update input without starting or reconfiguring ATIM. Use
    /// `capture_triggered` to freeze the route and obtain a coherent complete scan.
    #[cfg(atim_f030)]
    pub fn arm_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<(), Error> {
        self.prepare()?;
        I::regs().trigger().write(|w| w.set_atim(true));
        Ok(())
    }
    /// Disarm after completion and finish any already-started final conversion/scan.
    pub fn capture_triggered(&mut self, poll_limit: u32) -> Result<[u16; N], Error> {
        let mut left = poll_limit;
        while left > 0 {
            left -= 1;
            if self.complete() {
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
    /// Configure a hardware polling watchdog for a one-channel sequence.
    /// RM 22.9 limits it to single-channel mode; N > 1 fails before register writes.
    /// Fault means result < low or result >= high (upper threshold is exclusive).
    /// Disarms external triggers before checking idle; Busy leaves the route disarmed.
    pub fn watchdog(&mut self, low: u16, high: u16) -> Result<(), Error> {
        validate_watchdog::<N>(low, high)?;
        disarm_idle(&mut Hardware::<I>(PhantomData))?;
        I::regs().vtl().write(|w| w.set_vtl(low));
        I::regs().vth().write(|w| w.set_vth(high));
        let r = I::regs();
        r.cr1().modify(|w| {
            w.set_wdtch(self.channels[0].0.get_hw_channel());
            w.set_wdtall(true);
        });
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
    /// this instance's EOC/EOS sources, trigger, conversion and registered waker.
    pub async fn sample(&mut self) -> Result<[u16; N], Error> {
        self.convert(0).await
    }

    /// Arm ATIM on first poll and await the latest coherent complete scan.
    /// Does not start ATIM. Cancellation also works if no trigger arrives.
    #[cfg(atim_f030)]
    pub async fn sample_atim_update(
        &mut self,
        _timer: &crate::atim::ThreePhasePwm<'_>,
    ) -> Result<[u16; N], Error> {
        let mut trigger = regs::Trigger(0);
        trigger.set_atim(true);
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

#[cfg(any(dma_l012, dma_f030))]
impl<I: Instance, M: Mode, const N: usize> Sequence<'_, '_, 'static, I, M, N> {
    pub(super) fn prepare_dma(&mut self) -> Result<(), Error> {
        if N != 1 {
            return Err(Error::UnsupportedDmaSequence);
        }
        self.prepare()
    }
    pub(super) fn dma_config() -> crate::dma::RawConfig {
        crate::dma::RawConfig {
            trigger: crate::dma::Trigger::Hardware(I::dma_request()),
            // F030 requests every conversion, not EOS. Only MODE=0 is used:
            // exactly one conversion, one request, and one native RESULT0 word.
            mode: crate::dma::TransferMode::Block,
            source_increment: false,
            destination_increment: true,
        }
    }
    pub(super) fn start_dma() {
        I::regs().cr1().modify(|w| {
            w.set_align(false);
            w.set_discard(false);
            w.set_dmaen(true);
        });
        I::regs().start().write(|w| w.set_start(true));
    }
    pub(super) fn finish_dma(clean: bool) {
        // The hook must not call Adc::stop and recursively cancel the DMA.
        I::regs().cr1().modify(|w| w.set_dmaen(false));
        cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
        I::dma_state().complete(clean);
    }
}
impl<I: Instance, M: Mode, const N: usize> Drop for Sequence<'_, '_, '_, I, M, N> {
    fn drop(&mut self) {
        self.adc.stop();
        I::regs().cr1().modify(|w| w.set_wdtall(false));
    }
}

/// ADC conversion/sequence-complete handler for a verified per-instance binding.
///
/// Services EOC in single-channel mode or EOS in scan mode; preserves unrelated
/// ADC flags and NVIC state.
pub struct InterruptHandler<I: Instance> {
    _instance: PhantomData<I>,
}

impl<I: Instance> interrupt::typelevel::Handler<I::Interrupt> for InterruptHandler<I> {
    unsafe fn on_interrupt() {
        on_interrupt(&mut Hardware::<I>(PhantomData), I::state());
    }
}

impl<I: Instance> AsyncSequenceIo for Hardware<I> {
    fn clock_enabled(&self) -> bool {
        I::clock_resource().is_enabled()
    }
    fn completion_interrupt_enabled(&mut self) -> bool {
        let r = I::regs();
        let enabled = r.ier().read();
        if r.cr0().read().mode() == 0 {
            enabled.eoc()
        } else {
            enabled.eos()
        }
    }
    fn completion_pending(&mut self) -> bool {
        let r = I::regs();
        let status = r.isr().read();
        if r.cr0().read().mode() == 0 {
            status.eoc()
        } else {
            status.eos()
        }
    }
    fn completion_interrupt(&mut self, enabled: bool) {
        let r = I::regs();
        let single = r.cr0().read().mode() == 0;
        r.ier().modify(|w| {
            w.set_eoc(enabled && single);
            w.set_eos(enabled && !single);
        });
    }
    fn clear_completion(&mut self) {
        // ADC ICR is R1W0, not W1C (RM 22.13.11). Preserve AWD/other completion flags.
        let mut clear = regs::Icr(0x7f);
        if I::regs().cr0().read().mode() == 0 {
            clear.set_eoc(false);
        } else {
            clear.set_eos(false);
        }
        I::regs().icr().write_value(clear);
    }
    fn start(&mut self, enabled: bool) {
        // RM 22.13.8: START=0 stops conversion; the next MODE=4 scan starts at SQR0.
        I::regs().start().write(|w| w.set_start(enabled));
    }
    fn trigger(&mut self, mask: u32) {
        I::regs().trigger().write_value(regs::Trigger(mask));
    }
}

fn flags() -> u32 {
    let mut flags = regs::Icr(0);
    flags.set_eoc(true);
    flags.set_eos(true);
    flags.set_eoa(true);
    flags.set_wdtl(true);
    flags.set_wdth(true);
    flags.set_wdtr(true);
    flags.set_ovw(true);
    flags.0
}
fn control_mask() -> u32 {
    let mut mask = regs::Cr0(0);
    mask.set_en(true);
    mask.set_mode(7);
    mask.set_tsen(true);
    mask.set_ref(3);
    mask.set_clk(7);
    mask.set_sam(3);
    mask.set_buf(true);
    mask.0
}
fn control_word(old: u32, divider: ClockDivider, sample: SampleTime, count: usize) -> u32 {
    let mut word = regs::Cr0(old & !control_mask());
    word.set_mode(if count == 1 { 0 } else { 4 }); // single channel or complete scan, never continuous
    word.set_ref(3); // VDDA; no unowned external-reference pin
    word.set_clk(divider as u8);
    word.set_sam(sample as u8);
    word.set_buf(true);
    word.set_en(true);
    word.0
}

struct Hardware<I: Instance>(PhantomData<I>);
impl<I: Instance> SequenceIo for Hardware<I> {
    fn disarm(&mut self) {
        I::regs().trigger().write_value(regs::Trigger(0));
    }
    fn busy(&mut self) -> bool {
        // RM 22.5.1 (MODE=0/EOC) and 22.5.5 (MODE=4/EOS): START clears at completion.
        I::regs().start().read().start()
    }
    fn control(&mut self, word: u32) {
        critical_section::with(|_| {
            let r = I::regs();
            // Keep VC's current BGREN value, never restore a stale snapshot.
            r.cr0()
                .modify(|w| w.0 = (w.0 & !control_mask()) | (word & control_mask()));
        });
    }
    fn clear(&mut self) {
        I::regs().icr().write_value(regs::Icr(0x7f & !flags()));
    }
}
impl<'d, I: Instance, M: Mode> Drop for Adc<'d, I, M> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            #[cfg(any(dma_l012, dma_f030))]
            {
                I::dma_state().cancel();
                if I::dma_state().check().is_err() {
                    self._clock.pin();
                    return;
                }
            }
            // Also clean up a previously forgotten async future when its owner
            // is eventually dropped. Only this ADC is disabled, never its NVIC.
            I::regs().ier().write_value(regs::Ier(0));
            cancel_async_sequence(&mut Hardware::<I>(PhantomData), I::state());
            I::regs().cr0().modify(|w| w.0 &= !control_mask());
        });
    }
}
