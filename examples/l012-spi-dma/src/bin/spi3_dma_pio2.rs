//! Deployment C: SPI3 DMA RX2/TX4 across both DMA vectors, plus SPI2 PIO.
#![no_std]
#![no_main]

use cw32_l012_spi_dma_examples as examples;
use embassy_cw32::{
    self as hal, dma,
    gpio::{Level, Output, Speed},
    peripherals, spi,
};

hal::bind_interrupts!(struct Irqs {
    SPI23 => spi::SharedInterruptHandler<hal::interrupt::typelevel::SPI23>;
    DMACH12 => dma::InterruptHandler<peripherals::DMA_CH2>;
    DMACH34 => dma::InterruptHandler<peripherals::DMA_CH4>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let tx3 = cortex_m::singleton!(: [u8; 16] = [0; 16]).unwrap_or_else(|| examples::halt());
    let rx3 = cortex_m::singleton!(: [u8; 24] = [0; 24]).unwrap_or_else(|| examples::halt());
    let mut cs2 = Output::new(p.PB1, Level::High, Speed::Default);
    let mut cs3 = Output::new(p.PB2, Level::High, Speed::Default);
    // The constructor takes TX before RX; RX CH2 has the higher priority.
    let mut spi3 = spi::Spi::new_with_dma(
        p.SPI3,
        p.PA9,
        p.PA11,
        p.PA10,
        p.DMA_CH4,
        p.DMA_CH2,
        Irqs,
        tx3,
        rx3,
        examples::config(),
    )
    .unwrap_or_else(|_| examples::halt());
    let mut spi2 = spi::Spi::new_async(p.SPI2, p.PA2, p.PA1, p.PA0, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());

    // Both SPI23 peers stay owned. Cancellation requires the same selection
    // until successful flush; unresolved/error paths provide no drain proof.
    loop {
        cs3.set_low();
        let samples = examples::exchange(&mut spi3)
            .await
            .unwrap_or_else(|_| examples::halt());
        cs3.set_high();
        core::hint::black_box(samples);
        cs2.set_low();
        let samples = examples::exchange(&mut spi2)
            .await
            .unwrap_or_else(|_| examples::halt());
        cs2.set_high();
        core::hint::black_box(samples);
    }
}
