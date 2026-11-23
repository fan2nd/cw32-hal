#![no_std]

//! Allocation-free fixed-point FOC algorithms shared by the CW32L012 examples.
//!
//! This crate owns the current PID, speed PID, Clarke/Park transforms and SVPWM.
//! The kernel deliberately contains no PAC or HAL calls.  This keeps the
//! control law testable on a host and makes the hardware boundary explicit:
//! the application supplies calibrated currents and the electrical angle, and
//! receives three normalized PWM duties.

pub mod calibration;
pub mod control;
mod trig;

pub use trig::sin_cos_q15_lut;

const Q15_ONE: i32 = 32_767;

const fn q15_mul(lhs: i32, rhs: i32) -> i32 {
    (lhs * rhs) >> 15
}
const SQRT3_OVER_2_Q15: i32 = 0x6ed9;
const INV_SQRT3_Q15: i32 = 18_918;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhaseCurrent {
    pub a: i16,
    pub b: i16,
}

/// Two-shunt current sample filter copied from `SVPWM_GetPhaseCurrentValues`.
///
/// The ADC is deliberately sampled at the same timer phase every cycle, but
/// the OPA/ADC path can still produce one isolated conversion spike when a
/// complementary edge is nearby. The C controller discards one jump larger
/// than 50 counts and accepts it only when the next sample repeats the jump.
/// Keeping the history here makes that behavior explicit and keeps the fast
/// path in signed 32-bit arithmetic.
#[derive(Clone, Copy, Debug, Default)]
pub struct CurrentSampleFilter {
    last_a: i32,
    last_b: i32,
    first_a: bool,
    first_b: bool,
    glitch_a: bool,
    glitch_b: bool,
}

impl CurrentSampleFilter {
    pub const fn new() -> Self {
        Self {
            last_a: 0,
            last_b: 0,
            first_a: false,
            first_b: false,
            glitch_a: false,
            glitch_b: false,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    #[inline]
    fn accept(sample: i32, last: &mut i32, first: &mut bool, glitch: &mut bool) -> i16 {
        if !*first {
            *first = true;
            *last = sample;
            return sample.clamp(-32_767, 32_767) as i16;
        }

        let delta = sample - *last;
        if delta > 50 || delta < -50 {
            if *glitch {
                // Second consecutive excursion: it is a real current change.
                *last = sample;
                *glitch = true;
            } else {
                // Isolated conversion spike: retain the previous sample.
                *glitch = true;
                return (*last).clamp(-32_767, 32_767) as i16;
            }
        } else {
            *last = sample;
            *glitch = false;
        }
        (*last).clamp(-32_767, 32_767) as i16
    }

    pub fn sample(&mut self, a: u16, b: u16, offset_a: u16, offset_b: u16) -> PhaseCurrent {
        let raw_a = i32::from(offset_a) - i32::from(a);
        let raw_b = i32::from(offset_b) - i32::from(b);
        PhaseCurrent {
            a: Self::accept(
                raw_a,
                &mut self.last_a,
                &mut self.first_a,
                &mut self.glitch_a,
            ),
            b: Self::accept(
                raw_b,
                &mut self.last_b,
                &mut self.first_b,
                &mut self.glitch_b,
            ),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Duty {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

impl Duty {
    /// Reallocate the zero-vector dwell to 000 by clamping the lowest high-side duty.
    /// All three line-to-line duty differences are preserved.
    pub fn five_segment(self) -> Self {
        let low = self.a.min(self.b).min(self.c);
        Self { a: self.a - low, b: self.b - low, c: self.c - low }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorParameters {
    pub pole_pairs: u16,
    pub phase_resistance_mohm: u32,
    pub phase_inductance_uh: u32,
    pub rated_voltage_mv: u32,
    pub rated_current_ma: u32,
    pub max_current_ma: u32,
    /// Current-loop proportional gain numerator.
    pub current_kp_gain: i32,
    /// Right shift applied to the current-loop proportional term.
    pub current_kp_shift: u8,
    /// Current-loop integral gain in the controller's fixed-point scale.
    pub current_ki_gain: i32,
    /// Per-axis current-loop voltage output limit in Q1.15.
    pub current_voltage_limit: i32,
}

/// JD-Power 4621-class motor from `JD-4621.jpg` (24N22P = 22 poles = 11
/// pole pairs).
///
/// Encoder selection and electrical interface are kept in the board layer;
/// this parameter set describes only the motor and its control gains.
pub const JD4631: MotorParameters = MotorParameters {
    pole_pairs: 11,
    phase_resistance_mohm: 6_700,
    phase_inductance_uh: 1_750,
    rated_voltage_mv: 12_000,
    rated_current_ma: 1_100,
    max_current_ma: 2_000,
    current_kp_gain: 2_000,
    current_kp_shift: 7,
    current_ki_gain: 50,
    current_voltage_limit: 27_851,
};

/// HT2806 / GB2806 motor data from `assets/HT2806-param.png`.
///
/// The supplied sheet lists AS5048A/AS5600 as possible encoder options; encoder
/// selection is intentionally kept out of this motor-only parameter crate.
/// The sheet does not specify controller gains, so the same conservative
/// fixed-point bring-up gains are used until a current trace is available.
pub const HT2806: MotorParameters = MotorParameters {
    pole_pairs: 7,
    phase_resistance_mohm: 5_000,
    phase_inductance_uh: 1_510,
    rated_voltage_mv: 12_000,
    rated_current_ma: 810,
    max_current_ma: 1_220,
    current_kp_gain: 2_000,
    current_kp_shift: 7,
    current_ki_gain: 50,
    current_voltage_limit: 27_851,
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
        // The current-loop ranges are bounded to ADC/PWM 32-bit values. Keep
        // this ISR path in native-width arithmetic; the speed-loop accumulators
        // below use i64 where their elapsed-time product can grow larger.
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
    integral_scaled: i32,
    last_target_rpm: i32,
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
            integral_scaled: 0,
            last_target_rpm: 0,
        }
    }

    pub fn reset(&mut self) {
        self.integral_scaled = 0;
        self.last_target_rpm = 0;
    }

    pub fn update(&mut self, target_rpm: i32, feedback_rpm: i32) -> i16 {
        self.update_for_ticks(target_rpm, feedback_rpm, 1)
    }

    pub fn update_for_ticks(&mut self, target_rpm: i32, feedback_rpm: i32, elapsed_ms: u32) -> i16 {
        self.update_for_ticks_in_range(target_rpm, feedback_rpm, elapsed_ms, i16::MIN, i16::MAX)
    }

    /// Apply the same actuator limits to the integral and PI output.
    pub fn update_for_ticks_in_range(
        &mut self,
        target_rpm: i32,
        feedback_rpm: i32,
        elapsed_ms: u32,
        iq_min: i16,
        iq_max: i16,
    ) -> i16 {
        self.update_for_quarter_ms_in_range(
            target_rpm, feedback_rpm, elapsed_ms.clamp(1, 100) * 4, iq_min, iq_max,
        )
    }

    /// Integrate in 250 us ticks with the same gains as the millisecond API.
    pub fn update_for_quarter_ms_in_range(
        &mut self,
        target_rpm: i32,
        feedback_rpm: i32,
        quarter_ms: u32,
        iq_min: i16,
        iq_max: i16,
    ) -> i16 {
        let lower = i64::from(iq_min);
        let upper = i64::from(iq_max);
        if self.last_target_rpm > 0 && target_rpm < self.last_target_rpm {
            // A falling command must brake immediately. Keeping the positive
            // acceleration integral here can exactly cancel the negative
            // proportional term and produce iqref=0 while overspeeding.
            self.integral_scaled = 0;
        }
        self.last_target_rpm = target_rpm;
        let error = (target_rpm - feedback_rpm).clamp(-self.max_speed_rpm, self.max_speed_rpm);
        // Drop stale acceleration torque as soon as the measured speed is
        // above the reference.  Without this, the positive integral can keep
        // the motor accelerating for several slow-loop periods after a small
        // command, which is visible as target=30 while feedback is about 60.
        if (error < 0 && self.integral_scaled > 0) || (error > 0 && self.integral_scaled < 0) {
            self.integral_scaled = 0;
        }
        let proportional = (i64::from(error) * i64::from(self.kp_gain)) >> self.kp_shift;
        let elapsed = i64::from(quarter_ms.clamp(1, 400));
        let integral_shift = self.ki_shift + 2;
        // Accumulate before shifting, as in the C speed PI. A persistent
        // 1 rpm error must build torque even when one tick adds less than 1 count.
        let increment = i64::from(error) * i64::from(self.ki_gain) * elapsed;
        self.integral_scaled = (i64::from(self.integral_scaled) + increment)
            .clamp(lower << integral_shift, upper << integral_shift) as i32;
        (proportional + i64::from(self.integral_scaled) / (1 << integral_shift))
            .clamp(lower, upper) as i16
    }
}

impl FocController {
    pub const fn new(motor: MotorParameters) -> Self {
        // Gains are conservative Q1.15 starting values.  They are intentionally
        // exposed here rather than hidden in a board-specific ISR so they can be
        // tuned against the actual current amplifier and bus voltage.
        Self {
            motor,
            id_pi: Pi::new(
                motor.current_kp_gain,
                motor.current_kp_shift,
                motor.current_ki_gain,
                motor.current_voltage_limit,
            ),
            iq_pi: Pi::new(
                motor.current_kp_gain,
                motor.current_kp_shift,
                motor.current_ki_gain,
                motor.current_voltage_limit,
            ),
            id_reference: 0,
            iq_reference: 0,
            voltage_limit: motor.current_voltage_limit as i16,
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
        let (alpha, beta) = self.update_voltage(currents, sin_q15, cos_q15);
        svpwm(alpha, beta)
    }

    /// Run the same current PI with low-clamped five-segment modulation.
    pub fn update_five_segment(&mut self, currents: PhaseCurrent, sin_q15: i16, cos_q15: i16) -> Duty {
        let (alpha, beta) = self.update_voltage(currents, sin_q15, cos_q15);
        svpwm_five_segment(alpha, beta)
    }

    fn update_voltage(&mut self, currents: PhaseCurrent, sin_q15: i16, cos_q15: i16) -> (i32, i32) {
        let ia = i32::from(currents.a);
        let ib = i32::from(currents.b);
        let alpha = ia;
        let beta = q15_mul(ia + 2 * ib, INV_SQRT3_Q15);

        let sin = i32::from(sin_q15);
        let cos = i32::from(cos_q15);
        let id = q15_mul(alpha, cos) + q15_mul(beta, sin);
        let iq = q15_mul(-alpha, sin) + q15_mul(beta, cos);
        self.measured_id = id.clamp(-Q15_ONE, Q15_ONE) as i16;
        self.measured_iq = iq.clamp(-Q15_ONE, Q15_ONE) as i16;

        let vd = self.id_pi.update(i32::from(self.id_reference), id);
        let vq = self.iq_pi.update(i32::from(self.iq_reference), iq);
        // Bound each PI output before rotating it into the stationary frame.
        let limit = i32::from(self.voltage_limit);
        let vd = vd.clamp(-limit, limit);
        let vq = vq.clamp(-limit, limit);

        let v_alpha = q15_mul(vd, cos) - q15_mul(vq, sin);
        let v_beta = q15_mul(vd, sin) + q15_mul(vq, cos);
        (v_alpha, v_beta)
    }
}

/// Five-segment space-vector PWM from stationary-frame Q15 voltage components.
#[inline]
pub fn svpwm_five_segment(v_alpha: i32, v_beta: i32) -> Duty {
    svpwm(v_alpha, v_beta).five_segment()
}

/// Centered space-vector PWM from stationary-frame Q15 voltage components.
#[inline]
pub fn svpwm(v_alpha: i32, v_beta: i32) -> Duty {
    // C reference U1 = B-C, U2 = A-B in normalized timer ticks.
    // Choose B as the reference, then center the three compare values.
    let mut a = (v_alpha * SQRT3_OVER_2_Q15) / 32768 - (v_beta >> 1);
    let mut c = -v_beta;
    let span = a.max(c).max(0) - a.min(c).min(0);
    if span > 32768 {
        // As with the reference Tx+Ty limit, shorten both active vector times
        // proportionally so overmodulation preserves the requested angle.
        let gain = (32768 << 15) / span;
        a = q15_mul(a, gain);
        c = q15_mul(c, gain);
    }
    let center = 16384 - (a.max(c).max(0) + a.min(c).min(0)) / 2;
    Duty {
        a: (center + a).clamp(1, 32766) as u16,
        b: center.clamp(1, 32766) as u16,
        c: (center + c).clamp(1, 32766) as u16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_segment_clamps_one_leg_and_preserves_line_voltages() {
        for angle in (0..=u16::MAX).step_by(31) {
            let (sin, cos) = sin_cos_q15_lut(angle);
            for magnitude in [0, 8192, 32767] {
                let alpha = q15_mul(i32::from(cos), magnitude);
                let beta = q15_mul(i32::from(sin), magnitude);
                let centered = svpwm(alpha, beta);
                let duty = svpwm_five_segment(alpha, beta);
                assert_eq!(duty.a.min(duty.b).min(duty.c), 0);
                assert_eq!(i32::from(duty.a) - i32::from(duty.b),
                           i32::from(centered.a) - i32::from(centered.b));
                assert_eq!(i32::from(duty.b) - i32::from(duty.c),
                           i32::from(centered.b) - i32::from(centered.c));
            }
        }
    }

    #[test]
    fn sweep_matches_c_reference_compare_values() {
        // Reference SVPWM_GetDutyCycles at PWM_PERIOD=3000 and 25% excitation.
        let reference = [
            [1824, 1175, 1175], [1823, 1823, 1176], [1176, 1823, 1176],
            [1175, 1824, 1824], [1176, 1176, 1823], [1823, 1176, 1823],
        ];
        for (sector, expected) in reference.into_iter().enumerate() {
            let (sin, cos) = sin_cos_q15_lut((sector * 65536 / 6) as u16);
            let duty = svpwm(i32::from(cos) / 4, i32::from(sin) / 4);
            for (compare, expected) in [duty.a, duty.b, duty.c].into_iter().zip(expected) {
                assert!((i32::from(compare) * 3000 / 32767 - expected).abs() <= 2);
            }
        }
    }

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
    fn ht2806_uses_the_sheet_parameters() {
        assert_eq!(HT2806.pole_pairs, 7);
        assert_eq!(HT2806.phase_resistance_mohm, 5_000);
        assert_eq!(HT2806.phase_inductance_uh, 1_510);
        assert_eq!(HT2806.rated_current_ma, 810);
    }

    #[test]
    fn speed_controller_zero_command_removes_torque_reference() {
        let mut speed = SpeedController::new(800);
        assert!(speed.update(400, 0) > 0);
        speed.reset();
        assert_eq!(speed.update(0, 0), 0);
    }
}
