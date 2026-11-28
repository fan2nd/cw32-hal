//! Shared state between the SysTick outer loop and the ADC current-loop ISR.
use crate::foc::{CurrentSampleFilter, FocController, JD4631, MotorParameters};
use core::cell::RefCell;
use core::sync::atomic::{AtomicI16, AtomicI32, AtomicU16, AtomicU32};
use cortex_m::interrupt::Mutex;

pub static CURRENT_CONTROLLER: Mutex<RefCell<FocController>> =
    Mutex::new(RefCell::new(FocController::new(MotorParameters {
        current_kp_gain: 2_500,
        current_ki_gain: 64,
        ..JD4631
    })));
pub static CURRENT_SAMPLE_FILTER: Mutex<RefCell<CurrentSampleFilter>> =
    Mutex::new(RefCell::new(CurrentSampleFilter::new()));
pub static CURRENT_OFFSET_A: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_OFFSET_B: AtomicU16 = AtomicU16::new(0);
// Raw samples are retained for hardware bring-up diagnostics. They let the
// log distinguish a bad ADC trigger/analog path from a control-law problem.
pub static CURRENT_ADC_A: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_ADC_B: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_ELECTRICAL_ANGLE: AtomicU32 = AtomicU32::new(0);
pub static CURRENT_APPLIED_ANGLE: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_ANGLE_STEP: AtomicI32 = AtomicI32::new(0);
pub static CURRENT_IQ_REFERENCE: AtomicI16 = AtomicI16::new(0);
pub static CURRENT_ID: AtomicI16 = AtomicI16::new(0);
pub static CURRENT_IQ: AtomicI16 = AtomicI16::new(0);
pub static CURRENT_RESET: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_LOOP_COUNT: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_LOOP_TOTAL: AtomicU32 = AtomicU32::new(0);
pub static CURRENT_PROBE_ACTIVE: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_FAULT: AtomicU16 = AtomicU16::new(0);
pub static SYSTICK_OVERRUNS: AtomicU32 = AtomicU32::new(0);
pub static FIRST_STOP: AtomicU16 = AtomicU16::new(0);
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum StopReason {
    Overrun = 1,
    Encoder,
    BusVoltage,
    Overspeed,
    PhaseCurrent,
    Heartbeat,
    Release,
    Connect,
}

pub fn record_stop(reason: StopReason) {
    cortex_m::interrupt::free(|_| {
        use core::sync::atomic::Ordering::Relaxed;
        if FIRST_STOP.load(Relaxed) == 0 {
            FIRST_STOP.store(reason as u16, Relaxed);
        }
    });
}
pub const OUTPUT_PRIME: u16 = 1;
pub const OUTPUT_ENABLE_NEXT: u16 = 2;
pub const OUTPUT_RUNNING: u16 = 3;
pub static CURRENT_OUTPUT_ENABLE: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_ALIGNMENT_ACTIVE: AtomicU16 = AtomicU16::new(0);
