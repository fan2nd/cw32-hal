//! ATIM F030, CW32x030 RM2.5 chapter 15. A1–A3 compare preloads and the
//! master-output gate differ from GTIM; there is no per-pin hardware enable.
use super::{low_level::Config, Prescalers, TimerRegisters};
use crate::pac::atim::{regs, Atim};

fn control(r: Atim) -> regs::Cr {
    let mut value = r.cr().read();
    // Never replay the self-clearing software commands during a control RMW.
    value.set_bg(false);
    value.set_tg(false);
    value.set_ug(false);
    value
}
impl TimerRegisters for Atim {
    fn prescalers(self) -> Prescalers {
        Prescalers::AtimF030
    }
    fn initialize(self) {
        self.dtr().write_value(regs::Dtr(0));
        let mut value = regs::Cr(0);
        value.set_mode(2);
        value.set_pwm2s(true);
        value.set_bufpen(true);
        self.cr().write_value(value); // PCLK, up, single-ended; COMP=0.
        self.trig().write_value(regs::Trig(0)); // Never emit ADC triggers.
        self.mscr().write_value(regs::Mscr(0));
        self.rcr().write_value(regs::Rcr(0));
        self.fltr().write_value(regs::Fltr(0)); // All six compare outputs forced low.
        for n in 0..3 {
            self.chcr(n).write(|v| {
                *v = regs::Chcr(0); // Clear CISB reset selection and all DMA/IRQ bits.
                v.set_bufea(true);
                v.set_bufeb(true);
            });
            self.ccra(n).write(|v| v.set_ccr(0));
            self.ccrb(n).write(|v| v.set_ccr(0));
        }
        self.ch4cr().write_value(regs::Ch4cr(0));
    }
    fn set_running(self, enabled: bool) {
        let mut v = control(self);
        v.set_en(enabled);
        self.cr().write_value(v);
    }
    fn is_running(self) -> bool {
        self.cr().read().en()
    }
    fn counter(self) -> u16 {
        self.cnt().read().cnt()
    }
    fn set_counter(self, value: u16) {
        self.cnt().write(|v| v.set_cnt(value));
    }
    fn write_timing(self, c: Config) {
        let mut v = control(self);
        v.set_prs(if c.prescaler_divisor == 256 {
            7
        } else {
            c.prescaler_divisor.trailing_zeros() as u8
        });
        self.cr().write_value(v);
        self.arr().write(|v| v.set_arr((c.period_ticks - 1) as u16));
        self.cnt().write(|v| v.set_cnt(0));
    }
    fn load(self) {
        // ADC gate stays disabled. MMS=0 emits UG to downstream timers;
        // gate it to EN=0 too, restoring reset source before starting.
        self.trig().write_value(regs::Trig(0));
        self.mscr().write(|v| v.set_mms(1));
        let mut v = control(self);
        v.set_ug(true);
        self.cr().write_value(v);
        self.icr().write(|v| v.set_uif(false));
        self.mscr().write_value(regs::Mscr(0));
    }
    fn configure_pwm(self) {
        // initialize() configured the ATIM compare buffers and inactive outputs.
    }
    fn write_compare(self, channel: usize, duty: u32) {
        let ccr = if duty == 65536 { 0 } else { duty as u16 };
        self.ccra(channel).write(|v| v.set_ccr(ccr));
    }
    fn output(self, channel: usize, enabled: bool, duty: u32) {
        let mode = if !enabled {
            0
        } else if duty == 65536 {
            1
        } else {
            6
        };
        self.fltr().modify(|v| match channel {
            0 => v.set_ocm1aflt1a(mode),
            1 => v.set_ocm2aflt2a(mode),
            2 => v.set_ocm3aflt3a(mode),
            _ => unreachable!(),
        });
    }
    fn master_output(self, enabled: bool) {
        self.dtr().modify(|v| v.set_moe(enabled));
    }
}
