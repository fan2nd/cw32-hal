//! Apply independent voltage-compatible signals to all six generated input pads.
//! This program supplies no signal and does not loop an output back into a pad.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    gpio::Pull,
    time::Hertz,
    timer::{
        ClassicAtimChannel,
        input_capture::{ClassicAtimCaptureInput, ClassicAtimInputCapture},
        low_level::{ClassicAtimFilter, InputCaptureMode},
    },
};

// Each debugger slot is a latest latched value, with its separately observed
// sticky overwrite flag. These arrays do not constitute an atomic live stream.
#[unsafe(no_mangle)]
static CAPTURE_TICKS: [AtomicU32; 6] = [const { AtomicU32::new(0) }; 6];
#[unsafe(no_mangle)]
static CAPTURE_OVERWRITE: [AtomicU32; 6] = [const { AtomicU32::new(0) }; 6];
const CHANNELS: [ClassicAtimChannel; 6] = [
    ClassicAtimChannel::Ch1A,
    ClassicAtimChannel::Ch1B,
    ClassicAtimChannel::Ch2A,
    ClassicAtimChannel::Ch2B,
    ClassicAtimChannel::Ch3A,
    ClassicAtimChannel::Ch3B,
];

#[entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let mut capture = include!(concat!(env!("OUT_DIR"), "/capture-constructor.rs"));
    for channel in CHANNELS {
        capture.set_input_capture_mode(channel, InputCaptureMode::BothEdges);
        capture.set_input_capture_filter(channel, ClassicAtimFilter::PclkN3);
        capture.enable(channel);
    }
    loop {
        for (index, channel) in CHANNELS.into_iter().enumerate() {
            if capture.get_input_interrupt(channel) {
                // Disabling this half creates a stable latch/status observation
                // and deliberately loses edges during the acquisition gap.
                capture.disable(channel);
                CAPTURE_TICKS[index].store(
                    u32::from(capture.get_capture_value(channel)),
                    Ordering::Relaxed,
                );
                CAPTURE_OVERWRITE[index].store(
                    u32::from(capture.is_overcapture_pending(channel)),
                    Ordering::Relaxed,
                );
                // Timestamp reads do not acknowledge either flag. Clear F and E
                // independently; neither a false E nor this polling loop proves
                // that every external edge was observed.
                capture.clear_input_interrupt(channel);
                capture.clear_overcapture(channel);
                capture.enable(channel);
            }
        }
        if capture.is_overflow_pending() {
            capture.clear_overflow();
        }
        core::hint::black_box((capture.get_counter(), capture.tick_frequency_bounds()));
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
