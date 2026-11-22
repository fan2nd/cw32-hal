//! All eight slots, borrowed external channels, repeated internal sources.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, AdcChannel, Config, SampleTime},
    time::Hertz,
};
#[cortex_m_rt::entry]
fn main() -> ! {
    let mut system = hal::Config::default();
    // Change these board declarations before use. L011 VDDA must equal VDD.
    system.rcc.operating_conditions.supply_mv = 3_300;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div8;
    let p = hal::init(system).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(24_000_000);
    let mut adc = Adc::new(p.ADC, config).unwrap();
    let mut first = p.PA0;
    let mut second = p.PA1;
    let temperature = adc.enable_temperature().degrade_adc();
    let bandgap = adc.enable_vrefint().degrade_adc();
    loop {
        {
            let a = first.reborrow_adc();
            let b = second.reborrow_adc();
            let sequence = [
                (&a, SampleTime::Cycles54),
                (&b, SampleTime::Cycles102),
                (&temperature, SampleTime::Cycles390),
                (&bandgap, SampleTime::Cycles390),
                (&a, SampleTime::Cycles390),
                (&b, SampleTime::Cycles54),
                (&temperature, SampleTime::Cycles390),
                (&bandgap, SampleTime::Cycles390),
            ];
            let mut samples = [0; 8];
            let timing = adc.blocking_read_sequence(&sequence, &mut samples).unwrap();
            core::hint::black_box((samples, timing));
        }
        // Sequence borrows ended. A single read programs exactly one slot again.
        let sample = adc
            .blocking_read(&mut first, SampleTime::Cycles102)
            .unwrap();
        core::hint::black_box((sample, adc.timing()));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
