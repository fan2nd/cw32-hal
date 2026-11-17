#![no_std]
#![no_main]

//! Target-side timing of the current and speed algorithms.

use core::sync::atomic::{compiler_fence, Ordering};
use cortex_m::{asm, peripheral::{syst::SystClkSource, Peripherals, SYST}};
use cortex_m_rt::entry;
use cw32_l012_foc_algorithms::{FocController, PhaseCurrent, SpeedController, JD4631};
use defmt::info;
use embassy_cw32 as hal;
use defmt_rtt as _;

const SAMPLES: u32 = 256;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! { loop { cortex_m::asm::wfi(); } }

const CORE_HZ: u32 = 96_000_000;
const SYSTICK_MASK: u32 = 0x00ff_ffff;

#[inline(always)]
fn elapsed_cycles(start: u32, end: u32) -> u32 {
    // SysTick counts down and wraps at 24 bits.
    start.wrapping_sub(end) & SYSTICK_MASK
}

#[inline(never)]
fn current(c: &mut FocController) -> u32 {
    let start = SYST::get_current();
    let duty = c.update(PhaseCurrent { a: 120, b: -80 }, 23_170, 23_170);
    compiler_fence(Ordering::SeqCst);
    core::hint::black_box(duty.a);
    elapsed_cycles(start, SYST::get_current())
}

#[inline(never)]
fn speed(c: &mut SpeedController) -> u32 {
    let start = SYST::get_current();
    let value = c.update_for_ticks(240, 180, 1);
    compiler_fence(Ordering::SeqCst);
    core::hint::black_box(value);
    elapsed_cycles(start, SYST::get_current())
}

#[entry]
fn main() -> ! {
    let _ = hal::init(hal::Config::default());
    let mut core = Peripherals::take().unwrap();
    core.SYST.set_clock_source(SystClkSource::Core);
    core.SYST.set_reload(SYSTICK_MASK);
    core.SYST.clear_current();
    core.SYST.enable_counter();
    let mut current_controller = FocController::new(JD4631);
    current_controller.iq_reference = 1_000;
    let mut speed_controller = SpeedController::new(240);
    let mut current_total = 0u32;
    let mut current_min = u32::MAX;
    let mut current_max = 0;
    let mut speed_total = 0u32;
    let mut speed_min = u32::MAX;
    let mut speed_max = 0;
    for _ in 0..SAMPLES {
        let ticks = current(&mut current_controller);
        current_total += ticks;
        current_min = current_min.min(ticks);
        current_max = current_max.max(ticks);
        let ticks = speed(&mut speed_controller);
        speed_total += ticks;
        speed_min = speed_min.min(ticks);
        speed_max = speed_max.max(ticks);
    }
    info!("algorithm timing: systick={=u32}Hz samples={=u32}", CORE_HZ, SAMPLES);
    info!("current loop: min={=u32} max={=u32} avg_cycles={=u32} avg_us={=u32}", current_min, current_max, current_total / SAMPLES, current_total / SAMPLES / (CORE_HZ / 1_000_000));
    info!("speed loop: min={=u32} max={=u32} avg_cycles={=u32} avg_us={=u32}", speed_min, speed_max, speed_total / SAMPLES, speed_total / SAMPLES / (CORE_HZ / 1_000_000));
    loop { asm::wfi(); }
}
