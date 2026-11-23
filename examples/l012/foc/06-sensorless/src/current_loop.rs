//! ADC1 owns both current control and the injection waveform. No SSI data enters this module.
use crate::foc::{
    self, Duty, FocController, JD4631, PhaseCurrent, SpeedController,
    control::target_rpm_from_pot,
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

pub static READY: AtomicBool = AtomicBool::new(false);
pub static CURRENT_LOOP_COUNT: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_OFFSET_A: AtomicU16 = AtomicU16::new(0);
pub static CURRENT_OFFSET_B: AtomicU16 = AtomicU16::new(0);
pub static ANGLE: AtomicU16 = AtomicU16::new(0);
pub static STATE: AtomicU16 = AtomicU16::new(0);
pub static FAULT: AtomicU16 = AtomicU16::new(0);
pub static RPM: AtomicI32 = AtomicI32::new(0);
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

// Set to zero to use normal sensorless startup. In this build every start is scan-only.
const FIELD_ANGLES: [u16; 6] = [0, 10922, 21845, 32768, 43690, 54613];
const ROUNDS_PER_POSITION: usize = 6;
const AUTO_INDEX_DIAGNOSTIC: bool = false;
pub const SCAN_ROUNDS: usize = if AUTO_INDEX_DIAGNOSTIC {
    FIELD_ANGLES.len() * ROUNDS_PER_POSITION
} else {
    0
};
#[derive(Clone, Copy)]
pub struct Round {
    pub raw_start: u16,
    pub raw_end: u16,
    pub positive: [i16; 12],
    pub negative: [i16; 12],
}
const EMPTY: Round = Round {
    raw_start: u16::MAX,
    raw_end: u16::MAX,
    positive: [0; 12],
    negative: [0; 12],
};
static RECORDS: Mutex<RefCell<[Round; SCAN_ROUNDS]>> =
    Mutex::new(RefCell::new([EMPTY; SCAN_ROUNDS]));
pub static ROUND_COUNT: AtomicU16 = AtomicU16::new(0);
pub static RAW_LOG_ANGLE: AtomicU16 = AtomicU16::new(u16::MAX);
static SCAN_DONE: AtomicBool = AtomicBool::new(false);
pub fn record(index: usize) -> Round {
    interrupt::free(|cs| RECORDS.borrow(cs).borrow()[index])
}

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
            SCAN_DONE.store(false, Release);
            ARMED.store(true, Release);
        }
        return;
    }
    ZERO_TICKS.store(0, Relaxed);
    if SCAN_DONE.load(Acquire) {
        TARGET.store(0, Release);
        return;
    }
    if !ARMED.load(Acquire) {
        return;
    }
    let bus = adc_irq_result::<ADC2>(0).unwrap_or(0);
    let mv = u32::from(bus) * 55000 / 4095;
    if !(8000..=16000).contains(&mv) {
        trip(3);
        return;
    }
    let requested = target_rpm_from_pot(pot, 150, 200, 3523, 0, 600);
    let previous = TARGET.load(Relaxed);
    let target = if previous > 0 && requested > 0 && (requested - previous).abs() == 1 {
        previous
    } else {
        requested
    };
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
    eau: Option<Eau<EAU>>,
    scan: Scan,
    tracker: Tracker,
    standstill: StandstillWindow,
    current: FocController,
    ticks: u16,
    stalled: u16,
    raw_start: u16,
    copy_axis: usize,
}
static DRIVE: Mutex<RefCell<Drive>> = Mutex::new(RefCell::new(Drive {
    eau: None,
    scan: Scan::new(),
    tracker: Tracker::new(0),
    standstill: StandstillWindow::new(),
    current: FocController::new(JD4631),
    ticks: 0,
    stalled: 0,
    raw_start: u16::MAX,
    copy_axis: 0,
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
                self.scan = Scan::new();
                ALIGN_FIELD.store(0, Relaxed);
                ALIGN_ERROR.store(0, Relaxed);
                self.current.reset();
                self.current.iq_reference = 0;
                ROUND_COUNT.store(0, Release);
                self.raw_start = RAW_LOG_ANGLE.load(Relaxed);
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
                STATE.store(if SCAN_ROUNDS == 0 { 2 } else { 6 }, Release);
                Ok(foc::svpwm(0, 0))
            }
            state @ (4 | 6) => {
                // Use the same 25% stationary voltage vector as 01.
                // Ramp up for 100 ms, hold for 400 ms, then let current and motion settle.
                let tick = self.ticks;
                self.ticks += 1;
                if tick < 8000 {
                    let field = if state == 4 {
                        ALIGN_FIELD.load(Relaxed)
                    } else {
                        FIELD_ANGLES[usize::from(ROUND_COUNT.load(Relaxed)) / ROUNDS_PER_POSITION]
                    };
                    let amplitude = (i32::from(tick) * 8192 / 1600).min(8192);
                    let (a, b) = voltage(field, amplitude);
                    Ok(foc::svpwm(a, b))
                } else {
                    if self.ticks >= 12800 {
                        self.raw_start = RAW_LOG_ANGLE.load(Relaxed);
                        STATE.store(if state == 4 { 7 } else { 2 }, Release);
                    }
                    Ok(foc::svpwm(0, 0))
                }
            }
            state @ (2 | 7) => {
                let (axis, v, done) = self.scan.sample(phase);
                if let Some(result) = done {
                    CONTRAST.store(result.contrast, Relaxed);
                    POLARITY.store(result.polarity, Relaxed);
                    if SCAN_ROUNDS != 0 {
                        self.copy_axis = 0;
                        STATE.store(5, Release);
                        return Ok(foc::svpwm(0, 0));
                    }
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
            5 => {
                let index = usize::from(ROUND_COUNT.load(Relaxed));
                let (p, n) = self.scan.axis_response(self.copy_axis);
                let end = RAW_LOG_ANGLE.load(Relaxed);
                interrupt::free(|cs| {
                    let mut records = RECORDS.borrow(cs).borrow_mut();
                    let row = &mut records[index];
                    row.positive[self.copy_axis] = p;
                    row.negative[self.copy_axis] = n;
                    row.raw_start = self.raw_start;
                    row.raw_end = end;
                });
                self.copy_axis += 1;
                if self.copy_axis == 12 {
                    ROUND_COUNT.store((index + 1) as u16, Release);
                    if index + 1 == SCAN_ROUNDS {
                        irq_moe::<ATIM>(false);
                        SCAN_DONE.store(true, Release);
                        TARGET.store(0, Release);
                        STATE.store(0, Release);
                    } else {
                        self.scan = Scan::new();
                        self.raw_start = end;
                        if (index + 1) % ROUNDS_PER_POSITION == 0 {
                            self.ticks = 0;
                            STATE.store(6, Release);
                        } else {
                            STATE.store(2, Release);
                        }
                    }
                }
                Ok(foc::svpwm(0, 0))
            }
            state => {
                let carrier = self
                    .tracker
                    .track(phase, state == 8, |a, b| {
                        if let Some(eau) = self.eau.as_mut() {
                            if let Ok(result) = eau.divide_signed(a, b, 32) {
                                return result.quotient;
                            }
                        }
                        a / b
                    })
                    .ok_or(7u16)?;
                let angle = self.tracker.angle();
                ANGLE.store(angle, Relaxed);
                self.ticks = self.ticks.wrapping_add(1);
                if self.ticks % 16 == 0 {
                    let rpm = i32::from(ELECTRICAL_DIRECTION)
                        * self.tracker.rpm(i32::from(JD4631.pole_pairs));
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
                self.current.iq_reference = ELECTRICAL_DIRECTION * IQ.load(Acquire);
                let (sin, cos) = foc::sin_cos_q15_lut(angle);
                let (a, b) = self
                    .current
                    .update_voltage(self.tracker.fundamental, sin, cos);
                let (ha, hb) = voltage(angle, carrier);
                Ok(foc::svpwm(a + ha, b + hb))
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
        if !READY.load(Acquire)
            || SCAN_DONE.load(Acquire)
            || target == 0
            || FAULT.load(Acquire) != 0
        {
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
