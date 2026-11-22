//! Native low-power init-only LSE, qualified separately for L010/L011/L012.
//! Direct PAC branches retain each family's consumer, pad and monitor semantics.
use super::{ClockBounds, Error, OperatingConditions, SealedRccPeripheral};
use crate::{pac, peripherals, time::Hertz};

pub use pac::sysctrl::vals::{LseDrive, LseWait};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LseMode {
    Oscillator,
    Bypass,
}

/// The native detector policy; neither mode measures frequency or recovers time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LseFaultDetection {
    /// Count startup edges with CCS clear. STABLE may remain set after later
    /// source loss; subsequent RTC readiness checks cannot detect that loss.
    StartupOnly,
    /// Use an already stable, legally operating, unchanged LSI and the existing
    /// fault routes. No LSI calibration or startup is attempted here. LSIEN may
    /// be clear: native CCS requests LSI automatically. L011/L012 additionally
    /// require an unchanged factory-matching TRIM at the own LSI calibration
    /// halfword, within the declared source/board operating envelope. Factory
    /// matching does not measure frequency or prove individual-cycle jitter.
    ///
    /// A real fault can set timer capture/system-brake flags (including SBIF
    /// before BKE), request an enabled IRQ, and asynchronously clear PWM MOE.
    /// Existing AOE/output controls determine later behavior. These effects can
    /// precede an init error and survive it. Protection routes and flags are
    /// preserved; the HAL neither services their handlers nor restores outputs.
    MonitoredExistingRoutes,
}

/// Board-qualified nominal 32768 Hz source on exact qualified native packages.
/// Bounds must cover every individual cycle throughout the declared supply,
/// ambient, aging and load conditions. Average ppm is insufficient. The board
/// must qualify crystal load/drive/startup or bypass voltage, duty, pulses and
/// edges. No startup maximum or continuing source availability is measured.
/// See [`crate::init`] for the functional downstream-observer and whole GPIO-bank handover limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Lse {
    pub min_freq: Hertz,
    pub max_freq: Hertz,
    pub operating_conditions: OperatingConditions,
    pub mode: LseMode,
    /// Native four-bit running drive, selected for the actual board.
    pub drive: LseDrive,
    /// Independent four-bit startup drive; set before enable, never retuned.
    pub startup_drive: LseDrive,
    pub wait: LseWait,
    pub fault_detection: LseFaultDetection,
    /// Nonzero CPU-poll budget, not milliseconds. Failure retains the source
    /// request and permanent pad reservations. Ordinary reset may retain LSE.
    pub poll_budget: u32,
}
impl Lse {
    pub(crate) fn bounds(self, board: OperatingConditions) -> Result<ClockBounds, Error> {
        if self.poll_budget == 0 {
            return Err(Error::InvalidTimeout);
        }
        let c = self.operating_conditions;
        let bounds = ClockBounds::external(
            Hertz(crate::RCC_LSE_NOMINAL_HZ),
            self.min_freq,
            self.max_freq,
            c,
        )
        .ok_or(Error::InvalidLseBounds)?;
        if self.max_freq.0 > crate::RCC_LSE_MAXIMUM_HZ
            || c.supply_mv < crate::RCC_LSE_SUPPLY_MV.0
            || c.supply_mv > crate::RCC_LSE_SUPPLY_MV.1
            || c.min_temperature_c < crate::RCC_LSE_TEMPERATURE_C.0
            || c.max_temperature_c > crate::RCC_LSE_TEMPERATURE_C.1
            || board.supply_mv != c.supply_mv
            || board.min_temperature_c < c.min_temperature_c
            || board.max_temperature_c > c.max_temperature_c
        {
            return Err(Error::InvalidLseBounds);
        }
        // Sufficient worst-period detector proof, with one complete extra LSE
        // edge of phase margin, explicitly separate from the hardware threshold.
        // Own qualification selects inherited-legal L010 or factory-matching
        // L011/L012 bounds; STABLE never establishes these frequency bounds.
        if self.monitored()
            && u64::from(self.min_freq.0) * u64::from(crate::RCC_LSE_DETECTOR_LSI_CYCLES)
                <= (u64::from(crate::RCC_LSE_DETECTOR_LSE_EDGES)
                    + u64::from(crate::RCC_LSE_DETECTOR_MARGIN_LSE_EDGES))
                    * u64::from(crate::RCC_LSE_MONITORED_LSI_MAXIMUM_HZ)
        {
            return Err(Error::InvalidLseBounds);
        }
        if crate::RCC_LSE_PINS.0.is_none() || (!self.bypass() && crate::RCC_LSE_PINS.1.is_none()) {
            return Err(Error::LsePinConflict);
        }
        Ok(bounds)
    }
    fn bypass(self) -> bool {
        self.mode == LseMode::Bypass
    }
    pub(crate) fn monitored(self) -> bool {
        self.fault_detection == LseFaultDetection::MonitoredExistingRoutes
    }
}

static MONITOR_LSI: critical_section::Mutex<core::cell::Cell<Option<(u16, u8)>>> =
    critical_section::Mutex::new(core::cell::Cell::new(None));

fn lsi_parameters() -> (u16, u8) {
    let lsi = pac::SYSCTRL.lsi().read();
    (lsi.trim(), lsi.waitcycle())
}
// Mask through the own typed PAC field: L011 has ten TRIM bits, L012 nine.
// Never derive this address from HSI calibration or write another LSI owner's trim.
fn monitor_qualified() -> bool {
    if !pac::SYSCTRL.lsi().read().stable() {
        return false;
    }
    #[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
    {
        let factory = unsafe {
            core::ptr::read_volatile(crate::RCC_LSE_LSI_FACTORY_TRIM_ADDRESS as *const u16)
        };
        if factory == u16::MAX {
            return false;
        }
        let mut calibrated = pac::sysctrl::regs::Lsi::default();
        calibrated.set_trim(factory);
        if pac::SYSCTRL.lsi().read().trim() != calibrated.trim() {
            return false;
        }
    }
    true
}

fn parameters_match(config: Lse) -> bool {
    let r = pac::SYSCTRL.lse().read();
    r.mode() == config.bypass()
        && r.driver() == config.drive
        && r.pdriver() == config.startup_drive
        && r.waitcycle() == config.wait
}
fn faults() -> bool {
    let flags = pac::SYSCTRL.isr().read();
    flags.lsefail() || flags.lsefault()
}
fn ready(config: Lse, require_frozen: bool) -> bool {
    let cr1 = pac::SYSCTRL.cr1().read();
    if !cr1.lseen()
        || !pac::SYSCTRL.lse().read().stable()
        || faults()
        || !parameters_match(config)
        || cr1.lseccs() != config.monitored()
    {
        return false;
    }
    !config.monitored()
        || (monitor_qualified()
            && critical_section::with(|cs| match MONITOR_LSI.borrow(cs).get() {
                Some(parameters) => lsi_parameters() == parameters,
                None => !require_frozen,
            }))
}
pub(crate) fn healthy(config: Lse) -> bool {
    ready(config, true)
}

/// Captured before the first gate write; all frozen monitor parameters must
/// remain unchanged through the enclosing RCC transition.
#[derive(Clone, Copy)]
pub(crate) struct Admission {
    reused: bool,
    lsi: Option<(u16, u8)>,
    lse: u32,
    routes: u32,
    interrupts: u32,
}

impl Admission {
    /// Admit only the enclosing auxiliary-LSE transition's owned maximum WAIT
    /// change. Keep every other original fact and all native start guards.
    pub(crate) fn after_owned_flash_wait(self, expected_wait: u32) -> Result<Self, Error> {
        if u32::from(pac::FLASH.cr2().read().wait()) != expected_wait {
            return Err(Error::FlashLatencyTimeout);
        }
        // Change the local saved word, never the SYSCTRL mirror register.
        let mut routes = pac::sysctrl::regs::Cr2(self.routes);
        routes.set_flashwait(expected_wait as u8);
        let r = pac::SYSCTRL;
        if r.cr2().read().0 != routes.0
            || r.ier().read().0 != self.interrupts
            || r.lse().read().0 != self.lse
        {
            return Err(Error::LseClockInUse);
        }
        if let Some(parameters) = self.lsi {
            if !monitor_qualified() || lsi_parameters() != parameters {
                return Err(Error::LseMonitorNotReady);
            }
        }
        Ok(Self {
            routes: routes.0,
            ..self
        })
    }
}

fn oscillator_gpio() -> super::peripheral::RccInfo {
    #[cfg(rcc_cw32l010_v1)]
    {
        peripherals::GPIOB::RCC_INFO
    }
    #[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
    {
        peripherals::GPIOC::RCC_INFO
    }
}
fn rtc_access_requested() -> bool {
    #[cfg(rcc_cw32l010_v1)]
    {
        pac::RTC.cr1().read().access()
    }
    #[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
    {
        false
    } // Native CR1 bits1:0 are reserved, not ACCESS/WINDOW.
}
fn rtc_trigger(selector: u8) -> bool {
    #[cfg(rcc_cw32l011_v1)]
    {
        matches!(selector, 1..=4)
    } // Code5 is PC13 input.
    #[cfg(any(rcc_cw32l010_v1, rcc_cw32l012_v1))]
    {
        matches!(selector, 1..=5)
    }
}

fn central_new(config: Lse) -> Result<(), Error> {
    let r = pac::SYSCTRL;
    let cr1 = r.cr1().read();
    let lse = r.lse().read();
    let flags = r.isr().read();
    let ier = r.ier().read();
    if cr1.lseen()
        || cr1.lselock()
        || lse.pinlock()
        || lse.stable()
        || flags.lserdy()
        || flags.lsestable()
        || faults()
        || ier.lserdy()
        || ier.lsefail()
        || ier.lsefault()
        || r.cr0().read().sysclk() == pac::sysctrl::vals::Sysclk::Lse
        || r.mco().read().source() == crate::RCC_LSE_MCO_SOURCE
    {
        return Err(Error::LseClockInUse);
    }
    if !config.monitored() && cr1.lseccs() {
        return Err(Error::LseClockInUse);
    }
    if config.monitored() && !monitor_qualified() {
        return Err(Error::LseMonitorNotReady);
    }
    // Active-low resets are observed, never asserted or released to inspect.
    if oscillator_gpio().reset_asserted()
        || peripherals::RTC::RCC_INFO.reset_asserted()
        || peripherals::UART1::RCC_INFO.reset_asserted()
        || peripherals::UART2::RCC_INFO.reset_asserted()
        || peripherals::LPTIM::RCC_INFO.reset_asserted()
    {
        return Err(Error::LseClockInUse);
    }
    #[cfg(rcc_cw32l011_v1)]
    if peripherals::UART3::RCC_INFO.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    #[cfg(rcc_cw32l012_v1)]
    if peripherals::I2C1::RCC_INFO.reset_asserted() || peripherals::I2C2::RCC_INFO.reset_asserted()
    {
        return Err(Error::LseClockInUse);
    }
    Ok(())
}

/// Own-family configuration-only gates. Restores their incoming state on
/// rejection; failed restoration is an error. DATE/PSC/status are never written.
fn configuration_consumers(
    config: Lse,
    cs: critical_section::CriticalSection<'_>,
    l012_sysclk: bool,
) -> Result<bool, Error> {
    let inspection_error = |error| {
        use super::peripheral::ClockInspectionError;
        match error {
            ClockInspectionError::EnableFailed { restore_failed } => {
                Error::LseConfigurationGateTimeout {
                    enable_failed: true,
                    restore_failed,
                }
            }
            ClockInspectionError::RestoreFailed => Error::LseConfigurationGateTimeout {
                enable_failed: false,
                restore_failed: true,
            },
        }
    };
    let rtc = peripherals::RTC::RCC_INFO;
    if l012_sysclk && rtc.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    let rtc_lse = rtc
        .inspect_for_init(cs, config.poll_budget, || {
            if l012_sysclk && rtc.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            let result = (|| {
                let cr1 = pac::RTC.cr1().read();
                if cr1.source() > 3 {
                    return Err(Error::LseClockInUse);
                }
                if cr1.source() != crate::RCC_LSE_RTC_SOURCE {
                    return Ok(false);
                }
                let mut cr0 = pac::RTC.cr0().read();
                cr0.set_h24(false);
                if cr0.0 != 0
                    || rtc_access_requested()
                    || cr1.wait()
                    || pac::RTC.cr2().read().0 != 0
                    || pac::RTC.compcfr1().read().0 != 0
                    || pac::RTC.ier().read().0 != 0
                    || pac::RTC.isr().read().0 != 0
                {
                    return Err(Error::LseClockInUse);
                }
                // This quiet image does NOT define RTC1HZ=0 or prove no RTC_OUT observer.
                Ok(true)
            })();
            if l012_sysclk && rtc.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            result
        })
        .map_err(inspection_error)?;
    if l012_sysclk && rtc.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    let rtc_lse = rtc_lse?;
    for (info, uart) in [
        (peripherals::UART1::RCC_INFO, pac::UART1),
        (peripherals::UART2::RCC_INFO, pac::UART2),
        #[cfg(rcc_cw32l011_v1)]
        (peripherals::UART3::RCC_INFO, pac::UART3),
    ] {
        if l012_sysclk && info.reset_asserted() {
            return Err(Error::LseClockInUse);
        }
        let conflict = info
            .inspect_for_init(cs, config.poll_budget, || {
                if l012_sysclk && info.reset_asserted() {
                    return Err(Error::LseClockInUse);
                }
                let conflict = u8::from(uart.cr1().read().source()) == crate::RCC_LSE_UART_SOURCE;
                if l012_sysclk && info.reset_asserted() {
                    return Err(Error::LseClockInUse);
                }
                Ok(conflict)
            })
            .map_err(inspection_error)?;
        if l012_sysclk && info.reset_asserted() {
            return Err(Error::LseClockInUse);
        }
        if conflict? {
            return Err(Error::LseClockInUse);
        }
    }
    #[cfg(rcc_cw32l012_v1)]
    {
        // UART3's gate meaning conflicts between current native manuals.
        // A closed gate stays closed; its unverified owner is a functional
        // handover condition, never inferred absent from the gate bit.
        let info = peripherals::UART3::RCC_INFO;
        if l012_sysclk {
            if info.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            if info.is_enabled() {
                if info.reset_asserted() || !info.is_enabled() {
                    return Err(Error::LseClockInUse);
                }
                let conflict =
                    u8::from(pac::UART3.cr1().read().source()) == crate::RCC_LSE_UART_SOURCE;
                if info.reset_asserted() || !info.is_enabled() || conflict {
                    return Err(Error::LseClockInUse);
                }
            }
            if info.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
        } else if info.is_enabled()
            && !info.reset_asserted()
            && u8::from(pac::UART3.cr1().read().source()) == crate::RCC_LSE_UART_SOURCE
        {
            return Err(Error::LseClockInUse);
        }
        for (info, i2c) in [
            (peripherals::I2C1::RCC_INFO, pac::I2C1),
            (peripherals::I2C2::RCC_INFO, pac::I2C2),
        ] {
            if info.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            let conflict = info
                .inspect_for_init(cs, config.poll_budget, || {
                    if l012_sysclk && info.reset_asserted() {
                        return Err(Error::LseClockInUse);
                    }
                    // Both master and slave select their own timing clock. Code1
                    // is reserved; code3's HSI/LSI disagreement is not interpreted.
                    let conflict = matches!(u8::from(i2c.mcr0().read().clksrc()), 1 | 2)
                        || matches!(u8::from(i2c.scr0().read().clksrc()), 1 | 2);
                    if l012_sysclk && info.reset_asserted() {
                        return Err(Error::LseClockInUse);
                    }
                    Ok(conflict)
                })
                .map_err(inspection_error)?;
            if l012_sysclk && info.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            if conflict? {
                return Err(Error::LseClockInUse);
            }
        }
    }
    let lptim = peripherals::LPTIM::RCC_INFO;
    if l012_sysclk && lptim.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    let conflict = lptim
        .inspect_for_init(cs, config.poll_budget, || {
            if l012_sysclk && lptim.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            #[cfg(not(rcc_cw32l012_v1))]
            let cr = pac::LPTIM.cr().read();
            #[cfg(rcc_cw32l012_v1)]
            let cr = pac::LPTIM.cr0().read();
            let cfgr = pac::LPTIM.cfgr().read();
            let conflict = cr.en()
                && (cfgr.iclksrc() == pac::lptim::vals::Source::Lse
                    || (rtc_lse && u8::from(cfgr.trigen()) != 0 && rtc_trigger(cfgr.trigsel())));
            if l012_sysclk && lptim.reset_asserted() {
                return Err(Error::LseClockInUse);
            }
            Ok(conflict)
        })
        .map_err(inspection_error)?;
    if l012_sysclk && lptim.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    if conflict? {
        return Err(Error::LseClockInUse);
    }
    Ok(rtc_lse)
}

pub(crate) fn preflight(
    config: Lse,
    cs: critical_section::CriticalSection<'_>,
    l012_sysclk: bool,
) -> Result<Admission, Error> {
    let r = pac::SYSCTRL;
    let reused = r.cr1().read().lseen();
    if reused {
        #[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
        if r.lse().read().pinlock() {
            return Err(Error::LseClockInUse);
        }
        if !ready(config, false) {
            return Err(Error::LseNotReady);
        }
        if oscillator_gpio().reset_asserted() {
            return Err(Error::LsePinConflict);
        }
    } else {
        central_new(config)?;
    }
    let admission = Admission {
        reused,
        lsi: config.monitored().then(lsi_parameters),
        lse: r.lse().read().0,
        routes: r.cr2().read().0,
        interrupts: r.ier().read().0,
    };
    if !reused {
        configuration_consumers(config, cs, l012_sysclk)?;
    }
    Ok(admission)
}

#[cfg(rcc_cw32l010_v1)]
fn poll(mut ready: impl FnMut() -> bool, attempts: u32, error: Error) -> Result<(), Error> {
    for _ in 0..attempts {
        if ready() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(error)
}

#[cfg(rcc_cw32l010_v1)]
/// GPIOB's gate controls the WHOLE BANK's operation, not just configuration.
/// The public functional handover allows sampling/filter/edge progress even on
/// failure. Preserve unrelated pin controls/flags; never open a timer gate.
fn pads(
    config: Lse,
    unused: bool,
    rtc_lse: bool,
    configure: bool,
    l012_sysclk: bool,
) -> Result<(), Error> {
    let _ = l012_sysclk;
    if oscillator_gpio().reset_asserted() {
        return Err(Error::LsePinConflict);
    }
    let was_enabled = pac::SYSCTRL.ahben().read().gpiob();
    let set_gate = |enabled| {
        pac::SYSCTRL.ahben().modify(|w| {
            w.set_key(0x5a5a);
            w.set_gpiob(enabled);
        });
        poll(
            || pac::SYSCTRL.ahben().read().gpiob() == enabled,
            config.poll_budget,
            Error::LseGpioGateTimeout,
        )
    };
    if !was_enabled {
        if let Err(error) = set_gate(true) {
            // Best-effort bounded restoration; never report it succeeded on a timeout.
            set_gate(false)?;
            return Err(error);
        }
    }
    let result = (|| {
        let r = pac::GPIOB;
        let dir = r.dir().read();
        let analog = r.analog().read();
        let af = r.afrl().read();
        // Only inside this already-permitted operation window, and before any
        // PB01 change. This does not retrospectively prove gate opening inert.
        if rtc_lse
            && ((!analog.pin4() && !dir.pin4() && af.afr4() == 2)
                || (!analog.pin6() && !dir.pin6() && af.afr6() == 2))
        {
            return Err(Error::LseClockInUse);
        }
        let pin_mask = if config.bypass() { 2 } else { 3 };
        if !dir.pin1()
            || analog.pin1() != (unused || !config.bypass())
            || af.afr1() != 0
            || (!config.bypass() && (!dir.pin0() || !analog.pin0() || af.afr0() != 0))
            || (r.opendrain().read().0
                | r.pur().read().0
                | r.riseie().read().0
                | r.fallie().read().0
                | r.filter().read().0)
                & pin_mask
                != 0
        {
            return Err(Error::LsePinConflict);
        }
        if configure && config.bypass() {
            r.analog().modify(|w| w.set_pin1(false));
            if r.analog().read().pin1() {
                return Err(Error::LsePinConflict);
            }
        }
        Ok(())
    })();
    if !was_enabled {
        set_gate(false)?;
    }
    result
}

// Observe only already operational output banks. Closed-bank and downstream
// timer/external recipients remain explicit, unverified functional handovers.
#[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
fn output_routes_idle(rtc_lse: bool, l012_sysclk: bool) -> bool {
    if rtc_lse
        && peripherals::GPIOA::RCC_INFO.is_enabled()
        && (l012_sysclk || !peripherals::GPIOA::RCC_INFO.reset_asserted())
    {
        if l012_sysclk
            && (peripherals::GPIOA::RCC_INFO.reset_asserted()
                || !peripherals::GPIOA::RCC_INFO.is_enabled())
        {
            return false;
        }
        let r = pac::GPIOA;
        let analog = r.analog().read();
        let dir = r.dir().read();
        let af = r.afrl().read();

        #[cfg(rcc_cw32l011_v1)]
        let (af1, af3) = (af.afr1(), af.afr3());
        #[cfg(rcc_cw32l012_v1)]
        let (af1, af3) = (af.pin1(), af.pin3());
        if l012_sysclk
            && (peripherals::GPIOA::RCC_INFO.reset_asserted()
                || !peripherals::GPIOA::RCC_INFO.is_enabled())
        {
            return false;
        }

        if (!analog.pin1() && !dir.pin1() && af1 == 3)
            || (!analog.pin3() && !dir.pin3() && af3 == 3)
        {
            return false;
        }
    }
    #[cfg(rcc_cw32l012_v1)]
    {
        if peripherals::GPIOB::RCC_INFO.is_enabled()
            && (l012_sysclk || !peripherals::GPIOB::RCC_INFO.reset_asserted())
        {
            if l012_sysclk
                && (peripherals::GPIOB::RCC_INFO.reset_asserted()
                    || !peripherals::GPIOB::RCC_INFO.is_enabled())
            {
                return false;
            }
            let r = pac::GPIOB;
            let analog = r.analog().read();
            let dir = r.dir().read();
            let af = r.afrh().read();
            if l012_sysclk
                && (peripherals::GPIOB::RCC_INFO.reset_asserted()
                    || !peripherals::GPIOB::RCC_INFO.is_enabled())
            {
                return false;
            }

            if (!analog.pin12() && !dir.pin12() && af.pin12() == 4)
                || (rtc_lse
                    && ((!analog.pin14() && !dir.pin14() && af.pin14() == 4)
                        || (!analog.pin15() && !dir.pin15() && af.pin15() == 4)))
            {
                return false;
            }
        }
        if peripherals::GPIOF::RCC_INFO.is_enabled()
            && (l012_sysclk || !peripherals::GPIOF::RCC_INFO.reset_asserted())
        {
            if l012_sysclk
                && (peripherals::GPIOF::RCC_INFO.reset_asserted()
                    || !peripherals::GPIOF::RCC_INFO.is_enabled())
            {
                return false;
            }
            let r = pac::GPIOF;
            let analog = r.analog().read();
            let dir = r.dir().read();
            let af = r.afrl().read();
            if l012_sysclk
                && (peripherals::GPIOF::RCC_INFO.reset_asserted()
                    || !peripherals::GPIOF::RCC_INFO.is_enabled())
            {
                return false;
            }

            if (!analog.pin1() && !dir.pin1() && af.pin1() == 3)
                || (!analog.pin3() && !dir.pin3() && af.pin3() == 1)
            {
                return false;
            }
        }
    }
    true
}

/// GPIOC inspection resumes the entire bank's sampling/filter/event operation.
/// Central RCC restores the exact incoming gate even on rejection; restoration
/// does not undo events. No timer or other output bank is opened to inspect it.
#[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
fn pads(
    config: Lse,
    unused: bool,
    rtc_lse: bool,
    configure: bool,
    l012_sysclk: bool,
) -> Result<(), Error> {
    let info = peripherals::GPIOC::RCC_INFO;
    if info.reset_asserted() {
        return Err(Error::LsePinConflict);
    }
    if unused && !output_routes_idle(rtc_lse, l012_sysclk) {
        return Err(Error::LseClockInUse);
    }
    let result = critical_section::with(|cs| {
        info.inspect_for_init(cs, config.poll_budget, || {
            if l012_sysclk && info.reset_asserted() {
                return Err(Error::LsePinConflict);
            }
            let result = (|| {
                let r = pac::GPIOC;
                let dir = r.dir().read();
                let analog = r.analog().read();
                let af = r.afrh().read();
                #[cfg(rcc_cw32l011_v1)]
                let (af14, af15) = (af.afr14(), af.afr15());
                #[cfg(rcc_cw32l012_v1)]
                let (af14, af15) = (af.pin14(), af.pin15());
                #[cfg(rcc_cw32l012_v1)]
                if rtc_lse && !analog.pin13() && !dir.pin13() && af.pin13() == 4 {
                    return Err(Error::LseClockInUse);
                }
                let pin_mask = if config.bypass() {
                    1 << 14
                } else {
                    (1 << 14) | (1 << 15)
                };
                if !dir.pin14()
                    || analog.pin14() != (unused || !config.bypass())
                    || af14 != 0
                    || (!config.bypass() && (!dir.pin15() || !analog.pin15() || af15 != 0))
                    || (r.opendrain().read().0
                        | r.pur().read().0
                        | r.riseie().read().0
                        | r.fallie().read().0
                        | r.filter().read().0)
                        & pin_mask
                        != 0
                {
                    return Err(Error::LsePinConflict);
                }
                if configure && config.bypass() {
                    r.analog().modify(|w| w.set_pin14(false));
                    if r.analog().read().pin14() {
                        return Err(Error::LsePinConflict);
                    }
                }
                Ok(())
            })();
            if l012_sysclk && info.reset_asserted() {
                return Err(Error::LsePinConflict);
            }
            result
        })
    })
    .map_err(|_| Error::LseGpioGateTimeout)?;
    if l012_sysclk && info.reset_asserted() {
        return Err(Error::LsePinConflict);
    }
    result
}

pub(crate) fn verify(
    config: Lse,
    _cs: critical_section::CriticalSection<'_>,
    l012_sysclk: bool,
) -> Result<(), Error> {
    if !healthy(config) {
        return Err(Error::LseNotReady);
    }
    pads(config, false, false, false, l012_sysclk)?;
    if !healthy(config) {
        return Err(Error::LseNotReady);
    }
    Ok(())
}

pub(crate) fn start(
    config: Lse,
    admission: Admission,
    cs: critical_section::CriticalSection<'_>,
    l012_sysclk: bool,
) -> Result<(), Error> {
    let r = pac::SYSCTRL;
    if r.cr2().read().0 != admission.routes
        || r.ier().read().0 != admission.interrupts
        || r.lse().read().0 != admission.lse
    {
        return Err(Error::LseClockInUse);
    }
    if let Some(parameters) = admission.lsi {
        if !monitor_qualified() || lsi_parameters() != parameters {
            return Err(Error::LseMonitorNotReady);
        }
        if MONITOR_LSI
            .borrow(cs)
            .get()
            .is_some_and(|old| old != parameters)
        {
            return Err(Error::LseMonitorNotReady);
        }
        MONITOR_LSI.borrow(cs).set(Some(parameters));
    }
    if admission.reused {
        return verify(config, cs, l012_sysclk);
    }
    central_new(config)?;
    let rtc_lse = configuration_consumers(config, cs, l012_sysclk)?;
    central_new(config)?;
    pads(config, true, rtc_lse, true, l012_sysclk)?;
    central_new(config)?;
    #[cfg(any(rcc_cw32l011_v1, rcc_cw32l012_v1))]
    {
        // GPIOC working-clock progress may expose a retained event. Recheck
        // the admitted consumers and frozen source/routes at the commit edge.
        if configuration_consumers(config, cs, l012_sysclk)? != rtc_lse
            || r.cr2().read().0 != admission.routes
            || r.ier().read().0 != admission.interrupts
            || r.lse().read().0 != admission.lse
        {
            return Err(Error::LseClockInUse);
        }
        if admission
            .lsi
            .is_some_and(|parameters| lsi_parameters() != parameters)
            || (config.monitored() && !monitor_qualified())
        {
            return Err(Error::LseMonitorNotReady);
        }
        central_new(config)?;
    }
    r.lse().modify(|w| {
        w.set_mode(config.bypass());
        w.set_driver(config.drive);
        w.set_pdriver(config.startup_drive);
        w.set_waitcycle(config.wait);
    });
    if !parameters_match(config) {
        return Err(Error::LseNotReady);
    }
    r.cr1().modify(|w| {
        w.set_key(0x5a5a);
        w.set_lseccs(config.monitored());
        w.set_lseen(true);
    });
    for _ in 0..config.poll_budget {
        if faults() {
            return Err(Error::LseNotReady);
        }
        if healthy(config) {
            if r.cr2().read().0 != admission.routes || r.ier().read().0 != admission.interrupts {
                return Err(Error::LseClockInUse);
            }
            return verify(config, cs, l012_sysclk);
        }
        core::hint::spin_loop();
    }
    Err(Error::LseNotReady)
}

pub(crate) fn preserved_register(old: u32) -> bool {
    // Only requested parameters and read-only initial STABLE may differ.
    (pac::SYSCTRL.lse().read().0 ^ old) & !crate::RCC_LSE_CHANGE_MASK == 0
}
