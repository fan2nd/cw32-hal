use crate::foc::weakening::FieldWeakening;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::{SCB, SYST, scb::SystemHandler, syst::SystClkSource};
use cortex_m_rt::exception;
use embassy_cw32::gpio::Output;
use embassy_cw32::{
    Peri,
    interrupt::{
        Priority,
        typelevel::{BTIM1 as SpeedIrq, Interrupt},
    },
    pac::{self, interrupt},
    peripherals,
    time::Hertz,
    timer::low_level::Timer,
};
pub const SPEED_LOOP_HZ: u32 = 1000;
const SPEED_PERIOD_US: u32 = 1_000_000 / SPEED_LOOP_HZ;
pub static SPEED_TICKS: AtomicU32 = AtomicU32::new(0);
static TICKS: AtomicU32 = AtomicU32::new(0);
#[exception]
fn SysTick() {
    let tick = TICKS.load(Ordering::Relaxed);
    TICKS.store(tick.wrapping_add(1), Ordering::Release);
}

#[interrupt]
fn BTIM1() {
    static mut FW: FieldWeakening = FieldWeakening::new();
    static mut OBS: crate::foc::bemf::Bemf = crate::foc::bemf::Bemf::new();
    static mut SPEED: crate::foc::SpeedController = crate::foc::SpeedController::new(600);
    if !pac::BTIM1.isr().read().uif() {
        return;
    }
    // R1W0: clear only UIF; preserve the unrelated trigger flag.
    pac::BTIM1.icr().write(|v| v.set_tif(true));
    SPEED_TICKS.store(
        SPEED_TICKS.load(Ordering::Relaxed).wrapping_add(1),
        Ordering::Relaxed,
    );
    crate::current_loop::supervise(SPEED, FW);
    crate::current_loop::observe(OBS);
}
pub struct Timing {
    _speed_timer: Timer<'static, peripherals::BTIM1>,
    last_led: u32,
    led: Output<'static>,
}

impl Timing {
    pub fn new(led: Output<'static>, hclk: u32, timer: Peri<'static, peripherals::BTIM1>) -> Self {
        let mut core = cortex_m::Peripherals::take().unwrap();
        let syst = &mut core.SYST;
        syst.set_clock_source(SystClkSource::Core);
        syst.set_reload(hclk / SPEED_LOOP_HZ - 1);
        syst.clear_current();
        // SysTick runs below ADC1.
        unsafe { core.SCB.set_priority(SystemHandler::SysTick, 0xff) };
        SCB::clear_pendst();
        syst.enable_interrupt();
        syst.enable_counter();
        let mut timer = Timer::new(timer).unwrap();
        timer.set_frequency(Hertz(SPEED_LOOP_HZ));
        timer.clear_overflow();
        SpeedIrq::set_priority(Priority::P1);
        SpeedIrq::unpend();
        pac::BTIM1.dier().modify(|v| v.set_uie(true));
        unsafe { SpeedIrq::enable() };
        timer.start();
        Self {
            _speed_timer: timer,
            last_led: 0,
            led,
        }
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
