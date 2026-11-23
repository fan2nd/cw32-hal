//! Ordered, repeated external inputs; channel handles own both pin tokens.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, AdcChannel, Config, SampleTime},
    time::Hertz,
};
#[cortex_m_rt::entry]
fn main() -> ! {
    // Example board declaration: actual supply 3.0–3.6 V, ambient -40..85 C.
    // L011 requires VDDA=VDD. PA0/PA1 inputs must stay within the supply rails.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div8;
    let p = hal::init(system).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(24_000_000);
    let mut adc = Adc::new(p.ADC, config).unwrap();
    let first = p.PA0.degrade_adc();
    let second = p.PA1.degrade_adc();
    let sequence = [
        (&first, SampleTime::Cycles54),
        (&second, SampleTime::Cycles102),
        (&first, SampleTime::Cycles390),
    ];
    let mut samples = [0; 3];
    loop {
        let timing = adc.blocking_read_sequence(&sequence, &mut samples).unwrap();
        core::hint::black_box((samples, timing.maximum_conversion_time_ns()));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
