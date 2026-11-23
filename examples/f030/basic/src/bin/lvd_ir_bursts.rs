#![no_std]
#![no_main]

use cw32f030_examples as _;
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    ir::{IrModulator, Mode},
    lvd::{LowVoltageMonitor, Threshold, VoltageState},
    time::Hertz,
    timer::{low_level::CountingMode, simple_pwm::SimplePwm},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    // CW32F030C8T7. PB9 is IR_OUT, PB0 is a low-voltage indication.
    // Use suitable external transistor/LED/current-limiting circuitry on PB9;
    // this does not drive an IR LED directly and is not a receiver protocol.
    let p = hal::init(Default::default()).unwrap();
    let mut indicator = Output::new(p.PB0, Level::Low, Speed::Low);
    let monitor =
        LowVoltageMonitor::new_supply(p.LVD, Threshold::from_millivolts(3000).unwrap()).unwrap();

    // Configure the two source timers independently, without physical PWM pins.
    // This sends continuous nominal 1-kHz bursts of a nominal 38-kHz carrier.
    // Actual timer frequencies retain the factory-HSI tolerance and divisors.
    let mut carrier = SimplePwm::new(
        p.GTIM1,
        None,
        None,
        None,
        None,
        Hertz(38_000),
        CountingMode::EdgeAlignedUp,
    )
    .unwrap();
    let mut bursts = SimplePwm::new(
        p.GTIM2,
        None,
        None,
        None,
        None,
        Hertz(1_000),
        CountingMode::EdgeAlignedUp,
    )
    .unwrap();
    let carrier_duty = carrier.max_duty_cycle() / 5;
    carrier.ch1().set_duty_cycle(carrier_duty);
    carrier.ch1().enable();
    let burst_duty = bursts.max_duty_cycle() / 2;
    bursts.ch1().set_duty_cycle(burst_duty);
    bursts.ch1().enable();

    let mut ir = IrModulator::attach(p.IR);
    ir.set_mode(Mode::Gtim1Ch1AndGtim2Ch1);
    let _ir = ir.with_output(p.PB9);

    // Illustrative settling pause only. Qualify analog startup and board timing
    // on the target before relying on the indication; no silicon run is claimed.
    cortex_m::asm::delay(80_000);
    loop {
        indicator.set_level(if monitor.sample() == VoltageState::BelowThreshold {
            Level::High
        } else {
            Level::Low
        });
        cortex_m::asm::delay(80_000);
    }
}
