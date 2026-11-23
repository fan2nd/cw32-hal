#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32::{self as hal, usart::IdleCaptureError};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let mut rx = examples::receiver(p, ring);
    let mut chunk = [0u8; examples::CHUNK_SIZE];
    rx.start_idle_capture();
    loop {
        match rx.next_idle_marker().await {
            Ok(marker) => {
                // Diagnostic only: queued errors are historical, and an already
                // returned offset becomes stale as the application reads data.
                core::hint::black_box(marker);
            }
            Err(IdleCaptureError::Overflow) => {
                // Retained markers preceded this explicit loss notification.
                // Capture resumes; the missing observations cannot be recovered.
            }
            Err(IdleCaptureError::NotCapturing) => unreachable!(),
        }
        // Drain a bounded batch independently of marker history. Every consumed
        // byte adjusts positions of markers still in the internal queue.
        for _ in 0..=examples::RING_SIZE / examples::CHUNK_SIZE {
            match rx.blocking_read(&mut chunk) {
                Ok(0) => break,
                Ok(count) => {
                    core::hint::black_box(&chunk[..count]);
                }
                Err(error) => {
                    core::hint::black_box(error);
                }
            }
        }
    }
}
