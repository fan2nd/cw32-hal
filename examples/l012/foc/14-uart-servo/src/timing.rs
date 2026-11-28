//! SysTick owns the outer controller; the higher-priority ADC1 owns the current PI.
use crate::{
    config::{SPEED_LOOP_HZ, SPEED_PERIOD_US},
    global::{
        CURRENT_FAULT, CURRENT_IQ_REFERENCE, CURRENT_OUTPUT_ENABLE, SYSTICK_OVERRUNS, StopReason,
        record_stop,
    },
};
use core::cell::Cell;
use cortex_m::interrupt::{self, Mutex};
use embassy_cw32::{peripherals::ATIM, timer::complementary_pwm::irq_moe};

use crate::servo::Servo;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::{SCB, SYST, scb::SystemHandler, syst::SystClkSource};
use cortex_m_rt::exception;
use embassy_cw32::gpio::Output;

static HANDOFF: Mutex<Cell<Option<&'static mut Servo>>> = Mutex::new(Cell::new(None));

pub(crate) fn install(controller: &'static mut Servo) {
    interrupt::free(|cs| HANDOFF.borrow(cs).set(Some(controller)));
}
static TICKS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    static mut CONTROL: Option<&'static mut Servo> = None;
    let ticks = TICKS.load(Ordering::Relaxed);
    TICKS.store(ticks.wrapping_add(1), Ordering::Release);
    if CONTROL.is_none() {
        *CONTROL = interrupt::free(|cs| HANDOFF.borrow(cs).take());
    }
    if let Some(control) = CONTROL.as_deref_mut() {
        let start_us = Timing::micros();
        control.tick(start_us);
        // SysTick coalesces missed periods. Never continue driving with a stale tick.
        if SCB::is_pendst_pending() {
            let overruns = SYSTICK_OVERRUNS.load(Ordering::Relaxed);
            SYSTICK_OVERRUNS.store(overruns.saturating_add(1), Ordering::Relaxed);
            if CURRENT_OUTPUT_ENABLE.load(Ordering::Acquire) != 0 {
                record_stop(StopReason::Overrun);
            }
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            irq_moe::<ATIM>(false);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_FAULT.store(1, Ordering::Release);
        }
    }
}

pub struct Timing {
    last_led: u32,
    led: Output<'static>,
}

impl Timing {
    pub fn new(led: Output<'static>, hclk: u32) -> Self {
        let mut core = cortex_m::Peripherals::take().unwrap();
        let syst = &mut core.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(hclk / SPEED_LOOP_HZ - 1);
        syst.clear_current();
        // SysTick runs below ADC1.
        unsafe {
            core.SCB.set_priority(
                SystemHandler::SysTick,
                embassy_cw32::interrupt::Priority::P1 as u8,
            )
        };
        SCB::clear_pendst();
        syst.enable_interrupt();
        syst.enable_counter();
        Self { last_led: 0, led }
    }

    pub fn micros() -> u32 {
        loop {
            let ticks = TICKS.load(Ordering::Acquire);
            let remaining = SYST::get_current();
            if TICKS.load(Ordering::Acquire) == ticks {
                let sub = if remaining == 0 {
                    0
                } else {
                    let period = SYST::get_reload() + 1;
                    (period - remaining) / (period / SPEED_PERIOD_US)
                };
                return ticks.wrapping_mul(SPEED_PERIOD_US).wrapping_add(sub);
            }
        }
    }

    pub fn delay_us(&mut self, us: u32) {
        let start = Self::micros();
        while Self::micros().wrapping_sub(start) < us {
            core::hint::spin_loop();
        }
        self.heartbeat();
    }

    pub fn heartbeat(&mut self) {
        let now = TICKS.load(Ordering::Acquire);
        if now.wrapping_sub(self.last_led) >= SPEED_LOOP_HZ / 4 {
            self.led.toggle();
            self.last_led = now;
        }
    }
}
