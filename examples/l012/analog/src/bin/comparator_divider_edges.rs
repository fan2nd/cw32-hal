#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal, bind_interrupts,
    comparator::{self, Comparator, Config, Edge},
    peripherals,
};
bind_interrupts!(struct Irqs {
    VC13 => comparator::InterruptHandler<peripherals::VC1>, comparator::InterruptHandler<peripherals::VC3>;
    VC24 => comparator::InterruptHandler<peripherals::VC2>, comparator::InterruptHandler<peripherals::VC4>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut config = Config::default();
    config.supply_mv = 3300; // Board declaration; adjust to actual analog supply.
    let ref12 = hal::vref::Vref::new(p.VC12REF, Default::default()).unwrap();
    let ref34 = hal::vref::Vref::new(p.VC34REF, Default::default()).unwrap();
    let mut vc1 = Comparator::new_with_vref(p.VC1, p.PA0, &ref12, config).unwrap();
    let mut vc2 = Comparator::new_with_vref(p.VC2, p.PA1, &ref12, config).unwrap();
    let mut vc3 = Comparator::new_with_vref(p.VC3, p.PA6, &ref34, config).unwrap();
    let mut vc4 = Comparator::new_with_vref(p.VC4, p.PA9, &ref34, config).unwrap();
    // This example does not qualify startup or input settling. Early events
    // may be startup transients; never use them as safety/ready indications.
    loop {
        embassy_futures::join::join4(
            vc1.wait_for_edge(Edge::Either, Irqs),
            vc2.wait_for_edge(Edge::Either, Irqs),
            vc3.wait_for_edge(Edge::Either, Irqs),
            vc4.wait_for_edge(Edge::Either, Irqs),
        )
        .await;
        // All four fresh windows observed an event, not a simultaneous sample.
        core::hint::black_box((vc1.output(), vc2.output(), vc3.output(), vc4.output()));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
