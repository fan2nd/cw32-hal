//! Poll external inputs while retaining comparator and pin ownership.
//! See README.md for the exact board pins and startup qualification.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    comparator::{Comparator, Config, Readiness},
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
    let mut config = Config::default();
    // Example board declaration, not a measurement. Adjust to the actual board.
    config.supply_mv = 3300;
    #[cfg(any(feature = "cw32f002f3p7", feature = "cw32f003e4p7"))]
    let comparator = Comparator::new(p.VC1, p.PC2, p.PB0, config).unwrap();
    #[cfg(feature = "cw32r031c8u6")]
    let comparator = Comparator::new(p.VC1, p.PA4, p.PA5, config).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let comparator = Comparator::new(p.VC1, p.PB5, p.PA0, config).unwrap();
    #[cfg(any(feature = "cw32l011k8t6", feature = "cw32l012c8t6"))]
    let comparator = Comparator::new(p.VC1, p.PA0, p.PA2, config).unwrap();
    #[cfg(any(
        feature = "cw32f030c8t7",
        feature = "cw32a030c8t7",
        feature = "cw32f020c6u7",
        feature = "cw32l031c8t6",
        feature = "cw32l052c8t6",
        feature = "cw32l083mct6"
    ))]
    let comparator = Comparator::new(p.VC1, p.PA0, p.PA1, config).unwrap();
    loop {
        let output = comparator.output();
        if output.readiness == Readiness::Ready {
            // Startup READY does not remove propagation/offset requirements.
            core::hint::black_box(output.level);
        } else {
            // L010/L011/L012 report Unknown forever: no guaranteed startup
            // maximum exists in the sources. Qualify settling at board level
            // before using their raw level in control logic.
            core::hint::black_box(output);
        }
    }
}
