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
    feature = "cw32l031c8t6",
    feature = "cw32l052c8t6",
    feature = "cw32l010f8p6"
))]
hal::bind_interrupts!(pub struct Irqs {
    UART2 => hal::usart::InterruptHandler<hal::peripherals::UART2>;
});

#[cfg(feature = "cw32l011k8t6")]
hal::bind_interrupts!(pub struct Irqs {
    UART3 => hal::usart::InterruptHandler<hal::peripherals::UART3>;
});

pub const RING_SIZE: usize = 64;
pub const CHUNK_SIZE: usize = 16;

/// Construct a direct buffered owner on the exact-package routes in the README.
/// The ring is consumed, including when construction fails.
pub fn uart(p: hal::Peripherals, ring: &'static mut [u8]) -> hal::usart::BufferedUart<'static> {
    #[cfg(any(
        feature = "cw32f030c8t7",
        feature = "cw32l031c8t6",
        feature = "cw32l052c8t6"
    ))]
    let (peri, tx, rx) = (p.UART2, p.PA2, p.PA3);
    #[cfg(feature = "cw32f002f3p7")]
    let (peri, tx, rx) = (p.UART1, p.PB1, p.PB0);
    #[cfg(feature = "cw32l083rct6")]
    let (peri, tx, rx) = (p.UART1, p.PA8, p.PA9);
    #[cfg(feature = "cw32l010f8p6")]
    let (peri, tx, rx) = (p.UART2, p.PA3, p.PA4);
    #[cfg(feature = "cw32l011k8t6")]
    let (peri, tx, rx) = (p.UART3, p.PA4, p.PA5);
    #[cfg(feature = "cw32l012c8t6")]
    let (peri, tx, rx) = (p.UART1, p.PA2, p.PA3);
    hal::usart::BufferedUart::new(peri, tx, rx, Irqs, ring, hal::usart::Config::default()).unwrap()
}

/// Construct only the receive direction; L052 additionally owns its RTS pin.
pub fn receiver(
    p: hal::Peripherals,
    ring: &'static mut [u8],
) -> hal::usart::BufferedUartRx<'static> {
    #[cfg(feature = "cw32l052c8t6")]
    {
        hal::usart::BufferedUartRx::new_with_rts(
            p.UART2,
            p.PA3,
            p.PA1,
            Irqs,
            ring,
            hal::usart::Config::default(),
        )
        .unwrap()
    }
    #[cfg(not(feature = "cw32l052c8t6"))]
    {
        #[cfg(any(feature = "cw32f030c8t7", feature = "cw32l031c8t6"))]
        let (peri, rx) = (p.UART2, p.PA3);
        #[cfg(feature = "cw32f002f3p7")]
        let (peri, rx) = (p.UART1, p.PB0);
        #[cfg(feature = "cw32l083rct6")]
        let (peri, rx) = (p.UART1, p.PA9);
        #[cfg(feature = "cw32l010f8p6")]
        let (peri, rx) = (p.UART2, p.PA4);
        #[cfg(feature = "cw32l011k8t6")]
        let (peri, rx) = (p.UART3, p.PA5);
        #[cfg(feature = "cw32l012c8t6")]
        let (peri, rx) = (p.UART1, p.PA3);
        hal::usart::BufferedUartRx::new(peri, rx, Irqs, ring, hal::usart::Config::default())
            .unwrap()
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
