//! CW32F030 GTIM1 monotonic clock: 1 MHz nominal, 64-bit epoch, one IRQ per 65.536 ms
//! plus deadline IRQs. RM Rev 2.5 §§14.3.1,14.3.4.1,14.8.1,14.8.6,14.8.11–13.
//!
//! This reserves all of GTIM1, including its IRQ and channels, but no GPIO or
//! ATIM. The timer keeps counting during ordinary WFI (PCLK must stay running).
//! STOP, clock changes and debugger halts with a running timer are unsupported.
//! The interval from each overflow until its OV acknowledgment MUST be strictly
//! less than 65.536 ms, including critical sections, higher-priority interrupts,
//! flash stalls, and wakers executing inside this IRQ. A one-bit OV cannot
//! recover two missed wraps. All clock accuracy remains that of the HSI source.
//!
//! Alarms are never intentionally early. CCR is set to the actual deadline,
//! without an artificial minimum delay. A post-write time check pends the IRQ
//! if programming missed the match. Lateness still includes programming, IRQ
//! and executor delay; at an 8 MHz CPU this can exceed one timer tick.
//! This is a timestamp-resolution improvement, not a hard-real-time guarantee.
use super::core::{prescaler, Counter, Hardware, COMPARE, ICR_FLAGS, ICR_MASK, UPDATE};
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
    fn flags(&mut self) -> u32 {
        Self::read(pac::gtim::ISR)
    }
    fn counter(&mut self) -> u16 {
        Self::read(pac::gtim::CNT) as u16
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
/// Genuine CW32F030; exclusive initial ownership of GTIM1; fixed running PCLK.
pub(crate) unsafe fn init(clocks: crate::rcc::Clocks, priority: interrupt::Priority) {
    let hz = embassy_time_driver::TICK_HZ as u32;
    assert_eq!(hz, 1_000_000, "F030 time driver requires 1 MHz ticks");
    assert!(clocks.pclk >= hz && clocks.pclk % hz == 0);
    let prs = prescaler(clocks.pclk / hz).expect("unsupported F030 GTIM power-of-two divider");
    critical_section::with(|cs| {
        let mut state = DRIVER.state.borrow(cs).borrow_mut();
        assert!(!state.started);
        interrupt::GTIM1.disable();
        <crate::peripherals::GTIM1 as PeripheralClock>::enable_and_reset();
        Registers::write(pac::gtim::CR0, 0); // stopped, PCLK source, no encoder/trigger
        Registers::write(pac::gtim::IER, 0);
        Registers::write(pac::gtim::DMA, 0);
        Registers::write(pac::gtim::CR1, 0);
        Registers::write(pac::gtim::ETR, 0);
        Registers::write(pac::gtim::CMMR, 0);
        Registers::write(pac::gtim::ARR, 0xffff);
        Registers::write(pac::gtim::CNT, 0);
        Registers::write(pac::gtim::CCR1, 0xffff);
        // RM 14.3.4.1: CC1M=0xA sets CC1 on CNT==CCR1. No GPIO AF is
        // configured: only the internal comparator is used, not an output pin.
        Registers::write(
            pac::gtim::CMMR,
            pac::gtim::fields::cmmr::CC1M.write(0, 0x0a),
        );
        // W0 clears all implemented startup flags; preserve reserved reset bits.
        Registers::write(pac::gtim::ICR, ICR_MASK & !ICR_FLAGS);
        Registers::write(pac::gtim::IER, UPDATE);
        // F030 has CR0.PRS (2^n), not an L012 PSC register or UIFREMAP.
        // EN's 0->1 transition loads PRS into PRSSTATUS (RM 14.8.1), so no
        // software update event is required. Reset PCLK 8 MHz /8 = 1 MHz.
        Registers::write(
            pac::gtim::CR0,
            pac::gtim::fields::cr0::EN.write(pac::gtim::fields::cr0::PRS.write(0, prs), true),
        );
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
