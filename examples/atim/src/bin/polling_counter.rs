//! Internal PCLK ATIM counter. Poll flags and retain exclusive ATIM ownership.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    timer::low_level::{Prescaler, Timer},
};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    let mut timer = Timer::new(p.ATIM).unwrap();
    timer.set_period(Prescaler::Div64, 65536).unwrap();
    timer.start();
    loop {
        if timer.is_overflow_pending() {
            timer.clear_overflow();
        }
        core::hint::black_box((timer.get_counter(), timer.frequency_bounds()));
    }
}
