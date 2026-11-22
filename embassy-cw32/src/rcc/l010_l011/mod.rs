//! CW32L010/CW32L011 one-time factory-HSI and qualified direct-HSE clocks.
//! Exact L010 and L011 packages additionally permit initialization-only reuse of
//! already software-enabled, stable, factory-matching LSI; see
//! docs/l010-live-factory-lsi-sysclk.md and docs/l011-live-factory-lsi-sysclk.md.
//! The live target never starts or programs LSI.
//! Exact qualified L010/L011 packages also support native init-only LSE SYSCLK;
//! see docs/l010-lse-sysclk.md, docs/l011-lse-sysclk.md and [`crate::init`].
//!
//! Own source qualification, electrical limits and retained-owner rules:
//! docs/qualified-l010-l011-hse.md. The incoming source, buses, Flash latency,
//! HSI trim and unchanged LSI must already be legal for the declared board.
//! The initializer cannot repair an illegally clocked bootloader retroactively.
//! Other bus-dependent peripherals, DMA and application interrupt handlers must
//! be quiescent. ADC and SYSCLK/PCLK-filtered analog users are explicitly checked.
//! No oscillator frequency or board condition is measured. External loss can
//! halt execution when fallback is disabled; a fallback invalidates frozen
//! external rates. A failed init publishes no clocks and requires reset before
//! retry. PLL, runtime switching, sleep/resume and recovery are outside this API.

use crate::{pac, time::Hertz};
use core::cell::Cell;
use critical_section::Mutex;
use pac::sysctrl::vals::Sysclk as ClockSource;

mod configure;
mod inspection;
mod io;
#[cfg(rcc_lse)]
mod lse_sysclk;
#[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
mod lsi_sysclk;

use configure::configure;
#[cfg(rcc_lse)]
use inspection::lse_target_inspection_error;
use inspection::retained_inspection_error;
use io::{
    barrier, check_external_faults, hse_parameters_match, poll, poll_clock, set_flash_latency,
};
#[cfg(rcc_lse)]
use lse_sysclk::{LseSysclkSnapshot, finish_lse_sysclk};
#[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
use lsi_sysclk::{LsiSysclkSnapshot, finish_lsi_sysclk};
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
    /// Divide by 11 (L010 only).
    #[cfg(cw32l010)]
    Div11 = 11,
    /// Divide by 12.
    #[cfg(cw32l010)]
    Div12 = 12,
    /// Divide by 12.
    #[cfg(cw32l011)]
    Div12 = 11,
    /// Divide by 13 (L010 only).
    #[cfg(cw32l010)]
    Div13 = 13,
    /// Divide by 14 (L010 only).
    #[cfg(cw32l010)]
    Div14 = 14,
    /// Divide by 15 (L010 only).
    #[cfg(cw32l010)]
    Div15 = 15,
    /// Divide by 16.
    #[cfg(cw32l010)]
    Div16 = 0,
    /// Divide by 16.
    #[cfg(cw32l011)]
    Div16 = 12,
    /// Divide by 20 (L011 only).
    #[cfg(cw32l011)]
    Div20 = 13,
    /// Divide by 24 (L011 only).
    #[cfg(cw32l011)]
    Div24 = 14,
    /// Divide by 28 (L011 only).
    #[cfg(cw32l011)]
    Div28 = 15,
    /// Divide by 32 (L011 only).
    #[cfg(cw32l011)]
    Div32 = 0,
}
impl HsiDiv {
    /// Numeric division factor (not the register encoding).
    pub const fn divisor(self) -> u32 {
        #[cfg(cw32l010)]
        {
            if self as u8 == 0 { 16 } else { self as u32 }
        }
        #[cfg(cw32l011)]
        {
            [32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16, 20, 24, 28][self as usize]
        }
    }
}

const DEFAULT_DIV: HsiDiv = crate::RCC_DEFAULT_HSI_DIV;
const MAX_FLASH_WAIT: u32 = crate::RCC_INITIAL_FLASH_WAIT;

/// HSI configuration. HSI is always enabled in the supported clock tree.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hsi {
    /// Divider applied to the factory-calibrated family HSIOSC clock (48/96 MHz).
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

/// Divider from HCLK to PCLK. L010/L011 have one PCLK domain.
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
    /// Reuse live factory-matching LSI on CW32L010F8P6/F8U6/Y8M6
    /// and CW32L011K8T6/K8U6 only.
    /// Requires LSIEN and STABLE at original entry and preserves TRIM/WAIT.
    /// Ordinary POR and `Config::lse = Some(_)` are unsupported. Bounds are
    /// rate-only; IWDT may continue counting and reset during initialization.
    /// No clock-continuity, watchdog deadline or per-cycle guarantee is made.
    /// L011 also requires no configured/enabled HSE and no HSE/LSE detector;
    /// entry SYSCLK must be HSI or LSI. Retained raw-HSI RTC needs PSC1 >= 97.
    #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
    LSI,
    /// Native LSE on CW32L010F8P6/F8U6/Y8M6 and CW32L011K8T6/K8U6.
    /// Requires [`Config::lse`]; generic-family and L012 targets are excluded.
    ///
    /// HSI remains factory calibrated and enabled. StartupOnly can stop the CPU
    /// on later source loss without a fault or error return. MonitoredExistingRoutes
    /// requires an inherited stable, legal unchanged LSI; L011 additionally
    /// requires its own factory-matching TRIM at entry. Existing IRQ/brake routes
    /// are preserved and may affect observers before an init error. Inherited
    /// CLKCCS is preserved; documented HSI4MHz fallback invalidates frozen timings
    /// and does not guarantee progress or register/divider preservation.
    ///
    /// The existing HSI calibration path may temporarily request unchanged legal
    /// LSI; StartupOnly does not require factory LSI. The handover must permit
    /// residual timer/GPIO/IWDT/external progress and L011 PB0 AF3 raw-HSI output
    /// interruptions; concrete vetoes do not establish universal idleness. See
    /// [`crate::init`], docs/l010-lse-sysclk.md and
    /// docs/l011-lse-sysclk.md. All waits
    /// count CPU iterations while execution continues. Errors publish no clocks,
    /// retain source/pad ownership and require reset before retry; ordinary reset
    /// may retain LSE. The fixed 1 MHz time driver cannot use this system tree.
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
        if c.supply_mv < crate::RCC_HSE_SUPPLY_MV.0
            || c.supply_mv > crate::RCC_HSE_SUPPLY_MV.1
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
    /// Native board-qualified L010/L011 LSE. The exact qualified three L010 and
    /// two L011 packages may select it as SYSCLK.
    /// See [`crate::init`] for the required RTC_OUT/
    /// RTC_1Hz observer disconnection and whole-GPIOB (L010) or GPIOC (L011) handover.
    /// Genuine reset entry with untouched selectors meets those functional
    /// conditions; a register image does not prove reset. Failure permanently
    /// reserves requested pads and may leave the oscillator/gates enabled.
    #[cfg(rcc_lse)]
    pub lse: Option<super::Lse>,
    /// Requested system source. Factory HSI remains enabled.
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
    /// Reset-equivalent nominal 4 MHz clock tree.
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
        // New-target exclusions precede even requested external-source bounds.
        #[cfg(all(rcc_cw32l011_v1, rcc_lsi_sysclk))]
        if self.sys == Sysclk::LSI {
            if self.hse.is_some() {
                return Err(Error::UnsupportedLsiHseConfiguration);
            }
            if self.lse.is_some() {
                return Err(Error::UnsupportedLsiLseConfiguration);
            }
        }
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
        #[cfg(all(rcc_cw32l010_v1, rcc_lsi_sysclk))]
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
            #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
            Sysclk::LSI => crate::rcc::ClockBounds::lsi(),
            #[cfg(rcc_lse)]
            Sysclk::LSE => lse.ok_or(Error::LseNotConfigured)?.1,
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
        // Validate the requested retained HSI as a possible system source too.
        crate::rcc::operating::validate(
            self.operating_conditions,
            Clocks {
                source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                ..clocks
            },
        )?;
        // Both own families document effective HSI4MHz fallback, without guaranteeing
        // retained bus-divider fields. Give neither HCLK nor PCLK divisor credit.
        #[cfg(rcc_lse)]
        if self.sys == Sysclk::LSE {
            crate::rcc::operating::validate(
                self.operating_conditions,
                Clocks {
                    source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                    dividers: [self.hsi.div.divisor(), 1, 1],
                    ..clocks
                },
            )?;
        }
        #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
        if self.sys == Sysclk::LSI {
            crate::rcc::operating::validate(
                self.operating_conditions,
                Clocks {
                    source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                    dividers: [self.hsi.div.divisor(), 1, 1],
                    ..clocks
                },
            )?;
        }
        // The own-source fixed CCS divisor is independent of the startup default.
        crate::rcc::operating::validate(
            self.operating_conditions,
            Clocks {
                source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                ..clocks
            },
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
    /// Frozen source declaration and policy, conditional on continuing health.
    #[cfg(rcc_lse)]
    pub lse: Option<(super::Lse, super::ClockBounds)>,
}

/// A clock initialization failure.
///
/// Hardware errors may leave a partially changed clock tree. The initializer
/// never lowers flash latency before the requested clocks have been verified.
/// After a failure, no frequencies are published and peripherals must not be
/// used on the assumption that the requested configuration took effect.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// This L011 live-LSI slice cannot configure an auxiliary HSE.
    #[cfg(all(rcc_cw32l011_v1, rcc_lsi_sysclk))]
    UnsupportedLsiHseConfiguration,
    /// A live-LSI initialization cannot newly configure an auxiliary LSE.
    #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
    UnsupportedLsiLseConfiguration,
    /// Own factory halfword is erased or the entry LSI trim does not match it.
    #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
    InvalidLsiCalibration,
    /// Entry software request/readiness is absent, or live readiness was lost.
    #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
    LsiNotReady,
    /// Borrowed configuration-gate enable/restore failed, with both diagnostics.
    #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
    LsiConfigurationGateTimeout {
        enable_failed: bool,
        restore_failed: bool,
    },
    /// Final LSI selector did not acknowledge within the iteration budget.
    #[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l011_v1), rcc_lsi_sysclk))]
    LsiClockSwitchTimeout,
    #[cfg(rcc_lse)]
    InvalidLseBounds,
    #[cfg(rcc_lse)]
    LseClockInUse,
    #[cfg(rcc_lse)]
    LsePinConflict,
    #[cfg(rcc_lse)]
    LseGpioGateTimeout,
    /// An LSE or target monitor/bridge inspection configuration gate failed to
    /// open and/or return to its entry state. Both fields may be true; no
    /// successful restoration is then promised.
    #[cfg(rcc_lse)]
    LseConfigurationGateTimeout {
        enable_failed: bool,
        restore_failed: bool,
    },
    #[cfg(rcc_lse)]
    LseMonitorNotReady,
    #[cfg(rcc_lse)]
    LseNotReady,
    /// The LSE system target requires a board-qualified LSE declaration.
    #[cfg(rcc_lse)]
    LseNotConfigured,
    /// A first request for entry-nonstable LSI has an inherited consumer/IRQ owner.
    /// L011 live-LSI also rejects either inherited external-clock detector.
    #[cfg(rcc_lse)]
    LsiClockInUse,
    /// Starting/restarting or awaiting HSI conflicts with raw-HSI MCO or HSIRDY.
    #[cfg(rcc_lse)]
    HsiClockInUse,
    /// LSE selection did not acknowledge within the CPU-iteration budget.
    #[cfg(rcc_lse)]
    LseClockSwitchTimeout,
    InvalidHseBounds,
    HseOutsideQualifiedRange,
    HseConditionsOutsideQualifiedRange,
    HseConditionsDoNotCoverBoard,
    HseNotConfigured,
    HsePinsUnavailable,
    InvalidHseDetector,
    InvalidHseConfiguration,
    HseClockInUse,
    HsePinInUse,
    HsePinConfigurationTimeout,
    HseStopTimeout,
    HseTimeout,
    ExternalClockFault,
    InvalidEntryClock,
    RetainedClockInspectionTimeout,
    RetainedRtcConfiguration,
    AdcClockInUse,
    LvdClockInUse,
    ComparatorClockInUse,
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
    /// Factory HSI calibration storage reads as erased (0xffff).
    InvalidCalibration,
    /// An HSIOSC-selected RTC requires the current enabled, stable factory trim.
    /// No oscillator is changed on this error; the RTC bus gate is restored.
    /// L011 live-LSI also rejects RTC reset/lost access, HSE selection, and
    /// raw-HSI PSC1 divisors below 98, even when calendar/AWT is idle.
    RtcClockInUse,
    /// The RTC configuration bus gate did not acknowledge enable.
    RtcClockGateTimeout,
    /// The FLASH configuration clock could not be enabled.
    /// L011 live-LSI also uses this for asserted reset or lost Flash access.
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
    /// Requested source, retained configuration or bus dividers did not read back.
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
