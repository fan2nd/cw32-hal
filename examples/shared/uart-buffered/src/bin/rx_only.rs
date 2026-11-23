#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32 as hal;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let mut rx = examples::receiver(p, ring);
    loop {
        let mut chunk = [0u8; examples::CHUNK_SIZE];
        match embedded_io_async::Read::read(&mut rx, &mut chunk).await {
            Ok(count) => {
                // Inspect the returned prefix here; there is no TX owner.
                core::hint::black_box(&chunk[..count]);
            }
            Err(error) => {
                core::hint::black_box(error);
            }
        }
    }
}
