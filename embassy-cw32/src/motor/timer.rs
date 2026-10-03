//! Repetitive L012 basic timers used for motor ticks and commutation delays.
use crate::{
    interrupt::typelevel::{Binding, Handler, Interrupt},
    pac, PeripheralType,
};
use core::marker::PhantomData;

mod sealed {
    pub(crate) trait Instance {
        fn regs() -> crate::pac::btim::Btim;
    }
}

/// Metadata-generated L012 basic timer identity and its actual IRQ vector.
#[allow(private_bounds)]
pub trait BasicTimerInstance:
    sealed::Instance + crate::rcc::KernelClock + PeripheralType + 'static
{
    type Interrupt: Interrupt;
}

include!(concat!(env!("OUT_DIR"), "/_generated_motor_timer.rs"));

#[derive(Clone, Copy, Debug)]
pub struct TimerConfig {
    pub prescaler: u16,
    pub reload: u16,
}
/// Exclusive timer lease; no IRQ installation, clock-rate inference or wrap compensation.
pub struct BasicTimer<T: BasicTimerInstance> {
    regs: pac::btim::Btim,
    _domain: PhantomData<(*mut (), T)>,
}
impl<T: BasicTimerInstance> BasicTimer<T> {
    /// # Safety
    /// Exclude safe timer owners, other motor handles, PAC access and nested
    /// handlers for this timer throughout the lease. The caller owns its IRQ.
    pub unsafe fn acquire() -> Self {
        Self {
            regs: T::regs(),
            _domain: PhantomData,
        }
    }
    /// Configure stopped repetitive counting; does not enable update IRQs.
    pub fn configure(&mut self, config: TimerConfig) {
        let mut clock = <T as crate::rcc::PeripheralClock>::acquire_no_reset();
        clock.pin();
        let r = self.regs;
        r.cr1().write(|_| {});
        r.dier().write(|_| {});
        r.cr2().write(|_| {});
        r.smcr().write(|_| {});
        r.psc().write(|w| w.set_psc(config.prescaler));
        r.arr().write(|w| w.set_arr(config.reload));
        r.cnt().write(|_| {});
        r.icr().write_value(pac::btim::regs::Icr(0));
    }
    /// Clear update only (W0C), preserving the trigger event flag.
    pub fn clear_update(&mut self) {
        self.regs.icr().write_value(pac::btim::regs::Icr(0x40));
    }
    /// Enable updates only after proving that `H` is bound to this timer's IRQ.
    /// The handler must check and service this timer's enabled update source.
    /// No NVIC state, priority or pending flags are changed, including on the
    /// shared BTIM3_HALLTIM vector.
    pub fn enable_update_interrupt<H: Handler<T::Interrupt>>(
        &mut self,
        _irq: impl Binding<T::Interrupt, H>,
    ) {
        self.regs.dier().write(|w| w.set_uie(true));
    }
    /// Check enabled update source and acknowledge before caller state changes.
    pub fn take_update(&mut self) -> bool {
        if !(self.regs.isr().read().uif() & self.regs.dier().read().uie()) {
            return false;
        }
        self.clear_update();
        true
    }
    /// Start repetitive mode. Intentionally does not clear pending flags.
    pub fn start(&mut self) {
        self.regs.cr1().write(|w| w.set_en(true));
    }
    pub fn stop(&mut self) {
        self.regs.cr1().write(|_| {});
    }
    pub fn counter(&mut self) -> u16 {
        self.regs.cnt().read().cnt()
    }
    pub fn preset(&mut self, ticks: u16) {
        self.regs.cnt().write(|w| w.set_cnt(ticks));
    }
    /// Reload, zero CNT, enable repetitive mode, in that order. No one-shot
    /// conversion and no implicit event clear: existing pending events survive.
    pub fn arm(&mut self, reload: u16) {
        self.regs.arr().write(|w| w.set_arr(reload));
        self.preset(0);
        self.start();
    }
}
