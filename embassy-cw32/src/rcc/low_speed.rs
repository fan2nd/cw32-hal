//! Permanently enabled low-speed clocks for the independently clocked RTC.

use crate::pac;
#[cfg(gpio)]
use crate::{
    gpio::{AnyPin, Pin},
    Peri,
};

/// RTC input clock. Nominal frequencies are not measured frequencies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcClockSource {
    /// A board-mounted 32.768 kHz crystal on OSC32_IN/OUT.
    Lse,
    /// The internal RC at its existing trim, nominally 32.8 kHz on both chips.
    Lsi,
}
impl RtcClockSource {
    pub const fn nominal_frequency(self) -> u32 {
        match self {
            Self::Lse => 32_768,
            Self::Lsi => 32_800,
        }
    }
    pub(crate) const fn register_value(self) -> u8 {
        match self {
            Self::Lse => 0,
            Self::Lsi => 2,
        }
    }
}

/// Low-speed startup failure. A timed-out oscillator is left enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowSpeedError {
    InvalidConfig,
    IncompatibleRunningClock,
    NotStable,
    ReadbackMismatch,
}

/// Board-specific LSE crystal settings. These are applied only while LSE is off.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LseConfig {
    /// 0, 1, 2, 3 select 256, 1024, 4096, 16384 stabilization cycles.
    pub wait_cycles: u8,
    /// Crystal drive: 0–15 on L012, 0–3 on F030. Select for the actual board.
    pub drive: u8,
    /// L012 crystal drive before stabilization, 0–15.
    #[cfg(sysctrl_l012)]
    pub startup_drive: u8,
    /// F030 crystal amplitude, 0–3 (2 is the manual's recommended setting).
    #[cfg(sysctrl_f030)]
    pub amplitude: u8,
}
impl Default for LseConfig {
    fn default() -> Self {
        Self {
            wait_cycles: 2,
            #[cfg(sysctrl_l012)]
            drive: 2,
            #[cfg(sysctrl_l012)]
            startup_drive: 10,
            #[cfg(sysctrl_f030)]
            drive: 3,
            #[cfg(sysctrl_f030)]
            amplitude: 2,
        }
    }
}

/// A configured, permanently reserved RTC source.
///
/// Consumed by `rtc::Rtc`. There is deliberately no shutdown on Drop: the RTC
/// can keep time after its software owner is dropped. LSE takes static pins,
/// permanently reserving them for the lifetime of this boot, even on error.
/// Neither this token nor Drop guarantees retention through reset or power loss.
pub struct RtcClock {
    source: RtcClockSource,
}
impl RtcClock {
    /// Enable LSI without changing an already running oscillator's trim.
    ///
    /// Call after HAL initialization. The existing LSI trim is preserved: an
    /// autonomous watchdog/filter may already be using this shared oscillator.
    /// The bound counts register polls, not elapsed time. Zero is rejected.
    /// On timeout LSI stays enabled; this method may be called again to retry.
    pub fn start_lsi(poll_limit: u32) -> Result<Self, LowSpeedError> {
        let _ = super::clocks();
        if poll_limit == 0 {
            return Err(LowSpeedError::InvalidConfig);
        }
        critical_section::with(|_| {
            let mut value = pac::SYSCTRL.cr1().read();
            if !value.lsien() {
                value.set_key((pac::SYSCTRL_KEY >> 16) as u16);
                value.set_lsien(true);
                pac::SYSCTRL.cr1().write_value(value);
            }
        });
        Self::wait_stable(RtcClockSource::Lsi, poll_limit)
    }

    /// Start or adopt a 32.768 kHz crystal, permanently consuming its two pins.
    ///
    /// The crystal frequency/wiring and drive settings are board requirements.
    /// Both pins must be owned for the entire boot; borrowed local pins cannot
    /// be passed. Pins are consumed even on failure and are never returned.
    /// An already-enabled LSE is only adopted if its mode and settings match;
    /// its pads and oscillator registers are left untouched in that case.
    /// Returns as soon as the oscillator is enabled, before it is necessarily
    /// stable. Call `wait_ready` to wait; its timeout retains this handle and
    /// can be retried without reacquiring or reconfiguring either GPIO pin.
    #[cfg(gpio)]
    pub fn start_lse(
        input: Peri<'static, impl LseInputPin>,
        output: Peri<'static, impl LseOutputPin>,
        config: LseConfig,
    ) -> Result<Self, LowSpeedError> {
        let _ = super::clocks();
        let input: Peri<'static, AnyPin> = input.into();
        let output: Peri<'static, AnyPin> = output.into();
        #[cfg(sysctrl_l012)]
        let valid_drive = config.drive <= 15 && config.startup_drive <= 15;
        #[cfg(sysctrl_f030)]
        let valid_drive = config.drive <= 3 && config.amplitude <= 3;
        if config.wait_cycles > 3 || !valid_drive {
            return Err(LowSpeedError::InvalidConfig);
        }
        critical_section::with(|_| {
            let mut cr1 = pac::SYSCTRL.cr1().read();
            let mut lse = pac::SYSCTRL.lse().read();
            if cr1.lseen() {
                if !matches_lse(lse, config) || !input.is_analog() || !output.is_analog() {
                    return Err(LowSpeedError::IncompatibleRunningClock);
                }
            } else {
                // Safe because the oscillator is off and both exact pads are
                // exclusively held; no Flex destructor later disconnects them.
                input.configure_analog();
                output.configure_analog();
                lse.set_mode(false);
                lse.set_waitcycle(config.wait_cycles);
                lse.set_driver(config.drive);
                #[cfg(sysctrl_l012)]
                lse.set_pdriver(config.startup_drive);
                #[cfg(sysctrl_f030)]
                lse.set_amp(config.amplitude);
                pac::SYSCTRL.lse().write_value(lse);
                if !matches_lse(pac::SYSCTRL.lse().read(), config) {
                    return Err(LowSpeedError::ReadbackMismatch);
                }
                cr1.set_key((pac::SYSCTRL_KEY >> 16) as u16);
                cr1.set_lseen(true);
                pac::SYSCTRL.cr1().write_value(cr1);
            }
            Ok(())
        })?;
        if !pac::SYSCTRL.cr1().read().lseen() {
            return Err(LowSpeedError::ReadbackMismatch);
        }
        Ok(Self {
            source: RtcClockSource::Lse,
        })
    }
    fn wait_stable(source: RtcClockSource, limit: u32) -> Result<Self, LowSpeedError> {
        let clock = Self { source };
        clock.wait_ready(limit)?;
        Ok(clock)
    }
    /// Wait for enable/stability without consuming this source reservation.
    ///
    /// The bound counts polls, not elapsed time. On timeout the oscillator and
    /// pins remain reserved; this method can be retried on the same token.
    pub fn wait_ready(&self, limit: u32) -> Result<(), LowSpeedError> {
        if limit == 0 {
            return Err(LowSpeedError::InvalidConfig);
        }
        for _ in 0..limit {
            if self.is_ready() {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(LowSpeedError::NotStable)
    }
    pub const fn source(&self) -> RtcClockSource {
        self.source
    }
    /// Recheck enable and stability bits; this does not measure the frequency.
    pub fn is_ready(&self) -> bool {
        let cr1 = pac::SYSCTRL.cr1().read();
        match self.source {
            RtcClockSource::Lsi => cr1.lsien() && pac::SYSCTRL.lsi().read().stable(),
            RtcClockSource::Lse => cr1.lseen() && pac::SYSCTRL.lse().read().stable(),
        }
    }
}
#[cfg(gpio)]
fn matches_lse(lse: pac::sysctrl::regs::Lse, config: LseConfig) -> bool {
    let common =
        !lse.mode() && lse.waitcycle() == config.wait_cycles && lse.driver() == config.drive;
    #[cfg(sysctrl_l012)]
    {
        common && lse.pdriver() == config.startup_drive
    }
    #[cfg(sysctrl_f030)]
    {
        common && lse.amp() == config.amplitude
    }
}
#[cfg(gpio)]
pub(crate) mod sealed {
    pub trait LseInputPin {}
    pub trait LseOutputPin {}
}
/// Verified OSC32_IN pad, generated from device metadata.
#[allow(private_bounds)]
#[cfg(gpio)]
pub trait LseInputPin: Pin + sealed::LseInputPin {}
/// Verified OSC32_OUT pad, generated from device metadata.
#[allow(private_bounds)]
#[cfg(gpio)]
pub trait LseOutputPin: Pin + sealed::LseOutputPin {}
