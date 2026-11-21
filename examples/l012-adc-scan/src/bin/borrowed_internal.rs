//! Eight ADC1 slots and single reads while ADC2 remains independently owned.
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
    system.rcc.operating_conditions.min_supply_mv = 3000;
    system.rcc.operating_conditions.max_supply_mv = 3600;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div8;
    let p = hal::init(system);
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(24_000_000);
    let mut adc1 = Adc::new(p.ADC1, config).unwrap();
    let mut adc2 = Adc::new(p.ADC2, config).unwrap();
    let mut first = p.PA0;
    let mut second = p.PA1;
    let mut sibling = p.PA8;
    let temperature = adc1.enable_temperature().degrade_adc();
    let bandgap = adc1.enable_vrefint().degrade_adc();
    loop {
        {
            let a = first.reborrow_adc();
            let b = second.reborrow_adc();
            let sequence = [
                (&b, SampleTime::Cycles54),
                (&a, SampleTime::Cycles102),
                (&temperature, SampleTime::Cycles518),
                (&bandgap, SampleTime::Cycles518),
                (&a, SampleTime::Cycles262),
                (&b, SampleTime::Cycles54),
                (&temperature, SampleTime::Cycles518),
                (&bandgap, SampleTime::Cycles518),
            ];
            let mut samples = [0; 8];
            let timing = adc1
                .blocking_read_sequence(&sequence, &mut samples)
                .unwrap();
            core::hint::black_box((samples, timing));
        }
        let first = adc1
            .blocking_read(&mut first, SampleTime::Cycles102)
            .unwrap();
        let sibling = adc2
            .blocking_read(&mut sibling, SampleTime::Cycles262)
            .unwrap();
        core::hint::black_box((first, sibling));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
