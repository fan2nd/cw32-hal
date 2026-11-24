//! 16 kHz open-loop start and handoff for the 11-pole-pair JD4631.
use crate::{
    FocController, JD4631, PhaseCurrent,
    hfi::{Tracker, nearest_axis_angle, project},
    sin_cos_q15_lut,
};

pub const HANDOFF_RPM: i32 = 96;
pub struct Drag {
    ticks: u16,
    field: u16,
    agreement_ms: u16,
    pub pole_offset: u16,
    fade: u16,
    initial_dq: (i16, i16),
    remaining: u16,
}

pub struct Output {
    pub voltage: (i32, i32),
    pub rpm: Option<i32>,
    pub ready: bool,
}

fn voltage(angle: u16, amplitude: i32) -> (i32, i32) {
    let (sin, cos) = sin_cos_q15_lut(angle);
    (
        i32::from(cos) * amplitude >> 15,
        i32::from(sin) * amplitude >> 15,
    )
}

impl Drag {
    pub const fn new() -> Self {
        Self {
            ticks: 0,
            field: 0,
            agreement_ms: 0,
            pole_offset: 0,
            fade: 0,
            initial_dq: (0, 0),
            remaining: 0,
        }
    }

    pub fn tick(&mut self, phase: PhaseCurrent, tracker: &mut Tracker) -> Result<Output, u16> {
        self.ticks += 1;
        if self.ticks >= 64_000 {
            return Err(14);
        }
        let step = u32::from(self.ticks.saturating_sub(8000)).min(32000) * 72 / 32000;
        self.field = self.field.wrapping_sub(step as u16);
        let carrier = tracker
            .track(phase, self.ticks <= 8000, |a, b| a / b)
            .ok_or(7u16)?;
        let angle = tracker.angle();
        let rpm = (self.ticks % 16 == 0).then(|| -tracker.rpm(i32::from(JD4631.pole_pairs)));
        if let Some(rpm) = rpm.filter(|_| self.ticks >= 40000) {
            let north = nearest_axis_angle(angle, self.field);
            let pole = north.wrapping_sub(angle);
            let valid = tracker.carrier_valid
                && (north.wrapping_sub(self.field) as i16).unsigned_abs() < 8192
                && (78..=114).contains(&rpm);
            self.agreement_ms = if valid && self.pole_offset == pole {
                self.agreement_ms + 1
            } else {
                0
            };
            self.pole_offset = pole;
        }
        let ready = self.agreement_ms >= 48;
        if ready {
            self.fade = 512;
        }
        let (a, b) = voltage(self.field, (i32::from(self.ticks) * 8192 / 1600).min(8192));
        let (x, y) = voltage(angle, carrier);
        Ok(Output {
            voltage: (a + x, b + y),
            rpm,
            ready,
        })
    }

    /// Match the rotating-frame PI voltage and load current at the handoff.
    pub fn prime(&mut self, current: &mut FocController, tracker: &Tracker) {
        let angle = tracker.angle().wrapping_add(self.pole_offset);
        let (vd, vq) = voltage(self.field.wrapping_sub(angle), 8192);
        current.id_pi.integral = vd << 15;
        current.iq_pi.integral = vq << 15;
        let (d, q) = project(tracker.fundamental, angle);
        // Keep the initial reference inside the 200-count current circle.
        let sum = (d.abs() + q.abs()).max(200);
        self.initial_dq = ((d * 200 / sum) as i16, (q * 200 / sum) as i16);
        self.remaining = 2048;
    }

    pub fn references(&mut self, d: i16, q: i16) -> (i16, i16) {
        if self.remaining == 0 {
            return (d, q);
        }
        let w = i32::from(self.remaining);
        self.remaining -= 1;
        let mix =
            |a: i16, b: i16| (i32::from(b) + ((i32::from(a) - i32::from(b)) * w >> 11)) as i16;
        (mix(self.initial_dq.0, d), mix(self.initial_dq.1, q))
    }

    pub fn blend(&mut self, a: i32, b: i32) -> (i32, i32) {
        if self.fade == 0 {
            return (a, b);
        }
        self.field = self.field.wrapping_sub(72);
        let (oa, ob) = voltage(self.field, 8192);
        let w = i32::from(self.fade);
        self.fade -= 1;
        (a + ((oa - a) * w >> 9), b + ((ob - b) * w >> 9))
    }
}
