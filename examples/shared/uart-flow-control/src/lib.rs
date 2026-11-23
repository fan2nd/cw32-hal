#![no_std]
use embassy_cw32 as hal;
#[cfg(feature = "cw32l083rct6")]
hal::bind_interrupts!(pub struct Irqs {
    UART1_UART4 => hal::usart::SharedInterruptHandler<hal::interrupt::typelevel::UART1_UART4>;
});
#[cfg(any(feature = "cw32f002f3p7", feature = "cw32l012c8t6"))]
hal::bind_interrupts!(pub struct Irqs {
    UART1 => hal::usart::InterruptHandler<hal::peripherals::UART1>;
});
#[cfg(any(
    feature = "cw32f030c8t7",
    feature = "cw32l010f8p6",
    feature = "cw32l031c8t6",
    feature = "cw32l052c8t6"
))]
hal::bind_interrupts!(pub struct Irqs {
    UART2 => hal::usart::InterruptHandler<hal::peripherals::UART2>;
});

/// Return (UART, TX, RX, RTS, CTS) for the wiring documented in README.md.
#[macro_export]
macro_rules! pins {
    ($p:ident) => {{
        #[cfg(feature = "cw32f002f3p7")]
        let pins = ($p.UART1, $p.PB1, $p.PB0, $p.PA7, $p.PA6);
        #[cfg(feature = "cw32l010f8p6")]
        let pins = ($p.UART2, $p.PA3, $p.PA4, $p.PB1, $p.PB0);
        #[cfg(feature = "cw32l012c8t6")]
        let pins = ($p.UART1, $p.PA2, $p.PA3, $p.PA1, $p.PA0);
        #[cfg(feature = "cw32l083rct6")]
        let pins = ($p.UART1, $p.PA8, $p.PA9, $p.PA11, $p.PA10);
        #[cfg(any(
            feature = "cw32f030c8t7",
            feature = "cw32l031c8t6",
            feature = "cw32l052c8t6"
        ))]
        let pins = ($p.UART2, $p.PA2, $p.PA3, $p.PA1, $p.PA0);
        pins
    }};
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
