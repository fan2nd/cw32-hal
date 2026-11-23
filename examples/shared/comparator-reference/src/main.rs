//! Poll two comparators sharing an immutable internal divider.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    comparator::Comparator,
    vref::{Source, Tap, Vref},
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
    // Both declarations describe this example's 3.3 V board; neither measures it.
    let mut reference_config = hal::vref::Config::default();
    reference_config.supply_mv = 3300;
    reference_config.source = Source::Supply;
    reference_config.tap = Tap::FourEighths;
    #[cfg(any(feature = "cw32l010f8p6", feature = "cw32l011k8t6"))]
    let reference = Vref::new(p.VCDIV, reference_config).unwrap();
    #[cfg(feature = "cw32l012c8t6")]
    let reference = Vref::new(p.VC12REF, reference_config).unwrap();
    let mut config = hal::comparator::Config::default();
    config.supply_mv = 3300;
    #[cfg(feature = "cw32l010f8p6")]
    let (first, second) = (
        Comparator::new_with_vref(p.VC1, p.PB5, &reference, config).unwrap(),
        Comparator::new_with_vref(p.VC2, p.PB6, &reference, config).unwrap(),
    );
    #[cfg(any(feature = "cw32l011k8t6", feature = "cw32l012c8t6"))]
    let (first, second) = (
        Comparator::new_with_vref(p.VC1, p.PA0, &reference, config).unwrap(),
        Comparator::new_with_vref(p.VC2, p.PA1, &reference, config).unwrap(),
    );
    #[cfg(feature = "cw32l012c8t6")]
    let other_reference = {
        reference_config.source = Source::Core;
        reference_config.tap = Tap::TwoEighths;
        Vref::new(p.VC34REF, reference_config).unwrap()
    };
    #[cfg(feature = "cw32l012c8t6")]
    let (third, fourth) = (
        Comparator::new_with_vref(p.VC3, p.PA6, &other_reference, config).unwrap(),
        Comparator::new_with_vref(p.VC4, p.PA9, &other_reference, config).unwrap(),
    );
    loop {
        // These families have no readiness flag or guaranteed startup maximum.
        // Observations retain Readiness::Unknown. Qualify settling on the board
        // before using their raw levels for decisions or control.
        core::hint::black_box((first.output(), second.output()));
        #[cfg(feature = "cw32l012c8t6")]
        core::hint::black_box((third.output(), fourth.output()));
    }
}
