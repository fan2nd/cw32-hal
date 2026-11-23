#![no_std]
use embassy_cw32::{self as hal, rcc, time::Hertz};

/// Demonstration board assertions, not measurements or a certified BOM.
/// Both modes require every cycle within this envelope throughout these conditions.
#[cfg(not(any(
    feature = "cw32l010f8p6",
    feature = "cw32l010f8u6",
    feature = "cw32l010y8m6",
    feature = "cw32l011k8t6",
    feature = "cw32l011k8u6",
    feature = "cw32l012c8t6",
    feature = "cw32l012c8u6"
)))]
pub fn config(mode: rcc::LseMode) -> hal::Config {
    let board = rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = board;
    config.rcc.lse = Some(rcc::Lse {
        min_freq: Hertz(32_766),
        max_freq: Hertz(32_770),
        operating_conditions: board,
        mode,
        // Crystal board/load must qualify these settings. Bypass keeps explicit
        // parameters too, so later exact-source reuse checks the same tuple.
        drive: rcc::LseDrive::Strong,
        amplitude: rcc::LseAmplitude::Normal,
        // L052 has independent startup banks. These deliberately distinct
        // demonstration values require board qualification in both modes;
        // they are not a universal crystal/startup preset.
        #[cfg(any(
            feature = "cw32l052c8t6",
            feature = "cw32l052r8s6",
            feature = "cw32l052r8t6"
        ))]
        startup_drive: rcc::LseDrive::Normal,
        #[cfg(any(
            feature = "cw32l052c8t6",
            feature = "cw32l052r8s6",
            feature = "cw32l052r8t6"
        ))]
        startup_amplitude: rcc::LseAmplitude::Large,
        wait: rcc::LseWait::Cycles16384,
        poll_budget: 20_000_000,
    });
    config.rcc.sys = rcc::Sysclk::LSE;
    // HSI /6, AHB /1 and APB /1 retain their existing defaults. Initialization
    // prepares detector LSI internally; there is no second source declaration.
    config
}

/// Native L010/L011/L012 demonstration assertions; replace with qualified board data.
/// StartupOnly leaves running-loss detection off. Losing LSE can stop the CPU
/// without reaching an error return, even when inherited CLKCCS is enabled.
#[cfg(any(
    feature = "cw32l010f8p6",
    feature = "cw32l010f8u6",
    feature = "cw32l010y8m6",
    feature = "cw32l011k8t6",
    feature = "cw32l011k8u6",
    feature = "cw32l012c8t6",
    feature = "cw32l012c8u6"
))]
pub fn config(mode: rcc::LseMode) -> hal::Config {
    let board = rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = board;
    config.rcc.lse = Some(rcc::Lse {
        min_freq: Hertz(32_766),
        max_freq: Hertz(32_770),
        operating_conditions: board,
        mode,
        // Independent native four-bit drive fields, without amplitude fields.
        // Qualify both values and startup count against the actual board/load.
        // Bypass retains the same tuple for exact-source reuse checks.
        drive: rcc::LseDrive::Level2,
        startup_drive: rcc::LseDrive::Level10,
        wait: rcc::LseWait::Cycles16384,
        fault_detection: rcc::LseFaultDetection::StartupOnly,
        poll_budget: 20_000_000,
    });
    config.rcc.sys = rcc::Sysclk::LSE;
    // Native defaults are HSI /12 on L010/L012, HSI /24 on L011, and AHB/APB /1.
    // HSI is retained factory calibrated. The enclosing HSI-retrim path can
    // temporarily request unchanged, electrically legal LSI; this does not
    // prepare a monitor or impose a factory-LSI requirement on StartupOnly.
    // See README.md for the incoming-clock and downstream-observer handover.
    config
}

pub fn run(rtc: hal::rtc::Rtc<'_>) -> ! {
    loop {
        core::hint::black_box((
            hal::rcc::clocks().sys_bounds(),
            rtc.now().unwrap(),
            rtc.source_clock_bounds(),
            rtc.calendar_tick_bounds(),
        ));
    }
}

pub fn initial_time() -> hal::rtc::DateTime {
    // Demonstration epoch only. Replace it before using this as a clock product.
    hal::rtc::DateTime::from(2026, 10, 9, hal::rtc::DayOfWeek::Friday, 12, 0, 0, 0).unwrap()
}
