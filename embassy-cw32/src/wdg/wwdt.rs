//! Window watchdog: CW32L012 RM1.4 chapter 20 and CW32x030 RM2.5 chapter 17.

use crate::{rcc::KernelClock, Peri, PeripheralType};

mod sealed {
    pub trait Instance {
        fn regs() -> crate::pac::wwdt::Wwdt;
    }
}
/// Audited window-watchdog identity and PCLK input from device metadata.
#[allow(private_bounds)]
pub trait WindowInstance: sealed::Instance + KernelClock + PeripheralType + 'static {}
include!(concat!(env!("OUT_DIR"), "/_generated_wwdt.rs"));

/// Divider of the APB clock before the seven-bit window watchdog counter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum WindowPrescaler {
    #[default]
    Div4096 = 0,
    Div8192 = 1,
    Div16384 = 2,
    Div32768 = 3,
    Div65536 = 4,
    Div131072 = 5,
    Div262144 = 6,
    Div524288 = 7,
}

/// Reset-only window watchdog configuration. The pre-overflow IRQ is disabled.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct WindowConfig {
    pub prescaler: WindowPrescaler,
    /// Reload counter, in 0x41..=0x7f and strictly greater than `window`.
    pub reload: u8,
    /// Refresh becomes legal when the counter reaches this value, in 0x40..reload.
    pub window: u8,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            prescaler: WindowPrescaler::Div4096,
            reload: 0x7f,
            window: 0x5f,
        }
    }
}

impl WindowConfig {
    fn valid(&self) -> bool {
        self.reload <= 0x7f && self.window >= 0x40 && self.window < self.reload
    }

    /// Nominal PCLK cycles from reload until the window opens, using the manual's
    /// formula. Prescaler phase, clock accuracy and DeepSleep affect real timing.
    /// Returns `None` for an invalid configuration.
    pub const fn closed_window_cycles(&self) -> Option<u32> {
        if self.reload > 0x7f || self.window < 0x40 || self.window >= self.reload {
            return None;
        }
        Some((4096 << self.prescaler as u32) * (self.reload - self.window) as u32)
    }

    /// Nominal PCLK cycles from reload to reset at counter 0x3f. Refresh must finish
    /// strictly before this boundary. Returns `None` for an invalid configuration.
    pub const fn timeout_cycles(&self) -> Option<u32> {
        if self.reload > 0x7f || self.window < 0x40 || self.window >= self.reload {
            return None;
        }
        Some((4096 << self.prescaler as u32) * (self.reload - 0x3f) as u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowError {
    InvalidConfig,
    /// Existing watchdog policy is preserved; it cannot be safely restarted.
    AlreadyRunning,
    /// The early-warning IE bit was set previously and cannot be cleared.
    InterruptAlreadyEnabled,
    NotStarted,
    /// The live counter is still above the refresh window. No write was issued.
    TooEarly,
    /// The counter has reached the reset boundary. No write was issued.
    TooLate,
}

/// Exclusive window watchdog owner.
///
/// [`unleash`](Self::unleash) starts an irreversible countdown. Dropping or
/// forgetting the driver keeps its PCLK gate enabled and never disables the
/// watchdog. DeepSleep stops this hardware counter; this is not an independent
/// wall-clock watchdog. No stop or early-warning interrupt API is provided.
pub struct WindowWatchdog<'d, T: WindowInstance> {
    _peripheral: Peri<'d, T>,
    clock: crate::rcc::ClockGuard,
    config: WindowConfig,
    started: bool,
}

impl<'d, T: WindowInstance> WindowWatchdog<'d, T> {
    /// Acquire without reset, refresh, start, or changes to existing policy.
    /// Configuration is validated before any peripheral access.
    pub fn new(peripheral: Peri<'d, T>, config: WindowConfig) -> Result<Self, WindowError> {
        if !config.valid() {
            return Err(WindowError::InvalidConfig);
        }
        Ok(Self {
            _peripheral: peripheral,
            clock: T::acquire_no_reset(),
            config,
            started: false,
        })
    }

    /// Configure and start. After success only a reset can stop the watchdog.
    /// An inherited running watchdog or one-way interrupt enable is rejected
    /// before changing any watchdog register.
    pub fn unleash(&mut self) -> Result<(), WindowError> {
        critical_section::with(|_| {
            if self.started || T::regs().cr0().read().en() {
                return Err(WindowError::AlreadyRunning);
            }
            if T::regs().cr1().read().ie() {
                return Err(WindowError::InterruptAlreadyEnabled);
            }
            T::regs().cr1().write(|w| {
                w.set_prs(self.config.prescaler as u8);
                w.set_winr(self.config.window);
                w.set_ie(false);
            });
            T::regs().sr().write(|w| w.set_pov(false));
            // CR0 is a refresh command, never an RMW of a moving counter.
            T::regs().cr0().write(|w| {
                w.set_wcnt(self.config.reload);
                w.set_en(true);
            });
            self.started = true;
            Ok(())
        })
    }

    /// Refresh only after observing an open window, under one critical section.
    ///
    /// `TooEarly` and `TooLate` do not write CR0. The hardware keeps counting
    /// between the observation and write: a call at the deadline can still reset
    /// the MCU, including if an NMI/debug halt delays it. Schedule refresh with
    /// margin inside the window; this method cannot make a late caller safe.
    pub fn try_pet(&mut self) -> Result<(), WindowError> {
        critical_section::with(|_| {
            if !self.started {
                return Err(WindowError::NotStarted);
            }
            let counter = T::regs().cr0().read().wcnt();
            if counter > self.config.window {
                return Err(WindowError::TooEarly);
            }
            if counter <= 0x3f {
                return Err(WindowError::TooLate);
            }
            T::regs().cr0().write(|w| {
                w.set_wcnt(self.config.reload);
                w.set_en(true);
            });
            Ok(())
        })
    }

    /// Live countdown for diagnostics. Reading does not refresh it.
    pub fn counter(&self) -> u8 {
        T::regs().cr0().read().wcnt()
    }

    pub fn is_running(&self) -> bool {
        T::regs().cr0().read().en()
    }

    /// Fixed PCLK input frequency established during HAL initialization.
    pub fn clock_frequency(&self) -> u32 {
        crate::rcc::frequency::<T>()
    }

    pub fn config(&self) -> WindowConfig {
        self.config
    }
}

impl<T: WindowInstance> Drop for WindowWatchdog<'_, T> {
    fn drop(&mut self) {
        if self.started || T::regs().cr0().read().en() {
            self.clock.pin();
        }
    }
}
