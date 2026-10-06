#![deny(unsafe_code)]
//! 容量有界的 UART 缓冲，由应用 I/O 任务独占。

/// 一个发送期间不可修改的 UART 帧，以及一个合并更新的最新状态帧。
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
