//! Qualified HSI and direct digital HEX initialization for CW32F002/F003.
//! Factory-LSI SYSCLK is additionally admitted only by qualified exact-part metadata.
//!
//! Own manuals: F002 CN1.4 / F003 CN2.3 §§4.3–4.7, GPIO and AWT chapters;
//! own datasheet external/electrical tables. See docs/qualified-hex.md.
//! Both PB0 and PB1 are inputs, independently usable by retained AWT. There is
//! no HSE crystal, PLL, programmable startup count or documented clock-loss
//! recovery on these chips. STABLE latches startup and does not detect loss.
//! One-time init requires continuous legal clocks; runtime changes and
//! low-power entry/resume are outside this backend's contract.

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

/// Divider from HCLK to PCLK. F002/F003 have one PCLK domain.
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

/// Direct system-clock sources supported by F002/F003.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysclk {
    /// Factory-calibrated HSI through its divider.
    HSI,
    /// Externally driven digital clock on PB0 or PB1.
    HEX,
    /// Factory-calibrated 32.8 kHz LSI, qualified only for supported exact parts.
    ///
    /// Its bounds qualify frequency over the declared supply/ambient range, not
    /// individual cycle durations. A stopped source is admitted only after all
    /// retained direct consumers and ready observers have been excluded.
    /// That inspection may briefly enable whole GPIOA/B/C banks, advancing
    /// sampling, filters or armed events, including when initialization fails.
    /// Bounded gate restoration is attempted; a failed restore can leave a bank
    /// enabled. Elapsed work and naturally raised flags are not undone.
    /// An already requested/selected factory-matching source is reused without
    /// rewriting TRIM or WAIT. Success retains both LSI and factory HSI requests.
    #[cfg(rcc_lsi_sysclk)]
    LSI,
}

pub use pac::sysctrl::vals::HexInput;

/// Board-qualified digital input, independent of factory HSI accuracy.
///
/// The complete actual envelope must remain within 4–32 MHz. Include source
/// tolerance, temperature, aging, loading, supply and short-term cycle variation
/// throughout `operating_conditions`. Both the 40–60% duty requirement and the
/// minimum 15 ns high/low times apply; rise/fall times must be at most 20 ns.
/// Meet the own datasheet's voltage thresholds and I/O ratings as well.
/// These declarations are not measurements. The source must remain continuous
/// throughout initialization and use: these chips have no documented automatic
/// loss-clock fallback, and STABLE is a startup latch rather than a loss monitor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Hex {
    /// Nominal frequency; never a substitute for the actual upper bound.
    pub freq: Hertz,
    /// Guaranteed minimum actual frequency, in whole hertz.
    pub min_freq: Hertz,
    /// Guaranteed maximum actual frequency, in whole hertz.
    pub max_freq: Hertz,
    /// MCU supply and ambient interval qualifying these source bounds.
    pub operating_conditions: crate::rcc::OperatingConditions,
    /// One independently available digital input; neither is a crystal output.
    pub input: HexInput,
}
impl Hex {
    fn bounds(self) -> Result<crate::rcc::ClockBounds, Error> {
        let bounds = crate::rcc::ClockBounds::external(
            self.freq,
            self.min_freq,
            self.max_freq,
            self.operating_conditions,
        )
        .ok_or(Error::InvalidHexBounds)?;
        if self.min_freq.0 < crate::RCC_HEX_RANGE_HZ.0
            || self.max_freq.0 > crate::RCC_HEX_RANGE_HZ.1
        {
            return Err(Error::HexOutsideQualifiedRange);
        }
        let c = self.operating_conditions;
        if c.supply_mv < crate::RCC_HEX_SUPPLY_MV.0
            || c.supply_mv > crate::RCC_HEX_SUPPLY_MV.1
            || c.min_temperature_c < crate::RCC_HEX_TEMPERATURE_C.0
            || c.max_temperature_c > crate::RCC_HEX_TEMPERATURE_C.1
        {
            return Err(Error::HexConditionsOutsideQualifiedRange);
        }
        Ok(bounds)
    }
}

/// Clock tree initialization options, using Embassy's RCC configuration naming.
///
/// One-time initialization requires an electrically legal, stable inherited
/// HSI/HEX/LSI source, a documented HSI divider and correct incoming Flash wait.
/// Every source used during the transition must remain continuous and within
/// its own electrical limits. HSI/HEX targets use unchanged LSI as a trim bridge,
/// which must be in its documented 32.8 kHz ±10% range. The qualified LSI target
/// first establishes factory calibration and preserves its WAIT setting.
/// A ready flag cannot qualify arbitrary inherited trim. Clock-dependent peripherals, DMA and interrupts must be
/// quiescent, and no concurrent code (including NMI) may change the clock tree.
///
/// These conditions apply when passed to `embassy_cw32::init`.
/// This is not a runtime clock-switch or loss-clock recovery interface. See
/// `docs/qualified-hex.md` for the complete source and retained-owner contract.
#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// Board-declared supply and ambient TA envelope. No measurement is made.
    /// The full qualified range is the default; declare the actual narrower
    /// board range to permit faster operation where the datasheet allows it.
    pub operating_conditions: crate::rcc::OperatingConditions,
    /// HSI oscillator divider.
    pub hsi: Hsi,
    /// Optional external input, configured and reserved independently of SYSCLK.
    pub hex: Option<Hex>,
    /// System-clock source. HEX requires `hex: Some(...)`.
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
            hex: None,
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
        let hsi = HSI_FREQ / self.hsi.div.divisor();
        let hex = if let Some(hex) = self.hex {
            let bounds = hex.bounds()?;
            let board = self.operating_conditions;
            let source = hex.operating_conditions;
            if board.supply_mv != source.supply_mv
                || board.min_temperature_c < source.min_temperature_c
                || board.max_temperature_c > source.max_temperature_c
            {
                return Err(Error::HexConditionsDoNotCoverBoard);
            }
            if crate::RCC_HEX_PINS[hex.input.to_bits() as usize].is_none() {
                return Err(Error::HexPinUnavailable);
            }
            Some(bounds)
        } else {
            None
        };
        let source = match self.sys {
            Sysclk::HSI => crate::rcc::ClockBounds::hsi(self.hsi.div.divisor()),
            Sysclk::HEX => hex.ok_or(Error::HexNotConfigured)?,
            #[cfg(rcc_lsi_sysclk)]
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
            reserved_hex_inputs: self.hex.map_or(0, |hex| 1 << hex.input.to_bits()),
        };
        crate::rcc::operating::validate(self.operating_conditions, clocks)?;
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
    // Frozen whole-boot ownership: system HEX and independent retained AWT pads.
    reserved_hex_inputs: u8,
}

/// A clock initialization failure.
///
/// Hardware errors may leave a partially changed clock tree. The initializer
/// never lowers flash latency before the requested clocks have been verified.
/// Calibration failures may leave the CPU on LSI with HSI stopped or restarting.
/// A factory-LSI attempt may leave changed TRIM, enabled gates after a failed
/// restoration, or a permanent LSI request that becomes ready after timeout.
/// GPIO inspection windows may advance bank activity even on failure; no flag
/// clearing, source cleanup or transactional rollback is performed. Reset before retrying.
/// After a failure, no frequencies are published and peripherals must not be
/// used on the assumption that the requested configuration took effect.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// HEX system clock selected without a declaration.
    HexNotConfigured,
    /// External bounds are zero, inverted or exclude the nominal frequency.
    InvalidHexBounds,
    /// Actual HEX bounds extend beyond the own-source intersection 4–32 MHz.
    HexOutsideQualifiedRange,
    /// External qualification extends outside the own-device electrical range.
    HexConditionsOutsideQualifiedRange,
    /// External source qualification does not cover the declared board envelope.
    HexConditionsDoNotCoverBoard,
    /// The selected HEX pad is absent from the selected package projection.
    HexPinUnavailable,
    /// The selected input pad or its configuration gate did not acknowledge.
    HexPinConfigurationTimeout,
    /// HEX failed to stop and clear its startup-stable indication.
    HexStopTimeout,
    /// HEX did not report startup stability within the poll budget.
    HexTimeout,
    /// Active AWT prevents HEX reconfiguration or has an unproved ETR route.
    HexClockInUse,
    /// Active AWT or LVD filtering depends on HSIOSC that needs retuning.
    HsiClockInUse,
    /// Retained AWT's configuration gate could not be inspected/restored.
    RetainedClockInspectionTimeout,
    /// Active AWT uses a reserved source selector.
    InvalidRetainedClockSource,
    /// The selected inherited source does not report ready.
    InvalidEntryClock,

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
    /// The target or temporary bridge LSI did not stabilize within the poll budget.
    LsiTimeout,
    /// Board conditions extend outside the factory-LSI qualification.
    #[cfg(rcc_lsi_sysclk)]
    LsiConditionsOutsideQualifiedRange,
    /// The factory LSI halfword reads all ones (conservative software rejection).
    #[cfg(rcc_lsi_sysclk)]
    InvalidLsiCalibration,
    /// Live or in-flight LSI differs from factory calibration; it cannot be retuned.
    #[cfg(rcc_lsi_sysclk)]
    LsiCalibrationInUse,
    /// LSI ownership, consumer, observer or entry-state identity is not proved.
    #[cfg(rcc_lsi_sysclk)]
    LsiClockInUse,
    /// A consumer inspection gate failed to enable; its original state was restored.
    #[cfg(rcc_lsi_sysclk)]
    LsiGateEnableTimeout,
    /// A consumer inspection gate failed to restore and may remain enabled.
    #[cfg(rcc_lsi_sysclk)]
    LsiGateRestoreTimeout,
    /// Factory LSI parameters or its permanent software request did not read back.
    #[cfg(rcc_lsi_sysclk)]
    LsiConfigurationTimeout,
    /// HSI did not stop before factory calibration could be installed.
    HsiStopTimeout,
    /// The temporary LSI software-enable request could not be restored.
    LsiRestoreTimeout,
    /// The system clock selector did not read back as the requested source.
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
// Entry must be an electrically legal, stable HSI/HEX/LSI with documented HSI
// divider and appropriate Flash latency. The entry and any unchanged bridge
// sources must stay available. LSI used as a trim bridge must retain its own
// documented 32.8 kHz ±10% qualification. Unknown trim cannot be qualified by
// STABLE. This is not recovery from a failed source or runtime clock switching.
// Monotonic AHB >=/4 and APB /8 guards protect every legal incoming source and
// the retained HSI envelope before source changes; WAIT2 protects the bridge.
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

// Waits remain bounded by the caller's poll budget; each predicate performs
// one read of the selected PAC register.
fn poll(mut ready: impl FnMut() -> bool, timeout: u32, error: Error) -> Result<(), Error> {
    for _ in 0..timeout {
        if ready() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(error)
}

fn barrier() {
    #[cfg(target_arch = "arm")]
    {
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }
}

fn set_flash_latency(wait: u32, timeout: u32) -> Result<(), Error> {
    // Bits 15:3 are reserved on this register version; preserve them, never
    // apply the F030 FETCH/CACHE bits. Only WAIT is programmable here.
    pac::FLASH.cr2().modify(|w| {
        w.set_key(0x5a5a);
        w.set_wait(wait as u8);
    });
    poll(
        || pac::FLASH.cr2().read().wait() == wait as u8,
        timeout,
        Error::FlashLatencyTimeout,
    )?;
    barrier();
    Ok(())
}

pub(crate) fn hex_pin_reserved(pin: u8) -> bool {
    try_clocks().is_some_and(|clocks| {
        crate::RCC_HEX_PINS
            .iter()
            .enumerate()
            .any(|(input, pad)| clocks.reserved_hex_inputs & (1 << input) != 0 && *pad == Some(pin))
    })
}

fn select_source(source: ClockSource, timeout: u32) -> Result<(), Error> {
    pac::SYSCTRL.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(source);
    });
    poll(
        || pac::SYSCTRL.cr0().read().sysclk() == source,
        timeout,
        Error::ClockSwitchTimeout,
    )?;
    barrier();
    Ok(())
}

// Each state below is latched before the guard writes. Only the guard dividers,
// then the attempted factory TRIM, advance before the first source request.
// In particular, a later live observation never reclassifies a cold entry.
#[cfg(rcc_lsi_sysclk)]
struct FactoryLsi {
    clock: pac::sysctrl::regs::Cr0,
    sources: pac::sysctrl::regs::Cr1,
    hsi: pac::sysctrl::regs::Hsi,
    hex: pac::sysctrl::regs::Hex,
    ier: pac::sysctrl::regs::Ier,
    lsi: pac::sysctrl::regs::Lsi,
    factory_trim: u16,
    cold: bool,
    ready: bool,
    consumers: Option<crate::RccFactoryLsiConsumers>,
}

#[cfg(rcc_lsi_sysclk)]
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

#[cfg(rcc_lsi_sysclk)]
fn lsi_parameters(value: pac::sysctrl::regs::Lsi) -> u32 {
    // The actual PAC STABLE field defines this mask; WAIT and all reserved bits
    // remain in the parameter identity.
    value.0 & crate::RCC_LSI_PARAMETERS_MASK
}

#[cfg(rcc_lsi_sysclk)]
impl FactoryLsi {
    fn capture(
        mut clock: pac::sysctrl::regs::Cr0,
        mut sources: pac::sysctrl::regs::Cr1,
        hsi: pac::sysctrl::regs::Hsi,
        hex: pac::sysctrl::regs::Hex,
    ) -> Result<Self, Error> {
        let r = pac::SYSCTRL;
        let lsi = r.lsi().read();
        let isr = r.isr().read();
        let mut ier = r.ier().read();
        let raw =
            unsafe { core::ptr::read_volatile(crate::RCC_LSI_FACTORY_TRIM_ADDRESS as *const u16) };
        if raw == u16::MAX {
            return Err(Error::InvalidLsiCalibration);
        }
        let mut factory = lsi;
        // Zero is valid; high factory-halfword bits have no extra validity rule.
        factory.set_trim(raw);
        let requested = sources.lsien();
        let selected = clock.sysclk() == ClockSource::Lsi;
        let mirror = isr.lsistable();
        if selected && (!lsi.stable() || !mirror) {
            return Err(Error::InvalidEntryClock);
        }
        if lsi.trim() != factory.trim() && (requested || selected || lsi.stable() || mirror) {
            return Err(Error::LsiCalibrationInUse);
        }
        if lsi.stable() != mirror || (!requested && !selected && lsi.stable()) {
            return Err(Error::LsiClockInUse);
        }
        let cold = !requested && !selected;
        if cold
            && (ier.lsirdy()
                || isr.lsirdy()
                || cortex_m::peripheral::NVIC::is_pending(pac::Interrupt::RCC))
        {
            return Err(Error::LsiClockInUse);
        }
        // WO keys are not readable identity. Typed normalization retains every
        // actual parameter/reserved bit; each real write supplies a fresh key.
        clock.set_key(0);
        sources.set_key(0);
        ier.set_key(0);
        Ok(Self {
            clock,
            sources,
            hsi,
            hex,
            ier,
            lsi,
            factory_trim: factory.trim(),
            cold,
            ready: lsi.stable(),
            consumers: None,
        })
    }

    fn check_before_request(&self) -> Result<(), Error> {
        let r = pac::SYSCTRL;
        let mut clock = r.cr0().read();
        let mut sources = r.cr1().read();
        let hsi = r.hsi().read();
        let hex = r.hex().read();
        let mut ier = r.ier().read();
        let lsi = r.lsi().read();
        let isr = r.isr().read();
        clock.set_key(0);
        sources.set_key(0);
        ier.set_key(0);
        // HSI/HEX may naturally complete startup. Masks derived from their
        // actual PAC read-only STABLE fields retain all parameters/reserved bits;
        // LSI has the separately latched entry classification/readiness.
        if clock.0 != self.clock.0
            || sources.0 != self.sources.0
            || hsi.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK
                != self.hsi.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK
            || hex.0 & crate::RCC_LSI_HEX_PARAMETERS_MASK
                != self.hex.0 & crate::RCC_LSI_HEX_PARAMETERS_MASK
            || ier.0 != self.ier.0
            || lsi_parameters(lsi) != lsi_parameters(self.lsi)
            || lsi.stable() != isr.lsistable()
            || (self.ready && !lsi.stable())
        {
            return Err(Error::LsiClockInUse);
        }
        if self.cold
            && (sources.lsien()
                || !matches!(clock.sysclk(), ClockSource::Hsi | ClockSource::Hex)
                || lsi.stable()
                || isr.lsistable()
                || ier.lsirdy()
                || isr.lsirdy()
                || cortex_m::peripheral::NVIC::is_pending(pac::Interrupt::RCC))
        {
            return Err(Error::LsiClockInUse);
        }
        if !self.cold && !(sources.lsien() || clock.sysclk() == ClockSource::Lsi) {
            return Err(Error::LsiClockInUse);
        }
        Ok(())
    }

    fn prepare(
        &mut self,
        timeout: u32,
        cs: critical_section::CriticalSection<'_>,
    ) -> Result<(), Error> {
        let r = pac::SYSCTRL;
        if !self.cold {
            // An entry request may naturally finish startup, but parameters and
            // request/selection identity must remain exactly the latched ones.
            for _ in 0..timeout {
                self.check_before_request()?;
                if r.lsi().read().stable() && r.isr().read().lsistable() {
                    self.ready = true;
                    self.check_before_request()?;
                    return Ok(());
                }
                core::hint::spin_loop();
            }
            return Err(Error::LsiTimeout);
        }
        for _ in 0..2 {
            self.check_before_request()?;
            let consumers = crate::rcc_factory_lsi_consumers(timeout, cs)?;
            self.check_before_request()?;
            if self.consumers.is_some_and(|previous| previous != consumers) {
                return Err(Error::LsiClockInUse);
            }
            if self.consumers.is_none() {
                self.consumers = Some(consumers);
            }
        }
        // The first-TRIM edge is adjacent to the final global recheck. Matching
        // cold TRIM still reaches here only after both complete consumer passes.
        self.check_before_request()?;
        if self.lsi.trim() != self.factory_trim {
            r.lsi().modify(|w| w.set_trim(self.factory_trim));
        }
        self.lsi.set_trim(self.factory_trim);
        poll(
            || r.lsi().read().0 == self.lsi.0 && !r.isr().read().lsistable(),
            timeout,
            Error::LsiConfigurationTimeout,
        )?;
        // Third, use-edge pass compares the same consumers/gates/reset state
        // and the original global identity with only the attempted TRIM advanced.
        self.check_before_request()?;
        self.check_consumers(timeout, cs)?;
        self.check_before_request()?;
        // The caller's very next hardware write requests HSI and permanent LSI
        // together. No HSI/source/consumer write is inserted after this guard.
        Ok(())
    }

    fn check_consumers(
        &self,
        timeout: u32,
        cs: critical_section::CriticalSection<'_>,
    ) -> Result<(), Error> {
        if let Some(expected) = self.consumers {
            if crate::rcc_factory_lsi_consumers(timeout, cs)? != expected {
                return Err(Error::LsiClockInUse);
            }
        }
        Ok(())
    }

    fn wait_ready(&self, timeout: u32) -> Result<(), Error> {
        let r = pac::SYSCTRL;
        for _ in 0..timeout {
            let lsi = r.lsi().read();
            let isr = r.isr().read();
            let mut ier = r.ier().read();
            ier.set_key(0);
            if !r.cr1().read().lsien() || lsi_parameters(lsi) != lsi_parameters(self.lsi) {
                return Err(Error::LsiConfigurationTimeout);
            }
            if ier.0 != self.ier.0 || lsi.stable() != isr.lsistable() {
                return Err(Error::LsiClockInUse);
            }
            if lsi.stable() {
                return Ok(());
            }
            if self.ready {
                // Startup was already proved. A later loss of that identity is
                // not another startup opportunity or a new entry classification.
                return Err(Error::LsiTimeout);
            }
            // LSIRDY may now rise naturally. Neither it nor shared RCC pending
            // is cleared or required to remain zero after our first request.
            core::hint::spin_loop();
        }
        Err(Error::LsiTimeout)
    }

    fn verify_final(
        &self,
        config: Config,
        hsi_trim: u16,
        awt: pac::awt::regs::Cr,
        cs: critical_section::CriticalSection<'_>,
    ) -> Result<(), Error> {
        self.check_consumers(config.timeout, cs)?;
        let info = <crate::peripherals::AWT as crate::rcc::SealedRccPeripheral>::RCC_INFO;
        if info.reset_asserted() {
            return Err(Error::LsiClockInUse);
        }
        let original_gate = info.is_enabled();
        let retained_awt = info
            .inspect_for_init(cs, config.timeout, || {
                if !info.is_enabled() || info.reset_asserted() {
                    return Err(Error::LsiClockInUse);
                }
                let retained = pac::AWT.cr().read();
                if !info.is_enabled() || info.reset_asserted() {
                    return Err(Error::LsiClockInUse);
                }
                Ok(retained)
            })
            .map_err(lsi_inspection_error)?;
        // Resolve the buffered owner result only after restoration; a failed
        // restore must not be hidden by a reset/owner error from the closure.
        if info.is_enabled() != original_gate {
            return Err(Error::LsiGateRestoreTimeout);
        }
        if info.reset_asserted() {
            return Err(Error::LsiClockInUse);
        }
        if retained_awt?.0 != awt.0 {
            return Err(Error::LsiClockInUse);
        }
        if let Some(hex) = config.hex {
            if !crate::rcc_hex_pin_matches(hex.input, config.timeout, cs)? {
                return Err(Error::HexClockInUse);
            }
        }
        self.wait_ready(config.timeout)?;
        let r = pac::SYSCTRL;
        let mut clock = r.cr0().read();
        let mut sources = r.cr1().read();
        let hsi = r.hsi().read();
        let hex = r.hex().read();
        let isr = r.isr().read();
        clock.set_key(0);
        sources.set_key(0);
        let mut expected_clock = self.clock;
        expected_clock.set_sysclk(ClockSource::Lsi);
        expected_clock.set_hclkprs(config.ahb_pre as u8);
        expected_clock.set_pclkprs(config.apb_pre as u8);
        let mut expected_sources = self.sources;
        expected_sources.set_hsien(true);
        expected_sources.set_lsien(true);
        if config.hex.is_some() {
            expected_sources.set_hexen(true);
        }
        let mut expected_hsi = self.hsi;
        expected_hsi.set_trim(hsi_trim);
        expected_hsi.set_div(config.hsi.div as u8);
        let mut expected_hex = self.hex;
        if let Some(configured) = config.hex {
            expected_hex.set_pinmux(configured.input);
            expected_hex.set_pinen(true);
        }
        if clock.0 != expected_clock.0 {
            return Err(Error::ClockConfigurationTimeout);
        }
        if sources.0 != expected_sources.0
            || hsi.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK
                != expected_hsi.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        if !hsi.stable() || !isr.hsistable() {
            return Err(Error::HsiTimeout);
        }
        if (config.hex.is_some() || self.hex.stable()) && (!hex.stable() || !isr.hexstable()) {
            return Err(Error::HexTimeout);
        }
        if hex.0 & crate::RCC_LSI_HEX_PARAMETERS_MASK
            != expected_hex.0 & crate::RCC_LSI_HEX_PARAMETERS_MASK
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        Ok(())
    }
}

fn configure(config: Config, cs: critical_section::CriticalSection<'_>) -> Result<Clocks, Error> {
    let mut clocks = config.frequencies()?;
    #[cfg(rcc_lsi_sysclk)]
    let permanent_lsi = config.sys == Sysclk::LSI;
    #[cfg(not(rcc_lsi_sysclk))]
    let permanent_lsi = false;
    let r = pac::SYSCTRL;
    let old_clock = r.cr0().read();
    let old_sources = r.cr1().read();
    let old_hex = r.hex().read();
    match old_clock.sysclk() {
        ClockSource::Hsi if r.hsi().read().stable() => {}
        ClockSource::Hex if old_sources.hexen() && old_hex.pinen() && old_hex.stable() => {}
        ClockSource::Lsi if r.lsi().read().stable() => {}
        ClockSource::Hsi | ClockSource::Hex | ClockSource::Lsi => {
            return Err(Error::InvalidEntryClock);
        }
        _ => return Err(Error::InvalidClockSource),
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
    #[cfg(rcc_lsi_sysclk)]
    let mut factory_lsi = if permanent_lsi {
        Some(FactoryLsi::capture(
            old_clock,
            old_sources,
            old_hsi,
            old_hex,
        )?)
    } else {
        None
    };

    // AWT can use either pad independently of PINMUX and HEXEN, including in
    // DeepSleep. Always inspect it, even for HSI-only init with no HEX request.
    // Its APB gate controls configuration access, not whether its source is used.
    let awt = <crate::peripherals::AWT as crate::rcc::SealedRccPeripheral>::RCC_INFO
        .inspect_for_init(cs, config.timeout, || pac::AWT.cr().read())
        .map_err(|_| Error::RetainedClockInspectionTimeout)?;
    let mut awt_hex = false;
    if awt.en() {
        use pac::awt::vals::Source;
        match awt.src() {
            Source::Hsiosc if needs_trim => return Err(Error::HsiClockInUse),
            Source::Hsiosc | Source::Lsi => {}
            Source::HexPb0 => {
                awt_hex = true;
                clocks.reserved_hex_inputs |= 1;
            }
            Source::HexPb1 => {
                awt_hex = true;
                clocks.reserved_hex_inputs |= 2;
            }
            Source::Etr if config.hex.is_some() => return Err(Error::HexClockInUse),
            Source::Etr => {}
            _ => return Err(Error::InvalidRetainedClockSource),
        }
    }
    if needs_trim
        && pac::LVD.cr0().read().en()
        && pac::LVD.cr1().read().flten()
        && pac::LVD.cr1().read().fltclk()
    {
        return Err(Error::HsiClockInUse);
    }
    let preserve_hex = config.hex.is_some() && awt_hex;
    if let Some(hex) = config.hex.filter(|_| preserve_hex) {
        // Global HEX enable/input-gate writes are not proved harmless to AWT.
        // Only exact inherited enabled/ready/configured pad reuse is admitted.
        if !old_sources.hexen()
            || !old_hex.stable()
            || !old_hex.pinen()
            || old_hex.pinmux() != hex.input
            || !crate::rcc_hex_pin_matches(hex.input, config.timeout, cs)?
        {
            return Err(Error::HexClockInUse);
        }
    }
    if config.hex.is_none() && old_sources.hexen() {
        clocks.reserved_hex_inputs |= 1 << old_hex.pinmux().to_bits();
    }

    <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO
        .enable_with_cs_readback(
            cs,
            crate::rcc::Readback::Poll {
                attempts: config.timeout,
                spin: true,
            },
        )
        .map_err(|_| Error::FlashClockTimeout)?;
    set_flash_latency(crate::RCC_INITIAL_FLASH_WAIT, config.timeout)?;
    let guard_hclk = old_clock.hclkprs().max(AHBPrescaler::Div4 as u8);
    let guard_pclk = old_clock.pclkprs().max(APBPrescaler::Div8 as u8);
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_hclkprs(guard_hclk);
        w.set_pclkprs(guard_pclk);
    });
    poll(
        || {
            let v = r.cr0().read();
            v.hclkprs() == guard_hclk && v.pclkprs() == guard_pclk
        },
        config.timeout,
        Error::ClockConfigurationTimeout,
    )?;
    barrier();

    #[cfg(rcc_lsi_sysclk)]
    if let Some(lsi) = factory_lsi.as_mut() {
        // Advance only the P2 guard fields from the P1 entry identity.
        lsi.clock.set_hclkprs(guard_hclk);
        lsi.clock.set_pclkprs(guard_pclk);
        lsi.prepare(config.timeout, cs)?;
    }

    // Start the unchanged HSI before leaving the inherited source. CR1 bits
    // 15:4 and bit 2 are reserved here: no copied CCS, LSE or PLL writes.
    r.cr1().modify(|w| {
        w.set_key(0x5a5a);
        w.set_hsien(true);
        #[cfg(rcc_lsi_sysclk)]
        if permanent_lsi {
            w.set_lsien(true);
        }
    });
    poll(
        || {
            let sources = r.cr1().read();
            sources.hsien() && (!permanent_lsi || sources.lsien())
        },
        config.timeout,
        Error::ClockConfigurationTimeout,
    )?;
    #[cfg(rcc_lsi_sysclk)]
    if let Some(lsi) = factory_lsi.as_mut() {
        let mut expected = lsi.sources;
        expected.set_hsien(true);
        expected.set_lsien(true);
        poll(
            || {
                let mut sources = r.cr1().read();
                sources.set_key(0);
                sources.0 == expected.0
            },
            config.timeout,
            Error::LsiConfigurationTimeout,
        )?;
        lsi.wait_ready(config.timeout)?;
        lsi.ready = true;
        lsi.check_consumers(config.timeout, cs)?;
    }
    poll(
        || r.hsi().read().stable(),
        config.timeout,
        Error::HsiTimeout,
    )?;
    select_source(ClockSource::Hsi, config.timeout)?;
    if needs_trim {
        if !permanent_lsi {
            r.cr1().modify(|w| {
                w.set_key(0x5a5a);
                w.set_lsien(true);
            });
            poll(
                || r.cr1().read().lsien(),
                config.timeout,
                Error::ClockConfigurationTimeout,
            )?;
            poll(
                || r.lsi().read().stable(),
                config.timeout,
                Error::LsiTimeout,
            )?;
        }
        #[cfg(rcc_lsi_sysclk)]
        if let Some(lsi) = factory_lsi.as_ref() {
            lsi.wait_ready(config.timeout)?;
        }
        select_source(ClockSource::Lsi, config.timeout)?;
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hsien(false);
            #[cfg(rcc_lsi_sysclk)]
            if permanent_lsi {
                w.set_lsien(true);
            }
        });
        poll(
            || {
                let sources = r.cr1().read();
                !sources.hsien() && (!permanent_lsi || sources.lsien())
            },
            config.timeout,
            Error::HsiStopTimeout,
        )?;
        poll(
            || !r.hsi().read().stable(),
            config.timeout,
            Error::HsiStopTimeout,
        )?;
        r.hsi().modify(|w| w.set_trim(trim));
        poll(
            || r.hsi().read().trim() == trim,
            config.timeout,
            Error::ClockConfigurationTimeout,
        )?;
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hsien(true);
            #[cfg(rcc_lsi_sysclk)]
            if permanent_lsi {
                w.set_lsien(true);
            }
        });
        poll(
            || {
                let sources = r.cr1().read();
                sources.hsien() && (!permanent_lsi || sources.lsien())
            },
            config.timeout,
            Error::ClockConfigurationTimeout,
        )?;
        poll(
            || r.hsi().read().stable(),
            config.timeout,
            Error::HsiTimeout,
        )?;
        select_source(ClockSource::Hsi, config.timeout)?;
        if !permanent_lsi && !old_sources.lsien() {
            r.cr1().modify(|w| {
                w.set_key(0x5a5a);
                w.set_lsien(false);
            });
            poll(
                || !r.cr1().read().lsien(),
                config.timeout,
                Error::LsiRestoreTimeout,
            )?;
        }
    }
    r.hsi().modify(|w| w.set_div(config.hsi.div as u8));
    poll(
        || r.hsi().read().div() == config.hsi.div as u8,
        config.timeout,
        Error::ClockConfigurationTimeout,
    )?;
    poll(
        || r.hsi().read().stable(),
        config.timeout,
        Error::HsiTimeout,
    )?;

    if let Some(hex) = config.hex.filter(|_| !preserve_hex) {
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hexen(false);
            #[cfg(rcc_lsi_sysclk)]
            if permanent_lsi {
                w.set_lsien(true);
            }
        });
        poll(
            || {
                let sources = r.cr1().read();
                !sources.hexen() && (!permanent_lsi || sources.lsien())
            },
            config.timeout,
            Error::HexStopTimeout,
        )?;
        poll(
            || !r.hex().read().stable(),
            config.timeout,
            Error::HexStopTimeout,
        )?;
        #[cfg(rcc_lsi_sysclk)]
        if let Some(lsi) = factory_lsi.as_ref() {
            lsi.check_consumers(config.timeout, cs)?;
            lsi.wait_ready(config.timeout)?;
        }
        crate::rcc_configure_hex_pin(hex.input, config.timeout, cs)?;
        #[cfg(rcc_lsi_sysclk)]
        if let Some(lsi) = factory_lsi.as_mut() {
            if let Some(consumers) = lsi.consumers.as_mut() {
                // The existing HEX path deliberately leaves this bank enabled.
                // Advance only that verified own write, never a fresh snapshot.
                consumers.1[crate::RCC_LSI_HEX_GPIO_GATE_INDEX] = true;
            }
        }
        r.hex().modify(|w| {
            w.set_pinmux(hex.input);
            w.set_pinen(true);
        });
        poll(
            || {
                let v = r.hex().read();
                v.pinmux() == hex.input && v.pinen()
            },
            config.timeout,
            Error::ClockConfigurationTimeout,
        )?;
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hexen(true);
            #[cfg(rcc_lsi_sysclk)]
            if permanent_lsi {
                w.set_lsien(true);
            }
        });
        poll(
            || {
                let sources = r.cr1().read();
                sources.hexen() && (!permanent_lsi || sources.lsien())
            },
            config.timeout,
            Error::ClockConfigurationTimeout,
        )?;
        poll(
            || r.hex().read().stable(),
            config.timeout,
            Error::HexTimeout,
        )?;
    }
    let source = match config.sys {
        Sysclk::HSI => ClockSource::Hsi,
        Sysclk::HEX => ClockSource::Hex,
        #[cfg(rcc_lsi_sysclk)]
        Sysclk::LSI => ClockSource::Lsi,
    };
    #[cfg(rcc_lsi_sysclk)]
    if let Some(lsi) = factory_lsi.as_ref() {
        lsi.check_consumers(config.timeout, cs)?;
        lsi.wait_ready(config.timeout)?;
    }
    select_source(source, config.timeout)?;
    // Weaken final buses only after the requested source is selected/read back.
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        #[cfg(rcc_lsi_sysclk)]
        if permanent_lsi {
            w.set_sysclk(ClockSource::Lsi);
        }
        w.set_hclkprs(config.ahb_pre as u8);
        w.set_pclkprs(config.apb_pre as u8);
    });
    poll(
        || {
            let v = r.cr0().read();
            v.sysclk() == source
                && v.hclkprs() == config.ahb_pre as u8
                && v.pclkprs() == config.apb_pre as u8
        },
        config.timeout,
        Error::ClockConfigurationTimeout,
    )?;
    barrier();
    poll(
        || {
            let v = r.hsi().read();
            v.div() == config.hsi.div as u8 && v.trim() == trim
        },
        config.timeout,
        Error::ClockConfigurationTimeout,
    )?;
    poll(
        || r.hsi().read().stable(),
        config.timeout,
        Error::HsiTimeout,
    )?;
    if config.hex.is_some() {
        poll(
            || r.hex().read().stable(),
            config.timeout,
            Error::HexTimeout,
        )?;
    }
    #[cfg(rcc_lsi_sysclk)]
    if let Some(lsi) = factory_lsi.as_ref() {
        lsi.verify_final(config, trim, awt, cs)?;
        // Retained factory HSI is included without selecting it on the final
        // buses. This Flash margin is not an automatic clock-loss fallback.
        let retained_hsi = crate::rcc::ClockBounds::hsi(config.hsi.div.divisor())
            .divided_by(config.ahb_pre.divisor())
            .maximum()
            .0;
        let upper_hclk = clocks.hclk_bounds().maximum().0.max(retained_hsi);
        let wait = (upper_hclk - 1) / crate::RCC_FLASH_WAIT_STEP_HZ;
        set_flash_latency(wait, config.timeout)?;
        lsi.verify_final(config, trim, awt, cs)?;
        if pac::FLASH.cr2().read().wait() != wait as u8 {
            return Err(Error::FlashLatencyTimeout);
        }
        return Ok(clocks);
    }
    set_flash_latency(
        (clocks.hclk_bounds().maximum().0 - 1) / crate::RCC_FLASH_WAIT_STEP_HZ,
        config.timeout,
    )?;
    if r.cr0().read().sysclk() != source {
        return Err(Error::ClockSwitchTimeout);
    }
    Ok(clocks)
}
