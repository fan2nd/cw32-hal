//! Power-stage timing and PWM output helpers.
use crate::foc::Duty;
use embassy_cw32::{self as hal, peripherals, timer::{Channel, complementary_pwm::ComplementaryPwm, low_level::Timer}};
use crate::config::*;

pub fn wait_period<T: hal::timer::OverflowInstance>(timer: &mut Timer<'_, T>) {
    while !timer.take_overflow() {
        core::hint::spin_loop();
    }
}

pub fn wait_current_sample_phase<T: hal::timer::OverflowInstance>(timer: &mut Timer<'_, T>) {
    while timer.get_counter() < CURRENT_SAMPLE_TICKS {
        core::hint::spin_loop();
    }
}

pub fn apply_duty(pwm: &mut ComplementaryPwm<'_, peripherals::ATIM>, duty: Duty) {
    let max = pwm.get_max_duty();
    pwm.set_duty_cycle(Channel::Ch1, u32::from(duty.a) * max / 32_767);
    pwm.set_duty_cycle(Channel::Ch2, u32::from(duty.b) * max / 32_767);
    pwm.set_duty_cycle(Channel::Ch3, u32::from(duty.c) * max / 32_767);
}

