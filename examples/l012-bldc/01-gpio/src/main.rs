//! GPIO-only bring-up: LED and key only; motor pins are untouched.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_cw32::gpio::{Input, Level, Output, Pull};

#[entry]
fn main() -> ! {
    // Default 4 MHz reset profile. No time-driver feature is selected.
    let p = embassy_cw32::init(Default::default());
    let mut led = Output::new(p.PC13, Level::High);
    let key = Input::new(p.PA3, Pull::Up);
    loop {
        led.set_level(if key.is_low() {
            Level::Low
        } else {
            Level::High
        });
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
