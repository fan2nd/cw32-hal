#![no_std]
#![no_main]
use embassy_cw32::{self as hal, comparator::{Comparator, Config, DigitalFilter}};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let mut config = Config::default();
    config.supply_mv = 3300; // Set to the actual board analog supply.
    config.filter = Some(DigitalFilter::Div32Samples8);
    let vc = Comparator::new(p.VC1, p.PA0, p.PA2, config).unwrap();
    loop {
        // Unknown readiness remains unknown; filtering does not prove settling.
        core::hint::black_box(vc.output());
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { cortex_m::asm::bkpt(); }
}
