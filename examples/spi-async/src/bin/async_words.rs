//! 4/9/16-bit frame exchanges on each separately wired SPI loopback bus.
#![no_std]
#![no_main]

use cw32_spi_async_examples as examples;
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output},
    mode::Async,
    spi::{self, Spi},
};
use embedded_hal_async::spi::SpiBus;

#[cfg(any(feature = "cw32f020c6u7", feature = "cw32l052c8t6"))]
hal::bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<hal::peripherals::SPI1>;
    SPI2 => spi::InterruptHandler<hal::peripherals::SPI2>;
});

#[cfg(any(
    feature = "cw32l031c8t6",
    feature = "cw32r031c8u6",
))]
hal::bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<hal::peripherals::SPI1>;
});

#[cfg(any(
    feature = "cw32f002f3p7",
    feature = "cw32f003e4p7",
    feature = "cw32l010f8p6",
    feature = "cw32l011k8t6"
))]
hal::bind_interrupts!(struct Irqs {
    SPI => spi::InterruptHandler<hal::peripherals::SPI>;
});

#[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
hal::bind_interrupts!(struct Irqs {
    SPI1 => spi::InterruptHandler<hal::peripherals::SPI1>;
    SPI23 => spi::SharedInterruptHandler<hal::interrupt::typelevel::SPI23>;
});

type WordSamples = ([spi::word::U4; 2], [spi::word::U9; 3], [u16; 2]);

async fn exchange(bus: &mut Spi<'_, Async>) -> Result<WordSamples, spi::Error> {
    // Wider frames use the same PIO owner; each width switch occurs only
    // after the previous full-duplex frame and wire activity have settled.
    let mut four = [spi::word::U4(0xf), spi::word::U4(0xa)];
    SpiBus::<spi::word::U4>::transfer_in_place(bus, &mut four).await?;
    let mut nine = [spi::word::U9(0); 3];
    SpiBus::<spi::word::U9>::transfer(bus, &mut nine, &[spi::word::U9(0x1a5)]).await?;
    let mut sixteen = [0xa55au16, 0xffff];
    SpiBus::<u16>::transfer_in_place(bus, &mut sixteen).await?;
    SpiBus::<u16>::transfer(bus, &mut [], &[]).await?;
    SpiBus::<u16>::flush(bus).await?;
    Ok((four, nine, sixteen))
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let (peripheral, sck, mosi, miso, cs) = examples::primary_pins!(p);
    let mut primary_cs = Output::new(cs, Level::High, examples::cs_speed());
    let mut primary = Spi::new_async(peripheral, sck, mosi, miso, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    #[cfg(any(
        feature = "cw32f020c6u7",
        feature = "cw32l052c8t6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    ))]
    let (mut secondary, mut secondary_cs) = {
        let (peripheral, sck, mosi, miso, cs) = examples::secondary_pins!(p);
        let cs = Output::new(cs, Level::High, examples::cs_speed());
        let bus = Spi::new_async(peripheral, sck, mosi, miso, Irqs, examples::config())
            .unwrap_or_else(|_| examples::halt());
        (bus, cs)
    };
    #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    let (mut tertiary, mut tertiary_cs) = {
        let (peripheral, sck, mosi, miso, cs) = examples::tertiary_pins!(p);
        let cs = Output::new(cs, Level::High, examples::cs_speed());
        let bus = Spi::new_async(peripheral, sck, mosi, miso, Irqs, examples::config())
            .unwrap_or_else(|_| examples::halt());
        (bus, cs)
    };

    // Both L012 SPI23 peers stay alive here and both are used on every pass.
    // No operation is cancelled. An error parks before changing the active CS.
    loop {
        primary_cs.set_low();
        let samples = exchange(&mut primary)
            .await
            .unwrap_or_else(|_| examples::halt());
        primary_cs.set_high();
        core::hint::black_box(samples);
        #[cfg(any(
            feature = "cw32f020c6u7",
            feature = "cw32l052c8t6",
            feature = "cw32l012c8t6",
            feature = "cw32l012c8u6"
        ))]
        {
            secondary_cs.set_low();
            let samples = exchange(&mut secondary)
                .await
                .unwrap_or_else(|_| examples::halt());
            secondary_cs.set_high();
            core::hint::black_box(samples);
        }
        #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
        {
            tertiary_cs.set_low();
            let samples = exchange(&mut tertiary)
                .await
                .unwrap_or_else(|_| examples::halt());
            tertiary_cs.set_high();
            core::hint::black_box(samples);
        }
    }
}
