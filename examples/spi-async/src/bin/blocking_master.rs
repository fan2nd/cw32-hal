//! The same separately wired loopback buses through the existing blocking API.
#![no_std]
#![no_main]

use cw32_spi_async_examples::{self as examples, Samples};
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output},
    mode::Blocking,
    spi::{self, Spi},
};
use embedded_hal::spi::SpiBus;

fn exchange(bus: &mut Spi<'_, Blocking>) -> Result<Samples, spi::Error> {
    let mut samples = Samples {
        received: [0; 7],
        in_place: [0xa5, 0x5a, 0x00, 0xff],
    };
    SpiBus::transfer(bus, &mut samples.received, b"SPI")?;
    SpiBus::transfer_in_place(bus, &mut samples.in_place)?;
    SpiBus::write(bus, b"loopback")?;
    SpiBus::read(bus, &mut samples.received)?;
    SpiBus::<u8>::write(bus, &[])?;
    SpiBus::<u8>::read(bus, &mut [])?;
    SpiBus::<u8>::transfer(bus, &mut [], &[])?;
    SpiBus::<u8>::transfer_in_place(bus, &mut [])?;
    SpiBus::<u8>::flush(bus)?;
    // Loopback also demonstrates the existing 16-bit blocking word operation.
    // This does not extend the new async byte API or assume a slave protocol.
    let mut words = [0xa55au16, 0x00ff];
    bus.blocking_transfer_in_place(&mut words)?;
    core::hint::black_box(words);
    Ok(samples)
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let (peripheral, sck, mosi, miso, cs) = examples::primary_pins!(p);
    let mut primary_cs = Output::new(cs, Level::High, examples::cs_speed());
    let mut primary = Spi::new_blocking(peripheral, sck, mosi, miso, examples::config())
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
        let bus = Spi::new_blocking(peripheral, sck, mosi, miso, examples::config())
            .unwrap_or_else(|_| examples::halt());
        (bus, cs)
    };
    #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    let (mut tertiary, mut tertiary_cs) = {
        let (peripheral, sck, mosi, miso, cs) = examples::tertiary_pins!(p);
        let cs = Output::new(cs, Level::High, examples::cs_speed());
        let bus = Spi::new_blocking(peripheral, sck, mosi, miso, examples::config())
            .unwrap_or_else(|_| examples::halt());
        (bus, cs)
    };

    loop {
        primary_cs.set_low();
        let samples = exchange(&mut primary).unwrap_or_else(|_| examples::halt());
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
            let samples = exchange(&mut secondary).unwrap_or_else(|_| examples::halt());
            secondary_cs.set_high();
            core::hint::black_box(samples);
        }
        #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
        {
            tertiary_cs.set_low();
            let samples = exchange(&mut tertiary).unwrap_or_else(|_| examples::halt());
            tertiary_cs.set_high();
            core::hint::black_box(samples);
        }
    }
}
