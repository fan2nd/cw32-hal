#![no_std]
#![no_main]
use embassy_cw32 as hal;
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(example_awt)]
    let mut timer = hal::awt::Awt::new(p.AWT, Default::default()).unwrap();
    #[cfg(not(example_awt))]
    let mut timer = hal::lptim::Lptim::new(p.LPTIM, Default::default()).unwrap();
    #[cfg(example_awt)]
    timer.start();
    #[cfg(not(example_awt))]
    timer.start().unwrap();
    loop {
        while !timer.take_elapsed() {
            core::hint::spin_loop();
        }
        cortex_m::asm::nop(); // Put run-mode periodic work here.
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
