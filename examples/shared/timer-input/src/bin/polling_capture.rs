//! External input capture with both GTIM and ATIM; inspect timestamps in a debugger.
//! No signals are generated or looped back. See README for actual input pads.
#![no_std]
#![no_main]
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    gpio::Pull,
    time::Hertz,
    timer::{
        Channel, InputInstance,
        input_capture::{CaptureInput, InputCapture},
        low_level::CountingMode,
    },
};

#[cfg(not(feature = "time-driver"))]
use embassy_cw32::timer::low_level::{FilterValue, InputCaptureMode};

#[unsafe(no_mangle)]
static GTIM_TIMESTAMPS: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];
#[unsafe(no_mangle)]
static ATIM_TIMESTAMPS: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];
#[unsafe(no_mangle)]
static OVERRUN_OBSERVATIONS: AtomicU32 = AtomicU32::new(0);

fn poll<T: InputInstance>(capture: &mut InputCapture<'_, T>, values: &[AtomicU32; 4]) {
    for (n, channel) in [Channel::Ch1, Channel::Ch2, Channel::Ch3, Channel::Ch4]
        .into_iter()
        .enumerate()
    {
        if capture.is_enabled(channel) && capture.get_input_interrupt(channel) {
            values[n].store(
                u32::from(capture.get_capture_value(channel)),
                Ordering::Relaxed,
            );
            if capture.is_overcapture_pending(channel) {
                // This counts observations only, never lost events. Coalescing
                // and races prohibit reconstructing an event count.
                let old = OVERRUN_OBSERVATIONS.load(Ordering::Relaxed);
                OVERRUN_OBSERVATIONS.store(old.wrapping_add(1), Ordering::Relaxed);
                capture.clear_overcapture(channel);
            }
        }
    }
    if capture.is_overflow_pending() {
        capture.clear_overflow();
    }
    core::hint::black_box((
        capture.get_counter(),
        capture.tick_frequency(),
        capture.tick_frequency_bounds(),
    ));
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
        let g2 = CaptureInput::from_pin(p.PA5, Pull::None);
        #[cfg(not(l010_input_example))]
        let g2 = CaptureInput::from_pin(p.PA7, Pull::None);
        let mut gtim = InputCapture::new(
            p.GTIM1,
            Some(CaptureInput::from_pin(p.PA6, Pull::None)),
            Some(g2),
            None,
            None,
            Hertz(1_000_000),
            CountingMode::EdgeAlignedUp,
        )
        .unwrap();
        gtim.set_input_capture_mode(Channel::Ch1, InputCaptureMode::BothEdges);
        gtim.set_input_capture_mode(Channel::Ch2, InputCaptureMode::Falling);
        gtim.set_input_capture_filter(Channel::Ch1, FilterValue::FckIntN4);
        gtim.set_input_capture_prescaler(Channel::Ch2, 1);
        gtim.enable(Channel::Ch1);
        gtim.enable(Channel::Ch2);
        gtim
    };

    #[cfg(all(l010_input_example, not(feature = "cw32l010y8m6")))]
    let (a1, a2) = (
        Some(CaptureInput::from_pin(p.PB4, Pull::None)),
        Some(CaptureInput::from_pin(p.PB2, Pull::None)),
    );
    #[cfg(feature = "cw32l010y8m6")]
    let (a1, a2) = (None, None);
    #[cfg(not(l010_input_example))]
    let (a1, a2) = (
        Some(CaptureInput::from_pin(p.PA8, Pull::None)),
        Some(CaptureInput::from_pin(p.PA9, Pull::None)),
    );
    #[cfg(l010_input_example)]
    let (a3, a4) = (Some(CaptureInput::from_pin(p.PA3, Pull::None)), None);
    #[cfg(not(l010_input_example))]
    let (a3, a4) = (
        Some(CaptureInput::from_pin(p.PA10, Pull::None)),
        Some(CaptureInput::from_pin(p.PA11, Pull::None)),
    );
    let has_a1 = a1.is_some();
    let has_a4 = a4.is_some();
    let mut atim = InputCapture::new(
        p.ATIM,
        a1,
        a2,
        a3,
        a4,
        Hertz(1_000_000),
        CountingMode::EdgeAlignedUp,
    )
    .unwrap();
    if has_a1 {
        atim.enable(Channel::Ch1);
        atim.enable(Channel::Ch2);
    }
    atim.enable(Channel::Ch3);
    if has_a4 {
        atim.enable(Channel::Ch4);
    }
    loop {
        #[cfg(not(feature = "time-driver"))]
        poll(&mut gtim, &GTIM_TIMESTAMPS);
        poll(&mut atim, &ATIM_TIMESTAMPS);
    }
}
