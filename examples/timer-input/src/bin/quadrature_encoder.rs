//! Read external two-phase encoders through GTIM and (where bonded) ATIM.
#![no_std]
#![no_main]
#[cfg(all(feature = "time-driver", feature = "cw32l010y8m6"))]
compile_error!(
    "The selected time driver reserves GTIM1, and this package has no qualified ATIM encoder pair"
);
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
#[cfg(not(feature = "time-driver"))]
use embassy_cw32::timer::low_level::FilterValue;
use embassy_cw32::{
    self as hal,
    timer::{
        InputInstance,
        qei::{Config, Direction, Qei, QeiMode},
    },
};

#[unsafe(no_mangle)]
static GTIM_POSITION: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static ATIM_POSITION: AtomicU32 = AtomicU32::new(0);
fn poll<T: InputInstance>(qei: &mut Qei<'_, T>, position: &AtomicU32) {
    position.store(qei.count(), Ordering::Relaxed);
    let direction = match qei.read_direction() {
        Direction::Upcounting => 0,
        Direction::Downcounting => 1,
    };
    core::hint::black_box((direction, qei.period_ticks(), qei.kernel_clock_bounds()));
    if qei.is_overflow_pending() {
        qei.clear_overflow();
    }
}
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
    let mut gtim = {
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
    let mut atim = Qei::new(
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
    let mut atim = Qei::new(
        p.ATIM,
        p.PA8,
        p.PA9,
        Config {
            mode: QeiMode::Mode2,
            ..Config::default()
        },
    )
    .unwrap();
    loop {
        #[cfg(not(feature = "time-driver"))]
        poll(&mut gtim, &GTIM_POSITION);
        #[cfg(not(feature = "cw32l010y8m6"))]
        poll(&mut atim, &ATIM_POSITION);
    }
}
