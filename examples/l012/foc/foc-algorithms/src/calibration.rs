//! Angles use the board-oriented SSI count (14 bits) and electrical phase (16 bits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Calibration {
    pub direction: i32,
    pub pole_pairs: i32,
    pub mechanical_zero: u16,
    pub offset: u16,
}

impl Calibration {
    pub fn from_zero(direction: i32, pole_pairs: i32, mechanical_zero: u16) -> Self {
        let offset = (-direction * pole_pairs * 4 * i32::from(mechanical_zero)) as u16;
        Self {
            direction,
            pole_pairs,
            mechanical_zero,
            offset,
        }
    }

    pub fn angle(&self, mechanical: u16) -> u16 {
        let phase = self.direction * self.pole_pairs * 4 * i32::from(mechanical);
        (phase as u16).wrapping_add(self.offset)
    }

    /// Map torque in the board-oriented encoder frame into electrical q current.
    pub fn electrical_iq(&self, mechanical_iq: i16) -> i16 {
        mechanical_iq.saturating_mul(self.direction as i16)
    }

    pub fn step_for_rpm(&self, rpm: i32, pwm_hz: u32) -> i32 {
        (i64::from(self.direction * self.pole_pairs) * i64::from(rpm) * 65536
            / (60 * i64::from(pwm_hz))) as i32
    }

    // Versioned completion marker followed by the parameters and their complements.
    pub fn record(&self) -> [u8; 12] {
        let [lo, hi] = self.mechanical_zero.to_le_bytes();
        let data = [self.pole_pairs as u8, self.direction as u8, lo, hi];
        [
            b'F', b'O', b'C', 1, data[0], data[1], lo, hi, !data[0], !data[1], !lo, !hi,
        ]
    }

    pub fn decode(bytes: [u8; 12]) -> Option<Self> {
        if bytes[..4] != [b'F', b'O', b'C', 1] || (4..8).any(|i| bytes[i] != !bytes[i + 4]) {
            return None;
        }
        let pairs = i32::from(bytes[4]);
        let direction = i32::from(bytes[5] as i8);
        let zero = u16::from_le_bytes([bytes[6], bytes[7]]);
        if !(1..=64).contains(&pairs) || direction.abs() != 1 || zero >= 16384 {
            return None;
        }
        Some(Self::from_zero(direction, pairs, zero))
    }

    /// Shortest signed residual, reconstructed electrical angle minus field command.
    pub fn error(&self, mechanical: u16, command: u16) -> i16 {
        self.angle(mechanical).wrapping_sub(command) as i16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stored_parameters_survive_a_firmware_change() {
        for pairs in [1, 7, 11, 32, 64] {
            for dir in [-1, 1] {
                let c = Calibration::from_zero(dir, pairs, 16380);
                assert_eq!(Calibration::decode(c.record()), Some(c));
                for request in [-200, 0, 200] {
                    assert_eq!(i32::from(c.electrical_iq(request)), dir * i32::from(request));
                }
                assert_eq!(c.step_for_rpm(60, 16384), dir * pairs * 4);
                for n in 0..12 {
                    let mut damaged = c.record();
                    damaged[n] ^= 1;
                    assert_eq!(Calibration::decode(damaged), None);
                }
            }
        }
        assert_eq!(Calibration::decode([255; 12]), None);
    }
    #[test]
    fn mirrored_encoder_gives_same_electrical_angles_across_wrap() {
        for direction in [-1, 1] {
            let zero = (direction * 16380 & 16383) as u16;
            let calibration = Calibration::from_zero(direction, 8, zero);
            assert_eq!(calibration.error(zero, 0), 0);
            for (delta, command) in [(512, 16384), (-512, 49152)] {
                let observed = ((i32::from(zero) + direction * delta) & 16383) as u16;
                assert_eq!(calibration.error(observed, command), 0);
                let wrong = Calibration::from_zero(-direction, 8, zero);
                assert_eq!(wrong.error(observed, command), i16::MIN);
            }
        }
    }
}
