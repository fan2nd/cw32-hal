//! CW32x030 RM2.5 §15.3.3.5–6, table 15-9 and §15.7. No STM32 layout assumptions.
use super::{BreakConfig, ComplementaryRegisters, DeadTime, Error, Frequency};
use crate::pac::atim::{regs, Atim};

const fn decode(code: u8) -> u16 {
    match code {
        0..=127 => code as u16 + 2,
        128..=191 => (64 + (code & 63) as u16) * 2 + 2,
        192..=223 => (32 + (code & 31) as u16) * 8 + 2,
        _ => (32 + (code & 31) as u16) * 16 + 2,
    }
}
fn encode(ticks: u16) -> Result<u8, Error> {
    (0..=255u16)
        .find(|&c| decode(c as u8) >= ticks)
        .map(|c| c as u8)
        .ok_or(Error::DeadTimeTooLarge)
}
pub(super) fn validate_dead_time(value: DeadTime) -> Result<DeadTime, Error> {
    if value.rising_ticks != value.falling_ticks {
        return Err(Error::AsymmetricDeadTimeUnsupported);
    }
    // Code zero with DTEN=1 means TWO TTCLK ticks. DTEN=0 disables insertion.
    let ticks = if value.rising_ticks == 0 {
        0
    } else {
        decode(encode(value.rising_ticks)?)
    };
    Ok(DeadTime::symmetric(ticks))
}
pub(super) fn validate_break(value: BreakConfig) -> Result<(), Error> {
    if matches!(value.filter, 0 | 4..=7) {
        Ok(())
    } else {
        Err(Error::InvalidFilter)
    }
}
pub(super) fn dead_time_clock_ratio(frequency: Frequency) -> (u32, u32) {
    (frequency.clock_hz(), frequency.config().prescaler_divisor)
}
impl ComplementaryRegisters for Atim {
    fn configure_complementary(self, dead_time: DeadTime, brake: Option<BreakConfig>) {
        // Timer::new configured PCLK upcounting and ARR/A/B compare buffering,
        // and cleared channel IRQ/DMA enables, TRIG and CH4CR.
        let mut control = self.cr().read();
        control.set_bg(false);
        control.set_ug(false);
        control.set_tg(false);
        control.set_comp(true);
        control.set_pwm2s(true);
        self.cr().write_value(control);
        for n in 0..3 {
            self.chcr(n).write(|v| {
                *v = regs::Chcr(0);
                v.set_bufea(true);
                v.set_bufeb(true);
                v.set_bksa(2);
                v.set_bksb(2);
            });
            self.set_duty_mode(n, 0);
        }
        self.fltr().modify(|v| {
            if let Some(brake) = brake {
                v.set_fltbk(brake.filter);
                v.set_bkp(!brake.active_high);
            } else {
                v.set_fltbk(0);
                v.set_bkp(false);
            }
        });
        let mut dead = regs::Dtr(0);
        dead.set_bke(brake.is_some());
        // AOE/MOE/VCE/SAFEEN remain off. There is no implicit comparator route.
        self.dtr().write_value(dead);
        self.set_dead_time(dead_time);
    }
    fn set_dead_time(self, dead_time: DeadTime) {
        self.dtr().modify(|v| {
            v.set_moe(false);
            v.set_aoe(false);
            v.set_dten(dead_time.rising_ticks != 0);
            v.set_dtr(if dead_time.rising_ticks == 0 {
                0
            } else {
                encode(dead_time.rising_ticks).unwrap()
            });
        });
    }
    fn set_pair_enabled(self, _channel: usize, _enabled: bool) {
        // No per-pair output-enable bit exists. The owning driver gates each
        // pair through its GPIO mux, independently of the shared MOE gate.
    }
    fn set_duty_mode(self, channel: usize, duty: u32) {
        self.fltr().modify(|v| {
            v.set_ocmflta(channel, if duty == 65536 { 1 } else { 6 });
            // In COMP/single-point mode B is derived from A, not from CCRB.
            v.set_ocmfltb(channel, 6);
        });
    }
    fn master(self, enabled: bool) {
        self.dtr().modify(|v| {
            v.set_moe(enabled);
            v.set_aoe(false);
        });
    }
    fn master_enabled(self) -> bool {
        self.dtr().read().moe()
    }
    fn fault_pending(self) -> bool {
        self.isr().read().bif()
    }
    fn clear_fault(self) {
        // R1W0: use the PAC's documented full reset seed, including RFU bit 1.
        self.icr().write(|v| v.set_bif(false));
    }
}
