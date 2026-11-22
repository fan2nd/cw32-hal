//! Deployment D: SPI2 DMA RX1/TX2 and SPI3 DMA RX3/TX4, plus SPI1 PIO.
#![no_std]
#![no_main]

use cw32_l012_spi_dma_examples as examples;
use embassy_cw32::{
    self as hal, dma,
    gpio::{Level, Output, Speed},
    peripherals, spi,
};

hal::bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<peripherals::SPI1>;
    SPI23 => spi::SharedInterruptHandler<hal::interrupt::typelevel::SPI23>;
    DMACH12 => dma::InterruptHandler<peripherals::DMA_CH1>, dma::InterruptHandler<peripherals::DMA_CH2>;
    DMACH34 => dma::InterruptHandler<peripherals::DMA_CH3>, dma::InterruptHandler<peripherals::DMA_CH4>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let tx2 = cortex_m::singleton!(: [u8; 16] = [0; 16]).unwrap_or_else(|| examples::halt());
    let rx2 = cortex_m::singleton!(: [u8; 24] = [0; 24]).unwrap_or_else(|| examples::halt());
    let tx3 = cortex_m::singleton!(: [u8; 16] = [0; 16]).unwrap_or_else(|| examples::halt());
    let rx3 = cortex_m::singleton!(: [u8; 24] = [0; 24]).unwrap_or_else(|| examples::halt());
    let mut cs1 = Output::new(p.PB0, Level::High, Speed::Default);
    let mut cs2 = Output::new(p.PB1, Level::High, Speed::Default);
    let mut cs3 = Output::new(p.PB2, Level::High, Speed::Default);
    // Constructors take TX before RX; each RX has the higher pair priority.
    let mut spi2 = spi::Spi::new_with_dma(
        p.SPI2,
        p.PA2,
        p.PA1,
        p.PA0,
        p.DMA_CH2,
        p.DMA_CH1,
        Irqs,
        tx2,
        rx2,
        examples::config(),
    )
    .unwrap_or_else(|_| examples::halt());
    let mut spi3 = spi::Spi::new_with_dma(
        p.SPI3,
        p.PA9,
        p.PA11,
        p.PA10,
        p.DMA_CH4,
        p.DMA_CH3,
        Irqs,
        tx3,
        rx3,
        examples::config(),
    )
    .unwrap_or_else(|_| examples::halt());
    let mut spi1 = spi::Spi::new_async(p.SPI1, p.PA5, p.PA7, p.PA6, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());

    // All four channels remain reserved; no owner is dropped and re-admitted.
    // Retain the same CS until successful flush, including after cancellation.
    // An unresolved/error path proves no drain and parks before CS is changed.
    loop {
        cs2.set_low();
        let samples = examples::exchange(&mut spi2)
            .await
            .unwrap_or_else(|_| examples::halt());
        cs2.set_high();
        core::hint::black_box(samples);
        cs3.set_low();
        let samples = examples::exchange(&mut spi3)
            .await
            .unwrap_or_else(|_| examples::halt());
        cs3.set_high();
        core::hint::black_box(samples);
        cs1.set_low();
        let samples = examples::exchange(&mut spi1)
            .await
            .unwrap_or_else(|_| examples::halt());
        cs1.set_high();
        core::hint::black_box(samples);
    }
}
