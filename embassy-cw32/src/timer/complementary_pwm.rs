//! Three-channel, edge-aligned complementary PWM with explicit output arming.
//!
//! Each main and complementary pin is independently optional. Construction
//! leaves the counter stopped and all output pins disconnected. Select channels
//! with [`ComplementaryPwm::enable`], start the counter, then explicitly call
//! [`ComplementaryPwm::enable_outputs`]. Automatic output enable stays disabled.
//! An optional external brake forces both outputs low; no pin means no external
//! brake protection. External interlocks/pulls must establish board-level safety.
//!
//! Compare writes are buffered per channel on both supported ATIMs. Separate
//! writes are not an atomic multi-channel update, including inside a CPU critical
//! section. This driver does not alter the separate three-phase motor protocol.

use super::{
    low_level::{self, Timer},
    Ch1, Ch2, Ch3, Frequency, PwmInstance, TimerChannel, TimerPin,
};
use crate::{
    gpio::{AnyPin, Pin},
    pac, peripherals, Peri,
};
use core::marker::PhantomData;

#[cfg_attr(atim_l012, path = "complementary_pwm/l012.rs")]
#[cfg_attr(atim_f030, path = "complementary_pwm/f030.rs")]
mod backend;

pub(crate) mod sealed {
    pub trait Instance {
        fn registers() -> crate::pac::atim::Atim;
    }
    pub trait Pin<T, C> {}
    pub trait BrakePin<T> {}
    pub trait Channel {}
}
/// A metadata-generated timer with an audited complementary-output backend.
#[allow(private_bounds)]
pub trait Instance: PwmInstance + sealed::Instance {}
/// Only the three channels with audited external complementary routes.
#[allow(private_bounds)]
pub trait ComplementaryChannel: TimerChannel + sealed::Channel {}
macro_rules! channels { ($($channel:ident),*) => {$ (
    impl sealed::Channel for $channel {}
    impl ComplementaryChannel for $channel {}
)*}; }
channels!(Ch1, Ch2, Ch3);
/// Audited CHxN (L012) or CHxB (F030) AF route. Check package bonding separately.
#[allow(private_bounds)]
pub trait ComplementaryPin<T: Instance, C: ComplementaryChannel>: Pin + sealed::Pin<T, C> {
    const AF: u8;
}
/// Audited external BK route for this timer. Check package bonding separately.
#[allow(private_bounds)]
pub trait TimerBrakePin<T: Instance>: Pin + sealed::BrakePin<T> {
    const AF: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_complementary_pwm.rs"));

/// A physical complementary pair. There is deliberately no fourth channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
}
impl Channel {
    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Timing(super::Error),
    ChannelUnavailable,
    NoChannelEnabled,
    DutyOutOfRange,
    DeadTimeTooLarge,
    /// F030 has one common dead-time value for both edges.
    AsymmetricDeadTimeUnsupported,
    InvalidFilter,
    /// Disarm outputs before changing timing/dead time, selecting another
    /// channel, or crossing the 65536-tick forced-active endpoint.
    OutputsEnabled,
    FaultActive,
}
impl From<super::Error> for Error {
    fn from(value: super::Error) -> Self {
        Self::Timing(value)
    }
}
impl embedded_hal::pwm::Error for Error {
    fn kind(&self) -> embedded_hal::pwm::ErrorKind {
        embedded_hal::pwm::ErrorKind::Other
    }
}

/// Minimum insertion times, rounded UP to the hardware encoding.
///
/// L012 uses PCLK ticks (CKD=/1), independently of the counter prescaler, and
/// supports separate rising/falling values 0..=1008. F030 uses prescaled TTCLK
/// ticks, requires equal values, and supports 0 (DTEN off) or 2..=1010 ticks.
/// A positive F030 request of 1 rounds to 2. These units are intentionally not
/// interchangeable; use [`ComplementaryPwm::dead_time_clock_hz`] to interpret them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeadTime {
    pub rising_ticks: u16,
    pub falling_ticks: u16,
}
impl DeadTime {
    pub const fn symmetric(ticks: u16) -> Self {
        Self {
            rising_ticks: ticks,
            falling_ticks: ticks,
        }
    }
}
/// Counter timing and requested minimum dead time. Only edge-aligned upcounting
/// is exposed. ARR is `timing.period_ticks - 1`; the full duty endpoint is exact.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub timing: low_level::Config,
    pub dead_time: DeadTime,
}
/// External brake polarity/filter. AOE remains off for every configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreakConfig {
    pub active_high: bool,
    /// L012: BKF 0..=15. F030: FLTBK 0 or 4..=7. Zero is unfiltered.
    /// Filtering requires a live clock and is not clock-loss protection.
    pub filter: u8,
}
impl Default for BreakConfig {
    fn default() -> Self {
        Self {
            active_high: true,
            filter: 0,
        }
    }
}

struct PinState<'d> {
    pin: Peri<'d, AnyPin>,
    af: u8,
}
impl PinState<'_> {
    fn connect(&self, output: bool) {
        self.pin.configure_alternate(self.af, output);
    }
    fn disconnect(&self) {
        self.pin.disconnect();
    }
}
impl Drop for PinState<'_> {
    fn drop(&mut self) {
        self.disconnect();
    }
}
/// Typed main-output pin. Creation disconnects it until the driver is armed.
pub struct PwmPin<'d, T: Instance, C: ComplementaryChannel> {
    pin: PinState<'d>,
    _phantom: PhantomData<(T, C)>,
}
impl<'d, T: Instance, C: ComplementaryChannel> PwmPin<'d, T, C> {
    pub fn new<P: TimerPin<T, C>>(pin: Peri<'d, P>) -> Self {
        let pin = PinState {
            pin: pin.into(),
            af: P::AF,
        };
        pin.disconnect();
        Self {
            pin,
            _phantom: PhantomData,
        }
    }
}
/// Typed complementary-output pin, preserving the same pair/dead-time semantics
/// even when the matching main pin is absent.
pub struct ComplementaryPwmPin<'d, T: Instance, C: ComplementaryChannel> {
    pin: PinState<'d>,
    _phantom: PhantomData<(T, C)>,
}
impl<'d, T: Instance, C: ComplementaryChannel> ComplementaryPwmPin<'d, T, C> {
    pub fn new<P: ComplementaryPin<T, C>>(pin: Peri<'d, P>) -> Self {
        let pin = PinState {
            pin: pin.into(),
            af: P::AF,
        };
        pin.disconnect();
        Self {
            pin,
            _phantom: PhantomData,
        }
    }
}
/// Owns an external BK pin and its validated configuration. The pin is muxed as
/// an input only after the timer is initialized with disabled outputs.
pub struct BrakePin<'d, T: Instance> {
    pin: PinState<'d>,
    config: BreakConfig,
    _phantom: PhantomData<T>,
}
impl<'d, T: Instance> BrakePin<'d, T> {
    pub fn new<P: TimerBrakePin<T>>(pin: Peri<'d, P>, config: BreakConfig) -> Result<Self, Error> {
        backend::validate_break(config)?;
        let pin = PinState {
            pin: pin.into(),
            af: P::AF,
        };
        pin.disconnect();
        Ok(Self {
            pin,
            config,
            _phantom: PhantomData,
        })
    }
}

trait ComplementaryRegisters {
    fn configure_complementary(self, dead_time: DeadTime, brake: Option<BreakConfig>);
    fn set_dead_time(self, dead_time: DeadTime);
    fn set_pair_enabled(self, channel: usize, enabled: bool);
    fn set_duty_mode(self, channel: usize, duty: u32);
    fn master(self, enabled: bool);
    fn master_enabled(self) -> bool;
    fn fault_pending(self) -> bool;
    fn clear_fault(self);
}

/// Owns one timer, up to six independently optional output pins and optional BK.
///
/// Missing routes fail at compile time; a channel with neither pin returns
/// `ChannelUnavailable`. The single owner prevents overlapping timer drivers,
/// frequency changes through borrowed handles, or premature clock release.
/// Drop removes output drive, stops the counter, and disconnects all owned pins
/// before releasing the timer clock. Forgotten owners retain their resources.
pub struct ComplementaryPwm<'d, T: Instance> {
    timer: Timer<'d, T>,
    pins: [[Option<PinState<'d>>; 2]; 3],
    brake: Option<BrakePin<'d, T>>,
    duty: [u32; 3],
    selected: u8,
    dead_time: DeadTime,
}
impl<'d, T: Instance> ComplementaryPwm<'d, T> {
    /// Construct stopped/disarmed. Every pin and the external brake are optional;
    /// passing no brake explicitly leaves external fault protection disabled.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        timer: Peri<'d, T>,
        ch1: Option<PwmPin<'d, T, Ch1>>,
        ch1n: Option<ComplementaryPwmPin<'d, T, Ch1>>,
        ch2: Option<PwmPin<'d, T, Ch2>>,
        ch2n: Option<ComplementaryPwmPin<'d, T, Ch2>>,
        ch3: Option<PwmPin<'d, T, Ch3>>,
        ch3n: Option<ComplementaryPwmPin<'d, T, Ch3>>,
        brake: Option<BrakePin<'d, T>>,
        frequency_hz: u32,
        dead_time: DeadTime,
    ) -> Result<Self, Error> {
        let timing = low_level::solve(T::frequency(), frequency_hz, T::regs().prescalers())?;
        Self::new_with_config(
            timer,
            ch1,
            ch1n,
            ch2,
            ch2n,
            ch3,
            ch3n,
            brake,
            Config { timing, dead_time },
        )
    }
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_config(
        timer: Peri<'d, T>,
        ch1: Option<PwmPin<'d, T, Ch1>>,
        ch1n: Option<ComplementaryPwmPin<'d, T, Ch1>>,
        ch2: Option<PwmPin<'d, T, Ch2>>,
        ch2n: Option<ComplementaryPwmPin<'d, T, Ch2>>,
        ch3: Option<PwmPin<'d, T, Ch3>>,
        ch3n: Option<ComplementaryPwmPin<'d, T, Ch3>>,
        brake: Option<BrakePin<'d, T>>,
        config: Config,
    ) -> Result<Self, Error> {
        config.timing.validate(T::regs().prescalers())?;
        let dead_time = backend::validate_dead_time(config.dead_time)?;
        let mut timer = Timer::new(timer);
        timer.write_config(config.timing);
        if let Some(brake) = &brake {
            brake.pin.connect(false);
        }
        T::registers().configure_complementary(dead_time, brake.as_ref().map(|b| b.config));
        T::regs().load();
        Ok(Self {
            timer,
            pins: [
                [ch1.map(|p| p.pin), ch1n.map(|p| p.pin)],
                [ch2.map(|p| p.pin), ch2n.map(|p| p.pin)],
                [ch3.map(|p| p.pin), ch3n.map(|p| p.pin)],
            ],
            brake,
            duty: [0; 3],
            selected: 0,
            dead_time,
        })
    }
    fn check(&self, channel: Channel) -> Result<usize, Error> {
        let n = channel.index();
        if self.pins[n].iter().all(Option::is_none) {
            Err(Error::ChannelUnavailable)
        } else {
            Ok(n)
        }
    }
    pub fn frequency(&self) -> Frequency {
        self.timer.frequency()
    }
    /// Returns the actual rounded dead time, not an unachievable request.
    pub fn config(&self) -> Config {
        Config {
            timing: self.timer.config(),
            dead_time: self.dead_time,
        }
    }
    pub fn dead_time(&self) -> DeadTime {
        self.dead_time
    }
    /// Dead-time clock in whole hertz, rounded down. Use the ratio for an exact
    /// duration when PCLK is not divisible by the F030 counter divider.
    pub fn dead_time_clock_hz(&self) -> u32 {
        let (numerator, denominator) = self.dead_time_clock_ratio();
        numerator / denominator
    }
    /// Exact dead-time tick frequency as numerator / denominator hertz.
    pub fn dead_time_clock_ratio(&self) -> (u32, u32) {
        backend::dead_time_clock_ratio(self.frequency())
    }
    pub fn has_external_brake(&self) -> bool {
        self.brake.is_some()
    }
    pub fn max_duty_ticks(&self) -> u32 {
        self.timer.period_ticks()
    }
    /// Starts only the counter. Never writes MOE or acknowledges a fault.
    pub fn start(&mut self) {
        self.timer.start();
    }
    /// Stops only the counter; armed outputs may retain a static level.
    /// Call `disable_outputs` first to remove drive from all owned pins.
    pub fn stop(&mut self) {
        self.timer.stop();
    }
    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
    pub fn outputs_enabled(&self) -> bool {
        T::registers().master_enabled()
    }
    pub fn fault_pending(&self) -> bool {
        T::registers().fault_pending()
    }
    /// Select a channel for the next explicit `enable_outputs`. Does not connect
    /// pins or arm MOE. Adding a channel while MOE is enabled is rejected.
    pub fn enable(&mut self, channel: Channel) -> Result<(), Error> {
        let n = self.check(channel)?;
        if self.outputs_enabled() {
            return Err(Error::OutputsEnabled);
        }
        self.selected |= 1 << n;
        Ok(())
    }
    /// Reports channel selection, independently of the master gate or a break.
    pub fn is_enabled(&self, channel: Channel) -> bool {
        self.selected & (1 << channel.index()) != 0
    }
    /// Deselect/disconnect this pair without stopping the counter or rearming
    /// other channels. F030 has no per-pair enable; GPIO disconnection gates it.
    pub fn disable(&mut self, channel: Channel) -> Result<(), Error> {
        let n = self.check(channel)?;
        for pin in self.pins[n].iter().flatten() {
            pin.disconnect();
        }
        T::registers().set_pair_enabled(n, false);
        self.selected &= !(1 << n);
        if self.selected == 0 {
            T::registers().master(false);
        }
        Ok(())
    }
    /// Explicitly arm the selected outputs. Caller establishes board safety.
    /// Arming checks the fault latch before and after MOE and after connecting
    /// pins. Hardware may break asynchronously after this function returns.
    /// If running, pending compare preloads take effect at the next update.
    pub fn enable_outputs(&mut self) -> Result<(), Error> {
        if self.selected == 0 {
            return Err(Error::NoChannelEnabled);
        }
        self.disable_outputs();
        if !self.is_running() {
            T::regs().load();
        }
        arm_sequence(&mut HardwareArm { pwm: self })
    }
    /// Remove output drive while retaining channel selection and counter state.
    /// Pins become floating inputs. External pulls/interlocks define safe levels.
    pub fn disable_outputs(&mut self) {
        T::registers().master(false);
        for (n, pins) in self.pins.iter().enumerate() {
            for pin in pins.iter().flatten() {
                pin.disconnect();
            }
            T::registers().set_pair_enabled(n, false);
        }
    }
    /// Disable outputs then acknowledge only hardware break flags. A persistent
    /// source returns `FaultActive`. Never arms outputs or clears unrelated flags.
    pub fn acknowledge_fault(&mut self) -> Result<(), Error> {
        self.disable_outputs();
        T::registers().clear_fault();
        if self.fault_pending() {
            Err(Error::FaultActive)
        } else {
            Ok(())
        }
    }
    pub fn duty_ticks(&self, channel: Channel) -> Result<u32, Error> {
        Ok(self.duty[self.check(channel)?])
    }
    /// Set one buffered reference compare. Zero means main low/complement high;
    /// max means main high/complement low, after dead-time settling. It does not
    /// mean both pins off. Use `disable_outputs` to remove their drive.
    ///
    /// Running writes latch on the next update independently; several calls may
    /// straddle an update. No atomic multi-channel commit is promised on either
    /// chip. With the counter stopped, load the compare immediately. Crossing
    /// the 65536-tick endpoint changes forced mode and requires disarmed outputs.
    pub fn set_duty_ticks(&mut self, channel: Channel, duty: u32) -> Result<(), Error> {
        let n = self.check(channel)?;
        if duty > self.max_duty_ticks() {
            return Err(Error::DutyOutOfRange);
        }
        let mode_change = (self.duty[n] == 65536) != (duty == 65536);
        if mode_change && self.outputs_enabled() {
            return Err(Error::OutputsEnabled);
        }
        T::regs().write_compare(n, duty);
        if mode_change {
            T::registers().set_duty_mode(n, duty);
        }
        if !self.is_running() {
            T::regs().load();
        }
        self.duty[n] = duty;
        Ok(())
    }
    /// Change frequency only while disarmed, preserving requested duty fractions
    /// (rounded down) and counter running state. Resets phase; never rearms.
    pub fn set_frequency(&mut self, hz: u32) -> Result<Frequency, Error> {
        let timing = self.timer.frequency_config(hz)?;
        self.set_config(Config {
            timing,
            dead_time: self.dead_time,
        })
    }
    /// Change timing/dead time only while disarmed. Validation precedes register
    /// writes. Preserves counter running state, resets phase, leaves pins off.
    /// Changing the F030 divider also changes dead-time tick duration.
    pub fn set_config(&mut self, config: Config) -> Result<Frequency, Error> {
        config.timing.validate(T::regs().prescalers())?;
        let dead_time = backend::validate_dead_time(config.dead_time)?;
        if self.outputs_enabled() {
            return Err(Error::OutputsEnabled);
        }
        let running = self.is_running();
        let old = self.max_duty_ticks();
        self.disable_outputs();
        self.timer.stop();
        self.timer.write_config(config.timing);
        T::registers().set_dead_time(dead_time);
        self.dead_time = dead_time;
        for n in 0..3 {
            self.duty[n] = (u64::from(self.duty[n]) * u64::from(config.timing.period_ticks)
                / u64::from(old)) as u32;
            T::regs().write_compare(n, self.duty[n]);
            T::registers().set_duty_mode(n, self.duty[n]);
        }
        T::regs().load();
        if running {
            self.timer.start();
        }
        Ok(self.frequency())
    }
    /// A mutable borrowed duty handle; owner/frequency operations cannot overlap.
    pub fn channel(
        &mut self,
        channel: Channel,
    ) -> Result<ComplementaryPwmChannel<'_, 'd, T>, Error> {
        self.check(channel)?;
        Ok(ComplementaryPwmChannel { pwm: self, channel })
    }
}
impl<T: Instance> Drop for ComplementaryPwm<'_, T> {
    fn drop(&mut self) {
        self.disable_outputs();
        self.timer.stop();
        if let Some(brake) = &self.brake {
            brake.pin.disconnect();
        }
    }
}

// The only operation which can write MOE=1. Its single enable write happens
// while all output pins and L012 pair gates are disconnected. No later write can
// replay MOE=1 over an asynchronous break. The pure sequencing interface also
// permits external fault-injection probes without adding production tests.
trait ArmIo {
    fn fault(&mut self) -> bool;
    fn master(&mut self, enabled: bool);
    fn enabled(&mut self) -> bool;
    fn connect(&mut self);
    fn disconnect(&mut self);
}
fn arm_sequence(io: &mut impl ArmIo) -> Result<(), Error> {
    if io.fault() {
        return Err(Error::FaultActive);
    }
    io.master(true);
    if io.fault() || !io.enabled() {
        io.master(false);
        return Err(Error::FaultActive);
    }
    io.connect();
    if io.fault() || !io.enabled() {
        io.master(false);
        io.disconnect();
        return Err(Error::FaultActive);
    }
    Ok(())
}
struct HardwareArm<'a, 'd, T: Instance> {
    pwm: &'a mut ComplementaryPwm<'d, T>,
}
impl<T: Instance> ArmIo for HardwareArm<'_, '_, T> {
    fn fault(&mut self) -> bool {
        self.pwm.fault_pending()
    }
    fn master(&mut self, enabled: bool) {
        T::registers().master(enabled);
    }
    fn enabled(&mut self) -> bool {
        self.pwm.outputs_enabled()
    }
    fn connect(&mut self) {
        for n in 0..3 {
            if self.pwm.selected & (1 << n) != 0 {
                T::registers().set_pair_enabled(n, true);
                for pin in self.pwm.pins[n].iter().flatten() {
                    pin.connect(true);
                }
            }
        }
    }
    fn disconnect(&mut self) {
        self.pwm.disable_outputs();
    }
}
/// Exclusive borrowed pair handle with a normalized embedded-hal duty scale.
/// `u16::MAX` is exact 100% main duty; the complementary output is inverted.
pub struct ComplementaryPwmChannel<'a, 'd, T: Instance> {
    pwm: &'a mut ComplementaryPwm<'d, T>,
    channel: Channel,
}
impl<T: Instance> ComplementaryPwmChannel<'_, '_, T> {
    pub fn enable(&mut self) -> Result<(), Error> {
        self.pwm.enable(self.channel)
    }
    pub fn disable(&mut self) {
        self.pwm.disable(self.channel).unwrap();
    }
    pub fn is_enabled(&self) -> bool {
        self.pwm.is_enabled(self.channel)
    }
    pub fn max_duty_ticks(&self) -> u32 {
        self.pwm.max_duty_ticks()
    }
    pub fn duty_ticks(&self) -> u32 {
        self.pwm.duty[self.channel.index()]
    }
    pub fn set_duty_ticks(&mut self, duty: u32) -> Result<(), Error> {
        self.pwm.set_duty_ticks(self.channel, duty)
    }
    pub fn max_duty_cycle(&self) -> u16 {
        u16::MAX
    }
    pub fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Error> {
        let ticks =
            (u64::from(duty) * u64::from(self.max_duty_ticks()) / u64::from(u16::MAX)) as u32;
        self.set_duty_ticks(ticks)
    }
}
impl<T: Instance> embedded_hal::pwm::ErrorType for ComplementaryPwmChannel<'_, '_, T> {
    type Error = Error;
}
impl<T: Instance> embedded_hal::pwm::SetDutyCycle for ComplementaryPwmChannel<'_, '_, T> {
    fn max_duty_cycle(&self) -> u16 {
        self.max_duty_cycle()
    }
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty_cycle(duty)
    }
}
