#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    autotrim::{AutotrimTimer, Config, Prescaler},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    // 1500 prescaled cycles * 32768 raw HSIOSC cycles. This is a qualified
    // source-cycle period, not an exact one-second wall-clock deadline.
    let mut timer = AutotrimTimer::new(
        p.AUTOTRIM,
        Config {
            prescaler: Prescaler::Div32768,
            reload: 1499,
        },
    )
    .unwrap();
    timer.start().unwrap();
    loop {
        // A finite CPU-poll budget, deliberately unrelated to elapsed time.
        timer.blocking_wait(100_000_000).unwrap();
        cortex_m::asm::nop(); // Perform periodic application work here.
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
