//! Audited HSI startup clock tree for CW32F030.
//! Factory-trimmed 48 MHz oscillator, documented HSI and AHB/APB dividers.
//! The default remains HSI /6 and undivided buses: nominal 8 MHz.
//! Clock switching is startup-only; FLASH wait states precede faster HSI.

use crate::pac::sysctrl::regs;
use crate::pac::{self, sysctrl};

use super::{HclkDivider, PclkDivider};

/// HSI oscillator divider, as documented by the selected variant's manual.
/// Non-integral nominal frequencies are reported rounded down to integer Hz.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HsiDivider {
    Div1,
    Div2,
    Div4,
    #[default]
    Div6,
    Div8,
    Div10,
    Div12,
    Div14,
    Div16,
}
impl HsiDivider {
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
    pub const fn hz(self) -> u32 {
        48000000 / self.divisor()
    }
    const fn encoding(self) -> pac::sysctrl::vals::HsiDiv {
        match self {
            Self::Div1 => pac::sysctrl::vals::HsiDiv::DIV1,
            Self::Div2 => pac::sysctrl::vals::HsiDiv::DIV2,
            Self::Div4 => pac::sysctrl::vals::HsiDiv::DIV4,
            Self::Div6 => pac::sysctrl::vals::HsiDiv::DIV6,
            Self::Div8 => pac::sysctrl::vals::HsiDiv::DIV8,
            Self::Div10 => pac::sysctrl::vals::HsiDiv::DIV10,
            Self::Div12 => pac::sysctrl::vals::HsiDiv::DIV12,
            Self::Div14 => pac::sysctrl::vals::HsiDiv::DIV14,
            Self::Div16 => pac::sysctrl::vals::HsiDiv::DIV16,
        }
    }
}

/// Checked HSI and bus configuration.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    pub hsi_divider: HsiDivider,
    pub hclk_divider: HclkDivider,
    pub pclk_divider: PclkDivider,
    /// Maximum HSI stability polls after factory trim.
    pub hsi_stabilization_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            hsi_divider: HsiDivider::Div6,
            hclk_divider: HclkDivider::Div1,
            pclk_divider: PclkDivider::Div1,
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

/// A state that this deliberately limited clock implementation cannot accept.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockError {
    SystemClockNotHsi,
    BusPrescalerNotReset,
    HsiDividerNotReset,
    HsiDisabled,
    HsiNotStable,
    TrimReadbackMismatch,
    FlashWaitReadbackMismatch,
    ClockReadbackMismatch,
}

trait HighSpeedIo {
    fn bus(&mut self) -> regs::Cr0;
    fn set_bus(&mut self, value: regs::Cr0);
    fn flash_wait(&mut self) -> pac::flash::regs::Cr2;
    fn set_flash_wait(&mut self, value: pac::flash::regs::Cr2);
    fn hsi(&mut self) -> regs::Hsi;
    fn set_hsi(&mut self, value: regs::Hsi);
}
struct HardwareHighSpeed;
impl HighSpeedIo for HardwareHighSpeed {
    fn bus(&mut self) -> regs::Cr0 {
        pac::SYSCTRL.cr0().read()
    }
    fn set_bus(&mut self, value: regs::Cr0) {
        pac::SYSCTRL.cr0().write_value(value);
    }
    fn flash_wait(&mut self) -> pac::flash::regs::Cr2 {
        pac::FLASH.cr2().read()
    }
    fn set_flash_wait(&mut self, value: pac::flash::regs::Cr2) {
        pac::FLASH.cr2().write_value(value);
    }
    fn hsi(&mut self) -> regs::Hsi {
        pac::SYSCTRL.hsi().read()
    }
    fn set_hsi(&mut self, value: regs::Hsi) {
        pac::SYSCTRL.hsi().write_value(value);
    }
}

impl Config {
    /// Compute nominal requested clocks without touching hardware.
    pub fn clocks(self) -> Clocks {
        configured_clocks(self)
    }
}

fn configured_clocks(config: Config) -> Clocks {
    let sysclk = config.hsi_divider.hz();
    let hclk = sysclk >> config.hclk_divider as u8;
    let pclk = hclk >> config.pclk_divider as u8;
    Clocks { sysclk, hclk, pclk }
}

fn apply_configuration(io: &mut impl HighSpeedIo, config: Config) -> Result<Clocks, ClockError> {
    let clocks = configured_clocks(config);
    // Reduce buses first: the subsequent HSI change never overshoots final HCLK/PCLK.
    if config.hclk_divider != HclkDivider::Div1 || config.pclk_divider != PclkDivider::Div1 {
        let mut bus = io.bus();
        bus.set_key((pac::SYSCTRL_KEY >> 16) as u16);
        bus.set_hclkprs(config.hclk_divider.register_value());
        bus.set_pclkprs(config.pclk_divider.register_value());
        io.set_bus(bus);
        let bus = io.bus();
        if bus.hclkprs() != config.hclk_divider.register_value()
            || bus.pclkprs() != config.pclk_divider.register_value()
            || bus.sysclk() != pac::sysctrl::vals::Cr0Sysclk::HSI
        {
            return Err(ClockError::ClockReadbackMismatch);
        }
    }
    if config.hsi_divider != HsiDivider::Div6 {
        // Both old reset HCLK and target HCLK are safe at this wait count.
        // RMW retains cache/prefetch, SWD and reserved fields.
        let wait_count = ((clocks.hclk.max(pac::RESET_HCLK_HZ) - 1) / 24_000_000) as u8;
        let mut wait = io.flash_wait();
        wait.set_key((pac::SYSCTRL_KEY >> 16) as u16);
        wait.set_wait(wait_count);
        io.set_flash_wait(wait);
        if io.flash_wait().wait() != wait_count {
            return Err(ClockError::FlashWaitReadbackMismatch);
        }
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
    Ok(clocks)
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
    if hsi.div() != regs::Hsi(sysctrl::HSI_DIV6).div() {
        return Err(ClockError::HsiDividerNotReset);
    }
    if !cr1.hsien() {
        return Err(ClockError::HsiDisabled);
    }
    Ok(())
}

// F030 SystemInit assigns the factory halfword directly to the trim bitfield.
// The register width discards upper bits. Do not transplant L012 fallback codes.
fn trim_code(word: u16, mask: u32) -> u32 {
    u32::from(word) & mask
}

/// Verify the supported clock configuration and load HSI/LSI calibration.
///
/// Returns nominal 8 MHz for SYSCLK, HCLK and PCLK only after the actual
/// source/divider/enable registers and HSI stability have been checked. An
/// incompatible bootloader clock configuration is rejected without changing it.
/// After trim writes, a failed check returns an error but leaves the trim values
/// applied. The bounded stabilization loop is an iteration limit, not a timeout
/// calibrated to wall-clock time. SWD and Flash wait-state settings are unchanged.
///
/// # Safety
/// Call only on a genuine CW32F030 target, during exclusive early startup before
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
    // SAFETY: the caller guarantees this target and exclusive startup access.
    let (cr0, cr1, hsi) = (
        pac::SYSCTRL.cr0().read(),
        pac::SYSCTRL.cr1().read(),
        pac::SYSCTRL.hsi().read(),
    );
    validate_configuration(cr0, cr1, hsi)?;

    // The vendor startup uses halfword reads from these exact aligned addresses.
    // SAFETY: readable device factory calibration memory is a caller requirement.
    let hsi_trim = trim_code(
        unsafe { core::ptr::read_volatile(pac::HSI_TRIM_ADDRESS as *const u16) },
        u32::from(regs::Hsi(u32::MAX).trim()),
    );
    // SAFETY: as above; the LSI code follows the HSI halfword.
    let lsi_trim = trim_code(
        unsafe { core::ptr::read_volatile(pac::LSI_TRIM_ADDRESS as *const u16) },
        u32::from(regs::Lsi(u32::MAX).trim()),
    );

    // SAFETY: ordinary R/W fields, preserving divider and wait settings; unlike
    // CR0/CR1, HSI and LSI do not contain a keyed upper halfword.
    let mut hsi = pac::SYSCTRL.hsi().read();
    let mut lsi = pac::SYSCTRL.lsi().read();
    hsi.set_trim(hsi_trim as _);
    lsi.set_trim(lsi_trim as _);
    pac::SYSCTRL.hsi().write_value(hsi);
    pac::SYSCTRL.lsi().write_value(lsi);

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
    if u32::from(hsi.trim()) != hsi_trim || u32::from(lsi.trim()) != lsi_trim {
        return Err(ClockError::TrimReadbackMismatch);
    }
    if config.hsi_divider != HsiDivider::Div6 {
        // RM 7.4: FLASH configuration clock must be enabled before WAIT writes.
        use super::PeripheralClock;
        let mut flash_clock = crate::peripherals::FLASH::acquire_no_reset();
        flash_clock.pin();
    }
    apply_configuration(&mut HardwareHighSpeed, config)
}
