#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32::{self as hal, usart::BufferReplaceError};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let larger = cortex_m::singleton!(: [u8; examples::RING_SIZE * 2] =
        [0; examples::RING_SIZE * 2])
    .unwrap();
    let mut rx = examples::receiver(p, ring);
    let mut replacement: Option<&'static mut [u8]> = Some(larger);
    let mut chunk = [0u8; examples::CHUNK_SIZE];
    loop {
        // Read data/errors before attempting the change. Traffic can refill
        // the ring between the read and replacement, so retain rejected storage.
        match rx.read(&mut chunk).await {
            Ok(count) => {
                core::hint::black_box(&chunk[..count]);
            }
            Err(error) => {
                core::hint::black_box(error);
            }
        }
        if let Some(next) = replacement.take() {
            match rx.try_replace_buffer(next) {
                Ok(previous) => {
                    // RX remains live over the larger ring. The detached
                    // original allocation can immediately be reused.
                    previous.fill(0);
                    core::hint::black_box(previous);
                }
                Err((BufferReplaceError::PendingReceive, next)) => {
                    replacement = Some(next);
                }
                // This example never enables idle capture and supplies nonempty storage.
                Err(_) => unreachable!(),
            }
        }
    }
}
