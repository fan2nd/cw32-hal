//! CW32L052/CW32L083 qualified HSI/direct HSE and L083 HSI/HSE-fed PLL clock trees.
//! HSE, retained AUTOTRIM/RTC/LVD ownership and pad qualification:
//! docs/qualified-l052-hse.md and docs/qualified-l083-hse.md; own family sources.
//!
//! Sources: L052 UM CN V1.5 and L083 UM CN V2.0 §§4.3.4, 4.5, 4.7, 7.4,
//! 7.9.2; each official SDK's RCC_HSI_Enable and FLASH_SetLatency.
//! Both use factory-calibrated 48 MHz HSIOSC at 0x00100a00 and the same nine
//! documented divider encodings. L052 has no PLL. L083 admits the separately
//! qualified HSI/HSE-fed PLL; see docs/l083-hsi-pll.md. An inherited PLL is
//! stopped only after safely leaving it, before changing its reference or fields.
//! A changed trim requires temporary LSI, stopping HSI, then calibration and
//! restart. Divider-only live changes are explicitly permitted by §4.5.2.
//! Unrequested HSE/LSE, LSI parameters/previous enable, clock security, debug/reset, flash
//! reserved bits and unrelated gates are preserved. No CACHE/FETCH bits exist.
//! Every hardware wait is bounded. A failed transition can leave temporary
//! LSI enabled/selected or PLL disabled; reset before retrying. Call only before
//! peripheral/DMA/application interrupt code can depend on these clocks.
//! Polling requires CPU progress; HSE-fed PLL reference loss has no qualified
//! fallback, continuous lock indication or post-loss rate guarantee.
//! MCO and dedicated PLL_OUT consumers must also be quiescent: PLL stopping and
//! reconfiguration do not preserve clocks already exported to other hardware.
//! The incoming clock state must already satisfy the board's voltage limits.
//! Both datasheets require HCLK/PCLK <=24 MHz below 1.8 V (minimum 1.65 V);
//! above 24 MHz the board must supply at least 1.8 V. Init does not measure VDD.
//! Conservative temporary bus dividers prevent an intermediate overspeed.
//! Exact qualified L052/L083 factory-LSI SYSCLK uses an init-only transition: LSIEN
//! remains set, live LSI is never retuned, and failed partial changes have no
//! rollback or safe-retry guarantee. Reset, and power reset for POR-retained
//! LSE controls when required, before retrying. GPIOA/B/C/D/F (and L083 GPIOE) inspection
//! temporarily resumes whole-bank input sampling, filters, interrupts and
//! events even on a later refusal. The caller hands over those functional
//! effects and external pin feedback before initialization. Restoring a gate
//! does not undo hardware activity. No timer, LCD pump, watchdog, IRQ or RTC
//! command is used to manufacture admission.

use crate::{pac, time::Hertz};
use core::cell::Cell;
use critical_section::Mutex;

#[cfg(any(rcc_hse, rcc_lse))]
mod hse;
#[cfg(not(rcc_hse))]
mod hsi;
mod io;
#[cfg(rcc_lsi_sysclk)]
mod lsi;

#[cfg(rcc_hse)]
use hse::configure_hse;
#[cfg(rcc_lse)]
use hse::prepare_lse_monitor;
#[cfg(not(rcc_hse))]
use hsi::configure;
#[cfg(not(rcc_hse))]
use io::poll;
use io::{barrier, set_flash_latency};
#[cfg(rcc_hse)]
use io::{check_external_faults, hse_parameters_match, verify_hse_source, wait_until};
#[cfg(rcc_pll)]
use io::{pll_parameters_match, verify_pll_reference};
#[cfg(rcc_lsi_sysclk)]
use lsi::configure_lsi;
#[cfg(rcc_lsi_sysclk)]
pub(crate) use lsi::lsi_inspection_error;

#[cfg(all(rcc_lse, rcc_cw32l083_v1))]
mod l083_lse_sysclk;
#[cfg(all(rcc_lse, rcc_cw32l052_v1))]
mod lse_sysclk;
#[cfg(rcc_hse)]
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

/// Divider from HCLK to PCLK. L052/L083 have one PCLK domain.
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

/// L083 source-qualified PLL multipliers, using the literal hardware field.
#[cfg(rcc_pll)]
pub use pac::sysctrl::vals::PllMul;

/// Qualified PLL reference. The divider belongs to `Config.hsi`, not the PLL.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg(rcc_pll)]
pub enum PllSource {
    /// Factory-trimmed HSI after its configured divider.
    HSI,
    /// Undivided HSE from `Config.hse`, in oscillator or bypass mode. The
    /// existing HSE board and operating-condition contract and all PLL
    /// input/output limits apply. Oscillator mode follows the vendor-documented
    /// crystal-to-PLL path; admission does not independently certify internal
    /// reference duty. Bypass requires the specified OSC_IN waveform, including
    /// 40–60% duty. PLL bounds describe rates only; individual-cycle timing and
    /// recovery after reference loss are not guaranteed.
    HSE,
}

/// One-time L083 PLL configuration. It must be selected as `Sysclk::PLL`.
///
/// Both actual input endpoints must fit one documented input bin, and both
/// actual multiplied output endpoints must fit one output bin and the separate
/// electrical limits. This conservative policy admits HSI /6 x2,x4,x5,x7 and
/// /10 x3,x4,x6,x7,x8,x9,x11,x12 at the full factory-HSI qualification.
/// HSE uses the complete declared source bounds through the same checks.
/// Settings outside these bounds and bins are not qualified by this API.
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
    bounds: crate::rcc::ClockBounds,
    source: pac::sysctrl::vals::PllSource,
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
        let input_index = crate::RCC_PLL_INPUT_BINS_HZ
            .iter()
            .position(|(lo, hi)| !input.minimum_below(*lo) && !input.maximum_exceeds(*hi))
            .ok_or(Error::PllInputCrossesBin)?;
        let output_index = crate::RCC_PLL_OUTPUT_BINS_HZ
            .iter()
            .position(|(lo, hi)| !bounds.minimum_below(*lo) && !bounds.maximum_exceeds(*hi))
            .ok_or(Error::PllOutputCrossesBin)?;
        Ok(PllParameters {
            bounds,
            source,
            input_range: pac::sysctrl::vals::PllInputRange::from_bits(input_index as u8),
            output_range: pac::sysctrl::vals::PllOutputRange::from_bits(output_index as u8),
        })
    }
}

/// System-clock sources implemented by this backend.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg(rcc_hse)]
pub enum Sysclk {
    /// Factory-calibrated HSI through its configured divider.
    HSI,
    /// Qualified external high-speed oscillator or input.
    HSE,
    /// Init-only factory-calibrated LSI. Its bounds describe rates only.
    /// Exact qualified L052 and L083 parts: 1.65..5.5 V, VDDA = VDD, -40..85 C.
    /// Factory nominal 32800 Hz, rate envelope 31816..33784 Hz. No startup
    /// duration, per-cycle timing, continuing operation or recovery is promised.
    /// Admission temporarily resumes whole GPIOA/B/C/D/F banks, including
    /// unrelated pins and their armed filter/interrupt/event paths. Hand over
    /// these functional effects and external pin feedback before initialization.
    /// All qualified L052/L083 RTC LSI aliases are rate-only under every SYSCLK.
    /// L083 additionally inspects GPIOE, stops any admitted inherited PLL, and
    /// retains Flash WAIT2. External declarations require VDD >= 1.8 V.
    #[cfg(rcc_lsi_sysclk)]
    LSI,
    /// Init-only board-qualified LSE, with retained factory LSI monitoring.
    /// On the five qualified L083 packages, conservative fallback admission
    /// requires VDD >= 1.8 V and retains Flash WAIT2 regardless of dividers.
    #[cfg(rcc_lse)]
    LSE,
    /// L083 HSI/HSE-fed PLL, with independent electrical and analog-bin checks.
    #[cfg(rcc_pll)]
    PLL,
}

/// HSE electrical mode. The board must reserve the actual oscillator pads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg(rcc_hse)]
pub enum HseMode {
    /// Crystal or ceramic resonator on OSC_IN and OSC_OUT.
    Oscillator,
    /// Externally driven digital clock on OSC_IN; OSC_OUT remains available.
    Bypass,
}

#[cfg(rcc_hse)]
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
#[cfg(rcc_hse)]
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
    /// Crystal drive setting; L052 uses it for both pre-start and run phases.
    /// Ignored electrically in bypass mode.
    pub drive: HseDrive,
}
#[cfg(rcc_hse)]
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
    /// HSI oscillator divider.
    pub hsi: Hsi,
    /// Optional board-qualified source; its pads are reserved with any SYSCLK.
    #[cfg(rcc_hse)]
    pub hse: Option<Hse>,
    /// Init-only nominal 32768 Hz LSE; None preserves inherited state.
    #[cfg(rcc_lse)]
    pub lse: Option<super::Lse>,
    /// Optional L083 PLL, admitted only when selected as the system source.
    #[cfg(rcc_pll)]
    pub pll: Option<Pll>,
    /// Requested system-clock source. HSI remains enabled for a legal escape.
    #[cfg(rcc_hse)]
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
            #[cfg(rcc_hse)]
            hse: None,
            #[cfg(rcc_lse)]
            lse: None,
            #[cfg(rcc_hse)]
            sys: Sysclk::HSI,
            #[cfg(rcc_pll)]
            pll: None,
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
        #[cfg(not(rcc_hse))]
        let clocks = {
            let hsi = HSI_FREQ / self.hsi.div.divisor();
            let hclk = hsi / self.ahb_pre.divisor();
            let pclk = hclk / self.apb_pre.divisor();
            let clocks = Clocks {
                hsi,
                sys: hsi,
                hclk,
                pclk,
                dividers: [
                    self.hsi.div.divisor(),
                    self.hsi.div.divisor() * self.ahb_pre.divisor(),
                    self.hsi.div.divisor() * self.ahb_pre.divisor() * self.apb_pre.divisor(),
                ],
            };
            crate::rcc::operating::validate(self.operating_conditions, clocks)?;
            clocks
        };
        #[cfg(rcc_hse)]
        let clocks = {
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
                Sysclk::HSE => hse.ok_or(Error::HseNotConfigured)?,
                #[cfg(rcc_lsi_sysclk)]
                Sysclk::LSI => crate::rcc::ClockBounds::lsi(),
                #[cfg(rcc_lse)]
                Sysclk::LSE => {
                    use crate::rtc::sealed::Instance;
                    let (config, bounds) = lse.ok_or(Error::LseNotConfigured)?;
                    // Hardware counts and a distinct software phase margin;
                    // factory LSI rate facts are not per-cycle jitter bounds.
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
                #[cfg(rcc_lse)]
                lse,
                #[cfg(rcc_pll)]
                pll,
            };
            crate::rcc::operating::validate(self.operating_conditions, clocks)?;
            #[cfg(rcc_lsi_sysclk)]
            if self.sys == Sysclk::LSI {
                let board = self.operating_conditions;
                if board.supply_mv < crate::RCC_LSI_SUPPLY_MV.0
                    || board.supply_mv > crate::RCC_LSI_SUPPLY_MV.1
                {
                    return Err(Error::SupplyOutsideQualifiedRange);
                }
                if board.min_temperature_c < crate::RCC_LSI_TEMPERATURE_C.0
                    || board.max_temperature_c > crate::RCC_LSI_TEMPERATURE_C.1
                {
                    return Err(Error::TemperatureOutsideQualifiedRange);
                }
                // P8 executes configured factory HSI at the final divisors.
                crate::rcc::operating::validate(
                    board,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                        ..clocks
                    },
                )?;
                // Own L052 CCS escape is HSI /6. No post-fault divider credit.
                #[cfg(rcc_cw32l052_v1)]
                crate::rcc::operating::validate(
                    board,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                        dividers: [self.hsi.div.divisor(), 1, 1],
                        ..clocks
                    },
                )?;
            }
            #[cfg(all(rcc_lsi_sysclk, rcc_cw32l083_v1))]
            if self.sys == Sysclk::LSI && (self.hse.is_some() || self.lse.is_some()) {
                // New LSI-target admission only: own L083 fallback has no
                // divider contract, so bound raw factory HSI with no credit.
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(1),
                        dividers: [self.hsi.div.divisor(), 1, 1],
                        ..clocks
                    },
                )?;
            }
            #[cfg(all(rcc_lse, rcc_cw32l052_v1))]
            if self.sys == Sysclk::LSE {
                // Final dividers are installed while executing configured HSI.
                // This use edge is independent of an HSE declaration or CLKCCS.
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                        ..clocks
                    },
                )?;
                // Own fallback output is fixed HSI /6. Post-fault bus-divider
                // retention is unspecified, so cover its undivided upper rate.
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                        dividers: [self.hsi.div.divisor(), 1, 1],
                        ..clocks
                    },
                )?;
            }
            #[cfg(all(rcc_lse, rcc_cw32l083_v1))]
            if self.sys == Sysclk::LSE {
                // Requested final divisors are installed while still on HSI.
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                        ..clocks
                    },
                )?;
                // Own L083 manual specifies CCS selecting HSI, without an
                // explicit post-fault divider contract. This target gives no
                // HSI/AHB/APB divisor credit: 48.96 MHz requires VDD >= 1.8 V.
                // It is a conservative bound, not a hardware divider rewrite.
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(1),
                        dividers: [self.hsi.div.divisor(), 1, 1],
                        ..clocks
                    },
                )?;
            }
            // Qualify requested HSI and the own-family CCS fallback for every
            // HSE declaration, even when the incoming switching policy is off.
            // L052 hardware forces HSI /6; L083 retains the configured divider.
            // This admission restriction does not require enabling CCS.
            // It establishes an electrically legal HSI escape, not guaranteed
            // fallback or continued CPU progress after HSE-fed PLL reference loss.
            if self.hse.is_some() {
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
                        ..clocks
                    },
                )?;
                #[cfg(rcc_cw32l052_v1)]
                crate::rcc::operating::validate(
                    self.operating_conditions,
                    Clocks {
                        source: crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR),
                        ..clocks
                    },
                )?;
            }
            clocks
        };
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
    #[cfg(rcc_hse)]
    pub(crate) source: crate::rcc::ClockBounds,
    #[cfg(rcc_hse)]
    hse: Option<HseMode>,
    #[cfg(rcc_lse)]
    pub(crate) lse: Option<(super::Lse, crate::rcc::ClockBounds)>,
    #[cfg(rcc_pll)]
    pll: Option<crate::rcc::ClockBounds>,
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
/// After a failure, no frequencies are published and peripherals must not be
/// used on the assumption that the requested configuration took effect.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The incoming selected source has inconsistent request/readiness.
    #[cfg(rcc_lsi_sysclk)]
    InvalidEntryClock,
    /// The raw factory LSI calibration halfword is erased.
    #[cfg(rcc_lsi_sysclk)]
    InvalidLsiCalibration,
    /// A live LSI does not carry the single-read factory calibration.
    #[cfg(rcc_lsi_sysclk)]
    LsiCalibrationInUse,
    /// Retained consumers, observers or immutable configuration changed.
    #[cfg(rcc_lsi_sysclk)]
    LsiClockInUse,
    /// An owned LSI request or native parameter write did not acknowledge.
    #[cfg(rcc_lsi_sysclk)]
    LsiConfigurationTimeout,
    /// A temporary inspection gate could not be enabled; restoration succeeded.
    #[cfg(rcc_lsi_sysclk)]
    LsiGateEnableTimeout,
    /// An inspection gate did not return to its captured state.
    #[cfg(rcc_lsi_sysclk)]
    LsiGateRestoreTimeout,
    #[cfg(rcc_lse)]
    LseNotConfigured,
    #[cfg(rcc_lse)]
    InvalidLseDetector,
    #[cfg(rcc_lse)]
    LseMonitorConditionsOutsideQualifiedRange,
    #[cfg(rcc_lse)]
    InvalidLseBounds,
    #[cfg(rcc_lse)]
    LseNotReady,
    #[cfg(rcc_lse)]
    LsePinConflict,
    #[cfg(rcc_lse)]
    LseClockInUse,
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
    #[cfg(rcc_hse)]
    InvalidHseBounds,
    #[cfg(rcc_hse)]
    HseOutsideQualifiedRange,
    #[cfg(rcc_hse)]
    HseConditionsOutsideQualifiedRange,
    #[cfg(rcc_hse)]
    HseConditionsDoNotCoverBoard,
    #[cfg(rcc_hse)]
    HsePinsUnavailable,
    #[cfg(rcc_hse)]
    HseNotConfigured,
    #[cfg(rcc_hse)]
    InvalidHseDetector,
    #[cfg(rcc_hse)]
    HseTimeout,
    #[cfg(rcc_hse)]
    HseStopTimeout,
    #[cfg(rcc_hse)]
    HsePinConfigurationTimeout,
    /// Retained RTC/AUTOTRIM ownership conflicts with the requested HSE setup.
    #[cfg(rcc_hse)]
    HseClockInUse,
    #[cfg(rcc_hse)]
    RetainedClockInspectionTimeout,
    /// Recalibrating HSI would interrupt a retained AUTOTRIM or LVD filter.
    #[cfg(rcc_hse)]
    HsiClockInUse,
    /// Active calibration, automatic retrim or a reserved AUTOTRIM source.
    #[cfg(rcc_hse)]
    RetainedAutotrimConfiguration,
    /// The retained RTC source selector is reserved.
    #[cfg(rcc_hse)]
    RetainedRtcConfiguration,
    /// Active AUTOTRIM ETR may occupy the HSE input pad.
    #[cfg(rcc_hse)]
    HsePinInUse,
    /// A monitored external source has a sticky startup or clock-loss fault.
    #[cfg(rcc_hse)]
    ExternalClockFault,
    /// Temporary LSI selection or restoration did not acknowledge.
    #[cfg(rcc_hse)]
    TemporaryClockSwitchTimeout,
    #[cfg(rcc_hse)]
    LsiRestoreTimeout,

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
    /// Temporary LSI did not report stable within the poll budget.
    LsiTimeout,
    /// L083 PLL did not stop before an HSI configuration change.
    #[cfg(rcc_cw32l083_v1)]
    PllStopTimeout,
    /// HSI did not stop before a required trim change.
    HsiStopTimeout,
    /// HSI did not report stable within the poll budget.
    HsiTimeout,
    /// The system clock selector did not read back as HSI.
    ClockSwitchTimeout,
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
pub(crate) unsafe fn init(config: Config) -> Result<(), Error> {
    critical_section::with(|cs| {
        if CLOCKS.borrow(cs).get().is_some() {
            return Err(Error::AlreadyInitialized);
        }
        #[cfg(not(rcc_hse))]
        let clocks = configure(config)?;
        #[cfg(rcc_hse)]
        let clocks = configure_hse(config, cs)?;
        CLOCKS.borrow(cs).set(Some(clocks));
        Ok(())
    })
}

#[cfg(rcc_hse)]
pub(crate) fn hse_pin_reserved(pin: u8) -> bool {
    let Some(clocks) = try_clocks() else {
        return false;
    };
    clocks.hse.is_some_and(|mode| {
        crate::RCC_HSE_PINS.0 == Some(pin)
            || (mode == HseMode::Oscillator && crate::RCC_HSE_PINS.1 == Some(pin))
    })
}

#[cfg(all(rcc_lse, rcc_cw32l052_v1))]
pub(super) fn lse_sysclk_monitor_ready() -> bool {
    lse_sysclk::monitor_ready()
}

#[cfg(all(rcc_lse, rcc_cw32l083_v1))]
pub(super) fn lse_sysclk_monitor_ready() -> bool {
    l083_lse_sysclk::monitor_ready()
}
