//! Board-level peripherals and fixed-point conversions for the L012 FOC board.

use crate::foc::{Duty, JD4631, PhaseCurrent};

/// Encoder resolution is a board/sensor choice, independent of motor data.
/// The current L012 wiring uses the 14-bit AS5048A/MT6701-compatible frame.
pub const ENCODER_COUNTS_PER_REV: u16 = 16_384;
use embassy_cw32::{
    self as hal,
    cordic::{Cordic, Q1_31},
    gpio::Flex,
    mode::Blocking,
    peripherals,
    spi::{self, Spi},
    timer::{Channel, complementary_pwm::ComplementaryPwm, low_level::Timer},
};

pub const PWM_HZ: u32 = 16_000;
pub const CURRENT_SAMPLE_TICKS: u16 = 250; // BTIM1: 8 MHz / 16 kHz, sample at PWM midpoint.
pub const ADC_REFERENCE_MV: u32 = 5_000;
pub const ADC_FULL_SCALE: u32 = 4_095;
pub const CURRENT_BIAS_MV: u32 = 2_500;
pub const CURRENT_SHUNT_MOHM: u32 = 50;
pub const CURRENT_AMPLIFIER_GAIN: u32 = 10;
pub const BUS_DIVIDER_RATIO: u32 = 11; // C project: RV1=1, RV2=10.
pub const ADC_FULL_SCALE_COUNTS: i32 = 2_048;
pub const ALIGNMENT_PERIODS: u32 = PWM_HZ / 2; // C project: T_ALIGNMENT=500 ms.
// The closed-loop controller uses the C reference's A-axis alignment vector,
// so the encoder sample taken after alignment is electrical angle zero.
pub const ALIGNMENT_ELECTRICAL_OFFSET: u16 = 0;
pub const READY_ZERO_SAMPLES: u32 = 1_600;
pub const POT_ZERO_COUNTS: u16 = 150; // C project NMINVD-50.
pub const POT_MIN_COUNTS: u16 = 200; // C project NMINVD.
pub const POT_MAX_COUNTS: u16 = 3_523; // C project NMAXVD.
pub const POT_MIN_RPM: i32 = 0;
pub const POT_MAX_RPM: i32 = 240;
pub const SPEED_LOOP_DIV: u32 = 16;
pub const DIRECTION_TEST_PERIODS: u32 = PWM_HZ / 10;
// The reference C controller uses raw ADC-count current units. Keep the
// direction check in that same unit so it does not overdrive the amplifier.
pub const DIRECTION_TEST_IQ_Q15: i16 = 400;
pub const DIRECTION_MIN_DELTA_COUNTS: i32 = 4;
pub const BUS_MIN_MV: u32 = 8_000;
pub const BUS_MAX_MV: u32 = 16_000;

pub struct Encoder<'d> {
    spi: Spi<'d, Blocking>,
    cs: Flex<'d>,
}

impl<'d> Encoder<'d> {
    pub fn new(spi: Spi<'d, Blocking>, cs: Flex<'d>) -> Self {
        Self { spi, cs }
    }

    pub fn read_angle(&mut self) -> Result<u16, spi::Error> {
        let mut frame = [0u8; 3];
        self.cs.set_low();
        let result = self.spi.blocking_transfer_in_place(&mut frame);
        self.cs.set_high();
        result?;
        let raw = (u16::from(frame[0]) << 6) | (u16::from(frame[1]) >> 2);
        Ok((ENCODER_COUNTS_PER_REV - (raw & 0x3fff)) & 0x3fff)
    }
}

pub fn wait_period<T: hal::timer::OverflowInstance>(timer: &mut Timer<'_, T>) {
    while !timer.take_overflow() {
        core::hint::spin_loop();
    }
}

pub fn wait_current_sample_phase<T: hal::timer::OverflowInstance>(timer: &mut Timer<'_, T>) {
    while timer.get_counter() < CURRENT_SAMPLE_TICKS {
        core::hint::spin_loop();
    }
}

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

// 1/4-wave Q1.15 lookup table. The current ISR uses this instead of the
// blocking CORDIC call so angle prediction does not add peripheral latency.
const SIN_QUARTER_Q15: [i16; 65] = [
    0, 804, 1608, 2410, 3212, 4011, 4808, 5602, 6393, 7179, 7962, 8739, 9512, 10278, 11039, 11793,
    12539, 13279, 14010, 14732, 15446, 16151, 16846, 17530, 18204, 18868, 19519, 20159, 20787,
    21403, 22005, 22594, 23170, 23731, 24279, 24811, 25329, 25832, 26319, 26790, 27245, 27683,
    28105, 28510, 28898, 29268, 29621, 29956, 30273, 30571, 30852, 31113, 31356, 31580, 31785,
    31971, 32137, 32285, 32412, 32521, 32609, 32678, 32728, 32757, 32767,
];

/// Return Q1.15 sine/cosine for a 16-bit electrical angle without division or
/// peripheral access. Linear interpolation is intentionally omitted to keep
/// the 16 kHz current ISR bounded on Cortex-M0+.
pub fn sin_cos_q15_lut(angle: u16) -> (i16, i16) {
    fn quarter(index: u8) -> i16 {
        SIN_QUARTER_Q15[index as usize]
    }
    fn sin(angle: u16) -> i16 {
        let quadrant = (angle >> 14) as u8;
        let index = ((angle & 0x3fff) >> 8) as u8;
        match quadrant {
            0 => quarter(index),
            1 => quarter(64 - index),
            2 => -quarter(index),
            _ => -quarter(64 - index),
        }
    }
    (sin(angle), sin(angle.wrapping_add(16_384)))
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

pub fn sin_cos_q15(cordic: &mut Cordic<'_, peripherals::CORDIC>, angle: u16) -> (i16, i16) {
    // CORDIC input is angle/pi in Q1.31. The FOC angle is the C reference's
    // 16-bit electrical angle, so half a turn is 32768 and the conversion is
    // a 16-bit shift.
    let signed = if angle >= 32768 {
        i32::from(angle) - 65536
    } else {
        i32::from(angle)
    };
    let normalized = signed << 16;
    let waveform = cordic.sin_cos(Q1_31::from_bits(normalized), 1_024).unwrap();
    (
        (waveform.sin.to_bits() >> 16) as i16,
        (waveform.cos.to_bits() >> 16) as i16,
    )
}

pub fn apply_duty(pwm: &mut ComplementaryPwm<'_, peripherals::ATIM>, duty: Duty) {
    let max = pwm.get_max_duty();
    pwm.set_duty_cycle(Channel::Ch1, u32::from(duty.a) * max / 32_767);
    pwm.set_duty_cycle(Channel::Ch2, u32::from(duty.b) * max / 32_767);
    pwm.set_duty_cycle(Channel::Ch3, u32::from(duty.c) * max / 32_767);
}
