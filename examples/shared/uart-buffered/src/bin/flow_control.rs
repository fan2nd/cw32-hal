#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32::{self as hal, usart};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let mut config = usart::Config::default();
    config.cts_pull = hal::gpio::Pull::Up;
    let mut uart = usart::BufferedUart::new_with_rtscts(
        p.UART2,
        p.PA3,
        p.PA4,
        p.PB1,
        p.PB0,
        examples::Irqs,
        ring,
        config,
    )
    .unwrap();
    loop {
        let mut chunk = [0u8; examples::CHUNK_SIZE];
        // Hardware RTS follows RC, not the occupancy of the software ring.
        match uart.read_exact(&mut chunk).await {
            Ok(()) => uart.write(&chunk).await.unwrap(),
            Err(error) => {
                core::hint::black_box(error);
                uart.write(b"receive error\r\n").await.unwrap();
            }
        }
        uart.flush().await.unwrap();
    }
}
