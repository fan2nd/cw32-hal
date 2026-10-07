use anyhow::{ensure, Result};
use clap::{Args, ValueEnum};

// MT6701 Rev.1.8 section 8: snapshot all documented EEPROM configuration bytes.
pub const REGISTERS: [u8; 11] = [
    0x25, 0x29, 0x30, 0x31, 0x32, 0x33, 0x34, 0x38, 0x3e, 0x3f, 0x40,
];
pub type Registers = [u8; 11];

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Direction {
    Ccw,
    Cw,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Secondary {
    Uvw,
    InvertedAbz,
}

#[derive(Args, Debug, Default)]
pub struct Settings {
    /// AB pulses/revolution; quadrature counts/revolution = 4 * PPR
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..=1024))]
    pub ppr: Option<u16>,
    #[arg(long, value_enum)]
    pub direction: Option<Direction>,
    /// Select ABZ on the primary A/B/Z pins (clear ABZ_MUX)
    #[arg(long)]
    pub abz: bool,
    /// QFN pins U/V/W: UVW or complementary -A/-B/-Z
    #[arg(long, value_enum)]
    pub secondary: Option<Secondary>,
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=16))]
    pub pole_pairs: Option<u8>,
    /// Zero position in 1/4096 revolution units
    #[arg(long, value_parser = clap::value_parser!(u16).range(0..=4095))]
    pub zero: Option<u16>,
    #[arg(long, value_parser = ["1", "2", "4", "8", "12", "16", "180deg"])]
    pub z_width: Option<String>,
    #[arg(long, value_parser = ["0", "0.25", "0.5", "1", "2", "4", "8"])]
    pub hysteresis: Option<String>,
}

impl Settings {
    pub fn build(&self, before: Registers) -> Result<Registers> {
        ensure!(
            self.ppr.is_some()
                || self.direction.is_some()
                || self.abz
                || self.secondary.is_some()
                || self.pole_pairs.is_some()
                || self.zero.is_some()
                || self.z_width.is_some()
                || self.hysteresis.is_some(),
            "provide at least one configuration option"
        );
        let mut r = before;
        // Each mask changes only the requested field; unrelated and reserved bits survive.
        if let Some(ppr) = self.ppr {
            ensure!((1..=1024).contains(&ppr), "PPR must be 1..1024");
            let n = ppr - 1;
            r[2] = (r[2] & !0x03) | (n >> 8) as u8;
            r[3] = n as u8;
        }
        if let Some(dir) = self.direction {
            r[1] = (r[1] & !0x02) | if matches!(dir, Direction::Cw) { 2 } else { 0 };
        }
        if self.abz {
            r[1] &= !0x40;
        }
        if let Some(mode) = self.secondary {
            r[0] = (r[0] & !0x80)
                | if matches!(mode, Secondary::InvertedAbz) {
                    0x80
                } else {
                    0
                };
        }
        if let Some(n) = self.pole_pairs {
            ensure!((1..=16).contains(&n), "pole pairs must be 1..16");
            r[2] = (r[2] & !0xf0) | ((n - 1) << 4);
        }
        if let Some(zero) = self.zero {
            ensure!(zero < 4096, "zero must be 0..4095");
            r[4] = (r[4] & !0x0f) | (zero >> 8) as u8;
            r[5] = zero as u8;
        }
        if let Some(width) = &self.z_width {
            let code = ["1", "2", "4", "8", "12", "16", "180deg"]
                .iter()
                .position(|v| v == width)
                .ok_or_else(|| anyhow::anyhow!("invalid Z width"))?;
            r[4] = (r[4] & !0x70) | ((code as u8) << 4);
        }
        if let Some(hyst) = &self.hysteresis {
            let code = ["1", "2", "4", "8", "0", "0.25", "0.5"]
                .iter()
                .position(|v| v == hyst)
                .ok_or_else(|| anyhow::anyhow!("invalid hysteresis"))? as u8;
            r[4] = (r[4] & !0x80) | ((code & 4) << 5);
            r[6] = (r[6] & !0xc0) | ((code & 3) << 6);
        }
        Ok(r)
    }
}

pub fn describe(r: &Registers) {
    let ppr = (((r[2] & 3) as u16) << 8 | r[3] as u16) + 1;
    let zero = ((r[4] & 15) as u16) << 8 | r[5] as u16;
    let hyst = ((r[4] >> 5) & 4) | (r[6] >> 6);
    println!("AB: {ppr} PPR = {} quadrature counts/rev", ppr * 4);
    println!(
        "ABZ_MUX: {}; direction: {}; U/V/W: {}; pole pairs: {}",
        if r[1] & 0x40 == 0 { "ABZ" } else { "UVW" },
        if r[1] & 2 == 0 { "CCW" } else { "CW" },
        if r[0] & 0x80 == 0 { "UVW" } else { "-A/-B/-Z" },
        (r[2] >> 4) + 1
    );
    println!(
        "Zero: {zero}/4096 ({:.4} deg); Z width: {}; hysteresis: {} LSB",
        zero as f64 * 360.0 / 4096.0,
        ["1 LSB", "2 LSB", "4 LSB", "8 LSB", "12 LSB", "16 LSB", "180 deg", "1 LSB"]
            [(r[4] >> 4 & 7) as usize],
        ["1", "2", "4", "8", "0", "0.25", "0.5", "1"][hyst as usize]
    );
}

pub fn verify(expected: &Registers, actual: &Registers) -> Result<()> {
    let errors: Vec<_> = REGISTERS
        .iter()
        .zip(expected.iter().zip(actual))
        .filter(|(_, (a, b))| a != b)
        .map(|(reg, (a, b))| format!("0x{reg:02X}: expected 0x{a:02X}, got 0x{b:02X}"))
        .collect();
    ensure!(
        errors.is_empty(),
        "readback mismatch: {}",
        errors.join("; ")
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_ppr_preserves_other_fields() {
        for fill in [0, 0x55, 0xaa, 0xff] {
            for ppr in 1..=1024 {
                let old = [fill; 11];
                let r = Settings {
                    ppr: Some(ppr),
                    ..Default::default()
                }
                .build(old)
                .unwrap();
                assert_eq!((((r[2] & 3) as u16) << 8 | r[3] as u16) + 1, ppr);
                assert_eq!(r[2] & !3, old[2] & !3);
                for i in [0, 1, 4, 5, 6, 7, 8, 9, 10] {
                    assert_eq!(r[i], old[i]);
                }
            }
        }
    }
    #[test]
    fn shared_bytes_merge_without_clobbering() {
        let r = Settings {
            ppr: Some(1024),
            direction: Some(Direction::Ccw),
            abz: true,
            secondary: Some(Secondary::InvertedAbz),
            pole_pairs: Some(11),
            zero: Some(0xabc),
            z_width: Some("4".into()),
            hysteresis: Some("0.5".into()),
        }
        .build([0xff; 11])
        .unwrap();
        assert_eq!(
            r,
            [0xff, 0xbd, 0xaf, 0xff, 0xaa, 0xbc, 0xbf, 0xff, 0xff, 0xff, 0xff]
        );
        let mut wrong = r;
        wrong[2] ^= 1;
        assert!(verify(&r, &wrong).is_err());
    }
    #[test]
    fn rejects_empty_or_out_of_range() {
        assert!(Settings::default().build([0; 11]).is_err());
        for ppr in [0, 1025, u16::MAX] {
            assert!(Settings {
                ppr: Some(ppr),
                ..Default::default()
            }
            .build([0; 11])
            .is_err());
        }
    }
}
