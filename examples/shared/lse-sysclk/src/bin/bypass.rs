#![no_std]
#![no_main]
use cw32_lse_sysclk_examples as board;
use embassy_cw32::{
    self as hal,
    rcc::{LseClock, LseMode},
    rtc::Rtc,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(board::config(LseMode::Bypass)).unwrap();
    // Borrow the already configured physical SYSCLK oscillator for RTC.
    #[cfg(any(
        feature = "cw32l010f8p6",
        feature = "cw32l010f8u6",
        feature = "cw32l010y8m6"
    ))]
    let clock = LseClock::new_bypass(p.SYSCTRL, p.PB1).unwrap();
    #[cfg(not(any(
        feature = "cw32l010f8p6",
        feature = "cw32l010f8u6",
        feature = "cw32l010y8m6"
    )))]
    let clock = LseClock::new_bypass(p.SYSCTRL, p.PC14).unwrap();
    core::hint::black_box((hal::rcc::clocks().sys_bounds(), clock.bounds()));
    let rtc =
        Rtc::initialize_if_unset(p.RTC, clock, Default::default(), board::initial_time()).unwrap();
    board::run(rtc)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
