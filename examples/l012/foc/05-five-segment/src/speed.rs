use crate::telemetry::{Report, publish};
use crate::{
    Analog, Bridge,
    calibration::Calibration,
    control::{StopGate, target_rpm_from_pot},
    foc::SpeedController,
    global::*,
    hardware::*,
    sensor::Encoder,
    timing,
};
use core::sync::atomic::Ordering;
use embassy_cw32::{self as hal, adc::adc_irq_result, eau::Eau, peripherals::EAU};
const NO_MOTION_TIMEOUT_US: u32 = 2_000_000;
const MAX_ENCODER_INTERVAL_US: u32 = 10_000;
const REVERSE_TRIP_RPM: i32 = 20;
const OVERSPEED_TRIP_RPM: i32 = POT_MAX_RPM + 60;

pub(crate) struct SpeedLoop {
    _analog: Analog,
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    speed: SpeedController,
    eau: Option<Eau<EAU>>,
    sample_time_us: u32,
    last_angle: u16,
    speed_feedback_rpm_x64: i32,
    log_elapsed_us: u32,
    stop_anchor: u16,
    quiet_us: u32,
    command_active: bool,
    no_motion_us: u32,
    command_rpm_latched: i32,
    stop_gate: StopGate,
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
        eau: Eau::new().ok(),
        speed: SpeedController::new(POT_MAX_RPM).with_gains(125, 6),
        sample_time_us: timing::Timing::micros(),
        last_angle: initial_angle,
        speed_feedback_rpm_x64: 0,
        log_elapsed_us: 0,
        stop_anchor: initial_angle,
        quiet_us: 0,
        command_active: false,
        no_motion_us: 0,
        command_rpm_latched: 0,
        stop_gate: StopGate::default(),
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
        if CURRENT_FAULT.load(Ordering::Acquire) != 0 {
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            self.command_active = false;
            if pot_sample.is_some() && pot_now <= POT_ZERO_COUNTS {
                self.speed.reset();
                self.speed_feedback_rpm_x64 = 0;
                self.command_rpm_latched = 0;
                self.stop_gate = StopGate::default();
                self.no_motion_us = 0;
                CURRENT_RESET.store(1, Ordering::Release);
                CURRENT_FAULT.store(0, Ordering::Release);
                publish(Report::Stopped);
            }
            return;
        }
        // A zero command can still be actively braking.
        let check_drive = self.command_active || pot_now > POT_ZERO_COUNTS;
        let bus = adc_irq_result::<hal::peripherals::ADC2>(0).unwrap_or(0);
        let bus_mv = bus_voltage_mv(bus);
        if check_drive && !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv) {
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
        if check_drive
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
        // Keep fractional rpm through the filter and round only for integer-rpm consumers.
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
        if pot <= POT_ZERO_COUNTS {
            self.command_rpm_latched = 0;
        } else if pot > POT_MIN_COUNTS {
            self.command_rpm_latched = target_rpm_from_pot(
                pot, POT_ZERO_COUNTS, POT_MIN_COUNTS, POT_MAX_COUNTS, 0, POT_MAX_RPM,
            );
        }
        let target_rpm = self.stop_gate.command(
            self.command_rpm_latched, self.command_active, pot <= POT_ZERO_COUNTS, control_us,
        );
        // Require a quiet encoder window as well as low speed in either direction.
        if !self.command_active
            || target_rpm != 0
            || filtered.abs() > 64
            || angle_delta(mechanical_angle, self.stop_anchor).abs() > 2
        {
            self.stop_anchor = mechanical_angle;
            self.quiet_us = 0;
        } else {
            self.quiet_us += elapsed_us;
        }
        if target_rpm == 0 && (!self.command_active || self.quiet_us >= 20_000) {
            self.speed.reset();
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            self.speed_feedback_rpm_x64 = 0;
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            if self.command_active {
                publish(Report::Deadband {
                    pot,
                    moe: self.pwm.get_master_output_enable(),
                });
            }
            self.command_active = false;
            self.no_motion_us = 0;
        } else {
            if target_rpm == 0 || filtered >= (target_rpm * 32).min(5 * 64) {
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
            let iq_reference = if target_rpm == 0 {
                // Taper braking near rest so feedback noise cannot demand full reverse torque.
                self.speed.reset();
                (-filtered.min(raw_rpm_x64).max(0) * i32::from(IQ_LIMIT_COUNTS) / (5 * 64))
                    .clamp(-i32::from(IQ_LIMIT_COUNTS), i32::from(IQ_LIMIT_COUNTS))
                    as i16
            } else {
                self.speed.update_relative(
                    target_rpm,
                    filtered,
                    SPEED_PERIOD_US / 250,
                    IQ_LIMIT_COUNTS,
                    |numerator, denominator| {
                        if let Some(eau) = self.eau.as_mut() {
                            if let Ok(result) = eau.divide_signed(numerator, denominator, 32) {
                                return result.quotient;
                            }
                        }
                        numerator / denominator
                    },
                )
            };
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
                moe: self.pwm.get_master_output_enable(),
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
