//! SysTick owns the outer controller; the higher-priority ADC1 owns the current PI.
use crate::{
    config::{POT_ZERO_COUNTS, PWM_HZ, READY_ZERO_SAMPLES},
    control::ArmingGate,
    global::{CURRENT_FAULT, CURRENT_IQ_REFERENCE, CURRENT_OUTPUT_ENABLE},
};
use core::cell::Cell;
use cortex_m::interrupt::{self, Mutex};
use embassy_cw32::{
    adc::adc_irq_result,
    peripherals::{ADC2, ATIM},
    timer::complementary_pwm::irq_moe,
};

use crate::position::PositionLoop;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::{SCB, SYST, scb::SystemHandler, syst::SystClkSource};
use cortex_m_rt::exception;
use embassy_cw32::gpio::Output;

static HANDOFF: Mutex<Cell<Option<&'static mut PositionLoop>>> = Mutex::new(Cell::new(None));

pub(crate) fn install(controller: &'static mut PositionLoop) {
    interrupt::free(|cs| HANDOFF.borrow(cs).set(Some(controller)));
}
static MILLIS: AtomicU32 = AtomicU32::new(0);

#[exception]
fn SysTick() {
    static mut CONTROL: Option<&'static mut PositionLoop> = None;
    static mut ARM: ArmingGate = ArmingGate::new(READY_ZERO_SAMPLES / (PWM_HZ / 1000));
    static mut ARMED: bool = false;
    let ticks = MILLIS.load(Ordering::Relaxed);
    MILLIS.store(ticks.wrapping_add(1), Ordering::Release);
    if CONTROL.is_none() {
        *CONTROL = interrupt::free(|cs| HANDOFF.borrow(cs).take());
    }
    if let Some(control) = CONTROL.as_deref_mut() {
        if !*ARMED {
            // A missing ADC result cannot count toward the zero gesture.
            *ARMED = ARM.observe(
                adc_irq_result::<ADC2>(1).unwrap_or(u16::MAX),
                POT_ZERO_COUNTS,
            );
            return;
        }
        control.tick(Timing::micros());
        // SysTick coalesces missed periods. Never continue driving with a stale tick.
        if SCB::is_pendst_pending() {
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            irq_moe::<ATIM>(false);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_FAULT.store(1, Ordering::Release);
            crate::telemetry::publish(crate::telemetry::Report::Overrun);
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
        syst.set_reload(hclk / 1000 - 1);
        syst.clear_current();
        // SysTick runs below ADC1.
        unsafe { core.SCB.set_priority(SystemHandler::SysTick, 0xff) };
        SCB::clear_pendst();
        syst.enable_interrupt();
        syst.enable_counter();
        Self { last_led: 0, led }
    }

    pub fn micros() -> u32 {
        loop {
            let ms = MILLIS.load(Ordering::Acquire);
            let remaining = SYST::get_current();
            if MILLIS.load(Ordering::Acquire) == ms {
                let sub = if remaining == 0 {
                    0
                } else {
                    let period = SYST::get_reload() + 1;
                    (period - remaining) / (period / 1000)
                };
                return ms.wrapping_mul(1000).wrapping_add(sub);
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
        let now = MILLIS.load(Ordering::Acquire);
        if now.wrapping_sub(self.last_led) >= 250 {
            self.led.toggle();
            self.last_led = now;
        }
    }
}
