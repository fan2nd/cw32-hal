#[cfg(rcc_lse)]
use super::io::check_external_faults;
use super::{Config, Error};
#[cfg(any(rcc_lse, rcc_lsi_sysclk))]
use super::Sysclk;

// Retained non-target paths retain the original inspection/error contract.
pub(super) fn inspect_retained<const HSI_ONLY: bool, R>(
    config: Config,
    cs: critical_section::CriticalSection<'_>,
    monitor_hse: bool,
    info: crate::rcc::peripheral::RccInfo,
    read: impl FnOnce() -> R,
) -> Result<R, Error> {
    #[cfg(rcc_lsi_sysclk)]
    if !HSI_ONLY && config.sys == Sysclk::LSI {
        if info.reset_asserted() {
            return Err(Error::ClockConfigurationTimeout);
        }
        let result = info
            .inspect_for_init(cs, config.timeout, || {
                if !info.is_enabled() || info.reset_asserted() {
                    return None;
                }
                let value = read();
                (info.is_enabled() && !info.reset_asserted()).then_some(value)
            })
            .map_err(|error| match error {
                crate::rcc::peripheral::ClockInspectionError::EnableFailed { restore_failed } => {
                    Error::LsiConfigurationGateTimeout {
                        enable_failed: true,
                        restore_failed,
                    }
                }
                crate::rcc::peripheral::ClockInspectionError::RestoreFailed => {
                    Error::LsiConfigurationGateTimeout {
                        enable_failed: false,
                        restore_failed: true,
                    }
                }
            })?;
        if info.reset_asserted() {
            return Err(Error::ClockConfigurationTimeout);
        }
        return result.ok_or(Error::ClockConfigurationTimeout);
    }
    #[cfg(rcc_lse)]
    if !HSI_ONLY && config.sys == Sysclk::LSE {
        return lse_target_inspect(config, cs, info, monitor_hse, read);
    }
    let _ = monitor_hse;
    info.inspect_for_init(cs, config.timeout, read)
        .map_err(|_| Error::RetainedClockInspectionTimeout)
}

#[cfg(rcc_lse)]
pub(crate) fn lse_target_inspection_error(error: crate::rcc::peripheral::ClockInspectionError) -> Error {
    match error {
        crate::rcc::peripheral::ClockInspectionError::EnableFailed { restore_failed } => {
            Error::LseConfigurationGateTimeout {
                enable_failed: true,
                restore_failed,
            }
        }
        crate::rcc::peripheral::ClockInspectionError::RestoreFailed => {
            Error::LseConfigurationGateTimeout {
                enable_failed: false,
                restore_failed: true,
            }
        }
    }
}

#[cfg(rcc_lse)]
pub(super) fn lse_target_inspect<R>(
    config: Config,
    cs: critical_section::CriticalSection<'_>,
    info: crate::rcc::peripheral::RccInfo,
    monitor_hse: bool,
    read: impl FnOnce() -> R,
) -> Result<R, Error> {
    check_external_faults(monitor_hse, true)?;
    if info.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    // A semantic refusal is held until this gate's independent restoration
    // succeeds. In particular, enable+restore failure preserves both facts.
    let result = info
        .inspect_for_init(cs, config.timeout, || {
            if info.reset_asserted() {
                return None;
            }
            let result = read();
            (!info.reset_asserted()).then_some(result)
        })
        .map_err(lse_target_inspection_error)?;
    check_external_faults(monitor_hse, true)?;
    if info.reset_asserted() {
        return Err(Error::LseClockInUse);
    }
    result.ok_or(Error::LseClockInUse)
}

