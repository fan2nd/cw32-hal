//! Single-owner IRQ event latch. Critical sections work on Cortex-M0+ without CAS.
use core::{
    cell::{Cell, RefCell},
    task::Waker,
};
use critical_section::Mutex;

pub(crate) struct EventState {
    pending: Mutex<Cell<u32>>,
    waker: Mutex<RefCell<Option<Waker>>>,
}
impl EventState {
    pub(crate) const fn new() -> Self {
        Self {
            pending: Mutex::new(Cell::new(0)),
            waker: Mutex::new(RefCell::new(None)),
        }
    }
    pub(crate) fn reset(&self) {
        let old = critical_section::with(|cs| {
            self.pending.borrow(cs).set(0);
            self.waker.borrow(cs).borrow_mut().take()
        });
        drop(old);
    }
    /// Register before inspecting the event latch. Clone and drop callbacks must
    /// never run with an outstanding RefCell borrow (a waker may be reentrant).
    pub(crate) fn register(&self, waker: &Waker) {
        let new = waker.clone();
        let old = critical_section::with(|cs| self.waker.borrow(cs).borrow_mut().replace(new));
        drop(old);
    }
    pub(crate) fn take(&self) -> u32 {
        critical_section::with(|cs| self.pending.borrow(cs).replace(0))
    }
    /// Publish within the caller's peripheral-service critical section. The
    /// returned waker can then be woken outside that section without allowing
    /// cancel/rearm to receive a stale completion from an earlier operation.
    pub(crate) fn latch(&self, bits: u32) -> Option<Waker> {
        critical_section::with(|cs| {
            let pending = self.pending.borrow(cs);
            pending.set(pending.get() | bits);
            self.waker.borrow(cs).borrow_mut().take()
        })
    }
}
