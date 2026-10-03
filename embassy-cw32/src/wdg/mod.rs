//! Independent watchdog with an explicit start and no implicit stop on Drop.
//!
//! The watchdog runs from its dedicated RC10K oscillator, nominally 10 kHz.
//! Its tolerance is not the RCC clock accuracy. Choose margin for the actual
//! device's oscillator limits. Window mode and watchdog interrupts are not used.

use crate::{pac, peripherals::IWDT, rcc::PeripheralClock, Peri};

/// Divider of the dedicated watchdog RC oscillator.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescaler {
    #[default]
    Div4 = 0,
    Div8 = 1,
    Div16 = 2,
    Div32 = 3,
    Div64 = 4,
    Div128 = 5,
    Div256 = 6,
    Div512 = 7,
}

/// Reset-only watchdog configuration, with the early-feed window disabled.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    pub prescaler: Prescaler,
    /// Reload value in 0..=4095. A period contains reload + 1 divided ticks.
    pub reload: u16,
    /// Maximum status reads for each clock-domain synchronization. This is a
    /// finite work budget, not an elapsed-time deadline.
    pub poll_limit: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prescaler: Prescaler::Div4,
            reload: 2499,
            poll_limit: 1_000_000,
        }
    }
}

impl Config {
    /// Nominal period assuming exactly 10 kHz RC10K. Not a timing guarantee.
    pub const fn nominal_timeout_us(&self) -> u32 {
        100 * (4 << self.prescaler as u32) * (self.reload as u32 + 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The reload exceeds 12 bits, or the polling budget is zero.
    InvalidConfig,
    /// Starting over an already-running watchdog would change its safety policy.
    AlreadyRunning,
    /// The watchdog was not completely configured by a successful unleash.
    NotStarted,
    /// A status synchronization exceeded its polling budget. The watchdog may
    /// still be running; explicitly stop it or allow its reset to occur.
    Timeout,
}

/// Exclusive watchdog owner. Dropping or forgetting it never stops a started
/// watchdog. The application remains responsible for timely petting or reset.
pub struct IndependentWatchdog<'d> {
    _peripheral: Peri<'d, IWDT>,
    clock: crate::rcc::ClockGuard,
    config: Config,
    started: bool,
    configured: bool,
}

impl<'d> IndependentWatchdog<'d> {
    /// Acquire the watchdog without starting, stopping or resetting it.
    /// CW32 only accepts configuration after start, so hardware configuration
    /// is performed by [`Self::unleash`]. The default nominal period is 1 s.
    pub fn new(peripheral: Peri<'d, IWDT>, config: Config) -> Result<Self, Error> {
        if config.reload > 0xfff || config.poll_limit == 0 {
            return Err(Error::InvalidConfig);
        }
        let clock = IWDT::acquire_no_reset();
        Ok(Self {
            _peripheral: peripheral,
            clock,
            config,
            started: pac::IWDT.sr().read().run(),
            configured: false,
        })
    }

    /// Start and configure reset-on-expiry operation. Window mode is disabled,
    /// IRQs are disabled, and counting continues during DeepSleep.
    ///
    /// On an error after start the timer may keep counting with a partial
    /// configuration. The owner remains available for an explicit `stop`.
    pub fn unleash(&mut self) -> Result<(), Error> {
        if self.started || self.is_running() {
            return Err(Error::AlreadyRunning);
        }
        self.started = true;
        self.configured = false;
        key(0xcccc);
        let result = (|| {
            self.wait(|s| s.run())?;
            key(0x5555);
            self.wait(|s| !s.crf())?;
            pac::IWDT.cr().write(|w| {
                w.set_prs(self.config.prescaler as u8);
                w.set_action(false);
                w.set_ie(false);
                w.set_pause(false);
            });
            self.wait(|s| !s.crf())?;
            self.wait(|s| !s.arrf())?;
            pac::IWDT.arr().write(|w| w.set_arr(self.config.reload));
            self.wait(|s| !s.arrf())?;
            self.wait(|s| !s.winrf())?;
            pac::IWDT.winr().write(|w| w.set_winr(0xfff));
            self.wait(|s| !s.winrf() && !s.reload())?;
            key(0xaaaa);
            self.wait(|s| !s.reload())
        })();
        // Every key other than 0x5555 locks CR/ARR/WINR. Close even a failed
        // configuration; no implicit refresh, stop or reset is issued here.
        key(0);
        self.configured = result.is_ok();
        result
    }

    /// Reload a successfully started watchdog. No early-feed window is active.
    pub fn pet(&mut self) -> Result<(), Error> {
        if !self.configured || !self.is_running() {
            return Err(Error::NotStarted);
        }
        self.wait(|s| !s.reload())?;
        key(0xaaaa);
        self.wait(|s| !s.reload())
    }

    /// Explicitly stop the CW32 watchdog using its documented two-key command.
    /// This removes reset protection until another successful `unleash`.
    pub fn stop(&mut self) -> Result<(), Error> {
        self.configured = false;
        critical_section::with(|_| {
            key(0x5a5a);
            key(0xa5a5);
        });
        self.wait(|s| !s.run())?;
        self.started = false;
        Ok(())
    }

    /// Hardware RUN status. This does not reload the counter.
    pub fn is_running(&self) -> bool {
        pac::IWDT.sr().read().run()
    }

    fn wait(&self, ready: impl Fn(pac::iwdt::regs::Sr) -> bool) -> Result<(), Error> {
        for _ in 0..self.config.poll_limit {
            if ready(pac::IWDT.sr().read()) {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::Timeout)
    }
}

impl Drop for IndependentWatchdog<'_> {
    fn drop(&mut self) {
        if self.started || self.is_running() {
            // An uncertain start/stop or a running watchdog retains the config
            // gate. Dropping its Rust owner never weakens reset protection.
            self.clock.pin();
        }
    }
}

fn key(value: u16) {
    pac::IWDT.kr().write(|w| w.set_kr(value));
}
