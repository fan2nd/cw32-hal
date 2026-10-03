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
    /// Return an immediate or displaced waker for the caller to wake after
    /// releasing its queue borrow. Like Embassy's bounded generic queue, a
    /// full queue wakes one task early to retry rather than losing a wakeup or
    /// panicking. Embassy timers recheck their deadline after every wake.
    /// The candidate is cloned before the caller borrows its driver state. An
    /// unused duplicate remains in it for the caller to drop after that borrow.
    pub(crate) fn schedule(
        &mut self,
        now: u64,
        at: u64,
        candidate: &mut Option<Waker>,
    ) -> Option<Waker> {
        if at <= now {
            return candidate.take();
        }
        if let Some(e) = self
            .entries
            .iter_mut()
            .flatten()
            .find(|e| e.waker.will_wake(candidate.as_ref().unwrap()))
        {
            e.deadline = e.deadline.min(at);
            return None;
        }
        let index = self
            .entries
            .iter()
            .position(Option::is_none)
            .unwrap_or_else(|| {
                // Keep nearer deadlines queued. Return the displaced Waker
                // instead of invoking user wake code while State is borrowed.
                self.entries
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, e)| e.as_ref().unwrap().deadline)
                    .unwrap()
                    .0
            });
        self.entries[index]
            .replace(Entry {
                deadline: at,
                waker: candidate.take().unwrap(),
            })
            .map(|entry| entry.waker)
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
