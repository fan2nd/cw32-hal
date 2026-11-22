#![no_std]
#![no_main]

use cw32_four_channel_dma_example::STAGING_SIZE;
use embassy_cw32::{
    self as hal, dma,
    mode::Async,
    peripherals,
    usart::{self, UartTx},
};
use embassy_executor::Spawner;

#[cfg(not(any(feature = "cw32l031f8p6", feature = "cw32l031f8u6")))]
hal::bind_interrupts!(struct Irqs {
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>,
               dma::InterruptHandler<peripherals::DMA_CH3>;
    DMACH4 => dma::InterruptHandler<peripherals::DMA_CH4>;
    UART1 => usart::InterruptHandler<peripherals::UART1>;
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    UART3 => usart::InterruptHandler<peripherals::UART3>;
});

#[cfg(any(feature = "cw32l031f8p6", feature = "cw32l031f8u6"))]
hal::bind_interrupts!(struct Irqs {
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>,
               dma::InterruptHandler<peripherals::DMA_CH3>;
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    UART3 => usart::InterruptHandler<peripherals::UART3>;
});

// This ordinary read-only flash slice exceeds each private 32-byte stage.
static FLASH_MESSAGE: &[u8] =
    b"CW32 four-channel staged UART DMA: flash bytes are copied into private SRAM chunks.\r\n";

#[cfg_attr(
    any(feature = "cw32l031f8p6", feature = "cw32l031f8u6"),
    embassy_executor::task(pool_size = 2)
)]
#[cfg_attr(
    not(any(feature = "cw32l031f8p6", feature = "cw32l031f8u6")),
    embassy_executor::task(pool_size = 3)
)]
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
    let stage2 = cortex_m::singleton!(: [u8; STAGING_SIZE] = [0; STAGING_SIZE]).unwrap();
    let stage3 = cortex_m::singleton!(: [u8; STAGING_SIZE] = [0; STAGING_SIZE]).unwrap();

    #[cfg(any(
        feature = "cw32l052c8t6",
        feature = "cw32l052r8s6",
        feature = "cw32l052r8t6"
    ))]
    let uart3_pin = p.PA9;
    #[cfg(not(any(
        feature = "cw32l052c8t6",
        feature = "cw32l052r8s6",
        feature = "cw32l052r8t6"
    )))]
    let uart3_pin = p.PA10;

    // Separate UARTs, pins, physical channels and stages share only DMACH23.
    let tx2 = UartTx::new_with_dma(
        p.UART2,
        p.PA6,
        p.DMA_CH2,
        Irqs,
        stage2,
        usart::Config::default(),
    )
    .unwrap();
    let tx3 = UartTx::new_with_dma(
        p.UART3,
        uart3_pin,
        p.DMA_CH3,
        Irqs,
        stage3,
        usart::Config::default(),
    )
    .unwrap();
    spawner.spawn(transmit(tx2, b'2').unwrap());
    spawner.spawn(transmit(tx3, b'3').unwrap());

    // F8 packages have no safe UART1 TX route; PA14 remains excluded.
    #[cfg(not(any(feature = "cw32l031f8p6", feature = "cw32l031f8u6")))]
    {
        let stage1 = cortex_m::singleton!(: [u8; STAGING_SIZE] = [0; STAGING_SIZE]).unwrap();
        let tx1 = UartTx::new_with_dma(
            p.UART1,
            p.PA8,
            p.DMA_CH4,
            Irqs,
            stage1,
            usart::Config::default(),
        )
        .unwrap();
        spawner.spawn(transmit(tx1, b'1').unwrap());
    }
}
