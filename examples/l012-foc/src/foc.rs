//! Small, allocation-free FOC kernel used by the CW32L012 example.
//!
//! The kernel deliberately contains no PAC or HAL calls.  This keeps the
//! control law testable on a host and makes the hardware boundary explicit:
//! the application supplies calibrated currents and the electrical angle, and
//! receives three normalized PWM duties.

const Q15_ONE: i32 = 32_767;
const SQRT3_OVER_2_Q15: i32 = 28_346;
const INV_SQRT3_Q15: i32 = 18_918;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhaseCurrent {
    pub a: i16,
    pub b: i16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Duty {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorParameters {
    pub pole_pairs: u16,
    pub phase_resistance_mohm: u32,
    pub phase_inductance_uh: u32,
    pub rated_voltage_mv: u32,
    pub rated_current_ma: u32,
    pub max_current_ma: u32,
    pub encoder_counts_per_rev: u16,
}

/// JD-Power 4621-class motor from `JD-4621.jpg` (24N22P / 22 pole pairs).
///
/// The image identifies the encoder as 5048A, while the supplied reference
/// project uses an MT6701 SSI encoder.  The electrical interface is therefore
/// kept in the board layer and this parameter set only describes the motor.
pub const JD4631: MotorParameters = MotorParameters {
    pole_pairs: 22,
    phase_resistance_mohm: 6_700,
    phase_inductance_uh: 1_750,
    rated_voltage_mv: 12_000,
    rated_current_ma: 1_100,
    max_current_ma: 2_000,
    encoder_counts_per_rev: 16_384,
};

#[derive(Clone, Copy, Debug)]
struct Pi {
    kp_q15: i32,
    ki_q15: i32,
    integral: i32,
    limit: i32,
}

impl Pi {
    const fn new(kp_q15: i32, ki_q15: i32, limit: i32) -> Self {
        Self {
            kp_q15,
            ki_q15,
            integral: 0,
            limit,
        }
    }

    fn reset(&mut self) {
        self.integral = 0;
    }

    fn update(&mut self, reference: i32, feedback: i32) -> i32 {
        let error = reference - feedback;
        let next = self.integral + ((error * self.ki_q15) >> 15);
        self.integral = next.clamp(-self.limit, self.limit);
        (((error * self.kp_q15) >> 15) + self.integral).clamp(-self.limit, self.limit)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FocController {
    pub motor: MotorParameters,
    id_pi: Pi,
    iq_pi: Pi,
    pub id_reference: i16,
    pub iq_reference: i16,
    pub voltage_limit: i16,
}

/// Outer speed loop. The command and feedback are in rpm; the result is the
/// Q1.15 torque-current reference consumed by [`FocController`].
#[derive(Clone, Copy, Debug)]
pub struct SpeedController {
    max_speed_rpm: i32,
    kp_q15: i32,
    ki_q15: i32,
    max_iq_q15: i32,
    integral_q15: i32,
}

impl SpeedController {
    pub const fn new(max_speed_rpm: i32, max_iq_q15: i16) -> Self {
        Self {
            max_speed_rpm,
            kp_q15: 20_000,
            ki_q15: 700,
            max_iq_q15: max_iq_q15 as i32,
            integral_q15: 0,
        }
    }

    pub fn reset(&mut self) {
        self.integral_q15 = 0;
    }

    pub fn update(&mut self, target_rpm: i32, feedback_rpm: i32) -> i16 {
        let error = (target_rpm - feedback_rpm).clamp(-self.max_speed_rpm, self.max_speed_rpm);
        let proportional = error * self.kp_q15 / self.max_speed_rpm;
        self.integral_q15 += error * self.ki_q15 / self.max_speed_rpm;
        self.integral_q15 = self.integral_q15.clamp(-self.max_iq_q15, self.max_iq_q15);
        (proportional + self.integral_q15).clamp(-self.max_iq_q15, self.max_iq_q15) as i16
    }
}

impl FocController {
    pub const fn new(motor: MotorParameters) -> Self {
        // Gains are conservative Q1.15 starting values.  They are intentionally
        // exposed here rather than hidden in a board-specific ISR so they can be
        // tuned against the actual current amplifier and bus voltage.
        Self {
            motor,
            id_pi: Pi::new(7_000, 180, 28_000),
            iq_pi: Pi::new(7_000, 180, 28_000),
            id_reference: 0,
            iq_reference: 0,
            voltage_limit: 27_000,
        }
    }

    pub fn reset(&mut self) {
        self.id_pi.reset();
        self.iq_pi.reset();
    }

    /// One current-loop update. `sin_q15` and `cos_q15` describe the rotor
    /// electrical angle; currents and references use the same signed Q1.15
    /// current scale.  The result is normalized to the PWM period.
    pub fn update(&mut self, currents: PhaseCurrent, sin_q15: i16, cos_q15: i16) -> Duty {
        let ia = i32::from(currents.a);
        let ib = i32::from(currents.b);
        let alpha = ia;
        let beta = ((ia + 2 * ib) * INV_SQRT3_Q15) >> 15;

        let sin = i32::from(sin_q15);
        let cos = i32::from(cos_q15);
        let id = ((alpha * cos) + (beta * sin)) >> 15;
        let iq = ((-alpha * sin) + (beta * cos)) >> 15;

        let vd = self.id_pi.update(i32::from(self.id_reference), id);
        let vq = self.iq_pi.update(i32::from(self.iq_reference), iq);
        let magnitude_sq = i64::from(vd) * i64::from(vd) + i64::from(vq) * i64::from(vq);
        let limit_sq = i64::from(self.voltage_limit) * i64::from(self.voltage_limit);
        let (vd, vq) = if magnitude_sq > limit_sq {
            // Keep both axes bounded without a divide on the normal path.  This
            // conservative clamp keeps the result bounded on a saturated M0+.
            let limit = i32::from(self.voltage_limit);
            (vd.clamp(-limit, limit), vq.clamp(-limit, limit))
        } else {
            (vd, vq)
        };

        let v_alpha = ((vd * cos) - (vq * sin)) >> 15;
        let v_beta = ((vd * sin) + (vq * cos)) >> 15;
        let v_a = v_alpha;
        let v_b = ((-v_alpha * Q15_ONE) + (v_beta * SQRT3_OVER_2_Q15)) >> 16;
        let v_c = ((-v_alpha * Q15_ONE) - (v_beta * SQRT3_OVER_2_Q15)) >> 16;
        let max = v_a.max(v_b).max(v_c);
        let min = v_a.min(v_b).min(v_c);
        let common = (max + min) / 2;

        Duty {
            a: duty_from_q15(v_a - common),
            b: duty_from_q15(v_b - common),
            c: duty_from_q15(v_c - common),
        }
    }
}

fn duty_from_q15(value: i32) -> u16 {
    // The centered phase value is in [-Q15_ONE, Q15_ONE].  A half-bus phase
    // maps to 0..100% duty; clamp protects against arithmetic or sensor faults.
    let duty = 16_384 + (value / 2);
    duty.clamp(1, 32_766) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_current_and_zero_voltage_are_centered() {
        let mut controller = FocController::new(JD4631);
        let duty = controller.update(PhaseCurrent::default(), 0, Q15_ONE as i16);
        assert_eq!(
            duty,
            Duty {
                a: 16_384,
                b: 16_384,
                c: 16_384
            }
        );
    }

    #[test]
    fn output_stays_inside_pwm_endpoints() {
        let mut controller = FocController::new(JD4631);
        controller.iq_reference = 32_767;
        let duty = controller.update(PhaseCurrent::default(), Q15_ONE as i16, 0);
        assert!(
            [duty.a, duty.b, duty.c]
                .iter()
                .all(|&x| (1..32_767).contains(&x))
        );
    }

    #[test]
    fn jd4631_uses_the_22_pole_pair_motor() {
        assert_eq!(JD4631.pole_pairs, 22);
        assert_eq!(JD4631.phase_resistance_mohm, 6_700);
    }

    #[test]
    fn speed_controller_zero_command_removes_torque_reference() {
        let mut speed = SpeedController::new(800, 18_000);
        assert!(speed.update(400, 0) > 0);
        speed.reset();
        assert_eq!(speed.update(0, 0), 0);
    }
}
