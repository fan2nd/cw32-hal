#![no_std]
#![no_main]
use cw32_uart_flow_control_examples as examples;
use embassy_cw32 as hal;
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let (peri, _tx, rx, rts, _cts) = examples::pins!(p);
    let mut config = hal::usart::Config::default();
    config.cts_pull = hal::gpio::Pull::Up;
    let mut uart = hal::usart::UartRx::new_with_rts(peri, rx, rts, examples::Irqs, config).unwrap();
    loop {
        let mut byte = [0];
        uart.read(&mut byte).await.unwrap();
        core::hint::black_box(byte);
    }
}
