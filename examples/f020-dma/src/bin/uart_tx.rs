#![no_std]
#![no_main]

use cw32_f020_dma_example::STAGING_SIZE;
use embassy_cw32::{
    self as hal, dma,
    mode::Async,
    peripherals,
    usart::{self, UartTx},
};
use embassy_executor::Spawner;

#[cfg(any(feature = "cw32f020c6u7", feature = "cw32f020k6u7"))]
hal::bind_interrupts!(struct Irqs {
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH2 => dma::InterruptHandler<peripherals::DMA_CH2>;
    UART1 => usart::InterruptHandler<peripherals::UART1>;
    UART2 => usart::InterruptHandler<peripherals::UART2>;
});

#[cfg(feature = "cw32f020f6u7")]
hal::bind_interrupts!(struct Irqs {
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH2 => dma::InterruptHandler<peripherals::DMA_CH2>;
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    UART3 => usart::InterruptHandler<peripherals::UART3>;
});

// This ordinary read-only flash slice exceeds each private 32-byte stage.
static FLASH_MESSAGE: &[u8] =
    b"CW32F020 staged UART DMA: ordinary flash bytes are copied into private SRAM chunks.\r\n";

#[embassy_executor::task(pool_size = 2)]
async fn transmit(mut uart: UartTx<'static, Async>, stream: u8) {
    // Each task has its own ordinary local array, also larger than its stage.
    let mut line = *b"Stream A: local RAM bytes, transmission 0\r\n";
    line[7] = stream;
    let digit = line.len() - 3;
    loop {
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

    #[cfg(any(feature = "cw32f020c6u7", feature = "cw32f020k6u7"))]
    let (uart1, pin1, uart2, pin2) = (p.UART1, p.PA8, p.UART2, p.PA2);
    #[cfg(feature = "cw32f020f6u7")]
    let (uart1, pin1, uart2, pin2) = (p.UART2, p.PA6, p.UART3, p.PA2);

    // The two owners retain separate UARTs, pins, physical channels and stages.
    let tx1 = UartTx::new_with_dma(
        uart1,
        pin1,
        p.DMA_CH1,
        Irqs,
        stage1,
        usart::Config::default(),
    )
    .unwrap();
    let tx2 = UartTx::new_with_dma(
        uart2,
        pin2,
        p.DMA_CH2,
        Irqs,
        stage2,
        usart::Config::default(),
    )
    .unwrap();
    spawner.spawn(transmit(tx1, b'A').unwrap());
    spawner.spawn(transmit(tx2, b'B').unwrap());
}
