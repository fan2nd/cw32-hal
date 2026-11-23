//! Hardware peripherals and fixed-point conversions for the L012 FOC example.

use crate::foc::{JD4631, PhaseCurrent};
pub use crate::config::*;
pub use crate::foc::sin_cos_q15_lut;

/// Encoder resolution is a board/sensor choice, independent of motor data.
/// The current L012 wiring uses the 14-bit AS5048A/MT6701-compatible frame.
pub fn q15_from_adc(sample: u16, offset: u16) -> i16 {
    let signed = i32::from(offset) - i32::from(sample);
    // The C project feeds the calibrated ADC difference directly to Clarke /
    // Park and tunes both PI loops in ADC-count units (M1_IQ_MAX_COUNTS=2000).
    // Do not normalize by 2048 here: that multiplies the measured current by
    // sixteen and makes a normal speed command saturate the power stage.
    signed.clamp(-32_767, 32_767) as i16
}

/// Convert a calibrated ADC difference to mA using the board's 2.5 V bias,
/// 50 mOhm shunt and 10x current amplifier. The bias itself is removed by the
/// runtime offset sample before this function is called; no floating point or
/// 64-bit intermediate is needed.
pub fn current_ma_from_counts(counts: i16) -> i32 {
    let output_mv =
        i32::from(counts).saturating_mul(ADC_REFERENCE_MV as i32) / ADC_FULL_SCALE as i32;
    output_mv.saturating_mul(1_000) / (CURRENT_SHUNT_MOHM * CURRENT_AMPLIFIER_GAIN) as i32
}

pub fn phase_current(a: u16, b: u16, offset_a: u16, offset_b: u16) -> PhaseCurrent {
    PhaseCurrent {
        a: q15_from_adc(a, offset_a),
        b: q15_from_adc(b, offset_b),
    }
}


pub fn bus_voltage_mv(sample: u16) -> u32 {
    u32::from(sample) * ADC_REFERENCE_MV * BUS_DIVIDER_RATIO / ADC_FULL_SCALE
}

pub fn electrical_angle(mechanical: u16, offset: u16) -> u16 {
    // The validated board handoff uses the inverted relative displacement
    // from the captured alignment position. Keep the SSI inversion in
    // `Encoder::read_angle` and apply this one fixed electrical orientation.
    // The 14-bit mechanical angle is expanded to the C 16-bit full-turn
    // electrical representation by multiplying by four.
    let mechanical = i64::from(offset.wrapping_sub(mechanical) & 0x3fff);
    ((mechanical * i64::from(JD4631.pole_pairs) * 4
        + i64::from(ALIGNMENT_ELECTRICAL_OFFSET))
        & 0xffff) as u16
}

/// Electrical-angle increment per current-loop tick for a mechanical speed in
/// rpm. The sign follows `electrical_angle`, whose mechanical displacement is
/// inverted on this board. The product is widened before division so the
/// fixed-point phase increment matches the reference controller's range.
pub fn electrical_angle_step_for_rpm(mechanical_rpm: i32) -> i32 {
    let numerator = i64::from(mechanical_rpm)
        * i64::from(JD4631.pole_pairs)
        * 65_536;
    (numerator / i64::from(60 * PWM_HZ as i32)) as i32
}

pub fn phase_step_for_rpm(target_rpm: u32) -> u32 {
    // Q16 electrical-turn accumulator, with the same 32-bit split used by
    // the verified open-loop binary. No 64-bit intermediate is needed.
    const DENOMINATOR: u32 = 60 * PWM_HZ;
    const SCALE: u32 = 1 << 30;
    let turns = target_rpm.saturating_mul(u32::from(JD4631.pole_pairs));
    let quotient = SCALE / DENOMINATOR;
    turns.saturating_mul(quotient).max(1)
}

pub fn angle_delta(current: u16, previous: u16) -> i32 {
    let delta = i32::from(current) - i32::from(previous);
    if delta > 8_192 {
        delta - 16_384
    } else if delta < -8_192 {
        delta + 16_384
    } else {
        delta
    }
}

/// Keep one bad SSI frame from becoming a full speed-loop correction.
///
/// The absolute encoder wraps at 14 bits. `angle_delta` already handles that
/// intended wrap, but a malformed frame can still look like a several-thousand
/// count jump. At the configured 240 rpm limit the real displacement between
/// foreground samples is far below this bound, so holding the previous angle
/// for one sample is safer than asking the speed PI to brake the motor. The
/// limit is a sampling-integrity bound, not a low-speed control branch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncoderAngleTracker {
    angle: u16,
}

impl EncoderAngleTracker {
    pub const MAX_SAMPLE_DELTA_COUNTS: i32 = 512;

    pub const fn new(angle: u16) -> Self {
        Self { angle }
    }

    pub fn update_checked(&mut self, sample: u16) -> (u16, bool) {
        let delta = angle_delta(sample, self.angle);
        if delta.abs() <= Self::MAX_SAMPLE_DELTA_COUNTS {
            self.angle = sample;
            (self.angle, true)
        } else {
            (self.angle, false)
        }
    }

    pub fn update(&mut self, sample: u16) -> u16 {
        self.update_checked(sample).0
    }

    pub const fn angle(self) -> u16 {
        self.angle
    }
}

/// Convert an encoder displacement to mechanical rpm using the measured
/// number of PWM periods elapsed. The foreground loop can take longer than
/// one PWM period while it performs the blocking SSI transfer and CORDIC
/// operation, so its iteration count is not a valid clock. Keep the split
/// multiply/divide form entirely in signed 32-bit arithmetic.
pub fn speed_rpm_from_delta(delta_counts: i32, elapsed_pwm_periods: u32) -> i32 {
    if elapsed_pwm_periods == 0 {
        return 0;
    }
    let counts_per_rev = i32::from(ENCODER_COUNTS_PER_REV);
    let elapsed = elapsed_pwm_periods.min(i32::MAX as u32) as i32;
    let scaled = delta_counts.saturating_mul(60);
    let whole_turns = scaled / counts_per_rev;
    let remainder = scaled % counts_per_rev;
    let whole = whole_turns.saturating_mul(PWM_HZ as i32) / elapsed;
    let denominator = counts_per_rev.saturating_mul(elapsed);
    let fraction = if denominator == 0 {
        0
    } else {
        remainder.saturating_mul(PWM_HZ as i32) / denominator
    };
    whole.saturating_add(fraction)
}
