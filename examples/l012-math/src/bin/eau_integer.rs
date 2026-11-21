//! Integer scaling and magnitude arithmetic, inspect results in a debugger.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{self as hal, eau::Eau, peripherals};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let _p = hal::init(hal::Config::default());
    let mut eau = Eau::<peripherals::EAU>::new().unwrap();
    // This is a read-count limit, not a deadline or operation-duration claim.
    let polls = 1024;
    loop {
        let rate = eau.divide_unsigned(8_000_000, 3000, polls);
        let signed_scale = eau.divide_signed(-12_345, 100, polls);
        let magnitude = eau.square_root(30_000u32.pow(2) + 40_000u32.pow(2), polls);
        let _ = core::hint::black_box((rate, signed_scale, magnitude));
        // A timeout leaves the engine running; do not busy-spin new submissions
        // if hardware is stuck. This application chooses to halt on busy.
        if eau.is_busy() {
            loop {
                cortex_m::asm::wfi();
            }
        }
    }
}
