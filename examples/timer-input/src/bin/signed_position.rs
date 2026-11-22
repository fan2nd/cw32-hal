//! Sample signed encoder positions under an application-qualified movement bound.
#![no_std]
#![no_main]
#[cfg(all(feature = "time-driver", feature = "cw32l010y8m6"))]
compile_error!(
    "The selected time driver reserves GTIM1, and this package has no qualified ATIM encoder pair"
);
use cortex_m_rt::entry;
#[cfg(not(feature = "time-driver"))]
use embassy_cw32::timer::low_level::FilterValue;
use embassy_cw32::{
    self as hal,
    timer::{
        qei::{Config, Qei, QeiMode},
    },
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
    #[cfg(not(feature = "time-driver"))]
    let gtim = {
        #[cfg(l010_input_example)]
        let g2 = p.PA5;
        #[cfg(not(l010_input_example))]
        let g2 = p.PA7;
        let mut gtim = Qei::new(
            p.GTIM1,
            p.PA6,
            g2,
            Config {
                mode: QeiMode::Mode3,
                filter: FilterValue::FckIntN4,
                ..Config::default()
            },
        )
        .unwrap();
        gtim.reset();
        gtim
    };

    #[cfg(all(l010_input_example, not(feature = "cw32l010y8m6")))]
    let atim = Qei::new(
        p.ATIM,
        p.PB4,
        p.PB2,
        Config {
            mode: QeiMode::Mode1,
            ..Config::default()
        },
    )
    .unwrap();
    #[cfg(not(l010_input_example))]
    let atim = Qei::new(
        p.ATIM,
        p.PA8,
        p.PA9,
        Config {
            mode: QeiMode::Mode2,
            ..Config::default()
        },
    )
    .unwrap();
    // Board/application must ensure <= 1024 net counts between samples, even
    // across interrupts. A complete wrap can alias silently; no rate is assumed.
    #[cfg(not(feature = "time-driver"))]
    let mut gtim = gtim.track_position(0, 1024).unwrap_or_else(|_| panic!());
    #[cfg(not(feature = "cw32l010y8m6"))]
    let mut atim = atim.track_position(0, 1024).unwrap_or_else(|_| panic!());
    loop {
        #[cfg(not(feature = "time-driver"))]
        core::hint::black_box(gtim.sample().unwrap());
        #[cfg(not(feature = "cw32l010y8m6"))]
        core::hint::black_box(atim.sample().unwrap());
    }
}
