use super::{RtcError, WriteGuard};
use crate::{pac, rcc::RtcClockSource};

pub(super) fn configure(_: RtcClockSource) {
    // F030 has no programmable calendar prescaler. LSI is still divided by
    // 32768, giving nominal +976.5625ppm before RC frequency error.
    pac::RTC
        .compen()
        .write_value(pac::rtc::regs::Compen::default());
}
pub(super) fn configuration_matches(_: RtcClockSource) -> bool {
    !pac::RTC.compen().read().en() && !pac::RTC.cr1().read().access()
}
pub(super) fn with_access(
    limit: u32,
    operation: impl FnOnce() -> Result<(), RtcError>,
) -> Result<(), RtcError> {
    let mut operation = Some(operation);
    for _ in 0..limit {
        let result = critical_section::with(|_| {
            // Recheck WINDOW with interrupts masked: no interrupt can consume
            // the access window between its observation and setting ACCESS.
            if !pac::RTC.cr1().read().window() {
                return None;
            }
            let _write = WriteGuard::new();
            let mut cr1 = pac::RTC.cr1().read();
            cr1.set_access(true);
            pac::RTC.cr1().write_value(cr1);
            let _access = AccessGuard;
            if !pac::RTC.cr1().read().access() {
                return Some(Err(RtcError::ReadbackMismatch));
            }
            // Only a fixed-size register sequence executes here. No polls,
            // waits, user callback or interrupt extends ACCESS beyond 1 second
            // at the HAL's supported core/bus clocks (debug/NMI excluded).
            Some(operation.take().unwrap()())
        });
        if let Some(result) = result {
            return result;
        }
        core::hint::spin_loop();
    }
    Err(RtcError::SynchronizationTimeout)
}
struct AccessGuard;
impl Drop for AccessGuard {
    fn drop(&mut self) {
        let mut cr1 = pac::RTC.cr1().read();
        cr1.set_access(false);
        pac::RTC.cr1().write_value(cr1);
    }
}
