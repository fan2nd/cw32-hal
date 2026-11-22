//! F002F3P7: supply a settled 2.5 V reference to PB4 and a low-impedance
//! 0..2.5 V input to PA1. Board VDD must remain at least the declared 3.3 V.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{self as hal, adc::{Adc, Config, Reference, SampleTime}};

#[entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let mut input = p.PA1;
    let mut config = Config::default();
    config.vdda_mv = 3300;
    let mut adc = Adc::new_with_external_reference(p.ADC, p.PB4, config).unwrap();
    let sample = adc.blocking_read(&mut input, SampleTime::Cycles10);
    let _ = core::hint::black_box(sample);
    // The ExRef pin stays owned even while another source is selected.
    config.reference = Reference::Vdda;
    adc.set_config(&config).unwrap();
    config.reference = Reference::External;
    adc.set_config(&config).unwrap();
    let _ = core::hint::black_box(adc.blocking_read(&mut input, SampleTime::Cycles10));
    drop(adc);
    loop { core::hint::spin_loop(); }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}
