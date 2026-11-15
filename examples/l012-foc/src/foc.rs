//! Small, allocation-free FOC kernel used by the CW32L012 example.
//!
//! The kernel deliberately contains no PAC or HAL calls.  This keeps the
//! control law testable on a host and makes the hardware boundary explicit:
//! the application supplies calibrated currents and the electrical angle, and
//! receives three normalized PWM duties.

const Q15_ONE: i32 = 32_767;
const HALF_Q15: i32 = 16_384;
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

/// JD-Power 4621-class motor from `JD-4621.jpg` (24N22P = 22 poles = 11
/// pole pairs).
///
/// The image identifies the encoder as 5048A, while the supplied reference
/// project uses an MT6701 SSI encoder.  The electrical interface is therefore
/// kept in the board layer and this parameter set only describes the motor.
pub const JD4631: MotorParameters = MotorParameters {
    pole_pairs: 11,
    phase_resistance_mohm: 6_700,
    phase_inductance_uh: 1_750,
    rated_voltage_mv: 12_000,
    rated_current_ma: 1_100,
    max_current_ma: 2_000,
    encoder_counts_per_rev: 16_384,
};

#[derive(Clone, Copy, Debug)]
struct Pi {
    kp_gain: i32,
    kp_shift: u8,
    ki_gain: i32,
    integral: i32,
    limit: i32,
}

impl Pi {
    const fn new(kp_gain: i32, kp_shift: u8, ki_gain: i32, limit: i32) -> Self {
        Self {
            kp_gain,
            kp_shift,
            ki_gain,
            integral: 0,
            limit,
        }
    }

    fn reset(&mut self) {
        self.integral = 0;
    }

    fn update(&mut self, reference: i32, feedback: i32) -> i32 {
        let error = reference - feedback;
        // C stores the integral before the Q15 shift. Retain that accumulator
        // in i32; with the configured limits `limit << 15` remains bounded.
        let integral_limit = self.limit.saturating_mul(1 << 15);
        self.integral = self
            .integral
            .saturating_add(error.saturating_mul(self.ki_gain))
            .clamp(-integral_limit, integral_limit);
        let proportional = error.saturating_mul(self.kp_gain) >> self.kp_shift;
        (proportional + (self.integral >> 15)).clamp(-self.limit, self.limit)
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
    pub measured_id: i16,
    pub measured_iq: i16,
}

/// Outer speed loop. The command and feedback are in rpm; the result is the
/// Q1.15 torque-current reference consumed by [`FocController`].
#[derive(Clone, Copy, Debug)]
pub struct SpeedController {
    max_speed_rpm: i32,
    kp_gain: i32,
    kp_shift: u8,
    ki_gain: i32,
    ki_shift: u8,
    integral_q15: i32,
}

impl SpeedController {
    pub const fn new(max_speed_rpm: i32) -> Self {
        Self {
            max_speed_rpm,
            // Match the C reference exactly: Kp=100/2^6 and Ki=5/2^8.
            // The integral is intentionally allowed to build while the rotor
            // is stopped; that is the startup torque path in the reference
            // controller and prevents a low-speed command from only rocking
            // the rotor with a small proportional request.
            kp_gain: 100,
            kp_shift: 6,
            ki_gain: 5,
            ki_shift: 8,
            integral_q15: 0,
        }
    }

    pub fn reset(&mut self) {
        self.integral_q15 = 0;
    }

    pub fn update(&mut self, target_rpm: i32, feedback_rpm: i32) -> i16 {
        let error = (target_rpm - feedback_rpm).clamp(-self.max_speed_rpm, self.max_speed_rpm);
        let proportional = error.saturating_mul(self.kp_gain) >> self.kp_shift;
        self.integral_q15 = self
            .integral_q15
            .saturating_add(error.saturating_mul(self.ki_gain) >> self.ki_shift)
            .clamp(i32::from(i16::MIN), i32::from(i16::MAX));
        // There is no application-level iqref ceiling. The signed Q15
        // transport type is the only numeric boundary before the current PI.
        (proportional + self.integral_q15).clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16
    }

    pub fn update_for_ticks(&mut self, target_rpm: i32, feedback_rpm: i32, elapsed_ms: u32) -> i16 {
        let error = (target_rpm - feedback_rpm).clamp(-self.max_speed_rpm, self.max_speed_rpm);
        let proportional = error.saturating_mul(self.kp_gain) >> self.kp_shift;
        let elapsed = elapsed_ms.clamp(1, 100) as i32;
        let integral_delta =
            error.saturating_mul(self.ki_gain).saturating_mul(elapsed) >> self.ki_shift;
        self.integral_q15 = self
            .integral_q15
            .saturating_add(integral_delta)
            .clamp(i32::from(i16::MIN), i32::from(i16::MAX));
        (proportional + self.integral_q15).clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16
    }
}

impl FocController {
    pub const fn new(motor: MotorParameters) -> Self {
        // Gains are conservative Q1.15 starting values.  They are intentionally
        // exposed here rather than hidden in a board-specific ISR so they can be
        // tuned against the actual current amplifier and bus voltage.
        Self {
            motor,
            // C: Kp=2000/2^7 and Ki=50/2^15 for both current axes.
            id_pi: Pi::new(2_000, 7, 50, 27_851),
            iq_pi: Pi::new(2_000, 7, 50, 27_851),
            id_reference: 0,
            iq_reference: 0,
            voltage_limit: 27_851,
            measured_id: 0,
            measured_iq: 0,
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
        self.measured_id = id.clamp(-Q15_ONE, Q15_ONE) as i16;
        self.measured_iq = iq.clamp(-Q15_ONE, Q15_ONE) as i16;

        let vd = self.id_pi.update(i32::from(self.id_reference), id);
        let vq = self.iq_pi.update(i32::from(self.iq_reference), iq);
        // Keep the normal current-loop path free of software division. The
        // component clamp is the same bounded fallback used by the bring-up
        // controller and preserves the M0+ timing budget.
        let limit = i32::from(self.voltage_limit);
        let vd = vd.clamp(-limit, limit);
        let vq = vq.clamp(-limit, limit);

        let v_alpha = ((vd * cos) - (vq * sin)) >> 15;
        let v_beta = ((vd * sin) + (vq * cos)) >> 15;
        // This is the same center-aligned three-phase calculation as the C
        // FOC_MC_svpwm path.  The previous hand-written sector permutation
        // swapped phases in the alpha sectors: an alpha vector could become a
        // single-phase duty change, producing current without average torque.
        // Each product is below 2^31, so this remains entirely 32-bit.
        let v_a = v_alpha;
        // Inverse Clarke, matching the C reference's U2/U3 terms:
        //   Vb = -Va/2 + sqrt(3)/2*Vbeta
        //   Vc = -Va/2 - sqrt(3)/2*Vbeta
        // The previous code used Q15_ONE with a 16-bit shift for both terms,
        // halving the beta vector and leaving the rotating field too weak.
        let v_b = ((-v_alpha * HALF_Q15) + (v_beta * SQRT3_OVER_2_Q15)) >> 15;
        let v_c = ((-v_alpha * HALF_Q15) - (v_beta * SQRT3_OVER_2_Q15)) >> 15;
        let max = v_a.max(v_b).max(v_c);
        let min = v_a.min(v_b).min(v_c);
        let common = (max + min) / 2;
        let a = 16_384 + ((v_a - common) / 2);
        let b = 16_384 + ((v_b - common) / 2);
        let c = 16_384 + ((v_c - common) / 2);
        Duty {
            a: a.clamp(1, 32_766) as u16,
            b: b.clamp(1, 32_766) as u16,
            c: c.clamp(1, 32_766) as u16,
        }
    }
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
    fn jd4631_uses_the_11_pole_pair_motor() {
        assert_eq!(JD4631.pole_pairs, 11);
        assert_eq!(JD4631.phase_resistance_mohm, 6_700);
    }

    #[test]
    fn speed_controller_zero_command_removes_torque_reference() {
        let mut speed = SpeedController::new(800);
        assert!(speed.update(400, 0) > 0);
        speed.reset();
        assert_eq!(speed.update(0, 0), 0);
    }
}
