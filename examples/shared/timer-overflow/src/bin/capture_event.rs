//! External capture notification, not an edge counter or lossless timestamp stream.
//! Consult OUT_DIR/capture-route.txt for the exact package input and IRQ.
#![no_std]
#![no_main]
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_cw32::{
    self as hal,
    gpio::Pull,
    time::Hertz,
    timer::{
        Channel,
        input_capture::{CaptureInput, InputCapture},
        low_level::CountingMode,
    },
};
include!(concat!(env!("OUT_DIR"), "/capture-bindings.rs"));
#[unsafe(no_mangle)]
static LATEST_CAPTURE: AtomicU32 = AtomicU32::new(0);
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut capture = include!(concat!(env!("OUT_DIR"), "/capture-constructor.rs"));
    capture.enable(Channel::Ch1);
    loop {
        capture.wait_for_capture(Channel::Ch1, CaptureIrqs).await;
        // Deliberately gate this input for a stable latest-value snapshot.
        // Edges during the gap are lost; earlier captures can have coalesced.
        capture.disable(Channel::Ch1);
        LATEST_CAPTURE.store(
            u32::from(capture.get_capture_value(Channel::Ch1)),
            Ordering::Relaxed,
        );
        #[cfg(example_classic_capture)]
        capture.clear_input_interrupt(Channel::Ch1);
        capture.enable(Channel::Ch1);
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
