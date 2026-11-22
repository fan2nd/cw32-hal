#![no_std]
#![no_main]

use cw32_hse_clock_examples as _;
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    rcc,
    time::Hertz,
    usart,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // These are BOARD requirements, not measurements or a chip accuracy claim.
    // Connect the qualified 24 MHz source described in README.md before boot.
    let conditions = rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = conditions;
    config.rcc.hse = Some(rcc::Hse {
        freq: Hertz(24_000_000),
        min_freq: Hertz(23_999_280),
        max_freq: Hertz(24_000_720),
        operating_conditions: conditions,
        mode: rcc::HseMode::Bypass,
        drive: rcc::HseDrive::Level2,
    });
    #[cfg(not(feature = "retained-hse"))]
    {
        config.rcc.sys = rcc::Sysclk::HSE;
    }
    // On L012, Some(Hse) declares exact enabled-source reuse when present.
    // See README for the required stable bootloader source, pads and RTC PSC1.
    #[cfg(feature = "retained-hse")]
    {
        config.rcc.sys = rcc::Sysclk::HSI;
    }
    // Long startup-count selection is fixed by RCC; this remains an iteration
    // budget whose elapsed time depends on the incoming/bridge clocks.
    config.rcc.timeout = 2_000_000;
    let p = hal::init(config).unwrap();
    #[cfg(gpio_speed)]
    let speed = Speed::Low;
    #[cfg(not(gpio_speed))]
    let speed = Speed::Default;
    // Exact board wiring differs across these families. None of these routes
    // consumes the family's HSE pads; L010 also leaves its LSE and SWD pads free.
    #[cfg(any(
        feature = "cw32l010f8p6",
        feature = "cw32l010f8u6",
        feature = "cw32l010y8m6"
    ))]
    let (led_pin, tx_pin) = (p.PA3, p.PA6);
    #[cfg(any(
        feature = "cw32l011k8t6",
        feature = "cw32l011k8u6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    ))]
    let (led_pin, tx_pin) = (p.PB0, p.PA9);
    #[cfg(not(any(
        feature = "cw32l010f8p6",
        feature = "cw32l010f8u6",
        feature = "cw32l010y8m6",
        feature = "cw32l011k8t6",
        feature = "cw32l011k8u6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    )))]
    let (led_pin, tx_pin) = (p.PB0, p.PA8);
    let mut led = Output::new(led_pin, Level::Low, speed);
    let mut uart = usart::UartTx::new_blocking(p.UART1, tx_pin, usart::Config::default()).unwrap();
    // UART baud is derived by its driver from the frozen HSE-derived PCLK.
    loop {
        uart.blocking_write(b"qualified bypass HSE: UART1 115200 8N1\r\n")
            .unwrap();
        uart.blocking_flush().unwrap();
        led.toggle();
        // Observable activity only; not a calibrated wall-clock delay.
        cortex_m::asm::delay(rcc::clocks().hclk.0 / 4);
    }
}
