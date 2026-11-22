#![no_std]
#![no_main]
use cw32_l010_lse_clock_examples as board;
use embassy_cw32::{self as hal, rcc::CalendarClock, rtc::Rtc};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    // Existing one-argument L010 HSIOSC constructor is retained.
    let clock = CalendarClock::new(p.SYSCTRL).unwrap();
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
