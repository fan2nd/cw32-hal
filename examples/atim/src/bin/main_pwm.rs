//! 1 kHz, 25% active-high main-output PWM. See README for the exact output pad.
//! Disconnect power stages and use a scope/logic analyzer with a suitable load.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    time::Hertz,
    timer::{
        low_level::CountingMode,
        simple_pwm::{PwmPin, SimplePwm},
    },
};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    #[cfg(feature = "cw32f003e4p7")]
    let pin = PwmPin::new(p.PB6);
    #[cfg(feature = "cw32l010f8p6")]
    let pin = PwmPin::new(p.PB4);
    #[cfg(not(any(feature = "cw32f003e4p7", feature = "cw32l010f8p6")))]
    let pin = PwmPin::new(p.PA5);
    #[cfg(buffered_atim_example)]
    let mut pwm = SimplePwm::new(
        p.ATIM,
        Some(pin),
        None,
        None,
        None,
        Hertz(1000),
        CountingMode::EdgeAlignedUp,
    )
    .unwrap();
    #[cfg(not(buffered_atim_example))]
    let mut pwm = SimplePwm::new3(
        p.ATIM,
        Some(pin),
        None,
        None,
        Hertz(1000),
        CountingMode::EdgeAlignedUp,
    )
    .unwrap();
    pwm.ch1().set_duty_cycle_percent(25);
    pwm.ch1().enable();
    core::hint::black_box(pwm.frequency_bounds());
    loop {
        core::hint::spin_loop();
    }
}
