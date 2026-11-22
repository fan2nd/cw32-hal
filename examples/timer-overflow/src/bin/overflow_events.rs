//! Coalesced run-mode housekeeping on independent timer banks.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use embassy_cw32::{
    self as hal,
    mode::Async,
    timer::low_level::{Prescaler, Timer},
};

// Each bit is an observed notification in the latest loop, never a wrap count.
// Bit 0: GTIM/GTIM1 (GTIM3 on L012), bit 1: ATIM, bit 2: L012 GTIM4.
#[unsafe(no_mangle)]
static HOUSEKEEPING_EVENTS: AtomicU32 = AtomicU32::new(0);

// Generated from exact GLOBAL IRQ metadata. L012 GTIM34 dispatches both banks.
include!(concat!(env!("OUT_DIR"), "/event-bindings.rs"));

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut primary = Timer::<_, Async>::new_async(
        include!(concat!(env!("OUT_DIR"), "/primary-token.rs")),
        Irqs,
    )
    .unwrap();
    primary.set_period(Prescaler::Div256, 31_250).unwrap();
    primary.start();
    #[cfg(example_atim)]
    let mut advanced = {
        let mut timer = Timer::<_, Async>::new_async(p.ATIM, Irqs).unwrap();
        timer.set_period(Prescaler::Div256, 62_500).unwrap();
        timer.start();
        timer
    };
    #[cfg(example_l012)]
    let mut secondary = {
        let mut timer = Timer::<_, Async>::new_async(
            include!(concat!(env!("OUT_DIR"), "/secondary-token.rs")),
            Irqs,
        )
        .unwrap();
        timer.set_period(Prescaler::Div256, 46_875).unwrap();
        timer.start();
        timer
    };

    loop {
        #[cfg(example_l012)]
        let mut observed = match embassy_futures::select::select3(
            primary.wait_overflow(),
            advanced.wait_overflow(),
            secondary.wait_overflow(),
        )
        .await
        {
            embassy_futures::select::Either3::First(()) => 1,
            embassy_futures::select::Either3::Second(()) => 2,
            embassy_futures::select::Either3::Third(()) => 4,
        };
        #[cfg(all(example_atim, not(example_l012)))]
        let mut observed = match embassy_futures::select::select(
            primary.wait_overflow(),
            advanced.wait_overflow(),
        )
        .await
        {
            embassy_futures::select::Either::First(()) => 1,
            embassy_futures::select::Either::Second(()) => 2,
        };
        #[cfg(not(example_atim))]
        let mut observed = {
            primary.wait_overflow().await;
            1
        };
        // Unselected waits were cancelled; each owner keeps counting and retains
        // its event. Consume any notifications now pending for this work cycle.
        if primary.take_overflow() {
            observed |= 1;
        }
        #[cfg(example_atim)]
        if advanced.take_overflow() {
            observed |= 2;
        }
        #[cfg(example_l012)]
        if secondary.take_overflow() {
            observed |= 4;
        }
        HOUSEKEEPING_EVENTS.store(observed, Ordering::Relaxed);
        core::hint::black_box(observed); // Application housekeeping belongs here.
        #[cfg(example_l012)]
        {
            // An actual global-time consumer while all three event owners live.
            // The chip feature reserves GTIM1; it is not one of these owners.
            embassy_time::Timer::after_millis(1).await;
            core::hint::black_box(embassy_time::Instant::now());
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
