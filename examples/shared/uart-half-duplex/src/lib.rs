#![no_std]
use embassy_cw32::{self as hal, usart};

#[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
hal::bind_interrupts!(pub struct Irqs {
    UART1 => usart::InterruptHandler<hal::peripherals::UART1>;
});

#[cfg(feature = "cw32l010f8p6")]
hal::bind_interrupts!(pub struct Irqs {
    UART2 => usart::InterruptHandler<hal::peripherals::UART2>;
});
#[cfg(feature = "cw32l011k8t6")]
hal::bind_interrupts!(pub struct Irqs {
    UART3 => usart::InterruptHandler<hal::peripherals::UART3>;
});

pub fn config() -> usart::Config {
    let mut config = usart::Config::default();
    config.data_bits = usart::DataBits::DataBits8;
    config
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
