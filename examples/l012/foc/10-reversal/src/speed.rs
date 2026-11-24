use crate::telemetry::{Report, publish};
use crate::{
    Analog, Bridge, calibration::Calibration, global::*, hardware::*, sensor::Encoder, timing,
};
use core::sync::atomic::Ordering;
use embassy_cw32::{self as hal, adc::adc_irq_result};
const TARGET_RPM: i32 = 600;
const HOLD_US: u32 = 1_000_000;
const OVERSPEED_TRIP_RPM: i32 = 660;
const LEG_TIMEOUT_US: u32 = 5_000_000;
const MAX_ENCODER_INTERVAL_US: u32 = 10_000;

pub(crate) struct SpeedLoop {
    _analog: Analog,
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    sample_time_us: u32,
    last_angle: u16,
    speed_feedback_rpm_x64: i32,
    target: i32,
    active: bool,
    leg_start: u32,
    hold_start: Option<u32>,
    hold_integral_q10: i32,
    hold_rpm_sum: i32,
    hold_samples: u32,
    sum_iq: i32,
    samples: u32,
    peak_bus: u32,
    zero_us: Option<u32>,
}
pub(crate) fn install(
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    initial_angle: u16,
    analog: Analog,
) {
    let state = SpeedLoop {
        _analog: analog,
        pwm,
        encoder,
        calibration,
        sample_time_us: timing::Timing::micros(),
        last_angle: initial_angle,
        speed_feedback_rpm_x64: 0,
        target: TARGET_RPM,
        active: false,
        leg_start: 0,
        hold_start: None,
        hold_integral_q10: 0,
        hold_rpm_sum: 0,
        hold_samples: 0,
        sum_iq: 0,
        samples: 0,
        peak_bus: 0,
        zero_us: Some(0),
    };
    let control = cortex_m::singleton!(: SpeedLoop = state).unwrap();
    timing::install(control);
}
impl SpeedLoop {
    pub(crate) fn tick(&mut self, now_us: u32) {
        if CURRENT_FAULT.load(Ordering::Acquire) != 0 {
            self.stop();
            return;
        }
        let bus = adc_irq_result::<hal::peripherals::ADC2>(0).unwrap_or(0);
        let bus_mv = bus_voltage_mv(bus);
        if !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv) {
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            CURRENT_FAULT.store(1, Ordering::Relaxed);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            publish(Report::BusFault { bus_mv });
            return;
        }
        let sample = self.encoder.read_angle();
        let elapsed_us = now_us.wrapping_sub(self.sample_time_us);
        self.sample_time_us = now_us;
        let mechanical_angle = sample.unwrap_or(self.last_angle);
        let delta = angle_delta(mechanical_angle, self.last_angle);
        // At the trip speed a valid displacement grows with the sample interval.
        // Keep the original single-frame tolerance for short intervals.
        let limit = MAX_SAMPLE_DELTA_COUNTS.max(
            (OVERSPEED_TRIP_RPM as u32 * u32::from(ENCODER_COUNTS_PER_REV) / 60
                * elapsed_us.min(MAX_ENCODER_INTERVAL_US))
            .div_ceil(1_000_000) as i32
                + 2,
        );
        let timing_fault = elapsed_us == 0 || elapsed_us > MAX_ENCODER_INTERVAL_US;
        if sample.is_err() || (self.active && (timing_fault || delta.abs() > limit)) {
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_FAULT.store(1, Ordering::Release);
            if let Err(error) = sample {
                publish(Report::TransferFault { error, elapsed_us });
            } else if timing_fault {
                publish(Report::IntervalFault { elapsed_us });
            } else {
                publish(Report::JumpFault {
                    previous: self.last_angle,
                    sample: mechanical_angle,
                    delta,
                    limit,
                    elapsed_us,
                });
            }
            return;
        }
        let electrical_angle = self.calibration.angle(mechanical_angle);
        if sample.is_ok() {
            CURRENT_ELECTRICAL_ANGLE.store(u32::from(electrical_angle), Ordering::Relaxed);
        }
        self.last_angle = mechanical_angle;
        let raw_rpm_x64 = if self.active {
            delta * 234_375 / elapsed_us as i32
        } else {
            0
        };
        // A short filter rejects single-count jitter with little reversal delay.
        const FILTER_SAMPLES: i32 = (2_000 / SPEED_PERIOD_US) as i32;
        let difference = raw_rpm_x64 - self.speed_feedback_rpm_x64;
        self.speed_feedback_rpm_x64 +=
            (difference + difference.signum() * (FILTER_SAMPLES / 2)) / FILTER_SAMPLES;
        let filtered = self.speed_feedback_rpm_x64;
        let feedback_rpm = (filtered + filtered.signum() * 32) / 64;
        CURRENT_ANGLE_STEP.store(
            self.calibration.step_for_rpm(feedback_rpm, PWM_HZ),
            Ordering::Relaxed,
        );
        let direction = self.target.signum();
        if !self.active {
            self.leg_start = now_us;
        }
        let leg_us = now_us.wrapping_sub(self.leg_start);
        if feedback_rpm.abs() >= OVERSPEED_TRIP_RPM
            || (self.hold_start.is_none() && leg_us >= LEG_TIMEOUT_US)
        {
            publish(Report::MotionFault {
                target_rpm: self.target,
                feedback_rpm,
                iq: CURRENT_IQ_REFERENCE.load(Ordering::Relaxed),
            });
            self.stop();
            return;
        }
        if self.hold_start.is_none() {
            self.sum_iq += i32::from(CURRENT_IQ.load(Ordering::Relaxed));
            self.samples += 1;
            self.peak_bus = self.peak_bus.max(bus_mv);
            if self.zero_us.is_none() && direction * filtered >= 0 {
                self.zero_us = Some(leg_us);
            }
            if direction * filtered >= TARGET_RPM * 64 {
                self.hold_start = Some(now_us);
                self.hold_integral_q10 = 0;
                publish(Report::Leg {
                    reached: feedback_rpm,
                    elapsed_us: leg_us,
                    zero_us: self.zero_us.unwrap_or(0),
                    bus_mv: self.peak_bus,
                    avg_iq: self.sum_iq / self.samples as i32,
                });
            }
        }
        let mechanical_iq = if let Some(start) = self.hold_start {
            self.hold_rpm_sum += feedback_rpm;
            self.hold_samples += 1;
            let held_us = now_us.wrapping_sub(start);
            if held_us >= HOLD_US {
                publish(Report::Hold {
                    target: self.target,
                    held_us,
                    mean_rpm: self.hold_rpm_sum / self.hold_samples as i32,
                });
                self.hold_start = None;
                self.hold_rpm_sum = 0;
                self.hold_samples = 0;
                self.sum_iq = 0;
                self.samples = 0;
                self.peak_bus = 0;
                self.target = -self.target;
                self.leg_start = now_us;
                self.zero_us = None;
                self.target.signum() as i16 * IQ_LIMIT_COUNTS
            } else {
                // Retain load torque when encoder ripple crosses the setpoint.
                let error = self.target - feedback_rpm;
                let limit = i32::from(IQ_LIMIT_COUNTS);
                self.hold_integral_q10 =
                    (self.hold_integral_q10 + error * 16).clamp(-limit * 1024, limit * 1024);
                (error + self.hold_integral_q10 / 1024).clamp(-limit, limit) as i16
            }
        } else {
            direction as i16 * IQ_LIMIT_COUNTS
        };
        let iq = self.calibration.electrical_iq(mechanical_iq);
        CURRENT_IQ_REFERENCE.store(iq, Ordering::Release);
        if !self.active {
            CURRENT_RESET.store(1, Ordering::Release);
            cortex_m::interrupt::free(|_| {
                if CURRENT_FAULT.load(Ordering::Relaxed) == 0 {
                    CURRENT_OUTPUT_ENABLE.store(OUTPUT_PRIME, Ordering::Release);
                }
            });
            self.active = true;
        }
    }

    fn stop(&mut self) {
        CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
        self.pwm.set_master_output_enable(false);
        CURRENT_IQ_REFERENCE.store(0, Ordering::Release);
        CURRENT_FAULT.store(1, Ordering::Release);
    }
}
