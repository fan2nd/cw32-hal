//! CW32L012 RM1.4 §17.3.4.7–8, table 17-12/13 and §17.10.
use super::{BreakConfig, ComplementaryRegisters, DeadTime, Error, Frequency};
use crate::pac::atim::{regs, vals, Atim};

const fn decode(code: u8) -> u16 {
    match code {
        0..=127 => code as u16,
        128..=191 => (64 + (code & 63) as u16) * 2,
        192..=223 => (32 + (code & 31) as u16) * 8,
        _ => (32 + (code & 31) as u16) * 16,
    }
}
fn encode(ticks: u16) -> Result<u8, Error> {
    (0..=255u16)
        .find(|&c| decode(c as u8) >= ticks)
        .map(|c| c as u8)
        .ok_or(Error::DeadTimeTooLarge)
}
pub(super) fn validate_dead_time(value: DeadTime) -> Result<DeadTime, Error> {
    Ok(DeadTime {
        rising_ticks: decode(encode(value.rising_ticks)?),
        falling_ticks: decode(encode(value.falling_ticks)?),
    })
}
pub(super) fn validate_break(value: BreakConfig) -> Result<(), Error> {
    if value.filter > 15 {
        Err(Error::InvalidFilter)
    } else {
        Ok(())
    }
}
pub(super) fn dead_time_clock_ratio(frequency: Frequency) -> (u32, u32) {
    (frequency.clock_hz(), 1)
}
impl ComplementaryRegisters for Atim {
    fn configure_complementary(self, dead_time: DeadTime, brake: Option<BreakConfig>) {
        // Timer::new has already cleared CEN/CCER/DIER/CR2/AF1/AF2 and CKD.
        // No comparator, second-break, DMA, interrupt or trigger routing is added.
        for n in 0..3 {
            self.set_duty_mode(n, 0);
            self.ccmr_cmp(n / 2).modify(|v| v.set_ocpe(n % 2, true));
        }
        let mut bdtr = regs::Bdtr(0);
        bdtr.set_ossr(true);
        bdtr.set_ossi(true);
        if let Some(brake) = brake {
            bdtr.set_bke(true);
            bdtr.set_bkp(brake.active_high);
            bdtr.set_bkf(brake.filter);
        }
        // LOCK=0, AOE=0, MOE=0; both OIS and polarities remain low.
        self.bdtr().write_value(bdtr);
        self.af1().write(|v| v.set_bkine(brake.is_some()));
        self.af2().write_value(regs::Af2(0));
        self.set_dead_time(dead_time);
    }
    fn set_dead_time(self, dead_time: DeadTime) {
        let rise = encode(dead_time.rising_ticks).unwrap();
        let fall = encode(dead_time.falling_ticks).unwrap();
        let mut value = self.bdtr().read();
        value.set_moe(false);
        value.set_aoe(false);
        value.set_dtg(rise);
        self.bdtr().write_value(value);
        // DTAE must only change with CEN=0, enforced by the owning driver.
        let mut dtr2 = regs::Dtr2(0);
        dtr2.set_dtgf(fall);
        dtr2.set_dtae(rise != fall);
        self.dtr2().write_value(dtr2);
    }
    fn set_pair_enabled(self, channel: usize, enabled: bool) {
        // Both gates are essential even with only one physical pin: table 17-13
        // removes inversion/deadtime when only CCE or only CCNE is enabled.
        self.ccer().modify(|v| {
            v.set_cce(channel, enabled);
            v.set_ccne(channel, enabled);
        });
    }
    fn set_duty_mode(self, channel: usize, duty: u32) {
        self.ccmr_cmp(channel / 2).modify(|v| {
            v.set_ocmh(channel % 2, false);
            v.set_ocm(
                channel % 2,
                if duty == 65536 {
                    vals::CcmrCmpOcm::FORCE_ACTIVE
                } else {
                    vals::CcmrCmpOcm::PWM1
                },
            );
        });
    }
    fn master(self, enabled: bool) {
        // BDTR includes LOCK/write-once fields; preserve them explicitly.
        let mut value = self.bdtr().read();
        value.set_moe(enabled);
        value.set_aoe(false);
        self.bdtr().write_value(value);
    }
    fn master_enabled(self) -> bool {
        self.bdtr().read().moe()
    }
    fn fault_pending(self) -> bool {
        let value = self.isr().read();
        value.bif() || value.b2if() || value.sbif()
    }
    fn clear_fault(self) {
        // R1W0: the PAC reset seed preserves all unrelated status bits.
        self.icr().write(|v| {
            v.set_bif(false);
            v.set_b2if(false);
            v.set_sbif(false);
        });
    }
}
