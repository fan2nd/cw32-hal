//! Ordinary byte exchanges shared by the four native L012 deployments.
#![no_std]

use embassy_cw32::spi::Config;
use embedded_hal_async::spi::SpiBus;

// An immutable flash slice longer than either owner's 16-byte chunk capacity.
static FLASH_BYTES: &[u8] = b"Native L012 SPI copies this flash slice through private SRAM chunks.";

/// Default internal clocks, mode 0, MSB first, and a 1 MHz requested ceiling.
pub fn config() -> Config {
    Config::default()
}

/// Observations for debugger inspection, with no external-device assertion.
pub struct Samples {
    pub padded_rx: [u8; 43],
    pub short_rx: [u8; 7],
    pub in_place: [u8; 35],
    pub read_rx: [u8; 19],
}

/// Use all five real async byte `SpiBus` methods on a DMA or PIO owner.
///
/// Keep this bus's original GPIO CS asserted until this sequence, including
/// its final flush, succeeds. Cancellation or an error provides no wire-drain
/// proof; a cancelled sequence requires a later successful flush before CS
/// release. These applications never cancel, retry, or recreate an owner.
pub async fn exchange<B: SpiBus<u8>>(bus: &mut B) -> Result<Samples, B::Error> {
    let mut samples = Samples {
        padded_rx: [0; 43],
        short_rx: [0; 7],
        in_place: [0x55; 35],
        read_rx: [0; 19],
    };
    // The shorter TX tail is zero-filled; the longer TX discards excess RX.
    SpiBus::transfer(bus, &mut samples.padded_rx, b"short TX").await?;
    SpiBus::transfer(bus, &mut samples.short_rx, FLASH_BYTES).await?;
    SpiBus::transfer_in_place(bus, &mut samples.in_place).await?;
    SpiBus::write(bus, FLASH_BYTES).await?;
    SpiBus::read(bus, &mut samples.read_rx).await?;
    // Empty operations settle previous work without starting new traffic.
    SpiBus::write(bus, &[]).await?;
    SpiBus::read(bus, &mut []).await?;
    SpiBus::transfer(bus, &mut [], &[]).await?;
    SpiBus::transfer_in_place(bus, &mut []).await?;
    SpiBus::flush(bus).await?;
    Ok(samples)
}

/// Park without unwinding, releasing owners, changing CS, or retrying bytes.
pub fn halt() -> ! {
    cortex_m::interrupt::disable();
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    halt()
}
