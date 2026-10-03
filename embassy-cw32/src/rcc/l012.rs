//! Audited direct-reset startup clock tree for CW32L012.
//! Factory-trimmed 96 MHz oscillator, documented HSI and AHB/APB dividers.
//! The default remains HSI /24 and undivided buses: nominal 4 MHz.
//! Clock switching is startup-only; 96 MHz requires VDD >=1.8 V.

use crate::pac::sysctrl::regs;
use crate::pac::{self, sysctrl};

use super::{HclkDivider, HseConfig, HseMode, PclkDivider};

/// HSI oscillator divider, as documented by the selected variant's manual.
/// Non-integral nominal frequencies are reported rounded down to integer Hz.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HsiDivider {
    Div1,
    Div2,
    Div3,
    Div4,
    Div5,
    Div6,
    Div7,
    Div8,
    Div9,
    Div10,
    Div12,
    Div16,
    Div20,
    #[default]
    Div24,
    Div28,
    Div32,
}
impl HsiDivider {
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
    pub const fn hz(self) -> u32 {
        96000000 / self.divisor()
    }
    const fn encoding(self) -> pac::sysctrl::vals::HsiDiv {
        match self {
            Self::Div1 => pac::sysctrl::vals::HsiDiv::DIV1,
            Self::Div2 => pac::sysctrl::vals::HsiDiv::DIV2,
            Self::Div3 => pac::sysctrl::vals::HsiDiv::DIV3,
            Self::Div4 => pac::sysctrl::vals::HsiDiv::DIV4,
            Self::Div5 => pac::sysctrl::vals::HsiDiv::DIV5,
            Self::Div6 => pac::sysctrl::vals::HsiDiv::DIV6,
            Self::Div7 => pac::sysctrl::vals::HsiDiv::DIV7,
            Self::Div8 => pac::sysctrl::vals::HsiDiv::DIV8,
            Self::Div9 => pac::sysctrl::vals::HsiDiv::DIV9,
            Self::Div10 => pac::sysctrl::vals::HsiDiv::DIV10,
            Self::Div12 => pac::sysctrl::vals::HsiDiv::DIV12,
            Self::Div16 => pac::sysctrl::vals::HsiDiv::DIV16,
            Self::Div20 => pac::sysctrl::vals::HsiDiv::DIV20,
            Self::Div24 => pac::sysctrl::vals::HsiDiv::DIV24,
            Self::Div28 => pac::sysctrl::vals::HsiDiv::DIV28,
            Self::Div32 => pac::sysctrl::vals::HsiDiv::DIV32,
        }
    }
}

/// System clock chosen once during direct-reset initialization.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ClockSource {
    #[default]
    Hsi,
    Hse(HseConfig),
}

/// Checked startup source and bus configuration.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    pub source: ClockSource,
    /// Applies to HSI SYSCLK. HSE leaves HSI at its reset divide-by-24 setting.
    pub hsi_divider: HsiDivider,
    pub hclk_divider: HclkDivider,
    pub pclk_divider: PclkDivider,
    /// Caller-asserted minimum board VDD in millivolts, not a measured voltage.
    /// Defaults to 1800 mV, retaining the earlier full-speed supply requirement.
    pub supply_voltage_mv: u16,
    /// Maximum HSI stability polls after factory trim. Zero is invalid.
    pub hsi_stabilization_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            source: ClockSource::Hsi,
            hsi_divider: HsiDivider::Div24,
            hclk_divider: HclkDivider::Div1,
            pclk_divider: PclkDivider::Div1,
            supply_voltage_mv: 1800,
            hsi_stabilization_limit: 0xffff,
        }
    }
}
static CLOCKS: critical_section::Mutex<core::cell::Cell<Option<Clocks>>> =
    critical_section::Mutex::new(core::cell::Cell::new(None));
pub(crate) fn set_clocks(clocks: Clocks) {
    critical_section::with(|cs| CLOCKS.borrow(cs).set(Some(clocks)));
}
/// Verified nominal clocks. Panics before HAL initialization.
pub fn clocks() -> Clocks {
    critical_section::with(|cs| CLOCKS.borrow(cs).get().expect("HAL not initialized"))
}

/// Nominal clock frequencies after checked reset-clock initialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clocks {
    pub(crate) sysclk: u32,
    pub(crate) hclk: u32,
    pub(crate) pclk: u32,
}

impl Clocks {
    pub const fn sysclk_hz(&self) -> u32 {
        self.sysclk
    }
    pub const fn hclk_hz(&self) -> u32 {
        self.hclk
    }
    pub const fn pclk_hz(&self) -> u32 {
        self.pclk
    }
}

/// Invalid requests are rejected before MMIO; later errors leave completed
/// writes in place. No rollback or initialized clock tree is promised on error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockError {
    InvalidPollLimit,
    InvalidSupplyVoltage,
    ClockOutOfRange,
    InvalidHseConfig,
    FractionalClock,
    SystemClockNotHsi,
    BusPrescalerNotReset,
    HsiDividerNotReset,
    HsiDisabled,
    HsiNotStable,
    ExternalClockNotReset,
    HseNotStable,
    HsePinReadbackMismatch,
    HseReadbackMismatch,
    TrimReadbackMismatch,
    FlashWaitReadbackMismatch,
    ClockReadbackMismatch,
}
trait HighSpeedIo {
    fn control(&mut self) -> regs::Cr1;
    fn set_control(&mut self, value: regs::Cr1);
    fn hse(&mut self) -> regs::Hse;
    fn set_hse(&mut self, value: regs::Hse);
    fn prepare_hse_pins(&mut self, mode: HseMode) -> Result<(), ClockError>;

    fn bus(&mut self) -> regs::Cr0;
    fn set_bus(&mut self, value: regs::Cr0);
    fn flash_wait(&mut self) -> regs::Cr2;
    fn set_flash_wait(&mut self, value: regs::Cr2);
    fn hsi(&mut self) -> regs::Hsi;
    fn set_hsi(&mut self, value: regs::Hsi);
}
struct HardwareHighSpeed;
impl HighSpeedIo for HardwareHighSpeed {
    fn control(&mut self) -> regs::Cr1 {
        pac::SYSCTRL.cr1().read()
    }
    fn set_control(&mut self, value: regs::Cr1) {
        pac::SYSCTRL.cr1().write_value(value);
    }
    fn hse(&mut self) -> regs::Hse {
        pac::SYSCTRL.hse().read()
    }
    fn set_hse(&mut self, value: regs::Hse) {
        pac::SYSCTRL.hse().write_value(value);
    }
    fn prepare_hse_pins(&mut self, mode: HseMode) -> Result<(), ClockError> {
        super::configure_hse_pins(mode)
    }

    fn bus(&mut self) -> regs::Cr0 {
        pac::SYSCTRL.cr0().read()
    }
    fn set_bus(&mut self, value: regs::Cr0) {
        pac::SYSCTRL.cr0().write_value(value);
    }
    fn flash_wait(&mut self) -> regs::Cr2 {
        pac::SYSCTRL.cr2().read()
    }
    fn set_flash_wait(&mut self, value: regs::Cr2) {
        pac::SYSCTRL.cr2().write_value(value);
    }
    fn hsi(&mut self) -> regs::Hsi {
        pac::SYSCTRL.hsi().read()
    }
    fn set_hsi(&mut self, value: regs::Hsi) {
        pac::SYSCTRL.hsi().write_value(value);
    }
}

impl Config {
    /// Compute requested nominal clocks, panicking for an invalid request.
    /// Use `validate` for a fallible check without hardware access.
    pub fn clocks(self) -> Clocks {
        self.validate()
            .expect("invalid startup clock configuration")
    }

    pub const fn uses_hse(self) -> bool {
        self.hse_config().is_some()
    }
    pub const fn uses_hse_output(self) -> bool {
        matches!(self.hse_mode(), Some(HseMode::Crystal))
    }
    pub const fn hse_mode(self) -> Option<HseMode> {
        match self.hse_config() {
            Some(hse) => Some(hse.mode),
            None => None,
        }
    }
    const fn hse_config(self) -> Option<HseConfig> {
        match self.source {
            ClockSource::Hse(hse) => Some(hse),
            _ => None,
        }
    }
    const fn uses_hsi_divider(self) -> bool {
        matches!(self.source, ClockSource::Hsi)
    }
    fn frequency_ratio(self) -> (u64, u64) {
        match self.source {
            ClockSource::Hsi => (96000000, self.hsi_divider.divisor() as u64),
            ClockSource::Hse(hse) => (hse.frequency_hz as u64, 1),
        }
    }
    /// Validate every requested source, voltage, bus and poll setting before MMIO.
    /// HSI retains the historical rounded-down integer-Hz report. New HSE/PLL
    /// profiles require integral SYSCLK/HCLK/PCLK so timebase checks stay exact.
    pub fn validate(self) -> Result<Clocks, ClockError> {
        if self.hsi_stabilization_limit == 0 {
            return Err(ClockError::InvalidPollLimit);
        }
        if !(1700..=5500).contains(&self.supply_voltage_mv) {
            return Err(ClockError::InvalidSupplyVoltage);
        }
        if let Some(hse) = self.hse_config() {
            if !hse.valid() {
                return Err(ClockError::InvalidHseConfig);
            }
        }
        let (num, den) = self.frequency_ratio();
        let hden = den << self.hclk_divider as u8;
        let pden = hden << self.pclk_divider as u8;
        let max_bus = if self.supply_voltage_mv < 1800 {
            24_000_000
        } else {
            96000000
        };
        if num == 0 || num > 96000000 * den || num > max_bus * hden || num > max_bus * pden {
            return Err(ClockError::ClockOutOfRange);
        }
        if !matches!(self.source, ClockSource::Hsi)
            && (num % den != 0 || num % hden != 0 || num % pden != 0)
        {
            return Err(ClockError::FractionalClock);
        }
        Ok(Clocks {
            sysclk: (num / den) as u32,
            hclk: (num / hden) as u32,
            pclk: (num / pden) as u32,
        })
    }
}

fn apply_configuration(io: &mut impl HighSpeedIo, config: Config) -> Result<Clocks, ClockError> {
    let clocks = config.validate()?;
    // All source configuration is completed while still running reset HSI.
    // Reduce buses first, then protect Flash for the largest transitional HCLK.
    if config.hclk_divider != HclkDivider::Div1 || config.pclk_divider != PclkDivider::Div1 {
        let mut bus = io.bus();
        bus.set_key((pac::SYSCTRL_KEY >> 16) as u16);
        bus.set_hclkprs(config.hclk_divider.register_value());
        bus.set_pclkprs(config.pclk_divider.register_value());
        io.set_bus(bus);
        check_bus(io, config, pac::sysctrl::vals::Cr0Sysclk::HSI)?;
    }
    let change_hsi = config.uses_hsi_divider() && config.hsi_divider != HsiDivider::Div24;
    if change_hsi || !matches!(config.source, ClockSource::Hsi) {
        let (num, den) = config.frequency_ratio();
        let target_hclk = num.div_ceil(den << config.hclk_divider as u8) as u32;
        let intermediate_hclk = if change_hsi {
            96000000u64.div_ceil((config.hsi_divider.divisor() as u64) << config.hclk_divider as u8)
                as u32
        } else {
            pac::RESET_HCLK_HZ
        };
        let wait_count =
            ((target_hclk.max(intermediate_hclk).max(pac::RESET_HCLK_HZ) - 1) / 24_000_000) as u8;
        let mut wait = io.flash_wait();
        wait.set_key((pac::SYSCTRL_KEY >> 16) as u16);
        wait.set_flashwait(wait_count);
        io.set_flash_wait(wait);
        if io.flash_wait().flashwait() != wait_count {
            return Err(ClockError::FlashWaitReadbackMismatch);
        }
    }
    if change_hsi {
        let mut hsi = io.hsi();
        hsi.set_div(config.hsi_divider.encoding());
        io.set_hsi(hsi);
        let readback = io.hsi();
        if readback.div() != config.hsi_divider.encoding() || readback.trim() != hsi.trim() {
            return Err(ClockError::ClockReadbackMismatch);
        }
        if !readback.stable() {
            return Err(ClockError::HsiNotStable);
        }
    }
    if let Some(hse) = config.hse_config() {
        start_hse(io, hse)?;
    }
    let source = match config.source {
        ClockSource::Hsi => pac::sysctrl::vals::Cr0Sysclk::HSI,
        ClockSource::Hse(_) => pac::sysctrl::vals::Cr0Sysclk::HSE,
    };
    if !matches!(config.source, ClockSource::Hsi) {
        // Check readiness again immediately before selection. CR0 has no
        // independent switch-acknowledge field; its documented source readback
        // plus enable/STABLE checks are the available startup evidence.
        check_sources(io, config)?;
        let mut bus = io.bus();
        bus.set_key((pac::SYSCTRL_KEY >> 16) as u16);
        bus.set_sysclk(source);
        io.set_bus(bus);
    }
    check_bus(io, config, source)?;
    check_sources(io, config)?;
    Ok(clocks)
}

fn check_bus(
    io: &mut impl HighSpeedIo,
    config: Config,
    source: pac::sysctrl::vals::Cr0Sysclk,
) -> Result<(), ClockError> {
    let bus = io.bus();
    if bus.sysclk() != source
        || bus.hclkprs() != config.hclk_divider.register_value()
        || bus.pclkprs() != config.pclk_divider.register_value()
    {
        return Err(ClockError::ClockReadbackMismatch);
    }
    Ok(())
}

fn check_sources(io: &mut impl HighSpeedIo, config: Config) -> Result<(), ClockError> {
    if !io.control().hsien() || !io.hsi().stable() {
        return Err(ClockError::HsiNotStable);
    }
    if let Some(hse) = config.hse_config() {
        let value = io.hse();
        if !io.control().hseen() || !value.stable() {
            return Err(ClockError::HseNotStable);
        }
        if !matches_hse(value, hse) {
            return Err(ClockError::HseReadbackMismatch);
        }
    }
    Ok(())
}

fn matches_hse(value: regs::Hse, config: HseConfig) -> bool {
    value.mode() == (config.mode == HseMode::Bypass)
        && value.waitcycle() == config.wait_cycles
        && value.driver() == config.drive
        && value.detcnt() == config.detection_cycles()
        && value.pdriver() == config.startup_drive
        && !value.digflt()
}

fn start_hse(io: &mut impl HighSpeedIo, config: HseConfig) -> Result<(), ClockError> {
    if io.control().hseen() || io.hse().stable() {
        return Err(ClockError::ExternalClockNotReset);
    }
    io.prepare_hse_pins(config.mode)?;
    let mut value = io.hse();
    value.set_mode(config.mode == HseMode::Bypass);
    value.set_waitcycle(config.wait_cycles);
    value.set_driver(config.drive);
    value.set_detcnt(config.detection_cycles());
    value.set_pdriver(config.startup_drive);
    value.set_digflt(false);
    io.set_hse(value);
    if !matches_hse(io.hse(), config) {
        return Err(ClockError::HseReadbackMismatch);
    }
    let mut control = io.control();
    control.set_key((pac::SYSCTRL_KEY >> 16) as u16);
    control.set_hseen(true);
    io.set_control(control);
    if io.control().0 & 0xffff != control.0 & 0xffff {
        return Err(ClockError::HseReadbackMismatch);
    }
    for _ in 0..config.stabilization_limit {
        if io.hse().stable() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(ClockError::HseNotStable)
}
fn validate_configuration(
    cr0: regs::Cr0,
    cr1: regs::Cr1,
    hsi: regs::Hsi,
) -> Result<(), ClockError> {
    if cr0.sysclk() != pac::sysctrl::vals::Cr0Sysclk::HSI {
        return Err(ClockError::SystemClockNotHsi);
    }
    if cr0.hclkprs() != pac::sysctrl::vals::Cr0Hclkprs::DIV1
        || cr0.pclkprs() != pac::sysctrl::vals::Cr0Pclkprs::DIV1
    {
        return Err(ClockError::BusPrescalerNotReset);
    }
    if hsi.div() != regs::Hsi(sysctrl::HSI_DIV24).div() {
        return Err(ClockError::HsiDividerNotReset);
    }
    if !cr1.hsien() {
        return Err(ClockError::HsiDisabled);
    }
    if cr1.hseen() {
        return Err(ClockError::ExternalClockNotReset);
    }
    Ok(())
}

fn trim_or_fallback(word: u16, mask: u32, fallback: u32) -> u32 {
    let trim = u32::from(word) & mask;
    if trim == mask {
        fallback
    } else {
        trim
    }
}

/// Verify the supported clock configuration and load HSI/LSI calibration.
///
/// Returns nominal 4 MHz for SYSCLK, HCLK and PCLK only after the actual
/// source/divider/enable registers and HSI stability have been checked. An
/// incompatible bootloader clock configuration is rejected without changing it.
/// After trim writes, a failed check returns an error but leaves the trim values
/// applied. The bounded stabilization loop is an iteration limit, not a timeout
/// calibrated to wall-clock time. SWD and Flash wait-state settings are unchanged.
///
/// # Safety
/// Call only on a genuine CW32L012 target, during exclusive early startup before
/// enabling interrupts, DMA or time-sensitive peripheral drivers. The caller
/// must own SYSCTRL and ensure no interrupt, debugger automation or concurrent
/// code can change the clock tree during or after initialization while the
/// returned frequencies are in use. The device's factory calibration region
/// must be readable. This function is not safe to invoke on a host or a different
/// chip. It does not establish Rust peripheral ownership for the caller.
pub unsafe fn init_reset_clock() -> Result<Clocks, ClockError> {
    unsafe { init(Config::default()) }
}
pub(crate) unsafe fn init(config: Config) -> Result<Clocks, ClockError> {
    config.validate()?;

    // SAFETY: the caller guarantees this target and exclusive startup access.
    let (cr0, cr1, hsi) = (
        pac::SYSCTRL.cr0().read(),
        pac::SYSCTRL.cr1().read(),
        pac::SYSCTRL.hsi().read(),
    );
    validate_configuration(cr0, cr1, hsi)?;
    if pac::SYSCTRL.hse().read().stable() {
        return Err(ClockError::ExternalClockNotReset);
    }

    // The vendor startup uses halfword reads from these exact aligned addresses.
    // SAFETY: readable device factory calibration memory is a caller requirement.
    let hsi_trim = trim_or_fallback(
        unsafe { core::ptr::read_volatile(pac::HSI_TRIM_ADDRESS as *const u16) },
        u32::from(regs::Hsi(u32::MAX).trim()),
        pac::HSI_TRIM_FALLBACK,
    );
    // Auto-started or retained LSI must not have its trim rewritten. The
    // startup audit inspects every documented request and restores config gates.
    let trim_lsi = super::lsi_trim_is_safe();
    // SAFETY: as above; the LSI code follows the HSI halfword.
    let lsi_trim = trim_or_fallback(
        unsafe { core::ptr::read_volatile(pac::LSI_TRIM_ADDRESS as *const u16) },
        u32::from(regs::Lsi(u32::MAX).trim()),
        pac::LSI_TRIM_FALLBACK,
    );

    // SAFETY: ordinary R/W fields, preserving divider and wait settings; unlike
    // CR0/CR1/AHBEN, HSI and LSI do not contain a keyed upper halfword.
    let mut hsi = pac::SYSCTRL.hsi().read();
    let mut lsi = pac::SYSCTRL.lsi().read();
    hsi.set_trim(hsi_trim as _);
    pac::SYSCTRL.hsi().write_value(hsi);
    if trim_lsi {
        lsi.set_trim(lsi_trim as _);
        pac::SYSCTRL.lsi().write_value(lsi);
    }

    let mut stable = false;
    for _ in 0..config.hsi_stabilization_limit {
        // SAFETY: read-only observation of the exclusively owned controller.
        if pac::SYSCTRL.hsi().read().stable() {
            stable = true;
            break;
        }
        core::hint::spin_loop();
    }
    if !stable {
        return Err(ClockError::HsiNotStable);
    }

    // SAFETY: as above; verify settings and both trim fields after modification.
    let (cr0, cr1, hsi, lsi) = (
        pac::SYSCTRL.cr0().read(),
        pac::SYSCTRL.cr1().read(),
        pac::SYSCTRL.hsi().read(),
        pac::SYSCTRL.lsi().read(),
    );
    validate_configuration(cr0, cr1, hsi)?;
    if u32::from(hsi.trim()) != hsi_trim || (trim_lsi && u32::from(lsi.trim()) != lsi_trim) {
        return Err(ClockError::TrimReadbackMismatch);
    }
    apply_configuration(&mut HardwareHighSpeed, config)
}
