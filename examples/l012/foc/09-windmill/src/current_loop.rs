//! ADC1 owns both current control and the injection waveform. No SSI data enters this module.
use crate::foc::{
    self, Duty, FocController, JD4631, PhaseCurrent, SpeedController,
    bemf::{Bemf, Sampler},
    control::target_rpm_from_pot,
    drag::{Drag, HANDOFF_RPM},
    hfi::{Scan, StandstillWindow, Tracker, nearest_axis_angle},
};
use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, AtomicI16, AtomicI32, AtomicU16, Ordering::*},
};
use cortex_m::interrupt::{self, Mutex};
use embassy_cw32::{
    self as hal,
    adc::{adc_irq_clear, adc_irq_enable_eos, adc_irq_eos, adc_irq_result},
    eau::Eau,
    interrupt::typelevel::Handler,
    peripherals::{ADC1, ADC2, ATIM, EAU},
    timer::{
        Channel,
        complementary_pwm::{irq_arr, irq_ccr, irq_moe},
    },
};

// With this board's phase wiring, 03's positive mechanical direction
// corresponds to decreasing electrical angle (01 measured direction = -1).
const ELECTRICAL_DIRECTION: i16 = -1;

// 0 = HFI, 1 = BEMF, 2 = BEMF controlling while HFI reacquires.
// Latched for RTT even when the capture finishes between two log periods.
pub static WIND_RESULT: AtomicI16 = AtomicI16::new(0);
pub static OBSERVER: AtomicU16 = AtomicU16::new(0);
static EMF_ANGLE: AtomicU16 = AtomicU16::new(0);
static EMF_STEP: AtomicI32 = AtomicI32::new(0);
static EMF_RPM: AtomicI32 = AtomicI32::new(0);
static EMF_VALID: AtomicBool = AtomicBool::new(false);
static EMF_MV: AtomicI32 = AtomicI32::new(0);
static EMF_TIME: AtomicU16 = AtomicU16::new(0);

pub fn observe(observer: &mut Bemf) {
    let frame = interrupt::free(|cs| DRIVE.borrow(cs).borrow_mut().sampler.take());
    if !matches!(STATE.load(Acquire), 3 | 9) {
        *observer = Bemf::new();
        EMF_VALID.store(false, Release);
        return;
    }
    if let Some((tick, frame)) = frame {
        if tick.wrapping_sub(EMF_TIME.load(Relaxed)) != 16 {
            *observer = Bemf::new();
        }
        observer.update(frame, BUS_MV.load(Relaxed));
        let rpm = -observer.rpm();
        interrupt::free(|_| {
            EMF_MV.store(observer.magnitude_mv, Relaxed);
            EMF_ANGLE.store(observer.angle(), Relaxed);
            EMF_STEP.store(observer.electrical_step(), Relaxed);
            EMF_RPM.store(rpm, Relaxed);
            EMF_TIME.store(tick, Relaxed);
            EMF_VALID.store(observer.valid, Release);
        });
    }
}
static BUS_MV: AtomicI32 = AtomicI32::new(12000);
pub static READY: AtomicBool = AtomicBool::new(false);
pub static CURRENT_LOOP_COUNT: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_OFFSET_A: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_OFFSET_B: AtomicU16 = AtomicU16::new(0);
pub static ANGLE: AtomicU16 = AtomicU16::new(0);
pub static STATE: AtomicU16 = AtomicU16::new(0);
pub static FAULT: AtomicU16 = AtomicU16::new(0);
pub static RPM: AtomicI32 = AtomicI32::new(0);
static SETTLE_MS: AtomicU16 = AtomicU16::new(0);
pub static TARGET: AtomicI32 = AtomicI32::new(0);
pub static IQ: AtomicI16 = AtomicI16::new(0);
pub static CONTRAST: AtomicI32 = AtomicI32::new(0);
pub static POLARITY: AtomicI32 = AtomicI32::new(0);

pub static ALIGN_FIELD: AtomicU16 = AtomicU16::new(0);
pub static ALIGN_ERROR: AtomicI16 = AtomicI16::new(0);
pub static FAULT_TARGET: AtomicI32 = AtomicI32::new(0);
pub static FAULT_IQ: AtomicI16 = AtomicI16::new(0);
pub static FAULT_STATE: AtomicU16 = AtomicU16::new(0);
pub static FAULT_RPM: AtomicI32 = AtomicI32::new(0);
pub static ISR_CYCLES: AtomicI32 = AtomicI32::new(0);

fn trip(code: u16) {
    irq_moe::<ATIM>(false);
    interrupt::free(|_| {
        if FAULT.load(Relaxed) == 0 {
            FAULT_TARGET.store(TARGET.load(Relaxed), Relaxed);
            FAULT_RPM.store(RPM.load(Relaxed), Relaxed);
            FAULT_IQ.store(IQ.load(Relaxed), Relaxed);
            FAULT_STATE.store(STATE.load(Relaxed), Relaxed);
            FAULT.store(code, Release);
        }
        TARGET.store(0, Release);
    });
}

// BTIM1 also catches a missing ADC trigger while the bridge is enabled.
static START_IQ_LIMIT: AtomicI16 = AtomicI16::new(0);
static LAST_COUNT: AtomicU16 = AtomicU16::new(0);
static ZERO_TICKS: AtomicU16 = AtomicU16::new(0);
static ARMED: AtomicBool = AtomicBool::new(false);
pub fn supervise(speed: &mut SpeedController) {
    if !READY.load(Acquire) {
        return;
    }
    if STATE.load(Acquire) != 3 {
        START_IQ_LIMIT.store(0, Relaxed);
        speed.reset();
        IQ.store(0, Relaxed);
    }
    let count = CURRENT_LOOP_COUNT.load(Relaxed);
    let delta = count.wrapping_sub(LAST_COUNT.load(Relaxed));
    LAST_COUNT.store(count, Relaxed);
    if STATE.load(Acquire) != 0 && !(8..=24).contains(&delta) {
        trip(1);
    }
    let Some(pot) = adc_irq_result::<ADC2>(1) else {
        trip(2);
        return;
    };
    if FAULT.load(Acquire) != 0 {
        ARMED.store(false, Relaxed);
    }
    if pot <= 150 {
        TARGET.store(0, Release);
        let ticks = ZERO_TICKS.load(Relaxed).saturating_add(1);
        ZERO_TICKS.store(ticks, Relaxed);
        if ticks >= 100 && delta > 0 && STATE.load(Acquire) == 0 {
            FAULT.store(0, Release);
            ARMED.store(true, Release);
        }
        return;
    }
    ZERO_TICKS.store(0, Relaxed);
    if !ARMED.load(Acquire) {
        return;
    }
    let bus = adc_irq_result::<ADC2>(0).unwrap_or(0);
    let mv = u32::from(bus) * 55000 / 4095;
    if !(8000..=16000).contains(&mv) {
        trip(3);
        return;
    }
    BUS_MV.store(mv as i32, Relaxed);
    let requested = target_rpm_from_pot(pot, 150, 200, 3523, 0, 600);
    let previous = TARGET.load(Relaxed);
    let target = if previous > 0 && requested > 0 && (requested - previous).abs() == 1 {
        previous
    } else {
        requested
    };
    // During the first closed-loop second, keep the speed command at or below
    // the catch speed. Zero and lower operator commands still take effect.
    let remaining = SETTLE_MS.load(Relaxed);
    let target = if remaining > 0 {
        SETTLE_MS.store(remaining - 1, Relaxed);
        target.min(HANDOFF_RPM)
    } else { target };
    TARGET.store(target, Release);
    if target > 0 && STATE.load(Acquire) == 3 && FAULT.load(Acquire) == 0 {
        // Limit the PI itself during sensorless handover to avoid integral windup.
        let limit = (START_IQ_LIMIT.load(Relaxed) + 2).min(200);
        START_IQ_LIMIT.store(limit, Relaxed);
        let rpm = RPM.load(Relaxed);
        // Taper reverse torque as the rotor approaches the requested speed. The HFI speed
        // estimate lags the shaft during hard braking on this board.
        let brake_limit = (((rpm - target).clamp(0, 160) * 200 / 160) as i16).min(limit);
        IQ.store(
            speed.update_relative_in_range(target, rpm * 64, 4, -brake_limit, limit, |a, b| a / b),
            Release,
        );
    }
}

struct Drive {
    drag: Drag,
    eau: Option<Eau<EAU>>,
    scan: Scan,
    tracker: Tracker,
    sampler: Sampler,
    observing: bool,
    applied: Duty,
    wind_streak: i16,
    quiet_ms: u16,
    frame_tick: u16,
    braking: bool,
    consistent_ms: u16,
    bad_ms: u16,
    phase_offset: i16,
    standstill: StandstillWindow,
    current: FocController,
    ticks: u16,
    stalled: u16,
}
static DRIVE: Mutex<RefCell<Drive>> = Mutex::new(RefCell::new(Drive {
    drag: Drag::new(),
    eau: None,
    scan: Scan::new(),
    tracker: Tracker::new(0),
    sampler: Sampler::new(),
    observing: false,
    applied: Duty {
        a: 16384,
        b: 16384,
        c: 16384,
    },
    wind_streak: 0,
    quiet_ms: 0,
    frame_tick: 0,
    braking: false,
    consistent_ms: 0,
    bad_ms: 0,
    phase_offset: 0,
    standstill: StandstillWindow::new(),
    current: FocController::new(JD4631),
    ticks: 0,
    stalled: 0,
}));

pub fn init_eau() {
    let eau = Eau::new().ok();
    interrupt::free(|cs| DRIVE.borrow(cs).borrow_mut().eau = eau);
}

fn voltage(angle: u16, amplitude: i32) -> (i32, i32) {
    let (sin, cos) = foc::sin_cos_q15_lut(angle);
    (
        (i32::from(cos) * amplitude) >> 15,
        (i32::from(sin) * amplitude) >> 15,
    )
}

impl Drive {
    fn tick(&mut self, phase: PhaseCurrent, target: i32) -> Result<Duty, u16> {
        match STATE.load(Relaxed) {
            0 => {
                self.sampler = Sampler::new();
                self.observing = false;
                self.consistent_ms = 0;
                self.bad_ms = 0;
                self.phase_offset = 0;
                self.drag = Drag::new();
                SETTLE_MS.store(0, Relaxed);
                OBSERVER.store(0, Relaxed);
                self.scan = Scan::new();
                ALIGN_FIELD.store(0, Relaxed);
                ALIGN_ERROR.store(0, Relaxed);
                self.current.reset();
                self.current.iq_reference = 0;
                self.ticks = 0;
                self.stalled = 0;
                CONTRAST.store(0, Relaxed);
                POLARITY.store(0, Relaxed);
                STATE.store(1, Release);
                Ok(foc::svpwm(0, 0))
            }
            1 => {
                // One neutral preload has latched since the previous EOS.
                irq_moe::<ATIM>(true);
                WIND_RESULT.store(0, Relaxed);
                self.wind_streak = 0;
                self.quiet_ms = 0;
                self.braking = false;
                self.frame_tick = EMF_TIME.load(Relaxed);
                self.applied = foc::svpwm(0, 0);
                STATE.store(9, Release);
                Ok(self.applied)
            }
            9 => {
                // In stationary coordinates, regulating both currents to zero
                // exposes the rotor EMF in the applied PWM voltage.
                let tick = CURRENT_LOOP_COUNT.load(Relaxed);
                self.sampler.push(tick, self.applied, phase);
                self.ticks += 1;
                if self.ticks >= 48000 {
                    return Err(13);
                }
                let stamp = EMF_TIME.load(Relaxed);
                let age = tick.wrapping_sub(stamp);
                let valid = EMF_VALID.load(Relaxed) && age <= 32;
                let rpm = EMF_RPM.load(Relaxed);
                let step = EMF_STEP.load(Relaxed);
                let angle = EMF_ANGLE
                    .load(Relaxed)
                    .wrapping_add_signed((step * i32::from(age.min(32)) / 16) as i16);
                if stamp != self.frame_tick {
                    if stamp.wrapping_sub(self.frame_tick) != 16 {
                        self.wind_streak = 0;
                        self.quiet_ms = 0;
                        self.braking = false;
                    }
                    self.frame_tick = stamp;
                    RPM.store(rpm, Relaxed);
                    if valid && rpm.abs() > 660 {
                        return Err(8);
                    }
                    self.quiet_ms = if age <= 32 && EMF_MV.load(Relaxed) < 180 {
                        self.quiet_ms + 1
                    } else {
                        0
                    };
                    self.wind_streak = if valid && rpm >= 120 {
                        self.wind_streak.max(0).saturating_add(1)
                    } else if valid && rpm <= -40 {
                        self.wind_streak.min(0).saturating_sub(1)
                    } else {
                        0
                    };
                    if self.wind_streak >= 32 {
                        WIND_RESULT.store(1, Relaxed);
                        let (sin, cos) = foc::sin_cos_q15_lut(angle);
                        self.current.enter_rotor_frame(sin, cos);
                        self.current.id_reference = 0;
                        self.current.iq_reference = 0;
                        self.observing = true;
                        OBSERVER.store(1, Relaxed);
                        self.ticks = 0;
                        STATE.store(3, Release);
                        return Ok(self.applied);
                    }
                    if self.quiet_ms >= 100 {
                        self.current.reset();
                        self.current.id_reference = 0;
                        self.current.iq_reference = 0;
                        self.sampler = Sampler::new();
                        self.ticks = 0;
                        STATE.store(2, Release);
                        return Ok(foc::svpwm(0, 0));
                    }
                    self.braking = valid && rpm < -20 && (self.braking || self.wind_streak <= -24);
                }
                let (alpha, beta) = if valid && self.braking && rpm < -20 {
                    // angle() assumes decreasing electrical angle. Reverse
                    // rotation changes the EMF-to-flux quarter-turn by pi.
                    WIND_RESULT.store(-1, Relaxed);
                    let rotor = angle.wrapping_add(32768);
                    let (sin, cos) = foc::sin_cos_q15_lut(rotor);
                    let iq = rpm.clamp(-60, 0);
                    (
                        (-iq * i32::from(sin) >> 15) as i16,
                        (iq * i32::from(cos) >> 15) as i16,
                    )
                } else {
                    (0, 0)
                };
                self.current.id_reference = alpha;
                self.current.iq_reference = beta;
                let (a, b) = self.current.update_voltage(phase, 0, 32767);
                self.applied = foc::svpwm(a, b);
                Ok(self.applied)
            }
            4 => {
                // Use the same 25% stationary voltage vector as 01.
                // Ramp up for 100 ms, hold for 400 ms, then let current and motion settle.
                let tick = self.ticks;
                self.ticks += 1;
                if tick < 8000 {
                    let field = ALIGN_FIELD.load(Relaxed);
                    let amplitude = (i32::from(tick) * 8192 / 1600).min(8192);
                    let (a, b) = voltage(field, amplitude);
                    Ok(foc::svpwm(a, b))
                } else {
                    if self.ticks >= 12800 {
                        STATE.store(7, Release);
                    }
                    Ok(foc::svpwm(0, 0))
                }
            }
            2 if target >= HANDOFF_RPM => {
                self.tracker = Tracker::new(0);
                STATE.store(10, Release);
                Ok(foc::svpwm(0, 0))
            }
            10 => {
                let output = self.drag.tick(phase, &mut self.tracker)?;
                if let Some(rpm) = output.rpm { RPM.store(rpm, Relaxed); }
                ANGLE.store(self.tracker.angle(), Relaxed);
                if output.ready {
                    self.drag.prime(&mut self.current, &self.tracker);
                    SETTLE_MS.store(1200, Relaxed);
                    self.ticks = 0;
                    STATE.store(3, Release);
                }
                self.applied = foc::svpwm(output.voltage.0, output.voltage.1);
                Ok(self.applied)
            }
            state @ (2 | 7) => {
                let (axis, v, done) = self.scan.sample(phase);
                if let Some(result) = done {
                    CONTRAST.store(result.contrast, Relaxed);
                    POLARITY.store(result.polarity, Relaxed);
                    if result.contrast < 6 {
                        return Err(5);
                    }
                    if state == 2 {
                        // A quadrature field produces alignment torque for either pole.
                        ALIGN_FIELD.store((result.angle & 0x7fff) + 16384, Relaxed);
                        self.scan = Scan::new();
                        self.ticks = 0;
                        STATE.store(4, Release);
                    } else {
                        let field = ALIGN_FIELD.load(Relaxed);
                        let north = nearest_axis_angle(result.angle, field);
                        let error = north.wrapping_sub(field) as i16;
                        ALIGN_ERROR.store(error, Relaxed);
                        if error.unsigned_abs() > 6372 {
                            // 35 electrical degrees
                            return Err(6);
                        }
                        self.tracker = Tracker::new(north);
                        self.standstill = StandstillWindow::new();
                        ANGLE.store(north, Relaxed);
                        self.ticks = 0;
                        STATE.store(8, Release);
                    }
                    return Ok(foc::svpwm(0, 0));
                }
                let (a, b) = voltage(axis, v);
                Ok(foc::svpwm(a, b))
            }
            state => {
                let mode = OBSERVER.load(Relaxed);
                let tick = CURRENT_LOOP_COUNT.load(Relaxed);
                let window = state == 3 && self.ticks % 16 == 6;
                // Publish on carrier phase 4, away from the phase-0 demodulation
                // and phase-15 speed checks. Keep full consecutive windows.
                if !self.observing && state == 3 && self.ticks % 16 == 5 && RPM.load(Relaxed) >= 200
                {
                    self.observing = true;
                }
                if self.observing {
                    self.sampler.push(tick, self.applied, phase);
                }
                let carrier = if mode == 1 {
                    0
                } else {
                    self.tracker
                        .track(phase, state == 8, |a, b| {
                            if let Some(eau) = self.eau.as_mut() {
                                if let Ok(result) = eau.divide_signed(a, b, 32) {
                                    return result.quotient;
                                }
                            }
                            a / b
                        })
                        .ok_or(7u16)?
                };
                let injection_axis = self.tracker.angle();
                let hfi_angle = injection_axis.wrapping_add(self.drag.pole_offset);
                // With HFI in control the EMF estimate is only needed on
                // the 1 ms agreement check, away from carrier demodulation.
                let (step, emf_angle, valid) = if mode != 0 || window {
                    let age = tick.wrapping_sub(EMF_TIME.load(Relaxed));
                    let step = EMF_STEP.load(Relaxed);
                    let angle = EMF_ANGLE.load(Relaxed).wrapping_add_signed((step * i32::from(age.min(32)) / 16) as i16);
                    (step, angle, EMF_VALID.load(Acquire) && age <= 32)
                } else { (0, 0, false) };
                let mut source = mode;
                if window {
                    let emf_rpm = EMF_RPM.load(Relaxed);
                    let hfi_rpm = -self.tracker.rpm(i32::from(JD4631.pole_pairs));
                    let (angle_limit, speed_limit) =
                        if mode == 2 { (4551, 65) } else { (2731, 40) };
                    let agree = valid
                        && self.tracker.carrier_valid
                        && (emf_angle.wrapping_sub(hfi_angle) as i16).unsigned_abs() < angle_limit
                        && (emf_rpm - hfi_rpm).abs() < speed_limit;
                    if mode == 0 || mode == 2 {
                        let qualify = agree && (mode == 2 || (hfi_rpm >= 360 && target >= 360));
                        self.consistent_ms = if qualify { self.consistent_ms + 1 } else { 0 };
                        if self.consistent_ms >= if mode == 2 { 6 } else { 32 } {
                            source = if mode == 2 { 0 } else { 1 };
                            self.phase_offset = if source == 1 {
                                hfi_angle.wrapping_sub(emf_angle) as i16
                            } else {
                                emf_angle.wrapping_sub(hfi_angle) as i16
                            };
                            self.consistent_ms = 0;
                            self.bad_ms = 0;
                        }
                    }
                    if mode == 1 && (emf_rpm <= 280 || (target < 320 && emf_rpm <= 360)) {
                        self.drag.pole_offset = 0;
                        self.tracker = Tracker::from_motion(emf_angle, step);
                        source = 2;
                        self.consistent_ms = 0;
                        self.bad_ms = 0;
                    }
                    if mode != 0 {
                        self.bad_ms = if !valid || (mode == 2 && emf_rpm < 100) {
                            self.bad_ms + 1
                        } else {
                            0
                        };
                        if self.bad_ms >= 8 {
                            return Err(12);
                        }
                    }
                    OBSERVER.store(source, Relaxed);
                }
                let base = if source == 0 { hfi_angle } else { emf_angle };
                let angle = base.wrapping_add_signed(self.phase_offset);
                self.phase_offset -= self.phase_offset.clamp(-32, 32);
                ANGLE.store(angle, Relaxed);
                self.ticks = self.ticks.wrapping_add(1);
                if self.ticks % 16 == 0 {
                    let rpm = if source == 0 {
                        i32::from(ELECTRICAL_DIRECTION)
                            * self.tracker.rpm(i32::from(JD4631.pole_pairs))
                    } else {
                        EMF_RPM.load(Relaxed)
                    };
                    RPM.store(rpm, Relaxed);
                    if !(-20..=660).contains(&rpm) {
                        if state == 8 {
                            return Err(11);
                        }
                        return Err(8);
                    }
                    if state == 8 {
                        // Acquire with carrier alone. Only hand off after 32 consecutive
                        // low-speed carrier periods and bound the total acquisition time.
                        let field = ALIGN_FIELD.load(Relaxed);
                        if self
                            .standstill
                            .update(angle, field, self.tracker.carrier_valid)
                            && self.ticks >= 1600
                        {
                            self.stalled = 0;
                            STATE.store(3, Release);
                        } else if self.ticks >= 8000 {
                            return Err(11);
                        }
                    } else {
                        self.stalled = if rpm * 2 < target.min(10) {
                            self.stalled + 1
                        } else {
                            0
                        };
                        if self.stalled >= 2000 {
                            return Err(9);
                        }
                    }
                }
                if state == 8 {
                    let (a, b) = voltage(angle, carrier);
                    return Ok(foc::svpwm(a, b));
                }
                (self.current.id_reference, self.current.iq_reference) = self.drag.references(0, ELECTRICAL_DIRECTION * IQ.load(Acquire));
                let (sin, cos) = foc::sin_cos_q15_lut(angle);
                let (a, b) = self.current.update_voltage(
                    if mode == 1 {
                        phase
                    } else {
                        self.tracker.fundamental
                    },
                    sin,
                    cos,
                );
                let (a, b) = self.drag.blend(a, b);
                let (ha, hb) = if source == 1 {
                    (0, 0)
                } else if injection_axis == angle {
                    (i32::from(cos) * carrier >> 15, i32::from(sin) * carrier >> 15)
                } else {
                    voltage(injection_axis, carrier)
                };
                self.applied = foc::svpwm(a + ha, b + hb);
                Ok(self.applied)
            }
        }
    }
}

pub struct CurrentLoop;
impl Handler<hal::interrupt::typelevel::ADC1> for CurrentLoop {
    unsafe fn on_interrupt() {
        if !adc_irq_eos::<ADC1>() {
            return;
        }
        let entry = cortex_m::peripheral::SYST::get_current();
        adc_irq_enable_eos::<ADC1>(false);
        let samples = adc_irq_result::<ADC1>(0).zip(adc_irq_result::<ADC1>(1));
        adc_irq_clear::<ADC1>();
        CURRENT_LOOP_COUNT.store(CURRENT_LOOP_COUNT.load(Relaxed).wrapping_add(1), Relaxed);
        let target = TARGET.load(Acquire);
        if !READY.load(Acquire) || target == 0 || FAULT.load(Acquire) != 0 {
            irq_moe::<ATIM>(false);
            STATE.store(0, Release);
            IQ.store(0, Relaxed);
            RPM.store(0, Relaxed);
        } else if let Some((a, b)) = samples {
            let phase = PhaseCurrent {
                a: CURRENT_OFFSET_A.load(Relaxed) as i16 - a as i16,
                b: CURRENT_OFFSET_B.load(Relaxed) as i16 - b as i16,
            };
            if phase.a.unsigned_abs() > 2000 || phase.b.unsigned_abs() > 2000 {
                trip(4);
            } else {
                let duty = interrupt::free(|cs| DRIVE.borrow(cs).borrow_mut().tick(phase, target));
                match duty {
                    Ok(duty) => {
                        let period = irq_arr::<ATIM>();
                        for (channel, value) in [
                            (Channel::Ch1, duty.a),
                            (Channel::Ch2, duty.b),
                            (Channel::Ch3, duty.c),
                        ] {
                            irq_ccr::<ATIM>(channel, ((u32::from(value) * period) >> 15) as u16);
                        }
                    }
                    Err(code) => trip(code),
                }
            }
        } else {
            trip(2);
        }
        if adc_irq_eos::<ADC1>() && target != 0 && FAULT.load(Acquire) == 0 {
            let end = cortex_m::peripheral::SYST::get_current();
            let elapsed = if entry >= end {
                entry - end
            } else {
                entry + cortex_m::peripheral::SYST::get_reload() + 1 - end
            };
            ISR_CYCLES.store(elapsed as i32, Relaxed);
            trip(10);
        }
        adc_irq_enable_eos::<ADC1>(true);
    }
}
hal::bind_interrupts!(pub struct Irqs { ADC1 => CurrentLoop; });
