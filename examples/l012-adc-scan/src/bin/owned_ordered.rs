//! Owned ordered scans on both independently owned converters.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, AdcChannel, Config, SampleTime},
    time::Hertz,
};
#[cortex_m_rt::entry]
fn main() -> ! {
    // Board contract: VDDA=VDD, 3.0–3.6 V, qualified HSI temperature range.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div8;
    let p = hal::init(system).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(24_000_000);
    let mut adc1 = Adc::new(p.ADC1, config).unwrap();
    let mut adc2 = Adc::new(p.ADC2, config).unwrap();
    let first = p.PA0.degrade_adc();
    let second = p.PA1.degrade_adc();
    let sibling = p.PA8.degrade_adc();
    let sequence = [
        (&second, SampleTime::Cycles54),
        (&first, SampleTime::Cycles102),
        (&second, SampleTime::Cycles518),
    ];
    let mut samples = [0; 3];
    let timing = adc1
        .blocking_read_sequence(&sequence, &mut samples)
        .unwrap();
    core::hint::black_box((samples, timing.maximum_conversion_time_ns()));
    drop(adc1);
    let sequence2 = [
        (&sibling, SampleTime::Cycles54),
        (&sibling, SampleTime::Cycles262),
    ];
    let mut samples2 = [0; 2];
    loop {
        let timing2 = adc2
            .blocking_read_sequence(&sequence2, &mut samples2)
            .unwrap();
        core::hint::black_box((samples2, timing2));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
