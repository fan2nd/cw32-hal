//! JD4631 field weakening at a 1 kHz supervisor rate. Currents use ADC counts.
#[derive(Default)]
pub struct FieldWeakening {
    span: i32,
    id: i16,
}

impl FieldWeakening {
    pub const fn new() -> Self {
        Self { span: 0, id: 0 }
    }

    pub fn update(&mut self, enabled: bool, duty_span: u16) -> (i16, i16) {
        self.span += (i32::from(duty_span) - self.span) / 8;
        if enabled && self.span > 25_500 {
            self.id = (self.id - 1).max(-80);
        } else if !enabled || self.span < 24_000 {
            self.id = (self.id + 1).min(0);
        }
        let iq_limit = (200_i32.pow(2) - i32::from(self.id).pow(2)).isqrt();
        (self.id, iq_limit as i16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voltage_demand_and_release_stay_within_current_circle() {
        let mut fw = FieldWeakening::new();
        for _ in 0..200 {
            assert_eq!(fw.update(false, 30_000), (0, 200));
        }
        for n in 1..=100 {
            let (id, iq) = fw.update(true, 30_000);
            assert_eq!(id, -n.min(80));
            assert!(i32::from(id).pow(2) + i32::from(iq).pow(2) <= 40_000);
        }
        for _ in 0..200 {
            fw.update(true, 24_750);
        }
        assert_eq!(fw.id, -80);
        for _ in 0..200 {
            fw.update(true, 20_000);
        }
        assert_eq!(fw.update(false, 30_000), (0, 200));
    }
}
