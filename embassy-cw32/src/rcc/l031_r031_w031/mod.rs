//! Qualified HSI, direct HSE and exact-package LSE clocks on CW32L031/R031.
//! Factory-LSI SYSCLK is separately qualified for L031C8T6/C8U6/F8U6, and R031C8U6.
//! Generic aliases and other packages remain outside that qualification.
//!
//! Own sources: L031 RM CN1.6, R031 RM CN1.3 and W031 RM CN1.4
//! §§4.3–4.7 and 7.4, plus each own datasheet electrical tables.
//! See docs/qualified-l031-hse.md for source pages and board obligations.
//! Existing CCS controls are preserved. A requested new LSE start enables only
//! its detector; this monitored-source admission rule is a software policy.
//! There is no MCU system PLL, FLASH prefetch or FLASH cache on this register version.
//! R031 RF has a separate 16 MHz clock and synthesizer; PCLK/GPIOA host activity
//! remains within the existing functional handover. No RF state is read or written.
//! The default remains factory HSI /6. HSIOSC stays alive for independent users.
//! PLL configuration, runtime switching and low-power operation are unsupported.

use crate::{pac, time::Hertz};
use core::cell::Cell;
use critical_section::Mutex;
use pac::sysctrl::vals::Sysclk as ClockSource;

mod configure;
mod io;
#[cfg(rcc_lse)]
mod lse_sysclk;
#[cfg(rcc_lsi_sysclk)]
mod lsi_sysclk;

use configure::configure;
use io::{barrier, check_external_faults, hse_parameters_match, set_flash_latency, wait_until};
#[cfg(rcc_lse)]
use lse_sysclk::{prepare_lse_monitor, LseSysclkState, LSE_SYSCLK_MONITOR};
#[cfg(rcc_lsi_sysclk)]
use lsi_sysclk::LsiSysclkState;
#[cfg(rcc_lsi_sysclk)]
pub(crate) use lsi_sysclk::lsi_inspection_error;

/// Nominal calibrated internal high-speed oscillator frequency.
pub const HSI_FREQ: Hertz = Hertz(crate::RCC_HSI_FREQUENCY_HZ);

/// Documented divider between HSIOSC and HSI. Values are hardware encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum HsiDiv {
    /// Divide by 1 (48 MHz).
    Div1 = 0b0110,
    /// Divide by 2 (24 MHz).
    Div2 = 0b1000,
    /// Divide by 4 (12 MHz).
    Div4 = 0b1001,
    /// Divide by 6 (8 MHz, the reset divider).
    Div6 = 0b0101,
    /// Divide by 8 (6 MHz).
    Div8 = 0b1011,
    /// Divide by 10 (4.8 MHz).
    Div10 = 0b1100,
    /// Divide by 12 (4 MHz).
    Div12 = 0b1101,
    /// Divide by 14 (approximately 3.429 MHz).
    Div14 = 0b1110,
    /// Divide by 16 (3 MHz).
    Div16 = 0b1111,
}
impl HsiDiv {
    /// Numeric division factor (not the register encoding).
    pub const fn divisor(self) -> u32 {
        match self {
            Self::Div1 => 1,
            Self::Div2 => 2,
            Self::Div4 => 4,
            Self::Div6 => 6,
            Self::Div8 => 8,
            Self::Div10 => 10,
            Self::Div12 => 12,
            Self::Div14 => 14,
            Self::Div16 => 16,
        }
    }
}

/// HSI configuration. HSI is always enabled in the supported clock tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hsi {
    /// Divider applied to the factory-calibrated 48 MHz HSIOSC clock.
    pub div: HsiDiv,
}
impl Default for Hsi {
    fn default() -> Self {
        Self {
            div: crate::RCC_DEFAULT_HSI_DIV,
        }
    }
}

/// Divider from SysClk to HCLK.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum AHBPrescaler {
    /// Divide by 1.
    Div1 = 0,
    /// Divide by 2.
    Div2 = 1,
    /// Divide by 4.
    Div4 = 2,
    /// Divide by 8.
    Div8 = 3,
    /// Divide by 16.
    Div16 = 4,
    /// Divide by 32.
    Div32 = 5,
    /// Divide by 64.
    Div64 = 6,
    /// Divide by 128.
    Div128 = 7,
}
impl AHBPrescaler {
    /// Numeric division factor.
    pub const fn divisor(self) -> u32 {
        1 << self as u8
    }
}

/// Divider from HCLK to PCLK. Supported backends have one PCLK domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum APBPrescaler {
    /// Divide by 1.
    Div1 = 0,
    /// Divide by 2.
    Div2 = 1,
    /// Divide by 4.
    Div4 = 2,
    /// Divide by 8.
    Div8 = 3,
}
impl APBPrescaler {
    /// Numeric division factor.
    pub const fn divisor(self) -> u32 {
        1 << self as u8
    }
}

/// Direct system-clock sources implemented by this backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysclk {
    /// Factory-calibrated HSI through its configured divider.
    HSI,
    /// Qualified external high-speed oscillator or input.
    HSE,
    /// Factory-trimmed 32,800 Hz LSI on L031C8T6/C8U6/F8U6, R031C8U6.
    /// R031 requires 2.2..3.6 V, at -40..85 C.
    /// Rate bounds do not certify individual-cycle or jitter timing. Cold entry
    /// excludes every reviewed direct client and ready observer, even when the
    /// existing trim matches; live factory-matching clients remain untouched.
    #[cfg(rcc_lsi_sysclk)]
    LSI,
    /// The single board-qualified nominal 32768 Hz `Config.lse` source.
    /// Init-only, with a permanently retained factory-LSI detector reference.
    #[cfg(rcc_lse)]
    LSE,
}

/// HSE electrical mode. The board must reserve the actual oscillator pads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HseMode {
    /// Crystal or ceramic resonator on OSC_IN and OSC_OUT.
    Oscillator,
    /// Externally driven digital clock on OSC_IN; OSC_OUT remains available.
    Bypass,
}

pub use pac::sysctrl::vals::HseDrive;

/// Board-qualified external clock, independent of factory HSI accuracy.
///
/// All three frequencies are required. Bounds include oscillator tolerance,
/// temperature, aging, load, short-term cycle variation and external oscillator
/// supply effects throughout
/// `operating_conditions`. The MCU cannot measure or enforce that declaration.
/// Bypass also requires the own datasheet's input voltage, pulse width, edge and
/// duty constraints. In oscillator mode the board must qualify drive/load/startup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hse {
    /// Nominal source frequency; never an electrical upper-bound substitute.
    pub freq: Hertz,
    /// Guaranteed minimum actual frequency, in whole hertz.
    pub min_freq: Hertz,
    /// Guaranteed maximum actual frequency, in whole hertz.
    pub max_freq: Hertz,
    /// MCU supply and ambient interval over which the source bounds are valid.
    pub operating_conditions: crate::rcc::OperatingConditions,
    pub mode: HseMode,
    /// Crystal drive setting. Ignored electrically in bypass mode.
    pub drive: HseDrive,
}
impl Hse {
    fn bounds(self) -> Result<crate::rcc::ClockBounds, Error> {
        let range = match self.mode {
            HseMode::Oscillator => crate::RCC_HSE_CRYSTAL_RANGE_HZ,
            HseMode::Bypass => crate::RCC_HSE_BYPASS_RANGE_HZ,
        };
        let bounds = crate::rcc::ClockBounds::external(
            self.freq,
            self.min_freq,
            self.max_freq,
            self.operating_conditions,
        )
        .ok_or(Error::InvalidHseBounds)?;
        if self.min_freq.0 < range.0 || self.max_freq.0 > range.1 {
            return Err(Error::HseOutsideQualifiedRange);
        }
        let c = self.operating_conditions;
        if c.supply_mv < crate::RCC_HSE_SUPPLY_MV.0
            || c.supply_mv > crate::RCC_HSE_SUPPLY_MV.1
            || c.min_temperature_c < crate::RCC_HSE_TEMPERATURE_C.0
            || c.max_temperature_c > crate::RCC_HSE_TEMPERATURE_C.1
        {
            return Err(Error::HseConditionsOutsideQualifiedRange);
        }
        Ok(bounds)
    }

    fn range(self) -> pac::sysctrl::vals::HseRange {
        // The manual bins are explicitly nominal. At shared endpoints select
        // the upper bin; actual electrical endpoints are checked independently.
        let index = crate::RCC_HSE_FREQUENCY_RANGES_HZ
            .iter()
            .position(|(_, max)| self.freq.0 < *max)
            .unwrap_or(3);
        pac::sysctrl::vals::HseRange::from_bits(index as u8)
    }

    fn detector_count(self) -> Result<u16, Error> {
        let count = crate::RCC_HSE_CCS_NUMERATOR_HZ.div_ceil(u64::from(self.min_freq.0));
        // Check the count, and prove the minimum source can satisfy a complete
        // detector window even at the fastest documented legal unchanged LSI.
        if count == 0
            || count > u64::from(crate::RCC_HSE_CCS_MAXIMUM_COUNT)
            || u64::from(self.min_freq.0) * count
                <= u64::from(crate::RCC_HSE_CCS_CYCLE_COUNT)
                    * u64::from(crate::RCC_HSE_CCS_LSI_MAXIMUM_HZ)
        {
            return Err(Error::InvalidHseDetector);
        }
        Ok(count as u16)
    }
}

/// Clock tree initialization options, using Embassy's RCC configuration naming.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Board-declared supply and ambient TA envelope. No measurement is made.
    /// The full qualified range is the default; declare the actual narrower
    /// board range to permit faster operation where the datasheet allows it.
    pub operating_conditions: crate::rcc::OperatingConditions,
    /// HSI oscillator divider. HSI remains enabled for every system source.
    pub hsi: Hsi,
    /// Optional external source, configured and reserved even when SYSCLK is HSI.
    pub hse: Option<Hse>,
    /// Init-only board-qualified LSE. None preserves its inherited setup.
    #[cfg(rcc_lse)]
    pub lse: Option<super::Lse>,
    /// System clock source. HSE requires `hse`; LSE requires `lse`.
    pub sys: Sysclk,
    /// HCLK prescaler.
    pub ahb_pre: AHBPrescaler,
    /// PCLK prescaler.
    pub apb_pre: APBPrescaler,
    /// Maximum register polls for each hardware wait. Must be nonzero.
    ///
    /// This is an iteration budget, not a duration: the entry clock may be
    /// unknown. No timer or Embassy time driver is required during init.
    pub timeout: u32,
}
impl Config {
    /// Reset-equivalent nominal 8 MHz clock tree.
    pub const fn new() -> Self {
        Self {
            operating_conditions: crate::rcc::OperatingConditions::new(),
            hsi: Hsi {
                div: crate::RCC_DEFAULT_HSI_DIV,
            },
            hse: None,
            #[cfg(rcc_lse)]
            lse: None,
            sys: Sysclk::HSI,
            ahb_pre: AHBPrescaler::Div1,
            apb_pre: APBPrescaler::Div1,
            timeout: 100_000,
        }
    }

    /// Validate the board envelope and calculate clocks without touching hardware.
    ///
    /// Declared VDD/ambient ranges must be inside the selected family's source
    /// qualification. Actual upper HCLK/PCLK must fit the minimum-VDD bus limit.
    /// Nominal Hertz remains rounded down; the bound getters retain exact source
    /// division. A nominal rate equal to a rated ceiling may be rejected because
    /// its qualified positive HSI error crosses that ceiling. No voltage,
    /// temperature or frequency is measured. A stable, legal incoming clock and
    /// appropriate incoming FLASH latency remain initialization preconditions.
    pub fn frequencies(&self) -> Result<Clocks, Error> {
        if self.timeout == 0 {
            return Err(Error::InvalidTimeout);
        }
        #[cfg(rcc_lse)]
        let lse = self
            .lse
            .map(|c| c.bounds(self.operating_conditions).map(|b| (c, b)))
            .transpose()?;
        let hsi = HSI_FREQ / self.hsi.div.divisor();
        let hse = if let Some(hse) = self.hse {
            let bounds = hse.bounds()?;
            let board = self.operating_conditions;
            let source = hse.operating_conditions;
            if board.supply_mv != source.supply_mv
                || board.min_temperature_c < source.min_temperature_c
                || board.max_temperature_c > source.max_temperature_c
            {
                return Err(Error::HseConditionsDoNotCoverBoard);
            }
            if crate::RCC_HSE_PINS.0.is_none()
                || (hse.mode == HseMode::Oscillator && crate::RCC_HSE_PINS.1.is_none())
            {
                return Err(Error::HsePinsUnavailable);
            }
            hse.detector_count()?;
            Some(bounds)
        } else {
            None
        };
        let source = match self.sys {
            Sysclk::HSI => crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
            Sysclk::HSE => hse.ok_or(Error::HseNotConfigured)?,
            #[cfg(rcc_lsi_sysclk)]
            Sysclk::LSI => {
                let c = self.operating_conditions;
                if c.supply_mv < crate::RCC_LSI_SUPPLY_MV.0
                    || c.supply_mv > crate::RCC_LSI_SUPPLY_MV.1
                {
                    return Err(Error::SupplyOutsideQualifiedRange);
                }
                if c.min_temperature_c < crate::RCC_LSI_TEMPERATURE_C.0
                    || c.max_temperature_c > crate::RCC_LSI_TEMPERATURE_C.1
                {
                    return Err(Error::TemperatureOutsideQualifiedRange);
                }
                crate::rcc::ClockBounds::lsi()
            }
            #[cfg(rcc_lse)]
            Sysclk::LSE => {
                use crate::rtc::sealed::Instance;
                let (config, bounds) = lse.ok_or(Error::LseNotConfigured)?;
                // Own hardware counts and a separate software margin. Factory
                // LSI rate facts do not promise every-cycle/window timing.
                if u64::from(config.min_freq.0) * u64::from(crate::RCC_LSE_SYSCLK_LSI_CYCLES)
                    <= (u64::from(crate::RCC_LSE_SYSCLK_LSE_EDGES)
                        + u64::from(crate::RCC_LSE_SYSCLK_MARGIN_LSE_EDGES))
                        * u64::from(crate::peripherals::RTC::SOURCE_MAXIMUM_HZ)
                {
                    return Err(Error::InvalidLseDetector);
                }
                let c = self.operating_conditions;
                if c.supply_mv < crate::peripherals::RTC::SUPPLY_MV.0
                    || c.supply_mv > crate::peripherals::RTC::SUPPLY_MV.1
                    || c.min_temperature_c < crate::peripherals::RTC::TEMPERATURE_C.0
                    || c.max_temperature_c > crate::peripherals::RTC::TEMPERATURE_C.1
                {
                    return Err(Error::LseMonitorConditionsOutsideQualifiedRange);
                }
                bounds
            }
        };
        let clocks = Clocks {
            hsi,
            sys: source.nominal(),
            hclk: source.divided_by(self.ahb_pre.divisor()).nominal(),
            pclk: source
                .divided_by(self.ahb_pre.divisor() * self.apb_pre.divisor())
                .nominal(),
            dividers: [
                self.hsi.div.divisor(),
                self.ahb_pre.divisor(),
                self.ahb_pre.divisor() * self.apb_pre.divisor(),
            ],
            source,
            hse: self.hse.map(|hse| hse.mode),
            #[cfg(rcc_lse)]
            lse,
        };
        crate::rcc::operating::validate(self.operating_conditions, clocks)?;
        // A retained CCS policy may select HSI. Conservatively qualify HSI for
        // every declared HSE, even if incoming fault switching is disabled.
        // This is an admission restriction, not a must-enable CCS requirement.
        #[cfg(rcc_lse)]
        let retain_hsi = self.hse.is_some() || self.sys == Sysclk::LSE;
        #[cfg(not(rcc_lse))]
        let retain_hsi = self.hse.is_some();
        #[cfg(rcc_lsi_sysclk)]
        let retain_hsi = retain_hsi || self.sys == Sysclk::LSI;
        // LSE also executes configured HSI under the final buses before the
        // final mux write, regardless of the inherited system CCS policy.
        if retain_hsi {
            crate::rcc::operating::validate(
                self.operating_conditions,
                Clocks {
                    source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                    ..clocks
                },
            )?;
        }
        Ok(clocks)
    }
}
impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Nominal frequencies frozen after successful HAL initialization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Clocks {
    /// Divided HSI output, not the undivided HSIOSC frequency.
    pub hsi: Hertz,
    /// System clock mux output.
    pub sys: Hertz,
    /// CPU/AHB clock, including the flash interface.
    pub hclk: Hertz,
    /// APB clock; also the UART kernel clock when UART SOURCE is PCLK.
    pub pclk: Hertz,
    // Exact cumulative divisors; nominal display rounding must not weaken bounds.
    pub(crate) dividers: [u32; 3],
    pub(crate) source: crate::rcc::ClockBounds,
    hse: Option<HseMode>,
    #[cfg(rcc_lse)]
    pub(crate) lse: Option<(super::Lse, crate::rcc::ClockBounds)>,
}

/// A clock initialization failure.
///
/// Hardware errors may leave a partially changed clock tree. The initializer
/// never lowers flash latency before the requested clocks have been verified.
/// Calibration failures may leave the CPU on temporary LSI. Reset before retrying.
/// After a failure, no frequencies are published and peripherals must not be
/// used on the assumption that the requested configuration took effect.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Factory LSI calibration storage reads as erased before native masking.
    #[cfg(rcc_lsi_sysclk)]
    InvalidLsiCalibration,
    /// A requested or selected LSI uses a different trim and cannot be retuned.
    #[cfg(rcc_lsi_sysclk)]
    LsiCalibrationInUse,
    /// LSI ownership, observers or retained configuration are ambiguous.
    #[cfg(rcc_lsi_sysclk)]
    LsiClockInUse,
    #[cfg(rcc_lsi_sysclk)]
    LsiGateEnableTimeout,
    #[cfg(rcc_lsi_sysclk)]
    LsiGateRestoreTimeout,
    #[cfg(rcc_lsi_sysclk)]
    LsiConfigurationTimeout,
    /// The incoming selected source lacks both startup-ready observations.
    #[cfg(rcc_lsi_sysclk)]
    InvalidEntryClock,
    #[cfg(rcc_lse)]
    InvalidLseBounds,
    /// LSE SYSCLK requires the existing board source declaration.
    #[cfg(rcc_lse)]
    LseNotConfigured,
    /// The new system target does not satisfy its software detector margin.
    #[cfg(rcc_lse)]
    InvalidLseDetector,
    /// Board conditions do not qualify the own factory-LSI detector reference.
    #[cfg(rcc_lse)]
    LseMonitorConditionsOutsideQualifiedRange,
    #[cfg(rcc_lse)]
    LseNotReady,
    #[cfg(rcc_lse)]
    LsePinConflict,
    #[cfg(rcc_lse)]
    LseClockInUse,
    InvalidHseBounds,
    HseOutsideQualifiedRange,
    HseConditionsOutsideQualifiedRange,
    HseConditionsDoNotCoverBoard,
    HsePinsUnavailable,
    HseNotConfigured,
    InvalidHseDetector,
    HseTimeout,
    HseStopTimeout,
    HsePinConfigurationTimeout,
    /// Retained RTC/AWT ownership conflicts with the requested HSE setup.
    HseClockInUse,
    RetainedClockInspectionTimeout,
    /// Recalibrating HSI would interrupt a retained AWT or LVD filter.
    HsiClockInUse,
    /// The selected timer cannot divide this exact nominal PCLK to 1 MHz.
    #[cfg(feature = "_time-driver")]
    UnsupportedTimeDriverClock,
    /// The reserved timer clock/reset did not acknowledge initialization.
    #[cfg(feature = "_time-driver")]
    TimeDriverClockFailure,
    /// Declared minimum ambient TA exceeds the declared maximum.
    InvalidTemperatureRange,
    /// Declared supply voltage is outside the factory-HSI qualification.
    SupplyOutsideQualifiedRange,
    /// Declared ambient interval extends outside the factory-HSI qualification.
    TemperatureOutsideQualifiedRange,
    /// Worst-case actual HCLK exceeds the declared-voltage bus ceiling.
    HclkTooHigh,
    /// Worst-case actual PCLK exceeds the declared-voltage bus ceiling.
    PclkTooHigh,
    /// A polling budget of zero is invalid.
    InvalidTimeout,
    /// The clock tree has already been frozen by a successful initialization.
    AlreadyInitialized,
    /// The incoming system clock selector is reserved.
    InvalidClockSource,
    /// The incoming HSI divider is not documented by the supported manuals.
    InvalidHsiDivider,
    /// Factory HSI calibration storage reads as erased (0xffff).
    InvalidCalibration,
    /// The FLASH configuration clock could not be enabled.
    FlashClockTimeout,
    /// FLASH WAIT did not read back as requested.
    FlashLatencyTimeout,
    /// HSI did not report stable within the poll budget.
    HsiTimeout,
    /// Target or temporary LSI did not report stable within the polling budget.
    LsiTimeout,
    /// HSI did not disable and lose its STABLE indication before calibration.
    HsiStopTimeout,
    /// The system clock selector did not read back as the requested source.
    ClockSwitchTimeout,
    /// The temporary LSI software-enable state could not be restored.
    LsiRestoreTimeout,
    /// SYSCLK did not switch to LSI before stopping HSI.
    TemporaryClockSwitchTimeout,
    /// A relevant external source has a sticky startup-failure or loss flag.
    ExternalClockFault,
    /// Active AWT ETR routing may conflict with the HSE input pad.
    HsePinInUse,
    /// The requested HSI configuration or bus dividers did not read back.
    ClockConfigurationTimeout,
}

static CLOCKS: Mutex<Cell<Option<Clocks>>> = Mutex::new(Cell::new(None));

/// Get the frozen clock frequencies, or `None` before a successful HAL init.
pub fn try_clocks() -> Option<Clocks> {
    critical_section::with(|cs| CLOCKS.borrow(cs).get())
}

/// Get the frozen clock frequencies.
///
/// Panics before successful HAL initialization. Direct clock-register changes
/// after init invalidate these frequencies and are the caller's responsibility.
pub fn clocks() -> Clocks {
    try_clocks().expect("embassy-cw32 clocks are not initialized")
}

/// Configure once, before HAL drivers or application interrupts can use clocks.
///
/// Safety: the caller must own the device initialization sequence. No other
/// code, including DMA or NMI handlers, may change or depend on clock registers
/// during this call. Normal interrupts are masked for the complete sequence.
// The caller must enter from a clock/voltage operating point legal for the
// selected chip, and ensure the final HCLK/PCLK obey the board's VDD limits.
// At least /4 AHB and /8 APB keep the legal incoming sources and qualified
// HSI envelope below the low-voltage bus ceiling throughout the transition.
// Conservative initial Flash WAIT is retained until final readbacks succeed.
pub(crate) unsafe fn init(config: Config) -> Result<(), Error> {
    critical_section::with(|cs| {
        if CLOCKS.borrow(cs).get().is_some() {
            return Err(Error::AlreadyInitialized);
        }
        let clocks = configure(config, cs)?;
        CLOCKS.borrow(cs).set(Some(clocks));
        Ok(())
    })
}

pub(crate) fn hse_pin_reserved(pin: u8) -> bool {
    let Some(clocks) = try_clocks() else {
        return false;
    };
    clocks.hse.is_some_and(|mode| {
        crate::RCC_HSE_PINS.0 == Some(pin)
            || (mode == HseMode::Oscillator && crate::RCC_HSE_PINS.1 == Some(pin))
    })
}

#[cfg(rcc_lse)]
pub(super) fn lse_sysclk_monitor_ready() -> bool {
    critical_section::with(|cs| {
        LSE_SYSCLK_MONITOR
            .borrow(cs)
            .get()
            .is_none_or(|state| state.monitor_ready(true))
    })
}
