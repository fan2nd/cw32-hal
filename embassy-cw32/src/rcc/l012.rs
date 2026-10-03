//! Checked reset-clock bring-up for CW32L012C8.
//!
//! The default remains HSI /24 with undivided AHB/APB: nominal 4 MHz.
//! An explicit HSI /1 profile raises SYSCLK/HCLK to nominal 96 MHz after
//! factory trim and the vendor-prescribed Flash wait-state change. APB /2 is
//! available for a 48 MHz PCLK. VDD must be >=1.8 V for 96 MHz operation.
//! Frequencies are nominal, not a measurement or oscillator-accuracy guarantee.

use crate::pac::sysctrl::regs;
use crate::pac::{self, sysctrl};

/// The two audited direct-reset HSI configurations. Other clock trees are rejected.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HsiFrequency {
    #[default]
    Mhz4,
    /// HSI /1, AHB /1; requires VDD >=1.8 V and the supply/temperature limits of
    /// the CW32L012 datasheet. Does not make a peripheral's own clock legal.
    Mhz96,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PclkDivider {
    #[default]
    Div1,
    Div2,
}

/// Bounded reset-clock validation followed by an optional audited HSI change.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    pub hsi_frequency: HsiFrequency,
    /// Applied before raising HSI, so APB never overshoots its requested clock.
    pub pclk_divider: PclkDivider,
    /// Maximum HSI stability polls after applying vendor trim.
    pub hsi_stabilization_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            hsi_frequency: HsiFrequency::Mhz4,
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
    fn flash_wait(&mut self) -> regs::Cr2;
    fn set_flash_wait(&mut self, value: regs::Cr2);
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
fn set_pclk_divider(io: &mut impl HighSpeedIo, divider: PclkDivider) -> Result<(), ClockError> {
    let encoding = u8::from(divider == PclkDivider::Div2);
    let mut value = io.bus();
    value.set_key((pac::SYSCTRL_KEY >> 16) as u16);
    value.set_pclkprs(encoding);
    io.set_bus(value);
    if io.bus().pclkprs() != encoding {
        return Err(ClockError::ClockReadbackMismatch);
    }
    Ok(())
}
fn raise_hsi_to_96(io: &mut impl HighSpeedIo, polls: u32) -> Result<(), ClockError> {
    // FLASHWAIT=3 precedes HSI.DIV=1. Preserve SWD/fault/wake/reserved fields.
    let mut wait = io.flash_wait();
    wait.set_key((pac::SYSCTRL_KEY >> 16) as u16);
    wait.set_flashwait(3);
    io.set_flash_wait(wait);
    if io.flash_wait().flashwait() != 3 {
        return Err(ClockError::FlashWaitReadbackMismatch);
    }
    let mut hsi = io.hsi();
    hsi.set_div(1);
    io.set_hsi(hsi);
    for _ in 0..polls {
        let readback = io.hsi();
        if readback.stable() {
            return if readback.div() == 1 && readback.trim() == hsi.trim() {
                Ok(())
            } else {
                Err(ClockError::ClockReadbackMismatch)
            };
        }
        core::hint::spin_loop();
    }
    Err(ClockError::HsiNotStable)
}

fn validate_configuration(
    cr0: regs::Cr0,
    cr1: regs::Cr1,
    hsi: regs::Hsi,
) -> Result<(), ClockError> {
    if cr0.sysclk() != 0 {
        return Err(ClockError::SystemClockNotHsi);
    }
    if cr0.hclkprs() != 0 || cr0.pclkprs() != 0 {
        return Err(ClockError::BusPrescalerNotReset);
    }
    if hsi.div() != regs::Hsi(sysctrl::HSI_DIV24).div() {
        return Err(ClockError::HsiDividerNotReset);
    }
    if !cr1.hsien() {
        return Err(ClockError::HsiDisabled);
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
    // SAFETY: the caller guarantees this target and exclusive startup access.
    let (cr0, cr1, hsi) = (
        pac::SYSCTRL.cr0().read(),
        pac::SYSCTRL.cr1().read(),
        pac::SYSCTRL.hsi().read(),
    );
    validate_configuration(cr0, cr1, hsi)?;

    // The vendor startup uses halfword reads from these exact aligned addresses.
    // SAFETY: readable device factory calibration memory is a caller requirement.
    let hsi_trim = trim_or_fallback(
        unsafe { core::ptr::read_volatile(pac::HSI_TRIM_ADDRESS as *const u16) },
        u32::from(regs::Hsi(u32::MAX).trim()),
        pac::HSI_TRIM_FALLBACK,
    );
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
    if config.pclk_divider != PclkDivider::Div1 {
        set_pclk_divider(&mut HardwareHighSpeed, config.pclk_divider)?;
    }
    let divisor = if config.pclk_divider == PclkDivider::Div2 {
        2
    } else {
        1
    };
    if config.hsi_frequency == HsiFrequency::Mhz96 {
        raise_hsi_to_96(&mut HardwareHighSpeed, config.hsi_stabilization_limit)?;
        Ok(Clocks {
            sysclk: 96_000_000,
            hclk: 96_000_000,
            pclk: 96_000_000 / divisor,
        })
    } else {
        Ok(Clocks {
            sysclk: pac::RESET_SYSCLK_HZ,
            hclk: pac::RESET_HCLK_HZ,
            pclk: pac::RESET_PCLK_HZ / divisor,
        })
    }
}

/// Internal clock/reset implementation generated from audited metadata.
/// Shared register updates occur inside a Cortex-M0+ critical section (no CAS).
/// Drivers do not disable shared clock gates when dropped.
/// A reset shared with another singleton is intentionally skipped; drivers must
/// initialize their own registers without disturbing neighboring instances.
pub(crate) trait PeripheralClock {
    fn enable_and_reset();
}
include!(concat!(env!("OUT_DIR"), "/_generated_peripheral_clocks.rs"));
