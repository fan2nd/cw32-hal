//! Pulse-based initial position detection and continuous square-wave saliency tracking.
//! Angles wrap at one electrical turn in u16; input currents are offset-corrected ADC counts.
use crate::{PhaseCurrent, sin_cos_q15_lut};

pub const CARRIER_VOLTAGE: i32 = 3400;
const PULSE_VOLTAGE: i32 = 5400;
const AXES: usize = 12;
const REPEATS: i32 = 8;
const AXIS_ANGLES: [u16; AXES] = {
    let mut table = [0; AXES];
    let mut i = 0;
    while i < AXES {
        table[i] = (i as u32 * 32768 / AXES as u32) as u16;
        i += 1;
    }
    table
};

pub fn project(current: PhaseCurrent, angle: u16) -> (i32, i32) {
    let a = i32::from(current.a);
    let b = ((a + 2 * i32::from(current.b)) * 18918) >> 15;
    let (sin, cos) = sin_cos_q15_lut(angle);
    (
        (a * i32::from(cos) + b * i32::from(sin)) >> 15,
        (b * i32::from(cos) - a * i32::from(sin)) >> 15,
    )
}

pub struct Scan {
    phase: u8,
    axis: usize,
    second: bool,
    repeat: i32,
    baseline: i32,
    harmonic: (i32, i32),
    positive: [i32; AXES],
    negative: [i32; AXES],
}

pub struct Detection {
    pub angle: u16,
    pub contrast: i32,
    pub polarity: i32,
}

impl Scan {
    pub const fn new() -> Self {
        Self {
            phase: 0,
            axis: 0,
            second: false,
            repeat: 0,
            baseline: 0,
            harmonic: (0, 0),
            positive: [0; AXES],
            negative: [0; AXES],
        }
    }

    /// Each pulse gets 32 zero-voltage periods to decay, then eight driven periods.
    /// Read before writing the next preload. Equal latency applies to both signs.
    pub fn sample(&mut self, current: PhaseCurrent) -> (u16, i32, Option<Detection>) {
        let axis = self.axis;
        let phase = self.phase;
        let positive = !self.second ^ (self.repeat & 1 != 0);
        let angle = AXIS_ANGLES[axis];
        if phase == 31 {
            self.baseline = project(current, angle).0;
        }
        if phase == 39 {
            let d = project(current, angle).0;
            if positive {
                self.positive[axis] += d - self.baseline;
            } else {
                self.negative[axis] += self.baseline - d;
            }
        }
        let voltage = if (31..39).contains(&phase) {
            if positive {
                PULSE_VOLTAGE
            } else {
                -PULSE_VOLTAGE
            }
        } else {
            0
        };
        self.phase += 1;
        if self.phase == 40 {
            self.phase = 0;
            self.second = !self.second;
            if !self.second {
                if self.repeat == REPEATS - 1 {
                    self.accumulate_axis(axis);
                }
                self.axis += 1;
                if self.axis == AXES {
                    self.axis = 0;
                    self.repeat += 1;
                }
            }
        }
        let done = self.repeat == REPEATS;
        (angle, voltage, done.then(|| self.detect()))
    }

    /// Mean signed-axis response magnitude for each polarity after a complete scan.
    pub fn axis_response(&self, axis: usize) -> (i16, i16) {
        (
            (self.positive[axis] / REPEATS) as i16,
            (self.negative[axis] / REPEATS) as i16,
        )
    }

    fn accumulate_axis(&mut self, axis: usize) {
        // Q8 cosine at 30-degree intervals; sine is a quarter-turn shift.
        const COS: [i32; 12] = [256, 222, 128, 0, -128, -222, -256, -222, -128, 0, 128, 222];
        const SIN: [i32; 12] = [0, 128, 222, 256, 222, 128, 0, -128, -222, -256, -222, -128];
        let sum = self.positive[axis] + self.negative[axis];
        self.harmonic.0 += sum * COS[axis];
        self.harmonic.1 += sum * SIN[axis];
    }

    fn detect(&self) -> Detection {
        // For y = offset + A*cos(2*(axis-theta)), the phase of the
        // second harmonic is 2*theta; the bipolar fit cannot choose north.
        let x = self.harmonic.0 / (256 * 2 * REPEATS);
        let y = self.harmonic.1 / (256 * 2 * REPEATS);
        let (major, minor) = (x.abs().max(y.abs()), x.abs().min(y.abs()));
        // Bisect [0,45] degrees using tan(k*pi/128) in Q12.
        const TAN: [i32; 32] = [
            0, 101, 201, 302, 403, 505, 608, 711, 815, 920, 1026, 1134, 1243, 1353, 1466, 1580,
            1697, 1816, 1937, 2062, 2189, 2320, 2455, 2594, 2737, 2885, 3038, 3197, 3362, 3533,
            3712, 3900,
        ];
        let mut bin = 0usize;
        for bit in [16, 8, 4, 2, 1] {
            let candidate = bin | bit;
            if minor * 4096 >= major * TAN[candidate] {
                bin = candidate;
            }
        }
        let octant = bin as u16 * 256 + 128;
        let first = if x.abs() >= y.abs() {
            octant
        } else {
            16384 - octant
        };
        let upper = if x < 0 { 32768 - first } else { first };
        let doubled = if y < 0 { upper.wrapping_neg() } else { upper };
        let (sin, cos) = sin_cos_q15_lut(doubled);
        // Harmonic magnitude = 6*A for twelve axes; report peak-to-trough 2*A.
        let contrast = (x * (i32::from(cos) >> 7) + y * (i32::from(sin) >> 7)) / (256 * 3);
        let mut angle = doubled / 2;
        let nearest = (usize::from(angle) * AXES + 16384) >> 15;
        let axis = if nearest == AXES { 0 } else { nearest };
        let mut polarity = (self.positive[axis] - self.negative[axis]) / REPEATS;
        if nearest == AXES {
            polarity = -polarity;
        }
        if polarity < 0 {
            angle = angle.wrapping_add(32768);
        }
        Detection {
            angle,
            contrast,
            polarity,
        }
    }
}

/// Select the end of an unsigned electrical axis closest to a known field.
pub fn nearest_axis_angle(axis: u16, field: u16) -> u16 {
    if (axis.wrapping_sub(field) as i16).unsigned_abs() > 16384 {
        axis.wrapping_add(32768)
    } else {
        axis
    }
}

#[cfg(test)]
#[test]
fn aligned_axis_selects_nearest_pole_across_wrap() {
    assert_eq!(nearest_axis_angle(32770, 65534), 2);
    assert_eq!(nearest_axis_angle(2, 32766), 32770);
    assert_eq!(nearest_axis_angle(65530, 3), 65530);
}

/// Qualify standstill using the range of the observed axis over 32 carrier periods.
pub struct StandstillWindow {
    low: i16,
    high: i16,
    count: u8,
}

impl StandstillWindow {
    pub const fn new() -> Self {
        Self {
            low: i16::MAX,
            high: i16::MIN,
            count: 0,
        }
    }

    pub fn update(&mut self, angle: u16, field: u16, valid: bool) -> bool {
        let offset = angle.wrapping_sub(field) as i16;
        if !valid || offset.unsigned_abs() > 6372 {
            *self = Self::new();
            return false;
        }
        self.low = self.low.min(offset);
        self.high = self.high.max(offset);
        if self.high - self.low > 910 {
            // five electrical degrees peak to peak
            self.low = offset;
            self.high = offset;
            self.count = 0;
        }
        self.count += 1;
        if self.count == 32 {
            *self = Self::new();
            return true;
        }
        false
    }
}

pub struct Tracker {
    angle_q8: u32,
    step_q8: i32,
    phase: u8,
    positive: (i32, i32),
    previous_negative: Option<(i32, i32)>,
    primed: bool,
    pub carrier_valid: bool,
    half_carrier: [PhaseCurrent; 8],
    pub fundamental: PhaseCurrent,
    weak_cycles: u16,
}

impl Tracker {
    pub const fn new(angle: u16) -> Self {
        Self {
            angle_q8: (angle as u32) << 8,
            step_q8: 0,
            phase: 0,
            positive: (0, 0),
            previous_negative: None,
            primed: false,
            carrier_valid: false,
            half_carrier: [PhaseCurrent { a: 0, b: 0 }; 8],
            fundamental: PhaseCurrent { a: 0, b: 0 },
            weak_cycles: 0,
        }
    }

    pub fn angle(&self) -> u16 {
        (self.angle_q8 >> 8) as u16
    }
    pub fn rpm(&self, pole_pairs: i32) -> i32 {
        // 16000 * 60 / (65536 * 256) = 1875 / 32768.
        self.step_q8 * 1875 / (32768 * pole_pairs)
    }

    /// Demodulate a positive peak against the two neighbouring negative peaks.
    /// Linear changes in fundamental dq current cancel at the center sample.
    pub fn sample(&mut self, current: PhaseCurrent) -> Option<i32> {
        self.sample_with_divide(current, |a, b| a / b)
    }

    pub fn sample_with_divide(
        &mut self,
        current: PhaseCurrent,
        divide: impl FnOnce(i32, i32) -> i32,
    ) -> Option<i32> {
        self.track(current, false, divide)
    }

    /// Acquire angle at standstill without integrating alignment corrections into speed.
    pub fn track(
        &mut self,
        current: PhaseCurrent,
        stationary: bool,
        divide: impl FnOnce(i32, i32) -> i32,
    ) -> Option<i32> {
        // Pair samples half a carrier period apart, updating the PI feedback
        // on every ADC tick while cancelling the alternating injection current.
        let delayed =
            core::mem::replace(&mut self.half_carrier[usize::from(self.phase & 7)], current);
        self.fundamental = PhaseCurrent {
            a: ((i32::from(delayed.a) + i32::from(current.a)) / 2) as i16,
            b: ((i32::from(delayed.b) + i32::from(current.b)) / 2) as i16,
        };
        if self.phase == 8 {
            self.positive = project(current, self.angle());
        } else if self.phase == 0 {
            let negative = project(current, self.angle());
            // At entry there has been no negative half-cycle yet.
            let previous = if self.primed {
                self.previous_negative.replace(negative)
            } else {
                self.primed = true;
                None
            };
            self.carrier_valid = false;
            if let Some(previous) = previous {
                // Twice the peak-to-peak response keeps the midpoint exact in integers.
                let d = 2 * self.positive.0 - previous.0 - negative.0;
                let q = 2 * self.positive.1 - previous.1 - negative.1;
                if d < 8 || q.abs() > d {
                    self.weak_cycles += 1;
                    if self.weak_cycles >= 16 {
                        return None;
                    }
                } else {
                    self.weak_cycles = 0;
                    self.carrier_valid = true;
                    // For Ld < Lq, a positive HF q response means the rotor d-axis leads.
                    let correction = divide(q * 2048, d).clamp(-512, 512);
                    if stationary {
                        self.step_q8 = 0;
                    } else {
                        // Keep angle correction fast, but give sustained phase error
                        // less influence on velocity during current-loop handover.
                        self.step_q8 = (self.step_q8 + correction / 2).clamp(-200000, 200000);
                    }
                    self.angle_q8 = self.angle_q8.wrapping_add_signed(correction << 8);
                }
            }
        }
        let voltage = if self.phase < 8 {
            CARRIER_VOLTAGE
        } else {
            -CARRIER_VOLTAGE
        };
        self.phase = (self.phase + 1) % 16;
        self.angle_q8 = self.angle_q8.wrapping_add_signed(self.step_q8);
        Some(voltage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standstill_accepts_bounded_jitter_and_rejects_motion_or_signal_loss() {
        let mut window = StandstillWindow::new();
        for i in 0..32 {
            let angle = if i % 2 == 0 { 65400 } else { 130 };
            assert_eq!(window.update(angle, 0, true), i == 31);
        }
        for i in 0..60 {
            assert!(!window.update(i * 85, 0, true));
        }
        for i in 0..64 {
            assert!(!window.update(0, 0, i % 17 != 0));
            assert!(!window.update(32768, 0, true));
        }
    }

    #[test]
    fn current_feedback_tracks_each_sample_and_rejects_the_carrier() {
        let mut tracker = Tracker::new(0);
        for tick in 0..32i16 {
            let ripple = if tick % 16 < 8 { 30 } else { -30 };
            tracker.sample(PhaseCurrent {
                a: tick + ripple,
                b: -tick - ripple,
            });
            if tick >= 8 {
                assert_eq!(tracker.fundamental.a, tick - 4);
                assert_eq!(tracker.fundamental.b, 4 - tick);
            }
        }
    }

    #[test]
    fn fits_axis_between_peaks_in_board_scan() {
        // Automatic index run, round 7. Single-largest response is at 30 degrees.
        let positive = [70, 78, 86, 80, 82, 76, 80, 74, 65, 65, 70, 65];
        let negative = [65, 71, 78, 74, 71, 71, 75, 68, 61, 64, 69, 68];
        let mut scan = Scan::new();
        for i in 0..AXES {
            scan.positive[i] = positive[i] * REPEATS;
            scan.negative[i] = negative[i] * REPEATS;
            scan.accumulate_axis(i);
        }
        let fit = scan.detect();
        let degrees = i32::from(fit.angle & 0x7fff) * 360 / 65536;
        assert!((48..=51).contains(&degrees));
        assert!(fit.contrast >= 6);
    }

    #[test]
    fn scan_identifies_both_polarities_and_rejects_isotropic_response() {
        for (rotor, contrast, asymmetry) in [
            (3u16, 32, 10),
            (32771, 32, 10),
            (9000, 32, 10),
            (26900, 32, 10),
            (64600, 32, 10),
            (0, 0, 0),
        ] {
            let mut scan = Scan::new();
            let mut current = PhaseCurrent::default();
            loop {
                let (axis, voltage, result) = scan.sample(current);
                if let Some(result) = result {
                    if contrast == 0 {
                        assert!(result.contrast < 6 && result.polarity.abs() < 4);
                    } else {
                        assert!(result.contrast >= 6 && result.polarity.abs() >= 4);
                        assert!((result.angle.wrapping_sub(rotor) as i16).unsigned_abs() < 3000);
                    }
                    break;
                }
                let (_, alignment) = sin_cos_q15_lut(axis.wrapping_sub(rotor));
                let cos = i32::from(alignment);
                let gain = 50
                    + ((contrast * cos >> 15) * cos >> 15)
                    + (voltage.signum() * asymmetry * cos >> 15);
                let (s, c) = sin_cos_q15_lut(axis);
                let magnitude = voltage.signum() * gain;
                let alpha = magnitude * i32::from(c) >> 15;
                let beta = magnitude * i32::from(s) >> 15;
                current = PhaseCurrent {
                    a: alpha as i16,
                    b: ((beta * 56756 / 32768 - alpha) / 2) as i16,
                };
            }
        }
    }

    #[test]
    fn tracker_follows_a_rotating_saliency_axis_through_wraparound() {
        let mut tracker = Tracker::new(64800);
        let mut rotor = 65000u16;
        for tick in 0..16000 {
            let step = (tick / 80).min(180) as u16;
            let frame = tracker.angle();
            let (sin, cos) = sin_cos_q15_lut(frame);
            let (sin_err, _) = sin_cos_q15_lut(rotor.wrapping_sub(frame).wrapping_mul(2));
            let sign = if tick % 16 < 8 { -1 } else { 1 };
            let d = sign * 80;
            let q = sign * (32 * i32::from(sin_err) >> 15);
            let a = (d * i32::from(cos) - q * i32::from(sin)) >> 15;
            let beta = (d * i32::from(sin) + q * i32::from(cos)) >> 15;
            let current = PhaseCurrent {
                a: a as i16,
                b: ((beta * 56756 / 32768 - a) / 2) as i16,
            };
            assert!(tracker.sample(current).is_some());
            rotor = rotor.wrapping_add(step);
        }
        assert!((tracker.angle().wrapping_sub(rotor) as i16).unsigned_abs() < 1000);
        assert!((tracker.rpm(11) - 240).abs() < 10);
    }

    #[test]
    fn stationary_alignment_corrects_angle_without_creating_reverse_speed() {
        let mut tracker = Tracker::new(0);
        for tick in 0..1600 {
            let angle = tracker.angle();
            let (s, c) = sin_cos_q15_lut(angle);
            let (err, _) = sin_cos_q15_lut(60000u16.wrapping_sub(angle).wrapping_mul(2));
            let sign = if tick % 16 < 8 { -1 } else { 1 };
            let d = sign * 100;
            let q = sign * (24 * i32::from(err) >> 15);
            let a = (d * i32::from(c) - q * i32::from(s)) >> 15;
            let beta = (d * i32::from(s) + q * i32::from(c)) >> 15;
            let b = (beta * 56756 / 32768 - a) / 2;
            tracker
                .track(
                    PhaseCurrent {
                        a: a as i16,
                        b: b as i16,
                    },
                    true,
                    |n, d| n / d,
                )
                .unwrap();
            assert_eq!(tracker.rpm(11), 0);
        }
        assert!((tracker.angle().wrapping_sub(60000) as i16).unsigned_abs() < 1200);
        assert!(tracker.carrier_valid);
    }

    #[test]
    fn linear_current_ramp_does_not_steer_stationary_tracker() {
        let mut tracker = Tracker::new(0);
        let mut calls = 0;
        for tick in 0..=64 {
            let a = if tick % 16 < 8 { -128 } else { 128 };
            let b = -a / 2 + tick * 8;
            tracker
                .sample_with_divide(PhaseCurrent { a, b }, |numerator, denominator| {
                    calls += 1;
                    assert!(numerator.abs() <= 2 * 2048);
                    numerator / denominator
                })
                .unwrap();
        }
        assert_eq!(calls, 3);
        assert!(tracker.angle().cast_signed().unsigned_abs() < 32);
    }

    #[test]
    fn tracker_trips_on_missing_carrier_response() {
        let mut tracker = Tracker::new(0);
        // Warm up one carrier period, then reject sixteen invalid demodulations.
        for _ in 0..17 * 16 {
            assert!(tracker.sample(PhaseCurrent::default()).is_some());
        }
        assert!(tracker.sample(PhaseCurrent::default()).is_none());
    }
}

/// Compare directional saturation at two pulse strengths along the detected axis.
/// Subtract twice the half-amplitude asymmetry to reject sign-dependent linear gain.
pub struct PolarityCheck {
    amplitude: i32,
    peak_phase_counts: u16,
    angle: u16,
    phase: u8,
    slot: u8,
    baseline: i32,
    // Two independent banks, each with eight sets of four pulses.
    asymmetry: [[i32; 2]; 2],
}

impl PolarityCheck {
    pub const fn new(angle: u16) -> Self {
        Self {
            amplitude: PULSE_VOLTAGE,
            peak_phase_counts: 0,
            angle,
            phase: 0,
            slot: 0,
            baseline: 0,
            asymmetry: [[0; 2]; 2],
        }
    }

    pub fn peak_phase_counts(&self) -> u16 {
        self.peak_phase_counts
    }

    /// Retry only once, with headroom for the increased pulse current.
    pub fn retry_stronger(&mut self) -> bool {
        if self.amplitude != PULSE_VOLTAGE || self.peak_phase_counts >= 1000 {
            return false;
        }
        *self = Self::new(self.angle);
        self.amplitude = PULSE_VOLTAGE * 2;
        true
    }

    pub fn sample(&mut self, current: PhaseCurrent) -> (u16, i32, Option<(i32, i32, Option<u16>)>) {
        self.peak_phase_counts = self
            .peak_phase_counts
            .max(current.a.unsigned_abs())
            .max(current.b.unsigned_abs());
        let set = self.slot / 4;
        let high = ((self.slot / 2) ^ set) & 1 != 0;
        let positive = (self.slot ^ set) & 1 == 0;
        let magnitude = if high {
            self.amplitude
        } else {
            self.amplitude / 2
        };
        let drive = if (31..39).contains(&self.phase) {
            if positive { magnitude } else { -magnitude }
        } else {
            0
        };
        let mut result = None;
        if self.phase == 31 {
            self.baseline = project(current, self.angle).0;
        }
        if self.phase == 39 {
            // Both signed excursions add: positive peak minus magnitude of negative peak.
            self.asymmetry[(set / 8) as usize][high as usize] +=
                project(current, self.angle).0 - self.baseline;
            self.slot += 1;
            self.phase = 0;
            if self.slot == 64 {
                result = Some(self.decision());
            }
        } else {
            self.phase += 1;
        }
        (self.angle, drive, result)
    }

    fn decision(&self) -> (i32, i32, Option<u16>) {
        let [a, b] = self.asymmetry;
        let high = (a[1] + b[1]) / 16;
        let low = (a[0] + b[0]) / 16;
        let residual = |x: [i32; 2]| x[1] - 2 * x[0];
        let a = residual(a);
        let b = residual(b);
        let confirmed = if a >= 32 && b >= 32 {
            Some(self.angle)
        } else if a <= -32 && b <= -32 {
            Some(self.angle.wrapping_add(32768))
        } else {
            None
        };
        (high, low, confirmed)
    }
}

#[cfg(test)]
mod polarity_tests {
    use super::*;
    #[test]
    fn retry_resets_accumulation_and_is_bounded_by_current_and_attempts() {
        let mut p = PolarityCheck::new(1234);
        for _ in 0..2560 {
            p.sample(PhaseCurrent::default());
        }
        assert!(p.retry_stronger());
        let mut max = 0;
        for _ in 0..2560 {
            let (angle, voltage, result) = p.sample(PhaseCurrent::default());
            assert_eq!(angle, 1234);
            max = max.max(voltage.abs());
            if let Some((_, _, confirmed)) = result {
                assert!(confirmed.is_none());
            }
        }
        assert_eq!(max, 2 * PULSE_VOLTAGE);
        assert!(!p.retry_stronger());
        let mut p = PolarityCheck::new(0);
        p.sample(PhaseCurrent { a: -1000, b: 0 });
        assert!(!p.retry_stronger());
    }

    #[test]
    fn requires_repeatable_nonlinear_polarity() {
        let mut p = PolarityCheck::new(3000);
        for (banks, expected) in [
            ([[40, 80]; 2], None),
            ([[0, 40], [0, -40]], None),
            ([[8, 56]; 2], Some(3000)),
            ([[8, -24]; 2], Some(35768)),
        ] {
            p.asymmetry = banks;
            assert_eq!(p.decision().2, expected);
        }
    }
}
