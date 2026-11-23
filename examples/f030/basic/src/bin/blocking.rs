#![no_std]
#![no_main]

use cw32f030_examples as _;
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    spi, usart,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Default clocks use nominal 8 MHz HSI/HCLK/PCLK.
    let p = hal::init(Default::default()).unwrap();
    let mut led = Output::new(p.PB0, Level::Low, Speed::Low);
    let mut cs = Output::new(p.PA4, Level::High, Speed::Low);
    let mut uart =
        usart::Uart::new_blocking(p.UART1, p.PA8, p.PA9, usart::Config::default()).unwrap();
    let mut spi =
        spi::Spi::new_blocking(p.SPI1, p.PA5, p.PA7, p.PA6, spi::Config::default()).unwrap();

    uart.blocking_write(b"CW32F030 blocking GPIO/UART/SPI loopback\r\n")
        .unwrap();
    uart.blocking_flush().unwrap();
    loop {
        // With PA7 connected to PA6 these bytes are returned unchanged.
        // No SPI slave is required. CS is shown for a future real device.
        let mut data = [0xA5u8, 0x5A, 0x00, 0xFF];
        cs.set_low();
        let result = spi.blocking_transfer_in_place(&mut data);
        cs.set_high();
        result.unwrap();
        let message: &[u8] = if data == [0xA5, 0x5A, 0x00, 0xFF] {
            b"SPI loopback OK\r\n"
        } else {
            b"SPI loopback mismatch: check PA7-PA6 jumper\r\n"
        };
        uart.blocking_write(message).unwrap();
        uart.blocking_flush().unwrap();
        led.toggle();

        // Deliberate busy delay, not an Embassy timer or a calibrated timebase.
        cortex_m::asm::delay(4_000_000);
    }
}
