//! Apply two board-qualified low-impedance voltages to the generated pin routes.
//! The board must guarantee 3.3 V VDDA/VDD and input voltages within 0..VDDA.
//! Debugger-visible arrays hold raw sequential ADC counts in alternating order.
#![no_std]
#![no_main]
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    adc::{Adc, AdcChannel, Config, MAX_SEQUENCE_LEN, SampleTime},
};
#[unsafe(no_mangle)]
static ORDERED_SAMPLES: [AtomicU32; MAX_SEQUENCE_LEN] =
    [const { AtomicU32::new(0) }; MAX_SEQUENCE_LEN];
#[unsafe(no_mangle)]
static SUPPLY_SAMPLE: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static COMPLETED_SCANS: AtomicU32 = AtomicU32::new(0);
#[unsafe(no_mangle)]
static FAILED_READS: AtomicU32 = AtomicU32::new(0);
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
    config.vdda_mv = 3300;
    let mut adc = Adc::new_blocking(p.ADC, config).unwrap();
    let (first_pin, second_pin) = include!(concat!(env!("OUT_DIR"), "/channel-pins.rs"));
    let first = first_pin.degrade_adc();
    let second = second_pin.degrade_adc();
    let mut supply = adc.enable_vdda();
    loop {
        let sequence: [_; MAX_SEQUENCE_LEN] = core::array::from_fn(|slot| {
            (
                if slot % 2 == 0 { &first } else { &second },
                SampleTime::Cycles10,
            )
        });
        let mut samples = [0; MAX_SEQUENCE_LEN];
        match adc.blocking_read_sequence(&sequence, &mut samples) {
            Ok(timing) => {
                for (destination, sample) in ORDERED_SAMPLES.iter().zip(samples) {
                    destination.store(u32::from(sample), Ordering::Relaxed);
                }
                core::hint::black_box(timing.maximum_conversion_time_ns());
                COMPLETED_SCANS.store(
                    COMPLETED_SCANS.load(Ordering::Relaxed).wrapping_add(1),
                    Ordering::Relaxed,
                );
            }
            Err(_) => {
                FAILED_READS.store(
                    FAILED_READS.load(Ordering::Relaxed).wrapping_add(1),
                    Ordering::Relaxed,
                );
            }
        }
        // Existing single-read API switches back to MODE=0 and safely enables
        // the internal source's follower. The next scan restores direct input.
        match adc.blocking_read(&mut supply, SampleTime::Cycles10) {
            Ok(sample) => SUPPLY_SAMPLE.store(u32::from(sample), Ordering::Relaxed),
            Err(_) => {
                FAILED_READS.store(
                    FAILED_READS.load(Ordering::Relaxed).wrapping_add(1),
                    Ordering::Relaxed,
                );
            }
        }
    }
}
