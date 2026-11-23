#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Config, SampleTime, triggered::Btim1Adc},
    timer::{low_level::Prescaler, trigger::Btim1Update},
};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    // PA0 is a qualified external ADC input on all five exact L010/L011 packages.
    // Connect only a board-safe voltage within 0..=VDD, with a common ground.
    let source = Btim1Update::new(p.BTIM1, Prescaler::Div64, 1000).unwrap();
    let mut adc = Btim1Adc::new(
        p.ADC,
        source,
        p.PA0,
        Config::default(),
        SampleTime::Cycles390,
    )
    .unwrap();
    loop {
        adc.arm().unwrap();
        // Poll independently of the timer delay. Exactly one trigger per arm.
        // A production application may choose a finite wait budget; timeout
        // permanently disarms that owner, without an immediate-abort guarantee.
        loop {
            if let Some(sample) = adc.poll().unwrap() {
                core::hint::black_box((sample, adc.timing()));
                break;
            }
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
