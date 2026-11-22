#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32 as hal;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let mut uart = examples::uart(p, ring);
    loop {
        let mut chunk = [0u8; examples::CHUNK_SIZE];
        // IRQ collection continues while this partial read or the echo is pending.
        match uart.read(&mut chunk).await {
            Ok(count) => uart.write(&chunk[..count]).await.unwrap(),
            Err(error) => {
                // Clean queued prefixes are returned before a pending error.
                core::hint::black_box(error);
                uart.write(b"receive error\r\n").await.unwrap();
            }
        }
        uart.flush().await.unwrap();
    }
}
