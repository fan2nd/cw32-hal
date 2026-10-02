//! Fixed-size scheduler used by the GTIM hardware time backend.
use core::task::Waker;
pub(crate) const CAPACITY: usize = 16;
struct Entry {
    deadline: u64,
    waker: Waker,
}
pub(crate) struct Queue {
    entries: [Option<Entry>; CAPACITY],
}
impl Queue {
    pub(crate) const fn new() -> Self {
        Self {
            entries: [const { None }; CAPACITY],
        }
    }
    /// Returns true when the caller should wake immediately.
    pub(crate) fn schedule(&mut self, now: u64, at: u64, waker: &Waker) -> bool {
        if at <= now {
            return true;
        }
        if let Some(e) = self
            .entries
            .iter_mut()
            .flatten()
            .find(|e| e.waker.will_wake(waker))
        {
            e.deadline = e.deadline.min(at);
            return false;
        }
        let slot = self
            .entries
            .iter_mut()
            .find(|s| s.is_none())
            .expect("embassy-cw32 time queue full (16 distinct wakers)");
        *slot = Some(Entry {
            deadline: at,
            waker: waker.clone(),
        });
        false
    }
    pub(crate) fn next_expiration(&self) -> Option<u64> {
        self.entries.iter().flatten().map(|e| e.deadline).min()
    }
    pub(crate) fn take_due(&mut self, now: u64) -> [Option<Waker>; CAPACITY] {
        let mut ready = [const { None }; CAPACITY];
        for (slot, output) in self.entries.iter_mut().zip(ready.iter_mut()) {
            if slot.as_ref().is_some_and(|e| e.deadline <= now) {
                *output = slot.take().map(|e| e.waker);
            }
        }
        ready
    }
}
