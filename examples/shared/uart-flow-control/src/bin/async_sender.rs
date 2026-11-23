#![no_std]
#![no_main]
use cw32_uart_flow_control_examples as examples;
use embassy_cw32 as hal;
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let (peri, tx, _rx, _rts, cts) = examples::pins!(p);
    let mut config = hal::usart::Config::default();
    config.cts_pull = hal::gpio::Pull::Up;
    let mut uart = hal::usart::UartTx::new_with_cts(peri, tx, cts, examples::Irqs, config).unwrap();
    loop {
        uart.write(b"CW32 hardware CTS\r\n").await.unwrap();
        uart.flush().await.unwrap();
    }
}
