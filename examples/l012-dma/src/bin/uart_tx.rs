#![no_std]
#![no_main]

use cw32_l012_dma_example::STAGING_SIZE;
use embassy_cw32::{
    self as hal, dma,
    mode::Async,
    peripherals,
    usart::{self, UartTx},
};
use embassy_executor::Spawner;

hal::bind_interrupts!(struct Irqs {
    DMACH12 => dma::InterruptHandler<peripherals::DMA_CH1>,
               dma::InterruptHandler<peripherals::DMA_CH2>;
    DMACH34 => dma::InterruptHandler<peripherals::DMA_CH4>;
    UART1 => usart::InterruptHandler<peripherals::UART1>;
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    UART3 => usart::InterruptHandler<peripherals::UART3>;
});

// This ordinary read-only flash slice exceeds each private 32-byte stage.
static FLASH_MESSAGE: &[u8] =
    b"CW32L012 staged UART DMA: flash bytes are copied into private SRAM chunks.\r\n";

#[embassy_executor::task(pool_size = 3)]
async fn transmit(mut uart: UartTx<'static, Async>, stream: u8) {
    // Each task owns an ordinary local array, also longer than its stage.
    let mut line = *b"UART 0: local RAM bytes, transmission 0\r\n";
    line[5] = stream;
    let digit = line.len() - 3;
    loop {
        // The driver CPU-copies both caller slices into its private SRAM stage.
        uart.write(FLASH_MESSAGE).await.unwrap();
        uart.write(&line).await.unwrap();
        // DMA completion reaches TDR. flush also waits for wire completion.
        uart.flush().await.unwrap();
        line[digit] = if line[digit] == b'9' {
            b'0'
        } else {
            line[digit] + 1
        };
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let stage1 = cortex_m::singleton!(: [u8; STAGING_SIZE] = [0; STAGING_SIZE]).unwrap();
    let stage2 = cortex_m::singleton!(: [u8; STAGING_SIZE] = [0; STAGING_SIZE]).unwrap();
    let stage3 = cortex_m::singleton!(: [u8; STAGING_SIZE] = [0; STAGING_SIZE]).unwrap();

    // UART1 and UART2 share DMACH12, with separate channels and stages.
    let tx1 = UartTx::new_with_dma(
        p.UART1,
        p.PA9,
        p.DMA_CH1,
        Irqs,
        stage1,
        usart::Config::default(),
    )
    .unwrap();
    let tx2 = UartTx::new_with_dma(
        p.UART2,
        p.PA7,
        p.DMA_CH2,
        Irqs,
        stage2,
        usart::Config::default(),
    )
    .unwrap();
    // CH3 is unused here; owned_copy binds both DMACH34 siblings.
    let tx3 = UartTx::new_with_dma(
        p.UART3,
        p.PA8,
        p.DMA_CH4,
        Irqs,
        stage3,
        usart::Config::default(),
    )
    .unwrap();
    spawner.spawn(transmit(tx1, b'1').unwrap());
    spawner.spawn(transmit(tx2, b'2').unwrap());
    spawner.spawn(transmit(tx3, b'3').unwrap());
}
