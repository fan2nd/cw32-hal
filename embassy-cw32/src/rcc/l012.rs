//! Checked reset-clock bring-up for CW32L012C8.
//!
//! The default remains HSI /24 with undivided AHB/APB: nominal 4 MHz.
//! An explicit HSI /1 profile raises SYSCLK/HCLK to nominal 96 MHz after
//! factory trim and the vendor-prescribed Flash wait-state change. APB /2 is
//! available for a 48 MHz PCLK. VDD must be >=1.8 V for 96 MHz operation.
//! Frequencies are nominal, not a measurement or oscillator-accuracy guarantee.

use crate::pac::sysctrl::fields;
use crate::pac::{self, sysctrl, SYSCTRL_BASE};

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
    pub sysclk: u32,
    pub hclk: u32,
    pub pclk: u32,
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
    fn bus(&mut self) -> u32;
    fn set_bus(&mut self, word: u32);
    fn flash_wait(&mut self) -> u32;
    fn set_flash_wait(&mut self, word: u32);
    fn hsi(&mut self) -> u32;
    fn set_hsi(&mut self, word: u32);
}
struct HardwareHighSpeed;
impl HighSpeedIo for HardwareHighSpeed {
    fn bus(&mut self) -> u32 {
        unsafe { pac::read(SYSCTRL_BASE + sysctrl::CR0) }
    }
    fn set_bus(&mut self, word: u32) {
        unsafe { pac::write(SYSCTRL_BASE + sysctrl::CR0, word) }
    }
    fn flash_wait(&mut self) -> u32 {
        unsafe { pac::read(SYSCTRL_BASE + sysctrl::CR2) }
    }
    fn set_flash_wait(&mut self, word: u32) {
        unsafe { pac::write(SYSCTRL_BASE + sysctrl::CR2, word) }
    }
    fn hsi(&mut self) -> u32 {
        unsafe { pac::read(SYSCTRL_BASE + sysctrl::HSI) }
    }
    fn set_hsi(&mut self, word: u32) {
        unsafe { pac::write(SYSCTRL_BASE + sysctrl::HSI, word) }
    }
}
fn set_pclk_divider(io: &mut impl HighSpeedIo, divider: PclkDivider) -> Result<(), ClockError> {
    let encoding = u32::from(divider == PclkDivider::Div2);
    let old = io.bus();
    io.set_bus(
        fields::cr0::PCLKPRS.write(old & !pac::SYSCTRL_KEY_MASK, encoding) | pac::SYSCTRL_KEY,
    );
    if fields::cr0::PCLKPRS.read(io.bus()) != encoding {
        return Err(ClockError::ClockReadbackMismatch);
    }
    Ok(())
}
fn raise_hsi_to_96(io: &mut impl HighSpeedIo, polls: u32) -> Result<(), ClockError> {
    // Supplied official SDK SYSCTRL_HSI_Enable(DIV1): FLASHWAIT=3 must
    // precede HSI.DIV=1. Preserve SWD, fault, wake and reserved fields.
    let old = io.flash_wait();
    let wait = fields::cr2::FLASHWAIT.write(old & !pac::SYSCTRL_KEY_MASK, 3) | pac::SYSCTRL_KEY;
    io.set_flash_wait(wait);
    if fields::cr2::FLASHWAIT.read(io.flash_wait()) != 3 {
        return Err(ClockError::FlashWaitReadbackMismatch);
    }
    let hsi = fields::hsi::DIV.write(io.hsi(), 1);
    io.set_hsi(hsi);
    for _ in 0..polls {
        let readback = io.hsi();
        if fields::hsi::STABLE.read(readback) {
            return if fields::hsi::DIV.read(readback) == 1
                && fields::hsi::TRIM.read(readback) == fields::hsi::TRIM.read(hsi)
            {
                Ok(())
            } else {
                Err(ClockError::ClockReadbackMismatch)
            };
        }
        core::hint::spin_loop();
    }
    Err(ClockError::HsiNotStable)
}

fn validate_configuration(cr0: u32, cr1: u32, hsi: u32) -> Result<(), ClockError> {
    if fields::cr0::SYSCLK.read(cr0) != 0 {
        return Err(ClockError::SystemClockNotHsi);
    }
    if fields::cr0::HCLKPRS.read(cr0) != 0 || fields::cr0::PCLKPRS.read(cr0) != 0 {
        return Err(ClockError::BusPrescalerNotReset);
    }
    if fields::hsi::DIV.read(hsi) != fields::hsi::DIV.read(sysctrl::HSI_DIV24) {
        return Err(ClockError::HsiDividerNotReset);
    }
    if !fields::cr1::HSIEN.read(cr1) {
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
    let (cr0, cr1, hsi) = unsafe {
        (
            pac::read(SYSCTRL_BASE + sysctrl::CR0),
            pac::read(SYSCTRL_BASE + sysctrl::CR1),
            pac::read(SYSCTRL_BASE + sysctrl::HSI),
        )
    };
    validate_configuration(cr0, cr1, hsi)?;

    // The vendor startup uses halfword reads from these exact aligned addresses.
    // SAFETY: readable device factory calibration memory is a caller requirement.
    let hsi_trim = trim_or_fallback(
        unsafe { core::ptr::read_volatile(pac::HSI_TRIM_ADDRESS as *const u16) },
        fields::hsi::TRIM.mask(),
        pac::HSI_TRIM_FALLBACK,
    );
    // SAFETY: as above; the LSI code follows the HSI halfword.
    let lsi_trim = trim_or_fallback(
        unsafe { core::ptr::read_volatile(pac::LSI_TRIM_ADDRESS as *const u16) },
        fields::lsi::TRIM.mask(),
        pac::LSI_TRIM_FALLBACK,
    );

    // SAFETY: ordinary R/W fields, preserving divider and wait settings; unlike
    // CR0/CR1/AHBEN, HSI and LSI do not contain a keyed upper halfword.
    unsafe {
        let hsi = pac::read(SYSCTRL_BASE + sysctrl::HSI);
        let lsi = pac::read(SYSCTRL_BASE + sysctrl::LSI);
        pac::write(
            SYSCTRL_BASE + sysctrl::HSI,
            fields::hsi::TRIM.write(hsi, hsi_trim),
        );
        pac::write(
            SYSCTRL_BASE + sysctrl::LSI,
            fields::lsi::TRIM.write(lsi, lsi_trim),
        );
    }

    let mut stable = false;
    for _ in 0..config.hsi_stabilization_limit {
        // SAFETY: read-only observation of the exclusively owned controller.
        if fields::hsi::STABLE.read(unsafe { pac::read(SYSCTRL_BASE + sysctrl::HSI) }) {
            stable = true;
            break;
        }
        core::hint::spin_loop();
    }
    if !stable {
        return Err(ClockError::HsiNotStable);
    }

    // SAFETY: as above; verify settings and both trim fields after modification.
    let (cr0, cr1, hsi, lsi) = unsafe {
        (
            pac::read(SYSCTRL_BASE + sysctrl::CR0),
            pac::read(SYSCTRL_BASE + sysctrl::CR1),
            pac::read(SYSCTRL_BASE + sysctrl::HSI),
            pac::read(SYSCTRL_BASE + sysctrl::LSI),
        )
    };
    validate_configuration(cr0, cr1, hsi)?;
    if fields::hsi::TRIM.read(hsi) != hsi_trim || fields::lsi::TRIM.read(lsi) != lsi_trim {
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
