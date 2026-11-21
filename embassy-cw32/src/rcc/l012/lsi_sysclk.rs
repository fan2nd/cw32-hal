use super::io::{barrier, check_external_faults};
use super::{pac, ClockSource, Clocks, Config, Error, HseMode, L012HseTarget, MAX_FLASH_WAIT};

// A phase image contains only separately acknowledged owned fields. It is not
// an observed-register cache: full readbacks are never assigned into it.
#[cfg(rcc_lsi_sysclk)]
#[derive(Clone, Copy)]
pub(super) struct LsiSysclkPhase {
    pub(super) clock: pac::sysctrl::regs::Cr0,
    pub(super) sources: pac::sysctrl::regs::Cr1,
    pub(super) hsi: pac::sysctrl::regs::Hsi,
    pub(super) hse: pac::sysctrl::regs::Hse,
    pub(super) hsi_ready: bool,
    pub(super) hse_ready: bool,
}

#[cfg(rcc_lsi_sysclk)]
impl LsiSysclkPhase {
    fn matches(self, mut observed: Self) -> Result<(), Error> {
        let mut expected = self;
        expected.clock.set_key(0);
        observed.clock.set_key(0);
        expected.sources.set_key(0);
        observed.sources.set_key(0);
        if expected.clock.0 != observed.clock.0
            || expected.sources.0 != observed.sources.0
            || (expected.hsi.0 ^ observed.hsi.0) & crate::RCC_LSI_HSI_PARAMETERS_MASK != 0
            || expected.hsi_ready != observed.hsi_ready
            || (expected.hse.0 ^ observed.hse.0) & crate::RCC_LSI_HSE_PARAMETERS_MASK != 0
            || expected.hse_ready != observed.hse_ready
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        Ok(())
    }
}

// Accept a field only at its last acknowledged value or its requested value.
// Once expected advances to target, returning to the former value is rejected.
#[cfg(rcc_lsi_sysclk)]
pub(super) fn lsi_ack_field<T: Copy + Eq>(observed: T, expected: T, target: T) -> Result<T, Error> {
    if observed == expected || observed == target {
        Ok(observed)
    } else {
        Err(Error::ClockConfigurationTimeout)
    }
}

#[cfg(rcc_lsi_sysclk)]
#[derive(Clone, Copy)]
pub(super) struct LsiSysclkSnapshot {
    pub(super) clock: pac::sysctrl::regs::Cr0,
    pub(super) sources: pac::sysctrl::regs::Cr1,
    pub(super) hsi: pac::sysctrl::regs::Hsi,
    pub(super) hse: pac::sysctrl::regs::Hse,
    pub(super) lsi: pac::sysctrl::regs::Lsi,
    pub(super) lse: pac::sysctrl::regs::Lse,
    pub(super) routes: pac::sysctrl::regs::Cr2,
    pub(super) interrupts: pac::sysctrl::regs::Ier,
    pub(super) mco: pac::sysctrl::regs::Mco,
    pub(super) factory_raw: u16,
    pub(super) factory_trim: Option<u16>,
}

#[cfg(rcc_lsi_sysclk)]
impl LsiSysclkSnapshot {
    pub(super) fn phase(self) -> LsiSysclkPhase {
        LsiSysclkPhase {
            clock: self.clock,
            sources: self.sources,
            hsi: self.hsi,
            hse: self.hse,
            hsi_ready: self.hsi.stable(),
            hse_ready: self.hse.stable(),
        }
    }

    fn faults(self, config: Config) -> Result<(), Error> {
        check_external_faults(
            config.hse.is_some() || self.sources.hseen() || self.sources.hseccs(),
            self.sources.lseen() || self.sources.lseccs(),
        )
    }

    pub(super) fn admit(self, config: Config) -> Result<(), Error> {
        self.faults(config)?;
        if self.factory_raw == u16::MAX || Some(self.lsi.trim()) != self.factory_trim {
            return Err(Error::InvalidLsiCalibration);
        }
        if !self.sources.lsien() || !self.lsi.stable() {
            return Err(Error::LsiNotReady);
        }
        Ok(())
    }

    // Every observation checks relevant faults on both sides. A closed/reset
    // FLASH is never read; its access refusal is buffered behind source policy.
    // Only the supplied named-field acknowledgment may vary the phase image.
    fn check<R>(
        self,
        config: Config,
        flash: Option<pac::flash::regs::Cr2>,
        old_wait: u8,
        new_wait: u8,
        verify: impl FnOnce(LsiSysclkPhase) -> Result<R, Error>,
    ) -> Result<(R, u8), Error> {
        self.faults(config)?;
        let r = pac::SYSCTRL;
        let clock = r.cr0().read();
        let sources = r.cr1().read();
        let hsi = r.hsi().read();
        let hse = r.hse().read();
        let observed = LsiSysclkPhase {
            clock,
            sources,
            hsi,
            hse,
            hsi_ready: hsi.stable(),
            hse_ready: hse.stable(),
        };
        let lsi = r.lsi().read();
        let lse = r.lse().read();
        let mut routes = r.cr2().read();
        let interrupts = r.ier().read();
        let mco = r.mco().read();
        let info = <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO;
        let actual_flash = if flash.is_some() && info.is_enabled() && !info.reset_asserted() {
            let value = pac::FLASH.cr2().read();
            (info.is_enabled() && !info.reset_asserted()).then_some(value)
        } else {
            None
        };
        self.faults(config)?;
        let mut expected_routes = self.routes;
        routes.set_key(0);
        expected_routes.set_key(0);
        if flash.is_some() {
            expected_routes.set_flashwait(routes.flashwait());
        }
        let mut expected_sources = self.sources;
        let mut actual_sources = observed.sources;
        expected_sources.set_key(0);
        actual_sources.set_key(0);
        // These are the only fields any owned phase can change in CR1.
        expected_sources.set_hsien(actual_sources.hsien());
        expected_sources.set_hseen(actual_sources.hseen());
        expected_sources.set_clkccs(actual_sources.clkccs());
        if routes.0 != expected_routes.0
            || actual_sources.0 != expected_sources.0
            || !observed.sources.lsien()
            || (lsi.0 ^ self.lsi.0) & crate::RCC_LSI_PARAMETERS_MASK != 0
            || (lse.0 ^ self.lse.0) & crate::RCC_LSI_LSE_PARAMETERS_MASK != 0
            || lse.stable() != self.lse.stable()
            || interrupts.0 != self.interrupts.0
            || mco.0 != self.mco.0
        {
            return Err(Error::ClockConfigurationTimeout);
        }
        let result = verify(observed)?;
        if !lsi.stable() {
            return Err(Error::LsiNotReady);
        }
        if let Some(mut expected_flash) = flash {
            let mut current = actual_flash.ok_or(Error::ClockConfigurationTimeout)?;
            let wait = current.wait();
            current.set_key(0);
            expected_flash.set_key(0);
            expected_flash.set_wait(wait);
            if current.0 != expected_flash.0
                || !((wait == new_wait && routes.flashwait() == new_wait)
                    || (wait == old_wait && routes.flashwait() == old_wait))
            {
                return Err(Error::FlashLatencyTimeout);
            }
            Ok((result, wait))
        } else {
            Ok((result, self.routes.flashwait()))
        }
    }

    pub(super) fn settled(
        self,
        config: Config,
        phase: LsiSysclkPhase,
        flash: Option<pac::flash::regs::Cr2>,
        wait: u8,
    ) -> Result<(), Error> {
        self.check(config, flash, wait, wait, |observed| {
            phase.matches(observed)
        })
        .map(|_| ())
    }

    // This helper only polls. Writes and their pre-write settled guards remain
    // at their actual source-use edges in configure and finish_lsi_sysclk.
    // Share the polling loop across clock transitions; only the field update
    // depends on which transition is being acknowledged.
    #[inline(never)]
    pub(super) fn acknowledge(
        self,
        config: Config,
        phase: &mut LsiSysclkPhase,
        flash: pac::flash::regs::Cr2,
        wait: u8,
        error: Error,
        advance: &mut dyn FnMut(&mut LsiSysclkPhase, LsiSysclkPhase) -> Result<bool, Error>,
    ) -> Result<(), Error> {
        for _ in 0..config.timeout {
            let (done, _) = self.check(config, Some(flash), wait, wait, |observed| {
                let done = advance(phase, observed)?;
                phase.matches(observed)?;
                Ok(done)
            })?;
            if done {
                self.settled(config, *phase, Some(flash), wait)?;
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(error)
    }

    // The final selector is interpreted only after non-selector phase,
    // live-LSI readiness and both FLASH interfaces have passed the full guard.
    fn final_source(
        self,
        config: Config,
        phase: LsiSysclkPhase,
        flash: pac::flash::regs::Cr2,
        wait: u8,
    ) -> Result<ClockSource, Error> {
        let (source, _) = self.check(config, Some(flash), wait, wait, |mut observed| {
            let source = observed.clock.sysclk();
            observed.clock.set_sysclk(phase.clock.sysclk());
            phase.matches(observed)?;
            Ok(source)
        })?;
        Ok(source)
    }

    pub(super) fn write_wait(
        self,
        config: Config,
        phase: LsiSysclkPhase,
        flash: pac::flash::regs::Cr2,
        old_wait: u8,
        new_wait: u8,
    ) -> Result<(), Error> {
        self.settled(config, phase, Some(flash), old_wait)?;
        let info = <crate::peripherals::FLASH as crate::rcc::SealedRccPeripheral>::RCC_INFO;
        if !info.is_enabled() || info.reset_asserted() {
            self.settled(config, phase, Some(flash), old_wait)?;
            return Err(Error::ClockConfigurationTimeout);
        }
        pac::FLASH.cr2().modify(|w| {
            w.set_key(0x5a5a);
            w.set_wait(new_wait);
        });
        let accessible = info.is_enabled() && !info.reset_asserted();
        for _ in 0..config.timeout {
            let (_, wait) = self.check(config, Some(flash), old_wait, new_wait, |observed| {
                phase.matches(observed)
            })?;
            if !accessible {
                return Err(Error::ClockConfigurationTimeout);
            }
            if wait == new_wait {
                barrier();
                return self.settled(config, phase, Some(flash), new_wait);
            }
            core::hint::spin_loop();
        }
        Err(Error::FlashLatencyTimeout)
    }
}

#[cfg(rcc_lsi_sysclk)]
pub(super) fn finish_lsi_sysclk(
    config: Config,
    mut clocks: Clocks,
    entry: LsiSysclkSnapshot,
    mut phase: LsiSysclkPhase,
    flash: pac::flash::regs::Cr2,
    cs: critical_section::CriticalSection<'_>,
) -> Result<Clocks, Error> {
    let r = pac::SYSCTRL;
    entry.settled(config, phase, Some(flash), MAX_FLASH_WAIT as u8)?;
    if let Some(hse) = config.hse {
        let result = crate::rcc_hse_pins_match(
            hse.mode == HseMode::Bypass,
            config.timeout,
            cs,
            L012HseTarget::LiveLsiSysclk,
        );
        if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = result {
            return Err(error);
        }
        entry.settled(config, phase, Some(flash), MAX_FLASH_WAIT as u8)?;
        if !result? {
            return Err(Error::ClockConfigurationTimeout);
        }
    }
    entry.settled(config, phase, Some(flash), MAX_FLASH_WAIT as u8)?;
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(ClockSource::Hsi);
        w.set_hclkprs(config.ahb_pre as u8);
        w.set_pclkprs(config.apb_pre as u8);
    });
    entry.acknowledge(
        config,
        &mut phase,
        flash,
        MAX_FLASH_WAIT as u8,
        Error::ClockConfigurationTimeout,
        &mut |expected, observed| {
            expected.clock.set_hclkprs(lsi_ack_field(
                observed.clock.hclkprs(),
                expected.clock.hclkprs(),
                config.ahb_pre as u8,
            )?);
            expected.clock.set_pclkprs(lsi_ack_field(
                observed.clock.pclkprs(),
                expected.clock.pclkprs(),
                config.apb_pre as u8,
            )?);
            Ok(expected.clock.hclkprs() == config.ahb_pre as u8
                && expected.clock.pclkprs() == config.apb_pre as u8)
        },
    )?;
    barrier();
    entry.settled(config, phase, Some(flash), MAX_FLASH_WAIT as u8)?;
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
                .maximum()
                .0,
        );
    let final_wait = (upper_hclk - 1) / crate::RCC_FLASH_WAIT_STEP_HZ;
    if final_wait > MAX_FLASH_WAIT {
        return Err(Error::FlashLatencyTimeout);
    }
    let final_wait = final_wait as u8;
    entry.write_wait(config, phase, flash, MAX_FLASH_WAIT as u8, final_wait)?;
    entry.settled(config, phase, Some(flash), final_wait)?;
    // LAST clock/source/divider/FLASH write. Only reads and independently
    // required borrowed GPIO gate restoration are reachable below this edge.
    r.cr0().modify(|w| {
        w.set_key(0x5a5a);
        w.set_sysclk(ClockSource::Lsi);
    });
    for attempt in 0..config.timeout {
        let source = entry.final_source(config, phase, flash, final_wait)?;
        if source == ClockSource::Lsi {
            phase.clock.set_sysclk(ClockSource::Lsi);
            break;
        }
        if source != ClockSource::Hsi || attempt + 1 == config.timeout {
            return Err(Error::LsiClockSwitchTimeout);
        }
        core::hint::spin_loop();
    }
    let verify_lsi = || {
        if entry.final_source(config, phase, flash, final_wait)? != ClockSource::Lsi {
            return Err(Error::LsiClockSwitchTimeout);
        }
        Ok(())
    };
    verify_lsi()?;
    barrier();
    verify_lsi()?;
    if let Some(hse) = config.hse {
        let result = crate::rcc_hse_pins_match(
            hse.mode == HseMode::Bypass,
            config.timeout,
            cs,
            L012HseTarget::LiveLsiSysclk,
        );
        if let Err(error @ Error::LsiConfigurationGateTimeout { .. }) = result {
            return Err(error);
        }
        verify_lsi()?;
        if !result? {
            return Err(Error::ClockConfigurationTimeout);
        }
    }
    verify_lsi()?;
    if entry.sources.hseen() {
        if !entry.hse.mode() {
            clocks.hse = Some(HseMode::Oscillator);
        } else if clocks.hse.is_none() {
            clocks.hse = Some(HseMode::Bypass);
        }
    }
    Ok(clocks)
}

