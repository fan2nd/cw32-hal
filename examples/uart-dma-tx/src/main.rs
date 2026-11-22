#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal, dma, peripherals,
    usart::{self, UartTx},
};
use embassy_executor::Spawner;

#[cfg(not(example_l083))]
hal::bind_interrupts!(struct Irqs {
    UART2 => usart::InterruptHandler<peripherals::UART2>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
});

#[cfg(example_l083)]
hal::bind_interrupts!(struct Irqs {
    UART2_UART5 => usart::SharedInterruptHandler<hal::interrupt::typelevel::UART2_UART5>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
});

// This read-only flash slice is larger than the private staging buffer.
static FLASH_MESSAGE: &[u8] =
    b"CW32 UART2 TX DMA: a flash-backed slice is copied into private SRAM in several chunks.\r\n";

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Normal reset/clean-runtime startup is required. init rejects dirty DMA
    // state; it is not a recovery path for an arbitrary active bootloader.
    let p = hal::init(Default::default()).unwrap();
    let staging = cortex_m::singleton!(: [u8; 32] = [0; 32]).unwrap();
    #[cfg(not(example_l083))]
    let tx_pin = p.PA2;
    #[cfg(example_l083)]
    let tx_pin = p.PA6;
    let mut uart = UartTx::new_with_dma(
        p.UART2,
        tx_pin,
        p.DMA_CH1,
        Irqs,
        staging,
        usart::Config::default(),
    )
    .unwrap();

    // An ordinary local array: no static allocation or unsafe call is needed
    // for write inputs. This line also exceeds the 32-byte staging capacity.
    let mut line = *b"RAM-backed local slice, transmission 0\r\n";
    let digit = line.len() - 3;
    loop {
        uart.write(FLASH_MESSAGE).await.unwrap();
        uart.write(&line).await.unwrap();
        // DMA completion reaches TDR; flush waits for the final wire transfer.
        uart.flush().await.unwrap();
        line[digit] = if line[digit] == b'9' {
            b'0'
        } else {
            line[digit] + 1
        };
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // A failed transfer is not restarted and its resources are not recovered.
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
