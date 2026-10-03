//! GTIM L012, CW32L012 RM1.4 chapter 16.
use super::TimerRegisters;
use crate::pac::gtim::{regs, vals, Gtim};

impl TimerRegisters for Gtim {
    l012_common!();

    fn initialize(self) {
        self.cr1().write_value(regs::Cr1(0));
        self.ier().write_value(regs::Ier(0));
        self.ccer().write_value(regs::Ccer(0));
        self.cr2().write_value(regs::Cr2(0));
        self.smcr().write_value(regs::Smcr(0));
        for n in 0..2 {
            self.ccmr_cmp(n).write_value(regs::CcmrCmp(0));
        }
        // URS=0 makes software preload commits unambiguous under both the
        // update-source table and CR1 prose. Interrupt/DMA enables stay off.
        self.cr1().write(|v| v.set_arpe(true));
        for n in 0..4 {
            self.ccr(n).write(|v| v.set_ccr(0));
        }
    }
    fn load(self) {
        // Caller stopped CEN. MMS=0 emits UG: gate TRGO to stopped CNT_EN,
        // then restore reset-source selection before CEN can rise.
        self.cr2().write(|v| v.set_mms(1));
        self.egr().write(|v| v.set_ug(true));
        self.icr().write(|v| v.set_uif(false));
        self.cr2().write_value(regs::Cr2(0));
    }
    fn master_output(self, _enabled: bool) {
        // GTIM has per-channel enables but no ATIM master-output gate.
    }
}
