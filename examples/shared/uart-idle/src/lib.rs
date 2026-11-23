#![no_std]

use embassy_cw32 as hal;

#[cfg(feature = "cw32l010f8p6")]
hal::bind_interrupts!(pub struct Irqs {
    UART2 => hal::usart::InterruptHandler<hal::peripherals::UART2>;
});

#[cfg(feature = "cw32l011k8t6")]
hal::bind_interrupts!(pub struct Irqs {
    UART3 => hal::usart::InterruptHandler<hal::peripherals::UART3>;
});

#[cfg(feature = "cw32l012c8t6")]
hal::bind_interrupts!(pub struct Irqs {
    UART1 => hal::usart::InterruptHandler<hal::peripherals::UART1>;
});

/// Construct the async UART on the exact-package routes in the README.
pub fn uart(p: hal::Peripherals) -> hal::usart::Uart<'static, hal::mode::Async> {
    #[cfg(feature = "cw32l010f8p6")]
    let (peri, tx, rx) = (p.UART2, p.PA3, p.PA4);
    #[cfg(feature = "cw32l011k8t6")]
    let (peri, tx, rx) = (p.UART3, p.PA4, p.PA5);
    #[cfg(feature = "cw32l012c8t6")]
    let (peri, tx, rx) = (p.UART1, p.PA2, p.PA3);
    hal::usart::Uart::new(peri, tx, rx, Irqs, hal::usart::Config::default()).unwrap()
}

pub const BUFFER_SIZE: usize = 64;

/// Format the count returned for one of the examples' bounded receive buffers.
pub fn count_line(count: usize) -> [u8; 13] {
    assert!(count <= BUFFER_SIZE);
    let mut line = *b"received 00\r\n";
    line[9] = b'0' + (count / 10) as u8;
    line[10] = b'0' + (count % 10) as u8;
    line
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
