//! Owned, whole-second RTC calendar access on CW32L012 and CW32F030.
//!
//! A started low-speed clock token is required. Construction never resets or
//! starts the calendar; call `initialize` explicitly when it is stopped. Reads
//! preserve timekeeping, while `set_datetime` deliberately stops the counter
//! briefly to replace the complete calendar. See `docs/rtc.md` for accuracy,
//! reset retention and unsupported alarm/low-power features.

mod datetime;
pub use datetime::{DateTime, DateTimeError, DayOfWeek};

#[cfg(rtc_l012)]
#[path = "l012.rs"]
mod hardware;
#[cfg(rtc_f030)]
#[path = "f030.rs"]
mod hardware;

use crate::{
    pac,
    peripherals::RTC,
    rcc::{PeripheralClock, RtcClock, RtcClockSource},
    Peri,
};

/// Bounded polling configuration. The count is not a wall-clock timeout.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    /// Maximum synchronization/window polls or repeated calendar snapshots.
    /// A short bound may return `SynchronizationTimeout` during normal updates.
    pub poll_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            poll_limit: 100_000,
        }
    }
}
/// RTC operation failure. Errors never perform an implicit peripheral reset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcError {
    InvalidConfig,
    ClockNotReady,
    AlreadyRunning,
    NotRunning,
    /// Retained source, divider, 12-hour mode, compensation, alarm, output, IRQ
    /// or access mode
    /// is outside this driver's contract. Existing state is left unchanged.
    IncompatibleConfiguration,
    InvalidDateTime(DateTimeError),
    SynchronizationTimeout,
    /// A calendar write did not read back. The RTC may be stopped; query it and
    /// explicitly initialize again instead of assuming that the write succeeded.
    ReadbackMismatch,
}

/// Exclusive software owner of the RTC calendar.
///
/// Drop leaves the RTC, source oscillator and APB gate enabled. It neither
/// resets the retained calendar nor returns the clock's permanently owned pins.
/// Use `release` to recover the RTC and source tokens for a later attachment.
pub struct Rtc<'d> {
    _peripheral: Peri<'d, RTC>,
    _bus_clock: crate::rcc::ClockGuard,
    clock: RtcClock,
    config: Config,
}
impl<'d> Rtc<'d> {
    /// Attach without changing the retained calendar or its control registers.
    ///
    /// A running RTC must already use this token's source, 24-hour mode and this
    /// driver's prescaler, with compensation, alarms, wakeup, tamper, clock
    /// outputs and all RTC interrupts disabled.
    /// A stopped RTC may be explicitly initialized afterward. The bus gate is
    /// enabled without reset and permanently retained, including on error.
    pub fn new(
        peripheral: Peri<'d, RTC>,
        clock: RtcClock,
        config: Config,
    ) -> Result<Self, RtcError> {
        if config.poll_limit == 0 {
            return Err(RtcError::InvalidConfig);
        }
        if !clock.is_ready() {
            return Err(RtcError::ClockNotReady);
        }
        let mut bus_clock = RTC::acquire_no_reset();
        bus_clock.pin();
        let this = Self {
            _peripheral: peripheral,
            _bus_clock: bus_clock,
            clock,
            config,
        };
        if this.is_running() {
            this.check_configuration()?;
        }
        Ok(this)
    }
    pub fn is_running(&self) -> bool {
        pac::RTC.cr0().read().start()
    }
    pub fn clock_source(&self) -> RtcClockSource {
        self.clock.source()
    }

    /// Return the RTC and source tokens without changing timekeeping hardware.
    /// The bus gate stays pinned. The source token still owns the oscillator's
    /// permanent reservation, and can be passed to another `Rtc::new` call.
    pub fn release(self) -> (Peri<'d, RTC>, RtcClock) {
        (self._peripheral, self.clock)
    }

    /// Deliberately replace a stopped RTC's configuration and calendar, then start.
    ///
    /// Refuses a running RTC. This explicitly overwrites any retained date/time
    /// in a stopped device. It selects 24-hour mode, disables compensation,
    /// alarms, tamper, output and interrupts, and sets the chip's calendar divider.
    /// It never pulses the peripheral reset or clears system reset-cause flags.
    pub fn initialize(&mut self, datetime: DateTime) -> Result<(), RtcError> {
        if !self.clock.is_ready() {
            return Err(RtcError::ClockNotReady);
        }
        if self.is_running() {
            return Err(RtcError::AlreadyRunning);
        }
        critical_section::with(|_| {
            let _write = WriteGuard::new();
            let mut cr0 = pac::rtc::regs::Cr0::default();
            cr0.set_h24(true);
            pac::RTC.cr0().write_value(cr0);
            let mut cr1 = pac::rtc::regs::Cr1::default();
            cr1.set_source(self.clock.source().register_value());
            pac::RTC.cr1().write_value(cr1);
            pac::RTC.cr2().write_value(pac::rtc::regs::Cr2::default());
            pac::RTC.ier().write_value(pac::rtc::regs::Ier::default());
            hardware::configure(self.clock.source());
            write_calendar(datetime)?;
            // Configuration readback happens while the counter is stopped.
            self.check_configuration()?;
            cr0.set_start(true);
            pac::RTC.cr0().write_value(cr0);
            if !pac::RTC.cr0().read().start() {
                return Err(RtcError::ReadbackMismatch);
            }
            Ok(())
        })
    }

    /// Read a consistent complete calendar without unlocking or stopping it.
    ///
    /// The manuals permit two equal consecutive reads. The driver compares two
    /// complete DATE/TIME pairs in a short critical section, retrying a rollover.
    /// Invalid BCD, impossible Gregorian dates and inconsistent weekdays fail.
    pub fn now(&self) -> Result<DateTime, RtcError> {
        self.check_running()?;
        for _ in 0..self.config.poll_limit {
            let snapshot = critical_section::with(|_| {
                let date = pac::RTC.date().read().0;
                let time = pac::RTC.time().read().0;
                let date_after = pac::RTC.date().read().0;
                let time_after = pac::RTC.time().read().0;
                if date == date_after && time == time_after {
                    Some((date, time))
                } else {
                    None
                }
            });
            if let Some((date, time)) = snapshot {
                return DateTime::from_bits(date, time).map_err(RtcError::InvalidDateTime);
            }
            core::hint::spin_loop();
        }
        Err(RtcError::SynchronizationTimeout)
    }

    /// Set the entire running calendar, deliberately restarting its second phase.
    ///
    /// After the chip-specific WAIT or WINDOW handshake, the counter is stopped
    /// briefly so midnight cannot split the date/time write. Both fields are
    /// verified while stopped and the counter restarts. This does not preserve
    /// subsecond phase. On readback failure the counter remains stopped.
    /// A window timeout before access leaves the old calendar running unchanged.
    pub fn set_datetime(&mut self, datetime: DateTime) -> Result<(), RtcError> {
        self.check_running()?;
        hardware::with_access(self.config.poll_limit, || {
            let mut cr0 = pac::RTC.cr0().read();
            cr0.set_start(false);
            pac::RTC.cr0().write_value(cr0);
            if pac::RTC.cr0().read().start() {
                return Err(RtcError::ReadbackMismatch);
            }
            write_calendar(datetime)?;
            cr0.set_start(true);
            pac::RTC.cr0().write_value(cr0);
            if !pac::RTC.cr0().read().start() {
                return Err(RtcError::ReadbackMismatch);
            }
            Ok(())
        })
    }
    fn check_running(&self) -> Result<(), RtcError> {
        if !self.clock.is_ready() {
            return Err(RtcError::ClockNotReady);
        }
        if !self.is_running() {
            return Err(RtcError::NotRunning);
        }
        self.check_configuration()
    }
    fn check_configuration(&self) -> Result<(), RtcError> {
        if !pac::RTC.cr0().read().h24()
            || pac::RTC.cr0().read().rtc1hz() != 0
            || pac::RTC.cr1().read().source() != self.clock.source().register_value()
            || pac::RTC.cr2().read().0 != 0
            || pac::RTC.ier().read().0 != 0
            || !hardware::configuration_matches(self.clock.source())
        {
            return Err(RtcError::IncompatibleConfiguration);
        }
        Ok(())
    }
}

// Only called with write protection unlocked and START=0. No arbitrary user
// callback runs in this write sequence, keeping F030 ACCESS bounded below 1 s.
fn write_calendar(datetime: DateTime) -> Result<(), RtcError> {
    pac::RTC
        .date()
        .write_value(pac::rtc::regs::Date(datetime.date_bits()));
    pac::RTC
        .time()
        .write_value(pac::rtc::regs::Time(datetime.time_bits()));
    if pac::RTC.date().read().0 != datetime.date_bits()
        || pac::RTC.time().read().0 != datetime.time_bits()
    {
        return Err(RtcError::ReadbackMismatch);
    }
    Ok(())
}
struct WriteGuard;
impl WriteGuard {
    fn new() -> Self {
        pac::RTC.key().write_value(pac::rtc::regs::Key(0xca));
        pac::RTC.key().write_value(pac::rtc::regs::Key(0x53));
        Self
    }
}
impl Drop for WriteGuard {
    fn drop(&mut self) {
        // Both manuals prescribe CA then a value other than 53. The SDK's
        // alternative 55/55 macro is intentionally not used.
        pac::RTC.key().write_value(pac::rtc::regs::Key(0xca));
        pac::RTC.key().write_value(pac::rtc::regs::Key(0));
    }
}
