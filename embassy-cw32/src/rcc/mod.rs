//! Reset and clock control for the selected, verified device backend.
//!
//! All backends expose the same Embassy-style Config/init/frozen-clocks boundary.
//! Factory HSI is supported on all backends. F020/F030/A030 and L031/R031
//! L010/L011, L052 and L083 admit direct source-qualified HSE crystal and bypass
//! clocks. F002/F003
//! admit direct digital HEX inputs on PB0/PB1. F020/F030/A030 and L083 admit a separately
//! qualified factory-HSI-fed PLL with rate-only bounds. See each family
//! Config
//! types and the selected package's bonded external-clock pads.
//!
//! Initialization requires a valid incoming clock/voltage state and an entry
//! clock source that remains available during the transition. The HAL does not
//! guarantee recovery from arbitrary asynchronous external-clock loss during
//! register read/modify/write sequences. Readiness timeouts detect selected
//! failures; they are not a fault-tolerant clock-control system.
//! ClockBounds retains each source-qualified actual envelope separately from
//! nominal Hertz. Factory-HSI bounds require the factory trim and the selected
//! device's HSI_BOUND_TEMPERATURE_C (ambient TA) and HSI_BOUND_SUPPLY_MV; HSE/HEX
//! bounds require the explicitly declared board-qualified source endpoints.
//! Accuracy outside a source's qualified conditions is unqualified even if
//! general operating ratings permit it. No temperature or voltage is measured.
//! Config.operating_conditions declares the board's guaranteed VDD and ambient
//! interval. The HAL validates that declaration against the qualified source
//! range and checks actual upper HCLK/PCLK against the minimum-VDD bus ceiling
//! before MMIO. The declaration is not a measurement; the board must satisfy it.
//! Most families permit only24MHz below1.8V. Other device/peripheral/thermal
//! requirements still apply. Nominal clocks stay nominal.

#[cfg(any(rcc_v1, rcc_cw32f020_v1))]
mod hsi_48mhz;
#[cfg(any(rcc_v1, rcc_cw32f020_v1))]
pub use hsi_48mhz::*;
#[cfg(any(rcc_v1, rcc_cw32f020_v1))]
pub(crate) use hsi_48mhz::{init as init_backend, lsi_inspection_error};

#[cfg(any(rcc_cw32f002_v1, rcc_cw32f003_v1))]
mod f002_f003;
#[cfg(any(rcc_cw32f002_v1, rcc_cw32f003_v1))]
pub(crate) use f002_f003::init as init_backend;
#[cfg(all(any(rcc_cw32f002_v1, rcc_cw32f003_v1), rcc_lsi_sysclk))]
pub(crate) use f002_f003::lsi_inspection_error;
#[cfg(any(rcc_cw32f002_v1, rcc_cw32f003_v1))]
pub use f002_f003::*;

#[cfg(rcc_cw32l031_v1)]
mod l031_r031_w031;
#[cfg(rcc_cw32l031_v1)]
pub(crate) use l031_r031_w031::init as init_backend;
#[cfg(all(rcc_cw32l031_v1, rcc_lsi_sysclk))]
pub(crate) use l031_r031_w031::lsi_inspection_error;
#[cfg(rcc_cw32l031_v1)]
pub use l031_r031_w031::*;

#[cfg(any(rcc_cw32l010_v1, rcc_cw32l011_v1))]
mod l010_l011;
#[cfg(any(rcc_cw32l010_v1, rcc_cw32l011_v1))]
pub(crate) use l010_l011::init as init_backend;
#[cfg(any(rcc_cw32l010_v1, rcc_cw32l011_v1))]
pub use l010_l011::*;

#[cfg(rcc_cw32l012_v1)]
mod l012;
#[cfg(rcc_cw32l012_v1)]
use l012::init as init_backend;
#[cfg(rcc_cw32l012_v1)]
pub use l012::*;

#[cfg(any(rcc_cw32l052_v1, rcc_cw32l083_v1))]
mod l052_l083;
#[cfg(any(rcc_cw32l052_v1, rcc_cw32l083_v1))]
pub(crate) use l052_l083::init as init_backend;
#[cfg(all(any(rcc_cw32l052_v1, rcc_cw32l083_v1), rcc_lsi_sysclk))]
pub(crate) use l052_l083::lsi_inspection_error;
#[cfg(any(rcc_cw32l052_v1, rcc_cw32l083_v1))]
pub use l052_l083::*;

#[cfg(all(rcc_lse, not(any(rcc_cw32l010_v1, rcc_cw32l011_v1, rcc_cw32l012_v1))))]
mod lse;
#[cfg(all(rcc_lse, any(rcc_cw32l010_v1, rcc_cw32l011_v1, rcc_cw32l012_v1)))]
mod lse_native_low_power;
#[cfg(all(rcc_lse, not(any(rcc_cw32l010_v1, rcc_cw32l011_v1, rcc_cw32l012_v1))))]
pub use lse::LseAmplitude;
#[cfg(all(rcc_lse, any(rcc_cw32l010_v1, rcc_cw32l011_v1, rcc_cw32l012_v1)))]
pub use lse::LseFaultDetection;
#[cfg(rcc_lse)]
pub use lse::{Lse, LseDrive, LseMode, LseWait};
#[cfg(all(rcc_lse, any(rcc_cw32l010_v1, rcc_cw32l011_v1, rcc_cw32l012_v1)))]
use lse_native_low_power as lse;
#[cfg(rcc_lse)]
pub use rtc::LseClock;

#[cfg(rtc)]
mod rtc;
#[cfg(any(rtc_cw32l010_v1, rtc_cw32l011_v1, rtc_cw32l012_v1))]
pub use rtc::HsiOscClock;
#[cfg(any(
    rtc_v1,
    rtc_cw32f020_v1,
    rtc_cw32l031_v1,
    rtc_cw32l052_v1,
    rtc_native_live_lsi
))]
pub use rtc::LsiClock;
#[cfg(rtc)]
pub use rtc::{CalendarClock, RtcClockError};

#[cfg(mco_f030)]
mod mco;
#[cfg(mco_f030)]
pub use mco::{Mco, McoBoardConditions, McoError, McoPrescaler, McoSource};

mod bounds;
pub use bounds::{ClockBounds, HSI_BOUND_SUPPLY_MV, HSI_BOUND_TEMPERATURE_C};

mod operating;
pub use operating::OperatingConditions;

mod peripheral;
#[cfg(all(any(rcc_cw32l010_v1, rcc_cw32l012_v1), rcc_lsi_sysclk))]
pub(crate) use peripheral::ClockInspectionError;
pub use peripheral::{
    ClockError, RccPeripheral, bus_clock_bounds, bus_frequency, frequency, kernel_clock_bounds,
};
pub(crate) use peripheral::{
    RccInfo, RccPolicy, Readback, SealedRccPeripheral, disable_with_cs, enable_and_reset_with_cs,
};

#[cfg(lcd)]
mod low_speed;
#[cfg(lcd)]
pub(crate) use low_speed::enable_lsi;

// LSE ownership is independent of HSE support and the selected system clock.
static LSE_PADS: critical_section::Mutex<core::cell::Cell<(bool, bool)>> =
    critical_section::Mutex::new(core::cell::Cell::new((false, false)));

/// Capture inherited low-speed oscillator ownership before returning any tokens.
///
/// Safety: the caller owns initialization; no DMA, NMI or other code may mutate
/// clock or pad state during the sequence. Backends retain the inherited LSE.
pub(crate) unsafe fn init(config: Config, clocks: Clocks) -> Result<(), Error> {
    #[cfg(rcc_cw32l012_v1)]
    let backend = |config| unsafe { init_backend::<false>(config, clocks) };
    #[cfg(not(rcc_cw32l012_v1))]
    let backend = |config| {
        let _ = clocks;
        unsafe { init_backend(config) }
    };
    init_with(config, backend)
}

#[cfg(rcc_cw32l012_v1)]
pub(crate) unsafe fn init_hsi(config: Config, clocks: Clocks) -> Result<(), Error> {
    init_with(config, |config| unsafe { init_backend::<true>(config, clocks) })
}

fn init_with(config: Config, backend: impl FnOnce(Config) -> Result<(), Error>) -> Result<(), Error> {
    critical_section::with(|cs| {
        let inherited = crate::rcc_lse_owned_pads();
        let retained = LSE_PADS.borrow(cs).get();
        #[cfg(rcc_lse)]
        let requested = config
            .lse
            .map_or((false, false), |c| (true, c.mode == LseMode::Oscillator));
        #[cfg(not(rcc_lse))]
        let requested = (false, false);
        // Never release a captured pad during this boot, including on faults.
        LSE_PADS.borrow(cs).set((
            retained.0 || inherited.0 || requested.0,
            retained.1 || inherited.1 || requested.1,
        ));
        backend(config)
    })
}

pub(crate) fn lse_pin_reserved(pin: u8) -> bool {
    critical_section::with(|cs| {
        let retained = LSE_PADS.borrow(cs).get();
        (retained.0 && crate::RCC_LSE_PINS.0 == Some(pin))
            || (retained.1 && crate::RCC_LSE_PINS.1 == Some(pin))
    })
}
