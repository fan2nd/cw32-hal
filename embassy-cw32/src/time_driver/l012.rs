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
//! Deadline interrupts are never intentionally early. A saturated bounded
//! queue may wake a task early to retry; Embassy timers recheck their deadline.
//! CCR is set to the actual deadline,
//! without an artificial minimum delay. A post-write time check pends the IRQ
//! if programming missed the match. Lateness still includes programming, IRQ
//! and executor delay; at a 4 MHz CPU this can exceed one timer tick.
//! This is a timestamp-resolution improvement, not a hard-real-time guarantee.
use super::core::{Hardware, ICR_MASK};
use crate::{
    interrupt::{self, typelevel::Interrupt as _},
    pac::{self, gtim::regs},
    rcc::PeripheralClock,
};

pub(super) struct Registers;
impl Hardware for Registers {
    fn counter(&mut self) -> u32 {
        pac::GTIM1.cnt().read().0
    }
    fn clear(&mut self, flags: u32) {
        pac::GTIM1.icr().write_value(regs::Icr(ICR_MASK & !flags));
    }
    fn compare_irq(&mut self, enabled: bool) {
        pac::GTIM1.ier().write(|w| {
            w.set_uie(true);
            w.set_ccie(0, enabled);
        });
    }
    fn compare(&mut self, value: u16) {
        pac::GTIM1.ccr(0).write(|w| w.set_ccr(value));
    }
    fn pend(&mut self) {
        interrupt::typelevel::GTIM1::pend();
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
    // SAFETY: HAL init owns GTIM1 and the checks above establish its tick rate.
    unsafe {
        super::driver::init(priority, || {
            let mut clock = <crate::peripherals::GTIM1 as PeripheralClock>::acquire();
            // The globally installed time driver has process-long ownership.
            clock.pin();
            pac::GTIM1.cr1().write_value(regs::Cr1(0));
            pac::GTIM1.ier().write_value(regs::Ier(0));
            pac::GTIM1.cr2().write_value(regs::Cr2(0));
            pac::GTIM1.smcr().write_value(regs::Smcr(0)); // internal PCLK, no slave trigger
            pac::GTIM1.ccer().write_value(regs::Ccer(0));
            pac::GTIM1.ccmr_cmp(0).write_value(regs::CcmrCmp(0)); // CC1 output/frozen, no preload
            pac::GTIM1.ccmr_cmp(1).write_value(regs::CcmrCmp(0));
            pac::GTIM1.psc().write(|w| w.set_psc((divider - 1) as u16));
            pac::GTIM1.arr().write(|w| w.set_arr(0xffff));
            pac::GTIM1.cnt().write_value(regs::Cnt(0));
            pac::GTIM1.ccr(0).write_value(regs::Ccr(0));
            // MMS=0 forwards UG as TRGO. Keep TRGO at stopped CNT_EN while
            // loading PSC, so initialization cannot trigger another peripheral.
            pac::GTIM1.cr2().write(|w| w.set_mms(1));
            pac::GTIM1.egr().write(|w| w.set_ug(true)); // UG loads buffered PSC, resets CNT
            pac::GTIM1.icr().write_value(regs::Icr(0)); // stopped/exclusive: clear all startup flags
            pac::GTIM1.cr2().write_value(regs::Cr2(0));
            pac::GTIM1.ccer().write(|w| w.set_cce(0, true)); // CC1 enabled, no GPIO AF configured
            pac::GTIM1.ier().write(|w| w.set_uie(true));
            // UIFREMAP=1, URS=1, CEN=1, upcounting, continuous, updates enabled.
            pac::GTIM1.cr1().write(|w| {
                w.set_uifremap(true);
                w.set_urs(true);
                w.set_cen(true);
            });
        })
    };
}
