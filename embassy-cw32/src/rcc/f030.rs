//! Conservative reset-clock bring-up for CW32F030.
//!
//! This module accepts only HSI /6 with undivided AHB/APB: nominal 8 MHz.
//! It loads the factory trim exactly as prescribed by the official F030 startup
//! library (halfword loads, no L012-specific erased-code fallback). It does not
//! switch clock sources or raise the clock frequency.
//! Frequencies are nominal, not a measurement or oscillator-accuracy guarantee.

use crate::pac::sysctrl::fields;
use crate::pac::{self, sysctrl, SYSCTRL_BASE};

/// Bounded reset-clock validation. Source/divider changes are not yet supported.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    /// Maximum HSI stability polls after applying vendor trim.
    pub hsi_stabilization_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
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
}

fn validate_configuration(cr0: u32, cr1: u32, hsi: u32) -> Result<(), ClockError> {
    if fields::cr0::SYSCLK.read(cr0) != 0 {
        return Err(ClockError::SystemClockNotHsi);
    }
    if fields::cr0::HCLKPRS.read(cr0) != 0 || fields::cr0::PCLKPRS.read(cr0) != 0 {
        return Err(ClockError::BusPrescalerNotReset);
    }
    if fields::hsi::DIV.read(hsi) != fields::hsi::DIV.read(sysctrl::HSI_DIV6) {
        return Err(ClockError::HsiDividerNotReset);
    }
    if !fields::cr1::HSIEN.read(cr1) {
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
    let hsi_trim = trim_code(
        unsafe { core::ptr::read_volatile(pac::HSI_TRIM_ADDRESS as *const u16) },
        fields::hsi::TRIM.mask(),
    );
    // SAFETY: as above; the LSI code follows the HSI halfword.
    let lsi_trim = trim_code(
        unsafe { core::ptr::read_volatile(pac::LSI_TRIM_ADDRESS as *const u16) },
        fields::lsi::TRIM.mask(),
    );

    // SAFETY: ordinary R/W fields, preserving divider and wait settings; unlike
    // CR0/CR1, HSI and LSI do not contain a keyed upper halfword.
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
    Ok(Clocks {
        sysclk: pac::RESET_SYSCLK_HZ,
        hclk: pac::RESET_HCLK_HZ,
        pclk: pac::RESET_PCLK_HZ,
    })
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
