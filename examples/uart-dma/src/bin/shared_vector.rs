#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal, dma, peripherals,
    usart::{self, Uart},
};
use embassy_executor::Spawner;
use embassy_futures::join::join;

hal::bind_interrupts!(struct Irqs {
    UART1_UART4 => usart::SharedInterruptHandler<hal::interrupt::typelevel::UART1_UART4>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>;
    DMACH45 => dma::InterruptHandler<peripherals::DMA_CH4>, dma::InterruptHandler<peripherals::DMA_CH5>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let tx1_staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();
    let rx1_staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();
    let tx4_staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();
    let rx4_staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();

    // These routes are bonded on every qualified exact L083 package.
    // Each RX channel has higher DMA priority than its corresponding TX.
    let mut uart1 = Uart::new_with_dma(
        p.UART1,
        p.PA8,
        p.PA9,
        p.DMA_CH2,
        p.DMA_CH1,
        Irqs,
        tx1_staging,
        rx1_staging,
        usart::Config::default(),
    )
    .unwrap();
    let mut uart4 = Uart::new_with_dma(
        p.UART4,
        p.PA4,
        p.PB5,
        p.DMA_CH5,
        p.DMA_CH4,
        Irqs,
        tx4_staging,
        rx4_staging,
        usart::Config::default(),
    )
    .unwrap();

    let mut outgoing1: [u8; 16] = *b"*UART1 DMA ping*";
    let mut outgoing4: [u8; 16] = *b"*UART4 DMA ping*";
    let mut incoming1 = [0u8; 16];
    let mut incoming4 = [0u8; 16];
    loop {
        // Retain both UART owners outside the joined futures. Both shared-IRQ
        // partners remain live until both full-duplex exchanges and flushes
        // finish, including if one peer answers before the other.
        let (tx1, rx1) = uart1.split_ref();
        let (tx4, rx4) = uart4.split_ref();
        join(
            async {
                let (received, sent) = join(rx1.read(&mut incoming1), tx1.write(&outgoing1)).await;
                received.unwrap();
                sent.unwrap();
                tx1.flush().await.unwrap();
            },
            async {
                let (received, sent) = join(rx4.read(&mut incoming4), tx4.write(&outgoing4)).await;
                received.unwrap();
                sent.unwrap();
                tx4.flush().await.unwrap();
            },
        )
        .await;
        outgoing1.copy_from_slice(&incoming1);
        outgoing4.copy_from_slice(&incoming4);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // Neither shared-vector partner nor an unfinished DMA lease is recovered.
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
