//! CW32x030 RM2.5 chapters 14/15. GTIM and ATIM have different clocks,
//! compare encodings and preload facilities, and neither has a per-pin enable.
use super::{low_level::Config, Prescalers, Registers};
use crate::pac::{atim::regs as a, gtim::regs as g};

fn control(r: crate::pac::atim::Atim) -> a::Cr {
    let mut value = r.cr().read();
    // Never replay the self-clearing software commands during a control RMW.
    value.set_bg(false);
    value.set_tg(false);
    value.set_ug(false);
    value
}
impl Registers {
    pub(crate) fn prescalers(self) -> Prescalers {
        match self {
            Self::Atim(_) => Prescalers::AtimF030,
            Self::Gtim(_) => Prescalers::GtimF030,
        }
    }
    pub(crate) fn initialize(self) {
        match self {
            Self::Atim(r) => {
                r.dtr().write_value(a::Dtr(0));
                let mut value = a::Cr(0);
                value.set_mode(2);
                value.set_pwm2s(true);
                value.set_bufpen(true);
                r.cr().write_value(value); // PCLK, up, single-ended; COMP=0.
                r.trig().write_value(a::Trig(0)); // Never emit ADC triggers.
                r.mscr().write_value(a::Mscr(0));
                r.rcr().write_value(a::Rcr(0));
                r.fltr().write_value(a::Fltr(0)); // All six compare outputs forced low.
                for n in 0..3 {
                    r.chcr(n).write(|v| {
                        *v = a::Chcr(0); // Clear CISB reset selection and all DMA/IRQ bits.
                        v.set_bufea(true);
                        v.set_bufeb(true);
                    });
                    r.ccra(n).write(|v| v.set_ccr(0));
                    r.ccrb(n).write(|v| v.set_ccr(0));
                }
                r.ch4cr().write_value(a::Ch4cr(0));
            }
            Self::Gtim(r) => {
                r.cr0().write_value(g::Cr0(0)); // Stopped, internal PCLK, no encoder.
                r.ier().write_value(g::Ier(0));
                r.dma().write_value(g::Dma(0));
                r.cr1().write_value(g::Cr1(0));
                r.etr().write_value(g::Etr(0));
                r.cmmr().write_value(g::Cmmr(0));
                for n in 0..4 {
                    r.ccr(n).write(|v| v.set_ccr(0));
                }
            }
        }
    }
    pub(crate) fn set_running(self, enabled: bool) {
        match self {
            Self::Atim(r) => {
                let mut v = control(r);
                v.set_en(enabled);
                r.cr().write_value(v);
            }
            Self::Gtim(r) => r.cr0().modify(|v| v.set_en(enabled)),
        }
    }
    pub(crate) fn is_running(self) -> bool {
        match self {
            Self::Atim(r) => r.cr().read().en(),
            Self::Gtim(r) => r.cr0().read().en(),
        }
    }
    pub(crate) fn counter(self) -> u16 {
        match self {
            Self::Atim(r) => r.cnt().read().cnt(),
            Self::Gtim(r) => r.cnt().read().cnt(),
        }
    }
    pub(crate) fn set_counter(self, value: u16) {
        match self {
            Self::Atim(r) => r.cnt().write(|v| v.set_cnt(value)),
            Self::Gtim(r) => r.cnt().write(|v| v.set_cnt(value)),
        }
    }
    pub(crate) fn write_timing(self, c: Config) {
        let arr = (c.period_ticks - 1) as u16;
        match self {
            Self::Atim(r) => {
                let mut v = control(r);
                v.set_prs(if c.prescaler_divisor == 256 {
                    7
                } else {
                    c.prescaler_divisor.trailing_zeros() as u8
                });
                r.cr().write_value(v);
                r.arr().write(|v| v.set_arr(arr));
                r.cnt().write(|v| v.set_cnt(0));
            }
            Self::Gtim(r) => {
                r.cr0()
                    .modify(|v| v.set_prs(c.prescaler_divisor.trailing_zeros() as u8));
                r.arr().write(|v| v.set_arr(arr));
                r.cnt().write(|v| v.set_cnt(0));
            }
        }
    }
    pub(crate) fn load(self) {
        match self {
            Self::Atim(r) => {
                // ADC gate stays disabled. MMS=0 emits UG to downstream timers;
                // gate it to EN=0 too, restoring reset source before starting.
                r.trig().write_value(a::Trig(0));
                r.mscr().write(|v| v.set_mms(1));
                let mut v = control(r);
                v.set_ug(true);
                r.cr().write_value(v);
                r.icr().write(|v| v.set_uif(false));
                r.mscr().write_value(a::Mscr(0));
            }
            Self::Gtim(_) => {
                // No software update command and no compare preload. PRS is
                // latched on EN's 0->1 edge (RM14.8.1), never by inventing UG.
            }
        }
    }
    pub(crate) fn configure_pwm(self) {
        if let Self::Gtim(r) = self {
            r.cmmr().write(|v| {
                v.set_cc1m(8);
                v.set_cc2m(8);
                v.set_cc3m(8);
                v.set_cc4m(8);
            });
        }
    }
    pub(crate) fn write_compare(self, channel: usize, duty: u32) {
        let ccr = if duty == 65536 { 0 } else { duty as u16 };
        match self {
            Self::Atim(r) => r.ccra(channel).write(|v| v.set_ccr(ccr)),
            Self::Gtim(r) => r.ccr(channel).write(|v| v.set_ccr(ccr)),
        }
    }
    pub(crate) fn output(self, channel: usize, enabled: bool, duty: u32) {
        match self {
            Self::Atim(r) => {
                let mode = if !enabled {
                    0
                } else if duty == 65536 {
                    1
                } else {
                    6
                };
                r.fltr().modify(|v| match channel {
                    0 => v.set_ocm1aflt1a(mode),
                    1 => v.set_ocm2aflt2a(mode),
                    2 => v.set_ocm3aflt3a(mode),
                    _ => unreachable!(),
                });
            }
            Self::Gtim(r) => {
                // RM14.8.4: 8=forced low, 9=forced high, 15=high when CNT<CCR.
                let mode = if !enabled {
                    8
                } else if duty == 65536 {
                    9
                } else {
                    15
                };
                r.cmmr().modify(|v| match channel {
                    0 => v.set_cc1m(mode),
                    1 => v.set_cc2m(mode),
                    2 => v.set_cc3m(mode),
                    3 => v.set_cc4m(mode),
                    _ => unreachable!(),
                });
            }
        }
    }
    pub(crate) fn master_output(self, enabled: bool) {
        if let Self::Atim(r) = self {
            r.dtr().modify(|v| v.set_moe(enabled));
        }
    }
}
