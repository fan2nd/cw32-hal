//! Apply external quadrature phases to the generated native CH1A/CH1B pads.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    timer::qei::{ClassicAtimQei, ClassicAtimQeiConfig, Direction},
};

#[unsafe(no_mangle)]
static POSITION: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static DIRECTION: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static WRAP_FLAGS: AtomicU32 = AtomicU32::new(0);

#[entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let mut encoder = include!(concat!(env!("OUT_DIR"), "/qei-constructor.rs"));
    loop {
        POSITION.store(encoder.count(), Ordering::Relaxed);
        DIRECTION.store(
            u32::from(encoder.read_direction() == Direction::Downcounting),
            Ordering::Relaxed,
        );
        // Native OVF and UNDF are independent sticky indications, not UIF and
        // not wrap counts. Count, direction and these flags are non-atomic.
        WRAP_FLAGS.store(
            u32::from(encoder.is_overflow_pending())
                | (u32::from(encoder.is_underflow_pending()) << 1),
            Ordering::Relaxed,
        );
        encoder.clear_overflow();
        encoder.clear_underflow();
        core::hint::black_box((encoder.period_ticks(), encoder.kernel_clock_bounds()));
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
