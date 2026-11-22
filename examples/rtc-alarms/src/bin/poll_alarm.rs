#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    rtc::{Alarm, AlarmAConfig, AlarmDays, Rtc},
};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "hsi-source")]
    let clock = hal::rcc::HsiOscClock::new(p.SYSCTRL).unwrap();
    #[cfg(not(feature = "hsi-source"))]
    let clock = hal::rcc::LsiClock::new(p.SYSCTRL, 100_000).unwrap();
    // Provision with the rtc-calendar example first. Retained IRQ ownership
    // must already be inactive; this example never resets the calendar.
    let mut rtc = Rtc::attach_preserving_state(p.RTC, clock, Default::default()).unwrap();
    #[cfg(gpio_speed)]
    let speed = Speed::Low;
    #[cfg(not(gpio_speed))]
    let speed = Speed::Default;
    let mut indicator = Output::new(p.PA4, Level::Low, speed);
    rtc.set_alarm_a_enabled(false).unwrap();
    rtc.set_alarm_a(AlarmAConfig {
        days: AlarmDays::EVERY_DAY,
        hour: None,
        minute: None,
        second: Some(10),
    })
    .unwrap();
    rtc.clear_alarm(Alarm::A).unwrap();
    rtc.set_alarm_a_enabled(true).unwrap();
    loop {
        {
            let alarm = Alarm::A;
            if rtc.alarm_status(alarm).unwrap().pending {
                rtc.clear_alarm(alarm).unwrap();
                indicator.toggle();
            }
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
