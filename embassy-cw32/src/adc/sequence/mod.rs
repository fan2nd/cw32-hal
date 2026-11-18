//! L010/L011 12-bit, ordered 1–8 slot ADC sequences using each family's actual PAC.
//!
//! Blocking and interrupt-driven software conversions and the separately owned
//! L010/L011 `triggered` route are exposed. There is no READY,
//! overwrite flag, input follower, selectable internal reference, VDD/3, DMA,
//! or calibration API on this backend. Vdda means VDD
//! on L010 and VDDA on L011. Input must remain within 0..=reference and the
//! board's pin/supply ratings. L011 requires VDDA=VDD. Values are raw counts,
//! not calibrated units.
//!
//! The board declares its minimum supply; own-manual voltage bands and sample
//! rates, own-datasheet acquisition times, and L011's minimum 4 MHz ADC clock
//! are checked before register changes. These acquisition minima are only a
//! floor: high source impedance can require a longer SampleTime or an external
//! buffer, according to the selected family datasheet input-impedance limits. L011's manual 48 MHz maximum takes
//! precedence over the datasheet's conflicting 96 MHz maximum. Actual clocks
//! and waits include the source-qualified HSI envelope. The board must stay
//! inside rcc::HSI_BOUND_TEMPERATURE_C; extended-temperature HSI accuracy
//! beyond that interval is unqualified, even if other circuitry is rated.
//!
//! BGR is shared with VC1/VC2 through ADC_CR.BGREN. This driver never resets
//! ADC, clears BGREN, or disables a gate that was already enabled. After any
//! BGR use the gate and BGR remain enabled, including after drop. Conversion
//! and temperature circuitry are shut down. This deliberately trades power
//! for preserving a legitimate comparator owner; a future shared analog guard
//! may reclaim BGR only after every owner releases it. Raw ADC register access
//! must not concurrently change the conversion configuration or clear BGREN.
//!
//! Async IRQ/cancellation contract: docs/adc-low-async.md.
//! Current scan contract and source review: docs/adc-scan-sequences.md.
//! Historical electrical discrepancies: docs/adc-low-sequence-audit.md.
use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::mode::{Async, Blocking, Mode};
use crate::rcc::ClockBounds;
use crate::time::Hertz;
use crate::{Peri, PeripheralType, pac};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, Ordering, compiler_fence};
use core::task::Poll;
use embassy_sync::waitqueue::AtomicWaker;
#[cfg(trigger_btim1_update)]
pub mod triggered;

/// Maximum hardware slots, executed in caller-supplied order.
pub const MAX_SEQUENCE_LEN: usize = crate::ADC_SEQUENCE_MAX_LEN;

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
    /// 390 acquisition cycles.
    Cycles390 = 15,
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
    /// VDD on L010; VDDA on L011.
    Vdda,
}
/// ADC configuration. Acquisition duration is chosen separately for each read.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Supply reference; no selectable internal reference exists.
    pub reference: Reference,
    /// Guaranteed minimum supply in mV, including board supply tolerance.
    /// L010 accepts 1620..=5500; L011 accepts 1700..=5500. The board must
    /// independently satisfy the maximum supply and pin ratings. L011 requires
    /// VDDA=VDD (own datasheet table 7-4); separate rails are not supported.
    pub vdda_mv: u16,
    /// Requested maximum ADC clock, including HSI tolerance. Only PCLK/1, /2, /4, /8 exist.
    /// L011 additionally requires at least 4 MHz. A request may therefore be
    /// unattainable; `new`/`set_config`/`blocking_read` return an error.
    pub frequency: Hertz,
    /// Finite synchronous polling budget for setup, stop and finalization, plus
    /// blocking completion. Not an async deadline or duration in microseconds.
    /// Use caller `with_timeout`/`select` to cancel an asynchronous wait.
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
    /// The source qualifies average rate, but not the required cycle timing.
    UnqualifiedCycleTiming,
    /// Supply is outside the selected family's allowed range.
    InvalidSupplyVoltage,
    /// Requested maximum ADC frequency is zero.
    FrequencyZero,
    /// No divider meets the requested maximum and acquisition/rate limits.
    FrequencyTooLow,
    /// L011 ADC clock would fall below its documented 4 MHz minimum.
    ClockBelowMinimum,
    /// Polling budget is zero.
    InvalidTimeout,
    /// Peripheral clock gate failed to enable within the polling budget.
    ClockEnableTimeout,
    /// EOC, EOS and stopped START did not all occur within the budget.
    ConversionTimeout,
    /// A sequence must contain 1..=MAX_SEQUENCE_LEN slots.
    InvalidSequenceLength,
    /// A result slice must contain exactly one value per sequence slot.
    InvalidResultLength,
    /// Async acquisition found an inherited trigger or watchdog configuration.
    UnsupportedConfiguration,
    /// A required control readback failed. This ADC is unusable until chip reset,
    /// including through a reconstructed blocking or asynchronous owner.
    Faulted,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::ClockNotInitialized => "ADC clocks are not initialized",
            Self::UnqualifiedCycleTiming => "ADC source cycle timing is not qualified",
            Self::InvalidSupplyVoltage => "ADC supply is outside the allowed range",
            Self::FrequencyZero => "ADC frequency limit must be nonzero",
            Self::FrequencyTooLow => "no ADC divider meets the timing limits",
            Self::ClockBelowMinimum => "L011 ADC clock must be at least 4 MHz",
            Self::InvalidTimeout => "ADC polling budget must be nonzero",
            Self::ClockEnableTimeout => "ADC clock gate did not enable",
            Self::ConversionTimeout => "ADC conversion did not complete and stop",
            Self::InvalidSequenceLength => "ADC sequence length is outside the hardware range",
            Self::InvalidResultLength => "ADC result count must match the sequence length",
            Self::UnsupportedConfiguration => {
                "ADC has an inherited trigger or watchdog configuration"
            }
            Self::Faulted => "ADC control readback failed; chip reset required",
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
        if !pclk.has_cycle_timing_bounds() {
            return Err(Error::UnqualifiedCycleTiming);
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
/// Exclusive conversion owner. Pins are borrowed per read and remain analog.
/// Conversion failure stops the ADC/TS; a later read retries initialization.
/// Timing/configuration errors leave ADC registers and active state untouched.
/// Shared BGR and a preexisting clock gate are preserved (see module docs).
pub struct Adc<'d, T: Instance, M: Mode = Blocking> {
    _peripheral: Peri<'d, T>,
    config: Config,
    pclk: ClockBounds,
    hclk: ClockBounds,
    timing: Timing,
    active: bool,
    clock_was_enabled: bool,
    async_lifecycle: bool,
    _mode: PhantomData<M>,
}
impl<'d, T: Instance> Adc<'d, T, Blocking> {
    /// Validate first, acquire the clock without resetting shared BGR, and wait
    /// max(2 us, 15 ADCCLK cycles) for analog startup, conservatively
    /// covering the manual approximate 1 us and datasheet tSTAB=15/fADC.
    /// The default initial acquisition is Cycles390; reads choose their own.
    pub fn new(peripheral: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        check_fault::<T>()?;
        let clocks = crate::rcc::try_clocks().ok_or(Error::ClockNotInitialized)?;
        let timing = config.timing(clocks.pclk_bounds(), 0, SampleTime::Cycles390)?;
        let clock_was_enabled = critical_section::with(|cs| clock_acquire(config.timeout, cs))?;
        let mut adc = Self {
            _peripheral: peripheral,
            config,
            pclk: clocks.pclk_bounds(),
            hclk: clocks.hclk_bounds(),
            timing,
            active: false,
            clock_was_enabled,
            async_lifecycle: false,
            _mode: PhantomData,
        };
        adc.initialize();
        Ok(adc)
    }

    /// Fallible explicit blocking constructor.
    pub fn new_blocking(peripheral: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        Self::new(peripheral, config)
    }
    /// Read a verified pin or internal source. GPIO preparation can happen
    /// before timing rejection; the ADC itself remains unchanged on rejection.
    /// Internal sources require >=40 us acquisition, and wait 50 us
    /// when first enabled (TS datasheet maximum startup is 40 us).
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
    /// Convert one software-triggered sequence in the supplied slot order.
    ///
    /// L010/L011 have 1..=8 independently programmed slots. A mux may appear
    /// more than once; each slot has its own sample time and RESULT register.
    /// The result slice must match the sequence length exactly. Samples are
    /// sequential, not simultaneous. The returned timing uses one divider
    /// satisfying every slot's clock, acquisition and conversion-rate limits.
    ///
    /// Obtain channel handles with `degrade_adc` (owned) or `reborrow_adc`
    /// (borrowed). The slice borrows those handles for the entire conversion;
    /// referencing the same handle in several slots keeps the pin exclusively
    /// owned without manufacturing additional pin tokens. Internal sources use
    /// the same handles, startup waits and >=40 us acquisition checks as reads.
    ///
    /// Length and timing rejection leave ADC state and results unchanged.
    /// Results are copied only after EOS, EOC and hardware-cleared START. Timeout
    /// stops/resets the sequence, shuts down ADC/TS and leaves results unchanged;
    /// a later read initializes again. Shared BGR and its gate are retained.
    /// `Config::timeout` bounds polling for the whole sequence, not each slot.
    pub fn blocking_read_sequence(
        &mut self,
        sequence: &[(&BorrowedAdcChannel<'_, T>, SampleTime)],
        results: &mut [u16],
    ) -> Result<SequenceTiming, Error> {
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
        // Every slot already accepted its fastest valid divider. The slowest
        // accepted divider satisfies all upper bounds and acquisition minima;
        // its lower frequency bound was validated by the slot choosing it.
        let clock_bounds = self.pclk.divided_by(prescaler.divisor());
        let timing = SequenceTiming {
            prescaler,
            clock_bounds,
            length: sequence.len() as u8,
            conversion_cycles,
        };
        if !self.active {
            self.initialize();
        }
        let regs = T::regs();
        regs.start().write(|v| v.set_start(false)); // Reset slot cursor to zero.
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
        let newly_enabled_temperature = critical_section::with(|_| {
            let before = regs.cr().read();
            regs.cr().modify(|v| {
                v.set_en(true);
                v.set_cont(false);
                v.set_ens(sequence.len() as u8 - 1);
                v.set_clk(prescaler as u8);
                v.set_tsen(before.tsen() || temperature);
                v.set_bgren(before.bgren() || bandgap);
            });
            temperature && !before.tsen()
        });
        self.timing = Timing {
            prescaler,
            frequency: clock_bounds.nominal(),
            clock_bounds,
            sample_time: sequence.last().unwrap().1,
        };
        // An inherited BGR might have been enabled immediately before this
        // operation. Always settle it when used; TS needs settling on enable.
        if bandgap || newly_enabled_temperature {
            self.delay_us(50);
        }
        regs.start().write(|v| v.set_start(true));
        for _ in 0..self.config.timeout {
            let status = regs.isr().read();
            if status.eoc() && status.eos() && !regs.start().read().start() {
                for (slot, result) in results.iter_mut().enumerate() {
                    *result =
                        regs.result(slot).read().result() & Resolution::Bits12.max_count() as u16;
                }
                regs.icr().write(|v| {
                    v.set_eoc(false);
                    v.set_eos(false);
                    v.set_awdl(true);
                    v.set_awdh(true);
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
        let timing = config.timing(self.pclk, 0, SampleTime::Cycles390)?;
        self.shutdown();
        self.config = *config;
        self.timing = timing;
        self.initialize();
        Ok(())
    }

    /// Timing of the last slot at the common sequence clock, or initial Cycles390 acquisition.
    pub fn timing(&self) -> Timing {
        self.timing
    }
    /// Fixed 12-bit resolution.
    pub fn resolution(&self) -> Resolution {
        Resolution::Bits12
    }
}
impl<'d, T: Instance> Adc<'d, T, Async> {
    /// Validate before acquiring the gate, preserving shared BGR and its clock.
    /// Inherited hardware triggers or watchdog enables are rejected without
    /// changing their configuration or flags. Raw PAC code must not concurrently
    /// reconfigure this exclusively owned ADC. A prior cleanup fault is terminal
    /// across both blocking and asynchronous replacement owners until chip reset.
    /// The IRQ must be bound to `InterruptHandler<T>` with `bind_interrupts!`.
    pub fn new_async(
        peripheral: Peri<'d, T>,
        _irqs: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
        config: Config,
    ) -> Result<Self, Error> {
        check_fault::<T>()?;
        let clocks = crate::rcc::try_clocks().ok_or(Error::ClockNotInitialized)?;
        let timing = config.timing(clocks.pclk_bounds(), 0, SampleTime::Cycles390)?;
        // Keep gate acquisition, inherited-source rejection and initial masking
        // atomic with respect to a previously enabled/pending ADC vector.
        let mut adc = critical_section::with(|cs| {
            let clock_was_enabled = clock_acquire(config.timeout, cs)?;
            if inherited_async_configuration::<T>() {
                clock_release(T::regs(), clock_was_enabled, cs);
                return Err(Error::UnsupportedConfiguration);
            }
            let mut adc = Self {
                _peripheral: peripheral,
                config,
                pclk: clocks.pclk_bounds(),
                hclk: clocks.hclk_bounds(),
                timing,
                active: false,
                clock_was_enabled,
                async_lifecycle: true,
                _mode: PhantomData,
            };
            adc.async_cleanup()?;
            Ok(adc)
        })?;
        adc.initialize_async()?;
        T::state().irq_active.store(true, Ordering::Release);
        // Dedicated vector proven by the metadata generator. No pending clear
        // is needed: the handler ignores masked EOS and preserves sticky flags.
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
    /// disables owned ADC/TS circuitry and preserves BGR, its gate and watchdog
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
                v.set_tsen(temperature);
                v.set_bgren(v.bgren() || bandgap);
            });
        });
        if !self.async_poll_control(|| {
            let cr = regs.cr().read();
            let channels = regs.sqrcfr().read();
            let samples = regs.sample().read();
            cr.en()
                && !cr.cont()
                && cr.ens() == sequence.len() as u8 - 1
                && cr.clk() == prescaler as u8
                && cr.tsen() == temperature
                && (!bandgap || cr.bgren())
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
        // TS was shut down during cleanup. An inherited BGR can have just been
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
        let timing = config.timing(self.pclk, 0, SampleTime::Cycles390)?;
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
    /// Read a completed sequence slot without exposing the device PAC.
    #[inline]
    pub fn latest_result(&self, slot: usize) -> Option<u16> {
        if slot < crate::ADC_SEQUENCE_MAX_LEN {
            Some(T::regs().result(slot).read().result())
        } else {
            None
        }
    }

    /// Whether the end-of-sequence flag is pending.
    #[inline]
    pub fn end_of_sequence(&self) -> bool {
        T::regs().isr().read().eos()
    }

    /// Enable or mask the end-of-sequence interrupt.
    #[inline]
    pub fn enable_end_of_sequence_interrupt(&self, enable: bool) {
        T::regs().ier().modify(|v| v.set_eos(enable));
    }

    /// Acknowledge conversion-complete flags.
    #[inline]
    pub fn clear_completion(&self) {
        T::regs().icr().write(|v| {
            v.set_eoc(false);
            v.set_eos(false);
            v.set_awdl(true);
            v.set_awdh(true);
        });
    }

    fn async_poll_control(&self, mut ready: impl FnMut() -> bool) -> bool {
        for _ in 0..self.config.timeout {
            if ready() {
                return true;
            }
        }
        false
    }

    fn async_fault(&mut self) -> Error {
        T::state().faulted.store(true, Ordering::Release);
        T::state().irq_active.store(false, Ordering::Release);
        // Only a failed control readback disables the dedicated ADC vector.
        // Its exclusivity is asserted from metadata, never assumed for L012.
        T::Interrupt::disable();
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
            Ok(())
        } else {
            Err(self.async_fault())
        }
    }

    fn async_clear_completion(&mut self) -> Result<(), Error> {
        // R1W0: zero acknowledges only EOC/EOS; one preserves both watchdogs.
        T::regs().icr().write(|v| {
            v.set_eoc(false);
            v.set_eos(false);
            v.set_awdl(true);
            v.set_awdh(true);
        });
        if self.async_poll_control(|| {
            let status = T::regs().isr().read();
            !status.eoc() && !status.eos()
        }) {
            Ok(())
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
                v.set_tsen(false);
                v.set_cont(false);
            })
        });
        let disabled = self.async_poll_control(|| {
            let cr = regs.cr().read();
            !cr.en() && !cr.tsen() && !cr.cont()
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
                v.set_tsen(false);
                v.set_clk(self.timing.prescaler as u8);
                v.set_en(true);
            })
        });
        if !self.async_poll_control(|| {
            let cr = regs.cr().read();
            cr.en()
                && !cr.cont()
                && !cr.tsen()
                && cr.ens() == 0
                && cr.clk() == self.timing.prescaler as u8
        }) {
            let _ = self.async_cleanup();
            return Err(self.async_fault());
        }
        // Source-qualified startup at the requested sequence's actual clock.
        let time_cycles = self.hclk.delay_cycles_us(2);
        let adc_cycles = self.hclk.cycles_for(self.timing.clock_bounds, 15);
        self.delay_cycles(time_cycles.max(adc_cycles));
        self.active = true;
        Ok(())
    }
}

fn hardware_trigger_enabled<T: Instance>() -> bool {
    let v = T::regs().trigger().read();
    let enabled = v.atimtrgo()
        || v.atimtrgo2()
        || v.atimcc1()
        || v.atimcc2()
        || v.atimcc3()
        || v.atimcc4()
        || v.atimcc5()
        || v.atimcc6()
        || v.gtim1trgo()
        || v.gtim1cc1()
        || v.gtim1cc2()
        || v.gtim1cc3()
        || v.gtim1cc4()
        || v.btim1trgo()
        || v.btim2trgo()
        || v.btim3trgo()
        || v.spi1()
        || v.uart1()
        || v.uart2();
    #[cfg(adc_cw32l011_v1)]
    let enabled = enabled
        || v.uart3()
        || v.gtim2trgo()
        || v.gtim2cc1()
        || v.gtim2cc2()
        || v.gtim2cc3()
        || v.gtim2cc4();
    enabled
}

/// PAC-free accessors for timer-triggered ADC users whose interrupt handler
/// cannot borrow the foreground `Adc` value.
#[inline]
pub fn adc_irq_eos<T: Instance>() -> bool {
    T::regs().isr().read().eos()
}
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

fn inherited_async_configuration<T: Instance>() -> bool {
    let regs = T::regs();
    let ier = regs.ier().read();
    let awd = regs.awdcr().read();
    hardware_trigger_enabled::<T>()
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
        // A failed stop keeps the gate; do not turn an unresolved peripheral off.
        if check_fault::<T>().is_ok() {
            critical_section::with(|cs| clock_release(T::regs(), self.clock_was_enabled, cs));
        }
    }
}

pub(crate) trait SealedInstance {
    fn regs() -> pac::adc::Adc;
    fn state() -> &'static State;
}
/// A verified ADC instance, implemented only by generated chip metadata.
#[allow(private_bounds)]
pub trait Instance: PeripheralType + SealedInstance + 'static {
    /// Dedicated ADC interrupt, obtained from the selected instance's GLOBAL link.
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
pub struct InterruptHandler<T: Instance>(PhantomData<T>);
impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
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
    ($name:ident, $irq:ident) => {
        impl $crate::adc::SealedInstance for $crate::peripherals::$name {
            fn regs() -> $crate::pac::adc::Adc {
                $crate::pac::$name
            }
            fn state() -> &'static $crate::adc::State {
                static STATE: $crate::adc::State = $crate::adc::State::new();
                &STATE
            }
        }
        impl $crate::adc::Instance for $crate::peripherals::$name {
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
    fn clear_all_flags(&self) {
        T::regs().icr().write(|v| {
            v.set_eoc(false);
            v.set_eos(false);
            v.set_awdl(false);
            v.set_awdh(false);
        });
    }
    fn initialize(&mut self) {
        let regs = T::regs();
        regs.start().write(|v| v.set_start(false));
        critical_section::with(|_| {
            let bandgap = regs.cr().read().bgren();
            regs.cr().write(|v| v.set_bgren(bandgap));
        });
        regs.trigger().write(|_| {});
        regs.ier().write(|_| {});
        regs.awdcr().write(|_| {});
        regs.sqrcfr().write(|_| {});
        regs.sample()
            .write(|v| v.set_sqrch(0, self.timing.sample_time as u8));
        self.clear_all_flags();
        critical_section::with(|_| {
            regs.cr().modify(|v| {
                v.set_en(true);
                v.set_clk(self.timing.prescaler as u8);
            });
        });
        // RM: approximately 1 us after EN. DS: tSTAB = 15/fADC; cover both.
        let time_cycles = self.hclk.delay_cycles_us(2);
        let adc_cycles = self.hclk.cycles_for(self.timing.clock_bounds, 15);
        self.delay_cycles(time_cycles.max(adc_cycles));
        self.active = true;
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
        critical_section::with(|_| {
            let bandgap = regs.cr().read().bgren();
            regs.cr().write(|v| v.set_bgren(bandgap));
        });
        self.clear_all_flags();
        self.active = false;
    }
}

fn clock_acquire(timeout: u32, cs: critical_section::CriticalSection<'_>) -> Result<bool, Error> {
    use crate::rcc::{Readback, SealedRccPeripheral};
    let rcc = crate::peripherals::ADC::RCC_INFO;
    let was_enabled = rcc.is_enabled();
    // Preserve the caller's finite budget without resetting shared BGR.
    rcc.enable_with_cs_readback(
        cs,
        Readback::Poll {
            attempts: timeout,
            spin: false,
        },
    )
    .map_err(|_| Error::ClockEnableTimeout)?;
    Ok(was_enabled)
}
fn clock_release(adc: pac::adc::Adc, was_enabled: bool, cs: critical_section::CriticalSection<'_>) {
    use crate::rcc::SealedRccPeripheral;
    // Never tear down a gate inherited from another owner or used by BGR.
    if !was_enabled && !adc.cr().read().bgren() {
        let _ = crate::peripherals::ADC::RCC_INFO.disable_with_cs(cs);
    }
}
