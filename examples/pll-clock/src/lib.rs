#![no_std]
use embassy_cw32::{self as hal, rcc};

#[cfg(all(feature = "hse-crystal", feature = "hse-bypass"))]
compile_error!("select only one of hse-crystal and hse-bypass");
#[cfg(all(
    feature = "fractional",
    any(feature = "hse-crystal", feature = "hse-bypass")
))]
compile_error!("fractional selects an HSI PLL pair; omit it for HSE");

/// Board declarations in README.md must hold for this firmware's lifetime.
pub fn config() -> hal::Config {
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = rcc::OperatingConditions {
        supply_mv: if cfg!(feature = "low-voltage") { 1_700 } else { 3_300 },
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    config.rcc.hsi.div = if cfg!(feature = "fractional") {
        rcc::HsiDiv::Div10
    } else {
        rcc::HsiDiv::Div6
    };
    #[cfg(not(any(feature = "hse-crystal", feature = "hse-bypass")))]
    {
        config.rcc.pll = Some(rcc::Pll {
            src: rcc::PllSource::HSI,
            mul: if cfg!(classic_pll) {
                if cfg!(feature = "fractional") {
                    rcc::PllMul::Mul9
                } else {
                    rcc::PllMul::Mul4
                }
            } else if cfg!(feature = "fractional") {
                rcc::PllMul::Mul12
            } else {
                rcc::PllMul::Mul7
            },
        });
    }
    #[cfg(any(feature = "hse-crystal", feature = "hse-bypass"))]
    {
        use hal::time::Hertz;
        // Required actual board envelope, including short-term variation;
        // this declaration is not a measured oscillator specification.
        config.rcc.hse = Some(rcc::Hse {
            freq: Hertz(8_000_000),
            min_freq: Hertz(7_999_600),
            max_freq: Hertz(8_000_400),
            operating_conditions: config.rcc.operating_conditions,
            mode: if cfg!(feature = "hse-crystal") {
                rcc::HseMode::Oscillator
            } else {
                rcc::HseMode::Bypass
            },
            // Qualify this drive with the crystal and board; ignored in bypass.
            drive: rcc::HseDrive::Level2,
        });
        config.rcc.pll = Some(rcc::Pll {
            src: rcc::PllSource::HSE,
            mul: rcc::PllMul::Mul4,
        });
    }
    config.rcc.sys = rcc::Sysclk::PLL;
    if cfg!(feature = "low-voltage") {
        config.rcc.ahb_pre = rcc::AHBPrescaler::Div4;
    }
    // Long startup-cycle wait is selected internally; this is a poll budget,
    // not a microsecond guarantee across incoming and bridge clock choices.
    config.rcc.timeout = 2_000_000;
    config
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
