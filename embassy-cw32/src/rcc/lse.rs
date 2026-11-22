//! Init-only LSE on the exact packages admitted by the own-source metadata.
use super::{ClockBounds, Error, OperatingConditions};
use crate::{pac, time::Hertz};

pub use pac::sysctrl::vals::{LseAmplitude, LseDrive, LseWait};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LseMode {
    Oscillator,
    Bypass,
}

/// Board-qualified nominal 32768 Hz source.
///
/// Minimum/maximum must bound every cycle throughout the declared conditions,
/// including load, temperature, aging, supply and short-term variation. An
/// average ppm claim alone is insufficient. The board must qualify oscillator
/// drive/load and bypass voltage, duty, edge and pulse-width requirements.
/// No accuracy, startup upper bound or continuous availability is measured.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Lse {
    pub min_freq: Hertz,
    pub max_freq: Hertz,
    pub operating_conditions: OperatingConditions,
    pub mode: LseMode,
    pub drive: LseDrive,
    pub amplitude: LseAmplitude,
    /// Independent pre-stable drive. Programmed before enable, never retuned.
    #[cfg(rcc_lse_startup_analog)]
    pub startup_drive: LseDrive,
    /// Independent pre-stable amplitude; exact hardware phase-switch timing is
    /// unspecified. Both native analog banks are configured before enable.
    #[cfg(rcc_lse_startup_analog)]
    pub startup_amplitude: LseAmplitude,
    pub wait: LseWait,
    /// Explicit nonzero maximum polling attempts, not milliseconds. Crystal
    /// startup can be slow. A timeout retains EN and reservations. LSE control is
    /// POR-retained on supported devices; an ordinary reset may not permit retry.
    pub poll_budget: u32,
}
impl Lse {
    pub(crate) fn bounds(self, board: OperatingConditions) -> Result<ClockBounds, Error> {
        if self.poll_budget == 0 {
            return Err(Error::InvalidTimeout);
        }
        if self.max_freq.0 > crate::RCC_LSE_MAXIMUM_HZ {
            return Err(Error::InvalidLseBounds);
        }
        // Own L083 detector counts 128 LSE edges per 256 factory-trim LSI
        // clocks. Demand one extra edge of board-qualified phase margin.
        #[cfg(rcc_cw32l083_v1)]
        if u64::from(self.min_freq.0) * 256
            <= 129
                * u64::from(
                    <crate::peripherals::RTC as crate::rtc::sealed::Instance>::SOURCE_MAXIMUM_HZ,
                )
        {
            return Err(Error::InvalidLseBounds);
        }
        let c = self.operating_conditions;
        let bounds = ClockBounds::external(
            Hertz(crate::RCC_LSE_NOMINAL_HZ),
            self.min_freq,
            self.max_freq,
            c,
        )
        .ok_or(Error::InvalidLseBounds)?;
        if c.supply_mv < crate::RCC_LSE_SUPPLY_MV.0
            || c.supply_mv > crate::RCC_LSE_SUPPLY_MV.1
            || c.min_temperature_c < crate::RCC_LSE_TEMPERATURE_C.0
            || c.max_temperature_c > crate::RCC_LSE_TEMPERATURE_C.1
            || board.supply_mv != c.supply_mv
            || board.min_temperature_c < c.min_temperature_c
            || board.max_temperature_c > c.max_temperature_c
        {
            return Err(Error::InvalidLseBounds);
        }
        Ok(bounds)
    }
    fn bypass(self) -> bool {
        self.mode == LseMode::Bypass
    }
}

fn parameters_match(config: Lse) -> bool {
    let r = pac::SYSCTRL.lse().read();
    #[cfg(rcc_lse_startup_analog)]
    if r.pdriver() != config.startup_drive || r.pamp() != config.startup_amplitude {
        return false;
    }
    r.mode() == config.bypass()
        && r.driver() == config.drive
        && r.amp() == config.amplitude
        && r.waitcycle() == config.wait
}
fn faults() -> bool {
    let flags = pac::SYSCTRL.isr().read();
    flags.lsefail() || flags.lsefault()
}
// A successful configurable-CCS start freezes the detector clock parameters.
// No code here calibrates LSI or clears a retained fault.
static MONITOR_LSI: critical_section::Mutex<core::cell::Cell<Option<(u16, u8)>>> =
    critical_section::Mutex::new(core::cell::Cell::new(None));

fn monitor_ready(require_frozen: bool) -> bool {
    #[cfg(any(rcc_v1, rcc_cw32f020_v1))]
    if !super::hsi_48mhz::lse_sysclk_monitor_ready() {
        return false;
    }
    #[cfg(rcc_cw32l031_v1)]
    if !super::l031_r031_w031::lse_sysclk_monitor_ready() {
        return false;
    }
    #[cfg(any(rcc_cw32l052_v1, rcc_cw32l083_v1))]
    if !super::l052_l083::lse_sysclk_monitor_ready() {
        return false;
    }
    if !crate::RCC_LSE_CONFIGURABLE_CCS {
        return true;
    }
    let cr1 = pac::SYSCTRL.cr1().read();
    let lsi = pac::SYSCTRL.lsi().read();
    cr1.lseccs()
        && cr1.lsien()
        && lsi.stable()
        && critical_section::with(|cs| match MONITOR_LSI.borrow(cs).get() {
            Some((trim, wait)) => lsi.trim() == trim && lsi.waitcycle() == wait,
            None => !require_frozen,
        })
}
fn freeze_monitor(cs: critical_section::CriticalSection<'_>) -> Result<(), Error> {
    if crate::RCC_LSE_CONFIGURABLE_CCS {
        let cr1 = pac::SYSCTRL.cr1().read();
        let lsi = pac::SYSCTRL.lsi().read();
        if !cr1.lsien() || !lsi.stable() {
            return Err(Error::LseNotReady);
        }
        let parameters = (lsi.trim(), lsi.waitcycle());
        if MONITOR_LSI
            .borrow(cs)
            .get()
            .is_some_and(|old| old != parameters)
        {
            return Err(Error::LseNotReady);
        }
        MONITOR_LSI.borrow(cs).set(Some(parameters));
    }
    Ok(())
}
// L052/L083 system targets supply their own stronger cold/start use-edge checks.
// This target-only entry freezes the same common RTC monitor, without changing
// the existing auxiliary startup path or publishing system-clock success.
#[cfg(any(rcc_cw32l052_v1, rcc_cw32l083_v1))]
pub(super) fn freeze_sysclk_monitor(
    cs: critical_section::CriticalSection<'_>,
) -> Result<(), Error> {
    freeze_monitor(cs)
}

fn ready(config: Lse, require_frozen: bool) -> bool {
    pac::SYSCTRL.cr1().read().lseen()
        && pac::SYSCTRL.lse().read().stable()
        && !faults()
        && parameters_match(config)
        && monitor_ready(require_frozen)
}
pub(crate) fn healthy(config: Lse) -> bool {
    ready(config, true)
}
pub(crate) fn verify(config: Lse, cs: critical_section::CriticalSection<'_>) -> Result<(), Error> {
    verify_state(config, true, cs)
}
fn verify_state(
    config: Lse,
    require_frozen: bool,
    cs: critical_section::CriticalSection<'_>,
) -> Result<(), Error> {
    if !ready(config, require_frozen) {
        return Err(Error::LseNotReady);
    }
    if !crate::rcc_lse_pins_match(config.bypass(), false, config.poll_budget, cs)? {
        return Err(Error::LsePinConflict);
    }
    if !ready(config, require_frozen) {
        return Err(Error::LseNotReady);
    }
    Ok(())
}

/// Admission runs before any oscillator or pad mutation. Gate-only inspection
/// restores original gates on success; failed readback may leave a gate enabled.
pub(crate) fn preflight(
    config: Lse,
    cs: critical_section::CriticalSection<'_>,
) -> Result<bool, Error> {
    let control = pac::SYSCTRL.cr1().read();
    if control.lseen() {
        verify_state(config, false, cs)?;
        return Ok(true);
    }
    #[cfg(rcc_lse_native_consumers)]
    {
        let flags = pac::SYSCTRL.isr().read();
        if flags.lserdy() || flags.lsestable() {
            return Err(Error::LseClockInUse);
        }
    }
    let interrupts = pac::SYSCTRL.ier().read();
    if control.lselock()
        || pac::SYSCTRL.lse().read().stable()
        || faults()
        || interrupts.lserdy()
        || interrupts.lsefail()
        || interrupts.lsefault()
    {
        return Err(Error::LseClockInUse);
    }
    if !crate::rcc_lse_consumers_idle(config.poll_budget, cs)?
        || !crate::rcc_lse_pins_match(config.bypass(), true, config.poll_budget, cs)?
    {
        return Err(Error::LseClockInUse);
    }
    // Inspection is bounded; reject a source that changed while reading pads.
    if pac::SYSCTRL.cr1().read().lseen()
        || pac::SYSCTRL.cr1().read().lselock()
        || pac::SYSCTRL.lse().read().stable()
        || faults()
    {
        return Err(Error::LseClockInUse);
    }
    Ok(false)
}
pub(crate) fn start(
    config: Lse,
    reused: bool,
    cs: critical_section::CriticalSection<'_>,
) -> Result<(), Error> {
    if reused {
        freeze_monitor(cs)?;
        return verify(config, cs);
    }
    // Other RCC setup may have waited. Recheck admission immediately before
    // changing pads; retained RTC wake paths are independent of calendar START.
    if preflight(config, cs)? {
        return Err(Error::LseClockInUse);
    }
    freeze_monitor(cs)?;
    crate::rcc_configure_lse_pins(config.bypass(), config.poll_budget, cs)?;
    // Program only actual source-defined fields while disabled. Native
    // AUTOTRIM-equipped parts preserve reserved bits; L052 alone has a second
    // analog bank. Older cohorts retain their original write.
    let configure = |w: &mut pac::sysctrl::regs::Lse| {
        w.set_mode(config.bypass());
        w.set_driver(config.drive);
        w.set_amp(config.amplitude);
        #[cfg(rcc_lse_startup_analog)]
        {
            w.set_pdriver(config.startup_drive);
            w.set_pamp(config.startup_amplitude);
        }
        w.set_waitcycle(config.wait);
    };
    #[cfg(rcc_lse_native_consumers)]
    pac::SYSCTRL.lse().modify(configure);
    #[cfg(not(rcc_lse_native_consumers))]
    pac::SYSCTRL.lse().write(configure);
    if !parameters_match(config) {
        return Err(Error::LseNotReady);
    }
    pac::SYSCTRL.cr1().modify(|w| {
        w.set_key(0x5a5a);
        if crate::RCC_LSE_CONFIGURABLE_CCS {
            w.set_lseccs(true);
        }
        w.set_lseen(true);
    });
    for _ in 0..config.poll_budget {
        if faults() {
            return Err(Error::LseNotReady);
        }
        if healthy(config) {
            return verify(config, cs);
        }
        core::hint::spin_loop();
    }
    Err(Error::LseNotReady)
}
