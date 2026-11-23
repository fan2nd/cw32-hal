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
    let mut chunk = [0u8; examples::CHUNK_SIZE];
    loop {
        // Arm before sending a short burst from the peer. A silent line need
        // not notify, and later traffic may overflow the ring during this wait.
        let idle = rx.wait_for_idle().await;
        let mut remaining = idle.buffered_bytes;
        // Consume only the prefix seen by the ISR, excluding later appends.
        // This is a software observation position, not a wire packet boundary.
        while remaining != 0 {
            let count = remaining.min(chunk.len());
            rx.read_exact(&mut chunk[..count]).await.unwrap();
            core::hint::black_box(&chunk[..count]);
            remaining -= count;
        }
        if idle.error.is_some() {
            // The retained error follows that prefix and blocks later appends
            // until consumed. Reading it does not consume a subsequent byte.
            let result = rx.blocking_read(&mut chunk[..1]);
            core::hint::black_box(&result);
        }
    }
}
