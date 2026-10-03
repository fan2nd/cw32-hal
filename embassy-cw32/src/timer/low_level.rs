//! Minimal owned timer control. No raw PAC access or fabricated peripheral clocks.
use super::{CoreInstance, Error, Frequency, Prescalers};
use crate::{rcc::ClockGuard, Peri};

/// Edge-aligned upcounter timing. Values describe physical counts, not encodings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// PCLK divider. Legal values depend on the timer IP.
    pub prescaler_divisor: u32,
    /// Counter ticks per period, 2..=65536; ARR is this value minus one.
    pub period_ticks: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            prescaler_divisor: 1,
            period_ticks: 65536,
        }
    }
}
impl Config {
    pub(crate) fn validate(self, prescalers: Prescalers) -> Result<(), Error> {
        if !prescalers.validate(self.prescaler_divisor) {
            return Err(Error::InvalidPrescaler);
        }
        if !(2..=65536).contains(&self.period_ticks) {
            return Err(Error::InvalidPeriod);
        }
        Ok(())
    }
}

/// Owns a single timer. Construction and Drop leave its counter stopped.
///
/// A metadata-derived clock guard keeps the timer's clock alive. Shared clock
/// gates are reset only when the RCC resource can do so without affecting a
/// live peer, and are released only after the final owner drops.
/// GTIM1 is not safely obtainable when the Embassy time driver reserves it.
pub struct Timer<'d, T: CoreInstance> {
    _instance: Peri<'d, T>,
    pub(super) clock: ClockGuard,
    clock_hz: u32,
    config: Config,
}
impl<'d, T: CoreInstance> Timer<'d, T> {
    /// Use the validated RCC PCLK. This never starts the counter or enables pins.
    pub fn new(instance: Peri<'d, T>) -> Self {
        let clock_hz = T::frequency();
        let clock = T::acquire();
        T::regs().initialize();
        let config = Config::default();
        T::regs().write_timing(config);
        T::regs().load();
        Self {
            _instance: instance,
            clock,
            clock_hz,
            config,
        }
    }
    /// Explicitly start counting. No output, interrupt, DMA or ADC trigger is enabled.
    pub fn start(&mut self) {
        T::regs().set_running(true);
    }
    pub fn stop(&mut self) {
        T::regs().set_running(false);
    }
    pub fn is_running(&self) -> bool {
        T::regs().is_running()
    }
    pub fn counter(&self) -> u16 {
        T::regs().counter()
    }
    /// Change the current count, without issuing an update or trigger.
    pub fn set_counter(&mut self, value: u16) -> Result<(), Error> {
        if u32::from(value) >= self.config.period_ticks {
            return Err(Error::CounterOutOfRange);
        }
        T::regs().set_counter(value);
        Ok(())
    }
    pub fn config(&self) -> Config {
        self.config
    }
    pub fn frequency(&self) -> Frequency {
        Frequency {
            clock_hz: self.clock_hz,
            config: self.config,
        }
    }
    pub fn period_ticks(&self) -> u32 {
        self.config.period_ticks
    }
    pub fn prescaler_divisor(&self) -> u32 {
        self.config.prescaler_divisor
    }
    /// Solve without modifying hardware. Select the smallest legal divider and
    /// round the period upward, so the actual rate never exceeds the request.
    /// This prioritizes duty resolution, not the closest possible rational rate.
    pub fn frequency_config(&self, hz: u32) -> Result<Config, Error> {
        solve(self.clock_hz, hz, T::regs().prescalers())
    }
    /// Change timing, resetting phase to zero and preserving the running state.
    /// Does not generate an ADC trigger. Invalid values perform no register writes.
    pub fn set_config(&mut self, config: Config) -> Result<Frequency, Error> {
        config.validate(T::regs().prescalers())?;
        let running = self.is_running();
        self.stop();
        self.write_config(config);
        T::regs().load();
        if running {
            self.start();
        }
        Ok(self.frequency())
    }
    pub fn set_frequency(&mut self, hz: u32) -> Result<Frequency, Error> {
        self.set_config(self.frequency_config(hz)?)
    }
    pub fn set_period_ticks(&mut self, period_ticks: u32) -> Result<Frequency, Error> {
        self.set_config(Config {
            period_ticks,
            ..self.config
        })
    }
    pub fn set_prescaler_divisor(&mut self, prescaler_divisor: u32) -> Result<Frequency, Error> {
        self.set_config(Config {
            prescaler_divisor,
            ..self.config
        })
    }
    pub(crate) fn write_config(&mut self, config: Config) {
        T::regs().write_timing(config);
        self.config = config;
    }
}
impl<T: CoreInstance> Drop for Timer<'_, T> {
    fn drop(&mut self) {
        self.stop();
    }
}

// All products fit u64, including the full 65536 * 65536 divider. Never cast
// ARR+1 through u16, and reject impossible requests before touching the device.
pub(crate) fn solve(clock: u32, hz: u32, prescalers: Prescalers) -> Result<Config, Error> {
    if hz == 0 || u64::from(hz) * 2 > u64::from(clock) {
        return Err(Error::InvalidFrequency);
    }
    let clock = u64::from(clock);
    let minimum = clock.div_ceil(u64::from(hz) * 65536) as u32;
    let divisor = prescalers
        .at_least(minimum)
        .ok_or(Error::InvalidFrequency)?;
    let period_ticks = clock.div_ceil(u64::from(hz) * u64::from(divisor)) as u32;
    let config = Config {
        prescaler_divisor: divisor,
        period_ticks,
    };
    config
        .validate(prescalers)
        .map_err(|_| Error::InvalidFrequency)?;
    Ok(config)
}
