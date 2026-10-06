//! GPIO-only bring-up: LED and key only; motor pins are untouched.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embassy_cw32::gpio::{Input, Level, Output, Pull};

#[entry]
fn main() -> ! {
    defmt::info!("01 GPIO boot; motor gate pins untouched");
    // Default 4 MHz reset profile. No time-driver feature is selected.
    let p = embassy_cw32::init(Default::default());
    let clocks = embassy_cw32::rcc::clocks();
    defmt::info!(
        "nominal clocks: sysclk={} Hz hclk={} Hz pclk={} Hz",
        clocks.sysclk_hz(),
        clocks.hclk_hz(),
        clocks.pclk_hz()
    );
    let mut led = Output::new(p.PC13, Level::High);
    let key = Input::new(p.PA3, Pull::Up);
    let mut logged_pressed = key.is_low();
    defmt::info!("PA3 key_pressed={} (active low)", logged_pressed);
    // Limit diagnostics only; LED/key behavior remains immediate. This is a
    // loop-count divider, not a calibrated timebase or a key debouncer.
    let mut log_poll = 0u32;
    loop {
        let pressed = key.is_low();
        led.set_level(if pressed { Level::Low } else { Level::High });
        log_poll += 1;
        if log_poll == 100_000 {
            log_poll = 0;
            if pressed != logged_pressed {
                logged_pressed = pressed;
                defmt::info!("PA3 key_pressed={} PC13 led_on={}", pressed, pressed);
            }
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
