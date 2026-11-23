#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    rtc::{Alarm, AlarmAConfig, AlarmDays, Rtc},
};
hal::bind_interrupts!(struct Irqs { RTC => hal::rtc::AlarmInterruptHandler; });
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let clock = hal::rcc::HsiOscClock::new(p.SYSCTRL).unwrap();
    // Provision the retained RTC before this example. HSIOSC stays running.
    let mut rtc = Rtc::attach_preserving_state(p.RTC, clock, Default::default()).unwrap();
    let mut indicator = Output::new(p.PA4, Level::Low, Speed::Default);
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
            rtc.wait_for_alarm(alarm, Irqs).await.unwrap();
            // Waits preserve sticky flags; acknowledgement is explicit.
            rtc.clear_alarm(alarm).unwrap();
            indicator.toggle();
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
