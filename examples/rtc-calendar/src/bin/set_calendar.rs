#![no_std]
#![no_main]
use embassy_cw32::{self as hal, gpio::{Output, Level, Speed}, rtc::{Rtc, DateTime, DayOfWeek}};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p=hal::init(Default::default()).unwrap();
    #[cfg(feature="hsi-source")]
    let clock=hal::rcc::HsiOscClock::new(p.SYSCTRL).unwrap();
    #[cfg(not(feature="hsi-source"))]
    let clock=hal::rcc::LsiClock::new(p.SYSCTRL,100_000).unwrap();
    // Deliberate correction example: replace this value with the intended UTC time.
    let mut rtc=Rtc::attach_preserving_state(p.RTC,clock,Default::default()).unwrap();
    let requested=DateTime::from(2026,10,8,DayOfWeek::Thursday,12,0,0,0).unwrap();
    rtc.set_datetime(requested).unwrap();
    #[cfg(gpio_speed)]
    let speed=Speed::Low;
    #[cfg(not(gpio_speed))]
    let speed=Speed::Default;
    let mut indicator=Output::new(p.PA4,Level::Low,speed);
    let mut previous=None;
    loop {
        let now=rtc.now().unwrap();
        if previous != Some(now.second()) { indicator.toggle(); previous=Some(now.second()); }
        core::hint::black_box((now,rtc.calendar_tick_bounds()));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo)->! { loop { cortex_m::asm::bkpt(); } }
