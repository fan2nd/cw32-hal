use crate::telemetry::{Report, publish};
use crate::{
    Analog, Bridge,
    calibration::Calibration,
    control::{combined_torque_reference, holding_torque_reference, target_rpm_from_pot},
    foc::SpeedController,
    global::*,
    hardware::*,
    sensor::Encoder,
    timing,
};
use core::sync::atomic::Ordering;
use embassy_cw32::{self as hal, adc::adc_irq_result};
const NO_MOTION_TIMEOUT_US: u32 = 2_000_000;
const RAMP_MILLIRPM_PER_US: i32 = 2; // Smooth command slew: 2 rpm per elapsed ms.
// Same commissioning current range as 02, in calibrated ADC counts.
const IQ_LIMIT_COUNTS: i16 = 200;
const MAX_ENCODER_INTERVAL_US: u32 = 10_000;
const REVERSE_TRIP_RPM: i32 = 20;
const OVERSPEED_TRIP_RPM: i32 = POT_MAX_RPM + 60;

pub(crate) struct SpeedLoop {
    _analog: Analog,
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    speed: SpeedController,
    sample_time_us: u32,
    last_angle: u16,
    speed_feedback_rpm_x64: i32,
    log_elapsed_us: u32,
    command_active: bool,
    no_motion_us: u32,
    speed_reference_millirpm: i32,
    command_rpm_latched: i32,
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
        speed: SpeedController::new(POT_MAX_RPM),
        sample_time_us: timing::Timing::micros(),
        last_angle: initial_angle,
        speed_feedback_rpm_x64: 0,
        log_elapsed_us: 0,
        command_active: false,
        no_motion_us: 0,
        speed_reference_millirpm: 0,
        command_rpm_latched: 0,
    };
    let control = cortex_m::singleton!(: SpeedLoop = state).unwrap();
    timing::install(control);
}
impl SpeedLoop {
    pub(crate) fn tick(&mut self, now_us: u32) {
        let pot_sample = adc_irq_result::<hal::peripherals::ADC2>(1);
        let pot_now = pot_sample.unwrap_or(0);
        if pot_sample.is_none() {
            CURRENT_FAULT.store(1, Ordering::Release);
        }
        if pot_sample.is_some() && pot_now <= POT_ZERO_COUNTS {
            // Zero is also the operator's fault-clear command. Drop the
            // bridge immediately and clear both PI states so the next command
            // starts from a known condition.
            let was_active = self.command_active || CURRENT_FAULT.load(Ordering::Relaxed) != 0;
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            self.speed.reset();
            self.speed_feedback_rpm_x64 = 0;
            self.speed_reference_millirpm = 0;
            self.command_rpm_latched = 0;
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_FAULT.store(0, Ordering::Relaxed);
            CURRENT_RESET.store(1, Ordering::Release);
            self.command_active = false;
            self.no_motion_us = 0;
            if was_active {
                publish(Report::Stopped);
            }
        } else if CURRENT_FAULT.load(Ordering::Relaxed) != 0 {
            // Hold the bridge off until the operator returns the pot to zero.
            // This prevents the speed integrator from continuing to request
            // torque after a current fault.
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            return;
        }
        let bus = adc_irq_result::<hal::peripherals::ADC2>(0).unwrap_or(0);
        let bus_mv = bus_voltage_mv(bus);
        if pot_now > POT_ZERO_COUNTS && !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv) {
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
        if pot_now > POT_ZERO_COUNTS
            && (sample.is_err() || (self.command_active && (timing_fault || delta.abs() > limit)))
        {
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
        let control_us = if self.command_active {
            elapsed_us
        } else {
            SPEED_PERIOD_US
        };
        // At 3 rpm a 500 us sample advances only 0 or 1 encoder count.
        // Keep fractional rpm through the filter and round only at the PI input.
        let raw_rpm_x64 = if self.command_active {
            delta * 234_375 / elapsed_us as i32
        } else {
            0
        };
        // Keep about 8 ms of smoothing at the configured sample rate.
        const FILTER_SAMPLES: i32 = (8_000 / SPEED_PERIOD_US) as i32;
        let difference = raw_rpm_x64 - self.speed_feedback_rpm_x64;
        self.speed_feedback_rpm_x64 +=
            (difference + difference.signum() * (FILTER_SAMPLES / 2)) / FILTER_SAMPLES;
        let filtered = self.speed_feedback_rpm_x64;
        let feedback_rpm = (filtered + filtered.signum() * 32) / 64;
        CURRENT_ANGLE_STEP.store(
            self.calibration.step_for_rpm(feedback_rpm, PWM_HZ),
            Ordering::Relaxed,
        );
        let pot = pot_now;
        let mapped_command_rpm = target_rpm_from_pot(
            pot,
            POT_ZERO_COUNTS,
            POT_MIN_COUNTS,
            POT_MAX_COUNTS,
            POT_MIN_RPM,
            POT_MAX_RPM,
        );
        let command_rpm = if pot <= POT_ZERO_COUNTS {
            self.command_rpm_latched = 0;
            0
        } else if pot <= POT_MIN_COUNTS {
            // Keep the previous nonzero command in the C deadband. A
            // genuine stop still requires returning to POT_ZERO_COUNTS.
            self.command_rpm_latched
        } else {
            self.command_rpm_latched = mapped_command_rpm;
            mapped_command_rpm
        };
        // Keep the speed reference equal to the operator command. The
        // static-friction feed-forward below supplies starting torque;
        // raising a 30 rpm command to an artificial 120 rpm kick makes
        // the motor overshoot before the speed loop can take control.
        let ramp_step = RAMP_MILLIRPM_PER_US * control_us as i32;
        let distance = command_rpm * 1000 - self.speed_reference_millirpm;
        if command_rpm == 0 {
            self.speed_reference_millirpm = 0;
        } else {
            self.speed_reference_millirpm += distance.clamp(-ramp_step, ramp_step);
        }
        let target_rpm = (self.speed_reference_millirpm + 500) / 1000;
        // Keep the bridge off throughout the complete command deadband.
        // Previously pot=151..200 entered the current loop with a zero
        // speed reference, so ADC offset/noise was converted into holding
        // current even though the operator had not requested motion.
        if target_rpm == 0 {
            // A zero command is an explicit stop request. Clear the speed
            // integrator so a previous positive command cannot restart the
            // motor while the pot is still at zero, and remove MOE so the
            // current loop cannot turn ADC noise into holding torque.
            self.speed.reset();
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            if self.command_active {
                publish(Report::Deadband { pot });
            }
            self.command_active = false;
            self.no_motion_us = 0;
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
        } else {
            if filtered >= (target_rpm * 32).min(5 * 64) {
                self.no_motion_us = 0;
            } else {
                self.no_motion_us = self.no_motion_us.saturating_add(control_us);
            }
            if feedback_rpm <= -REVERSE_TRIP_RPM
                || feedback_rpm >= OVERSPEED_TRIP_RPM
                || self.no_motion_us >= NO_MOTION_TIMEOUT_US
            {
                let timeout_iq_reference = CURRENT_IQ_REFERENCE.load(Ordering::Relaxed);
                CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
                // Latch the startup failure with the bridge off.  Keep
                // the foreground state machine alive so returning the
                // potentiometer to zero can clear the fault and retry.
                CURRENT_FAULT.store(1, Ordering::Release);
                self.command_active = false;
                self.no_motion_us = 0;
                CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
                self.pwm.set_master_output_enable(false);
                publish(Report::MotionFault {
                    target_rpm,
                    feedback_rpm,
                    iq: timeout_iq_reference,
                });
                return;
            }
            let speed_iq_reference = self.speed.update_for_quarter_ms_in_range(
                target_rpm,
                feedback_rpm,
                SPEED_PERIOD_US / 250,
                -IQ_LIMIT_COUNTS,
                IQ_LIMIT_COUNTS,
            );
            let holding_iq_reference = holding_torque_reference(target_rpm, POT_MAX_RPM);
            let iq_reference = combined_torque_reference(
                speed_iq_reference,
                holding_iq_reference,
                target_rpm,
                feedback_rpm,
            )
            .clamp(-IQ_LIMIT_COUNTS, IQ_LIMIT_COUNTS);
            CURRENT_IQ_REFERENCE.store(
                self.calibration.electrical_iq(iq_reference),
                Ordering::Relaxed,
            );
            if !self.command_active {
                CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
                self.pwm.set_master_output_enable(false);
                CURRENT_ALIGNMENT_ACTIVE.store(0, Ordering::Release);
                CURRENT_RESET.store(1, Ordering::Release);
                publish(Report::Started { pot, target_rpm });
                self.command_active = true;
                cortex_m::interrupt::free(|_| {
                    if CURRENT_FAULT.load(Ordering::Relaxed) == 0 {
                        CURRENT_OUTPUT_ENABLE.store(OUTPUT_PRIME, Ordering::Release);
                    }
                });
            }
        }
        self.log_elapsed_us =
            self.log_elapsed_us
                .saturating_add(if self.command_active { elapsed_us } else { 0 });
        if self.log_elapsed_us >= 1_000_000 {
            publish(Report::Running {
                pot,
                target_rpm,
                feedback_rpm,
                iqref: CURRENT_IQ_REFERENCE.load(Ordering::Relaxed),
                id: CURRENT_ID.load(Ordering::Relaxed),
                iq: CURRENT_IQ.load(Ordering::Relaxed),
                mechanical_angle,
                electrical_angle,
                applied_angle: CURRENT_APPLIED_ANGLE.load(Ordering::Relaxed),
                adc_a: CURRENT_ADC_A.load(Ordering::Relaxed),
                adc_b: CURRENT_ADC_B.load(Ordering::Relaxed),
                offset_a: CURRENT_OFFSET_A.load(Ordering::Relaxed),
                offset_b: CURRENT_OFFSET_B.load(Ordering::Relaxed),
                bus_mv,
            });
            CURRENT_LOOP_COUNT.store(0, Ordering::Relaxed);
            self.log_elapsed_us = 0;
        }
    }
}
