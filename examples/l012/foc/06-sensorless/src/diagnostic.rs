//! Encoder scoring in thread mode. No reference angle is published to either ISR.
use crate::{
    current_loop::{
        FAULT, READY, ROUND_COUNT, ROUNDS_PER_POSITION, RUN_ID, SCAN_ROUNDS, STATE, record,
    },
    foc::calibration::Calibration,
    sensor::Encoder,
    timing::Timing,
};
use core::sync::atomic::Ordering::{Acquire, Release};

#[derive(Default)]
struct Samples {
    anchor: u16,
    low: i32,
    high: i32,
    sum: i32,
    count: i32,
    first_us: u32,
    last_us: u32,
    invalid: bool,
}

impl Samples {
    fn add(&mut self, raw: u16, now: u32) {
        if self.count == 0 {
            self.anchor = raw;
            self.first_us = now;
        } else if now.wrapping_sub(self.last_us) > 10_000 {
            self.invalid = true;
        }
        self.last_us = now;
        let delta = (i32::from(raw) - i32::from(self.anchor) + 8192).rem_euclid(16384) - 8192;
        self.low = self.low.min(delta);
        self.high = self.high.max(delta);
        self.sum += delta;
        self.count += 1;
    }
}

#[derive(Default)]
struct Stats {
    axes: u16,
    sum_abs: u32,
    max_abs: u16,
    poles: u16,
    wrong: u16,
}

impl Stats {
    fn add(&mut self, axis_error: i16, wrong: Option<bool>) {
        let abs = axis_error.unsigned_abs();
        self.axes += 1;
        self.sum_abs += u32::from(abs);
        self.max_abs = self.max_abs.max(abs);
        if let Some(wrong) = wrong {
            self.poles += 1;
            self.wrong += u16::from(wrong);
        }
    }
}

fn degrees_x10(q16: i32) -> i32 {
    q16 * 3600 / 65536
}

fn calibration() -> Calibration {
    unsafe extern "C" {
        static __foc_calibration: [u8; 12];
    }
    let address = core::ptr::addr_of!(__foc_calibration).cast::<u8>();
    let bytes = core::array::from_fn(|i| unsafe { address.add(i).read_volatile() });
    let Some(cal) = Calibration::decode(bytes) else {
        defmt::error!("06 comparison needs valid 01 FLASH calibration");
        crate::halt()
    };
    cal
}

pub fn run(encoder: &mut Encoder<'_>, timing: &mut Timing) -> ! {
    let cal = calibration();
    defmt::info!(
        "reference pairs={} dir={} zero={}; automatic 36-round scan",
        cal.pole_pairs,
        cal.direction,
        cal.mechanical_zero
    );
    // Leave time to finish the boot log before enabling the scan.
    timing.delay_us(100_000);
    READY.store(true, Release);
    let mut run = 0;
    let mut reported = 0;
    let mut captured = 0;
    let mut windows = [(None::<u16>, 0i32, 0i32); SCAN_ROUNDS];
    let mut samples = Samples::default();
    let mut position = Stats::default();
    let mut total = Stats::default();
    let mut last_sample = Timing::micros();
    let mut last_fault = 0;
    loop {
        timing.heartbeat();
        let epoch = RUN_ID.load(Acquire);
        if epoch != run {
            run = epoch;
            reported = 0;
            captured = 0;
            samples = Samples::default();
            position = Stats::default();
            total = Stats::default();
        }
        let fault = FAULT.load(Acquire);
        if fault != last_fault && fault != 0 {
            defmt::warn!(
                "scan stopped: fault={} completed={} state={} cycles={}",
                fault,
                ROUND_COUNT.load(Acquire),
                crate::current_loop::FAULT_STATE.load(Acquire),
                crate::current_loop::ISR_CYCLES.load(Acquire)
            );
        }
        last_fault = fault;
        let complete = usize::from(ROUND_COUNT.load(Acquire));
        while captured < complete {
            let sample = core::mem::take(&mut samples);
            let span = (sample.high - sample.low) * 4 * cal.pole_pairs;
            let valid = !sample.invalid
                && sample.count >= 2
                && sample.last_us.wrapping_sub(sample.first_us) >= 705_000
                && span <= 910;
            let raw = valid
                .then(|| (i32::from(sample.anchor) + sample.sum / sample.count) as u16 & 0x3fff);
            windows[captured] = (raw, span, sample.count);
            captured += 1;
        }
        if reported < captured && STATE.load(Acquire) == 0 {
            let (raw, span, count) = windows[reported];
            let result = record(reported);
            let pos = reported / ROUNDS_PER_POSITION + 1;
            let round = reported % ROUNDS_PER_POSITION + 1;
            if let Some(raw) = raw {
                // 01 uses the reversed 14-bit SSI coordinate before calibration.
                let reference = cal.angle(raw.wrapping_neg() & 0x3fff);
                let full = result.angle.wrapping_sub(reference) as i16;
                let axis = full.wrapping_mul(2) / 2;
                for (level, volts) in crate::pulse_sweep::VOLTS.iter().enumerate() {
                    defmt::info!(
                        "matrix pos={} round={} v={} ref={} axis={} hfi={} contrast={} p={=?} n={=?}",
                        pos,
                        round,
                        *volts,
                        reference,
                        crate::current_loop::probe_axis(reported),
                        result.angle,
                        result.contrast,
                        result.positive[level],
                        result.negative[level]
                    );
                }
                if result.contrast < 6 {
                    defmt::info!(
                        "pos={} round={} weak axis: contrast={} ref_q16={}",
                        pos,
                        round,
                        result.contrast,
                        reference
                    );
                } else {
                    // At exactly 90 degrees the two ends are equally distant.
                    let wrong = (result.polarity != 0 && full.unsigned_abs() != 16384)
                        .then_some(full.unsigned_abs() > 16384);
                    position.add(axis, wrong);
                    total.add(axis, wrong);
                    defmt::info!(
                        "pos={} round={} ref_q16={} hfi_q16={} axis_deg_x10={} pulse_diff={} pole_wrong={} (-1=undecided)",
                        pos,
                        round,
                        reference,
                        result.angle,
                        degrees_x10(i32::from(axis)),
                        result.polarity,
                        wrong.map_or(-1, i16::from)
                    );
                }
            } else {
                defmt::info!(
                    "pos={} round={} reference invalid: n={} motion_deg_x10={}",
                    pos,
                    round,
                    count,
                    degrees_x10_span(span)
                );
            }
            timing.delay_us(35_000);
            reported += 1;
            if round == ROUNDS_PER_POSITION {
                for (group, stats, attempts) in [(pos, &position, round), (0, &total, reported)] {
                    if group != 0 || reported == SCAN_ROUNDS {
                        if stats.axes > 0 {
                            defmt::info!(
                                "summary pos={} attempts={} axis_n={} mean_abs_deg_x10={} max_abs_deg_x10={} pole_wrong={} pole_n={}",
                                group,
                                attempts,
                                stats.axes,
                                degrees_x10((stats.sum_abs / u32::from(stats.axes)) as i32),
                                degrees_x10(i32::from(stats.max_abs)),
                                stats.wrong,
                                stats.poles
                            );
                        } else {
                            defmt::info!(
                                "summary pos={} attempts={} axis_n=0 pole_wrong=0 pole_n=0",
                                group,
                                attempts
                            );
                        }
                    }
                }
                position = Stats::default();
            }
        }
        let now = Timing::micros();
        if matches!(STATE.load(Acquire), 2 | 9)
            && usize::from(ROUND_COUNT.load(Acquire)) == captured
            && now.wrapping_sub(last_sample) >= 2000
        {
            last_sample = now;
            let raw = encoder.read_angle();
            if matches!(STATE.load(Acquire), 2 | 9)
                && usize::from(ROUND_COUNT.load(Acquire)) == captured
                && RUN_ID.load(Acquire) == run
            {
                match raw {
                    Ok(angle) => samples.add(angle, now),
                    Err(_) => samples.invalid = true,
                }
            }
        }
        cortex_m::asm::wfi();
    }
}

// The unwrapped motion span may cover many electrical turns.
fn degrees_x10_span(q16: i32) -> i32 {
    (q16 / 65536) * 3600 + degrees_x10(q16 % 65536)
}

/// Log only after MOE drops so RTT cannot perturb the current ISR.
pub fn drag(encoder: &mut Encoder<'_>, timing: &mut Timing) -> ! {
    use crate::current_loop::{ANGLE, DRAG_STATE, RPM, SCAN_DONE, TARGET};
    let cal = calibration();
    let mut trace = [(0u32, 0u16, 0u16, 0u16, 0u16, 0i32); 72];
    let mut n = 0;
    let start = Timing::micros();
    let mut last = start;
    READY.store(true, Release);
    while !SCAN_DONE.load(Acquire) && FAULT.load(Acquire) == 0 {
        let now = Timing::micros();
        if matches!(STATE.load(Acquire), DRAG_STATE | 3)
            && now.wrapping_sub(last) >= 100_000
            && n < trace.len()
        {
            last = now;
            if let Ok(raw) = encoder.read_angle() {
                trace[n] = (
                    now.wrapping_sub(start),
                    raw,
                    STATE.load(Acquire),
                    TARGET.load(Acquire) as u16,
                    ANGLE.load(Acquire),
                    RPM.load(Acquire),
                );
                n += 1;
            }
        }
        timing.heartbeat();
        cortex_m::asm::wfi();
    }
    defmt::info!("drag end fault={} samples={}", FAULT.load(Acquire), n);
    for &(us, raw, stage, target, angle, rpm) in &trace[..n] {
        defmt::info!(
            "drag us={} raw={} ref={} state={} target={} hfi={} rpm={}",
            us,
            raw,
            cal.angle(raw.wrapping_neg() & 0x3fff),
            stage,
            target,
            angle,
            rpm
        );
        timing.delay_us(5000);
    }
    crate::halt()
}
