//! Ordinary byte exchanges on each separately wired SPI loopback bus.
#![no_std]
#![no_main]

use cw32_spi_async_examples::{self as examples, Samples};
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

async fn exchange(bus: &mut Spi<'_, Async>) -> Result<Samples, spi::Error> {
    let mut samples = Samples {
        received: [0; 7],
        in_place: [0xa5, 0x5a, 0x00, 0xff],
    };
    // All five calls below use the real byte SpiBus trait. Unequal lengths
    // clock seven bytes, padding the short TX slice with zeroes.
    SpiBus::<u8>::transfer(bus, &mut samples.received, b"SPI").await?;
    SpiBus::<u8>::transfer_in_place(bus, &mut samples.in_place).await?;
    SpiBus::<u8>::write(bus, b"loopback").await?;
    SpiBus::<u8>::read(bus, &mut samples.received).await?;
    SpiBus::<u8>::write(bus, &[]).await?;
    SpiBus::<u8>::read(bus, &mut []).await?;
    SpiBus::<u8>::transfer(bus, &mut [], &[]).await?;
    SpiBus::<u8>::transfer_in_place(bus, &mut []).await?;
    SpiBus::<u8>::flush(bus).await?;
    Ok(samples)
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
