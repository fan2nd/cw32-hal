//! GTIM1 monotonic clock: 1 MHz nominal, 64-bit epoch, one IRQ per 65.536 ms
//! plus deadline IRQs. RM v1.4 §§16.3.1,16.3.5,16.10.6,16.10.14–16.
//!
//! This reserves all of GTIM1, including its IRQ and channels, but no GPIO or
//! ATIM. The timer keeps counting during ordinary WFI (PCLK must stay running).
//! STOP, clock changes and debugger halts with a running timer are unsupported.
//! The interval from each overflow until its UIF acknowledgment MUST be strictly
//! less than 65.536 ms, including critical sections, higher-priority interrupts,
//! flash stalls, and wakers executing inside this IRQ. A one-bit UIF cannot
//! recover two missed wraps. All clock accuracy remains that of the HSI source.
//!
//! Alarms are never intentionally early. CCR is set to the actual deadline,
//! without an artificial minimum delay. A post-write time check pends the IRQ
//! if programming missed the match. Lateness still includes programming, IRQ
//! and executor delay; at a 4 MHz CPU this can exceed one timer tick.
//! This is a timestamp-resolution improvement, not a hard-real-time guarantee.
use super::core::{Counter, Hardware, COMPARE, ICR_MASK, UPDATE};
use crate::{
    interrupt::{self, InterruptExt},
    pac,
    rcc::PeripheralClock,
    time_driver::queue::Queue,
};
use core::{cell::RefCell, task::Waker};
use critical_section::Mutex;
use embassy_time_driver::Driver;

struct Registers;
impl Registers {
    fn read(offset: usize) -> u32 {
        // SAFETY: only initialized, exclusively reserved GTIM1 is accessed.
        unsafe { pac::read(pac::GTIM1_BASE + offset) }
    }
    fn write(offset: usize, value: u32) {
        // SAFETY: exclusive ownership and CS serialization; values follow RM.
        unsafe { pac::write(pac::GTIM1_BASE + offset, value) }
    }
}
impl Hardware for Registers {
    fn counter(&mut self) -> u32 {
        Self::read(pac::gtim::CNT)
    }
    fn clear(&mut self, flags: u32) {
        Self::write(pac::gtim::ICR, ICR_MASK & !flags);
    }
    fn compare_irq(&mut self, enabled: bool) {
        Self::write(pac::gtim::IER, UPDATE | if enabled { COMPARE } else { 0 });
    }
    fn compare(&mut self, value: u16) {
        Self::write(pac::gtim::CCR1, u32::from(value));
    }
    fn pend(&mut self) {
        interrupt::GTIM1.pend();
    }
}
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

/// Called once by HAL init after the clock tree has been verified. GTIM1 is
/// excluded from the returned singleton set when this driver is enabled.
/// # Safety
/// Genuine CW32L012; exclusive initial ownership of GTIM1; fixed running PCLK.
pub(crate) unsafe fn init(clocks: crate::rcc::Clocks, priority: interrupt::Priority) {
    let hz = embassy_time_driver::TICK_HZ as u32;
    assert!(clocks.pclk >= hz && clocks.pclk % hz == 0);
    let divider = clocks.pclk / hz;
    assert!(divider <= 65536);
    critical_section::with(|cs| {
        let mut state = DRIVER.state.borrow(cs).borrow_mut();
        assert!(!state.started);
        interrupt::GTIM1.disable();
        <crate::peripherals::GTIM1 as PeripheralClock>::enable_and_reset();
        Registers::write(pac::gtim::CR1, 0);
        Registers::write(pac::gtim::IER, 0);
        Registers::write(pac::gtim::CR2, 0);
        Registers::write(pac::gtim::SMCR, 0); // internal PCLK, no slave trigger
        Registers::write(pac::gtim::CCER, 0);
        Registers::write(pac::gtim::CCMR1CMP, 0); // CC1 output/frozen, no preload
        Registers::write(pac::gtim::CCMR2CMP, 0);
        Registers::write(pac::gtim::PSC, divider - 1);
        Registers::write(pac::gtim::ARR, 0xffff);
        Registers::write(pac::gtim::CNT, 0);
        Registers::write(pac::gtim::CCR1, 0);
        Registers::write(pac::gtim::EGR, 1); // UG loads buffered PSC, resets CNT
        Registers::write(pac::gtim::ICR, 0); // stopped/exclusive: clear all startup flags
        Registers::write(pac::gtim::CCER, 1); // CC1 enabled, no GPIO AF configured
        Registers::write(pac::gtim::IER, UPDATE);
        // UIFREMAP=1, URS=1, CEN=1, upcounting, continuous, updates enabled.
        Registers::write(pac::gtim::CR1, (1 << 11) | (1 << 2) | 1);
        state.started = true;
        interrupt::GTIM1.unpend();
        interrupt::GTIM1.set_priority(priority);
        // SAFETY: our real GTIM1 handler below is installed by the PAC runtime.
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
