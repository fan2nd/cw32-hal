#![no_std]
#![no_main]
use cw32_uart_flow_control_examples as examples;
use embassy_cw32 as hal;
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let (peri, tx, rx, rts, cts) = examples::pins!(p);
    let mut config = hal::usart::Config::default();
    config.cts_pull = hal::gpio::Pull::Up;
    let uart = hal::usart::Uart::new_blocking_with_rtscts(peri, tx, rx, rts, cts, config).unwrap();
    let (mut tx, mut rx) = uart.split();
    loop {
        let mut byte = [0];
        rx.blocking_read(&mut byte).unwrap();
        tx.blocking_write(&byte).unwrap();
        tx.blocking_flush().unwrap();
    }
}
