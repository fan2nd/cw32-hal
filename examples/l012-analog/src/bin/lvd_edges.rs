#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal, bind_interrupts,
    lvd::{self, Edge, LowVoltageMonitor, Threshold},
};
bind_interrupts!(struct Irqs { LVD => lvd::InterruptHandler; });
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let threshold = Threshold::from_millivolts(2600).unwrap();
    let mut monitor = LowVoltageMonitor::new_supply(p.LVD, threshold).unwrap();
    // Board must qualify Vcore=1.6 V, analog settling and legal supply limits.
    // This is event observation only, not undervoltage protection or wakeup.
    loop {
        monitor.wait_for_edge(Edge::Either, Irqs).await;
        core::hint::black_box(monitor.sample()); // Separate, later observation.
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
