#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal, dma,
    gpio::{Level, Output, Speed},
    peripherals, spi,
};
use embassy_executor::Spawner;
use embedded_hal_async::spi::SpiBus;

#[cfg(feature = "cw32f020c6u7")]
hal::bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<peripherals::SPI1>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH2 => dma::InterruptHandler<peripherals::DMA_CH2>;
});

#[cfg(not(feature = "cw32f020c6u7"))]
hal::bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<peripherals::SPI1>;
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>, dma::InterruptHandler<peripherals::DMA_CH3>;
});

static FLASH_BYTES: &[u8] = b"SPI DMA copies this flash slice through several private SRAM chunks.";

// All five async trait methods take ordinary borrowed slices. Unequal lengths,
// in-place replacement, zero-padding, empty operations and chunking are real
// production calls, not register-model tests or expected-device-data checks.
async fn exchange(bus: &mut impl SpiBus<u8>) {
    let mut received = [0u8; 43];
    let mut in_place = [0x55u8; 35];
    bus.transfer(&mut received, b"short TX").await.unwrap();
    bus.transfer_in_place(&mut in_place).await.unwrap();
    bus.write(FLASH_BYTES).await.unwrap();
    bus.read(&mut received).await.unwrap();
    bus.write(&[]).await.unwrap();
    bus.read(&mut []).await.unwrap();
    bus.transfer(&mut [], &[]).await.unwrap();
    bus.transfer_in_place(&mut []).await.unwrap();
    bus.flush().await.unwrap();
    core::hint::black_box((received, in_place));
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let tx = cortex_m::singleton!(: [u8; 16] = [0; 16]).unwrap();
    let rx = cortex_m::singleton!(: [u8; 24] = [0; 24]).unwrap();
    #[cfg(any(
        feature = "cw32f030c8t7",
        feature = "cw32a030c8t7",
        feature = "cw32f020c6u7"
    ))]
    let cs_speed = Speed::High;
    #[cfg(not(any(
        feature = "cw32f030c8t7",
        feature = "cw32a030c8t7",
        feature = "cw32f020c6u7"
    )))]
    let cs_speed = Speed::Default;
    let mut cs = Output::new(p.PB0, Level::High, cs_speed);
    #[cfg(feature = "cw32f020c6u7")]
    let (tx_channel, rx_channel) = (p.DMA_CH2, p.DMA_CH1);
    #[cfg(not(feature = "cw32f020c6u7"))]
    let (tx_channel, rx_channel) = (p.DMA_CH3, p.DMA_CH2);
    let mut bus = spi::Spi::new_with_dma(
        p.SPI1,
        p.PA5,
        p.PA7,
        p.PA6,
        tx_channel,
        rx_channel,
        Irqs,
        tx,
        rx,
        spi::Config::default(),
    )
    .unwrap();

    // RX has higher hardware priority: CH1 before TX CH2 on F020, otherwise
    // CH2 before TX CH3. This helps arbitration but is not a proof of lossless
    // performance at the chosen SCK.
    loop {
        cs.set_low();
        exchange(&mut bus).await;
        // Success already proves wire idle. If a caller adds cancellation, it
        // must retain this same CS selection and successfully await bus.flush()
        // before reaching set_high or selecting another device.
        cs.set_high();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // A DMA/SPI error leaves the bus quarantined. Do not change CS, reset the
    // peripheral or reuse its private memory to pretend the transfer stopped.
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
