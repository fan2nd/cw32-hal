#![no_std]
#![no_main]

//! Target-side timing comparison for FOC arithmetic and math accelerators.

mod hfi;

use core::sync::atomic::{Ordering, compiler_fence};
use cortex_m::{
    asm,
    peripheral::{Peripherals, SYST, syst::SystClkSource},
};
use cortex_m_rt::entry;
use cw32_l012_foc_algorithms::{
    FocController, JD4631, PhaseCurrent, SpeedController, sin_cos_q15_lut,
};
use defmt::{error, info};
use defmt_rtt as _;
use embassy_cw32 as hal;
use hal::{
    eau::{Eau, Error as EauError},
    peripherals,
    rcc::{self, HsiDiv},
};

const SAMPLES: u32 = 256;
const SYSTICK_MASK: u32 = 0x00ff_ffff;
const POLLS: u32 = 1_024;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

#[inline(always)]
fn elapsed_cycles(start: u32, end: u32) -> u32 {
    start.wrapping_sub(end) & SYSTICK_MASK
}

#[inline(never)]
fn time_lut(angle: u16) -> u32 {
    let start = SYST::get_current();
    let value = sin_cos_q15_lut(angle);
    compiler_fence(Ordering::SeqCst);
    core::hint::black_box(value);
    elapsed_cycles(start, SYST::get_current())
}

#[inline(never)]
fn time_software_division(angle: u16) -> u32 {
    let start = SYST::get_current();
    // Keep operand preparation and division after the opening SysTick read.
    let angle = core::hint::black_box(angle);
    let dividend = 8_000_000u32.saturating_add(u32::from(angle));
    let divisor = 3_000u32.saturating_add(u32::from(angle & 31));
    let value = (dividend / divisor, dividend % divisor);
    compiler_fence(Ordering::SeqCst);
    core::hint::black_box(value);
    elapsed_cycles(start, SYST::get_current())
}

#[inline(never)]
fn time_eau(eau: &mut Eau<peripherals::EAU>, angle: u16) -> Result<u32, EauError> {
    let start = SYST::get_current();
    // Keep operand preparation and division after the opening SysTick read.
    let angle = core::hint::black_box(angle);
    let dividend = 8_000_000u32.saturating_add(u32::from(angle));
    let divisor = 3_000u32.saturating_add(u32::from(angle & 31));
    let value = eau.divide_unsigned(dividend, divisor, POLLS)?;
    compiler_fence(Ordering::SeqCst);
    core::hint::black_box(value);
    Ok(elapsed_cycles(start, SYST::get_current()))
}

#[derive(Clone, Copy)]
struct Stats {
    min: u32,
    max: u32,
    total: u32,
}
impl Stats {
    const fn new() -> Self {
        Self {
            min: u32::MAX,
            max: 0,
            total: 0,
        }
    }
    fn add(&mut self, value: u32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.total = self.total.saturating_add(value);
    }
}

#[entry]
fn main() -> ! {
    // Match the speed-loop clock configuration on the 5 V motor-control board.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 5_000;
    system.rcc.hsi.div = HsiDiv::Div1;
    system.rcc.allow_nominal_bus_overclock = true;
    if let Err(err) = hal::init(system) {
        error!("RCC init failed: {:?}", err);
        loop {
            asm::wfi();
        }
    }
    let mut core = Peripherals::take().unwrap();
    core.SYST.set_clock_source(SystClkSource::Core);
    core.SYST.set_reload(SYSTICK_MASK);
    core.SYST.clear_current();
    core.SYST.enable_counter();
    let mut eau = match Eau::<peripherals::EAU>::new() {
        Ok(engine) => Some(engine),
        Err(err) => {
            error!("EAU init failed: {:?}", err);
            None
        }
    };
    let mut eau_samples = 0;
    let mut current_controller = FocController::new(JD4631);
    current_controller.iq_reference = 1_000;
    let mut speed_controller = SpeedController::new(240);
    let mut current_stats = Stats::new();
    let mut speed_stats = Stats::new();
    let mut lut_stats = Stats::new();
    let mut software_div_stats = Stats::new();
    let mut eau_stats = Stats::new();
    for sample in 0..SAMPLES {
        let angle = (sample as u16).wrapping_mul(257);
        let start = SYST::get_current();
        let duty = current_controller.update(PhaseCurrent { a: 120, b: -80 }, 23_170, 23_170);
        compiler_fence(Ordering::SeqCst);
        core::hint::black_box(duty.a);
        current_stats.add(elapsed_cycles(start, SYST::get_current()));
        let start = SYST::get_current();
        let value = speed_controller.update_for_ticks(240, 180, 1);
        compiler_fence(Ordering::SeqCst);
        core::hint::black_box(value);
        speed_stats.add(elapsed_cycles(start, SYST::get_current()));
        lut_stats.add(time_lut(angle));
        software_div_stats.add(time_software_division(angle));
        if let Some(engine) = eau.as_mut() {
            match time_eau(engine, angle) {
                Ok(cycles) => {
                    eau_stats.add(cycles);
                    eau_samples += 1;
                }
                Err(err) => {
                    error!("EAU division failed at sample {=u32}: {:?}", sample, err);
                    eau = None;
                }
            }
        }
    }
    info!(
        "algorithm timing: systick={=u32}Hz samples={=u32}",
        rcc::clocks().hclk.0,
        SAMPLES
    );
    info!(
        "current loop: min_cycles={=u32} max_cycles={=u32} avg_cycles={=u32}",
        current_stats.min,
        current_stats.max,
        current_stats.total / SAMPLES
    );
    info!(
        "speed loop: min_cycles={=u32} max_cycles={=u32} avg_cycles={=u32}",
        speed_stats.min,
        speed_stats.max,
        speed_stats.total / SAMPLES
    );
    info!(
        "sin/cos LUT: min_cycles={=u32} max_cycles={=u32} avg_cycles={=u32}",
        lut_stats.min,
        lut_stats.max,
        lut_stats.total / SAMPLES
    );
    info!(
        "u32 division software: min_cycles={=u32} max_cycles={=u32} avg_cycles={=u32}",
        software_div_stats.min,
        software_div_stats.max,
        software_div_stats.total / SAMPLES
    );
    if eau_samples > 0 {
        info!(
            "u32 division EAU: samples={=u32} min_cycles={=u32} max_cycles={=u32} avg_cycles={=u32}",
            eau_samples,
            eau_stats.min,
            eau_stats.max,
            eau_stats.total / eau_samples
        );
    }
    hfi::measure(eau.as_mut());
    loop {
        asm::wfi();
    }
}
