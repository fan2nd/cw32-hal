//! CW32L012 RM1.4 chapters 16/17. These distinct PAC blocks share audited
//! counter/compare semantics; they are not cast to another chip's register map.
use super::{low_level::Config, Prescalers, Registers};
use crate::pac::{atim::regs as a, gtim::regs as g};

macro_rules! both {
    ($self:expr, $r:ident, $body:block) => {
        match $self {
            Registers::Atim($r) => $body,
            Registers::Gtim($r) => $body,
        }
    };
}
impl Registers {
    pub(crate) fn prescalers(self) -> Prescalers {
        Prescalers::Linear
    }
    pub(crate) fn initialize(self) {
        match self {
            Self::Atim(r) => {
                r.bdtr().write_value(a::Bdtr(0));
                r.cr1().write_value(a::Cr1(0));
                r.dier().write_value(a::Dier(0));
                r.ccer().write_value(a::Ccer(0));
                r.cr2().write_value(a::Cr2(0));
                r.smcr().write_value(a::Smcr(0));
                r.rcr().write_value(a::Rcr(0));
                r.dtr2().write_value(a::Dtr2(0));
                r.af1().write_value(a::Af1(0));
                r.af2().write_value(a::Af2(0));
                for n in 0..3 {
                    r.ccmr_cmp(n).write_value(a::CcmrCmp(0));
                }
            }
            Self::Gtim(r) => {
                r.cr1().write_value(g::Cr1(0));
                r.ier().write_value(g::Ier(0));
                r.ccer().write_value(g::Ccer(0));
                r.cr2().write_value(g::Cr2(0));
                r.smcr().write_value(g::Smcr(0));
                for n in 0..2 {
                    r.ccmr_cmp(n).write_value(g::CcmrCmp(0));
                }
            }
        }
        // URS=0 makes software preload commits unambiguous under both the
        // update-source table and CR1 prose. All interrupt/DMA enables stay off.
        both!(self, r, {
            r.cr1().write(|v| {
                v.set_arpe(true);
            });
            for n in 0..4 {
                r.ccr(n).write(|v| v.set_ccr(0));
            }
        });
    }
    pub(crate) fn set_running(self, on: bool) {
        both!(self, r, {
            r.cr1().modify(|v| v.set_cen(on));
        });
    }
    pub(crate) fn is_running(self) -> bool {
        both!(self, r, { r.cr1().read().cen() })
    }
    pub(crate) fn counter(self) -> u16 {
        both!(self, r, { r.cnt().read().cnt() })
    }
    pub(crate) fn set_counter(self, value: u16) {
        both!(self, r, {
            r.cnt().write(|v| v.set_cnt(value));
        });
    }
    pub(crate) fn write_timing(self, c: Config) {
        both!(self, r, {
            r.psc()
                .write(|v| v.set_psc((c.prescaler_divisor - 1) as u16));
            r.arr().write(|v| v.set_arr((c.period_ticks - 1) as u16));
            r.cnt().write(|v| v.set_cnt(0));
        });
    }
    /// Caller has stopped CEN. MMS=0 is NOT a disabled trigger: it emits UG.
    /// Gate both TRGO and ATIM TRGO2 to stopped CNT_EN while issuing UG, then
    /// restore reset-source selection before CEN can rise. RM16.10.2/17.10.2.
    pub(crate) fn load(self) {
        match self {
            Self::Atim(r) => {
                r.cr2().write(|v| {
                    v.set_mms(1);
                    v.set_mms2(1);
                });
                r.egr().write(|v| v.set_ug(true));
                r.icr().write(|v| v.set_uif(false));
                r.cr2().write_value(a::Cr2(0));
            }
            Self::Gtim(r) => {
                r.cr2().write(|v| v.set_mms(1));
                r.egr().write(|v| v.set_ug(true));
                r.icr().write(|v| v.set_uif(false));
                r.cr2().write_value(g::Cr2(0));
            }
        }
    }
    pub(crate) fn configure_pwm(self) {
        both!(self, r, {
            for n in 0..2 {
                r.ccmr_cmp(n).write(|v| {
                    v.set_ocm(0, 6);
                    v.set_ocm(1, 6);
                    v.set_ocpe(0, true);
                    v.set_ocpe(1, true);
                });
            }
        });
    }
    pub(crate) fn write_compare(self, channel: usize, duty: u32) {
        // 65536 is represented by forced-active mode, never truncated into CCR.
        let ccr = if duty == 65536 { 0 } else { duty as u16 };
        both!(self, r, {
            r.ccr(channel).write(|v| v.set_ccr(ccr));
        });
    }
    pub(crate) fn output(self, channel: usize, enabled: bool, duty: u32) {
        both!(self, r, {
            // Disable before any mode write. This also makes endpoint changes
            // safe when their requested duty differs from the active mode.
            if !enabled {
                r.ccer().modify(|v| match channel {
                    0 => v.set_cc1e(false),
                    1 => v.set_cc2e(false),
                    2 => v.set_cc3e(false),
                    3 => v.set_cc4e(false),
                    _ => unreachable!(),
                });
            } else {
                // CCxNP/CCxP/CCxNE remain zero: only main active-high outputs.
                r.ccmr_cmp(channel / 2)
                    .modify(|v| v.set_ocm(channel % 2, if duty == 65536 { 5 } else { 6 }));
                r.ccer().modify(|v| match channel {
                    0 => v.set_cc1e(true),
                    1 => v.set_cc2e(true),
                    2 => v.set_cc3e(true),
                    3 => v.set_cc4e(true),
                    _ => unreachable!(),
                });
            }
        });
    }
    pub(crate) fn master_output(self, enabled: bool) {
        if let Self::Atim(r) = self {
            // BDTR contains mixed/write-once fields: explicit read/write only.
            let mut value = r.bdtr().read();
            value.set_moe(enabled);
            r.bdtr().write_value(value);
        }
    }
}
