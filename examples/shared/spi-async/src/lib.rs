//! Exact-package wiring shared by the ordinary SPI loopback applications.
#![no_std]

use embassy_cw32::{gpio::Speed, spi::Config};

/// Default internal clocks, mode 0, MSB first, and a 1 MHz requested ceiling.
pub fn config() -> Config {
    Config::default()
}

/// F020 exposes drive-speed control; the other selected families do not.
pub fn cs_speed() -> Speed {
    #[cfg(feature = "cw32f020c6u7")]
    return Speed::Low;
    #[cfg(not(feature = "cw32f020c6u7"))]
    return Speed::Default;
}

/// Observations can be inspected with a debugger; no loopback result is asserted.
pub struct Samples {
    pub received: [u8; 7],
    pub in_place: [u8; 4],
}

/// Return the exact primary (peripheral, SCK, MOSI, MISO, software CS) route.
#[macro_export]
macro_rules! primary_pins {
    ($p:ident) => {{
        #[cfg(any(feature = "cw32f002f3p7", feature = "cw32f003e4p7"))]
        let pins = ($p.SPI, $p.PA0, $p.PA1, $p.PA4, $p.PB0);
        #[cfg(feature = "cw32l010f8p6")]
        let pins = ($p.SPI, $p.PA5, $p.PA6, $p.PA3, $p.PA4);
        #[cfg(feature = "cw32l011k8t6")]
        let pins = ($p.SPI, $p.PA5, $p.PA7, $p.PA6, $p.PB0);
        #[cfg(any(
            feature = "cw32f020c6u7",
            feature = "cw32l012c8t6",
            feature = "cw32l012c8u6",
            feature = "cw32l031c8t6",
            feature = "cw32r031c8u6",
            feature = "cw32l052c8t6"
        ))]
        let pins = ($p.SPI1, $p.PA5, $p.PA7, $p.PA6, $p.PB0);
        pins
    }};
}

/// Return the separate second bus on F020, L052 and L012.
#[cfg(any(
    feature = "cw32f020c6u7",
    feature = "cw32l052c8t6",
    feature = "cw32l012c8t6",
    feature = "cw32l012c8u6"
))]
#[macro_export]
macro_rules! secondary_pins {
    ($p:ident) => {
        ($p.SPI2, $p.PA2, $p.PA1, $p.PA0, $p.PB1)
    };
}

/// L012's third bus shares SPI23 with SPI2, but owns separate pins and CS.
#[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
#[macro_export]
macro_rules! tertiary_pins {
    ($p:ident) => {
        ($p.SPI3, $p.PA9, $p.PA11, $p.PA10, $p.PB2)
    };
}

/// Park without unwinding, dropping the bus, changing CS, or retrying bytes.
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
