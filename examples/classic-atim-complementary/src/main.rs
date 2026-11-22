//! Scope firmware for classic F030/A030 complementary PWM.
//! Small packages use CH1+CH3; larger packages additionally own CH2.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPair, ComplementaryPwmPin, Config},
        low_level::CountingMode,
        simple_pwm::PwmPin,
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
    #[cfg(any(feature = "cw32f030f6p7", feature = "cw32f030f8v7"))]
    let ch2 = None;
    #[cfg(not(any(feature = "cw32f030f6p7", feature = "cw32f030f8v7")))]
    let ch2 = Some(ComplementaryPwmPair::new(
        PwmPin::new(p.PA4),
        ComplementaryPwmPin::new(p.PB0),
    ));
    let mut pwm = ComplementaryPwm::new3(
        p.ATIM,
        Some(ComplementaryPwmPair::new(
            PwmPin::new(p.PA5),
            ComplementaryPwmPin::new(p.PA7),
        )),
        ch2,
        Some(ComplementaryPwmPair::new(
            PwmPin::new(p.PA3),
            ComplementaryPwmPin::new(p.PB1),
        )),
        Hertz(1_000),
        CountingMode::EdgeAlignedUp,
        Config {
            dead_time_ns: 1_000,
        },
    )
    .unwrap();
    // Constructor leaves MOE clear. Only interior reference comparisons are accepted.
    assert!(!pwm.get_master_output_enable());
    pwm.set_duty(Channel::Ch1, pwm.get_max_duty() / 4);
    pwm.set_duty(Channel::Ch3, pwm.get_max_duty() * 3 / 4);
    if pwm.has_channel(Channel::Ch2) {
        pwm.set_duty(Channel::Ch2, pwm.get_max_duty() / 2);
    }
    core::hint::black_box((
        pwm.frequency_bounds(),
        pwm.dead_time_bounds_ns(),
        pwm.dead_time_code(),
        pwm.prescaler(),
    ));
    pwm.set_master_output_enable(true);
    loop {
        core::hint::spin_loop();
    }
}
