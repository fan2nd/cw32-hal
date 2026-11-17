//! F030/A030 classic ATIM, using its typed CR/FLTR/DTR/CHxCR registers directly.
//!
//! Counter and dead-time timing are fixed for the lifetime of the whole-timer
//! owner. Outputs use fixed noninverted polarity. Construction leaves MOE zero;
//! enabling outputs is explicit. No electrical idle level, brake/rearm, pair-off,
//! endpoint, minimum pulse-width or board safety guarantee is provided.
use super::super::{
    Ch1, Ch2, Ch3, Channel, ComplementaryInstance, TimerChannel, TimerComplementaryPin,
    low_level::{
        ConfigError as TimerConfigError, CountingMode, Prescaler, Timing, select_timing_for,
    },
    simple_pwm::PwmPin,
};
use crate::{
    Peri,
    gpio::{Flex, Pull},
    pac,
    rcc::ClockBounds,
    time::Hertz,
};
use core::marker::PhantomData;

/// A package-qualified classic CHxB pad, retained with its AF disconnected.
pub struct ComplementaryPwmPin<'d, T: ComplementaryInstance, C: TimerChannel> {
    pin: Flex<'d>,
    af: u8,
    _phantom: PhantomData<(T, C)>,
}
impl<'d, T: ComplementaryInstance, C: TimerChannel> ComplementaryPwmPin<'d, T, C> {
    /// Own a CHxB pad without connecting its AF yet.
    pub fn new(pin: Peri<'d, impl TimerComplementaryPin<T, C>>) -> Self {
        let af = super::super::sealed::TimerComplementaryPin::<T, C>::af(&*pin);
        Self {
            pin: Flex::new(pin),
            af,
            _phantom: PhantomData,
        }
    }
    fn connect(mut self) -> Flex<'d> {
        self.pin.set_as_af(self.af, true, Pull::None);
        self.pin
    }
}

/// One complete, owned A+B output pair. Both pads remain disconnected until
/// construction has configured ATIM with its master output enable cleared.
pub struct ComplementaryPwmPair<'d, T: ComplementaryInstance, C: TimerChannel> {
    main: PwmPin<'d, T, C>,
    complementary: ComplementaryPwmPin<'d, T, C>,
}
impl<'d, T: ComplementaryInstance, C: TimerChannel> ComplementaryPwmPair<'d, T, C> {
    /// Retain the two package-qualified pads belonging to the same channel.
    pub fn new(main: PwmPin<'d, T, C>, complementary: ComplementaryPwmPin<'d, T, C>) -> Self {
        Self {
            main,
            complementary,
        }
    }
    fn connect(self) -> [Option<Flex<'d>>; 2] {
        [
            Some(self.main.connect()),
            Some(self.complementary.connect()),
        ]
    }
}

/// Constructor-only configuration. Frequency, prescaler and dead time stay fixed.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Minimum symmetric dead time in ns under the RCC clock envelope.
    /// Zero disables DTEN; a nonzero request rounds upward to one of the
    /// classic 2..1010 TCLK-tick delays, where TCLK=PCLK/PRS.
    pub dead_time_ns: u32,
}
/// Configuration rejected before ATIM or its RCC gate is changed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// Unsupported timing or uninitialized RCC.
    Timer(TimerConfigError),
    /// At least one complete A+B pair must be supplied.
    NoPairs,
    /// This source qualifies clock rate but not strict cycle or dead-time duration.
    UnqualifiedCycleTiming,
    /// The requested minimum exceeds code 255 at the fastest qualified clock.
    DeadTimeTooLong,
}
impl From<TimerConfigError> for ConfigError {
    fn from(value: TimerConfigError) -> Self {
        Self::Timer(value)
    }
}
/// Duty comparison rejected without changing registers or master output enable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DutyError {
    /// The requested pair was not supplied, or channel 4 was requested.
    UnavailableChannel,
    /// Only 0 < compare < period is qualified by this API.
    NonInteriorDuty,
}

fn dead_time_ticks(code: u8) -> u32 {
    2 + match code {
        0..=127 => u32::from(code),
        128..=191 => (64 + u32::from(code & 63)) * 2,
        192..=223 => (32 + u32::from(code & 31)) * 8,
        _ => (32 + u32::from(code & 31)) * 16,
    }
}
fn select_dead_time(bounds: ClockBounds, ns: u32) -> Result<Option<u8>, ConfigError> {
    if ns == 0 {
        return Ok(None);
    }
    (0..=u8::MAX)
        .find(|&code| bounds.minimum_duration_ns(dead_time_ticks(code)) >= u64::from(ns))
        .map(Some)
        .ok_or(ConfigError::DeadTimeTooLong)
}

/// Whole-ATIM owner with one to three complete pairs, including packages with
/// no CH2B pin. Absent pairs never connect pads. All supplied pairs share MOE.
///
/// The initial reference comparison is period/2; it is not a guaranteed 50%
/// pad duty after dead time. Duty writes preload CCRA for the next update.
/// Duty endpoints and independent pair gating are deliberately not exposed.
pub struct ComplementaryPwm<'d, T: ComplementaryInstance> {
    _peripheral: Peri<'d, T>,
    pins: [Option<Flex<'d>>; 6],
    timing: Timing,
    bounds: ClockBounds,
    dead_time: Option<u8>,
    duties: [u32; 3],
}
impl<'d, T: ComplementaryInstance> ComplementaryPwm<'d, T> {
    /// Validate before changing ATIM/RCC. Pin wrappers have already disconnected
    /// their pads; use reborrowed tokens to retain ownership on error. Rate-only
    /// clocks are rejected even when zero dead time is requested.
    pub fn new3(
        tim: Peri<'d, T>,
        ch1: Option<ComplementaryPwmPair<'d, T, Ch1>>,
        ch2: Option<ComplementaryPwmPair<'d, T, Ch2>>,
        ch3: Option<ComplementaryPwmPair<'d, T, Ch3>>,
        frequency: Hertz,
        counting_mode: CountingMode,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let CountingMode::EdgeAlignedUp = counting_mode;
        if ch1.is_none() && ch2.is_none() && ch3.is_none() {
            return Err(ConfigError::NoPairs);
        }
        let bounds =
            crate::rcc::bus_clock_bounds::<T>().ok_or(TimerConfigError::ClockNotInitialized)?;
        if !bounds.has_cycle_timing_bounds() {
            return Err(ConfigError::UnqualifiedCycleTiming);
        }
        let timing = select_timing_for::<T::CounterRegisters>(bounds.maximum(), frequency)?;
        let dead_time = select_dead_time(
            bounds.divided_by(timing.prescaler.divisor()),
            config.dead_time_ns,
        )?;
        let duty = timing.period() / 2;
        critical_section::with(|cs| {
            super::super::low_level::retire_overflow_state::<T>(cs);
            crate::rcc::enable_and_reset_with_cs::<T>(cs).expect("ATIM clock gate did not enable");
        });
        // Reset and all configuration occur with pads disconnected and MOE=0.
        pac::ATIM.cr().write(|v| *v = control(timing));
        pac::ATIM.dtr().write(|v| {
            v.set_dtr(dead_time.unwrap_or(0));
            v.set_dten(dead_time.is_some());
            // BKE, VCE, SAFEEN, AOE and MOE all retain their zero reset values.
        });
        pac::ATIM.mscr().write(|_| {}); // No slave/master/cascade request.
        pac::ATIM.trig().write(|_| {}); // No ADC trigger.
        pac::ATIM.rcr().write(|v| v.set_rcr(0));
        pac::ATIM.fltr().write(|v| {
            // In compare mode, 6 is single-point PWM1. COMP routes B from A;
            // every CCPxA/CCPxB stays noninverted. No filter or brake is enabled.
            v.set_ocm1aflt1a(6);
            v.set_ocm2aflt2a(6);
            v.set_ocm3aflt3a(6);
        });
        // CSA/CSB=0: output compare; A preload; all channel IRQ/DMA off.
        pac::ATIM.ch1cr().write(|v| v.set_bufea(true));
        pac::ATIM.ch2cr().write(|v| v.set_bufea(true));
        pac::ATIM.ch3cr().write(|v| v.set_bufea(true));
        pac::ATIM.ch4cr().write(|_| {}); // CH4 stays internal and disabled.
        pac::ATIM.ch1ccra().write(|v| v.set_ccr1a(duty as u16));
        pac::ATIM.ch2ccra().write(|v| v.set_ccr2a(duty as u16));
        pac::ATIM.ch3ccra().write(|v| v.set_ccr3a(duty as u16));
        pac::ATIM.ch1ccrb().write(|v| v.set_ccr1b(0));
        pac::ATIM.ch2ccrb().write(|v| v.set_ccr2b(0));
        pac::ATIM.ch3ccrb().write(|v| v.set_ccr3b(0));
        pac::ATIM.ch4ccr().write(|v| v.set_ccr4(0));
        pac::ATIM.arr().write(|v| v.set_arr(timing.reload));
        pac::ATIM.cnt().write(|v| v.set_cnt(0));
        // Fresh command value, never a broad CR read/restore: BG/TG cannot replay.
        pac::ATIM.cr().write(|v| {
            *v = control(timing);
            v.set_urs(false);
            v.set_ug(true);
        });
        let _ = pac::ATIM.cr().read();
        pac::ATIM.cr().write(|v| *v = control(timing));
        // Acknowledge only the generated update, preserving reserved bit1 and
        // every other event. ICR is a command register, never read-modify-write.
        pac::ATIM.icr().write(|v| {
            *v = pac::atim::regs::Icr::write_noop();
            v.set_uif(false);
        });
        let [a1, b1] = ch1
            .map(ComplementaryPwmPair::connect)
            .unwrap_or([None, None]);
        let [a2, b2] = ch2
            .map(ComplementaryPwmPair::connect)
            .unwrap_or([None, None]);
        let [a3, b3] = ch3
            .map(ComplementaryPwmPair::connect)
            .unwrap_or([None, None]);
        pac::ATIM.cr().write(|v| {
            *v = control(timing);
            v.set_en(true);
        });
        Ok(Self {
            _peripheral: tim,
            pins: [a1, b1, a2, b2, a3, b3],
            timing,
            bounds,
            dead_time,
            duties: [duty; 3],
        })
    }
    /// Explicitly enable/disable all supplied pairs using hardware DTR.MOE.
    /// Clearing MOE does not promise a pad voltage or pair-off electrical state.
    pub fn set_master_output_enable(&mut self, enable: bool) {
        pac::ATIM.dtr().modify(|v| v.set_moe(enable));
    }
    /// Read actual DTR.MOE, not a cached software state.
    pub fn get_master_output_enable(&self) -> bool {
        pac::ATIM.dtr().read().moe()
    }
    /// True only for a complete pair owned by this instance.
    pub fn has_channel(&self, channel: Channel) -> bool {
        channel.index() < 3 && self.pins[channel.index() * 2].is_some()
    }
    /// Number of TCLK ticks in one period. This is an exclusive upper bound on
    /// accepted comparison duty; zero and this value are not accepted endpoints.
    pub fn get_max_duty(&self) -> u32 {
        self.timing.period()
    }
    /// Last requested comparison, which may still be waiting for the next UEV.
    /// Panics for an absent pair or channel 4.
    pub fn get_duty(&self, channel: Channel) -> u32 {
        assert!(self.has_channel(channel), "unavailable complementary pair");
        self.duties[channel.index()]
    }
    /// Preload an interior A-reference comparison. Panics on an unavailable pair
    /// or an endpoint; the complementary pad pulse also depends on dead time.
    pub fn set_duty(&mut self, channel: Channel, duty: u32) {
        self.try_set_duty(channel, duty)
            .expect("invalid complementary duty");
    }
    /// Embassy STM32-compatible name for [`Self::set_duty`].
    pub fn set_duty_cycle(&mut self, channel: Channel, duty: u32) {
        self.set_duty(channel, duty);
    }
    /// Write only the selected CCRA. Does not change MOE, reset phase, or commit
    /// peers' pending comparisons. The next UEV latches the requested duty.
    pub fn try_set_duty(&mut self, channel: Channel, duty: u32) -> Result<(), DutyError> {
        if !self.has_channel(channel) {
            return Err(DutyError::UnavailableChannel);
        }
        if duty == 0 || duty >= self.timing.period() {
            return Err(DutyError::NonInteriorDuty);
        }
        match channel {
            Channel::Ch1 => pac::ATIM.ch1ccra().write(|v| v.set_ccr1a(duty as u16)),
            Channel::Ch2 => pac::ATIM.ch2ccra().write(|v| v.set_ccr2a(duty as u16)),
            Channel::Ch3 => pac::ATIM.ch3ccra().write(|v| v.set_ccr3a(duty as u16)),
            Channel::Ch4 => unreachable!(),
        }
        self.duties[channel.index()] = duty;
        Ok(())
    }
    /// Nominal counter frequency, using the qualified RCC nominal PCLK.
    pub fn get_frequency(&self) -> Hertz {
        self.timing.frequency(self.bounds.nominal())
    }
    /// Outward-rounded frequency envelope under the qualified RCC bounds.
    pub fn frequency_bounds(&self) -> (Hertz, Hertz) {
        let bounds = self.bounds.divided_by(self.timing.clock_divisor());
        (bounds.minimum(), bounds.maximum())
    }
    /// Selected PCLK divisor shared by the counter and dead-time clock.
    pub fn prescaler(&self) -> Prescaler {
        self.timing.prescaler
    }
    /// Encoded DTR value, or None when DTEN is disabled by a zero request.
    pub fn dead_time_code(&self) -> Option<u8> {
        self.dead_time
    }
    /// Programmed dead time in TCLK ticks. Zero means DTEN is disabled.
    pub fn dead_time_ticks(&self) -> u32 {
        self.dead_time.map(dead_time_ticks).unwrap_or(0)
    }
    /// Conservative dead-time range in ns using PCLK/PRS, with rational clock
    /// bounds retained until duration conversion. No nominal-clock substitution.
    pub fn dead_time_bounds_ns(&self) -> (u64, u64) {
        let bounds = self.bounds.divided_by(self.timing.prescaler.divisor());
        let ticks = self.dead_time_ticks();
        (
            bounds.minimum_duration_ns(ticks),
            bounds.maximum_duration_ns(ticks),
        )
    }
}

// x030 RM pp295–297: continuous edge-aligned up-counting, internal PCLK,
// COMP=1, PWM2S=1, ARR preload, overflow-only UEV. No interrupts/DMA/commands.
fn control(timing: Timing) -> pac::atim::regs::Cr {
    let mut value = pac::atim::regs::Cr::default();
    value.set_comp(true);
    value.set_pwm2s(true);
    value.set_prs(match timing.prescaler {
        Prescaler::Div256 => 7,
        other => {
            assert!(other.bits() <= 6);
            other.bits()
        }
    });
    value.set_bufpen(true);
    value.set_mode(2);
    value.set_urs(true);
    value
}
impl<T: ComplementaryInstance> Drop for ComplementaryPwm<'_, T> {
    fn drop(&mut self) {
        critical_section::with(|cs| {
            super::super::low_level::retire_overflow_state::<T>(cs);
            pac::ATIM.dtr().modify(|v| v.set_moe(false));
            // Fresh stopped control also masks all requests without replaying commands.
            pac::ATIM.cr().write(|v| *v = control(self.timing));
            for pin in self.pins.iter_mut().flatten() {
                pin.set_as_disconnected();
            }
            pac::ATIM.dtr().write(|_| {});
            crate::rcc::disable_with_cs::<T>(cs).expect("ATIM clock gate did not disable");
        });
    }
}
