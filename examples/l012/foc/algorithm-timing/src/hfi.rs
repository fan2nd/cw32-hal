//! Exercise the same software kernels and carrier phases as 06, with the bridge off.
use super::{Stats, elapsed_cycles};
use core::hint::black_box;
use cortex_m::peripheral::SYST;
use cw32_l012_foc_algorithms::{
    Duty, FocController, JD4631, PhaseCurrent, SpeedController,
    hfi::{Scan, Tracker},
    sin_cos_q15_lut, svpwm,
};

fn voltage(angle: u16, amplitude: i32) -> (i32, i32) {
    let (s, c) = sin_cos_q15_lut(angle);
    (
        amplitude * i32::from(c) >> 15,
        amplitude * i32::from(s) >> 15,
    )
}

#[inline(never)]
fn scan_tick(scan: &mut Scan, sample: PhaseCurrent) -> (Duty, bool) {
    let (angle, amplitude, result) = scan.sample(sample);
    let done = result.is_some();
    if let Some(detection) = result {
        black_box((detection.angle, detection.contrast, detection.polarity));
    }
    let (a, b) = voltage(angle, amplitude);
    (svpwm(a, b), done)
}

#[inline(never)]
fn run_tick(
    tracker: &mut Tracker,
    current: &mut FocController,
    speed: &mut SpeedController,
    sample: PhaseCurrent,
    speed_tick: bool,
    target: i32,
    eau: Option<&mut embassy_cw32::eau::Eau<embassy_cw32::peripherals::EAU>>,
) -> Option<Duty> {
    let amplitude = tracker.sample_with_divide(sample, |a, b| {
        if let Some(engine) = eau {
            return engine
                .divide_signed(a, b, 32)
                .expect("EAU timing failed")
                .quotient;
        }
        a / b
    })?;
    if speed_tick {
        current.iq_reference =
            speed.update_relative(target, tracker.rpm(11) * 64, 4, 200, |a, b| a / b);
    }
    let angle = tracker.angle();
    let (s, c) = sin_cos_q15_lut(angle);
    let (a, b) = current.update_voltage(tracker.fundamental, s, c);
    let (ha, hb) = voltage(angle, amplitude);
    Some(svpwm(a + ha, b + hb))
}

pub fn measure(mut eau: Option<&mut embassy_cw32::eau::Eau<embassy_cw32::peripherals::EAU>>) {
    let mut stats = [Stats::new(); 7];
    let mut counts = [0u32; 7];
    for trial in 0..8 {
        let mut scan = Scan::new();
        for tick in 0.. {
            let a = ((tick + trial) % 173) as i16 - 86;
            let sample = PhaseCurrent { a, b: -a / 2 };
            let start = SYST::get_current();
            let (_, done) = black_box(scan_tick(black_box(&mut scan), black_box(sample)));
            let cycles = elapsed_cycles(start, SYST::get_current());
            let group = if done {
                2
            } else if tick % 40 == 31 || tick % 40 == 39 {
                1
            } else {
                0
            };
            stats[group].add(cycles);
            counts[group] += 1;
            if done {
                break;
            }
        }
    }
    for hardware in [false, true] {
        if hardware && eau.is_none() {
            continue;
        }
        let mut tracker = Tracker::new(1200);
        let mut current = FocController::new(JD4631);
        let mut speed = SpeedController::new(600);
        for tick in 0..4096 {
            let phase = tick % 16;
            let peak = if phase < 8 { -80 } else { 80 };
            let (a, beta) = voltage(tracker.angle(), peak);
            let sample = PhaseCurrent {
                a: a as i16,
                b: ((beta * 56756 / 32768 - a) / 2 + 4) as i16,
            };
            let start = SYST::get_current();
            let duty = black_box(run_tick(
                black_box(&mut tracker),
                &mut current,
                &mut speed,
                black_box(sample),
                black_box(phase == 15),
                black_box(120),
                if hardware { eau.as_deref_mut() } else { None },
            ));
            let cycles = elapsed_cycles(start, SYST::get_current());
            if duty.is_none() {
                defmt::error!("HFI timing fixture lost carrier lock");
                return;
            }
            if hardware && phase != 0 {
                continue;
            }
            let group = match phase {
                0 if hardware => 6,
                0 => 4,
                15 => 5,
                _ => 3,
            };
            stats[group].add(cycles);
            counts[group] += 1;
        }
    }
    for (group, s) in stats.iter().enumerate() {
        if counts[group] == 0 {
            continue;
        }
        defmt::info!(
            "HFI group={=usize} n={=u32} min={=u32} max={=u32} avg={=u32} cycles",
            group,
            counts[group],
            s.min,
            s.max,
            s.total / counts[group]
        );
    }
}
