//! Shared Embassy driver, alarm queue and IRQ workflow. Counter snapshots and
//! timer initialization remain in the selected GTIM IP implementation.
#[cfg(gtim_f030)]
use super::f030::Registers;
#[cfg(gtim_l012)]
use super::l012::Registers;
use super::{core::Counter, queue::Queue};
use crate::interrupt::{self, InterruptExt};
use core::{cell::RefCell, task::Waker};
use critical_section::Mutex;
use embassy_time_driver::Driver;

struct State {
    counter: Counter,
    started: bool,
    queue: Queue,
}
struct TimerDriver {
    state: Mutex<RefCell<State>>,
}
embassy_time_driver::time_driver_impl!(static DRIVER: TimerDriver = TimerDriver {
    state: Mutex::new(RefCell::new(State { counter: Counter::new(), started: false, queue: Queue::new() }))
});
impl Driver for TimerDriver {
    fn now(&self) -> u64 {
        critical_section::with(|cs| {
            let state = self.state.borrow(cs).borrow();
            assert!(state.started, "time driver used before HAL init");
            state.counter.now(&mut Registers)
        })
    }
    fn schedule_wake(&self, at: u64, waker: &Waker) {
        let immediate = critical_section::with(|cs| {
            let mut state = self.state.borrow(cs).borrow_mut();
            assert!(state.started, "time driver used before HAL init");
            let now = state.counter.now(&mut Registers);
            let immediate = state.queue.schedule(now, at, waker);
            state
                .counter
                .arm(&mut Registers, state.queue.next_expiration());
            immediate
        });
        // Never invoke arbitrary wake code while State/RefCell is borrowed.
        if immediate {
            waker.wake_by_ref();
        }
    }
}

/// Initialize the selected IP while its state and interrupt are inaccessible.
///
/// # Safety
/// The caller must exclusively own GTIM1 and configure a running, fixed-rate
/// timer with the selected IP's counter/compare contract in `configure`.
pub(super) unsafe fn init(priority: interrupt::Priority, configure: impl FnOnce()) {
    critical_section::with(|cs| {
        let mut state = DRIVER.state.borrow(cs).borrow_mut();
        assert!(!state.started);
        interrupt::GTIM1.disable();
        configure();
        state.started = true;
        interrupt::GTIM1.unpend();
        interrupt::GTIM1.set_priority(priority);
        // SAFETY: the selected IP is configured; our handler below is installed
        // at the real GTIM1 vector by the PAC runtime.
        unsafe {
            interrupt::GTIM1.enable();
        }
    });
}

// Strong symbol overrides device.x's default handler at the actual GTIM1 vector.
// No public manual handler binding or core.SYST acquisition is needed.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
unsafe extern "C" fn GTIM1() {
    let ready = critical_section::with(|cs| {
        let mut state = DRIVER.state.borrow(cs).borrow_mut();
        state.counter.service_overflow(&mut Registers);
        let now = state.counter.now(&mut Registers);
        let ready = state.queue.take_due(now);
        state
            .counter
            .arm(&mut Registers, state.queue.next_expiration());
        ready
    });
    for waker in ready.into_iter().flatten() {
        waker.wake();
    }
}
