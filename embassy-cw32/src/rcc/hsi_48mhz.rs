//! Qualified HSI, factory LSI, direct HSE/LSE and HSI- or HSE-fed PLL on F020/F030/A030.
//!
//! Own sources: F020 RM CN1.4 and x030 RM CN2.5 §§4.3–4.7, own datasheet
//! external-clock/electrical tables. See docs/qualified-hse.md for conflicts,
//! board obligations, operation correspondence and verification limits.
//! The default remains factory HSI /6. HSIOSC stays alive for independent users.
//! PLL is admitted only as the one-time system source, with rate-only bounds.
//! Runtime switching, low-power restore and guaranteed fault recovery are
//! unsupported. See docs/f020-x030-hsi-pll.md for own-source limits.
//! Entry clocks/Flash must already be legal and stable. DMA, peripheral,
//! IRQ/NMI, MCO and dedicated PLL_OUT consumers must be quiescent. A failure
//! can leave partial state, including LSI execution or stopped HSI/PLL; reset
//! before retrying. Poll budgets are not wall-time guarantees.

use crate::{pac, time::Hertz};
use core::cell::Cell;
use critical_section::Mutex;
use pac::sysctrl::vals::Sysclk as ClockSource;

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

/// Source-qualified PLL multipliers, using the literal hardware field.
#[cfg(rcc_pll)]
pub use pac::sysctrl::vals::PllMul;

/// Qualified PLL reference. Only HSI uses the divider in `Config.hsi`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg(rcc_pll)]
pub enum PllSource {
    /// Factory-trimmed HSI after its configured divider.
    HSI,
    /// Undivided HSE from `Config.hse`, in oscillator or bypass mode. The existing
    /// HSE board and operating-condition contract and all PLL input/output limits
    /// apply. Oscillator mode follows the vendor-documented crystal-to-PLL path;
    /// admission does not independently certify internal reference duty. Bypass
    /// requires the specified OSC_IN waveform, including 40–60% duty. PLL bounds
    /// describe rates only; individual-cycle timing and recovery after reference
    /// loss are not guaranteed.
    HSE,
}

/// One-time HSI- or HSE-fed PLL configuration, selected as `Sysclk::PLL`.
///
/// Both actual input endpoints must fit one documented input bin, and both
/// actual multiplied output endpoints must fit one output bin and the separate
/// electrical limits of the selected family. See docs/f020-x030-hsi-pll.md
/// for admitted pairs, including the separate F020 output ceiling.
/// Other documented hardware settings are not qualified by this API.
/// The PLL supplies qualified clock-rate bounds, not absolute period/jitter bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg(rcc_pll)]
pub struct Pll {
    pub src: PllSource,
    pub mul: PllMul,
}
#[cfg(rcc_pll)]
struct PllParameters {
    source: pac::sysctrl::vals::PllSource,
    bounds: crate::rcc::ClockBounds,
    input_range: pac::sysctrl::vals::PllInputRange,
    output_range: pac::sysctrl::vals::PllOutputRange,
}
#[cfg(rcc_pll)]
impl Pll {
    fn parameters(self, config: &Config) -> Result<PllParameters, Error> {
        let c = config.operating_conditions;
        if c.supply_mv < crate::RCC_PLL_SUPPLY_MV.0
            || c.supply_mv > crate::RCC_PLL_SUPPLY_MV.1
            || c.min_temperature_c < crate::RCC_PLL_TEMPERATURE_C.0
            || c.max_temperature_c > crate::RCC_PLL_TEMPERATURE_C.1
        {
            return Err(Error::PllConditionsOutsideQualifiedRange);
        }
        let multiplier = self.mul.to_bits();
        if !(crate::RCC_PLL_MULTIPLIER_RANGE.0..=crate::RCC_PLL_MULTIPLIER_RANGE.1)
            .contains(&multiplier)
        {
            return Err(Error::InvalidPllMultiplier);
        }
        let (input, source) = match self.src {
            PllSource::HSI => (
                crate::rcc::ClockBounds::hsi(config.hsi.div.divisor()),
                pac::sysctrl::vals::PllSource::Hsi,
            ),
            PllSource::HSE => {
                let hse = config.hse.ok_or(Error::HseNotConfigured)?;
                let bounds = hse.bounds()?;
                if !crate::RCC_PLL_HSE_SUPPORTED {
                    return Err(Error::PllInputOutsideQualifiedRange);
                }
                let source = match hse.mode {
                    HseMode::Oscillator => pac::sysctrl::vals::PllSource::HseCrystal,
                    HseMode::Bypass => pac::sysctrl::vals::PllSource::HseBypass,
                };
                (bounds, source)
            }
        };
        if input.minimum_below(crate::RCC_PLL_INPUT_RANGE_HZ.0)
            || input.maximum_exceeds(crate::RCC_PLL_INPUT_RANGE_HZ.1)
        {
            return Err(Error::PllInputOutsideQualifiedRange);
        }
        let bounds = input
            .multiplied_by(u32::from(multiplier))
            .ok_or(Error::PllArithmeticOverflow)?;
        if bounds.minimum_below(crate::RCC_PLL_OUTPUT_RANGE_HZ.0)
            || bounds.maximum_exceeds(crate::RCC_PLL_OUTPUT_RANGE_HZ.1)
        {
            return Err(Error::PllOutputOutsideQualifiedRange);
        }
        // Choose the first fitting closed bin at a shared exact endpoint.
        // The complete source envelope must never straddle a bin boundary.
        let input_index = crate::RCC_PLL_INPUT_BINS_HZ
            .iter()
            .position(|(lo, hi)| !input.minimum_below(*lo) && !input.maximum_exceeds(*hi))
            .ok_or(Error::PllInputCrossesBin)?;
        let output_index = crate::RCC_PLL_OUTPUT_BINS_HZ
            .iter()
            .position(|(lo, hi)| !bounds.minimum_below(*lo) && !bounds.maximum_exceeds(*hi))
            .ok_or(Error::PllOutputCrossesBin)?;
        Ok(PllParameters {
            source,
            bounds,
            input_range: pac::sysctrl::vals::PllInputRange::from_bits(input_index as u8),
            output_range: pac::sysctrl::vals::PllOutputRange::from_bits(output_index as u8),
        })
    }
}

/// System-clock sources implemented by this backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysclk {
    /// Factory-calibrated HSI through its configured divider.
    HSI,
    /// Factory-qualified nominal 32,800 Hz, with rate bounds only.
    ///
    /// Initialization may briefly open each entire GPIOA/B/C/F bank to inspect
    /// retained LSI selectors. Sampling, filters and armed events can advance,
    /// including before an error; gate restoration cannot undo those effects.
    /// GPIO configuration and flags are preserved. Running factory-matching LSI
    /// is reused unchanged; a stopped source needs complete consumer admission.
    /// See docs/factory-lsi-sysclk.md for the entry and partial-failure contract.
    LSI,
    /// Board-qualified nominal 32768 Hz source from `Config.lse`.
    ///
    /// Available only on CW32F020C6U7, CW32F030C8T7 and CW32A030C8T7.
    /// Internally prepares/retains factory LSI for mandatory failure detection.
    /// The same whole-GPIO-bank functional handover as LSI applies on cold entry.
    /// HSI and an inherited PLL/reference remain enabled. No runtime switching,
    /// guaranteed source-loss recovery or fixed 1 MHz time driver is supported.
    /// See docs/classic-lse-sysclk.md for qualification and partial-error limits.
    #[cfg(rcc_lse)]
    LSE,
    /// Qualified external high-speed oscillator or input.
    HSE,
    /// HSI- or HSE-fed PLL, with qualified rate bounds only.
    #[cfg(rcc_pll)]
    PLL,
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
    /// HSI oscillator divider. HSI remains enabled, including with HSE SysClk.
    pub hsi: Hsi,
    /// Optional external source, configured and reserved even when SYSCLK is HSI.
    pub hse: Option<Hse>,
    /// Init-only PLL, permitted only when selected as the system source.
    #[cfg(rcc_pll)]
    pub pll: Option<Pll>,
    /// Init-only board-qualified LSE. None preserves LSE parameters and pads;
    /// the existing mandatory CLKCCS/HSECCS/LSECCS setup still applies.
    #[cfg(rcc_lse)]
    pub lse: Option<super::Lse>,
    /// System clock source. HSE/LSE/PLL require their corresponding configuration.
    /// LSI includes the documented whole-GPIO-bank inspection handover.
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
            #[cfg(rcc_pll)]
            pll: None,
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
        #[cfg(rcc_pll)]
        let pll = match (self.pll, self.sys) {
            (Some(pll), Sysclk::PLL) => Some(pll.parameters(self)?.bounds),
            (None, Sysclk::PLL) => return Err(Error::PllNotConfigured),
            (Some(_), _) => return Err(Error::PllNotSelected),
            (None, _) => None,
        };
        let source = match self.sys {
            Sysclk::HSI => crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
            Sysclk::LSI => {
                let c = self.operating_conditions;
                if c.supply_mv < crate::RCC_LSI_SUPPLY_MV.0
                    || c.supply_mv > crate::RCC_LSI_SUPPLY_MV.1
                    || c.min_temperature_c < crate::RCC_LSI_TEMPERATURE_C.0
                    || c.max_temperature_c > crate::RCC_LSI_TEMPERATURE_C.1
                {
                    return Err(Error::LsiConditionsOutsideQualifiedRange);
                }
                crate::rcc::ClockBounds::lsi()
            }
            #[cfg(rcc_lse)]
            Sysclk::LSE => {
                let (config, bounds) = lse.ok_or(Error::LseNotConfigured)?;
                // Target-only software count margin: the own manuals specify
                // 128 LSE edges per 256 LSI cycles. Demand an extra edge with
                // strict inequality against this family's factory rate maximum.
                // LSI rate bounds do not establish absolute per-cycle jitter or
                // a hard detector deadline. General auxiliary LSE is unchanged.
                if u64::from(config.min_freq.0) * u64::from(crate::RCC_LSE_SYSCLK_LSI_CYCLES)
                    <= (u64::from(crate::RCC_LSE_SYSCLK_LSE_EDGES)
                        + u64::from(crate::RCC_LSE_SYSCLK_MARGIN_LSE_EDGES))
                        * u64::from(crate::RCC_LSI_MAXIMUM_HZ)
                {
                    return Err(Error::InvalidLseDetector);
                }
                let c = self.operating_conditions;
                if c.supply_mv < crate::RCC_LSI_SUPPLY_MV.0
                    || c.supply_mv > crate::RCC_LSI_SUPPLY_MV.1
                    || c.min_temperature_c < crate::RCC_LSI_TEMPERATURE_C.0
                    || c.max_temperature_c > crate::RCC_LSI_TEMPERATURE_C.1
                {
                    return Err(Error::LsiConditionsOutsideQualifiedRange);
                }
                bounds
            }
            Sysclk::HSE => hse.ok_or(Error::HseNotConfigured)?,
            #[cfg(rcc_pll)]
            Sysclk::PLL => pll.ok_or(Error::PllNotConfigured)?,
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
            #[cfg(rcc_pll)]
            pll,
            #[cfg(rcc_lse)]
            lse,
        };
        crate::rcc::operating::validate(self.operating_conditions, clocks)?;
        // HSI remains available during the transition and after it. Qualify its
        // actual envelope under the final buses too; this does not promise PLL
        // fault recovery or a continuous lock-loss indication.
        if self.sys != Sysclk::HSI || self.hse.is_some() {
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
    #[cfg(rcc_pll)]
    pll: Option<crate::rcc::ClockBounds>,
    #[cfg(rcc_lse)]
    pub(crate) lse: Option<(super::Lse, crate::rcc::ClockBounds)>,
}

#[cfg(rcc_pll)]
impl Clocks {
    /// Raw PLL clock-rate bounds after successful initialization, before AHB/APB.
    /// A `Config::frequencies()` result is a prospective calculation only.
    pub fn pll_bounds(self) -> Option<crate::rcc::ClockBounds> {
        self.pll
    }
    /// Nominal raw PLL rate, rounded down. No independent PLL output is enabled.
    pub fn pll_frequency(self) -> Option<Hertz> {
        self.pll.map(|b| b.nominal())
    }
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
    LsiConditionsOutsideQualifiedRange,
    /// The public factory-LSI halfword is erased. Zero itself is valid.
    InvalidLsiCalibration,
    /// A running or starting LSI has a different trim; it was not retuned.
    LsiCalibrationInUse,
    /// A retained root, observer, reset or changing state prevents a new start.
    /// A use-edge recheck failure can leave the admitted TRIM, without enable.
    LsiClockInUse,
    /// Inspection gate enable failed; its original state was restored.
    /// On a post-trim recheck, the admitted TRIM may already remain.
    LsiGateEnableTimeout,
    /// An inspection gate could not be restored and may remain active.
    /// On a post-trim recheck, the admitted TRIM may already remain.
    LsiGateRestoreTimeout,
    /// Factory trim/readback failed; attempted trim may remain, without enable.
    LsiConfigurationTimeout,
    /// Retuning an inherited live PLL reference would interrupt its consumers.
    RetainedPllClockInUse,
    #[cfg(rcc_pll)]
    PllNotConfigured,
    #[cfg(rcc_pll)]
    PllNotSelected,
    #[cfg(rcc_pll)]
    InvalidPllMultiplier,
    #[cfg(rcc_pll)]
    PllArithmeticOverflow,
    #[cfg(rcc_pll)]
    PllConditionsOutsideQualifiedRange,
    #[cfg(rcc_pll)]
    PllInputOutsideQualifiedRange,
    #[cfg(rcc_pll)]
    PllOutputOutsideQualifiedRange,
    #[cfg(rcc_pll)]
    PllInputCrossesBin,
    #[cfg(rcc_pll)]
    PllOutputCrossesBin,
    /// A nondefault inherited reserved/debug value is not qualified for PLL use.
    #[cfg(rcc_pll)]
    PllReservedConfiguration,
    #[cfg(rcc_pll)]
    PllConfigurationTimeout,
    #[cfg(rcc_pll)]
    PllTimeout,
    /// A retained or requested external source has a relevant sticky fault.
    ExternalClockFault,
    /// LSE as SYSCLK requires the single `Config.lse` declaration.
    #[cfg(rcc_lse)]
    LseNotConfigured,
    /// The new LSE SYSCLK-only detector count margin is not met.
    #[cfg(rcc_lse)]
    InvalidLseDetector,
    #[cfg(rcc_lse)]
    InvalidLseBounds,
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
    /// FLASH WAIT/FETCH/CACHE did not read back as requested.
    FlashLatencyTimeout,
    /// HSI did not report stable within the poll budget.
    HsiTimeout,
    /// Requested or temporary LSI did not report stable within the polling budget.
    LsiTimeout,
    /// HSI did not disable and lose its STABLE indication before calibration.
    HsiStopTimeout,
    /// The system clock selector did not read back as the requested source.
    ClockSwitchTimeout,
    /// The temporary LSI software-enable state could not be restored.
    LsiRestoreTimeout,
    /// PLL did not stop within the poll budget.
    PllStopTimeout,
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
/// code, including DMA or exception handlers, may race clock-register writes
/// during this call. Retained consumers follow the documented functional
/// handover and source admission. Normal interrupts are masked throughout.
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
    let mode = clocks.hse.or_else(|| {
        pac::SYSCTRL.cr1().read().hseen().then(|| {
            if pac::SYSCTRL.hse().read().mode() {
                HseMode::Bypass
            } else {
                HseMode::Oscillator
            }
        })
    });
    mode.is_some_and(|mode| {
        crate::RCC_HSE_PINS.0 == Some(pin)
            || (mode == HseMode::Oscillator && crate::RCC_HSE_PINS.1 == Some(pin))
    })
}

fn barrier() {
    #[cfg(target_arch = "arm")]
    {
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }
}

fn wait_until(timeout: u32, error: Error, mut ready: impl FnMut() -> bool) -> Result<(), Error> {
    for _ in 0..timeout {
        if ready() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(error)
}

fn set_flash_latency(wait: u32, timeout: u32, preserve_features: bool) -> Result<(), Error> {
    let before = pac::FLASH.cr2().read();
    let fetch = preserve_features && before.fetch() || !preserve_features;
    let cache = preserve_features && before.cache() || !preserve_features;
    pac::FLASH.cr2().modify(|w| {
        w.set_key(0x5a5a);
        w.set_wait(wait as u8);
        // The new PLL path retains the inherited cache/prefetch settings.
        // Direct HSI/HSE keeps its existing enable-both initialization policy.
        w.set_fetch(fetch);
        w.set_cache(cache);
    });
    wait_until(timeout, Error::FlashLatencyTimeout, || {
        let r = pac::FLASH.cr2().read();
        u32::from(r.wait()) == wait && r.fetch() == fetch && r.cache() == cache
    })?;
    barrier();
    Ok(())
}

// A read-only snapshot of the stopped-source admission controls. This is current
// hardware evidence, not a claim to reconstruct reset history.
struct LsiColdSnapshot {
    cr0: u32,
    cr1: u32,
    hsi: u32,
    pll: u32,
}
impl LsiColdSnapshot {
    fn capture() -> Self {
        let r = pac::SYSCTRL;
        Self {
            cr0: r.cr0().read().0,
            cr1: r.cr1().read().0,
            hsi: r.hsi().read().0,
            pll: r.pll().read().0,
        }
    }
    fn check(&self, expected: pac::sysctrl::regs::Lsi) -> Result<(), Error> {
        let r = pac::SYSCTRL;
        let cr0 = r.cr0().read();
        let cr1 = r.cr1().read();
        let isr = r.isr().read();
        if cr0.0 != self.cr0
            || cr1.0 != self.cr1
            || r.hsi().read().0 != self.hsi
            || r.pll().read().0 != self.pll
            || r.lsi().read().0 != expected.0
            || !matches!(cr0.sysclk(), ClockSource::Hsi | ClockSource::Pll)
            || cr1.lsien()
            || isr.lsistable()
            || r.lsi().read().stable()
            || cr1.hseccs()
            || cr1.lseccs()
            || cr1.hseen()
            || cr1.lseen()
            || r.hse().read().stable()
            || r.lse().read().stable()
            || isr.hsestable()
            || isr.lsestable()
            || r.ier().read().lsirdy()
            || isr.lsirdy()
            || cortex_m::peripheral::NVIC::is_pending(pac::Interrupt::RCC)
            || (cr0.sysclk() == ClockSource::Pll
                && r.pll().read().source() != pac::sysctrl::vals::PllSource::Hsi)
        {
            return Err(Error::LsiClockInUse);
        }
        Ok(())
    }
}

// All callers use the existing central gate protocol, including its restoration
// attempt after a failed enable. A restoration failure may leave the bank active.
pub(crate) fn lsi_inspection_error(error: super::peripheral::ClockInspectionError) -> Error {
    match error {
        super::peripheral::ClockInspectionError::EnableFailed {
            restore_failed: false,
        } => Error::LsiGateEnableTimeout,
        super::peripheral::ClockInspectionError::EnableFailed {
            restore_failed: true,
        }
        | super::peripheral::ClockInspectionError::RestoreFailed => Error::LsiGateRestoreTimeout,
    }
}

fn lsi_parameters(value: pac::sysctrl::regs::Lsi) -> u32 {
    // The mask is generated from the selected PAC's read-only STABLE field.
    // WAIT and every reserved bit remain part of the exact comparison.
    value.0 & crate::RCC_LSI_PARAMETERS_MASK
}

fn prepare_factory_lsi(
    timeout: u32,
    cs: critical_section::CriticalSection<'_>,
) -> Result<pac::sysctrl::regs::Lsi, Error> {
    let r = pac::SYSCTRL;
    let factory =
        unsafe { core::ptr::read_volatile(crate::RCC_LSI_FACTORY_TRIM_ADDRESS as *const u16) };
    if factory == u16::MAX {
        return Err(Error::InvalidLsiCalibration);
    }
    let original = r.lsi().read();
    let mut expected = original;
    expected.set_trim(factory); // Typed ten-bit field; zero is a valid trim.
    let requested = r.cr1().read().lsien();
    let selected = r.cr0().read().sysclk() == ClockSource::Lsi;
    let mirror = r.isr().read().lsistable();
    if original.trim() != expected.trim() && (requested || selected || original.stable() || mirror)
    {
        return Err(Error::LsiCalibrationInUse);
    }
    if original.stable() != mirror {
        return Err(Error::LsiClockInUse);
    }
    if requested || selected {
        // Reuse without touching TRIM or WAIT, including an enabled matching
        // oscillator still starting. Selected LSI has hardware stop protection.
        wait_until(timeout, Error::LsiTimeout, || {
            let lsi = r.lsi().read();
            lsi_parameters(lsi) == lsi_parameters(original)
                && lsi.stable()
                && r.isr().read().lsistable()
                && (r.cr1().read().lsien() || r.cr0().read().sysclk() == ClockSource::Lsi)
        })?;
        return Ok(original);
    }
    if original.stable() || mirror {
        return Err(Error::LsiClockInUse);
    }
    let snapshot = LsiColdSnapshot::capture();
    let mut previous = None;
    for _ in 0..2 {
        snapshot.check(original)?;
        let consumers = crate::rcc_factory_lsi_consumers(timeout, cs)?;
        snapshot.check(original)?;
        if previous.is_some_and(|old| old != consumers) {
            return Err(Error::LsiClockInUse);
        }
        previous = Some(consumers);
    }
    // An accidental stopped factory match still requires both complete passes.
    snapshot.check(original)?;
    if original.trim() != expected.trim() {
        r.lsi().modify(|w| w.set_trim(expected.trim()));
    }
    wait_until(timeout, Error::LsiConfigurationTimeout, || {
        r.lsi().read().0 == expected.0 && !r.isr().read().lsistable()
    })?;
    // The use-edge snapshot advances only TRIM to the admitted factory value.
    // No source-request write has occurred. Reopening GPIO banks is the same
    // explicitly documented functional handover, including on a later failure.
    snapshot.check(expected)?;
    if Some(crate::rcc_factory_lsi_consumers(timeout, cs)?) != previous {
        return Err(Error::LsiClockInUse);
    }
    snapshot.check(expected)?;
    Ok(expected)
}

fn wait_factory_lsi(expected: pac::sysctrl::regs::Lsi, timeout: u32) -> Result<(), Error> {
    wait_until(timeout, Error::LsiTimeout, || {
        let r = pac::SYSCTRL;
        let lsi = r.lsi().read();
        r.cr1().read().lsien()
            && lsi.stable()
            && r.isr().read().lsistable()
            && lsi_parameters(lsi) == lsi_parameters(expected)
    })
}

fn modify_sources(target_lsi: bool, change: impl FnOnce(&mut pac::sysctrl::regs::Cr1)) {
    pac::SYSCTRL.cr1().modify(|w| {
        change(w);
        w.set_key(0x5a5a);
        w.set_clkccs(true);
        w.set_hseccs(true);
        w.set_lseccs(true);
        if target_lsi {
            w.set_lsien(true);
        }
    });
}

// Frozen only by a successful LSE system-source initialization. Old auxiliary
// LSE paths do not acquire this additional factory-reference obligation.
#[cfg(rcc_lse)]
static LSE_SYSCLK_LSI: Mutex<Cell<Option<u32>>> = Mutex::new(Cell::new(None));

#[cfg(rcc_lse)]
pub(super) fn lse_sysclk_monitor_ready() -> bool {
    critical_section::with(|cs| {
        let Some(expected) = LSE_SYSCLK_LSI.borrow(cs).get() else {
            return true;
        };
        let r = pac::SYSCTRL;
        let cr1 = r.cr1().read();
        let lsi = r.lsi().read();
        cr1.clkccs()
            && cr1.hseccs()
            && cr1.lseccs()
            && cr1.lsien()
            && lsi.stable()
            && r.isr().read().lsistable()
            && lsi_parameters(lsi) == expected
    })
}

fn configure(config: Config, cs: critical_section::CriticalSection<'_>) -> Result<Clocks, Error> {
    let clocks = config.frequencies()?;
    let target_lsi = config.sys == Sysclk::LSI;
    #[cfg(rcc_lse)]
    let target_lse = config.sys == Sysclk::LSE;
    #[cfg(not(rcc_lse))]
    let target_lse = false;
    let permanent_lsi = target_lsi || target_lse;
    #[cfg(rcc_lse)]
    let reuse_lse = config
        .lse
        .map(|c| super::lse::preflight(c, cs))
        .transpose()?;
    let r = pac::SYSCTRL;
    let old_clock = r.cr0().read();
    let old_sources = r.cr1().read();
    #[cfg(rcc_pll)]
    let old_lsi = r.lsi().read();
    #[cfg(rcc_pll)]
    let old_hse = r.hse().read();
    #[cfg(rcc_pll)]
    let old_lse = r.lse().read();
    #[cfg(rcc_pll)]
    if config.pll.is_some() {
        if r.pll().read().reserved_debug().to_bits() != crate::RCC_PLL_RESERVED_DEBUG_DEFAULT {
            return Err(Error::PllReservedConfiguration);
        }
        check_external_faults(old_sources.hseen(), old_sources.lseen())?;
        if old_sources.hseen() && !old_hse.stable() {
            return Err(Error::HseTimeout);
        }
        if old_sources.lseen() && !old_lse.stable() {
            return Err(Error::ClockConfigurationTimeout);
        }
    }
    let needs_lsi =
        target_lsi || config.hse.is_some() || old_sources.hseen() || old_sources.lseen();
    #[cfg(rcc_lse)]
    let needs_lsi = needs_lsi || config.lse.is_some();
    if !matches!(
        old_clock.sysclk(),
        ClockSource::Hsi
            | ClockSource::Hse
            | ClockSource::Pll
            | ClockSource::Lsi
            | ClockSource::Lse
    ) {
        return Err(Error::InvalidClockSource);
    }
    let old_hsi = r.hsi().read();
    if !matches!(old_hsi.div(), 5 | 6 | 8 | 9 | 11..=15) {
        return Err(Error::InvalidHsiDivider);
    }
    let factory =
        unsafe { core::ptr::read_volatile(crate::RCC_FACTORY_HSI_TRIM_ADDRESS as *const u16) };
    if factory == u16::MAX {
        return Err(Error::InvalidCalibration);
    }
    let mut calibrated = pac::sysctrl::regs::Hsi::default();
    calibrated.set_trim(factory);
    let trim = calibrated.trim();
    let needs_trim = old_hsi.trim() != trim;
    let old_pll = r.pll().read();
    let retain_pll = permanent_lsi
        && (old_sources.pllen() || old_pll.stable() || old_clock.sysclk() == ClockSource::Pll);
    if retain_pll {
        // This new source selection does not acquire or stop an inherited PLL.
        // Its reference stays unchanged, including the divided HSI source.
        if !old_sources.pllen()
            || !old_pll.stable()
            || (old_pll.source() == pac::sysctrl::vals::PllSource::Hsi
                && (needs_trim || old_hsi.div() != config.hsi.div as u8))
            || !matches!(
                old_pll.source(),
                pac::sysctrl::vals::PllSource::Hsi
                    | pac::sysctrl::vals::PllSource::HseCrystal
                    | pac::sysctrl::vals::PllSource::HseBypass
            )
        {
            return Err(Error::RetainedPllClockInUse);
        }
    }
    // APB gates control register access, not the independent AWT/RTC source.
    // Inspect with their original gates restored; never reset retained owners.
    let awt = if needs_trim || config.hse.is_some() {
        Some(
            <crate::peripherals::AWT as crate::rcc::SealedRccPeripheral>::RCC_INFO
                .inspect_for_init(cs, config.timeout, || pac::AWT.cr().read())
                .map_err(|_| Error::RetainedClockInspectionTimeout)?,
        )
    } else {
        None
    };
    if needs_trim && awt.is_some_and(|v| v.en() && v.src() == pac::awt::vals::Source::Hsiosc) {
        return Err(Error::HsiClockInUse);
    }
    if needs_trim
        && pac::LVD.cr0().read().en()
        && pac::LVD.cr1().read().flten()
        && pac::LVD.cr1().read().fltclk()
    {
        return Err(Error::HsiClockInUse);
    }
    let mut preserve_hse = false;
    if let Some(hse) = config.hse {
        let rtc_source = <crate::peripherals::RTC as crate::rcc::SealedRccPeripheral>::RCC_INFO
            .inspect_for_init(cs, config.timeout, || pac::RTC.cr1().read().source())
            .map_err(|_| Error::RetainedClockInspectionTimeout)?;
        let rtc_hse = matches!(
            rtc_source,
            pac::rtc::vals::Source::HseDiv128
                | pac::rtc::vals::Source::HseDiv256
                | pac::rtc::vals::Source::HseDiv512
                | pac::rtc::vals::Source::HseDiv1024
        );
        preserve_hse = rtc_hse
            || (retain_pll && old_pll.source() != pac::sysctrl::vals::PllSource::Hsi)
            || awt.is_some_and(|v| v.en() && v.src() == pac::awt::vals::Source::Hse);
        if preserve_hse
            && (!r.cr1().read().hseen()
                || !r.hse().read().stable()
                || !hse_parameters_match(hse)?
                || !crate::rcc_hse_pins_match(hse.mode == HseMode::Bypass, config.timeout, cs)?)
        {
            return Err(Error::HseClockInUse);
        }
    }

    // Establish conservative Flash and monotonic bus guards before increasing
    // any source. Preserve an inherited divider already stronger than /4,/8.
    <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO
        .enable_with_cs_readback(
            cs,
            crate::rcc::Readback::Poll {
                attempts: config.timeout,
                spin: true,
            },
        )
        .map_err(|_| Error::FlashClockTimeout)?;
    #[cfg(rcc_pll)]
    let preserve_flash_features = config.pll.is_some();
    #[cfg(not(rcc_pll))]
    let preserve_flash_features = false;
    set_flash_latency(
        crate::RCC_INITIAL_FLASH_WAIT,
        config.timeout,
        preserve_flash_features,
    )?;
    let flash_features = pac::FLASH.cr2().read();
    let guard_hclk = old_clock.hclkprs().max(AHBPrescaler::Div4 as u8);
    let guard_pclk = old_clock.pclkprs().max(APBPrescaler::Div8 as u8);
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_hclkprs(guard_hclk);
        w.set_pclkprs(guard_pclk);
    });
    wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
        let v = r.cr0().read();
        v.hclkprs() == guard_hclk && v.pclkprs() == guard_pclk
    })?;
    barrier();

    let factory_lsi = if permanent_lsi {
        Some(prepare_factory_lsi(config.timeout, cs)?)
    } else {
        None
    };
    modify_sources(permanent_lsi, |w| {
        w.set_key(0x5a5a);
        w.set_clkccs(true);
        w.set_hseccs(true);
        w.set_lseccs(true);
        w.set_hsien(true);
    });
    wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
        let v = r.cr1().read();
        v.hsien() && v.clkccs() && v.hseccs() && v.lseccs()
    })?;
    if let Some(expected) = factory_lsi {
        wait_factory_lsi(expected, config.timeout)?;
    }
    wait_until(config.timeout, Error::HsiTimeout, || {
        r.hsi().read().stable()
    })?;
    // Leave any PLL via unchanged HSI before stopping PLL or retuning HSI.
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(ClockSource::Hsi);
    });
    wait_until(config.timeout, Error::ClockSwitchTimeout, || {
        r.cr0().read().sysclk() == ClockSource::Hsi
    })?;
    barrier();
    if !permanent_lsi {
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_pllen(false);
        });
        wait_until(config.timeout, Error::PllStopTimeout, || {
            !r.cr1().read().pllen()
        })?;
        wait_until(config.timeout, Error::PllStopTimeout, || {
            !r.pll().read().stable()
        })?;
    }

    let lsi_was_enabled = r.cr1().read().lsien();
    // HSE requires unchanged, legal LSI for CCS even without a trim bridge.
    if needs_trim || needs_lsi {
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_lsien(true);
        });
        wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
            let v = r.cr1().read();
            v.lsien() && v.clkccs() && v.hseccs() && v.lseccs()
        })?;
        wait_until(config.timeout, Error::LsiTimeout, || {
            r.lsi().read().stable()
        })?;
    }
    if needs_trim {
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(ClockSource::Lsi);
        });
        wait_until(config.timeout, Error::ClockSwitchTimeout, || {
            r.cr0().read().sysclk() == ClockSource::Lsi
        })?;
        barrier();
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_hsien(false);
        });
        wait_until(config.timeout, Error::HsiStopTimeout, || {
            !r.cr1().read().hsien()
        })?;
        wait_until(config.timeout, Error::HsiStopTimeout, || {
            !r.hsi().read().stable()
        })?;
        r.hsi().modify(|w| w.set_trim(trim));
        wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
            r.hsi().read().trim() == trim
        })?;
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_hsien(true);
        });
        wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
            let v = r.cr1().read();
            v.hsien() && v.clkccs() && v.hseccs() && v.lseccs()
        })?;
        wait_until(config.timeout, Error::HsiTimeout, || {
            r.hsi().read().stable()
        })?;
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(ClockSource::Hsi);
        });
        wait_until(config.timeout, Error::ClockSwitchTimeout, || {
            r.cr0().read().sysclk() == ClockSource::Hsi
        })?;
        barrier();
        if !lsi_was_enabled && !needs_lsi {
            modify_sources(permanent_lsi, |w| {
                w.set_key(0x5a5a);
                w.set_lsien(false);
            });
            wait_until(config.timeout, Error::LsiRestoreTimeout, || {
                !r.cr1().read().lsien()
            })?;
        }
    }
    r.hsi().modify(|w| w.set_div(config.hsi.div as u8));
    wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
        r.hsi().read().div() == config.hsi.div as u8
    })?;
    wait_until(config.timeout, Error::HsiTimeout, || {
        r.hsi().read().stable()
    })?;

    #[cfg(rcc_lse)]
    if target_lse {
        // Any independently requested external source starts only after the
        // persistent factory detector reference and HSI intermediate are ready.
        wait_factory_lsi(factory_lsi.ok_or(Error::LsiClockInUse)?, config.timeout)?;
        if r.cr0().read().sysclk() != ClockSource::Hsi {
            return Err(Error::ClockSwitchTimeout);
        }
    }
    if let Some(hse) = config.hse.filter(|_| !preserve_hse) {
        // Even an already-ready source is stopped before pin or parameter
        // changes: STABLE alone is not continuous oscillator validity.
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_hseen(false);
        });
        wait_until(config.timeout, Error::HseStopTimeout, || {
            !r.cr1().read().hseen()
        })?;
        wait_until(config.timeout, Error::HseStopTimeout, || {
            !r.hse().read().stable()
        })?;
        crate::rcc_configure_hse_pins(hse.mode == HseMode::Bypass, config.timeout, cs)?;
        let detector = hse.detector_count()?;
        r.hse().modify(|w| {
            w.set_mode(hse.mode == HseMode::Bypass);
            w.set_driver(hse.drive);
            #[cfg(rcc_cw32f020_v1)]
            w.set_freq(hse.range());
            #[cfg(rcc_v1)]
            w.set_freqrange(hse.range());
            w.set_waitcycle(pac::sysctrl::vals::HseWait::Cycles262144);
            w.set_flt(false);
            w.set_detcnt(detector);
        });
        wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
            let v = r.hse().read();
            #[cfg(rcc_cw32f020_v1)]
            let range = v.freq();
            #[cfg(rcc_v1)]
            let range = v.freqrange();
            v.mode() == (hse.mode == HseMode::Bypass)
                && v.driver() == hse.drive
                && range == hse.range()
                && v.waitcycle() == pac::sysctrl::vals::HseWait::Cycles262144
                && !v.flt()
                && v.detcnt() == detector
        })?;
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_hseen(true);
        });
        wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
            r.cr1().read().hseen()
        })?;
        wait_until(config.timeout, Error::HseTimeout, || {
            r.hse().read().stable()
        })?;
    }
    #[cfg(rcc_pll)]
    let verify_pll_reference = |pll: Pll| -> Result<(), Error> {
        let sources = r.cr1().read();
        if !sources.hsien()
            || !ccs_enabled()
            || r.hsi().read().div() != config.hsi.div as u8
            || r.hsi().read().trim() != trim
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        if !r.hsi().read().stable() {
            return Err(Error::HsiTimeout);
        }
        if pll.src == PllSource::HSE {
            let hse = config.hse.ok_or(Error::HseNotConfigured)?;
            if !sources.hseen() || !r.hse().read().stable() || !hse_parameters_match(hse)? {
                return Err(Error::HseTimeout);
            }
            if !crate::rcc_hse_pins_match(hse.mode == HseMode::Bypass, config.timeout, cs)? {
                return Err(Error::HsePinConfigurationTimeout);
            }
        }
        check_external_faults(sources.hseen(), sources.lseen())
    };
    #[cfg(rcc_pll)]
    if let Some(pll) = config.pll {
        let parameters = pll.parameters(&config)?;
        // Both PLLEN and the real STABLE latch were observed clear before any
        // HSI/divider changes above. Modify preserves reserved/debug defaults.
        r.pll().modify(|w| {
            w.set_source(parameters.source);
            w.set_freqin(parameters.input_range);
            w.set_mul(pll.mul);
            w.set_freqout(parameters.output_range);
            w.set_waitcycle(pac::sysctrl::vals::PllWait::from_bits(
                crate::RCC_PLL_STARTUP_ENCODING,
            ));
        });
        wait_until(config.timeout, Error::PllConfigurationTimeout, || {
            pll_parameters_match(pll, &parameters)
                && !r.cr1().read().pllen()
                && !r.pll().read().stable()
        })?;
        // Recheck the selected reference and its physical pads immediately
        // before enabling the dependent PLL; STABLE alone is insufficient.
        verify_pll_reference(pll)?;
        modify_sources(permanent_lsi, |w| {
            w.set_key(0x5a5a);
            w.set_pllen(true);
        });
        wait_until(config.timeout, Error::PllConfigurationTimeout, || {
            r.cr1().read().pllen() && ccs_enabled() && pll_parameters_match(pll, &parameters)
        })?;
        // PLLRDY is a clearable event; only PLL.STABLE is the startup handshake.
        wait_until(config.timeout, Error::PllTimeout, || {
            r.pll().read().stable()
        })?;
    }
    #[cfg(rcc_pll)]
    if let Some(pll) = config.pll {
        verify_pll_reference(pll)?;
        if !r.cr1().read().pllen() || !pll_parameters_match(pll, &pll.parameters(&config)?) {
            return Err(Error::PllConfigurationTimeout);
        }
        if !r.pll().read().stable() {
            return Err(Error::PllTimeout);
        }
    }
    #[cfg(rcc_lse)]
    if target_lse {
        let (lse, _) = clocks.lse.ok_or(Error::LseNotConfigured)?;
        let detector = factory_lsi.ok_or(Error::LsiClockInUse)?;
        // All source preparation and pin/gate inspections happen while the
        // verified intermediate HSI is selected. This branch never returns to
        // the old targets' late LSE startup or divider-after-selection sequence.
        if r.cr0().read().sysclk() != ClockSource::Hsi {
            return Err(Error::ClockSwitchTimeout);
        }
        wait_factory_lsi(detector, config.timeout)?;
        super::lse::start(lse, reuse_lse.unwrap(), cs)?;
        let verify_tree = |source: ClockSource, final_buses: bool| -> Result<(), Error> {
            super::lse::verify(lse, cs)?;
            if let Some(hse) = config.hse {
                if !crate::rcc_hse_pins_match(hse.mode == HseMode::Bypass, config.timeout, cs)? {
                    return Err(Error::HsePinConfigurationTimeout);
                }
            }
            // Pad verification can open/restore gates. Read the entire source
            // predicate again afterward; an earlier successful start is not proof.
            wait_factory_lsi(detector, config.timeout)?;
            let cr0 = r.cr0().read();
            let cr1 = r.cr1().read();
            let hsi = r.hsi().read();
            if cr0.sysclk() != source {
                return Err(Error::ClockSwitchTimeout);
            }
            if (final_buses
                && (cr0.hclkprs() != config.ahb_pre as u8 || cr0.pclkprs() != config.apb_pre as u8))
                || !cr1.hsien()
                || !hsi.stable()
                || hsi.trim() != trim
                || hsi.div() != config.hsi.div as u8
                || !cr1.clkccs()
                || !cr1.hseccs()
                || !cr1.lseccs()
                || !cr1.lsien()
                || !cr1.lseen()
                || cr1.lselock() != old_sources.lselock()
                || cr1.pllen() != old_sources.pllen()
                || r.pll().read().0 != old_pll.0
                || cr1.hseen() != (old_sources.hseen() || config.hse.is_some())
            {
                return Err(Error::ClockConfigurationTimeout);
            }
            if let Some(hse) = config.hse {
                if !r.hse().read().stable() || !hse_parameters_match(hse)? {
                    return Err(Error::HseTimeout);
                }
            } else if old_sources.hseen() && r.hse().read().0 != old_hse.0 {
                return Err(Error::HseClockInUse);
            }
            if retain_pll
                && old_pll.source() != pac::sysctrl::vals::PllSource::Hsi
                && (!cr1.hseen() || !r.hse().read().stable())
            {
                return Err(Error::RetainedPllClockInUse);
            }
            if reuse_lse == Some(true) && r.lse().read().0 != old_lse.0 {
                return Err(Error::LseNotReady);
            }
            check_external_faults(cr1.hseen(), true)?;
            if !super::lse::healthy(lse) {
                return Err(Error::LseNotReady);
            }
            wait_factory_lsi(detector, config.timeout)?;
            Ok(())
        };
        verify_tree(ClockSource::Hsi, false)?;
        // Pure validation separately qualified the configured HSI envelope
        // under these final divisors. Explicit HSI prevents a sampled external
        // selector being reasserted by this new path's divider update.
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(ClockSource::Hsi);
            w.set_hclkprs(config.ahb_pre as u8);
            w.set_pclkprs(config.apb_pre as u8);
        });
        wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
            let cr0 = r.cr0().read();
            cr0.sysclk() == ClockSource::Hsi
                && cr0.hclkprs() == config.ahb_pre as u8
                && cr0.pclkprs() == config.apb_pre as u8
        })?;
        barrier();
        let upper_hclk = clocks.hclk_bounds().maximum().0.max(
            crate::rcc::ClockBounds::hsi(config.hsi.div.divisor())
                .divided_by(config.ahb_pre.divisor())
                .maximum()
                .0,
        );
        let flash_wait = (upper_hclk - 1) / crate::RCC_FLASH_WAIT_STEP_HZ;
        set_flash_latency(flash_wait, config.timeout, preserve_flash_features)?;
        verify_tree(ClockSource::Hsi, true)?;
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(ClockSource::Lse);
        });
        wait_until(config.timeout, Error::ClockSwitchTimeout, || {
            let cr0 = r.cr0().read();
            cr0.sysclk() == ClockSource::Lse
                && cr0.hclkprs() == config.ahb_pre as u8
                && cr0.pclkprs() == config.apb_pre as u8
        })?;
        barrier();
        // No divider/source write follows the final LSE selector. Verify the
        // complete tree after every source, pad, gate and Flash handshake.
        verify_tree(ClockSource::Lse, true)?;
        let final_flash = pac::FLASH.cr2().read();
        if u32::from(final_flash.wait()) != flash_wait
            || final_flash.fetch() != flash_features.fetch()
            || final_flash.cache() != flash_features.cache()
        {
            return Err(Error::FlashLatencyTimeout);
        }
        let final_clock = r.cr0().read();
        if final_clock.sysclk() != ClockSource::Lse
            || final_clock.hclkprs() != config.ahb_pre as u8
            || final_clock.pclkprs() != config.apb_pre as u8
        {
            return Err(Error::ClockSwitchTimeout);
        }
        LSE_SYSCLK_LSI
            .borrow(cs)
            .set(Some(lsi_parameters(detector)));
        return Ok(clocks);
    }
    let sysclk = match config.sys {
        Sysclk::HSI => ClockSource::Hsi,
        Sysclk::LSI => ClockSource::Lsi,
        #[cfg(rcc_lse)]
        Sysclk::LSE => unreachable!("LSE returns through its dedicated sequence"),
        Sysclk::HSE => ClockSource::Hse,
        #[cfg(rcc_pll)]
        Sysclk::PLL => ClockSource::Pll,
    };
    // Keep restrictive buses while selecting the source. Install final dividers
    // only after mux acknowledgment, so a slower target never exposes old HSI
    // through newly weakened final buses.
    if config.sys != Sysclk::HSI {
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(sysclk);
        });
        wait_until(config.timeout, Error::ClockSwitchTimeout, || {
            r.cr0().read().sysclk() == sysclk
        })?;
        barrier();
    }
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(sysclk);
        w.set_hclkprs(config.ahb_pre as u8);
        w.set_pclkprs(config.apb_pre as u8);
    });
    wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
        let v = r.cr0().read();
        v.sysclk() == sysclk
            && v.hclkprs() == config.ahb_pre as u8
            && v.pclkprs() == config.apb_pre as u8
    })?;
    barrier();
    wait_until(config.timeout, Error::ClockConfigurationTimeout, || {
        let v = r.hsi().read();
        v.div() == config.hsi.div as u8 && v.trim() == trim
    })?;
    wait_until(config.timeout, Error::HsiTimeout, || {
        r.hsi().read().stable()
    })?;
    if config.hse.is_some() {
        wait_until(config.timeout, Error::HseTimeout, || {
            r.hse().read().stable()
        })?;
    }
    let mut upper_hclk = clocks.hclk_bounds().maximum().0;
    if config.sys != Sysclk::HSI || config.hse.is_some() {
        upper_hclk = upper_hclk.max(
            crate::rcc::ClockBounds::hsi(config.hsi.div.divisor())
                .divided_by(config.ahb_pre.divisor())
                .maximum()
                .0,
        );
    }
    #[cfg(rcc_pll)]
    let verify_pll_tree = |expected_lse: bool| -> Result<(), Error> {
        let Some(pll) = config.pll else { return Ok(()) };
        let cr0 = r.cr0().read();
        let sources = r.cr1().read();
        let hsi = r.hsi().read();
        if cr0.sysclk() != sysclk
            || cr0.hclkprs() != config.ahb_pre as u8
            || cr0.pclkprs() != config.apb_pre as u8
            || !sources.hsien()
            || !ccs_enabled()
            || sources.lselock() != old_sources.lselock()
            || sources.lseen() != expected_lse
            || sources.lsien() != (lsi_was_enabled || needs_lsi)
            || sources.hseen() != (old_sources.hseen() || config.hse.is_some())
            || hsi.div() != config.hsi.div as u8
            || hsi.trim() != trim
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        if !hsi.stable() {
            return Err(Error::HsiTimeout);
        }
        if !sources.pllen() || !pll_parameters_match(pll, &pll.parameters(&config)?) {
            return Err(Error::PllConfigurationTimeout);
        }
        if !r.pll().read().stable() {
            return Err(Error::PllTimeout);
        }
        if sources.lsien() {
            let lsi = r.lsi().read();
            if !lsi.stable()
                || lsi.trim() != old_lsi.trim()
                || lsi.waitcycle() != old_lsi.waitcycle()
            {
                return Err(Error::LsiTimeout);
            }
        }
        if let Some(hse) = config.hse {
            if !r.hse().read().stable() || !hse_parameters_match(hse)? {
                return Err(Error::HseTimeout);
            }
            if !crate::rcc_hse_pins_match(hse.mode == HseMode::Bypass, config.timeout, cs)? {
                return Err(Error::HsePinConfigurationTimeout);
            }
        } else if old_sources.hseen() && r.hse().read().0 != old_hse.0 {
            return Err(Error::HseClockInUse);
        }
        // An inherited LSE is never stopped, retuned or unlocked by this path.
        if old_sources.lseen() && (!sources.lseen() || r.lse().read().0 != old_lse.0) {
            return Err(Error::ClockConfigurationTimeout);
        }
        check_external_faults(sources.hseen(), sources.lseen())
    };
    #[cfg(rcc_pll)]
    verify_pll_tree(old_sources.lseen())?;
    let flash_wait = (upper_hclk - 1) / crate::RCC_FLASH_WAIT_STEP_HZ;
    set_flash_latency(flash_wait, config.timeout, preserve_flash_features)?;
    #[cfg(rcc_pll)]
    verify_pll_tree(old_sources.lseen())?;
    #[cfg(rcc_lse)]
    if let Some(lse) = config.lse {
        super::lse::start(lse, reuse_lse.unwrap(), cs)?;
    }
    // Detect mux changes during the final Flash handshake before publishing.
    if r.cr0().read().sysclk() != sysclk {
        return Err(Error::ClockSwitchTimeout);
    }
    #[cfg(rcc_pll)]
    {
        let expected_lse = old_sources.lseen();
        #[cfg(rcc_lse)]
        let expected_lse = expected_lse || config.lse.is_some();
        verify_pll_tree(expected_lse)?;
    }
    let final_flash = pac::FLASH.cr2().read();
    if u32::from(final_flash.wait()) != flash_wait
        || final_flash.fetch() != flash_features.fetch()
        || final_flash.cache() != flash_features.cache()
    {
        return Err(Error::FlashLatencyTimeout);
    }
    if let Some(expected) = factory_lsi {
        wait_factory_lsi(expected, config.timeout)?;
        let cr0 = r.cr0().read();
        let cr1 = r.cr1().read();
        let hsi = r.hsi().read();
        if cr0.sysclk() != ClockSource::Lsi
            || cr0.hclkprs() != config.ahb_pre as u8
            || cr0.pclkprs() != config.apb_pre as u8
            || !cr1.hsien()
            || !hsi.stable()
            || hsi.trim() != trim
            || hsi.div() != config.hsi.div as u8
            || !cr1.clkccs()
            || !cr1.hseccs()
            || !cr1.lseccs()
            || (retain_pll && (!cr1.pllen() || r.pll().read().0 != old_pll.0))
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        check_external_faults(cr1.hseen(), cr1.lseen())?;
    }
    Ok(clocks)
}

#[cfg(rcc_pll)]
fn ccs_enabled() -> bool {
    let v = pac::SYSCTRL.cr1().read();
    v.clkccs() && v.hseccs() && v.lseccs()
}

#[cfg(rcc_pll)]
fn check_external_faults(hse: bool, lse: bool) -> Result<(), Error> {
    let v = pac::SYSCTRL.isr().read();
    if (hse && (v.hsefail() || v.hsefault())) || (lse && (v.lsefail() || v.lsefault())) {
        Err(Error::ExternalClockFault)
    } else {
        Ok(())
    }
}

fn hse_parameters_match(hse: Hse) -> Result<bool, Error> {
    let v = pac::SYSCTRL.hse().read();
    #[cfg(rcc_cw32f020_v1)]
    let range = v.freq();
    #[cfg(rcc_v1)]
    let range = v.freqrange();
    Ok(v.mode() == (hse.mode == HseMode::Bypass)
        && v.driver() == hse.drive
        && range == hse.range()
        && v.waitcycle() == pac::sysctrl::vals::HseWait::Cycles262144
        && !v.flt()
        && v.detcnt() == hse.detector_count()?)
}

#[cfg(rcc_pll)]
fn pll_parameters_match(pll: Pll, parameters: &PllParameters) -> bool {
    let v = pac::SYSCTRL.pll().read();
    v.source() == parameters.source
        && v.freqin() == parameters.input_range
        && v.mul() == pll.mul
        && v.freqout() == parameters.output_range
        && v.waitcycle().to_bits() == crate::RCC_PLL_STARTUP_ENCODING
        && v.reserved_debug().to_bits() == crate::RCC_PLL_RESERVED_DEBUG_DEFAULT
}
