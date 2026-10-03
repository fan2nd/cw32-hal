use super::{RtcError, WriteGuard};
use crate::{pac, rcc::RtcClockSource};

pub(super) fn configure(source: RtcClockSource) {
    // RM13.3.2 requires TICKCLK=2 Hz. PSC1=0 gives <=1MHz for either source;
    // PSC2 is 16383 for 32768Hz LSE and 16399 for nominal 32800Hz LSI.
    let mut psc = pac::rtc::regs::Psc::default();
    psc.set_psc1(0);
    psc.set_psc2(source.nominal_frequency() / 2 - 1);
    pac::RTC.psc().write_value(psc);
    pac::RTC
        .compcfr1()
        .write_value(pac::rtc::regs::Compcfr1::default());
}
pub(super) fn configuration_matches(source: RtcClockSource) -> bool {
    let psc = pac::RTC.psc().read();
    psc.psc1() == 0
        && psc.psc2() == source.nominal_frequency() / 2 - 1
        && !pac::RTC.compcfr1().read().en()
}
pub(super) fn with_access(
    limit: u32,
    operation: impl FnOnce() -> Result<(), RtcError>,
) -> Result<(), RtcError> {
    let mut operation = Some(operation);
    for _ in 0..limit {
        let result = critical_section::with(|_| {
            if pac::RTC.cr1().read().wait() {
                return None;
            }
            let _write = WriteGuard::new();
            // The only caller stops the counter before DATE/TIME writes, then
            // restarts it, so there is no running-register load to wait for.
            Some(operation.take().unwrap()())
        });
        if let Some(result) = result {
            return result;
        }
        core::hint::spin_loop();
    }
    Err(RtcError::SynchronizationTimeout)
}
