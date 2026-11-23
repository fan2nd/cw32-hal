use crate::telemetry::{Report, publish};
use crate::{
    Analog, Bridge,
    calibration::Calibration,
    config::*,
    global::*,
    hardware::{MAX_SAMPLE_DELTA_COUNTS, angle_delta, bus_voltage_mv},
    position_control::PositionControl,
    sensor::Encoder,
    timing,
};
use core::sync::atomic::Ordering;
use embassy_cw32::{adc::adc_irq_result, peripherals::ADC2};
pub(crate) struct PositionLoop {
    _analog: Analog,
    command: i32,
    control: PositionControl,
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    previous: u16,
    last: u32,
    log_time: u32,
    active: bool,
    rpm_x8: i32,
}

pub(crate) fn install(
    pwm: Bridge,
    encoder: Encoder<'static>,
    calibration: Calibration,
    angle: u16,
    analog: Analog,
) {
    let now = timing::Timing::micros();
    let state = PositionLoop {
        _analog: analog,
        pwm,
        encoder,
        calibration,
        previous: angle,
        last: now,
        log_time: now,
        active: false,
        rpm_x8: 0,
        command: 0,
        control: PositionControl::default(),
    };
    let control = cortex_m::singleton!(: PositionLoop = state).unwrap();
    timing::install(control);
}
impl PositionLoop {
    pub(crate) fn tick(&mut self, now: u32) {
        let us = now.wrapping_sub(self.last);
        self.last = now;
        let zero = self.calibration.mechanical_zero;
        let sample = self.encoder.read_angle();
        let pot = adc_irq_result::<ADC2>(1);
        let bus = adc_irq_result::<ADC2>(0).map(bus_voltage_mv);
        let mut delta = 0;
        if let Ok(angle) = sample {
            delta = angle_delta(angle, self.previous);
            self.previous = angle;
        }
        let stopped = pot.is_some_and(|p| p <= POT_ZERO_COUNTS);
        let valid = sample.is_ok()
            && pot.is_some()
            && bus.is_some_and(|v| (BUS_MIN_MV..=BUS_MAX_MV).contains(&v))
            && (!self.active || (delta.abs() <= MAX_SAMPLE_DELTA_COUNTS && us <= 10_000));
        if !stopped && !valid {
            CURRENT_FAULT.store(1, Ordering::Release);
        }
        if stopped || CURRENT_FAULT.load(Ordering::Acquire) != 0 {
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            self.pwm.set_master_output_enable(false);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_RESET.store(1, Ordering::Release);
            if self.active && !stopped {
                publish(Report::Fault);
            }
            if stopped {
                CURRENT_FAULT.store(0, Ordering::Release);
            }
            self.active = false;
            self.rpm_x8 = 0;
            return;
        }
        if !self.active && pot.unwrap() <= POT_MIN_COUNTS {
            return;
        }
        // 60e6 / 16384 = 234375 / 64. Delta is bounded while active.
        if self.active {
            let measured_x8 = delta * 234_375 / (8 * us.max(1) as i32);
            // Weight the latest measurement by 1/2. Retain fractional rpm for damping near rest.
            self.rpm_x8 = (self.rpm_x8 + measured_x8) / 2;
        }
        let command =
            i32::from(pot.unwrap().clamp(POT_MIN_COUNTS, POT_MAX_COUNTS) - POT_MIN_COUNTS);
        let span = i32::from(POT_MAX_COUNTS - POT_MIN_COUNTS);
        let turn = i32::from(ENCODER_COUNTS_PER_REV);
        if !self.active || (command - self.command).abs() >= 4 || command == 0 || command == span {
            self.command = command;
        }
        let target = self.command * turn / span;
        let measured = angle_delta(self.previous, zero).rem_euclid(turn);
        let error = angle_delta(target.rem_euclid(turn) as u16, measured as u16);
        let elapsed_ms = if self.active { (us / 1000).max(1) } else { 0 };
        let (iq, reference) = self.control.update(error, self.rpm_x8, elapsed_ms);
        let request = (!self.control.settled()).then_some(iq);
        CURRENT_ELECTRICAL_ANGLE.store(
            u32::from(self.calibration.angle(self.previous)),
            Ordering::Relaxed,
        );
        CURRENT_ANGLE_STEP.store(
            self.calibration.step_for_rpm(self.rpm_x8 / 8, PWM_HZ),
            Ordering::Relaxed,
        );
        let iq = self.calibration.electrical_iq(request.unwrap_or(0));
        CURRENT_IQ_REFERENCE.store(iq, Ordering::Relaxed);
        cortex_m::interrupt::free(|_| {
            if request.is_none() {
                CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
                self.pwm.set_master_output_enable(false);
                CURRENT_RESET.store(1, Ordering::Release);
            } else if CURRENT_FAULT.load(Ordering::Relaxed) == 0 {
                CURRENT_OUTPUT_ENABLE.store(1, Ordering::Release);
                self.pwm.set_master_output_enable(true);
            }
        });
        self.active = true;
        if now.wrapping_sub(self.log_time) >= 250_000 {
            self.log_time = now;
            publish(Report::Running {
                target,
                measured,
                error,
                reference,
                settled: self.control.settled(),
                rpm: self.rpm_x8 / 8,
                iqref: iq,
                id: CURRENT_ID.load(Ordering::Relaxed),
                iq: CURRENT_IQ.load(Ordering::Relaxed),
            });
        }
    }
}
