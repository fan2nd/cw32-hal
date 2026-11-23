//! Position demand in the signed mechanical encoder frame.
const IQ_LIMIT: i32 = 200;
const MAX_RPM: i32 = 120;
const SPEED_SCALE: i32 = 8;
const INTEGRAL_SCALE: i32 = 2048;
const SETTLE_COUNTS: i32 = 4;
const WAKE_COUNTS: i32 = 12;
const SETTLE_MS: u32 = 25;

// Accumulate a small holding bias only near rest (within 1.4 mechanical degrees).
#[derive(Default)]
pub struct PositionControl {
    holding_integral: i32,
    previous_error: i32,
    quiet_ms: u32,
    settled: bool,
}

impl PositionControl {
    pub fn settled(&self) -> bool {
        self.settled
    }

    /// Return mechanical Iq in ADC counts and the reference speed in eighths of rpm.
    pub fn update(&mut self, error: i32, rpm_x8: i32, elapsed_ms: u32) -> (i16, i32) {
        if elapsed_ms == 0 {
            *self = Self::default();
        }
        if self.settled {
            if error.abs() < WAKE_COUNTS && rpm_x8.abs() < 8 * SPEED_SCALE {
                return (0, 0);
            }
            self.settled = false;
            self.quiet_ms = 0;
        }
        if error.abs() <= SETTLE_COUNTS && rpm_x8.abs() <= 2 * SPEED_SCALE {
            self.quiet_ms += elapsed_ms.min(10);
            self.holding_integral = 0;
            if self.quiet_ms >= SETTLE_MS {
                self.settled = true;
                self.previous_error = 0;
                return (0, 0);
            }
        } else {
            self.quiet_ms = 0;
        }
        // 8 position counts request 1 rpm. Keep fractional rpm down to one count.
        let reference = error.clamp(-MAX_RPM * SPEED_SCALE, MAX_RPM * SPEED_SCALE);
        let speed_error = reference - rpm_x8;
        let proportional = speed_error * 3 / SPEED_SCALE;
        let crossed = error != 0
            && self.previous_error != 0
            && error.signum() != self.previous_error.signum();
        if elapsed_ms == 0 || crossed || error.abs() > 64 || rpm_x8.abs() > 4 * SPEED_SCALE {
            self.holding_integral = 0;
        } else if self.quiet_ms == 0 && proportional.abs() < IQ_LIMIT {
            // Ki = 5/256 Iq counts per rpm per ms. Restrict the bias so it
            // cannot mask a substantial braking request near the target.
            self.holding_integral = (self.holding_integral
                + speed_error * 5 * elapsed_ms.min(10) as i32)
                .clamp(-32 * INTEGRAL_SCALE, 32 * INTEGRAL_SCALE);
        }
        if error != 0 || elapsed_ms == 0 {
            self.previous_error = error;
        }
        if proportional != 0
            && proportional.signum() != self.holding_integral.signum()
            && (error == 0 || error.signum() == rpm_x8.signum())
            && rpm_x8.abs() > reference.abs()
        {
            self.holding_integral = 0;
        }
        let iq = proportional + self.holding_integral / INTEGRAL_SCALE;
        (iq.clamp(-IQ_LIMIT, IQ_LIMIT) as i16, reference)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responds_to_small_errors_and_brakes_before_arrival() {
        for sign in [-1, 1] {
            let mut control = PositionControl::default();
            assert_eq!(control.update(sign * 16, 0, 0).0, (sign * 6) as i16);
            for _ in 0..1000 {
                assert_eq!(
                    control.update(sign * 4096, 0, 1),
                    ((sign * 200) as i16, sign * 120 * 8)
                );
            }
            assert_eq!(
                control.update(sign * 16, sign * 20 * 8, 1).0,
                (-sign * 54) as i16
            );
        }
    }

    #[test]
    fn settling_clears_bias_and_wakes_for_displacement_or_motion() {
        for sign in [-1, 1] {
            let mut c = PositionControl::default();
            for _ in 0..1000 {
                c.update(32 * sign, 0, 1);
            }
            for _ in 0..25 {
                assert_eq!(c.update(0, 0, 1).0, 0);
            }
            assert!(c.settled());
            for _ in 0..200 {
                assert_eq!(c.update(7 * sign, sign * 12, 1), (0, 0));
            }
            assert!(c.settled());
            assert_eq!(c.update(12 * sign, 0, 1).0, (4 * sign) as i16);
            assert!(!c.settled());
            for _ in 0..25 {
                c.update(0, 0, 1);
            }
            assert_eq!(c.update(0, sign * 64, 1).0, (-24 * sign) as i16);
            assert!(!c.settled());
        }
    }

    #[test]
    fn crossing_and_reenable_clear_holding_bias() {
        let mut control = PositionControl::default();
        for _ in 0..1000 {
            control.update(32, 0, 1);
        }
        assert!(control.update(32, 0, 1).0 > 12);
        control.update(0, 0, 1);
        assert!(control.update(-8, 0, 1).0 < 0);
        assert_eq!(control.update(0, 0, 0).0, 0);
    }
}
