#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    timer::{
        low_level::Prescaler,
        trigger::{Btim1Update, Btim2Cascade},
    },
};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let source = Btim1Update::new(p.BTIM1, Prescaler::Div64, 1000).unwrap();
    let mut cascade = Btim2Cascade::new(source, p.BTIM2, 100).unwrap();
    let interval = (cascade.clock_divisor(), cascade.kernel_clock_bounds());
    cascade.start();
    loop {
        // Inspect these values with a debugger. Overflow flags can coalesce.
        core::hint::black_box((cascade.get_counter(), interval));
        if cascade.is_overflow_pending() {
            cascade.clear_overflow();
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
