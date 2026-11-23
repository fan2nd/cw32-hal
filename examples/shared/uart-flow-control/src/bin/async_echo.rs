#![no_std]
#![no_main]
use cw32_uart_flow_control_examples as examples;
use embassy_cw32 as hal;
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let (peri, tx, rx, rts, cts) = examples::pins!(p);
    let mut config = hal::usart::Config::default();
    config.cts_pull = hal::gpio::Pull::Up;
    let uart =
        hal::usart::Uart::new_with_rtscts(peri, tx, rx, rts, cts, examples::Irqs, config).unwrap();
    let (mut tx, mut rx) = uart.split();
    loop {
        let mut byte = [0];
        rx.read(&mut byte).await.unwrap();
        tx.write(&byte).await.unwrap();
        tx.flush().await.unwrap();
    }
}
