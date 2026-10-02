#![deny(unsafe_code)]
//! Bounded messages between the motor IRQ owner and slow application tasks.
//! No peripheral handle, register pointer or controller reference crosses here.

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiFeedback {
    pub led_on: Option<bool>,
    pub telemetry: Option<[u8; 7]>,
}

impl UiFeedback {
    /// Latest complete message wins while a consumer is delayed. A frame is
    /// copied as seven bytes, never interleaved with an older partial frame.
    pub fn merge(&mut self, newer: Self) {
        if newer.led_on.is_some() {
            self.led_on = newer.led_on;
        }
        if newer.telemetry.is_some() {
            self.telemetry = newer.telemetry;
        }
    }
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiCommand {
    key_pressed: bool,
    age_ms: u16,
}

impl UiCommand {
    pub const fn default_const() -> Self {
        Self {
            key_pressed: false,
            age_ms: 0,
        }
    }
    pub fn update(&mut self, key_pressed: bool) {
        self.key_pressed = key_pressed;
        self.age_ms = 0;
    }
    pub fn tick_1ms(&mut self) {
        self.age_ms = self.age_ms.saturating_add(1);
    }
    // Debounce requires a stream of fresh observations. One pressed sample
    // followed by a stalled task must never masquerade as a 60 ms key hold.
    pub const fn key_pressed(&self) -> bool {
        self.key_pressed && self.age_ms < 2
    }
    /// Losing the UI task is a software fault during active motor operation.
    pub const fn is_stale(&self) -> bool {
        self.age_ms >= 100
    }
    pub const fn requires_stop(&self, motor_enabled: bool, requested_percent: u8) -> bool {
        self.is_stale() && (motor_enabled || requested_percent != 0)
    }
}
