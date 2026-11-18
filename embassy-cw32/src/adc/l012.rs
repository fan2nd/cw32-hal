//! CW32L012 independent 12-bit ADC1/ADC2, ordered 1–8 slot software scans.
//!
//! Both typed converters borrow one [`Common`] guard owning BGR. No path resets
//! either converter or disables the common ADC gate. BGR/TS enables are monotonic:
//! constructor, error and drop preserve inherited comparator/OPA/ADC resources.
//! Shared analog resources intentionally remain powered after all owners drop.
//! This explicit guard and power retention differ from an ordinary standalone
//! Embassy ADC; reclaiming shared power requires a future whole-analog owner.
//!
//! VDDA is the only conversion reference and must equal VDD (1.7..=5.5 V).
//! The guaranteed minimum board supply selects the manual clock/rate bands and
//! datasheet acquisition minimum. ADCCLK is PCLK/1/2/4/8, at least 4 MHz and at
//! most 48 MHz. The manual's stricter 1 MSPS limit is also enforced. SampleTime
//! does not replace source-impedance/settling checks required by the board.
//! ADC limits and delays include the source-qualified HSI envelope. The board
//! must stay within rcc::HSI_BOUND_TEMPERATURE_C; extended-temperature HSI
//! accuracy beyond that interval is unqualified, even if other circuitry is rated.
//!
//! Both ADCs support IRQ-driven finite software reads/scans. ADC2 admits its
//! shared `ADC2_DAC` vector only after reading both DAC underrun interrupt enables
//! as zero in a clocked, reset-released domain. Current safe DAC operations keep
//! those enables zero, so ordinary direct-loading DAC outputs may coexist. ADC2
//! never writes DAC registers or clears its flags. Acquiring the retained DAC
//! gate can advance inherited DAC activity, including on a rejected constructor.
//! Future DAC interrupt support needs shared dispatch and a new fault policy.
//! No ADC DMA, external-trigger, synchronized/slave,
//! calibration or engineering-unit API is exposed. Internal channels return raw
//! counts and require >=40 us acquisition. There is no READY or overrun flag.
//! Inherited sibling slave configuration is rejected before converter writes.
//! Raw register users must not change shared gate/reset/BGR or either owned ADC.
//! Raw users of an unowned sibling must not change its SLAVE bit concurrently:
//! the preflight check is a snapshot, not synchronization with arbitrary raw code.
//! During ADC2 async ownership, raw/PAC or external DMA users must not arm DAC
//! DMAUDRIE1/2 or change the DAC gate/reset. Admission does not stop an inherited
//! external DMA writer. This is a functional ownership contract; the ISR retains
//! no pointer to caller buffers, channels or futures.
//! Current scan contract: docs/adc-l012-scans.md. Historical electrical review:
//! docs/adc-l012-dual-ownership.md. Shared-vector contract: docs/adc-l012-adc2-async.md.
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::mode::{Async, Blocking, Mode};
use crate::rcc::ClockBounds;
use crate::time::Hertz;
use crate::{Peri, PeripheralType, pac};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::compiler_fence;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::Poll;
use embassy_sync::waitqueue::AtomicWaker;
/// Maximum independently programmed hardware slots, in caller-supplied order.
pub const MAX_SEQUENCE_LEN: usize = crate::ADC_SEQUENCE_MAX_LEN;

/// External trigger selected for an ADC motor-control scan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdcTrigger {
    /// Advanced timer compare channel 4 reference.
    AtimCc4,
}

/// Fixed hardware conversion resolution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resolution {
    /// Right-aligned 12-bit result, 0..=4095.
    Bits12,
}
impl Resolution {
    /// Number of conversion bits; effective analog accuracy is lower.
    pub const fn bits(self) -> u8 {
        12
    }
    /// Largest result.
    pub const fn max_count(self) -> u32 {
        4095
    }
}
/// Largest result at the selected resolution.
pub const fn resolution_to_max_count(resolution: Resolution) -> u32 {
    resolution.max_count()
}

/// Slot acquisition time; every conversion then needs 15 comparison cycles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum SampleTime {
    /// 6 acquisition cycles.
    Cycles6 = 0,
    /// 7 acquisition cycles.
    Cycles7 = 1,
    /// 9 acquisition cycles.
    Cycles9 = 2,
    /// 12 acquisition cycles.
    Cycles12 = 3,
    /// 18 acquisition cycles.
    Cycles18 = 4,
    /// 24 acquisition cycles.
    Cycles24 = 5,
    /// 30 acquisition cycles.
    Cycles30 = 6,
    /// 42 acquisition cycles.
    Cycles42 = 7,
    /// 54 acquisition cycles.
    Cycles54 = 8,
    /// 70 acquisition cycles.
    Cycles70 = 9,
    /// 102 acquisition cycles.
    Cycles102 = 10,
    /// 134 acquisition cycles.
    Cycles134 = 11,
    /// 166 acquisition cycles.
    Cycles166 = 12,
    /// 198 acquisition cycles.
    Cycles198 = 13,
    /// 262 acquisition cycles.
    Cycles262 = 14,
    /// 518 acquisition cycles.
    Cycles518 = 15,
}
impl SampleTime {
    /// Acquisition cycles.
    pub const fn cycles(self) -> u16 {
        crate::ADC_SAMPLE_CYCLES[self as usize]
    }
    /// Acquisition plus comparison cycles.
    pub const fn conversion_cycles(self) -> u16 {
        self.cycles() + crate::ADC_COMPARISON_CYCLES
    }
}
/// PCLK divider using the actual two-bit CR.CLK encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Prescaler {
    /// PCLK / 1.
    Div1 = 0,
    /// PCLK / 2.
    Div2 = 1,
    /// PCLK / 4.
    Div4 = 2,
    /// PCLK / 8.
    Div8 = 3,
}
impl Prescaler {
    /// Numeric division factor.
    pub const fn divisor(self) -> u32 {
        1 << self as u8
    }
}
/// The sole hardware reference, retained as an enum for common API shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reference {
    /// Analog supply voltage VDDA, which must equal VDD.
    Vdda,
}
/// ADC configuration. Acquisition duration is chosen separately for each read.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Supply reference; no selectable internal reference exists.
    pub reference: Reference,
    /// Guaranteed minimum supply in mV, including board supply tolerance.
    /// Accepted range: 1700..=5500. The board must independently satisfy the
    /// maximum supply and pin ratings; VDDA must equal VDD.
    pub vdda_mv: u16,
    /// Requested maximum ADC clock, including HSI tolerance. Only PCLK/1, /2, /4, /8 exist.
    /// L012 additionally requires at least 4 MHz. A request may therefore be
    /// unattainable; `new`/`set_config`/`blocking_read` return an error.
    pub frequency: Hertz,
    /// Finite polling budget for completion; not a duration in microseconds.
    pub timeout: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            reference: Reference::Vdda,
            vdda_mv: crate::ADC_SUPPLY_RANGE_MV.0,
            frequency: Hertz(6_000_000),
            timeout: 100_000,
        }
    }
}
/// Configuration or bounded conversion failure.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// HAL clocks have not been initialized successfully.
    ClockNotInitialized,
    /// Supply is outside the selected family's allowed range.
    InvalidSupplyVoltage,
    /// Requested maximum ADC frequency is zero.
    FrequencyZero,
    /// No divider meets the requested maximum and acquisition/rate limits.
    FrequencyTooLow,
    /// L012 ADC clock would fall below its documented 4 MHz minimum.
    ClockBelowMinimum,
    /// Polling budget is zero.
    InvalidTimeout,
    /// Peripheral clock gate failed to enable within the polling budget.
    ClockEnableTimeout,
    /// The shared ADC reset is asserted; it cannot be released per instance.
    CommonInReset,
    /// ADC2 could not verify the DAC gate for shared-vector admission.
    /// An attempted gate enable is retained and may advance inherited DAC activity.
    DacClockUnavailable,
    /// The DAC reset is asserted; ADC2 never releases it for async admission.
    DacInReset,
    /// Either DAC underrun interrupt is enabled; ADC2 leaves DAC registers intact.
    /// A DAC gate acquired before this rejection remains enabled.
    DacInterruptEnabled,
    /// The sibling has inherited synchronized slave mode; no ADC writes occur.
    SiblingInSlaveMode,
    /// A scan must contain between one and eight slots.
    InvalidSequenceLength,
    /// The result slice must contain exactly one element per slot.
    InvalidResultLength,
    /// Inherited trigger, watchdog, DMA or local slave state is unsupported.
    UnsupportedConfiguration,
    /// Required local control readback failed; terminal until chip reset.
    Faulted,
    /// EOC, EOS and stopped START did not all occur within the budget.
    ConversionTimeout,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::ClockNotInitialized => "ADC clocks are not initialized",
            Self::InvalidSupplyVoltage => "ADC supply is outside the allowed range",
            Self::FrequencyZero => "ADC frequency limit must be nonzero",
            Self::FrequencyTooLow => "no ADC divider meets the timing limits",
            Self::ClockBelowMinimum => "L012 ADC clock must be at least 4 MHz",
            Self::InvalidTimeout => "ADC polling budget must be nonzero",
            Self::ClockEnableTimeout => "ADC clock gate did not enable",
            Self::CommonInReset => "shared ADC reset is asserted",
            Self::DacClockUnavailable => "DAC clock gate is unavailable for ADC2 async",
            Self::DacInReset => "DAC reset is asserted",
            Self::DacInterruptEnabled => "DAC underrun interrupt is enabled",
            Self::SiblingInSlaveMode => {
                "ADC sibling is configured for synchronized slave conversion"
            }
            Self::InvalidSequenceLength => "ADC sequence must have one to eight slots",
            Self::InvalidResultLength => "ADC results must match the sequence length",
            Self::UnsupportedConfiguration => "ADC inherited configuration is unsupported",
            Self::Faulted => "ADC cleanup failed; chip reset is required",
            Self::ConversionTimeout => "ADC conversion did not complete and stop",
        })
    }
}
impl core::error::Error for Error {}
/// Nominal timing chosen for the last configuration/conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timing {
    /// Divider selected for this acquisition time and source.
    pub prescaler: Prescaler,
    /// Whole-hertz floor of nominal PCLK / divider. Validation uses the exact
    /// rational clock before this display rounding.
    pub frequency: Hertz,
    /// Source-qualified ADC clock envelope, with exact prescaler arithmetic.
    pub clock_bounds: ClockBounds,
    /// Acquisition cycles, followed by 15 comparison cycles.
    pub sample_time: SampleTime,
}
impl Timing {
    /// Guaranteed minimum acquisition in ns, rounded down. Requires the clock
    /// source's documented operating conditions and temperature range.
    pub fn minimum_acquisition_time_ns(self) -> u64 {
        self.clock_bounds
            .minimum_duration_ns(u32::from(self.sample_time.cycles()))
    }
    /// Worst-case conversion duration in ns, rounded up at the slowest HSI.
    /// Excludes startup, source settling, trigger latency and software overhead.
    /// The configured timeout remains an iteration count, not this duration.
    pub fn maximum_conversion_time_ns(self) -> u64 {
        self.clock_bounds
            .maximum_duration_ns(u32::from(self.sample_time.conversion_cycles()))
    }
    /// Nominal conversion nanoseconds, rounded up using the displayed clock.
    /// Excludes startup, source settling and software overhead.
    pub fn conversion_time_ns(self) -> u64 {
        (u64::from(self.sample_time.conversion_cycles()) * 1_000_000_000)
            .div_ceil(u64::from(self.frequency.0))
    }
}
/// Common ADC clock and conversion duration of one completed sequence.
/// Excludes startup, source settling and software overhead. These are source-
/// qualified timing bounds, not an analog accuracy or silicon timing guarantee.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SequenceTiming {
    /// One divider shared by all slots.
    pub prescaler: Prescaler,
    /// Qualified actual ADC clock range including oscillator tolerance.
    pub clock_bounds: ClockBounds,
    /// Number of results, in the supplied slot order.
    pub length: u8,
    /// Sum of the per-slot acquisition and comparison cycles.
    pub conversion_cycles: u32,
}
impl SequenceTiming {
    /// Worst-case conversion duration at the slowest qualified ADC clock.
    pub fn maximum_conversion_time_ns(self) -> u64 {
        self.clock_bounds
            .maximum_duration_ns(self.conversion_cycles)
    }
}

impl Config {
    // Tuple: maximum ADCCLK, maximum samples/s, minimum acquisition in ps.
    fn limits(&self) -> Result<(u32, u32, u32), Error> {
        if self.timeout == 0 {
            return Err(Error::InvalidTimeout);
        }
        if self.frequency.0 == 0 {
            return Err(Error::FrequencyZero);
        }
        if !(crate::ADC_SUPPLY_RANGE_MV.0..=crate::ADC_SUPPLY_RANGE_MV.1).contains(&self.vdda_mv) {
            return Err(Error::InvalidSupplyVoltage);
        }
        let band = crate::ADC_SUPPLY_BANDS
            .iter()
            .rev()
            .find(|b| self.vdda_mv >= b.0)
            .ok_or(Error::InvalidSupplyVoltage)?;
        let limits = (band.1, band.2, band.3);
        Ok(limits)
    }
    fn timing(
        &self,
        pclk: ClockBounds,
        channel: u8,
        sample_time: SampleTime,
    ) -> Result<Timing, Error> {
        let (clock, rate, external_acquisition_ps) = self.limits()?;
        if pclk.nominal().0 == 0 {
            return Err(Error::ClockNotInitialized);
        }
        let acquisition_ps = if channel >= 14 {
            crate::ADC_INTERNAL_ACQUISITION_PS
        } else {
            u64::from(external_acquisition_ps)
        };
        let maximum = clock
            .min(self.frequency.0)
            .min(u32::from(sample_time.conversion_cycles()) * rate);
        for prescaler in [
            Prescaler::Div1,
            Prescaler::Div2,
            Prescaler::Div4,
            Prescaler::Div8,
        ] {
            let candidate = pclk.divided_by(prescaler.divisor());
            if candidate.maximum_exceeds(maximum)
                || candidate.acquisition_too_short(u32::from(sample_time.cycles()), acquisition_ps)
            {
                continue;
            }
            if candidate.minimum_below(crate::ADC_MINIMUM_CLOCK_HZ) {
                return Err(Error::ClockBelowMinimum);
            }
            let clock_bounds = pclk.divided_by(prescaler.divisor());
            let frequency = clock_bounds.nominal();
            if frequency.0 == 0 {
                return Err(Error::FrequencyTooLow);
            }
            return Ok(Timing {
                prescaler,
                frequency,
                clock_bounds,
                sample_time,
            });
        }
        Err(Error::FrequencyTooLow)
    }
}
/// Shared ADC clock and analog-source guard, owning the real BGR singleton.
///
/// Both ADCs borrow this one guard. Acquisition enables the common gate but
/// never resets either ADC. Neither this guard nor its borrowers ever disable
/// the common gate or clear BGR/TS: a bootloader, comparator or OPA may depend
/// on these sticky analog resources. This deliberately retains power on drop.
/// Raw access must not reset/gate ADC, change BGR/TS or alter either converter
/// while the corresponding HAL resources are owned.
pub struct Common<'d> {
    _peripheral: Peri<'d, crate::peripherals::BGR>,
}
impl<'d> Common<'d> {
    /// Acquire the common clock with a finite poll budget, without reset.
    /// An asserted shared reset is rejected without changing any register.
    pub fn new(peripheral: Peri<'d, crate::peripherals::BGR>, timeout: u32) -> Result<Self, Error> {
        crate::rcc::try_clocks().ok_or(Error::ClockNotInitialized)?;
        if timeout == 0 {
            return Err(Error::InvalidTimeout);
        }
        critical_section::with(|cs| clock_acquire(timeout, cs))?;
        Ok(Self {
            _peripheral: peripheral,
        })
    }
}

/// One independently owned ADC borrowing the shared analog/clock guard.
/// Timing/configuration errors leave ADC registers and active state untouched.
/// Shared BGR and a preexisting clock gate are preserved (see module docs).
pub struct Adc<'d, T: Instance, M: Mode = Blocking> {
    _peripheral: Peri<'d, T>,
    config: Config,
    pclk: ClockBounds,
    hclk: ClockBounds,
    timing: Timing,
    active: bool,
    async_lifecycle: bool,
    _common: &'d Common<'d>,
    _mode: PhantomData<M>,
}
impl<'d, T: Instance> Adc<'d, T, Blocking> {
    /// Validate before converter writes, then wait at least 50 us for
    /// analog/BGR startup. Reject inherited slave-mode configuration on the
    /// sibling: starting this ADC must never start an unowned converter.
    /// The default initial acquisition is Cycles518; reads choose their own.
    /// ADC2 first retires a forgotten async owner with bounded local cleanup.
    /// Cleanup failure remains terminal; blocking construction does not admit
    /// or reserve the shared DAC interrupt.
    pub fn new(
        peripheral: Peri<'d, T>,
        common: &'d Common<'d>,
        config: Config,
    ) -> Result<Self, Error> {
        check_fault::<T>()?;
        let clocks = crate::rcc::try_clocks().ok_or(Error::ClockNotInitialized)?;
        let timing = config.timing(clocks.pclk_bounds(), 0, SampleTime::Cycles518)?;
        // Reject before constructing the RAII owner: dropping a rejected owner
        // would otherwise alter the converter during an error return.
        Self::check_sibling()?;
        let construct = || Self {
            _peripheral: peripheral,
            config,
            pclk: clocks.pclk_bounds(),
            hclk: clocks.hclk_bounds(),
            timing,
            active: false,
            async_lifecycle: false,
            _common: common,
            _mode: PhantomData,
        };
        let mut adc = if matches!(T::ASYNC_POLICY, AsyncPolicy::SharedDac) {
            critical_section::with(|_| -> Result<Self, Error> {
                // A forgotten async owner can retain static dispatch after its
                // token reborrow expires. Reject an ISR fault before RAII exists.
                check_fault::<T>()?;
                let mut adc = construct();
                if T::state().irq_active.load(Ordering::Acquire) {
                    adc.async_lifecycle = true;
                    T::state().irq_active.store(false, Ordering::Release);
                    adc.async_cleanup()?;
                    adc.async_lifecycle = false;
                }
                Ok(adc)
            })?
        } else {
            construct()
        };
        adc.initialize();
        Ok(adc)
    }

    /// Construct a blocking ADC while consuming the application's interrupt
    /// binding. The foreground retains ADC ownership; the bound handler may
    /// use the `adc_irq_*` accessors from its ISR.
    pub fn new_with_irq<I, H>(
        peripheral: Peri<'d, T>,
        common: &'d Common<'d>,
        _irqs: I,
        config: Config,
    ) -> Result<Self, Error>
    where
        T: AsyncInstance,
        I: Binding<T::Interrupt, H> + 'd,
        H: Handler<T::Interrupt>,
    {
        Self::new(peripheral, common, config)
    }

    /// Motor-control ADC mode: foreground ownership plus an application-owned
    /// fast current-loop ISR binding.
    pub fn new_motor<I, H>(
        peripheral: Peri<'d, T>,
        common: &'d Common<'d>,
        irqs: I,
        config: Config,
    ) -> Result<Self, Error>
    where
        T: AsyncInstance,
        I: Binding<T::Interrupt, H> + 'd,
        H: Handler<T::Interrupt>,
    {
        Self::new_with_irq(peripheral, common, irqs, config)
    }

    /// Fallible explicit blocking constructor.
    pub fn new_blocking(
        peripheral: Peri<'d, T>,
        common: &'d Common<'d>,
        config: Config,
    ) -> Result<Self, Error> {
        Self::new(peripheral, common, config)
    }
    /// Read the most recently completed hardware result without starting a
    /// conversion. This is intended for timer-triggered motor-control loops:
    /// the trigger/ISR owns conversion timing, while the foreground or a
    /// cooperating observer only consumes the sampled value.
    ///
    /// `None` is returned for a slot outside the device's result bank. The
    /// value is raw right-aligned ADC counts; no flag is cleared and no ADC
    /// register is reconfigured.
    pub fn latest_result(&self, slot: usize) -> Option<u16> {
        if slot >= crate::ADC_SEQUENCE_MAX_LEN {
            return None;
        }
        Some(T::regs().result(slot).read().result() & Resolution::Bits12.max_count() as u16)
    }
    /// Read a verified pin or internal source. GPIO preparation can happen
    /// before timing rejection; the ADC itself remains unchanged on rejection.
    /// Internal sources require >=40 us acquisition, and wait 50 us
    /// before every internal read (TS datasheet maximum startup is 40 us).
    pub fn blocking_read<'a>(
        &mut self,
        channel: impl BorrowedChannel<'a, T>,
        sample_time: SampleTime,
    ) -> Result<u16, Error> {
        let channel = channel.reborrow_adc();
        let mut result = [0];
        self.blocking_read_sequence(&[(&channel, sample_time)], &mut result)?;
        Ok(result[0])
    }
    /// Convert one software-triggered, noncontinuous sequence on this ADC.
    ///
    /// Each of the 1..=8 slots independently selects a verified channel and
    /// sample time. Results preserve slot order; a channel may be repeated by
    /// referencing the same owned or borrowed `BorrowedAdcChannel` handle.
    /// Both ADCs may remain independently owned through one `Common` guard;
    /// these scans do not synchronize their sampling instants.
    ///
    /// Length, timing and sibling SLAVE checks precede ADC/BGR writes. One
    /// divider must satisfy all slots at both clock-envelope extrema. Internal
    /// TS/BGR slots require >=40 us acquisition and a 50 us settling wait.
    /// Results change only after EOC, EOS and hardware-cleared START. The
    /// timeout is a polling budget for the entire sequence. Timeout writes
    /// START=0 to stop/reset the local slot cursor, disables only this ADC and
    /// clears its flags. A later read reinitializes it. Shared gate, BGR, TS
    /// and the sibling's state remain unchanged during cleanup.
    pub fn blocking_read_sequence(
        &mut self,
        sequence: &[(&BorrowedAdcChannel<'_, T>, SampleTime)],
        results: &mut [u16],
    ) -> Result<SequenceTiming, Error> {
        check_fault::<T>()?;
        if sequence.is_empty() || sequence.len() > MAX_SEQUENCE_LEN {
            return Err(Error::InvalidSequenceLength);
        }
        if results.len() != sequence.len() {
            return Err(Error::InvalidResultLength);
        }
        let mut prescaler = Prescaler::Div1;
        let mut conversion_cycles = 0;
        let mut temperature = false;
        let mut bandgap = false;
        for &(channel, sample_time) in sequence {
            let timing = self
                .config
                .timing(self.pclk, channel.channel, sample_time)?;
            if timing.prescaler as u8 > prescaler as u8 {
                prescaler = timing.prescaler;
            }
            conversion_cycles += u32::from(sample_time.conversion_cycles());
            temperature |= channel.channel == 14;
            bandgap |= channel.channel == 15;
        }
        // A slower accepted divider satisfies every upper clock/rate bound
        // and minimum acquisition. Its own selecting slot also proved the
        // shared 4 MHz minimum at the slow clock-envelope extremum.
        let clock_bounds = self.pclk.divided_by(prescaler.divisor());
        let timing = SequenceTiming {
            prescaler,
            clock_bounds,
            length: sequence.len() as u8,
            conversion_cycles,
        };
        Self::check_sibling()?;
        if !self.active {
            self.initialize();
        }
        let regs = T::regs();
        regs.start().write(|v| v.set_start(false));
        self.clear_all_flags();
        regs.sqrcfr().write(|v| {
            for (slot, &(channel, _)) in sequence.iter().enumerate() {
                v.set_sqrch(slot, channel.channel);
            }
        });
        regs.sample().write(|v| {
            for (slot, &(_, sample_time)) in sequence.iter().enumerate() {
                v.set_sqrch(slot, sample_time as u8);
            }
        });
        regs.cr().modify(|v| {
            v.set_en(true);
            v.set_cont(false);
            v.set_slave(false);
            v.set_ens(sequence.len() as u8 - 1);
            v.set_clk(prescaler as u8);
        });
        self.timing = Timing {
            prescaler,
            frequency: clock_bounds.nominal(),
            clock_bounds,
            sample_time: sequence.last().unwrap().1,
        };
        if temperature || bandgap {
            critical_section::with(|_| {
                pac::BGR.cr().modify(|v| {
                    v.set_tsen(v.tsen() || temperature);
                    v.set_bgren(v.bgren() || bandgap);
                });
            });
            // Even inherited enables may have been set immediately before
            // this scan. The maximum HCLK bound keeps this wait >=50 us.
            self.delay_us(50);
        }
        regs.start().write(|v| v.set_start(true));
        for _ in 0..self.config.timeout {
            let status = regs.isr().read();
            if status.eoc() && status.eos() && !regs.start().read().start() {
                for (slot, result) in results.iter_mut().enumerate() {
                    *result = regs.result(slot).read().result();
                }
                regs.icr().write(|v| {
                    *v = pac::adc::regs::Icr::write_noop();
                    v.set_eoc(false);
                    v.set_eos(false);
                });
                return Ok(timing);
            }
        }
        self.shutdown();
        Err(Error::ConversionTimeout)
    }
    /// Internal temperature channel; results are raw counts.
    pub fn enable_temperature(&mut self) -> Temperature<T> {
        Temperature(PhantomData)
    }
    /// Internal nominal 1.2 V BGR measurement channel, not a selectable reference.
    pub fn enable_vrefint(&mut self) -> VrefInt<T> {
        VrefInt(PhantomData)
    }
    /// Apply configuration after validation. Invalid input preserves ADC state.
    pub fn set_config(&mut self, config: &Config) -> Result<(), Error> {
        check_fault::<T>()?;
        let timing = config.timing(self.pclk, 0, SampleTime::Cycles518)?;
        Self::check_sibling()?;
        self.shutdown();
        self.config = *config;
        self.timing = timing;
        self.initialize();
        Ok(())
    }

    /// Timing of the last slot at the common clock, or initial Cycles518 acquisition.
    pub fn timing(&self) -> Timing {
        self.timing
    }
    /// Fixed 12-bit resolution.
    pub fn resolution(&self) -> Resolution {
        Resolution::Bits12
    }
}
impl<'d, T: AsyncInstance> Adc<'d, T, Async> {
    /// Validate before converter writes, preserving shared BGR/TS and its clock.
    /// Inherited hardware triggers or watchdog enables are rejected without
    /// changing their configuration or flags. Raw PAC code must not concurrently
    /// reconfigure this exclusively owned ADC. A prior cleanup fault is terminal
    /// across both blocking and asynchronous replacement owners until chip reset.
    ///
    /// ADC2 also requires both actual DAC DMAUDRIE bits to be zero. Admission
    /// checks DAC reset before enabling its gate and brackets the CR0 read with
    /// gate/reset checks, before an ADC owner or any ADC2 writes exist. It never
    /// changes DAC registers, output routing or flags. A newly acquired DAC gate
    /// remains enabled even on rejection and may advance retained DAC activity.
    /// Current safe DAC construction, set/read and drop preserve zero interrupt
    /// enables, so direct outputs may coexist without borrowing the DAC token.
    /// Raw DAC interrupt/control writers require coordination outside this API.
    /// The IRQ must be bound to `InterruptHandler<T>` with `bind_interrupts!`.
    pub fn new_async(
        peripheral: Peri<'d, T>,
        common: &'d Common<'d>,
        _irqs: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
        config: Config,
    ) -> Result<Self, Error> {
        check_fault::<T>()?;
        let clocks = crate::rcc::try_clocks().ok_or(Error::ClockNotInitialized)?;
        let timing = config.timing(clocks.pclk_bounds(), 0, SampleTime::Cycles518)?;
        // Keep inherited-source rejection and initial masking
        // atomic with respect to a previously enabled/pending ADC vector.
        let mut adc = critical_section::with(|cs| {
            Self::check_sibling()?;
            if inherited_async_configuration::<T>() {
                return Err(Error::UnsupportedConfiguration);
            }
            if matches!(T::ASYNC_POLICY, AsyncPolicy::SharedDac) {
                check_fault::<T>()?;
                admit_dac_peer(config.timeout, cs)?;
                // Retire a forgotten owner's dispatch before local cleanup,
                // in the same critical section as peer admission and masking.
                T::state().irq_active.store(false, Ordering::Release);
            }
            let mut adc = Self {
                _peripheral: peripheral,
                config,
                pclk: clocks.pclk_bounds(),
                hclk: clocks.hclk_bounds(),
                timing,
                active: false,
                _common: common,
                async_lifecycle: true,
                _mode: PhantomData,
            };
            adc.async_cleanup()?;
            Ok(adc)
        })?;
        adc.initialize_async()?;
        T::state().irq_active.store(true, Ordering::Release);
        // Metadata proves the dedicated/shared policy; ADC2 has admitted its
        // quiet DAC peer. No pending clear is needed: masked EOS is ignored.
        unsafe { T::Interrupt::enable() };
        Ok(adc)
    }

    /// Read one borrowed channel through the same EOS-driven one-slot path.
    /// `Config::timeout` does not impose a deadline while waiting for EOS.
    /// Wrap this future in `with_timeout` or `select` for caller cancellation.
    pub async fn read<'a>(
        &mut self,
        channel: impl BorrowedChannel<'a, T>,
        sample_time: SampleTime,
    ) -> Result<u16, Error> {
        let channel = channel.reborrow_adc();
        let mut result = [0];
        self.read_sequence(&[(&channel, sample_time)], &mut result)
            .await?;
        Ok(result[0])
    }

    /// Read an ordered 1..=8-slot software sequence, retaining channel, sequence
    /// and output borrows until this future completes or is dropped. Repeated
    /// channel handles are allowed; each slot has its own sample time/result.
    /// Length and timing validation precede ADC changes. No ISR accesses caller
    /// storage; results change only after the full sequence and cleanup succeed.
    ///
    /// EOS wakes once; EOC and cleared START are then checked with the finite
    /// synchronous budget. There is no built-in elapsed-time deadline. Dropping
    /// this future masks completion, requests documented START stop/slot reset,
    /// disables owned ADC circuitry and preserves shared BGR/TS, their gate and watchdog
    /// flags. Successful logical cleanup permits a freshly initialized read.
    /// A failed control readback latches `Faulted`, including after reconstruction.
    /// Forgetting skips cleanup; the next operation explicitly stops and clears
    /// the ADC before reinitializing. No instantaneous analog cutoff, BUSY/drain
    /// acknowledgment, continuous acquisition or deep-sleep guarantee is made.
    pub async fn read_sequence(
        &mut self,
        sequence: &[(&BorrowedAdcChannel<'_, T>, SampleTime)],
        results: &mut [u16],
    ) -> Result<SequenceTiming, Error> {
        check_fault::<T>()?;
        if sequence.is_empty() || sequence.len() > MAX_SEQUENCE_LEN {
            return Err(Error::InvalidSequenceLength);
        }
        if results.len() != sequence.len() {
            return Err(Error::InvalidResultLength);
        }
        let mut prescaler = Prescaler::Div1;
        let mut conversion_cycles = 0;
        let mut temperature = false;
        let mut bandgap = false;
        for &(channel, sample_time) in sequence {
            let timing = self
                .config
                .timing(self.pclk, channel.channel, sample_time)?;
            if timing.prescaler as u8 > prescaler as u8 {
                prescaler = timing.prescaler;
            }
            conversion_cycles += u32::from(sample_time.conversion_cycles());
            temperature |= channel.channel == 14;
            bandgap |= channel.channel == 15;
        }
        // The slowest accepted divider satisfies every slot's upper frequency
        // and acquisition constraints; its selecting slot checked the minimum.
        let clock_bounds = self.pclk.divided_by(prescaler.divisor());
        let timing = SequenceTiming {
            prescaler,
            clock_bounds,
            length: sequence.len() as u8,
            conversion_cycles,
        };
        Self::check_sibling()?;
        // This also repairs a forgotten future; never rely on its destructor.
        self.async_cleanup()?;
        self.timing = Timing {
            prescaler,
            frequency: clock_bounds.nominal(),
            clock_bounds,
            sample_time: sequence.last().unwrap().1,
        };
        self.initialize_async()?;
        let regs = T::regs();
        regs.sqrcfr().write(|v| {
            for (slot, &(channel, _)) in sequence.iter().enumerate() {
                v.set_sqrch(slot, channel.channel);
            }
        });
        regs.sample().write(|v| {
            for (slot, &(_, sample_time)) in sequence.iter().enumerate() {
                v.set_sqrch(slot, sample_time as u8);
            }
        });
        critical_section::with(|_| {
            regs.cr().modify(|v| {
                v.set_cont(false);
                v.set_ens(sequence.len() as u8 - 1);
                v.set_slave(false);
            });
        });
        if temperature || bandgap {
            critical_section::with(|_| {
                pac::BGR.cr().modify(|v| {
                    v.set_tsen(v.tsen() || temperature);
                    v.set_bgren(v.bgren() || bandgap);
                });
            });
        }
        if !self.async_poll_control(|| {
            let cr = regs.cr().read();
            let channels = regs.sqrcfr().read();
            let samples = regs.sample().read();
            cr.en()
                && !cr.cont()
                && cr.ens() == sequence.len() as u8 - 1
                && cr.clk() == prescaler as u8
                && !cr.slave()
                && (!temperature || pac::BGR.cr().read().tsen())
                && (!bandgap || pac::BGR.cr().read().bgren())
                && sequence
                    .iter()
                    .enumerate()
                    .all(|(slot, &(channel, sample_time))| {
                        channels.sqrch(slot) == channel.channel
                            && samples.sqrch(slot) == sample_time as u8
                    })
        }) {
            let _ = self.async_cleanup();
            return Err(self.async_fault());
        }
        // Shared TS/BGR are monotonic. Either source can have just been
        // enabled by its comparator owner, so always settle either source used.
        if temperature || bandgap {
            self.delay_us(50);
        }
        if let Err(error) = self.async_clear_completion() {
            let _ = self.async_cleanup();
            return Err(error);
        }
        let mut guard = AsyncGuard {
            adc: self,
            armed: true,
        };
        let mut started = false;
        poll_fn(|cx| {
            T::state().waker.register(cx.waker());
            compiler_fence(Ordering::SeqCst);
            if let Err(error) = check_fault::<T>() {
                return Poll::Ready(Err(error));
            }
            if !started {
                critical_section::with(|_| {
                    regs.ier().modify(|v| {
                        v.set_eoc(false);
                        v.set_eos(true);
                    })
                });
                if !guard.adc.async_poll_control(|| {
                    let ier = regs.ier().read();
                    ier.eos() && !ier.eoc()
                }) {
                    return Poll::Ready(Err(guard.adc.async_fault()));
                }
                compiler_fence(Ordering::SeqCst);
                regs.start().write(|v| v.set_start(true));
                started = true;
            }
            if regs.isr().read().eos() {
                Poll::Ready(Ok(()))
            } else {
                Poll::Pending
            }
        })
        .await?;
        // EOS is sticky and its interrupt is now masked. Never await a second
        // wake if START/EOC lags EOS: finalize with this finite synchronous poll.
        let mut complete = false;
        for _ in 0..guard.adc.config.timeout {
            let status = regs.isr().read();
            if status.eos() && status.eoc() && !regs.start().read().start() {
                complete = true;
                break;
            }
        }
        if !complete {
            let cleanup = guard.adc.async_cleanup();
            guard.armed = false;
            cleanup?;
            return Err(Error::ConversionTimeout);
        }
        compiler_fence(Ordering::SeqCst);
        // Retained results cannot be overwritten by a following sequence:
        // CONT and all hardware triggers are off, and START is clear.
        let mut samples = [0; MAX_SEQUENCE_LEN];
        for (slot, result) in samples[..sequence.len()].iter_mut().enumerate() {
            *result = regs.result(slot).read().result() & Resolution::Bits12.max_count() as u16;
        }
        guard.adc.async_mask_completion()?;
        guard.adc.async_clear_completion()?;
        check_fault::<T>()?;
        results.copy_from_slice(&samples[..sequence.len()]);
        guard.armed = false;
        Ok(timing)
    }

    /// Internal raw temperature channel, with the normal startup/acquisition bounds.
    pub fn enable_temperature(&mut self) -> Temperature<T> {
        Temperature(PhantomData)
    }
    /// Internal raw bandgap channel; not a selectable conversion reference.
    pub fn enable_vrefint(&mut self) -> VrefInt<T> {
        VrefInt(PhantomData)
    }
    /// Validate configuration first, then stop and reinitialize. A terminal fault
    /// cannot be cleared by configuration changes or replacement owners.
    pub fn set_config(&mut self, config: &Config) -> Result<(), Error> {
        check_fault::<T>()?;
        let timing = config.timing(self.pclk, 0, SampleTime::Cycles518)?;
        Self::check_sibling()?;
        self.async_cleanup()?;
        self.config = *config;
        self.timing = timing;
        self.initialize_async()
    }
    /// Timing of the last slot at the sequence clock, or initial acquisition.
    pub fn timing(&self) -> Timing {
        self.timing
    }
    /// Fixed 12-bit resolution.
    pub fn resolution(&self) -> Resolution {
        Resolution::Bits12
    }
}

// A scoped owner borrow, never an ISR-visible pointer. Exists before EOS is
// armed and before START; every error/drop path after arming requests cleanup.
struct AsyncGuard<'a, 'd, T: Instance> {
    adc: &'a mut Adc<'d, T, Async>,
    armed: bool,
}
impl<T: Instance> Drop for AsyncGuard<'_, '_, T> {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.adc.async_cleanup();
        }
    }
}

impl<T: Instance, M: Mode> Adc<'_, T, M> {
    /// Whether an end-of-sequence conversion is pending.
    #[inline]
    pub fn end_of_sequence(&self) -> bool { adc_irq_eos::<T>() }
    /// Enable or mask the end-of-sequence interrupt.
    #[inline]
    pub fn enable_end_of_sequence_interrupt(&self, enable: bool) {
        adc_irq_enable_eos::<T>(enable)
    }
    /// Acknowledge conversion-complete flags.
    #[inline]
    pub fn clear_completion(&self) { adc_irq_clear::<T>() }

    fn async_poll_control(&self, mut ready: impl FnMut() -> bool) -> bool {
        for _ in 0..self.config.timeout {
            if ready() {
                return true;
            }
        }
        false
    }

    fn async_fault(&mut self) -> Error {
        if matches!(T::ASYNC_POLICY, AsyncPolicy::SharedDac) {
            self.active = false;
            return contain_shared_fault::<T>(self.config.timeout);
        }
        T::state().faulted.store(true, Ordering::Release);
        T::state().irq_active.store(false, Ordering::Release);
        // Only a failed control readback disables the dedicated ADC vector.
        // ADC1 exclusivity is asserted from metadata.
        T::disable_async_interrupt();
        self.active = false;
        Error::Faulted
    }

    fn async_mask_completion(&mut self) -> Result<(), Error> {
        let regs = T::regs();
        critical_section::with(|_| {
            regs.ier().modify(|v| {
                v.set_eoc(false);
                v.set_eos(false);
            })
        });
        if self.async_poll_control(|| {
            let ier = regs.ier().read();
            !ier.eoc() && !ier.eos()
        }) {
            check_fault::<T>()
        } else {
            Err(self.async_fault())
        }
    }

    fn async_clear_completion(&mut self) -> Result<(), Error> {
        // R1W0: zero acknowledges only EOC/EOS; one preserves both watchdogs.
        T::regs().icr().write(|v| {
            *v = pac::adc::regs::Icr::write_noop();
            v.set_eoc(false);
            v.set_eos(false);
        });
        if self.async_poll_control(|| {
            let status = T::regs().isr().read();
            !status.eoc() && !status.eos()
        }) {
            check_fault::<T>()
        } else {
            Err(self.async_fault())
        }
    }

    fn async_cleanup(&mut self) -> Result<(), Error> {
        let regs = T::regs();
        // Continue the bounded cleanup even if one readback fails. A latched
        // fault is never cleared when a later cleanup attempt happens to work.
        let masked = self.async_mask_completion().is_ok();
        regs.trigger().write(|_| {});
        regs.start().write(|v| v.set_start(false));
        let stopped = self.async_poll_control(|| {
            !regs.start().read().start() && !hardware_trigger_enabled::<T>()
        });
        critical_section::with(|_| {
            regs.cr().modify(|v| {
                v.set_en(false);
                v.set_slave(false);
                v.set_cont(false);
            })
        });
        let disabled = self.async_poll_control(|| {
            let cr = regs.cr().read();
            !cr.en() && !cr.slave() && !cr.cont()
        });
        let cleared = self.async_clear_completion().is_ok();
        self.active = false;
        if masked && stopped && disabled && cleared {
            check_fault::<T>()
        } else {
            Err(self.async_fault())
        }
    }

    fn initialize_async(&mut self) -> Result<(), Error> {
        check_fault::<T>()?;
        let regs = T::regs();
        critical_section::with(|_| {
            regs.cr().modify(|v| {
                v.set_cont(false);
                v.set_ens(0);
                v.set_slave(false);
                v.set_clk(self.timing.prescaler as u8);
                v.set_en(true);
            })
        });
        if !self.async_poll_control(|| {
            let cr = regs.cr().read();
            cr.en()
                && !cr.cont()
                && !cr.slave()
                && cr.ens() == 0
                && cr.clk() == self.timing.prescaler as u8
        }) {
            let _ = self.async_cleanup();
            return Err(self.async_fault());
        }
        // Source-qualified startup at the requested sequence's actual clock.
        let time_cycles = self.hclk.delay_cycles_us(50);
        let adc_cycles = self.hclk.cycles_for(self.timing.clock_bounds, 15);
        self.delay_cycles(time_cycles.max(adc_cycles));
        self.active = true;
        Ok(())
    }
}

/// PAC-free accessors for timer-triggered ADC users whose interrupt handler
/// cannot borrow the foreground `Adc` value.
#[inline]
pub fn adc_irq_eos<T: Instance>() -> bool { T::regs().isr().read().eos() }
#[inline]
pub fn adc_irq_result<T: Instance>(slot: usize) -> Option<u16> {
    (slot < crate::ADC_SEQUENCE_MAX_LEN).then(|| T::regs().result(slot).read().result())
}
#[inline]
pub fn adc_irq_enable_eos<T: Instance>(enable: bool) {
    T::regs().ier().modify(|v| v.set_eos(enable));
}
#[inline]
pub fn adc_irq_clear<T: Instance>() {
    T::regs().icr().write(|v| {
        v.set_eoc(false);
        v.set_eos(false);
        v.set_awdl(true);
        v.set_awdh(true);
    });
}

/// Configure a two-slot ATIM-CC4 triggered scan used by motor current loops.
/// Register details remain inside the HAL; the application only supplies the
/// two ADC mux channels and acquisition time.
pub fn configure_timer_triggered<T: Instance>(
    channels: [u8; 2],
    sample: SampleTime,
    clk: u8,
    trigger: AdcTrigger,
) {
    let regs = T::regs();
    regs.start().write(|v| v.set_start(false));
    regs.sqrcfr().write(|v| {
        v.set_sqrch(0, channels[0]);
        v.set_sqrch(1, channels[1]);
    });
    regs.sample().write(|v| {
        v.set_sqrch(0, sample as u8);
        v.set_sqrch(1, sample as u8);
    });
    regs.cr().modify(|v| {
        v.set_en(true);
        v.set_cont(false);
        v.set_slave(false);
        v.set_ens(1);
        v.set_clk(clk);
    });
    regs.icr().write(|v| {
        v.set_eoc(false);
        v.set_eos(false);
        v.set_awdl(true);
        v.set_awdh(true);
    });
    regs.ier().modify(|v| v.set_eos(true));
    regs.trigger().write(|v| match trigger {
        AdcTrigger::AtimCc4 => v.set_atimoc4refc(true),
    });
}

/// Configure a continuously running two-slot scan.
pub fn configure_continuous<T: Instance>(channels: [u8; 2], sample: SampleTime, clk: u8) {
    let regs = T::regs();
    regs.start().write(|v| v.set_start(false));
    regs.sqrcfr().write(|v| {
        v.set_sqrch(0, channels[0]);
        v.set_sqrch(1, channels[1]);
    });
    regs.sample().write(|v| {
        v.set_sqrch(0, sample as u8);
        v.set_sqrch(1, sample as u8);
    });
    regs.cr().modify(|v| {
        v.set_en(true);
        v.set_cont(true);
        v.set_slave(false);
        v.set_ens(1);
        v.set_clk(clk);
    });
    regs.ier().write(|v| {
        v.set_eoc(false);
        v.set_eos(false);
    });
    regs.trigger().write(|_| {});
    regs.start().write(|v| v.set_start(true));
}

fn hardware_trigger_enabled<T: Instance>() -> bool {
    let v = T::regs().trigger().read();
    v.atimtrgo()
        || v.atimtrgo2()
        || v.atimoc1refc()
        || v.atimoc2refc()
        || v.atimoc3refc()
        || v.atimoc4refc()
        || v.atimoc5refc()
        || v.atimoc6refc()
        || v.gtim1trgo()
        || v.gtim1oc1refc()
        || v.gtim1oc2refc()
        || v.gtim1oc3refc()
        || v.gtim1oc4refc()
        || v.gtim2trgo()
        || v.gtim2oc1refc()
        || v.gtim2oc2refc()
        || v.gtim2oc3refc()
        || v.gtim2oc4refc()
        || v.gtim3trgo()
        || v.gtim3oc1refc()
        || v.gtim3oc2refc()
        || v.gtim3oc3refc()
        || v.gtim3oc4refc()
        || v.gtim4trgo()
        || v.gtim4oc1refc()
        || v.gtim4oc2refc()
        || v.gtim4oc3refc()
        || v.gtim4oc4refc()
        || v.btim1trgo()
        || v.btim2trgo()
        || v.btim3trgo()
        || v.halltimtrgo()
}

fn inherited_async_configuration<T: Instance>() -> bool {
    let regs = T::regs();
    let ier = regs.ier().read();
    let awd = regs.awdcr().read();
    regs.cr().read().slave()
        || ier.dmaeoc()
        || ier.dmaeos()
        || hardware_trigger_enabled::<T>()
        || ier.awdl()
        || ier.awdh()
        || awd.in0()
        || awd.in1()
        || awd.in2()
        || awd.in3()
        || awd.in4()
        || awd.in5()
        || awd.in6()
        || awd.in7()
        || awd.in8()
        || awd.in9()
        || awd.in10()
        || awd.in11()
        || awd.in12()
        || awd.in13()
        || awd.in14()
        || awd.in15()
}

impl<T: Instance, M: Mode> Drop for Adc<'_, T, M> {
    fn drop(&mut self) {
        if self.async_lifecycle {
            let _ = self.async_cleanup();
            T::state().irq_active.store(false, Ordering::Release);
        } else {
            self.shutdown();
        }
    }
}

pub(crate) trait SealedInstance {
    const ASYNC_POLICY: AsyncPolicy;
    fn regs() -> pac::adc::Adc;
    fn sibling() -> pac::adc::Adc;
    fn state() -> &'static State;
    fn disable_async_interrupt();
}
/// A verified ADC instance, implemented only by generated chip metadata.
#[allow(private_bounds)]
pub trait Instance: PeripheralType + SealedInstance + 'static {}

/// Private, generated choice; sharing never inherits the dedicated fault policy.
pub(crate) enum AsyncPolicy {
    Dedicated,
    SharedDac,
}

pub(crate) trait SealedAsyncInstance {}
/// ADC with a metadata-proven interrupt policy and safe async lifetime.
/// ADC1 is dedicated; ADC2 checks its DAC peer before using `ADC2_DAC`.
#[allow(private_bounds)]
pub trait AsyncInstance: Instance + SealedAsyncInstance {
    /// The instance's GLOBAL interrupt, dedicated or admitted shared ADC2/DAC.
    type Interrupt: Interrupt;
}

/// Per-instance interrupt state. Never contains a caller buffer or channel pointer.
pub(crate) struct State {
    waker: AtomicWaker,
    irq_active: AtomicBool,
    faulted: AtomicBool,
}
impl State {
    pub(crate) const fn new() -> Self {
        Self {
            waker: AtomicWaker::new(),
            irq_active: AtomicBool::new(false),
            faulted: AtomicBool::new(false),
        }
    }
}

/// EOS-only software conversion handler, bound using `bind_interrupts!`.
/// The sticky completion is retained for the future; no sample is copied here.
/// ADC2 touches no DAC registers. Shared-vector disable occurs only when this
/// handler cannot verify that its owned EOS source was masked.
pub struct InterruptHandler<T: AsyncInstance>(PhantomData<T>);
impl<T: AsyncInstance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let state = T::state();
        let wake = critical_section::with(|_| {
            if !state.irq_active.load(Ordering::Acquire) {
                return false;
            }
            let regs = T::regs();
            if regs.ier().read().eos() && regs.isr().read().eos() {
                regs.ier().modify(|v| v.set_eos(false));
                // A failed masking readback must not trap the executor in an
                // IRQ storm. One read is a finite, conservative ISR check.
                if regs.ier().read().eos() {
                    state.faulted.store(true, Ordering::Release);
                    state.irq_active.store(false, Ordering::Release);
                    T::Interrupt::disable();
                }
                true
            } else {
                false
            }
        });
        if wake {
            state.waker.wake();
        }
    }
}

fn check_fault<T: Instance>() -> Result<(), Error> {
    if T::state().faulted.load(Ordering::Acquire) {
        Err(Error::Faulted)
    } else {
        Ok(())
    }
}

fn admit_dac_peer(timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<(), Error> {
    use crate::rcc::{Readback, SealedRccPeripheral};
    // This is DAC's distinct APBEN2/APBRST2 domain, never the common ADC gate.
    let rcc = crate::peripherals::DAC::RCC_INFO;
    if rcc.reset_asserted() {
        return Err(Error::DacInReset);
    }
    if !rcc.is_enabled() {
        // Retain even a delayed enable; closing this gate could affect outputs.
        rcc.enable_with_cs_readback(
            cs,
            Readback::Poll {
                attempts: timeout,
                spin: false,
            },
        )
        .map_err(|_| Error::DacClockUnavailable)?;
    }
    if !rcc.is_enabled() {
        return Err(Error::DacClockUnavailable);
    }
    if rcc.reset_asserted() {
        return Err(Error::DacInReset);
    }
    let control = pac::DAC.cr0().read();
    // Never interpret a zero sample from a closed/reset domain as quietness.
    if !rcc.is_enabled() {
        return Err(Error::DacClockUnavailable);
    }
    if rcc.reset_asserted() {
        return Err(Error::DacInReset);
    }
    if control.dmaudrie1() || control.dmaudrie2() {
        return Err(Error::DacInterruptEnabled);
    }
    // EN/OUT and masked sticky DMAUDR flags do not request the shared vector.
    // No DAC control, output, data or status register is written here.
    Ok(())
}

fn contain_shared_fault<T: Instance>(timeout: u32) -> Error {
    // A leaf: raw local masking/readback must not consult the new fault latch
    // or call fallible cleanup/mask wrappers that could reenter this function.
    critical_section::with(|_| {
        let state = T::state();
        state.faulted.store(true, Ordering::Release);
        let regs = T::regs();
        regs.ier().modify(|v| {
            v.set_eoc(false);
            v.set_eos(false);
        });
        let mut masked = false;
        for _ in 0..timeout {
            let ier = regs.ier().read();
            if !ier.eoc() && !ier.eos() {
                masked = true;
                break;
            }
        }
        state.irq_active.store(false, Ordering::Release);
        if !masked {
            // Only a failed owned-source mask permits whole-vector containment.
            // The admitted DAC peer has no enabled interrupt contribution.
            T::disable_async_interrupt();
        }
    });
    Error::Faulted
}

pub(crate) trait SealedAdcChannel<T> {
    fn setup(&mut self) {}
    fn channel(&self) -> u8;
}
/// A channel verified for this ADC. External channels own an Embassy pin token;
/// internal channels can only be obtained from their ADC instance.
#[allow(private_bounds)]
pub trait AdcChannel<'d, T>: SealedAdcChannel<T> + Sized {
    /// Consume and erase a channel's concrete type, retaining its ownership.
    fn degrade_adc(mut self) -> BorrowedAdcChannel<'d, T> {
        self.setup();
        BorrowedAdcChannel {
            channel: self.channel(),
            _marker: PhantomData,
        }
    }
    /// Borrow and erase a channel. Its pin cannot be used until this borrow ends.
    fn reborrow_adc(&mut self) -> BorrowedAdcChannel<'_, T> {
        self.setup();
        BorrowedAdcChannel {
            channel: self.channel(),
            _marker: PhantomData,
        }
    }
}
/// Type-erased owned or borrowed channel. Cannot be constructed with an arbitrary
/// channel number. Its lifetime retains the original channel's exclusive borrow.
pub struct BorrowedAdcChannel<'a, T> {
    channel: u8,
    _marker: PhantomData<&'a mut T>,
}
impl<T: Instance> BorrowedAdcChannel<'_, T> {
    /// Verified ADC mux index.
    pub fn get_hw_channel(&self) -> u8 {
        self.channel
    }
}
impl<T> SealedAdcChannel<T> for BorrowedAdcChannel<'_, T> {
    fn channel(&self) -> u8 {
        self.channel
    }
}
impl<'a, T> AdcChannel<'a, T> for BorrowedAdcChannel<'a, T> {}
trait SealedBorrowedChannel<'a, T> {
    fn reborrow_adc(self) -> BorrowedAdcChannel<'a, T>;
}
/// A mutable channel reference or already type-erased channel.
#[allow(private_bounds)]
pub trait BorrowedChannel<'a, T>: SealedBorrowedChannel<'a, T> {}
impl<'a, T, C: SealedBorrowedChannel<'a, T>> BorrowedChannel<'a, T> for C {}
impl<'a, 'd, T, C: AdcChannel<'d, T>> SealedBorrowedChannel<'a, T> for &'a mut C {
    fn reborrow_adc(self) -> BorrowedAdcChannel<'a, T> {
        AdcChannel::reborrow_adc(self)
    }
}
impl<'a, T> SealedBorrowedChannel<'a, T> for BorrowedAdcChannel<'a, T> {
    fn reborrow_adc(self) -> BorrowedAdcChannel<'a, T> {
        self
    }
}
/// Internal temperature sensor (channel 14), returning raw ADC counts.
pub struct Temperature<T: Instance>(PhantomData<T>);
/// Internal nominal 1.2 V bandgap source (channel 15).
pub struct VrefInt<T: Instance>(PhantomData<T>);
macro_rules! impl_internal {
    ($name:ident, $channel:literal) => {
        impl<T: Instance> SealedAdcChannel<T> for $name<T> {
            fn channel(&self) -> u8 {
                $channel
            }
        }
        impl<'d, T: Instance> AdcChannel<'d, T> for $name<T> {}
    };
}
impl_internal!(Temperature, 14);
impl_internal!(VrefInt, 15);

// Emitted only for the selected device and package's verified analog routes.
macro_rules! impl_instance {
    ($name:ident, $sibling:ident, $irq:ident, $policy:ident) => {
        impl $crate::adc::SealedInstance for $crate::peripherals::$name {
            const ASYNC_POLICY: $crate::adc::AsyncPolicy = $crate::adc::AsyncPolicy::$policy;
            fn regs() -> $crate::pac::adc::Adc { $crate::pac::$name }
            fn sibling() -> $crate::pac::adc::Adc { $crate::pac::$sibling }
            fn state() -> &'static $crate::adc::State {
                static STATE: $crate::adc::State = $crate::adc::State::new();
                &STATE
            }
            fn disable_async_interrupt() {
                <$crate::interrupt::typelevel::$irq as $crate::interrupt::typelevel::Interrupt>::disable();
            }
        }
        impl $crate::adc::Instance for $crate::peripherals::$name {}
        impl $crate::adc::SealedAsyncInstance for $crate::peripherals::$name {}
        impl $crate::adc::AsyncInstance for $crate::peripherals::$name {
            type Interrupt = $crate::interrupt::typelevel::$irq;
        }
    };
}
pub(crate) use impl_instance;
macro_rules! impl_pin {
    ($instance:ident, $pin:ident, $channel:literal) => {
        impl $crate::adc::SealedAdcChannel<$crate::peripherals::$instance>
            for $crate::Peri<'_, $crate::peripherals::$pin>
        {
            fn setup(&mut self) {
                // Flex enables/unlocks the port and disconnects the digital
                // path. Its drop also leaves analog mode and weak pulls off.
                let mut pin = $crate::gpio::Flex::new(self.reborrow());
                pin.set_as_analog();
            }
            fn channel(&self) -> u8 {
                $channel
            }
        }
        impl<'d> $crate::adc::AdcChannel<'d, $crate::peripherals::$instance>
            for $crate::Peri<'d, $crate::peripherals::$pin>
        {
        }
    };
}
pub(crate) use impl_pin;

impl<T: Instance, M: Mode> Adc<'_, T, M> {
    fn check_sibling() -> Result<(), Error> {
        if T::sibling().cr().read().slave() {
            Err(Error::SiblingInSlaveMode)
        } else {
            Ok(())
        }
    }
    fn clear_all_flags(&self) {
        T::regs().icr().write(|v| {
            *v = pac::adc::regs::Icr::write_noop();
            v.set_eoc(false);
            v.set_eos(false);
            v.set_awdl(false);
            v.set_awdh(false);
        });
    }
    fn initialize(&mut self) {
        let regs = T::regs();
        regs.start().write(|v| v.set_start(false));
        self.disable_converter();
        regs.trigger().write(|_| {});
        regs.ier().write(|_| {});
        regs.awdcr().write(|_| {});
        regs.sqrcfr().write(|_| {});
        regs.sample()
            .write(|v| v.set_sqrch(0, self.timing.sample_time as u8));
        self.clear_all_flags();
        regs.cr().modify(|v| {
            v.set_en(true);
            v.set_clk(self.timing.prescaler as u8);
        });
        // EN starts this converter (~1 us) and auto-starts sticky shared BGR
        // (~30 us). Cover both and DS tSTAB=15/fADC at ClockBounds extrema.
        let time_cycles = self.hclk.delay_cycles_us(50);
        let adc_cycles = self.hclk.cycles_for(self.timing.clock_bounds, 15);
        self.delay_cycles(time_cycles.max(adc_cycles));
        self.active = true;
    }
    fn disable_converter(&self) {
        // CR reset bit8 is reserved and set. Preserve all live reserved bits
        // while assigning every documented field of this owned converter.
        T::regs().cr().modify(|v| {
            v.set_en(false);
            v.set_cont(false);
            v.set_clk(0);
            v.set_ens(0);
            v.set_slave(false);
        });
    }
    fn delay_us(&self, microseconds: u32) {
        self.delay_cycles(self.hclk.delay_cycles_us(microseconds));
    }
    fn delay_cycles(&self, mut cycles: u64) {
        while cycles > u64::from(u32::MAX) {
            cortex_m::asm::delay(u32::MAX);
            cycles -= u64::from(u32::MAX);
        }
        cortex_m::asm::delay(cycles as u32);
    }
    fn shutdown(&mut self) {
        let regs = T::regs();
        regs.start().write(|v| v.set_start(false));
        regs.ier().write(|_| {});
        regs.trigger().write(|_| {});
        regs.awdcr().write(|_| {});
        self.disable_converter();
        self.clear_all_flags();
        self.active = false;
    }
}

fn clock_acquire(timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<(), Error> {
    use crate::rcc::{Readback, SealedRccPeripheral};
    if timeout == 0 {
        return Err(Error::InvalidTimeout);
    }
    // ADC1 and ADC2 describe the same common gate and active-low reset.
    // BGR has no independent gate. Never release an asserted common reset.
    let rcc = crate::peripherals::ADC1::RCC_INFO;
    if rcc.reset_asserted() {
        return Err(Error::CommonInReset);
    }
    if rcc.is_enabled() {
        return Ok(());
    }
    // No rollback may remove a shared gate, including a delayed enable.
    rcc.enable_with_cs_readback(
        cs,
        Readback::Poll {
            attempts: timeout,
            spin: false,
        },
    )
    .map_err(|_| Error::ClockEnableTimeout)
}
