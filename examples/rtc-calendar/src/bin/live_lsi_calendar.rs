#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal,
    rcc::LsiClock,
    rtc::{DateTime, DayOfWeek, Rtc},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // Board startup or a bootloader must already have software-enabled LSI,
    // loaded its own factory trim, and reached STABLE. Ordinary POR is not enough.
    // Default system initialization keeps an HSI CPU; the RTC owns LSI separately.
    let p = hal::init(Default::default()).unwrap();
    let clock = LsiClock::new(p.SYSCTRL).unwrap();

    // Deliberate stopped-unset provisioning: review and replace this sample time
    // before flashing, and permit the existing RTC output/consumer handover.
    let initial = DateTime::from(2026, 10, 12, DayOfWeek::Monday, 12, 0, 0, 0).unwrap();
    let rtc = Rtc::initialize_if_unset(p.RTC, clock, Default::default(), initial).unwrap();

    loop {
        let now = rtc.now().unwrap();
        // Both envelopes are rate-only. Strict duration helpers are unsupported.
        core::hint::black_box((now, rtc.source_clock_bounds(), rtc.calendar_tick_bounds()));
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
