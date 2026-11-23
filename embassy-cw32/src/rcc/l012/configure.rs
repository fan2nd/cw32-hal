use super::inspection::inspect_retained;
use super::io::{barrier, check_external_faults, hse_parameters_match, poll, poll_clock, set_flash_latency};
#[cfg(rcc_lse)]
use super::lse_sysclk::{finish_lse_sysclk, LseSysclkSnapshot};
#[cfg(rcc_lsi_sysclk)]
use super::lsi_sysclk::{finish_lsi_sysclk, lsi_ack_field, LsiSysclkPhase, LsiSysclkSnapshot};
use super::{
    pac, AHBPrescaler, APBPrescaler, ClockSource, Clocks, Config, Error, HseDrive, HseMode,
    L012HseTarget, Sysclk, MAX_FLASH_WAIT,
};

pub(super) fn configure<const HSI_ONLY: bool>(config: Config, mut clocks: Clocks, cs: critical_section::CriticalSection<'_>) -> Result<Clocks, Error> {
    // Keep the statically admitted requests separate from the hardware
    // snapshots: inherited sources must still be inspected and preserved.
    let sys = if HSI_ONLY { Sysclk::HSI } else { config.sys };
    let hse_request = if HSI_ONLY { None } else { config.hse };
    #[cfg(rcc_lse)]
    let lse_request = if HSI_ONLY { None } else { config.lse };
    let r = pac::SYSCTRL;
    let old_sources = r.cr1().read();
    #[cfg(not(rcc_lsi_sysclk))]
    let live_lsi_target = false;
    #[cfg(rcc_lsi_sysclk)]
    let live_lsi_target = sys == Sysclk::LSI;
    // Keep the Stage41 no-write refusal for any undeclared inherited HSE,
    // including HSE retained only for RTC/AWT while SYSCLK is another source.
    if !live_lsi_target && old_sources.hseen() && hse_request.is_none() {
        return Err(Error::InheritedHseEnabled);
    }
    let old_clock = r.cr0().read();
    let old_hsi = r.hsi().read();
    let old_hse = r.hse().read();
    let old_lsi = r.lsi().read();
    let old_lse = r.lse().read();
    // Immutable target identity precedes native preflight and every gate write.
    #[cfg(rcc_lse)]
    let lse_target = (sys == Sysclk::LSE).then(|| LseSysclkSnapshot {
        clock: old_clock,
        sources: old_sources,
        hsi: old_hsi,
        hse: old_hse,
        lsi: old_lsi,
        lse: old_lse,
        routes: r.cr2().read(),
        interrupts: r.ier().read(),
        mco: r.mco().read(),
    });
    #[cfg(rcc_lsi_sysclk)]
    let lsi_target = live_lsi_target.then(|| {
        let routes = r.cr2().read();
        let interrupts = r.ier().read();
        let mco = r.mco().read();
        let factory_raw =
            unsafe { core::ptr::read_volatile(crate::RCC_LSI_FACTORY_TRIM_ADDRESS as *const u16) };
        // Erased storage is never passed through the typed TRIM mask. Keep
        // refusal deferred so inherited-HSE and relevant-fault priority holds.
        let factory_trim = (factory_raw != u16::MAX).then(|| {
            let mut calibrated = pac::sysctrl::regs::Lsi::default();
            calibrated.set_trim(factory_raw);
            calibrated.trim()
        });
        LsiSysclkSnapshot {
            clock: old_clock,
            sources: old_sources,
            hsi: old_hsi,
            hse: old_hse,
            lsi: old_lsi,
            lse: old_lse,
            routes,
            interrupts,
            mco,
            factory_raw,
            factory_trim,
        }
    });
    #[cfg(rcc_lsi_sysclk)]
    let mut lsi_phase = lsi_target.map(LsiSysclkSnapshot::phase);
    #[cfg(rcc_lsi_sysclk)]
    let mut lsi_flash = None;
    // These local guards have no write operation. Every write stays visible
    // below at its own phase edge; generic and old targets take their old polls.
    macro_rules! lsi_guard {
        () => {{
            #[cfg(rcc_lsi_sysclk)]
            if let (Some(entry), Some(phase)) = (lsi_target, lsi_phase) {
                entry.settled(
                    config,
                    phase,
                    lsi_flash,
                    if lsi_flash.is_some() {
                        MAX_FLASH_WAIT as u8
                    } else {
                        entry.routes.flashwait()
                    },
                )?;
            }
        }};
    }
    macro_rules! lsi_ack {
        ($error:expr, $advance:expr) => {{
            #[cfg(rcc_lsi_sysclk)]
            if let (Some(entry), Some(phase), Some(flash)) =
                (lsi_target, lsi_phase.as_mut(), lsi_flash)
            {
                entry.acknowledge(config, phase, flash, MAX_FLASH_WAIT as u8, $error, &mut $advance)?;
            }
        }};
    }
    if live_lsi_target && old_sources.hseen() && hse_request.is_none() {
        return Err(Error::InheritedHseEnabled);
    }
    #[cfg(rcc_lse)]
    let l012_sysclk = lse_target.is_some();
    let hse_target = L012HseTarget::Legacy;
    #[cfg(rcc_lse)]
    let hse_target = if l012_sysclk {
        L012HseTarget::LseSysclk
    } else {
        hse_target
    };
    #[cfg(rcc_lsi_sysclk)]
    let hse_target = if live_lsi_target {
        L012HseTarget::LiveLsiSysclk
    } else {
        hse_target
    };
    let monitor_hse = hse_request.is_some() || old_sources.hseen() || old_sources.hseccs();
    let monitor_lse = old_sources.lseen() || old_sources.lseccs();
    #[cfg(rcc_lse)]
    let monitor_lse = monitor_lse || lse_request.is_some();
    check_external_faults(monitor_hse, monitor_lse)?;
    #[cfg(rcc_lsi_sysclk)]
    if let Some(entry) = lsi_target {
        entry.admit(config)?;
    }
    // Frequency legality is an entry precondition; STABLE only proves startup.
    // Every HSI divider encoding is legal on this own register version.
    let entry_ready = match old_clock.sysclk() {
        ClockSource::Hsi => old_sources.hsien() && old_hsi.stable(),
        ClockSource::Hse => old_sources.hseen() && old_hse.stable(),
        ClockSource::Lsi => old_lsi.stable(),
        ClockSource::Lse => old_sources.lseen() && old_lse.stable(),
        _ => return Err(Error::InvalidClockSource),
    };
    if !entry_ready {
        return Err(Error::InvalidEntryClock);
    }
    if old_sources.hseen() && !old_hse.stable() {
        return Err(Error::InvalidEntryClock);
    }
    if old_sources.hseen()
        && (old_hse.driver().to_bits() > HseDrive::Level7.to_bits()
            || old_hse.pdriver().to_bits() > HseDrive::Level7.to_bits())
    {
        return Err(Error::InvalidHseConfiguration);
    }
    let factory =
        unsafe { core::ptr::read_volatile(crate::RCC_FACTORY_HSI_TRIM_ADDRESS as *const u16) };
    if factory == u16::MAX {
        return Err(Error::InvalidCalibration);
    }
    let mut calibrated = pac::sysctrl::regs::Hsi::default();
    calibrated.set_trim(factory);
    let trim = calibrated.trim();
    let needs_trim = old_hsi.trim() != trim;

    lsi_guard!();
    // Native central rejection precedes the first configuration-gate write.
    // This phase inspects only configuration domains, never GPIO/timer work.
    #[cfg(rcc_lse)]
    if let Some(entry) = lse_target {
        entry.unchanged_sources(config, false, false)?;
    }
    #[cfg(rcc_lse)]
    let lse_admission = lse_request
        .map(|c| crate::rcc::lse::preflight(c, cs, l012_sysclk))
        .transpose();
    #[cfg(rcc_lse)]
    if let Some(entry) = lse_target {
        if let Err(
            error @ (Error::LseConfigurationGateTimeout { .. } | Error::LseGpioGateTimeout),
        ) = lse_admission
        {
            return Err(error);
        }
        entry.policy(config)?;
    }
    #[cfg(rcc_lse)]
    let lse_admission = lse_admission?;

    // Inspect actual central gates, restore their incoming state, never reset.
    // SOURCE owns RTC/AWT's raw clock even when the calendar START bit is zero.
    lsi_guard!();
    let retained_result = inspect_retained::<HSI_ONLY, _>(
        config,
        cs,
        monitor_hse,
        <crate::peripherals::RTC as crate::rcc::SealedRccPeripheral>::RCC_INFO,
        || {
            (
                pac::RTC.cr1().read().source(),
                u32::from(pac::RTC.psc().read().psc1()) + 1,
            )
        },
    );
    #[cfg(rcc_lsi_sysclk)]
    if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = retained_result {
        return Err(error);
    }
    lsi_guard!();
    let (rtc_source, rtc_first) = retained_result?;
    let invalid_rtc_source = rtc_source > 3;
    #[cfg(rcc_lse)]
    let invalid_rtc_source = invalid_rtc_source && lse_target.is_none();
    if invalid_rtc_source {
        return Err(Error::RetainedRtcConfiguration);
    }
    if rtc_source == 3 && (needs_trim || !old_sources.hsien() || !old_hsi.stable()) {
        return Err(Error::RtcClockInUse);
    }
    // Any enabled incoming HSE is readback-only reuse, whether RTC selects it
    // or not. No inferred absence of owners authorizes stopping or retuning it.
    let preserve_hse = old_sources.hseen();
    if rtc_source == 1 && !preserve_hse {
        return Err(Error::HseClockInUse);
    }
    if let Some(hse) = hse_request {
        if preserve_hse {
            lsi_guard!();
            let parameters = hse_parameters_match(hse);
            lsi_guard!();
            if !parameters? {
                return Err(Error::HseClockInUse);
            }
            let result = crate::rcc_hse_pins_match(
                hse.mode == HseMode::Bypass,
                config.timeout,
                cs,
                hse_target,
            );
            #[cfg(rcc_lsi_sysclk)]
            if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = result {
                return Err(error);
            }
            lsi_guard!();
            if !result? {
                return Err(Error::HseClockInUse);
            }
        } else if old_hse.stable() {
            return Err(Error::InvalidEntryClock);
        }
    }
    // PSC1 also feeds AWT through RTCCLKD; START=0 never releases the source.
    // Its actual input/divisor must satisfy the own <=1 MHz requirement.
    let rtc_upper = match rtc_source {
        1 => hse_request.ok_or(Error::HseClockInUse)?.max_freq.0,
        3 => crate::rcc::ClockBounds::hsi(1).maximum().0,
        _ => 0, // Unchanged legal LSE/LSI are already below this ceiling.
    };
    if u64::from(rtc_upper) > 1_000_000 * u64::from(rtc_first) {
        return Err(Error::RetainedRtcConfiguration);
    }
    // Both ADCs share a configuration-and-work gate. The target's documented
    // functional handover permits conversion/trigger progress during this
    // inspection, including before EN refusal; restoration cannot undo it.
    // The separate platform bus-master/memory-ownership boundary still applies.
    lsi_guard!();
    let retained_result = inspect_retained::<HSI_ONLY, _>(
        config,
        cs,
        monitor_hse,
        <crate::peripherals::ADC1 as crate::rcc::SealedRccPeripheral>::RCC_INFO,
        || {
            let adc1 = pac::ADC1.cr().read().en();
            let adc2 = pac::ADC2.cr().read().en();
            adc1 || adc2
        },
    );
    #[cfg(rcc_lsi_sysclk)]
    if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = retained_result {
        return Err(error);
    }
    lsi_guard!();
    let adc_enabled = retained_result?;
    if adc_enabled {
        return Err(Error::AdcClockInUse);
    }
    // L012 LVD has no RCC configuration gate. Do not borrow another IP's VC gate.
    lsi_guard!();
    let lvd0 = pac::LVD.cr0().read();
    let lvd1 = pac::LVD.cr1().read();
    #[cfg(rcc_lse)]
    if let Some(entry) = lse_target {
        entry.faults(config)?;
    }
    lsi_guard!();
    let lvd_filtered = lvd0.en() && lvd1.flttime() != 0;
    if lvd_filtered && lvd0.fltclk() {
        return Err(Error::LvdClockInUse);
    }
    // All four comparators share VC's gate. Every CR2 bit is a blank-trigger
    // enable, and blank duration uses PCLK independently of filter selection.
    lsi_guard!();
    let retained_result = inspect_retained::<HSI_ONLY, _>(
        config,
        cs,
        monitor_hse,
        <crate::peripherals::VC1 as crate::rcc::SealedRccPeripheral>::RCC_INFO,
        || {
            [pac::VC1, pac::VC2, pac::VC3, pac::VC4]
                .map(|vc| (vc.cr0().read(), vc.cr1().read(), vc.cr2().read()))
        },
    );
    #[cfg(rcc_lsi_sysclk)]
    if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = retained_result {
        return Err(error);
    }
    lsi_guard!();
    let comparators = retained_result?;
    let mut vc_lsi = false;
    #[cfg(rcc_lse)]
    let mut target_vc_lsi = false;
    for (control, filter, blank) in comparators {
        if control.en() {
            if blank.0 != 0
                || (filter.flttime() != 0 && filter.fltclk() == pac::vc::vals::FilterClock::Pclk)
            {
                return Err(Error::ComparatorClockInUse);
            }
            vc_lsi |= filter.flttime() != 0;
            #[cfg(rcc_lse)]
            {
                // Native automatic request is independent of filter count.
                target_vc_lsi |= filter.fltclk() == pac::vc::vals::FilterClock::InternalRc;
            }
        }
    }
    // OPA static amplification is preserved; calibration uses PCLK even when
    // AZRUN is low, so enabled/armed calibration is independently refused.
    lsi_guard!();
    let retained_result = inspect_retained::<HSI_ONLY, _>(
        config,
        cs,
        monitor_hse,
        <crate::peripherals::OPA1 as crate::rcc::SealedRccPeripheral>::RCC_INFO,
        || [pac::OPA1.cal().read(), pac::OPA2.cal().read()],
    );
    #[cfg(rcc_lsi_sysclk)]
    if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = retained_result {
        return Err(error);
    }
    lsi_guard!();
    let calibrations = retained_result?;
    if calibrations
        .iter()
        .any(|cal| cal.calen() || cal.azrun() || cal.start())
    {
        return Err(Error::OpaClockInUse);
    }
    lsi_guard!();
    let retained_result = inspect_retained::<HSI_ONLY, _>(
        config,
        cs,
        monitor_hse,
        <crate::peripherals::DAC as crate::rcc::SealedRccPeripheral>::RCC_INFO,
        || pac::DAC.cr0().read(),
    );
    #[cfg(rcc_lsi_sysclk)]
    if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = retained_result {
        return Err(error);
    }
    lsi_guard!();
    let dac = retained_result?;
    if (dac.en1() && (dac.ten1() || dac.dmaen1() || dac.wave1() != pac::dac::vals::Wave::Disabled))
        || (dac.en2()
            && (dac.ten2() || dac.dmaen2() || dac.wave2() != pac::dac::vals::Wave::Disabled))
    {
        return Err(Error::DacClockInUse);
    }
    // The own package table establishes no ADC/VC/OPA overlap with PF0/PF1.
    // Digital AF owners, DMA and asynchronous writers must be quiescent on entry.
    let needs_lsi = old_sources.hseccs() || old_sources.lseccs() || lvd_filtered || vc_lsi;
    #[cfg(rcc_lse)]
    let needs_lsi = if lse_target.is_some() {
        old_sources.hseccs()
            || old_sources.lseccs()
            || (lvd0.en() && !lvd0.fltclk())
            || target_vc_lsi
    } else {
        needs_lsi
    };
    // Entry readiness, not LSIEN or later STABLE progress, owns this latch.
    #[cfg(rcc_lse)]
    let first_lsi_request = lse_target.is_some() && (needs_trim || needs_lsi) && !old_lsi.stable();
    #[cfg(rcc_lse)]
    if let Some(entry) = lse_target {
        entry.hsi_ownership(config, trim, cs)?;
        if first_lsi_request {
            entry.admit_first_lsi_request(config, cs)?;
        }
        if rtc_source > 3 {
            return Err(Error::RetainedRtcConfiguration);
        }
    }
    #[cfg(rcc_lsi_sysclk)]
    if let Some(entry) = lsi_target {
        if needs_trim || !old_sources.hsien() || !old_hsi.stable() {
            lsi_guard!();
            if entry.mco.source() == 3 || entry.interrupts.hsirdy() {
                return Err(Error::HsiClockInUse);
            }
            use crate::rcc::SealedRccPeripheral;
            for (info, i2c) in [
                (crate::peripherals::I2C1::RCC_INFO, pac::I2C1),
                (crate::peripherals::I2C2::RCC_INFO, pac::I2C2),
            ] {
                lsi_guard!();
                let result = inspect_retained::<HSI_ONLY, _>(config, cs, monitor_hse, info, || {
                    let master = u8::from(i2c.mcr0().read().clksrc());
                    let slave = u8::from(i2c.scr0().read().clksrc());
                    matches!(master, 1 | 3) || matches!(slave, 1 | 3)
                });
                if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = result {
                    return Err(error);
                }
                lsi_guard!();
                if result? {
                    return Err(Error::HsiClockInUse);
                }
            }
        }
    }
    let ccs_unchanged = |after_lse: bool| {
        #[cfg(rcc_lse)]
        let expected_lse = if after_lse { lse_request } else { None };
        #[cfg(rcc_lse)]
        let (lseen, lseccs) = expected_lse
            .map_or((old_sources.lseen(), old_sources.lseccs()), |c| {
                (true, c.monitored())
            });
        #[cfg(not(rcc_lse))]
        let (lseen, lseccs) = {
            let _ = after_lse;
            (old_sources.lseen(), old_sources.lseccs())
        };
        let v = r.cr1().read();
        v.clkccs() == old_sources.clkccs()
            && v.hseccs() == old_sources.hseccs()
            && v.lseccs() == lseccs
            && v.lselock() == old_sources.lselock()
            && v.lseen() == lseen
    };
    // The source-owned legal incoming range is a precondition, not a TRIM
    // measurement. Preserve larger dividers and prove the transition guard.
    let guard_hclk = old_clock.hclkprs().max(AHBPrescaler::Div8 as u8);
    let guard_pclk = old_clock.pclkprs().max(APBPrescaler::Div8 as u8);
    let bus_max = if config.operating_conditions.supply_mv < crate::RCC_LOW_VOLTAGE_THRESHOLD_MV
    {
        crate::RCC_LOW_VOLTAGE_BUS_MAX_HZ
    } else {
        crate::RCC_HIGH_VOLTAGE_BUS_MAX_HZ
    };
    if !config.allow_nominal_bus_overclock
        && (crate::RCC_HSI_OPERATING_RANGE_HZ
            .1
            .div_ceil(1 << guard_hclk)
            > bus_max
            || crate::RCC_HSI_OPERATING_RANGE_HZ
                .1
                .div_ceil(1 << (guard_hclk + guard_pclk))
                > bus_max)
    {
        return Err(Error::InvalidEntryClock);
    }
    #[cfg(rcc_lse)]
    if let Some(entry) = lse_target {
        entry.faults(config)?;
        if <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO.reset_asserted()
        {
            return Err(Error::LseClockInUse);
        }
    }
    lsi_guard!();
    if live_lsi_target
        && <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO.reset_asserted()
    {
        lsi_guard!();
        return Err(Error::ClockConfigurationTimeout);
    }
    <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO
        .enable_with_cs_readback(
            cs,
            crate::rcc::Readback::Poll {
                attempts: config.timeout,
                spin: true,
            },
        )
        .map_err(|_| Error::FlashClockTimeout)?;
    lsi_guard!();
    #[cfg(rcc_lsi_sysclk)]
    if let (Some(entry), Some(phase)) = (lsi_target, lsi_phase) {
        let info = <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO;
        if !info.is_enabled() || info.reset_asserted() {
            lsi_guard!();
            return Err(Error::ClockConfigurationTimeout);
        }
        let flash = pac::FLASH.cr2().read();
        let accessible = info.is_enabled() && !info.reset_asserted();
        lsi_guard!();
        if !accessible {
            return Err(Error::ClockConfigurationTimeout);
        }
        let original_wait = entry.routes.flashwait();
        if u32::from(original_wait) > MAX_FLASH_WAIT {
            return Err(Error::FlashLatencyTimeout);
        }
        entry.settled(config, phase, Some(flash), original_wait)?;
        entry.write_wait(config, phase, flash, original_wait, MAX_FLASH_WAIT as u8)?;
        lsi_flash = Some(flash);
    }
    // Capture authoritative FLASH controls only after its gate acknowledges.
    #[cfg(rcc_lse)]
    let target_flash = lse_target
        .map(|entry| -> Result<_, Error> {
            entry.flash_access(config)?;
            let flash = pac::FLASH.cr2().read();
            entry.flash_access(config)?;
            Ok(flash)
        })
        .transpose()?;
    #[cfg(rcc_lse)]
    if let (Some(entry), Some(flash)) = (lse_target, target_flash) {
        let verify = || {
            entry.unchanged_sources(config, false, false)?;
            entry.clock_matches(old_clock.sysclk(), old_clock.hclkprs(), old_clock.pclkprs())
        };
        verify()?;
        let original_wait = entry.routes.flashwait();
        if u32::from(original_wait) > MAX_FLASH_WAIT {
            return Err(Error::FlashLatencyTimeout);
        }
        entry.require_wait(config, flash, original_wait)?;
        entry.write_wait(config, flash, original_wait, MAX_FLASH_WAIT as u8, verify)?;
    } else if !live_lsi_target {
        if u32::from(pac::FLASH.cr2().read().wait()) > MAX_FLASH_WAIT {
            return Err(Error::InvalidEntryClock);
        }
        set_flash_latency(MAX_FLASH_WAIT, config.timeout, monitor_hse, monitor_lse)?;
    }
    #[cfg(not(rcc_lse))]
    if !live_lsi_target {
        if u32::from(pac::FLASH.cr2().read().wait()) > MAX_FLASH_WAIT {
            return Err(Error::InvalidEntryClock);
        }
        set_flash_latency(MAX_FLASH_WAIT, config.timeout, monitor_hse, monitor_lse)?;
    }
    // Enable unchanged HSI before selecting it (own RM section4.5.1). Its
    // incoming TRIM must already yield the legal HSIOSC range whenever enabled,
    // including when HSI was initially disabled. This is not measured here.
    lsi_guard!();
    r.cr1().modify(|w| {
        w.set_key(0x5a5a);
        w.set_hsien(true);
    });
    lsi_ack!(
        Error::HsiTimeout,
        |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
            expected.sources.set_hsien(lsi_ack_field(
                observed.sources.hsien(),
                expected.sources.hsien(),
                true,
            )?);
            expected.hsi_ready = lsi_ack_field(observed.hsi_ready, expected.hsi_ready, true)?;
            Ok(expected.sources.hsien() && expected.hsi_ready)
        }
    );
    if !live_lsi_target {
        poll_clock(
            config.timeout,
            Error::HsiTimeout,
            monitor_hse,
            monitor_lse,
            || r.cr1().read().hsien() && r.hsi().read().stable(),
        )?;
    }
    lsi_guard!();
    // All three fields share keyed CR0 (own section4.7.1). Select HSI and
    // install the guarded buses in one write, so an intervening external
    // fallback cannot be overwritten by an inherited external selector.
    // Flash is already WAIT3; /8 covers the source-owned legal HSIOSC range.
    lsi_guard!();
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_hclkprs(guard_hclk);
        w.set_pclkprs(guard_pclk);
        w.set_sysclk(ClockSource::Hsi);
    });
    lsi_ack!(
        Error::ClockConfigurationTimeout,
        |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
            let mut target = expected.clock;
            target.set_key(0);
            target.set_sysclk(ClockSource::Hsi);
            target.set_hclkprs(guard_hclk);
            target.set_pclkprs(guard_pclk);
            let mut current = observed.clock;
            current.set_key(0);
            let mut prior = expected.clock;
            prior.set_key(0);
            if current.0 == target.0 {
                expected.clock.set_sysclk(ClockSource::Hsi);
                expected.clock.set_hclkprs(guard_hclk);
                expected.clock.set_pclkprs(guard_pclk);
                Ok(true)
            } else if current.0 == prior.0 {
                Ok(false)
            } else {
                Err(Error::ClockConfigurationTimeout)
            }
        }
    );
    if !live_lsi_target {
        poll_clock(
            config.timeout,
            Error::ClockConfigurationTimeout,
            monitor_hse,
            monitor_lse,
            || {
                let v = r.cr0().read();
                v.sysclk() == ClockSource::Hsi
                    && v.hclkprs() == guard_hclk
                    && v.pclkprs() == guard_pclk
                    && r.cr1().read().hsien()
                    && r.hsi().read().stable()
            },
        )?;
    }
    lsi_guard!();
    barrier();
    lsi_guard!();
    if !live_lsi_target && (needs_trim || needs_lsi) {
        #[cfg(rcc_lse)]
        if let (Some(entry), Some(flash)) = (lse_target, target_flash) {
            if first_lsi_request {
                entry.admit_first_lsi_request(config, cs)?;
            }
            // Mandatory for EVERY needed temporary assertion, even if entry
            // LSI was already stable. Consumer gate restorations finish first;
            // no gate/source write intervenes between this commit check and EN.
            entry.unchanged_sources(config, true, true)?;
            entry.clock_matches(ClockSource::Hsi, guard_hclk, guard_pclk)?;
            entry.require_wait(config, flash, MAX_FLASH_WAIT as u8)?;
        }
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_lsien(true);
        });
        poll_clock(
            config.timeout,
            Error::LsiTimeout,
            monitor_hse,
            monitor_lse,
            || r.cr1().read().lsien() && r.lsi().read().stable(),
        )?;
    }
    if needs_trim {
        // Automatic switching must not select HSI while stopped for retrim.
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_clkccs(false);
        });
        lsi_ack!(
            Error::ClockConfigurationTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.sources.set_clkccs(lsi_ack_field(
                    observed.sources.clkccs(),
                    expected.sources.clkccs(),
                    false,
                )?);
                Ok(!expected.sources.clkccs())
            }
        );
        if !live_lsi_target {
            poll(
                || !r.cr1().read().clkccs(),
                config.timeout,
                Error::ClockConfigurationTimeout,
                monitor_hse,
                monitor_lse,
            )?;
        }
        lsi_guard!();
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(ClockSource::Lsi);
        });
        lsi_ack!(
            Error::TemporaryClockSwitchTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.clock.set_sysclk(lsi_ack_field(
                    observed.clock.sysclk(),
                    expected.clock.sysclk(),
                    ClockSource::Lsi,
                )?);
                Ok(expected.clock.sysclk() == ClockSource::Lsi)
            }
        );
        if !live_lsi_target {
            poll_clock(
                config.timeout,
                Error::TemporaryClockSwitchTimeout,
                monitor_hse,
                monitor_lse,
                || r.cr0().read().sysclk() == ClockSource::Lsi,
            )?;
        }
        lsi_guard!();
        barrier();
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hsien(false);
        });
        lsi_ack!(
            Error::HsiStopTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.sources.set_hsien(lsi_ack_field(
                    observed.sources.hsien(),
                    expected.sources.hsien(),
                    false,
                )?);
                expected.hsi_ready = lsi_ack_field(observed.hsi_ready, expected.hsi_ready, false)?;
                Ok(!expected.sources.hsien() && !expected.hsi_ready)
            }
        );
        if !live_lsi_target {
            poll_clock(
                config.timeout,
                Error::HsiStopTimeout,
                monitor_hse,
                monitor_lse,
                || !r.cr1().read().hsien() && !r.hsi().read().stable(),
            )?;
        }
        lsi_guard!();
        r.hsi().modify(|w| {
            w.set_trim(trim);
            w.set_div(config.hsi.div as u8);
        });
        lsi_ack!(
            Error::ClockConfigurationTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.hsi.set_trim(lsi_ack_field(
                    observed.hsi.trim(),
                    expected.hsi.trim(),
                    trim,
                )?);
                expected.hsi.set_div(lsi_ack_field(
                    observed.hsi.div(),
                    expected.hsi.div(),
                    config.hsi.div as u8,
                )?);
                Ok(expected.hsi.trim() == trim && expected.hsi.div() == config.hsi.div as u8)
            }
        );
        if !live_lsi_target {
            poll(
                || {
                    let v = r.hsi().read();
                    v.trim() == trim && v.div() == config.hsi.div as u8
                },
                config.timeout,
                Error::ClockConfigurationTimeout,
                monitor_hse,
                monitor_lse,
            )?;
        }
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hsien(true);
        });
        lsi_ack!(
            Error::HsiTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.sources.set_hsien(lsi_ack_field(
                    observed.sources.hsien(),
                    expected.sources.hsien(),
                    true,
                )?);
                expected.hsi_ready = lsi_ack_field(observed.hsi_ready, expected.hsi_ready, true)?;
                Ok(expected.sources.hsien() && expected.hsi_ready)
            }
        );
        if !live_lsi_target {
            poll_clock(
                config.timeout,
                Error::HsiTimeout,
                monitor_hse,
                monitor_lse,
                || r.cr1().read().hsien() && r.hsi().read().stable(),
            )?;
        }
        lsi_guard!();
        r.cr0().modify(|w| {
            w.set_key(0x5a5a);
            w.set_sysclk(ClockSource::Hsi);
        });
        lsi_ack!(
            Error::ClockSwitchTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.clock.set_sysclk(lsi_ack_field(
                    observed.clock.sysclk(),
                    expected.clock.sysclk(),
                    ClockSource::Hsi,
                )?);
                Ok(expected.clock.sysclk() == ClockSource::Hsi)
            }
        );
        if !live_lsi_target {
            poll_clock(
                config.timeout,
                Error::ClockSwitchTimeout,
                monitor_hse,
                monitor_lse,
                || r.cr0().read().sysclk() == ClockSource::Hsi,
            )?;
        }
        lsi_guard!();
        barrier();
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_clkccs(old_sources.clkccs());
        });
        lsi_ack!(
            Error::ClockConfigurationTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.sources.set_clkccs(lsi_ack_field(
                    observed.sources.clkccs(),
                    expected.sources.clkccs(),
                    old_sources.clkccs(),
                )?);
                Ok(expected.sources.clkccs() == old_sources.clkccs())
            }
        );
        lsi_guard!();
    } else {
        // Own §4.5.2 allows live divider changes, without changing TRIM.
        lsi_guard!();
        r.hsi().modify(|w| w.set_div(config.hsi.div as u8));
        lsi_ack!(
            Error::ClockConfigurationTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.hsi.set_div(lsi_ack_field(
                    observed.hsi.div(),
                    expected.hsi.div(),
                    config.hsi.div as u8,
                )?);
                Ok(expected.hsi.div() == config.hsi.div as u8)
            }
        );
        lsi_guard!();
    }
    lsi_guard!();
    if !live_lsi_target {
        poll_clock(
            config.timeout,
            Error::ClockConfigurationTimeout,
            monitor_hse,
            monitor_lse,
            || {
                let v = r.hsi().read();
                v.trim() == trim
                    && v.div() == config.hsi.div as u8
                    && v.stable()
                    && r.cr1().read().hsien()
                    && ccs_unchanged(false)
                    && r.cr0().read().sysclk() == ClockSource::Hsi
            },
        )?;
    }
    lsi_guard!();
    if let Some(hse) = hse_request.filter(|_| !preserve_hse) {
        // Fresh start only: both stopped indicators must still be clear before
        // any pad/configuration write. Never stop an inherited enabled HSE.
        lsi_guard!();
        if r.cr1().read().hseen() || r.hse().read().stable() {
            lsi_guard!();
            return Err(Error::HseClockInUse);
        }
        lsi_guard!();
        let pins_result = crate::rcc_configure_hse_pins(
            hse.mode == HseMode::Bypass,
            config.timeout,
            cs,
            hse_target,
        );
        #[cfg(rcc_lsi_sysclk)]
        if live_lsi_target {
            if let Err(
                error @ (Error::LsiConfigurationGateTimeout { .. }
                | Error::HsePinConfigurationTimeout),
            ) = pins_result
            {
                return Err(error);
            }
        }
        lsi_guard!();
        pins_result?;
        let detector = hse.detector_count()?;
        lsi_guard!();
        r.hse().modify(|w| {
            w.set_mode(hse.mode == HseMode::Bypass);
            w.set_driver(hse.drive);
            w.set_pdriver(hse.drive);
            w.set_waitcycle(pac::sysctrl::vals::HseWait::Cycles262144);
            w.set_digflt(false);
            w.set_detcnt(detector);
        });
        lsi_ack!(
            Error::ClockConfigurationTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.hse.set_mode(lsi_ack_field(
                    observed.hse.mode(),
                    expected.hse.mode(),
                    hse.mode == HseMode::Bypass,
                )?);
                expected.hse.set_driver(lsi_ack_field(
                    observed.hse.driver(),
                    expected.hse.driver(),
                    hse.drive,
                )?);
                expected.hse.set_pdriver(lsi_ack_field(
                    observed.hse.pdriver(),
                    expected.hse.pdriver(),
                    hse.drive,
                )?);
                expected.hse.set_waitcycle(lsi_ack_field(
                    observed.hse.waitcycle(),
                    expected.hse.waitcycle(),
                    pac::sysctrl::vals::HseWait::Cycles262144,
                )?);
                expected.hse.set_digflt(lsi_ack_field(
                    observed.hse.digflt(),
                    expected.hse.digflt(),
                    false,
                )?);
                expected.hse.set_detcnt(lsi_ack_field(
                    observed.hse.detcnt(),
                    expected.hse.detcnt(),
                    detector,
                )?);
                Ok(expected.hse.mode() == (hse.mode == HseMode::Bypass)
                    && expected.hse.driver() == hse.drive
                    && expected.hse.pdriver() == hse.drive
                    && expected.hse.waitcycle() == pac::sysctrl::vals::HseWait::Cycles262144
                    && !expected.hse.digflt()
                    && expected.hse.detcnt() == detector)
            }
        );
        if !live_lsi_target {
            poll(
                || hse_parameters_match(hse).unwrap_or(false),
                config.timeout,
                Error::ClockConfigurationTimeout,
                monitor_hse,
                monitor_lse,
            )?;
        }
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_hseen(true);
        });
        lsi_ack!(
            Error::HseTimeout,
            |expected: &mut LsiSysclkPhase, observed: LsiSysclkPhase| {
                expected.sources.set_hseen(lsi_ack_field(
                    observed.sources.hseen(),
                    expected.sources.hseen(),
                    true,
                )?);
                expected.hse_ready = lsi_ack_field(observed.hse_ready, expected.hse_ready, true)?;
                Ok(expected.sources.hseen() && expected.hse_ready)
            }
        );
        if !live_lsi_target {
            poll_clock(
                config.timeout,
                Error::HseTimeout,
                monitor_hse,
                monitor_lse,
                || r.cr1().read().hseen() && r.hse().read().stable(),
            )?;
        }
        lsi_guard!();
    }
    lsi_guard!();
    if !live_lsi_target {
        // Restore only the software request; automatic LSI consumers stay active.
        // Never require STABLE to clear when LSIEN is cleared.
        lsi_guard!();
        r.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_lsien(old_sources.lsien());
        });
        poll(
            || r.cr1().read().lsien() == old_sources.lsien(),
            config.timeout,
            Error::TemporaryClockRestoreTimeout,
            monitor_hse,
            monitor_lse,
        )?;
    }
    lsi_guard!();
    #[cfg(rcc_lsi_sysclk)]
    if let (Some(entry), Some(phase), Some(flash)) = (lsi_target, lsi_phase, lsi_flash) {
        return finish_lsi_sysclk(config, clocks, entry, phase, flash, cs);
    }
    #[cfg(rcc_lse)]
    if let (Some(lse), Some(admission)) = (lse_request, lse_admission) {
        if let (Some(entry), Some(flash)) = (lse_target, target_flash) {
            entry.prepared_sources(config, trim, needs_lsi, false)?;
            entry.clock_matches(ClockSource::Hsi, guard_hclk, guard_pclk)?;
            entry.require_wait(config, flash, MAX_FLASH_WAIT as u8)?;
        }
        if let Some(entry) = lse_target {
            entry.flash_access(config)?;
        }
        let admission = admission.after_owned_flash_wait(MAX_FLASH_WAIT);
        if let Some(entry) = lse_target {
            entry.flash_access(config)?;
        }
        let admission = admission?;
        let result = crate::rcc::lse::start(lse, admission, cs, l012_sysclk);
        if let Some(entry) = lse_target {
            // Gate restoration is resolved first; otherwise both external
            // fault domains win over native semantic/readiness errors.
            if let Err(
                error @ (Error::LseConfigurationGateTimeout { .. } | Error::LseGpioGateTimeout),
            ) = result
            {
                return Err(error);
            }
            entry.policy(config)?;
        }
        result?;
    }
    #[cfg(rcc_lse)]
    if let (Some(entry), Some(flash)) = (lse_target, target_flash) {
        return finish_lse_sysclk(config, clocks, entry, flash, trim, needs_lsi, cs);
    }
    let sysclk = match sys {
        Sysclk::HSI => ClockSource::Hsi,
        Sysclk::HSE => ClockSource::Hse,
        #[cfg(rcc_lsi_sysclk)]
        Sysclk::LSI => unreachable!(),
        #[cfg(rcc_lse)]
        Sysclk::LSE => unreachable!(),
    };
    // Install final prescalers while verified HSI is selected. Requested HSI,
    // requested HSE and fixed CCS fallback are all qualified at these divisors.
    // An external fallback therefore cannot race this CR0 RMW into reselecting
    // stale HSE. Flash remains at its maximum documented WAIT throughout.
    if r.cr0().read().sysclk() != ClockSource::Hsi {
        return Err(Error::ClockSwitchTimeout);
    }
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(ClockSource::Hsi);
        w.set_hclkprs(config.ahb_pre as u8);
        w.set_pclkprs(config.apb_pre as u8);
    });
    poll_clock(
        config.timeout,
        Error::ClockConfigurationTimeout,
        monitor_hse,
        monitor_lse,
        || {
            let v = r.cr0().read();
            v.sysclk() == ClockSource::Hsi
                && v.hclkprs() == config.ahb_pre as u8
                && v.pclkprs() == config.apb_pre as u8
        },
    )?;
    barrier();
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(sysclk);
    });
    poll_clock(
        config.timeout,
        Error::ClockSwitchTimeout,
        monitor_hse,
        monitor_lse,
        || r.cr0().read().sysclk() == sysclk,
    )?;
    barrier();
    let upper_hclk = clocks
        .hclk_bounds()
        .maximum()
        .0
        .max(
            crate::rcc::ClockBounds::hsi(config.hsi.div.divisor())
                .divided_by(config.ahb_pre.divisor())
                .maximum()
                .0,
        )
        .max(
            crate::rcc::ClockBounds::hsi(crate::RCC_FIXED_CCS_HSI_DIVISOR)
                .divided_by(config.ahb_pre.divisor())
                .maximum()
                .0,
        );
    set_flash_latency(
        (upper_hclk - 1) / crate::RCC_FLASH_WAIT_STEP_HZ,
        config.timeout,
        monitor_hse,
        monitor_lse,
    )?;
    check_external_faults(monitor_hse, monitor_lse)?;
    let final_clock = r.cr0().read();
    let final_sources = r.cr1().read();
    let final_hsi = r.hsi().read();
    let final_hse = r.hse().read();
    let final_lsi = r.lsi().read();
    let lse_preserved = r.lse().read().0 == old_lse.0;
    #[cfg(rcc_lse)]
    let lse_preserved = if lse_request.is_some() {
        crate::rcc::lse::preserved_register(old_lse.0)
    } else {
        lse_preserved
    };
    if final_clock.sysclk() != sysclk
        || !ccs_unchanged(true)
        || final_clock.hclkprs() != config.ahb_pre as u8
        || final_clock.pclkprs() != config.apb_pre as u8
        || !final_sources.hsien()
        || !final_hsi.stable()
        || final_hsi.trim() != trim
        || final_hsi.div() != config.hsi.div as u8
        || final_sources.hseen() != (hse_request.is_some() || old_sources.hseen())
        || final_sources.lsien() != old_sources.lsien()
        || final_lsi.trim() != old_lsi.trim()
        || final_lsi.waitcycle() != old_lsi.waitcycle()
        || ((needs_lsi || old_lsi.stable()) && !final_lsi.stable())
        || !lse_preserved
        || final_hse.hexenpol() != old_hse.hexenpol()
    {
        return Err(Error::ClockConfigurationTimeout);
    }
    if let Some(hse) = hse_request {
        if !final_hse.stable()
            || !hse_parameters_match(hse)?
            || !crate::rcc_hse_pins_match(
                hse.mode == HseMode::Bypass,
                config.timeout,
                cs,
                L012HseTarget::Legacy,
            )?
        {
            return Err(Error::ClockConfigurationTimeout);
        }
    } else if final_hse.0 != old_hse.0 {
        return Err(Error::ClockConfigurationTimeout);
    }
    // Union inherited and requested pad ownership for the complete boot.
    // Current exact-reuse policy never changes the inherited oscillator mode.
    if old_sources.hseen() {
        if !old_hse.mode() {
            clocks.hse = Some(HseMode::Oscillator);
        } else if clocks.hse.is_none() {
            clocks.hse = Some(HseMode::Bypass);
        }
    }
    #[cfg(rcc_lse)]
    if let Some(lse) = lse_request {
        crate::rcc::lse::verify(lse, cs, false)?;
    }
    // Pad inspection may have opened/restored a gate; recheck fault/mux at freeze.
    check_external_faults(monitor_hse, monitor_lse)?;
    if r.cr0().read().sysclk() != sysclk {
        return Err(Error::ClockSwitchTimeout);
    }
    Ok(clocks)
}

