#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32::{self as hal, usart};
use embassy_futures::join::join;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring1 =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let ring4 =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    // One binding dispatches both live owners; each consumes a distinct ring.
    let mut uart1 = usart::BufferedUart::new(
        p.UART1,
        p.PA8,
        p.PA9,
        examples::Irqs,
        ring1,
        usart::Config::default(),
    )
    .unwrap();
    let mut uart4 = usart::BufferedUart::new(
        p.UART4,
        p.PA4,
        p.PB5,
        examples::Irqs,
        ring4,
        usart::Config::default(),
    )
    .unwrap();
    join(echo(&mut uart1), echo(&mut uart4)).await;
}

async fn echo(uart: &mut usart::BufferedUart<'_>) -> ! {
    loop {
        let mut chunk = [0u8; examples::CHUNK_SIZE];
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
