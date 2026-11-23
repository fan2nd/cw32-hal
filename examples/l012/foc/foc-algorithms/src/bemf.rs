//! Voltage-model back EMF for the JD4631 board. A complete injection cycle is 1 ms.
use crate::{Duty, JD4631, PhaseCurrent};

#[derive(Clone, Copy, Default)]
pub struct Window {
    volts: [i32; 2],
    amps: [i32; 2],
    delta: [i32; 2],
}

pub struct Sampler {
    window: Window,
    count: u8,
    first: PhaseCurrent,
}

impl Sampler {
    pub const fn new() -> Self {
        Self {
            window: Window {
                volts: [0; 2],
                amps: [0; 2],
                delta: [0; 2],
            },
            count: 0,
            first: PhaseCurrent { a: 0, b: 0 },
        }
    }
    pub fn sample(&mut self, duty: Duty, i: PhaseCurrent) -> Option<Window> {
        let (a, b, c) = (i32::from(duty.a), i32::from(duty.b), i32::from(duty.c));
        self.window.volts[0] += a - b;
        self.window.volts[1] += b - c;
        self.window.amps[0] += i32::from(i.a);
        self.window.amps[1] += i32::from(i.b);
        self.count += 1;
        if self.count < 16 {
            return None;
        }
        self.count = 0;
        self.window.delta = [
            i32::from(i.a) - i32::from(self.first.a),
            i32::from(i.b) - i32::from(self.first.b),
        ];
        self.first = i;
        Some(core::mem::take(&mut self.window))
    }
}

pub struct Bemf {
    previous: Option<u16>,
    angle_q4: u32,
    step: i32, // Q16 electrical turns per millisecond
    pub valid: bool,
    pub magnitude_mv: i32,
}

impl Bemf {
    pub const fn new() -> Self {
        Self {
            previous: None,
            angle_q4: 0,
            step: 0,
            valid: false,
            magnitude_mv: 0,
        }
    }

    pub fn angle(&self) -> u16 {
        (self.angle_q4 >> 4) as u16
    }
    pub fn electrical_step(&self) -> i32 {
        self.step
    }
    pub fn rpm(&self) -> i32 {
        self.step * 60000 / (65536 * i32::from(JD4631.pole_pairs))
    }

    /// Duty that produced the sample, including injection and any SVPWM saturation.
    /// Positive mechanical commands on this board give negative electrical rotation.
    pub fn update(&mut self, w: Window, bus_mv: i32) {
        self.angle_q4 = self.angle_q4.wrapping_add_signed(self.step * 16);
        let ab = (w.volts[0] / 16 * bus_mv) >> 15;
        let bc = (w.volts[1] / 16 * bus_mv) >> 15;
        // 2.442 mA/count. R*i in mV and L*delta_i/1ms in mV.
        let drop = |sum: i32, delta: i32| (sum * 1047 >> 10) + (delta * 4376 >> 10);
        let ra = drop(w.amps[0], w.delta[0]);
        let rb = drop(w.amps[1], w.delta[1]);
        let alpha = (2 * ab + bc) / 3 - ra;
        let beta = ((bc - ra - 2 * rb) * 18919) >> 15;
        self.magnitude_mv = alpha.abs().max(beta.abs());
        if self.magnitude_mv < 350 {
            self.valid = false;
            self.previous = None;
            self.step = 0;
            return;
        }
        let measured = angle_of(-beta, alpha);
        let old = self.previous.replace(measured);
        self.valid = false;
        if let Some(old) = old {
            let delta = i32::from(measured.wrapping_sub(old) as i16);
            if delta.abs() < 11000 {
                self.step += (delta - self.step) / 8;
                // The reconstructed voltage is centered half a window in the past.
                let present = measured.wrapping_add_signed((self.step / 2) as i16);
                let error = i32::from(present.wrapping_sub(self.angle()) as i16);
                self.angle_q4 = self.angle_q4.wrapping_add_signed(error * 4);
                self.valid = error.abs() < 5461;
            }
        } else {
            self.angle_q4 = u32::from(measured) << 4;
        }
    }
}

fn angle_of(x: i32, y: i32) -> u16 {
    let (ax, ay) = (x.abs(), y.abs());
    let r = ax.min(ay) * 1024 / ax.max(ay).max(1);
    // atan(r) on [0,1] in binary angle units.
    let octant = (r * (8192 + (2848 * (1024 - r) >> 10))) >> 10;
    let first = if ay > ax { 16384 - octant } else { octant };
    let upper = if x < 0 { 32768 - first } else { first };
    (if y < 0 { -upper } else { upper }) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tracks_negative_rotation_from_pwm_alone() {
        let mut obs = Bemf::new();
        let mut sampler = Sampler::new();
        let mut theta = 0u16;
        for _ in 0..4000 {
            let (s, c) = crate::sin_cos_q15_lut(theta);
            let d = crate::svpwm(i32::from(s) / 2, -i32::from(c) / 2);
            if let Some(w) = sampler.sample(d, PhaseCurrent { a: 0, b: 0 }) {
                obs.update(w, 12000);
            }
            theta = theta.wrapping_sub(225);
        }
        assert!(obs.valid);
        assert!((obs.rpm() + 300).abs() < 5);
        assert!((obs.angle().wrapping_sub(theta) as i16).unsigned_abs() < 1200);
        for _ in 0..16 {
            if let Some(w) = sampler.sample(crate::svpwm(0, 0), PhaseCurrent { a: 0, b: 0 }) {
                obs.update(w, 12000);
            }
        }
        assert!(!obs.valid);
    }
}
