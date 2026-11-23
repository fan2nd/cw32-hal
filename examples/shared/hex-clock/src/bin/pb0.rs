#![no_std]
#![no_main]

use cw32_hex_clock_examples as _;
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
    config.rcc.hex = Some(rcc::Hex {
        freq: Hertz(24_000_000),
        min_freq: Hertz(23_999_280),
        max_freq: Hertz(24_000_720),
        operating_conditions: conditions,
        input: rcc::HexInput::Pb0,
    });
    config.rcc.sys = rcc::Sysclk::HEX;
    // This is an iteration budget, not a duration or clock-loss safeguard.
    config.rcc.timeout = 2_000_000;
    let p = hal::init(config).unwrap();
    let mut led = Output::new(p.PA0, Level::Low, Speed::Default);
    let mut uart = usart::UartTx::new_blocking(p.UART1, p.PB2, usart::Config::default()).unwrap();
    // UART baud is derived by its driver from the frozen HEX-derived PCLK.
    loop {
        uart.blocking_write(b"qualified PB0 HEX: UART1 115200 8N1\r\n")
            .unwrap();
        uart.blocking_flush().unwrap();
        led.toggle();
        // Observable activity only; not a calibrated wall-clock delay.
        cortex_m::asm::delay(rcc::clocks().hclk.0 / 4);
    }
}
