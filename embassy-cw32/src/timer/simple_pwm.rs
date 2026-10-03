//! Single-ended, active-high PWM with individually owned optional pins.
//!
//! This is not a complementary or motor-safety driver. Disabled channels are
//! disconnected floating inputs. External pulls establish any required level.
//! Enabling a channel and starting the counter are separate explicit actions.
//! No DMA, interrupt, break input, dead time or ADC trigger is enabled.

use super::{
    low_level::{self, Config, Timer},
    Ch1, Ch2, Ch3, Ch4, Channel, Error, Frequency, PwmInstance, TimerChannel, TimerPin,
};
use crate::{gpio::AnyPin, rcc, Peri};
use core::marker::PhantomData;

struct PinState<'d> {
    pin: Peri<'d, AnyPin>,
    af: u8,
}
impl PinState<'_> {
    fn connect(&self) {
        self.pin.configure_alternate(self.af, true);
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

/// A pin with a metadata-proved timer/channel route. Creation disconnects it;
/// it is not muxed to the peripheral until `SimplePwm::enable` is called.
pub struct PwmPin<'d, T: PwmInstance, C: TimerChannel> {
    pin: PinState<'d>,
    _phantom: PhantomData<(T, C)>,
}
impl<'d, T: PwmInstance, C: TimerChannel> PwmPin<'d, T, C> {
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

/// Owns the timer and zero to four independently optional output pins.
///
/// L012 ATIM/GTIM offer four audited main channels; F030 ATIM offers only A1–A3,
/// while F030 GTIM offers four. Missing `TimerPin` implementations make unsupported
/// routes unconstructible. An absent pin returns `ChannelUnavailable` before writes.
/// Drop disables outputs, stops the timer, and disconnects the pins.
pub struct SimplePwm<'d, T: PwmInstance> {
    timer: Timer<'d, T>,
    pins: [Option<PinState<'d>>; 4],
    duty: [u32; 4],
    enabled: u8,
}
impl<'d, T: PwmInstance> SimplePwm<'d, T> {
    /// Construct with the highest-resolution divider that produces a rate no
    /// higher than `frequency_hz`. Counter and all pins remain disabled.
    pub fn new(
        timer: Peri<'d, T>,
        ch1: Option<PwmPin<'d, T, Ch1>>,
        ch2: Option<PwmPin<'d, T, Ch2>>,
        ch3: Option<PwmPin<'d, T, Ch3>>,
        ch4: Option<PwmPin<'d, T, Ch4>>,
        frequency_hz: u32,
    ) -> Result<Self, Error> {
        let config = low_level::solve(rcc::clocks().pclk, frequency_hz, T::regs().prescalers())?;
        Self::new_with_config(timer, ch1, ch2, ch3, ch4, config)
    }
    /// Construct with exact divider/count settings, including periods below 1 Hz.
    pub fn new_with_config(
        timer: Peri<'d, T>,
        ch1: Option<PwmPin<'d, T, Ch1>>,
        ch2: Option<PwmPin<'d, T, Ch2>>,
        ch3: Option<PwmPin<'d, T, Ch3>>,
        ch4: Option<PwmPin<'d, T, Ch4>>,
        config: Config,
    ) -> Result<Self, Error> {
        config.validate(T::regs().prescalers())?;
        let mut timer = Timer::new(timer);
        T::regs().configure_pwm();
        timer.write_config(config);
        // Timer initialization set every compare to zero, with outputs disabled.
        T::regs().load();
        Ok(Self {
            timer,
            pins: [
                ch1.map(|p| p.pin),
                ch2.map(|p| p.pin),
                ch3.map(|p| p.pin),
                ch4.map(|p| p.pin),
            ],
            duty: [0; 4],
            enabled: 0,
        })
    }
    /// Starts the counter only. Enable each desired output separately.
    pub fn start(&mut self) {
        self.timer.start();
    }
    /// Stops the counter only; enabled outputs retain their current compare level.
    /// Use `disable`/`disable_all` when the pins must be disconnected.
    pub fn stop(&mut self) {
        self.timer.stop();
    }
    pub fn is_running(&self) -> bool {
        self.timer.is_running()
    }
    pub fn frequency(&self) -> Frequency {
        self.timer.frequency()
    }
    pub fn config(&self) -> Config {
        self.timer.config()
    }
    /// Native tick range is 0..=period_ticks, including 65536 at maximum period.
    pub fn max_duty_ticks(&self) -> u32 {
        self.timer.period_ticks()
    }
    fn check(&self, channel: Channel) -> Result<usize, Error> {
        let n = channel.index();
        if self.pins[n].is_none() {
            return Err(Error::ChannelUnavailable);
        }
        Ok(n)
    }
    pub fn is_enabled(&self, channel: Channel) -> bool {
        self.enabled & (1 << channel.index()) != 0
    }
    /// Connect one pin without starting the counter. When stopped, pending
    /// compare preloads are loaded before the pin is muxed. When already running,
    /// a preload-capable timer can output the previous active duty until the next
    /// update loads the latest requested value; enabling does not reset its phase.
    pub fn enable(&mut self, channel: Channel) -> Result<(), Error> {
        let n = self.check(channel)?;
        if self.is_enabled(channel) {
            return Ok(());
        }
        if !self.is_running() {
            T::regs().load();
        }
        // This channel is hardware-disabled/forced-inactive before muxing.
        self.pins[n].as_ref().unwrap().connect();
        T::regs().output(n, true, self.duty[n]);
        self.enabled |= 1 << n;
        T::regs().master_output(true);
        Ok(())
    }
    /// Disconnect one pin to floating input. Does not stop other channels or the counter.
    pub fn disable(&mut self, channel: Channel) -> Result<(), Error> {
        let n = self.check(channel)?;
        T::regs().output(n, false, self.duty[n]);
        self.pins[n].as_ref().unwrap().disconnect();
        self.enabled &= !(1 << n);
        if self.enabled == 0 {
            T::regs().master_output(false);
        }
        Ok(())
    }
    /// Disconnect every owned channel without stopping the counter.
    pub fn disable_all(&mut self) {
        self.gate_outputs();
        self.enabled = 0;
    }
    /// Last requested duty, which may still be waiting for a hardware update.
    pub fn duty_ticks(&self, channel: Channel) -> Result<u32, Error> {
        Ok(self.duty[self.check(channel)?])
    }
    /// Set a native compare count. 0 is always low; period_ticks is always high.
    ///
    /// Ordinary L012 and F030 ATIM writes use the per-channel preload and take
    /// effect at the next update, independently of other channels. F030 GTIM
    /// has no compare preload: its write takes effect immediately and can
    /// shorten or extend the current pulse. Neither case is an atomic multi-
    /// channel update. With the counter stopped, preloads are loaded immediately.
    ///
    /// At period=65536, exact 100% needs forced-active mode because CCR is only
    /// 16 bits. Transitions into/out of that endpoint briefly disconnect all
    /// enabled pins, stop/load/reset phase, then restore their previous state.
    pub fn set_duty_ticks(&mut self, channel: Channel, duty: u32) -> Result<(), Error> {
        let n = self.check(channel)?;
        if duty > self.max_duty_ticks() {
            return Err(Error::DutyOutOfRange);
        }
        let mode_change = (self.duty[n] == 65536) != (duty == 65536);
        self.duty[n] = duty;
        if mode_change {
            self.reconfigure(self.config());
        } else {
            T::regs().write_compare(n, duty);
            if !self.is_running() {
                T::regs().load();
            }
        }
        Ok(())
    }
    /// Set frequency, preserving each requested duty fraction (rounded down).
    /// All outputs briefly disconnect; the counter phase resets. Previous
    /// counter/channel enables are restored. No software ADC trigger is emitted.
    pub fn set_frequency(&mut self, hz: u32) -> Result<Frequency, Error> {
        self.set_config(self.timer.frequency_config(hz)?)
    }
    /// Exact timing change with the same disconnected, phase-resetting semantics.
    /// Validate all limits before touching timer or pin registers.
    pub fn set_config(&mut self, config: Config) -> Result<Frequency, Error> {
        config.validate(T::regs().prescalers())?;
        let old = self.max_duty_ticks();
        for duty in &mut self.duty {
            *duty = (u64::from(*duty) * u64::from(config.period_ticks) / u64::from(old)) as u32;
        }
        self.reconfigure(config);
        Ok(self.frequency())
    }
    fn gate_outputs(&mut self) {
        T::regs().master_output(false);
        for n in 0..4 {
            if let Some(pin) = &self.pins[n] {
                T::regs().output(n, false, self.duty[n]);
                pin.disconnect();
            }
        }
    }
    fn reconfigure(&mut self, config: Config) {
        let running = self.is_running();
        self.gate_outputs();
        self.timer.stop();
        self.timer.write_config(config);
        for n in 0..4 {
            if self.pins[n].is_some() {
                T::regs().write_compare(n, self.duty[n]);
            }
        }
        T::regs().load();
        // Preloads are active and CEN is still stopped while reconnecting pins.
        for n in 0..4 {
            if self.enabled & (1 << n) != 0 {
                self.pins[n].as_ref().unwrap().connect();
                T::regs().output(n, true, self.duty[n]);
            }
        }
        T::regs().master_output(self.enabled != 0);
        if running {
            self.timer.start();
        }
    }
    /// Exclusive borrowed channel handle. The whole driver's mutable borrow
    /// prevents frequency changes, aliasing channel handles, or early owner drop.
    pub fn channel(&mut self, channel: Channel) -> Result<SimplePwmChannel<'_, 'd, T>, Error> {
        self.check(channel)?;
        Ok(SimplePwmChannel { pwm: self, channel })
    }
    pub fn ch1(&mut self) -> Result<SimplePwmChannel<'_, 'd, T>, Error> {
        self.channel(Channel::Ch1)
    }
    pub fn ch2(&mut self) -> Result<SimplePwmChannel<'_, 'd, T>, Error> {
        self.channel(Channel::Ch2)
    }
    pub fn ch3(&mut self) -> Result<SimplePwmChannel<'_, 'd, T>, Error> {
        self.channel(Channel::Ch3)
    }
    pub fn ch4(&mut self) -> Result<SimplePwmChannel<'_, 'd, T>, Error> {
        self.channel(Channel::Ch4)
    }
}
impl<T: PwmInstance> Drop for SimplePwm<'_, T> {
    fn drop(&mut self) {
        self.gate_outputs();
        self.timer.stop();
    }
}

/// Borrowed channel with a normalized u16 embedded-hal duty scale.
/// 65535 always means exactly 100%, even when the native period has 65536 ticks.
/// For exact native counts, use `set_duty_ticks`/`max_duty_ticks` instead.
pub struct SimplePwmChannel<'a, 'd, T: PwmInstance> {
    pwm: &'a mut SimplePwm<'d, T>,
    channel: Channel,
}
impl<T: PwmInstance> SimplePwmChannel<'_, '_, T> {
    pub fn enable(&mut self) {
        self.pwm.enable(self.channel).unwrap();
    }
    pub fn disable(&mut self) {
        self.pwm.disable(self.channel).unwrap();
    }
    pub fn is_enabled(&self) -> bool {
        self.pwm.is_enabled(self.channel)
    }
    pub fn frequency(&self) -> Frequency {
        self.pwm.frequency()
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
    /// Normalized duty; lower hardware resolution is rounded downward.
    pub fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Error> {
        let ticks =
            (u64::from(duty) * u64::from(self.max_duty_ticks()) / u64::from(u16::MAX)) as u32;
        self.set_duty_ticks(ticks)
    }
}
impl<T: PwmInstance> embedded_hal::pwm::ErrorType for SimplePwmChannel<'_, '_, T> {
    type Error = Error;
}
impl<T: PwmInstance> embedded_hal::pwm::SetDutyCycle for SimplePwmChannel<'_, '_, T> {
    fn max_duty_cycle(&self) -> u16 {
        self.max_duty_cycle()
    }
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty_cycle(duty)
    }
}
