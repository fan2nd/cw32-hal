#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal, dma,
    gpio::{Level, Output, Speed},
    peripherals, spi,
};
use embassy_executor::Spawner;

#[cfg(feature = "cw32f020c6u7")]
hal::bind_interrupts!(struct Irqs {
    SPI2 => spi::InterruptHandler<peripherals::SPI2>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH2 => dma::InterruptHandler<peripherals::DMA_CH2>;
});

#[cfg(feature = "cw32l052c8t6")]
hal::bind_interrupts!(struct Irqs {
    SPI2 => spi::InterruptHandler<peripherals::SPI2>;
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH4 => dma::InterruptHandler<peripherals::DMA_CH4>;
});

#[cfg(feature = "l083")]
hal::bind_interrupts!(struct Irqs {
    SPI2 => spi::InterruptHandler<peripherals::SPI2>;
    DMACH45 => dma::InterruptHandler<peripherals::DMA_CH4>, dma::InterruptHandler<peripherals::DMA_CH5>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let tx = cortex_m::singleton!(: [u8; 16] = [0; 16]).unwrap();
    let rx = cortex_m::singleton!(: [u8; 24] = [0; 24]).unwrap();
    #[cfg(feature = "cw32f020c6u7")]
    let cs_speed = Speed::High;
    #[cfg(not(feature = "cw32f020c6u7"))]
    let cs_speed = Speed::Default;
    let mut cs = Output::new(p.PB0, Level::High, cs_speed);
    #[cfg(feature = "cw32f020c6u7")]
    let (tx_channel, rx_channel) = (p.DMA_CH2, p.DMA_CH1);
    #[cfg(feature = "cw32l052c8t6")]
    let (tx_channel, rx_channel) = (p.DMA_CH4, p.DMA_CH1);
    #[cfg(feature = "l083")]
    let (tx_channel, rx_channel) = (p.DMA_CH5, p.DMA_CH4);
    let mut bus = spi::Spi::new_with_dma(
        p.SPI2,
        p.PA2,
        p.PA1,
        p.PA0,
        tx_channel,
        rx_channel,
        Irqs,
        tx,
        rx,
        spi::Config::default(),
    )
    .unwrap();

    // Connect a compatible SPI NOR flash: 0x9f reads its three-byte JEDEC ID.
    // Commands and dummy bytes are transferred through paired byte DMA.
    let mut identification = [0x9fu8, 0, 0, 0];
    cs.set_low();
    bus.transfer_in_place(&mut identification).await.unwrap();
    bus.flush().await.unwrap();
    cs.set_high();
    let jedec_id = [identification[1], identification[2], identification[3]];
    core::hint::black_box(jedec_id);

    loop {
        // Read status register 1 without modifying the flash. The logical
        // write discards RX; the logical read sends zero dummy bytes. Both
        // still use the TX/RX DMA pair, with CS held across the two calls.
        let mut status = [0u8; 1];
        cs.set_low();
        bus.write(&[0x05u8]).await.unwrap();
        bus.read(&mut status).await.unwrap();
        bus.flush().await.unwrap();
        cs.set_high();
        core::hint::black_box((jedec_id, status[0]));
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // An error does not prove bus drain. Retain CS and the quarantined pair.
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
