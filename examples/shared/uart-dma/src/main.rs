#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal, dma, peripherals,
    usart::{self, Uart},
};
use embassy_executor::Spawner;
use embassy_futures::join::join;

#[cfg(not(any(example_l083, example_f020)))]
hal::bind_interrupts!(struct Irqs {
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>, dma::InterruptHandler<peripherals::DMA_CH3>;
});

#[cfg(example_f020)]
hal::bind_interrupts!(struct Irqs {
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    DMACH2 => dma::InterruptHandler<peripherals::DMA_CH2>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
});

#[cfg(example_l083)]
hal::bind_interrupts!(struct Irqs {
    UART2_UART5 => usart::SharedInterruptHandler<hal::interrupt::typelevel::UART2_UART5>;
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>, dma::InterruptHandler<peripherals::DMA_CH3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let tx_staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();
    let rx_staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();
    #[cfg(not(any(example_l083, example_f020, example_four_channel)))]
    let (tx_pin, rx_pin) = (p.PA2, p.PA3);
    #[cfg(any(example_l083, example_f020, example_four_channel))]
    let (tx_pin, rx_pin) = (p.PA6, p.PA7);
    #[cfg(not(any(example_f020, example_four_channel)))]
    let (tx_dma, rx_dma) = (p.DMA_CH2, p.DMA_CH3);
    #[cfg(example_f020)]
    let (tx_dma, rx_dma) = (p.DMA_CH2, p.DMA_CH1);
    #[cfg(example_four_channel)]
    let (tx_dma, rx_dma) = (p.DMA_CH3, p.DMA_CH2);
    let mut uart = Uart::new_with_dma(
        p.UART2,
        tx_pin,
        rx_pin,
        tx_dma,
        rx_dma,
        Irqs,
        tx_staging,
        rx_staging,
        usart::Config::default(),
    )
    .unwrap();
    let mut outgoing = *b"*CW32 DMA hello*";
    let mut incoming = [0u8; 16];
    // Poll RX first, then TX; each operates through its own static staging.
    // Both caller buffers are ordinary local arrays in the task frame.
    {
        let (tx, rx) = uart.split_ref();
        let (received, sent) = join(rx.read(&mut incoming), tx.write(&outgoing)).await;
        received.unwrap();
        sent.unwrap();
        tx.flush().await.unwrap();
    }
    let (mut tx, mut rx) = uart.split();
    loop {
        outgoing.copy_from_slice(&incoming);
        let (received, sent) = join(rx.read(&mut incoming), tx.write(&outgoing)).await;
        received.unwrap();
        sent.unwrap();
        tx.flush().await.unwrap();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
