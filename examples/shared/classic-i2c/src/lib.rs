//! Board configuration and bonded pin routes shared by the two master examples.
//! Attach a real target and external pull-ups as described in README.md.
#![no_std]

use embassy_cw32::i2c::Config;

/// Example board's unshifted seven-bit target address; change for your hardware.
pub const TARGET_ADDRESS: u8 = 0x48;
/// The example target accepts this pointer and exposes a two-byte read register.
pub const REGISTER_POINTER: [u8; 1] = [0x00];

/// Raw target bytes for each ordinary read pattern; no units are assumed.
pub struct Readings {
    pub direct: [u8; 2],
    pub combined: [u8; 2],
    pub adjacent: [u8; 2],
}

pub fn config() -> Config {
    // Default 100 kHz ceiling, with weak internal pull-ups disabled. External
    // pull-ups and the actual board's rise time must suit both target and MCU.
    Config::default()
}

/// Return the selected package's first (peripheral, SCL, SDA) route.
#[macro_export]
macro_rules! primary_pins {
    ($p:ident) => {{
        #[cfg(any(feature = "cw32f002f3p7", feature = "cw32f003e4p7"))]
        let pins = ($p.I2C, $p.PB1, $p.PA6);
        #[cfg(feature = "cw32l011k8t6")]
        let pins = ($p.I2C, $p.PB6, $p.PB7);
        #[cfg(feature = "cw32l010f8p6")]
        let pins = ($p.I2C1, $p.PA1, $p.PA0);
        #[cfg(any(
            feature = "cw32f030c8t7",
            feature = "cw32a030c8t7",
            feature = "cw32f020c6u7",
            feature = "cw32l031c8t6",
            feature = "cw32r031c8u6",
            feature = "cw32l052c8t6",
            feature = "cw32l083rct6"
        ))]
        let pins = ($p.I2C1, $p.PA9, $p.PA10);
        pins
    }};
}

/// Return the second bonded route, only for packages with two controllers.
#[cfg(any(
    feature = "cw32f030c8t7",
    feature = "cw32a030c8t7",
    feature = "cw32f020c6u7",
    feature = "cw32l052c8t6",
    feature = "cw32l083rct6"
))]
#[macro_export]
macro_rules! secondary_pins {
    ($p:ident) => {
        ($p.I2C2, $p.PA1, $p.PA2)
    };
}

/// Stop this example on an error; already transferred bytes are not retried.
pub fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    halt()
}
