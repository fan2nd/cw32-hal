//! ATIM L012, CW32L012 RM1.4 chapter 17.
use super::TimerRegisters;
use crate::pac::atim::{regs, Atim};

impl TimerRegisters for Atim {
    l012_common!();

    fn initialize(self) {
        self.bdtr().write_value(regs::Bdtr(0));
        self.cr1().write_value(regs::Cr1(0));
        self.dier().write_value(regs::Dier(0));
        self.ccer().write_value(regs::Ccer(0));
        self.cr2().write_value(regs::Cr2(0));
        self.smcr().write_value(regs::Smcr(0));
        self.rcr().write_value(regs::Rcr(0));
        self.dtr2().write_value(regs::Dtr2(0));
        self.af1().write_value(regs::Af1(0));
        self.af2().write_value(regs::Af2(0));
        for n in 0..3 {
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
        // Caller stopped CEN. MMS=0 emits UG: gate TRGO and TRGO2 to stopped
        // CNT_EN, then restore reset-source selection before CEN can rise.
        self.cr2().write(|v| {
            v.set_mms(1);
            v.set_mms2(1);
        });
        self.egr().write(|v| v.set_ug(true));
        self.icr().write(|v| v.set_uif(false));
        self.cr2().write_value(regs::Cr2(0));
    }
    fn master_output(self, enabled: bool) {
        // BDTR contains mixed/write-once fields: explicit read/write only.
        let mut value = self.bdtr().read();
        value.set_moe(enabled);
        self.bdtr().write_value(value);
    }
}
