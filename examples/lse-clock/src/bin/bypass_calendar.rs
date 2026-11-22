#![no_std]
#![no_main]
use cw32_lse_clock_examples as board;
use embassy_cw32::{
    self as hal,
    rcc::{LseClock, LseMode},
    rtc::Rtc,
};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(board::config(LseMode::Bypass)).unwrap();
    let clock = LseClock::new_bypass(p.SYSCTRL, p.PC14).unwrap();
    // Deliberate cold provisioning. Use attach_preserving_state for a retained calendar.
    let rtc =
        Rtc::initialize_if_unset(p.RTC, clock, Default::default(), board::initial_time()).unwrap();
    board::display(rtc, p.PA4)
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
