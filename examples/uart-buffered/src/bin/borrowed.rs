#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32::{self as hal, usart::BufferedUartRx};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let mut ring = [0u8; examples::RING_SIZE];
    {
        // SAFETY: the local buffer cannot move or be reused while rx borrows it.
        // This synchronous scope never leaks rx or suspends in a forgettable
        // future. RX is explicitly dropped before reuse; the shared example
        // panic handler disables all interrupts and never resumes execution.
        let mut rx = unsafe {
            BufferedUartRx::new_borrowed(
                p.UART1,
                p.PB0,
                examples::Irqs,
                &mut ring,
                Default::default(),
            )
        }
        .unwrap();
        let mut byte = [0u8; 1];
        while matches!(rx.blocking_read(&mut byte), Ok(0)) {}
        drop(rx);
    }
    // Normal RX Drop has ended the original borrow; no explicit return is needed.
    ring.fill(0);
    core::hint::black_box(&mut ring);
    loop {
        cortex_m::asm::wfi();
    }
}
