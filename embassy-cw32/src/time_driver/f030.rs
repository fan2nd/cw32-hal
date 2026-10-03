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
//! Deadline interrupts are never intentionally early. A saturated bounded
//! queue may wake a task early to retry; Embassy timers recheck their deadline.
//! CCR is set to the actual deadline,
//! without an artificial minimum delay. A post-write time check pends the IRQ
//! if programming missed the match. Lateness still includes programming, IRQ
//! and executor delay; at an 8 MHz CPU this can exceed one timer tick.
//! This is a timestamp-resolution improvement, not a hard-real-time guarantee.
use super::core::{prescaler, Hardware, ICR_FLAGS, ICR_MASK};
use crate::{
    interrupt::{self, typelevel::Interrupt as _},
    pac::{self, gtim::regs},
    rcc::PeripheralClock,
};

pub(super) struct Registers;
impl Hardware for Registers {
    fn flags(&mut self) -> u32 {
        pac::GTIM1.isr().read().0
    }
    fn counter(&mut self) -> u16 {
        pac::GTIM1.cnt().read().0 as u16
    }
    fn clear(&mut self, flags: u32) {
        pac::GTIM1.icr().write_value(regs::Icr(ICR_MASK & !flags));
    }
    fn compare_irq(&mut self, enabled: bool) {
        pac::GTIM1.ier().write(|w| {
            w.set_ov(true);
            w.set_cc1(enabled);
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
/// Genuine CW32F030; exclusive initial ownership of GTIM1; fixed running PCLK.
pub(crate) unsafe fn init(clocks: crate::rcc::Clocks, priority: interrupt::Priority) {
    let hz = embassy_time_driver::TICK_HZ as u32;
    assert_eq!(hz, 1_000_000, "F030 time driver requires 1 MHz ticks");
    assert!(clocks.pclk >= hz && clocks.pclk % hz == 0);
    let prs = prescaler(clocks.pclk / hz).expect("unsupported F030 GTIM power-of-two divider");
    // SAFETY: HAL init owns GTIM1 and the checks above establish its tick rate.
    unsafe {
        super::driver::init(priority, || {
            <crate::peripherals::GTIM1 as PeripheralClock>::enable_and_reset();
            pac::GTIM1.cr0().write_value(regs::Cr0(0)); // stopped, PCLK source, no encoder/trigger
            pac::GTIM1.ier().write_value(regs::Ier(0));
            pac::GTIM1.dma().write_value(regs::Dma(0));
            pac::GTIM1.cr1().write_value(regs::Cr1(0));
            pac::GTIM1.etr().write_value(regs::Etr(0));
            pac::GTIM1.cmmr().write_value(regs::Cmmr(0));
            pac::GTIM1.arr().write(|w| w.set_arr(0xffff));
            pac::GTIM1.cnt().write_value(regs::Cnt(0));
            pac::GTIM1.ccr(0).write(|w| w.set_ccr(0xffff));
            // RM 14.3.4.1: CC1M=0xA sets CC1 on CNT==CCR1. No GPIO AF is
            // configured: only the internal comparator is used, not an output pin.
            pac::GTIM1.cmmr().write(|w| w.set_cc1m(0x0a));
            // W0 clears all implemented startup flags; preserve reserved reset bits.
            pac::GTIM1
                .icr()
                .write_value(regs::Icr(ICR_MASK & !ICR_FLAGS));
            pac::GTIM1.ier().write(|w| w.set_ov(true));
            // F030 has CR0.PRS (2^n), not an L012 PSC register or UIFREMAP.
            // EN's 0->1 transition loads PRS into PRSSTATUS (RM 14.8.1), so no
            // software update event is required. Reset PCLK 8 MHz /8 = 1 MHz.
            pac::GTIM1.cr0().write(|w| {
                w.set_prs(prs as u8);
                w.set_en(true);
            });
        })
    };
}
