//! CW32L012 one-time factory-HSI and qualified direct-HSE/LSE clocks.
//!
//! Own-source qualification and retained-owner rules: docs/qualified-l012-hse.md.
//! HSI defaults to /12 (8 MHz); the distinct hardware CCS fallback is /24 (4 MHz).
//! Incoming oscillator frequencies, buses and Flash latency must already be legal.
//! Other bus-dependent peripherals, DMA and application interrupts must be quiescent.
//! No source frequency or board condition is measured. STABLE is a startup latch;
//! later source loss can halt execution or invalidate frozen clocks after fallback.
//! Poll limits count CPU iterations while execution continues, not elapsed time.
//! Failure publishes no clocks and requires reset before retrying. No PLL exists;
//! runtime retuning, automatic HEXEN, sleep/resume and recovery are not provided.
//! Common RCC captures inherited LSE ownership before this backend runs.

use crate::{pac, time::Hertz};
use core::cell::Cell;
use critical_section::Mutex;
use pac::sysctrl::vals::Sysclk as ClockSource;

mod configure;
mod inspection;
mod io;
#[cfg(rcc_lse)]
mod lse_sysclk;
#[cfg(rcc_lsi_sysclk)]
mod lsi_sysclk;

use configure::configure;
#[cfg(rcc_lse)]
pub(crate) use inspection::lse_target_inspection_error;

/// Nominal calibrated internal high-speed oscillator frequency.
pub const HSI_FREQ: Hertz = Hertz(crate::RCC_HSI_FREQUENCY_HZ);

/// Documented divider between HSIOSC and HSI. Values are hardware encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum HsiDiv {
    /// Divide by 1.
    Div1 = 1,
    /// Divide by 2.
    Div2 = 2,
    /// Divide by 3.
    Div3 = 3,
    /// Divide by 4.
    Div4 = 4,
    /// Divide by 5.
    Div5 = 5,
    /// Divide by 6.
    Div6 = 6,
    /// Divide by 7.
    Div7 = 7,
    /// Divide by 8.
    Div8 = 8,
    /// Divide by 9.
    Div9 = 9,
    /// Divide by 10.
    Div10 = 10,
    /// Divide by 12.
    Div12 = 11,
    /// Divide by 16.
    Div16 = 12,
    /// Divide by 20.
    Div20 = 13,
    /// Divide by 24.
    Div24 = 14,
    /// Divide by 28.
    Div28 = 15,
    /// Divide by 32.
    Div32 = 0,
}
impl HsiDiv {
    /// Numeric division factor (not the register encoding).
    pub const fn divisor(self) -> u32 {
        match self {
            Self::Div1 => 1,
            Self::Div2 => 2,
            Self::Div3 => 3,
            Self::Div4 => 4,
            Self::Div5 => 5,
            Self::Div6 => 6,
            Self::Div7 => 7,
            Self::Div8 => 8,
            Self::Div9 => 9,
            Self::Div10 => 10,
            Self::Div12 => 12,
            Self::Div16 => 16,
            Self::Div20 => 20,
            Self::Div24 => 24,
            Self::Div28 => 28,
            Self::Div32 => 32,
        }
    }
}

const DEFAULT_DIV: HsiDiv = crate::RCC_DEFAULT_HSI_DIV;
const MAX_FLASH_WAIT: u32 = crate::RCC_INITIAL_FLASH_WAIT;

/// HSI configuration. HSI is always enabled in the supported clock tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hsi {
    /// Divider applied to the factory-calibrated 96 MHz HSIOSC clock.
    pub div: HsiDiv,
}
impl Default for Hsi {
    fn default() -> Self {
        Self { div: DEFAULT_DIV }
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

/// Divider from HCLK to PCLK. L012 has one PCLK domain.
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
    /// Already requested, stable, own factory-calibrated 32,800 Hz LSI.
    /// Only qualified C8T6/C8U6 live handovers are supported; POR is refused.
    #[cfg(rcc_lsi_sysclk)]
    LSI,
    /// Native board-qualified 32768 Hz source on C8T6/C8U6.
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
    /// Crystal drive setting for both pre-start and run phases.
    /// Ignored electrically in bypass mode.
    pub drive: HseDrive,
}
impl Hse {
    fn bounds(self) -> Result<crate::rcc::ClockBounds, Error> {
        if self.drive.to_bits() > HseDrive::Level7.to_bits() {
            return Err(Error::InvalidHseConfiguration);
        }
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
        if c.min_supply_mv < crate::RCC_HSE_SUPPLY_MV.0
            || c.max_supply_mv > crate::RCC_HSE_SUPPLY_MV.1
            || c.min_temperature_c < crate::RCC_HSE_TEMPERATURE_C.0
            || c.max_temperature_c > crate::RCC_HSE_TEMPERATURE_C.1
        {
            return Err(Error::HseConditionsOutsideQualifiedRange);
        }
        Ok(bounds)
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
    /// HSI oscillator divider.
    pub hsi: Hsi,
    /// Board-qualified HSE; its oscillator pads are reserved even with HSI SYSCLK.
    pub hse: Option<Hse>,
    /// Native board-qualified LSE, independent of system-clock selection.
    /// Requires the downstream RTC/LSE-output, UART3 and whole-GPIOC
    /// functional handover documented by [`crate::init`]. Failure retains
    /// requested pads and may retain the source request.
    #[cfg(rcc_lse)]
    pub lse: Option<super::Lse>,
    /// Requested system source. Factory HSI remains enabled.
    pub sys: Sysclk,
    /// HCLK prescaler.
    pub ahb_pre: AHBPrescaler,
    /// PCLK prescaler.
    pub apb_pre: APBPrescaler,
    /// Permit a nominal HSI bus rate above the worst-case oscillator bound.
    ///
    /// This is intended for calibrated motor-control boards that explicitly
    /// accept the factory HSI tolerance at the selected supply voltage. The
    /// default remains conservative and rejects that configuration.
    pub allow_nominal_bus_overclock: bool,
    /// Maximum register polls for each hardware wait. Must be nonzero.
    ///
    /// This is an iteration budget, not a duration: the entry clock may be
    /// unknown. No timer or Embassy time driver is required during init.
    pub timeout: u32,
}
impl Config {
    /// Default nominal 8 MHz clock tree. Silicon reset is 4 MHz (HSI /24).
    pub const fn new() -> Self {
        Self {
            operating_conditions: crate::rcc::OperatingConditions::new(),
            hsi: Hsi { div: DEFAULT_DIV },
            hse: None,
            #[cfg(rcc_lse)]
            lse: None,
            sys: Sysclk::HSI,
            ahb_pre: AHBPrescaler::Div1,
            apb_pre: APBPrescaler::Div1,
            allow_nominal_bus_overclock: false,
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
        let hsi = HSI_FREQ / self.hsi.div.divisor();
        let hse = if let Some(hse) = self.hse {
            let bounds = hse.bounds()?;
            let board = self.operating_conditions;
            let source = hse.operating_conditions;
            if board.min_supply_mv < source.min_supply_mv
                || board.max_supply_mv > source.max_supply_mv
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
        #[cfg(all(rcc_lsi_sysclk, rcc_lse))]
        if self.sys == Sysclk::LSI && self.lse.is_some() {
            return Err(Error::UnsupportedLsiLseConfiguration);
        }
        #[cfg(rcc_lse)]
        let lse = self
            .lse
            .map(|c| {
                c.bounds(self.operating_conditions)
                    .map(|bounds| (c, bounds))
            })
            .transpose()?;
        let source = match self.sys {
            Sysclk::HSI => crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
            Sysclk::HSE => hse.ok_or(Error::HseNotConfigured)?,
            #[cfg(rcc_lsi_sysclk)]
            Sysclk::LSI => crate::rcc::ClockBounds::lsi(),
            #[cfg(rcc_lse)]
            Sysclk::LSE => {
                let (config, bounds) = lse.ok_or(Error::LseNotConfigured)?;
                // Consume the independently qualified SYSCLK detector receipt,
                // in addition to native auxiliary source qualification above.
                if config.monitored()
                    && u64::from(config.min_freq.0) * u64::from(crate::RCC_LSE_SYSCLK_LSI_CYCLES)
                        <= (u64::from(crate::RCC_LSE_SYSCLK_LSE_EDGES)
                            + u64::from(crate::RCC_LSE_SYSCLK_MARGIN_LSE_EDGES))
                            * u64::from(crate::RCC_LSE_MONITORED_LSI_MAXIMUM_HZ)
                {
                    return Err(Error::InvalidLseBounds);
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
        crate::rcc::operating::validate_with_bus_policy(
            self.operating_conditions,
            clocks,
            self.allow_nominal_bus_overclock,
        )?;
        // Validate the requested retained HSI as a possible system source too.
        crate::rcc::operating::validate_with_bus_policy(
            self.operating_conditions,
            Clocks {
                source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                ..clocks
            },
            self.allow_nominal_bus_overclock,
        )?;
        // LSE target admission takes no bus-divider retention credit for the
        // full effective HSI/24 fallback. Other source contracts stay unchanged.
        let fallback_dividers = clocks.dividers;
        #[cfg(rcc_lse)]
        let fallback_dividers = if self.sys == Sysclk::LSE {
            [self.hsi.div.divisor(), 1, 1]
        } else {
            fallback_dividers
        };
        #[cfg(rcc_lsi_sysclk)]
        let fallback_dividers = if self.sys == Sysclk::LSI {
            [self.hsi.div.divisor(), 1, 1]
        } else {
            fallback_dividers
        };
        // Own L012 hardware forces /24 after CCS, independently of the default /12.
        crate::rcc::operating::validate_with_bus_policy(
            self.operating_conditions,
            Clocks {
                source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                dividers: fallback_dividers,
                ..clocks
            },
            self.allow_nominal_bus_overclock,
        )?;
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
    /// APB clock before peripheral-local prescalers and muxes.
    pub pclk: Hertz,
    // Exact cumulative divisors; nominal display rounding must not weaken bounds.
    pub(crate) dividers: [u32; 3],
    pub(crate) source: crate::rcc::ClockBounds,
    hse: Option<HseMode>,
    /// Frozen source declaration, conditional on continuing health.
    #[cfg(rcc_lse)]
    pub lse: Option<(super::Lse, super::ClockBounds)>,
}

/// A clock initialization failure.
///
/// Hardware errors may leave a partially changed clock tree. The initializer
/// lowers flash latency only after verifying that the active clock, final bus
/// dividers, retained source and qualified fallback are safe at that latency.
/// After a failure, no frequencies are published and peripherals must not be
/// used on the assumption that the requested configuration took effect.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[cfg(rcc_lsi_sysclk)]
    UnsupportedLsiLseConfiguration,
    #[cfg(rcc_lsi_sysclk)]
    InvalidLsiCalibration,
    #[cfg(rcc_lsi_sysclk)]
    LsiNotReady,
    /// A borrowed gate could not open and/or independently restore.
    #[cfg(rcc_lsi_sysclk)]
    LsiConfigurationGateTimeout {
        enable_failed: bool,
        restore_failed: bool,
    },
    #[cfg(rcc_lsi_sysclk)]
    LsiClockSwitchTimeout,
    #[cfg(rcc_lse)]
    InvalidLseBounds,
    #[cfg(rcc_lse)]
    LseNotConfigured,
    #[cfg(rcc_lse)]
    LseClockSwitchTimeout,
    #[cfg(any(rcc_lse, rcc_lsi_sysclk))]
    HsiClockInUse,
    #[cfg(rcc_lse)]
    LsiClockInUse,
    #[cfg(rcc_lse)]
    LseClockInUse,
    #[cfg(rcc_lse)]
    LsePinConflict,
    #[cfg(rcc_lse)]
    LseGpioGateTimeout,
    /// A configuration gate failed to open and/or return to its entry state.
    /// Both fields may be true; no successful restoration is then promised.
    #[cfg(rcc_lse)]
    LseConfigurationGateTimeout {
        enable_failed: bool,
        restore_failed: bool,
    },
    #[cfg(rcc_lse)]
    LseMonitorNotReady,
    #[cfg(rcc_lse)]
    LseNotReady,
    InvalidHseBounds,
    HseOutsideQualifiedRange,
    HseConditionsOutsideQualifiedRange,
    HseConditionsDoNotCoverBoard,
    HseNotConfigured,
    HsePinsUnavailable,
    InvalidHseDetector,
    InvalidHseConfiguration,
    HseClockInUse,
    HsePinConfigurationTimeout,
    /// Enabled inherited HSE needs an explicit, exactly matching declaration.
    InheritedHseEnabled,
    HseTimeout,
    ExternalClockFault,
    InvalidEntryClock,
    RetainedClockInspectionTimeout,
    RetainedRtcConfiguration,
    AdcClockInUse,
    LvdClockInUse,
    ComparatorClockInUse,
    OpaClockInUse,
    DacClockInUse,
    /// The selected timer cannot divide this exact nominal PCLK to 1 MHz.
    #[cfg(feature = "_time-driver")]
    UnsupportedTimeDriverClock,
    /// The reserved timer clock/reset did not acknowledge initialization.
    #[cfg(feature = "_time-driver")]
    TimeDriverClockFailure,
    /// Declared minimum VDD exceeds the declared maximum.
    InvalidSupplyRange,
    /// Declared minimum ambient TA exceeds the declared maximum.
    InvalidTemperatureRange,
    /// Declared supply interval extends outside the factory-HSI qualification.
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
    /// Factory HSI calibration storage reads as erased (0xffff).
    InvalidCalibration,
    /// An HSIOSC-selected RTC requires the current enabled, stable factory trim.
    /// No oscillator is changed on this error; the RTC bus gate is restored.
    RtcClockInUse,
    /// The RTC configuration bus gate did not acknowledge enable.
    RtcClockGateTimeout,
    /// The FLASH configuration clock could not be enabled.
    FlashClockTimeout,
    /// FLASH WAIT did not read back as requested.
    FlashLatencyTimeout,
    /// The temporary LSI source did not report stable within the poll budget.
    LsiTimeout,
    /// The system clock did not switch to LSI before stopping HSI.
    TemporaryClockSwitchTimeout,
    /// HSI enable or stable status did not clear before calibration.
    HsiStopTimeout,
    /// HSI did not report stable within the poll budget.
    HsiTimeout,
    /// Temporary LSI/clock-security state did not restore after HSI handover.
    TemporaryClockRestoreTimeout,
    /// The system clock selector did not read back as HSI.
    ClockSwitchTimeout,
    /// The requested HSI configuration or bus dividers did not read back.
    ClockConfigurationTimeout,
}

/// Private policy selector for the existing generated L012 HSE helpers.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum L012HseTarget {
    Legacy,
    #[cfg(rcc_lse)]
    LseSysclk,
    #[cfg(rcc_lsi_sysclk)]
    LiveLsiSysclk,
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

