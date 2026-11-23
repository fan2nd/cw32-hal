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
    // Board declaration: a stable 3.95 MHz source with total +/-30 ppm bounds
    // must drive PA0 before boot. These are requirements, not measurements.
    let conditions = rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = conditions;
    config.rcc.hse = Some(rcc::Hse {
        freq: Hertz(3_950_000),
        // Round outward to include the full declared tolerance.
        min_freq: Hertz(3_949_881),
        max_freq: Hertz(3_950_119),
        operating_conditions: conditions,
        mode: rcc::HseMode::Bypass,
        drive: rcc::HseDrive::Level2,
    });
    #[cfg(not(feature = "retained-hse"))]
    {
        config.rcc.sys = rcc::Sysclk::HSE;
    }
    #[cfg(feature = "retained-hse")]
    {
        config.rcc.sys = rcc::Sysclk::HSI;
    }
    config.rcc.timeout = 2_000_000;
    let p = hal::init(config).unwrap();
    #[cfg(gpio_speed)]
    let speed = Speed::Low;
    #[cfg(not(gpio_speed))]
    let speed = Speed::Default;
    let mut led = Output::new(p.PA3, Level::Low, speed);
    let mut uart_config = usart::Config::default();
    uart_config.baudrate = 9_600;
    let mut uart = usart::UartTx::new_blocking(p.UART1, p.PA6, uart_config).unwrap();
    loop {
        uart.blocking_write(b"L010 low bypass: UART1 9600 8N1\r\n")
            .unwrap();
        uart.blocking_flush().unwrap();
        led.toggle();
        // Visible activity only; not a calibrated timebase.
        cortex_m::asm::delay(rcc::clocks().hclk.0 / 4);
    }
}
