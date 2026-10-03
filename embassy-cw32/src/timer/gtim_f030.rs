//! GTIM F030, CW32x030 RM2.5 chapter 14. There is no compare preload,
//! software update command, per-pin enable, or ATIM master-output gate.
use super::{low_level::Config, Prescalers, TimerRegisters};
use crate::pac::gtim::{regs, Gtim};

impl TimerRegisters for Gtim {
    fn prescalers(self) -> Prescalers {
        Prescalers::GtimF030
    }
    fn initialize(self) {
        self.cr0().write_value(regs::Cr0(0)); // Stopped, internal PCLK, no encoder.
        self.ier().write_value(regs::Ier(0));
        self.dma().write_value(regs::Dma(0));
        self.cr1().write_value(regs::Cr1(0));
        self.etr().write_value(regs::Etr(0));
        self.cmmr().write_value(regs::Cmmr(0));
        for n in 0..4 {
            self.ccr(n).write(|v| v.set_ccr(0));
        }
    }
    fn set_running(self, enabled: bool) {
        self.cr0().modify(|v| v.set_en(enabled));
    }
    fn is_running(self) -> bool {
        self.cr0().read().en()
    }
    fn counter(self) -> u16 {
        self.cnt().read().cnt()
    }
    fn set_counter(self, value: u16) {
        self.cnt().write(|v| v.set_cnt(value));
    }
    fn write_timing(self, c: Config) {
        self.cr0()
            .modify(|v| v.set_prs(c.prescaler_divisor.trailing_zeros() as u8));
        self.arr().write(|v| v.set_arr((c.period_ticks - 1) as u16));
        self.cnt().write(|v| v.set_cnt(0));
    }
    fn load(self) {
        // No software update command and no compare preload. PRS is latched
        // on EN's 0->1 edge (RM14.8.1), never by inventing UG.
    }
    fn configure_pwm(self) {
        self.cmmr().write(|v| {
            v.set_cc1m(8);
            v.set_cc2m(8);
            v.set_cc3m(8);
            v.set_cc4m(8);
        });
    }
    fn write_compare(self, channel: usize, duty: u32) {
        let ccr = if duty == 65536 { 0 } else { duty as u16 };
        self.ccr(channel).write(|v| v.set_ccr(ccr));
    }
    fn output(self, channel: usize, enabled: bool, duty: u32) {
        // RM14.8.4: 8=forced low, 9=forced high, 15=high when CNT<CCR.
        let mode = if !enabled {
            8
        } else if duty == 65536 {
            9
        } else {
            15
        };
        self.cmmr().modify(|v| match channel {
            0 => v.set_cc1m(mode),
            1 => v.set_cc2m(mode),
            2 => v.set_cc3m(mode),
            3 => v.set_cc4m(mode),
            _ => unreachable!(),
        });
    }
    fn master_output(self, _enabled: bool) {
        // GTIM outputs are controlled only by each channel's compare mode.
    }
}
