//! Two independently owned native comparators sharing an immutable divider.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    comparator::{self, Comparator, DigitalFilter, Edge},
    vref::{Source, Tap, Vref},
};
#[cfg(any(feature = "cw32l010f8p6", feature = "cw32l011k8t6"))]
hal::bind_interrupts!(struct Irqs {
    VC1 => comparator::InterruptHandler<hal::peripherals::VC1>;
    VC2 => comparator::InterruptHandler<hal::peripherals::VC2>;
});
#[cfg(feature = "cw32l012c8t6")]
hal::bind_interrupts!(struct Irqs {
    VC13 => comparator::InterruptHandler<hal::peripherals::VC1>;
    VC24 => comparator::InterruptHandler<hal::peripherals::VC2>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut reference_config = hal::vref::Config::default();
    reference_config.supply_mv = 3300; // Set both declarations for the actual board.
    reference_config.source = Source::Supply;
    reference_config.tap = Tap::FourEighths;
    #[cfg(any(feature = "cw32l010f8p6", feature = "cw32l011k8t6"))]
    let reference = Vref::new(p.VCDIV, reference_config).unwrap();
    #[cfg(feature = "cw32l012c8t6")]
    let reference = Vref::new(p.VC12REF, reference_config).unwrap();
    let mut config = comparator::Config::default();
    config.supply_mv = 3300;
    config.filter = Some(DigitalFilter::Div32Samples8);
    #[cfg(feature = "cw32l010f8p6")]
    let (mut first, mut second) = (
        Comparator::new_with_vref(p.VC1, p.PB5, &reference, config).unwrap(),
        Comparator::new_with_vref(p.VC2, p.PB6, &reference, config).unwrap(),
    );
    #[cfg(any(feature = "cw32l011k8t6", feature = "cw32l012c8t6"))]
    let (mut first, mut second) = (
        Comparator::new_with_vref(p.VC1, p.PA0, &reference, config).unwrap(),
        Comparator::new_with_vref(p.VC2, p.PA1, &reference, config).unwrap(),
    );
    loop {
        // Fresh event windows, not simultaneous observations or settled inputs.
        embassy_futures::join::join(
            first.wait_for_edge(Edge::Either, Irqs),
            second.wait_for_edge(Edge::Either, Irqs),
        ).await;
        core::hint::black_box((first.output(), second.output()));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}
