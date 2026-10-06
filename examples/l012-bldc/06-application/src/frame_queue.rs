#![deny(unsafe_code)]
//! Bounded UART buffering owned exclusively by the application I/O task.

/// One immutable in-flight UART frame and one coalescing newest status frame.
pub struct FrameQueue {
    active: [u8; 7],
    next: usize,
    pending: Option<[u8; 7]>,
}
impl FrameQueue {
    pub const fn new() -> Self {
        Self {
            active: [0; 7],
            next: 7,
            pending: None,
        }
    }
    pub fn push(&mut self, frame: [u8; 7]) {
        if self.next == 7 {
            self.active = frame;
            self.next = 0;
        } else {
            self.pending = Some(frame);
        }
    }
    pub fn pop_byte(&mut self) -> Option<u8> {
        if self.next == 7 {
            return None;
        }
        let byte = self.active[self.next];
        self.next += 1;
        if self.next == 7 {
            if let Some(frame) = self.pending.take() {
                self.active = frame;
                self.next = 0;
            }
        }
        Some(byte)
    }
}

impl Default for FrameQueue {
    fn default() -> Self {
        Self::new()
    }
}
