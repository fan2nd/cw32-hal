//! Board assumptions shared by the L012 command/FIFO master examples.
//! Qualify real targets, external pull-ups and timing as described in README.md.
#![no_std]

use embassy_cw32::{i2c::Config, time::Hertz};

/// Unshifted seven-bit target address on each of the two separate example buses.
pub const TARGET_ADDRESS: u8 = 0x48;
/// The target must accept a pointer-only write and expose a two-byte register.
pub const REGISTER_POINTER: [u8; 1] = [0x00];

/// Raw bytes from each read pattern; interpretation depends on the real target.
pub struct Readings {
    pub direct: [u8; 2],
    pub combined: [u8; 2],
    pub adjacent: [u8; 2],
}

pub fn config() -> Config {
    let mut config = Config::default();
    // The driver derives its timing only from the frozen PCLK envelope.
    // These rise bounds are board assumptions, not measured or guaranteed
    // values. Qualify both buses and increase them for slower physical edges.
    config.frequency = Hertz(100_000);
    config.scl_rise_time_ns = 100;
    config.sda_rise_time_ns = 100;
    config.scl_filter_cycles = 0;
    config.sda_filter_cycles = 0;
    config.scl_pullup = false;
    config.sda_pullup = false;
    config.poll_limit = 100_000;
    config
}

/// Stop on an error. Bytes already transferred are never retried by the example.
pub fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt()
}
