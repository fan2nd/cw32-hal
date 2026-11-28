use crate::{
    Analog, Bridge,
    calibration::Calibration,
    config::*,
    foc::SpeedController,
    global::*,
    hardware::{MAX_SAMPLE_DELTA_COUNTS, angle_delta, bus_voltage_mv},
    link::{self, Feedback},
    sensor::Encoder,
    timing::{self, Timing},
};
use core::sync::atomic::Ordering;
use embassy_cw32::{
    adc::adc_irq_result,
    peripherals::{ADC2, ATIM},
    timer::complementary_pwm::irq_moe,
};

pub struct Servo {
    _analog: Analog,
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    speed: SpeedController,
    previous: u16,
    counts: i32,
    rpm_x64: i32,
    last: u32,
    mode: u8,
    position_lost: bool,
}
pub(crate) fn install(
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    angle: u16,
    analog: Analog,
) {
    let state = Servo {
        _analog: analog,
        pwm,
        encoder,
        calibration,
        speed: SpeedController::new(600),
        previous: angle,
        counts: 0,
        rpm_x64: 0,
        last: Timing::micros(),
        mode: 0,
        position_lost: false,
    };
    timing::install(cortex_m::singleton!(: Servo=state).unwrap());
}
pub fn off() {
    CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
    irq_moe::<ATIM>(false);
    CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
    CURRENT_RESET.store(1, Ordering::Release);
}
impl Servo {
    pub fn tick(&mut self, now: u32) {
        let cmd = link::demand(now);
        let elapsed = now.wrapping_sub(self.last);
        // Once a gap can conceal motion the boot-relative position needs a fresh origin.
        if elapsed > 10_000 {
            self.position_lost = true;
        }
        let sample = self.encoder.read_angle();
        let mut valid = false;
        if let Ok(angle) = sample {
            let delta = angle_delta(angle, self.previous);
            let max_delta =
                MAX_SAMPLE_DELTA_COUNTS.max((elapsed.min(10_000) * 181 / 1000) as i32 + 2);
            if !self.position_lost && elapsed != 0 && delta.abs() <= max_delta {
                self.previous = angle;
                self.last = now;
                self.counts = self.counts.saturating_add(delta);
                let raw = delta * 234375 / elapsed as i32;
                self.rpm_x64 += (raw - self.rpm_x64) / 16;
                CURRENT_ELECTRICAL_ANGLE
                    .store(self.calibration.angle(angle).into(), Ordering::Relaxed);
                valid = true;
            }
        }
        let rpm = (self.rpm_x64 / 64).clamp(-32767, 32767) as i16;
        CURRENT_ANGLE_STEP.store(
            self.calibration.step_for_rpm(i32::from(rpm), PWM_HZ),
            Ordering::Relaxed,
        );
        let angle = crate::hardware::deci_degrees(self.counts);
        if let Some(c) = cmd {
            let bus = adc_irq_result::<ADC2>(0).map(bus_voltage_mv);
            let stop = if !valid {
                Some(StopReason::Encoder)
            } else if !bus.is_some_and(|v| (BUS_MIN_MV..=BUS_MAX_MV).contains(&v)) {
                Some(StopReason::BusVoltage)
            } else if rpm.unsigned_abs() > 660 {
                Some(StopReason::Overspeed)
            } else {
                None
            };
            if let Some(reason) = stop {
                record_stop(reason);
                CURRENT_FAULT.store(1, Ordering::Release);
            }
            if CURRENT_FAULT.load(Ordering::Acquire) == 0 {
                if self.mode != c.mode || CURRENT_OUTPUT_ENABLE.load(Ordering::Acquire) == 0 {
                    self.speed.reset();
                    CURRENT_RESET.store(1, Ordering::Release);
                }
                self.mode = c.mode;
                let error = c.angle - angle;
                let velocity = if c.mode == 1 {
                    (error.saturating_mul(c.pos_gain.into()) / 10)
                        .clamp(-i32::from(c.max_rpm), c.max_rpm.into())
                } else {
                    i32::from(c.rpm)
                };
                let limit = i32::from(c.max_iq);
                let iq = match c.mode {
                    1 | 2 => {
                        self.speed = self.speed.with_gains(c.speed_kp.into(), c.speed_ki.into());
                        if velocity == 0 {
                            self.speed.reset();
                            (-self.rpm_x64 * limit / (5 * 64)).clamp(-limit, limit)
                        } else {
                            i32::from(self.speed.update_relative(
                                velocity,
                                self.rpm_x64,
                                2,
                                c.max_iq,
                                |n, d| n / d,
                            ))
                        }
                    }
                    3 => i32::from(c.iq),
                    _ => crate::protocol::mit_iq(c, error, self.rpm_x64),
                }
                .clamp(-limit, limit);
                let iq = if c.mode >= 3
                    && rpm.unsigned_abs() >= c.max_rpm as u16
                    && iq.signum() == i32::from(rpm.signum())
                {
                    0
                } else {
                    iq as i16
                };
                CURRENT_IQ_REFERENCE.store(self.calibration.electrical_iq(iq), Ordering::Relaxed);
                cortex_m::interrupt::free(|_| {
                    if CURRENT_FAULT.load(Ordering::Relaxed) == 0
                        && CURRENT_OUTPUT_ENABLE.load(Ordering::Relaxed) == 0
                    {
                        CURRENT_OUTPUT_ENABLE.store(OUTPUT_PRIME, Ordering::Release);
                    }
                });
            } else {
                off();
                self.mode = 0;
            }
        } else {
            off();
            self.mode = 0;
        }
        let moe = self.pwm.get_master_output_enable();
        link::publish(Feedback {
            angle,
            rpm,
            iq: if moe {
                self.calibration
                    .electrical_iq(CURRENT_IQ.load(Ordering::Relaxed))
            } else {
                0
            },
            moe,
        });
    }
}
