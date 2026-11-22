#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::trng::{Config, Trng};
#[entry]
fn main() -> ! {
    let p = embassy_cw32::init(Default::default()).unwrap();
    let mut source = Trng::new(p.TRNG, Config::default()).unwrap();
    loop {
        match source.generate(100_000) {
            Ok(sample) => {
                core::hint::black_box(sample);
            }
            Err(error) => {
                // No output is available on failure. Never substitute zeros,
                // stale data, or a software PRNG for a failed hardware sample.
                core::hint::black_box(error);
                loop {
                    cortex_m::asm::bkpt();
                }
            }
        }
        // Application demonstration only: no entropy/health qualification.
        cortex_m::asm::delay(800_000);
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
