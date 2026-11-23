//! Owned buffered-ATIM complementary PWM, with symmetric dead time and BK1.
//!
//! Construction leaves MOE and every channel disabled. Configure duty, enable
//! desired pairs, then explicitly set master output enable. BK1 is unfiltered;
//! AOE remains zero, so a break never causes automatic output restart. The driver
//! reads hardware MOE; it does not maintain a software copy of fault state.
//!
//! CKD=/1, active-high outputs, LOCK=0, OSSI=OSSR=1 and zero idle-state bits are
//! fixed. No board-level shutdown voltage, motor safety, glitch-free update or
//! fault-latch guarantee is made. See `docs/atim-complementary-pwm.md`.
use super::super::{
    Ch1, Ch2, Ch3, Ch4, Channel, ComplementaryInstance, TimerBreakPin, TimerChannel,
    TimerComplementaryPin,
    low_level::{ConfigError as TimerConfigError, CountingMode, Timer, select_timing_for},
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

/// A complementary output pad kept disconnected until the owner is configured.
pub struct ComplementaryPwmPin<'d, T: ComplementaryInstance, C: TimerChannel> {
    pin: Flex<'d>,
    af: u8,
    _phantom: PhantomData<(T, C)>,
}
impl<'d, T: ComplementaryInstance, C: TimerChannel> ComplementaryPwmPin<'d, T, C> {
    /// Own a package-qualified complementary pad without connecting its AF.
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

/// Active level at the owned external BK1 pad, before its AF1 input inversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BreakInputPolarity {
    /// A high input requests break.
    ActiveHigh,
    /// A low input requests break.
    ActiveLow,
}
/// Owned external BK1 input. L010 has no qualified route in this bounded API.
pub struct BreakInput<'d, T: ComplementaryInstance> {
    pin: Flex<'d>,
    af: u8,
    polarity: BreakInputPolarity,
    pull: Pull,
    _phantom: PhantomData<T>,
}
impl<'d, T: ComplementaryInstance> BreakInput<'d, T> {
    /// Own the pad; the constructor connects the selected input AF before outputs.
    /// GPIO pull restrictions still apply. Use an external pull-down for active-high
    /// BK on the qualified routes; L012's internal pull-down is limited to PF3.
    pub fn new(
        pin: Peri<'d, impl TimerBreakPin<T>>,
        polarity: BreakInputPolarity,
        pull: Pull,
    ) -> Self {
        let af = super::super::sealed::TimerBreakPin::<T>::af(&*pin);
        Self {
            pin: Flex::new(pin),
            af,
            polarity,
            pull,
            _phantom: PhantomData,
        }
    }
    fn connect(mut self) -> Flex<'d> {
        self.pin.set_as_af(self.af, false, self.pull);
        self.pin
    }
}

/// Constructor configuration, fixed for the owner's lifetime.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Minimum symmetric dead time in nanoseconds under the qualified RCC
    /// envelope. Rounded upward to the next supported DTG value; never saturated.
    pub dead_time_ns: u32,
}
/// Configuration rejected before timer register access.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// Unsupported counter timing or uninitialized RCC.
    Timer(TimerConfigError),
    /// The source qualifies rate, but not the required cycle/dead-time bounds.
    UnqualifiedCycleTiming,
    /// The requested minimum exceeds DTG=255 at the fastest qualified PCLK.
    DeadTimeTooLong,
}
impl From<TimerConfigError> for ConfigError {
    fn from(value: TimerConfigError) -> Self {
        Self::Timer(value)
    }
}
fn dead_time_ticks(bits: u8) -> u32 {
    match bits {
        0..=127 => u32::from(bits),
        128..=191 => (64 + u32::from(bits & 63)) * 2,
        192..=223 => (32 + u32::from(bits & 31)) * 8,
        _ => (32 + u32::from(bits & 31)) * 16,
    }
}
fn select_dead_time(bounds: ClockBounds, nanoseconds: u32) -> Result<u8, ConfigError> {
    (0..=u8::MAX)
        .find(|&bits| bounds.minimum_duration_ns(dead_time_ticks(bits)) >= u64::from(nanoseconds))
        .ok_or(ConfigError::DeadTimeTooLong)
}

/// Whole-timer owner with up to four main/complementary pairs and one BK1 pin.
///
/// Pins and ATIM cannot be independently reused while this owner exists.
/// Counter timing/dead time cannot be changed at runtime in this bounded subset.
/// Pair disable and Drop do not promise an electrical level at the pad.
pub struct ComplementaryPwm<'d, T: ComplementaryInstance> {
    inner: Timer<'d, T>,
    pins: [Option<Flex<'d>>; 8],
    break_pin: Option<Flex<'d>>,
    dead_time: u8,
    duties: [u32; 4],
}

/// Interrupt-side access for a PWM owner retained by the foreground. The
/// foreground `ComplementaryPwm` remains the owner; these helpers only update
/// preloaded compare values and the global safety gate from an ISR.
#[inline]
pub fn irq_moe<T: ComplementaryInstance>(enable: bool) { pac::ATIM.bdtr().modify(|v| v.set_moe(enable)); }
#[inline]
pub fn irq_arr<T: ComplementaryInstance>() -> u32 { u32::from(pac::ATIM.arr().read().arr()) + 1 }
#[inline]
pub fn irq_ccr<T: ComplementaryInstance>(channel: Channel, duty: u16) {
    match channel {
        Channel::Ch1 => pac::ATIM.ccr1().write(|v| v.set_ccr1(duty)),
        Channel::Ch2 => pac::ATIM.ccr2().write(|v| v.set_ccr2(duty)),
        Channel::Ch3 => pac::ATIM.ccr3().write(|v| v.set_ccr3(duty)),
        Channel::Ch4 => pac::ATIM.ccr4().write(|v| v.set_ccr4(duty)),
    }
}
#[inline]
pub fn irq_ccr_read<T: ComplementaryInstance>(channel: Channel) -> u16 {
    match channel {
        Channel::Ch1 => pac::ATIM.ccr1().read().ccr1(),
        Channel::Ch2 => pac::ATIM.ccr2().read().ccr2(),
        Channel::Ch3 => pac::ATIM.ccr3().read().ccr3(),
        Channel::Ch4 => pac::ATIM.ccr4().read().ccr4(),
    }
}
#[inline]
pub fn irq_center<T: ComplementaryInstance>() {
    let cr1 = pac::ATIM.cr1();
    cr1.modify(|v| v.set_cen(false));
    cr1.modify(|v| v.set_cms(1));
    cr1.modify(|v| v.set_cen(true));
}
#[inline]
pub fn irq_cc4<T: ComplementaryInstance>(compare: u16) {
    pac::ATIM.ccr4().write(|v| v.set_ccr4(compare));
    pac::ATIM.ccmr2cmp().modify(|v| { v.set_oc4pe(true); v.set_oc4m(7); });
    pac::ATIM.ccer().modify(|v| { v.set_cc4p(false); v.set_cc4e(true); });
}
impl<'d, T: ComplementaryInstance> ComplementaryPwm<'d, T> {
    /// Construct a running counter with all channel gates and MOE disabled.
    /// Output polarity is active high.
    ///
    /// Pin wrappers disconnect their pads before this call;
    /// errors consume tokens and leave those pads disconnected. Use reborrows to
    /// retain ownership after errors. No timer registers or output connections
    /// change on invalid timing, including rate-only sources and zero dead time.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tim: Peri<'d, T>,
        ch1: Option<PwmPin<'d, T, Ch1>>,
        ch1n: Option<ComplementaryPwmPin<'d, T, Ch1>>,
        ch2: Option<PwmPin<'d, T, Ch2>>,
        ch2n: Option<ComplementaryPwmPin<'d, T, Ch2>>,
        ch3: Option<PwmPin<'d, T, Ch3>>,
        ch3n: Option<ComplementaryPwmPin<'d, T, Ch3>>,
        ch4: Option<PwmPin<'d, T, Ch4>>,
        ch4n: Option<ComplementaryPwmPin<'d, T, Ch4>>,
        break_input: Option<BreakInput<'d, T>>,
        frequency: Hertz,
        counting_mode: CountingMode,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let CountingMode::EdgeAlignedUp = counting_mode;
        let clock =
            crate::rcc::bus_frequency::<T>().ok_or(TimerConfigError::ClockNotInitialized)?;
        let bounds =
            crate::rcc::bus_clock_bounds::<T>().ok_or(TimerConfigError::ClockNotInitialized)?;
        if !bounds.has_cycle_timing_bounds() {
            return Err(ConfigError::UnqualifiedCycleTiming);
        }
        let timing = select_timing_for::<T::CounterRegisters>(bounds.maximum(), frequency)?;
        let dead_time = select_dead_time(bounds, config.dead_time_ns)?;
        assert!(dead_time_ticks(dead_time) <= u32::from(T::DEAD_TIME_MAX_TICKS));
        let mut inner = Timer::new_configured(tim, clock, bounds, timing);
        // Reset/initialization has already stopped CEN, cleared CCER and MOE,
        // selected CKD=/1, disabled asymmetric/preloaded DT and zeroed OISy/N.
        // LOCK=0 remains writable; no nonzero lock level is requested here.
        // The reference C initialization leaves BK1 disabled unless a real
        // break source is routed.  Enabling BKE with no BKIN route can still
        // admit a package/system break source and clear MOE asynchronously,
        // which looks like a PWM polarity or dead-output fault to users.
        pac::ATIM.bdtr().write(|v| {
            v.set_dtg(dead_time);
            v.set_ossi(true);
            v.set_ossr(true);
            v.set_bke(break_input.is_some());
            v.set_bkp(true); // OR-combined break request active high.
        });
        // BKE/BKP writes take one APB cycle to apply. Complete a bus read before
        // routing the pad. MOE remains zero throughout every following step.
        let _ = pac::ATIM.bdtr().read();
        pac::ATIM.af1().write(|v| {
            v.set_bkine(break_input.is_some());
            v.set_bkinp(
                break_input
                    .as_ref()
                    .is_some_and(|p| p.polarity == BreakInputPolarity::ActiveLow),
            );
        });
        let break_pin = break_input.map(BreakInput::connect);
        let pins = [
            ch1.map(PwmPin::connect),
            ch1n.map(ComplementaryPwmPin::connect),
            ch2.map(PwmPin::connect),
            ch2n.map(ComplementaryPwmPin::connect),
            ch3.map(PwmPin::connect),
            ch3n.map(ComplementaryPwmPin::connect),
            ch4.map(PwmPin::connect),
            ch4n.map(ComplementaryPwmPin::connect),
        ];
        inner.start();
        Ok(Self {
            inner,
            pins,
            break_pin,
            dead_time,
            duties: [0; 4],
        })
    }
    /// Enable each owned pad in the pair. This never changes MOE or clears a break.
    pub fn enable(&mut self, channel: Channel) {
        let i = channel.index() * 2;
        set_enabled(channel, self.pins[i].is_some(), self.pins[i + 1].is_some());
    }
    /// Disable both gates in the pair, retaining duty. No pad-level guarantee.
    pub fn disable(&mut self, channel: Channel) {
        set_enabled(channel, false, false);
    }
    /// Enable or disable either side of one owned half-bridge.
    ///
    /// This is independent from MOE: channel gating selects which pairs are
    /// connected to the timer, while MOE remains the global power-stage gate.
    pub fn set_channel_enable(&mut self, channel: Channel, main: bool, complementary: bool) {
        assert!(
            self.pins[channel.index() * 2].is_some(),
            "unavailable PWM channel"
        );
        set_enabled(channel, main, complementary);
    }
    /// Explicitly write MOE. An asserted break can keep it cleared; inspect
    /// `get_master_output_enable`. This is not an atomic fault-clear/rearm protocol.
    pub fn set_master_output_enable(&mut self, enable: bool) {
        pac::ATIM.bdtr().modify(|v| v.set_moe(enable));
    }
    /// Read actual hardware MOE, including asynchronous break clearing.
    pub fn get_master_output_enable(&self) -> bool {
        pac::ATIM.bdtr().read().moe()
    }
    /// Set the same active polarity on the main and complementary outputs.
    pub fn set_polarity(&mut self, channel: Channel, polarity: super::OutputPolarity) {
        self.set_main_polarity(channel, polarity);
        self.set_complementary_polarity(channel, polarity);
    }
    /// Set only the main output polarity.
    pub fn set_main_polarity(&mut self, channel: Channel, polarity: super::OutputPolarity) {
        assert!(
            self.pins[channel.index() * 2].is_some(),
            "unavailable PWM channel"
        );
        let active_low = polarity == super::OutputPolarity::ActiveLow;
        pac::ATIM.ccer().modify(|v| match channel {
            Channel::Ch1 => v.set_cc1p(active_low),
            Channel::Ch2 => v.set_cc2p(active_low),
            Channel::Ch3 => v.set_cc3p(active_low),
            Channel::Ch4 => v.set_cc4p(active_low),
        });
    }
    /// Set only the complementary output polarity.
    pub fn set_complementary_polarity(
        &mut self,
        channel: Channel,
        polarity: super::OutputPolarity,
    ) {
        assert!(
            self.pins[channel.index() * 2 + 1].is_some(),
            "unavailable PWM channel"
        );
        let active_low = polarity == super::OutputPolarity::ActiveLow;
        pac::ATIM.ccer().modify(|v| match channel {
            Channel::Ch1 => v.set_cc1np(active_low),
            Channel::Ch2 => v.set_cc2np(active_low),
            Channel::Ch3 => v.set_cc3np(active_low),
            Channel::Ch4 => v.set_cc4np(active_low),
        });
    }
    /// Generate software BK1. Hardware clears MOE and sets BIF.
    pub fn trigger_software_break(&mut self) {
        pac::ATIM.egr().write(|v| v.set_bg(true));
    }
    /// Read the coalescing BK1 event flag; this is not a live pad-level reading.
    pub fn is_break_pending(&self) -> bool {
        pac::ATIM.isr().read().bif()
    }
    /// Acknowledge BIF only, preserving every other flag with an explicit R1W0
    /// write. Active break sources prevent clearing; MOE is never re-enabled.
    pub fn clear_break(&mut self) {
        pac::ATIM.icr().write(|v| {
            v.set_uif(true);
            v.set_cc1if(true);
            v.set_cc2if(true);
            v.set_cc3if(true);
            v.set_cc4if(true);
            v.set_comif(true);
            v.set_tif(true);
            v.set_b2if(true);
            v.set_sbif(true);
            v.set_cc1of(true);
            v.set_cc2of(true);
            v.set_cc3of(true);
            v.set_cc4of(true);
            v.set_cc5if(true);
            v.set_cc6if(true);
            v.set_cc5of(true);
            v.set_cc6of(true);
            v.set_idxf(true);
            v.set_dirf(true);
            v.set_ierrf(true);
            v.set_terrf(true);
        });
    }
    /// Set 0…`get_max_duty` main-active ticks; the N output is complementary.
    /// Interior changes use CCR preload and latch on update. Transitions to/from
    /// an endpoint clear global MOE, restart the whole period, commit all pending
    /// CCRs and leave MOE disabled until explicitly re-enabled. No glitch-free
    /// or pulse-width guarantee applies during any runtime change.
    pub fn set_duty(&mut self, channel: Channel, duty: u32) {
        let period = self.get_max_duty();
        assert!(duty <= period, "duty exceeds period");
        let mode = |value| {
            if value == 0 {
                4
            } else if value == period {
                5
            } else {
                6
            }
        };
        let changed_mode = mode(duty) != mode(self.duties[channel.index()]);
        if changed_mode {
            self.set_master_output_enable(false);
            self.inner.stop();
        }
        match channel {
            Channel::Ch1 => pac::ATIM
                .ccr1()
                .write(|v| v.set_ccr1(duty.min(65535) as u16)),
            Channel::Ch2 => pac::ATIM
                .ccr2()
                .write(|v| v.set_ccr2(duty.min(65535) as u16)),
            Channel::Ch3 => pac::ATIM
                .ccr3()
                .write(|v| v.set_ccr3(duty.min(65535) as u16)),
            Channel::Ch4 => pac::ATIM
                .ccr4()
                .write(|v| v.set_ccr4(duty.min(65535) as u16)),
        }
        if changed_mode {
            set_mode(channel, 0); // Freeze -> PWM refreshes comparison in own manuals.
            set_mode(channel, mode(duty));
            pac::ATIM.egr().write(|v| v.set_ug(true));
            self.inner.start();
        }
        self.duties[channel.index()] = duty;
    }
    /// Embassy STM32-compatible name for [`Self::set_duty`].
    pub fn set_duty_cycle(&mut self, channel: Channel, duty: u32) {
        self.set_duty(channel, duty);
    }
    /// Last requested main-active ticks; a buffered change may still be pending.
    pub fn get_duty(&self, channel: Channel) -> u32 {
        self.duties[channel.index()]
    }
    /// Full counter period, including an exactly representable API value of 65536.
    pub fn get_max_duty(&self) -> u32 {
        self.inner.period_ticks()
    }
    /// Nominal frequency rounded down to whole hertz.
    pub fn get_frequency(&self) -> Hertz {
        self.inner.get_frequency()
    }
    /// Outward-rounded frequency bounds under the declared RCC conditions.
    pub fn frequency_bounds(&self) -> (Hertz, Hertz) {
        self.inner.frequency_bounds()
    }
    /// Qualified PCLK envelope; dead time is independent of the counter prescaler.
    pub fn kernel_clock_bounds(&self) -> ClockBounds {
        self.inner.kernel_clock_bounds()
    }
    /// Outward-rounded symmetric dead-time interval, in nanoseconds.
    pub fn dead_time_bounds_ns(&self) -> (u64, u64) {
        let bounds = self.inner.kernel_clock_bounds();
        let ticks = dead_time_ticks(self.dead_time);
        (
            bounds.minimum_duration_ns(ticks),
            bounds.maximum_duration_ns(ticks),
        )
    }
}
fn set_enabled(channel: Channel, main: bool, complementary: bool) {
    pac::ATIM.ccer().modify(|v| match channel {
        Channel::Ch1 => {
            v.set_cc1e(main);
            v.set_cc1ne(complementary);
        }
        Channel::Ch2 => {
            v.set_cc2e(main);
            v.set_cc2ne(complementary);
        }
        Channel::Ch3 => {
            v.set_cc3e(main);
            v.set_cc3ne(complementary);
        }
        Channel::Ch4 => {
            v.set_cc4e(main);
            v.set_cc4ne(complementary);
        }
    });
}
fn set_mode(channel: Channel, mode: u8) {
    match channel {
        Channel::Ch1 => pac::ATIM.ccmr1cmp().modify(|v| v.set_oc1m(mode)),
        Channel::Ch2 => pac::ATIM.ccmr1cmp().modify(|v| v.set_oc2m(mode)),
        Channel::Ch3 => pac::ATIM.ccmr2cmp().modify(|v| v.set_oc3m(mode)),
        Channel::Ch4 => pac::ATIM.ccmr2cmp().modify(|v| v.set_oc4m(mode)),
    }
}
impl<T: ComplementaryInstance> Drop for ComplementaryPwm<'_, T> {
    fn drop(&mut self) {
        self.set_master_output_enable(false);
        pac::ATIM.ccer().write(|_| {});
        self.inner.stop();
        for pin in self.pins.iter_mut().flatten() {
            pin.set_as_disconnected();
        }
        if let Some(pin) = self.break_pin.as_mut() {
            pin.set_as_disconnected();
        }
        // Timer::drop disables requests and the central RCC_INFO gate afterward.
    }
}
