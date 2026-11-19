//! Power-stage timing and PWM output helpers.
use crate::foc::Duty;
use embassy_cw32::{peripherals, timer::{Channel, complementary_pwm::ComplementaryPwm}};

pub fn apply_duty(pwm: &mut ComplementaryPwm<'_, peripherals::ATIM>, duty: Duty) {
    let max = pwm.get_max_duty();
    pwm.set_duty_cycle(Channel::Ch1, u32::from(duty.a) * max / 32_767);
    pwm.set_duty_cycle(Channel::Ch2, u32::from(duty.b) * max / 32_767);
    pwm.set_duty_cycle(Channel::Ch3, u32::from(duty.c) * max / 32_767);
}

