#![no_std]
#![no_main]
use cw32_uart_flow_control_examples as examples;
use embassy_cw32 as hal;
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let (peri, tx, _rx, _rts, cts) = examples::pins!(p);
    let mut config = hal::usart::Config::default();
    config.cts_pull = hal::gpio::Pull::Up;
    let mut uart = hal::usart::UartTx::new_blocking_with_cts(peri, tx, cts, config).unwrap();
    loop {
        uart.blocking_write(b"CW32 hardware CTS\r\n").unwrap();
        uart.blocking_flush().unwrap();
    }
}
