#![no_std]
#![no_main]
//! Passive debugger-visible RAM parity observation. Build/link only in CI.
use core::cell::Cell;
use embassy_cw32::{
    self as hal,
    ram::{Ram, Status},
};

// Inspect this symbol in a debugger. No fault injection or SRAM scan is done.
#[unsafe(no_mangle)]
pub static RAM_PARITY_DIAGNOSTIC: critical_section::Mutex<Cell<Option<Status>>> =
    critical_section::Mutex::new(Cell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let ram = Ram::new(p.RAM);
    loop {
        let observation = ram.status();
        critical_section::with(|cs| RAM_PARITY_DIAGNOSTIC.borrow(cs).set(Some(observation)));
        // Flags, interrupt masks and the shared NVIC vector are preserved.
        cortex_m::asm::nop();
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
