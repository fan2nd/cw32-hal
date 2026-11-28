//! Little endian frames. Angles are in tenths of mechanical degrees.
pub const FRAME: usize = 32;
pub const HEARTBEAT: u8 = 1;
pub const APPLY: u8 = 2;
pub const RELEASE: u8 = 3;
/// Session open releases a surviving command while retaining the fault report.
pub const CONNECT: u8 = 4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Command {
    pub mode: u8,
    pub angle: i32,
    pub rpm: i16,
    pub iq: i16,
    pub kp: i16,
    pub kd: i16,
    pub max_rpm: i16,
    pub max_iq: i16,
    pub pos_gain: i16,
    pub speed_kp: i16,
    pub speed_ki: i16,
}
impl Command {
    pub const DEFAULT: Self = Self {
        mode: 1,
        angle: 0,
        rpm: 0,
        iq: 0,
        kp: 0,
        kd: 0,
        max_rpm: 600,
        max_iq: 200,
        pos_gain: 4,
        speed_kp: 125,
        speed_ki: 6,
    };
    pub fn encode(self, buf: &mut [u8]) {
        buf[0] = self.mode;
        buf[1..5].copy_from_slice(&self.angle.to_le_bytes());
        for (field, value) in buf[5..23].chunks_exact_mut(2).zip([
            self.rpm,
            self.iq,
            self.kp,
            self.kd,
            self.max_rpm,
            self.max_iq,
            self.pos_gain,
            self.speed_kp,
            self.speed_ki,
        ]) {
            field.copy_from_slice(&value.to_le_bytes());
        }
    }
    pub fn decode(buf: &[u8]) -> Option<Self> {
        let word = |i| i16::from_le_bytes([buf[i], buf[i + 1]]);
        let c = Self {
            mode: buf[0],
            angle: i32::from_le_bytes(buf[1..5].try_into().ok()?),
            rpm: word(5),
            iq: word(7),
            kp: word(9),
            kd: word(11),
            max_rpm: word(13),
            max_iq: word(15),
            pos_gain: word(17),
            speed_kp: word(19),
            speed_ki: word(21),
        };
        ((1..=4).contains(&c.mode)
            && (-3600000..=3600000).contains(&c.angle)
            && (1..=600).contains(&c.max_rpm)
            && (1..=200).contains(&c.max_iq)
            && c.rpm.unsigned_abs() <= c.max_rpm as u16
            && c.iq.unsigned_abs() <= c.max_iq as u16
            && (0..=1000).contains(&c.kp)
            && (0..=1000).contains(&c.kd)
            && (1..=64).contains(&c.pos_gain)
            && (0..=512).contains(&c.speed_kp)
            && (0..=128).contains(&c.speed_ki))
        .then_some(c)
    }
}
const CRC_TABLE: [u16; 256] = {
    let mut table = [0; 256];
    let mut i = 0;
    while i < table.len() {
        let mut crc = (i as u16) << 8;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};
pub fn crc(bytes: &[u8]) -> u16 {
    bytes.iter().fold(0xffff, |crc, &b| {
        (crc << 8) ^ CRC_TABLE[usize::from((crc >> 8) as u8 ^ b)]
    })
}
// Gains on the wire are hundredths of Iq/degree and Iq/rpm.
pub fn mit_iq(c: Command, error_deci_deg: i32, rpm_x64: i32) -> i32 {
    let spring = i64::from(error_deci_deg) * i64::from(c.kp) * 64;
    let damping = i64::from(i32::from(c.rpm) * 64 - rpm_x64) * i64::from(c.kd) * 10;
    ((spring + damping) / 64000 + i64::from(c.iq)).clamp(-i64::from(c.max_iq), i64::from(c.max_iq))
        as i32
}

#[derive(Clone, Copy)]
pub struct State {
    pub command: Command,
    pub enabled: bool,
    pub heartbeat: Option<u32>,
    pub timed_out: bool,
}
impl State {
    pub fn expire(&mut self, now: u32) {
        if self
            .heartbeat
            .is_some_and(|t| now.wrapping_sub(t) >= 250_000)
        {
            self.heartbeat = None;
            self.timed_out = self.enabled || self.timed_out;
            self.enabled = false;
        }
    }

    pub fn accept(&mut self, op: u8, cmd: Option<Command>, now: u32, fault: bool) -> bool {
        self.expire(now);
        match op {
            HEARTBEAT => self.heartbeat = Some(now),
            CONNECT => {
                self.enabled = false;
                self.heartbeat = None;
            }
            RELEASE => {
                self.enabled = false;
                self.timed_out = false;
            }
            APPLY => {
                let Some(c) = cmd else {
                    return false;
                };
                if self.heartbeat.is_none() || fault {
                    return false;
                }
                self.command = c;
                self.enabled = true;
                self.timed_out = false;
            }
            _ => return false,
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heartbeat_expires_across_wrap_and_cannot_rearm() {
        let mut s = State {
            command: Command::DEFAULT,
            enabled: false,
            heartbeat: None,
            timed_out: false,
        };
        let t = u32::MAX - 10;
        assert!(!s.accept(APPLY, Some(Command::DEFAULT), t, false));
        s.accept(HEARTBEAT, None, t, false);
        assert!(s.accept(APPLY, Some(Command::DEFAULT), t, false));
        s.expire(t.wrapping_add(249999));
        assert!(s.enabled);
        s.expire(t.wrapping_add(250000));
        assert!(!s.enabled && s.timed_out);
        s.accept(HEARTBEAT, None, t.wrapping_add(250010), false);
        assert!(!s.enabled);
        assert!(!s.accept(APPLY, Some(Command::DEFAULT), t.wrapping_add(250011), true));
    }
    #[test]
    fn mit_multiturn_direction_feedforward_and_limit() {
        let c = Command {
            mode: 4,
            kp: 50,
            kd: 20,
            iq: 10,
            rpm: -50,
            ..Command::DEFAULT
        };
        assert_eq!(mit_iq(c, 7200, 0), 200);
        assert_eq!(mit_iq(c, -7200, 0), -200);
        assert_eq!(mit_iq(c, 200, -100 * 64), 30);
        let mut b = [0; 23];
        c.encode(&mut b);
        assert_eq!(Command::decode(&b), Some(c));
        b[15..17].copy_from_slice(&201i16.to_le_bytes());
        assert_eq!(Command::decode(&b), None);
        assert_eq!(crc(b"123456789"), 0x29b1);
    }
}
