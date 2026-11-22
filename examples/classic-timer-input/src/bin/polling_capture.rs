//! Wire external voltage-compatible edges to the generated CAP1/CAP2 pin routes.
//! Nothing generates or loops back the signal. Debugger variables hold snapshots.
#![no_std]
#![no_main]
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    gpio::Pull,
    time::Hertz,
    timer::{
        Channel,
        input_capture::{CaptureInput, InputCapture},
        low_level::{CountingMode, FilterValue, InputCaptureMode},
    },
};
#[unsafe(no_mangle)]
static CAPTURE_TICKS: [AtomicU32; 2] = [const { AtomicU32::new(0) }; 2];
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    let mut capture = include!(concat!(env!("OUT_DIR"), "/capture-constructor.rs"));
    capture.set_input_capture_mode(Channel::Ch1, InputCaptureMode::BothEdges);
    capture.set_input_capture_mode(Channel::Ch2, InputCaptureMode::Falling);
    capture.set_input_capture_filter(Channel::Ch1, FilterValue::FckIntN4);
    capture.enable(Channel::Ch1);
    capture.enable(Channel::Ch2);
    loop {
        for (index, channel) in [Channel::Ch1, Channel::Ch2].into_iter().enumerate() {
            if capture.get_input_interrupt(channel) {
                // Stable latest-value snapshot with an intentional acquisition gap.
                // Multiple prior edges may already have overwritten the latch.
                capture.disable(channel);
                CAPTURE_TICKS[index].store(
                    u32::from(capture.get_capture_value(channel)),
                    Ordering::Relaxed,
                );
                capture.clear_input_interrupt(channel);
                capture.enable(channel);
            }
        }
        if capture.is_overflow_pending() {
            capture.clear_overflow();
        }
        core::hint::black_box((capture.get_counter(), capture.tick_frequency_bounds()));
    }
}
