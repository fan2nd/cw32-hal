//! ADC and timer-trigger setup for the motor-control example.
use embassy_cw32 as hal;
use embassy_cw32::adc::SampleTime;
use embassy_cw32::interrupt::typelevel::{ADC1, Interrupt};
use embassy_cw32::timer::Channel;

pub fn current_loop(period: u32) {
    hal::adc::configure_timer_triggered::<hal::peripherals::ADC1>(
        [8, 9],
        SampleTime::Cycles30,
        1,
        hal::adc::AdcTrigger::AtimCc4,
    );
    let compare = period.saturating_sub(5).min(65_535) as u16;
    hal::timer::complementary_pwm::irq_cc4::<hal::peripherals::ATIM>(compare);
    ADC1::set_priority(hal::interrupt::Priority::P0);
    unsafe { ADC1::enable() };
    defmt::info!(
        "current loop armed: adc1 timer-triggered scan ccr4={=u16}",
        hal::timer::complementary_pwm::irq_ccr_read::<hal::peripherals::ATIM>(Channel::Ch4)
    );
}

pub fn auxiliary() {
    hal::adc::configure_continuous::<hal::peripherals::ADC2>([5, 5], SampleTime::Cycles518, 3);
    cortex_m::asm::delay(8_000);
}
