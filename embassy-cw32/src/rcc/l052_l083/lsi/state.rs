use super::*;

#[cfg(rcc_lsi_sysclk)]
impl LsiSysclkState {
    pub(super) fn capture(config: Config) -> Result<Self, Error> {
        // No inspection gate or source/pad/Flash write precedes this record.
        let entry = LsiIdentity::read();
        let raw_hsi =
            unsafe { core::ptr::read_volatile(crate::RCC_FACTORY_HSI_TRIM_ADDRESS as *const u16) };
        let raw_lsi =
            unsafe { core::ptr::read_volatile(crate::RCC_LSI_FACTORY_TRIM_ADDRESS as *const u16) };
        let monitor_hse = config.hse.is_some()
            || entry.sources.hseen()
            || entry.sources.hseccs()
            || entry.clock.sysclk() == ClockSource::Hse;
        #[cfg(rcc_cw32l083_v1)]
        let monitor_hse = monitor_hse
            || (entry.sources.pllen()
                && matches!(
                    entry.pll.source(),
                    pac::sysctrl::vals::PllSource::HseCrystal
                        | pac::sysctrl::vals::PllSource::HseBypass
                ));
        let monitor_lse = config.lse.is_some()
            || entry.sources.lseen()
            || entry.sources.lseccs()
            || entry.clock.sysclk() == ClockSource::Lse;
        entry.external_faults(monitor_hse, monitor_lse)?;
        // Reject erased halfwords before native TRIM masking. Zero is valid.
        if raw_hsi == u16::MAX {
            return Err(Error::InvalidCalibration);
        }
        if raw_lsi == u16::MAX {
            return Err(Error::InvalidLsiCalibration);
        }
        let mut factory_hsi = entry.hsi;
        factory_hsi.set_trim(raw_hsi);
        let mut factory_lsi = entry.lsi;
        factory_lsi.set_trim(raw_lsi);
        if !matches!(
            entry.clock.sysclk(),
            ClockSource::Hsi | ClockSource::Hse | ClockSource::Lsi | ClockSource::Lse
        ) && {
            #[cfg(rcc_cw32l083_v1)]
            {
                entry.clock.sysclk() != ClockSource::Pll
            }
            #[cfg(rcc_cw32l052_v1)]
            {
                true
            }
        } {
            return Err(Error::InvalidClockSource);
        }
        if !matches!(entry.hsi.div(), 5 | 6 | 8 | 9 | 11..=15) {
            return Err(Error::InvalidHsiDivider);
        }
        if entry.hsi.stable() != entry.flags.hsistable()
            || entry.hse.stable() != entry.flags.hsestable()
            || entry.lse.stable() != entry.flags.lsestable()
            || entry.sources.hsien() != entry.hsi.stable()
            || entry.sources.hseen() != entry.hse.stable()
            || entry.sources.lseen() != entry.lse.stable()
            || !entry.selected_ready()
        {
            return Err(Error::InvalidEntryClock);
        }
        #[cfg(rcc_cw32l083_v1)]
        if entry.sources.pllen() != entry.pll.stable()
            || entry.pll.stable() != entry.flags.pllstable()
            || entry.cr2.flashwait() > 2
            || entry.flash_reset
        {
            return Err(Error::InvalidEntryClock);
        }
        let selected_lsi = entry.clock.sysclk() == ClockSource::Lsi;
        if (entry.sources.lsien() || selected_lsi || entry.lsi.stable() || entry.flags.lsistable())
            && entry.lsi.trim() != factory_lsi.trim()
        {
            return Err(Error::LsiCalibrationInUse);
        }
        if entry.lsi.stable() != entry.flags.lsistable() {
            return Err(Error::LsiClockInUse);
        }
        let detector = entry.sources.hseccs() || entry.sources.lseccs();
        if detector && !(entry.sources.lsien() && entry.lsi.stable()) {
            return Err(Error::LsiClockInUse);
        }
        let class = if !entry.sources.lsien() {
            if selected_lsi
                || entry.lsi.stable()
                || detector
                || entry.ier.lsirdy()
                || entry.flags.lsirdy()
                || entry.pending
            {
                return Err(Error::LsiClockInUse);
            }
            LsiEntryClass::Cold
        } else if entry.lsi.stable() {
            LsiEntryClass::LiveReady
        } else {
            if selected_lsi || detector {
                return Err(Error::LsiClockInUse);
            }
            LsiEntryClass::LiveStarting
        };
        Ok(Self {
            entry,
            class,
            factory_lsi: factory_lsi.trim(),
            factory_hsi: factory_hsi.trim(),
            clock: entry.clock,
            sources: entry.sources,
            hsi: entry.hsi,
            hse: entry.hse,
            lse: entry.lse,
            lsi: entry.lsi,
            ready: [
                entry.hsi.stable(),
                entry.hse.stable(),
                entry.lse.stable(),
                entry.lsi.stable(),
            ],
            requested: false,
            lsirdy_seen: entry.flags.lsirdy(),
            pending_seen: entry.pending,
            cold_lse_requested: config.lse.is_some() && !entry.sources.lseen(),
            lse_config: config.lse,
            lse_pads_configured: false,
            lse_requested: false,
            lserdy_seen: entry.flags.lserdy(),
            monitor_hse,
            monitor_lse,
            timeout: config.timeout,
            consumers: None,
            pads: None,
            lse_preflight: None,
            #[cfg(rcc_cw32l083_v1)]
            pll_stopped: !entry.sources.pllen(),
            #[cfg(rcc_cw32l083_v1)]
            pll_ready: entry.pll.stable(),
            #[cfg(rcc_cw32l083_v1)]
            pll_bridge: None,
            #[cfg(rcc_cw32l083_v1)]
            cr2: entry.cr2,
            #[cfg(rcc_cw32l083_v1)]
            flash_gate: entry.flash_gate,
            #[cfg(rcc_cw32l083_v1)]
            flash_cr2: None,
            #[cfg(rcc_cw32l083_v1)]
            events: [
                entry.flags.hsirdy(),
                entry.flags.hserdy(),
                entry.flags.lserdy(),
                entry.flags.lsirdy(),
            ],
            #[cfg(rcc_cw32l083_v1)]
            event_owned: [false; 4],
        })
    }

    pub(super) fn check_identity(&mut self, phase: LsiTransition) -> Result<LsiIdentity, Error> {
        let actual = LsiIdentity::read();
        actual.external_faults(self.monitor_hse, self.monitor_lse)?;
        let clock_target = match phase {
            LsiTransition::Clock(target) => target,
            _ => self.clock,
        };
        let sources_target = match phase {
            #[cfg(rcc_cw32l083_v1)]
            LsiTransition::PllStopping(target) => target,
            LsiTransition::LsiRequest(target)
            | LsiTransition::HsiStopRequest(target)
            | LsiTransition::HsiRequest(target)
            | LsiTransition::HseRequest(target)
            | LsiTransition::LseRequest(target) => target,
            _ => self.sources,
        };
        let hsi_target = match phase {
            LsiTransition::HsiTrim(target) | LsiTransition::HsiDivider(target) => target,
            _ => self.hsi,
        };
        let hse_target = match phase {
            LsiTransition::HseParameters(target) => target,
            _ => self.hse,
        };
        let lse_target = match phase {
            LsiTransition::LseParameters(target) => target,
            _ => self.lse,
        };
        let lsi_target = match phase {
            LsiTransition::LsiTrim(target) => target,
            _ => self.lsi,
        };
        let hsi_word = actual.hsi.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK;
        let hse_word = actual.hse.0 & crate::RCC_LSI_HSE_PARAMETERS_MASK;
        let lse_word = actual.lse.0 & crate::RCC_LSI_LSE_PARAMETERS_MASK;
        let lsi_word = actual.lsi.0 & crate::RCC_LSI_PARAMETERS_MASK;
        if (actual.clock.0 != self.clock.0 && actual.clock.0 != clock_target.0)
            || (actual.sources.0 != self.sources.0 && actual.sources.0 != sources_target.0)
            || actual.ier.0 != self.entry.ier.0
            || (hsi_word != self.hsi.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK
                && hsi_word != hsi_target.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK)
            || (hse_word != self.hse.0 & crate::RCC_LSI_HSE_PARAMETERS_MASK
                && hse_word != hse_target.0 & crate::RCC_LSI_HSE_PARAMETERS_MASK)
            || (lse_word != self.lse.0 & crate::RCC_LSI_LSE_PARAMETERS_MASK
                && lse_word != lse_target.0 & crate::RCC_LSI_LSE_PARAMETERS_MASK)
            || (lsi_word != self.lsi.0 & crate::RCC_LSI_PARAMETERS_MASK
                && lsi_word != lsi_target.0 & crate::RCC_LSI_PARAMETERS_MASK)
        {
            return Err(Error::LsiClockInUse);
        }
        #[cfg(rcc_cw32l083_v1)]
        self.check_l083_identity(actual, phase)?;
        let observed = [
            actual.hsi.stable(),
            actual.hse.stable(),
            actual.lse.stable(),
            actual.lsi.stable(),
        ];
        let mirrors = [
            actual.flags.hsistable(),
            actual.flags.hsestable(),
            actual.flags.lsestable(),
            actual.flags.lsistable(),
        ];
        let starts = [
            matches!(
                phase,
                LsiTransition::HsiRequest(_) | LsiTransition::HsiReady
            ),
            matches!(
                phase,
                LsiTransition::HseRequest(_) | LsiTransition::HseReady
            ),
            matches!(
                phase,
                LsiTransition::LseRequest(_) | LsiTransition::LseReady
            ),
            self.requested
                && matches!(
                    phase,
                    LsiTransition::LsiRequest(_) | LsiTransition::LsiReady
                ),
        ];
        let stopping_hsi = matches!(
            phase,
            LsiTransition::HsiStopRequest(_) | LsiTransition::HsiStopped
        );
        let lost = [
            Error::HsiTimeout,
            Error::HseTimeout,
            Error::LseNotReady,
            Error::LsiTimeout,
        ];
        for index in 0..4 {
            if self.ready[index]
                && !(index == 0 && stopping_hsi)
                && (!observed[index] || !mirrors[index])
            {
                return Err(lost[index]);
            }
            if observed[index] != mirrors[index] {
                return Err(Error::LsiClockInUse);
            }
            if index == 0 && stopping_hsi {
                if !self.ready[index] && observed[index] {
                    return Err(Error::LsiClockInUse);
                }
            } else if observed[index] != self.ready[index] && !starts[index] {
                return Err(Error::LsiClockInUse);
            }
        }
        if !actual.selected_ready() {
            return Err(Error::InvalidEntryClock);
        }
        #[cfg(rcc_cw32l052_v1)]
        if !self.requested {
            // Live-starting may not be resampled as live-ready during admission.
            if actual.lsi.stable() != self.entry.lsi.stable()
                || actual.flags.lsirdy() != self.entry.flags.lsirdy()
                || actual.pending != self.entry.pending
            {
                return Err(Error::LsiClockInUse);
            }
        } else if (self.lsirdy_seen && !actual.flags.lsirdy())
            || (self.pending_seen && !actual.pending)
        {
            return Err(Error::LsiClockInUse);
        }
        if self.cold_lse_requested {
            if (!self.lse_requested && actual.flags.lserdy() != self.entry.flags.lserdy())
                || (self.lse_requested && self.lserdy_seen && !actual.flags.lserdy())
            {
                return Err(Error::LseClockInUse);
            }
        }
        self.ready = observed;
        self.lsirdy_seen |= actual.flags.lsirdy();
        self.pending_seen |= actual.pending;
        self.lserdy_seen |= actual.flags.lserdy();
        // Advance only the known owned delta, immediately after its complete
        // normalized acknowledgment. A later old value is then a refusal.
        if actual.clock.0 == clock_target.0 {
            self.clock = clock_target;
        }
        if actual.sources.0 == sources_target.0 {
            self.sources = sources_target;
        }
        if hsi_word == hsi_target.0 & crate::RCC_LSI_HSI_PARAMETERS_MASK {
            self.hsi = hsi_target;
        }
        if hse_word == hse_target.0 & crate::RCC_LSI_HSE_PARAMETERS_MASK {
            self.hse = hse_target;
        }
        if lse_word == lse_target.0 & crate::RCC_LSI_LSE_PARAMETERS_MASK {
            self.lse = lse_target;
        }
        if lsi_word == lsi_target.0 & crate::RCC_LSI_PARAMETERS_MASK {
            self.lsi = lsi_target;
        }
        Ok(actual)
    }

    #[cfg(rcc_cw32l083_v1)]
    fn check_l083_identity(
        &mut self,
        actual: LsiIdentity,
        phase: LsiTransition,
    ) -> Result<(), Error> {
        let (system_target, flash_target) = match phase {
            LsiTransition::FlashWait(system, flash) => (system, Some(flash)),
            _ => (self.cr2, self.flash_cr2),
        };
        if (actual.cr2.0 != self.cr2.0 && actual.cr2.0 != system_target.0)
            || actual.irq_enabled != self.entry.irq_enabled
            || actual.fault_irq_enabled != self.entry.fault_irq_enabled
            || actual.fault_pending != self.entry.fault_pending
            || actual.flash_gate != self.flash_gate
            || actual.flash_reset != self.entry.flash_reset
        {
            return Err(Error::LsiClockInUse);
        }
        if let Some(expected) = self.flash_cr2 {
            // The central gate and reset were checked before this local read.
            let mut flash = pac::FLASH.cr2().read();
            flash.0 &= !crate::RCC_L083_LSI_FLASH_CR2_KEY_MASK;
            let target = flash_target.ok_or(Error::FlashLatencyTimeout)?;
            if flash.0 != expected.0 && flash.0 != target.0 {
                return Err(Error::FlashLatencyTimeout);
            }
            if !matches!(phase, LsiTransition::FlashWait(_, _))
                && flash.wait() != actual.cr2.flashwait()
            {
                return Err(Error::FlashLatencyTimeout);
            }
            if actual.cr2.0 == system_target.0 && flash.0 == target.0 {
                self.cr2 = system_target;
                self.flash_cr2 = Some(target);
            }
        }
        if actual.pll.0 & crate::RCC_LSI_PLL_PARAMETERS_MASK
            != self.entry.pll.0 & crate::RCC_LSI_PLL_PARAMETERS_MASK
            || actual.pll.stable() != actual.flags.pllstable()
        {
            return Err(Error::PllConfigurationTimeout);
        }
        // SYSCLK departure alone never releases the original reference.
        if self.entry.sources.pllen() && !self.pll_stopped {
            if matches!(
                phase,
                LsiTransition::HsiStopRequest(_)
                    | LsiTransition::HsiStopped
                    | LsiTransition::HsiTrim(_)
                    | LsiTransition::HsiDivider(_)
                    | LsiTransition::LsiTrim(_)
                    | LsiTransition::LsiRequest(_)
            ) {
                return Err(Error::PllStopTimeout);
            }
            match self.entry.pll.source() {
                pac::sysctrl::vals::PllSource::Hsi => {
                    if !actual.sources.hsien()
                        || !actual.hsi.stable()
                        || !actual.flags.hsistable()
                        || actual.hsi.0 != self.entry.hsi.0
                    {
                        return Err(Error::PllConfigurationTimeout);
                    }
                }
                pac::sysctrl::vals::PllSource::HseCrystal
                | pac::sysctrl::vals::PllSource::HseBypass => {
                    if !actual.sources.hseen()
                        || !actual.hse.stable()
                        || !actual.flags.hsestable()
                        || actual.hse.0 != self.entry.hse.0
                    {
                        return Err(Error::PllConfigurationTimeout);
                    }
                }
                _ => return Err(Error::PllConfigurationTimeout),
            }
        }
        if self.pll_stopped {
            if actual.sources.pllen() || actual.pll.stable() {
                return Err(Error::PllStopTimeout);
            }
        } else if matches!(phase, LsiTransition::PllStopping(_)) {
            if !self.pll_ready && actual.pll.stable() {
                return Err(Error::PllStopTimeout);
            }
            self.pll_ready = actual.pll.stable();
            if !actual.sources.pllen() && !actual.pll.stable() {
                self.pll_stopped = true;
            }
        } else if !actual.sources.pllen() || !actual.pll.stable() {
            return Err(Error::PllConfigurationTimeout);
        }
        let enabled = [
            actual.sources.hsien(),
            actual.sources.hseen(),
            actual.sources.lseen(),
            actual.sources.lsien(),
        ];
        let ready = [
            actual.hsi.stable(),
            actual.hse.stable(),
            actual.lse.stable(),
            actual.lsi.stable(),
        ];
        let stopping_hsi = matches!(
            phase,
            LsiTransition::HsiStopRequest(_) | LsiTransition::HsiStopped
        );
        for index in 0..4 {
            if ready[index] && !enabled[index] && !(index == 0 && stopping_hsi) {
                return Err(Error::LsiClockInUse);
            }
        }
        if self.requested
            && !actual.sources.lsien()
            && !matches!(phase, LsiTransition::LsiRequest(_))
        {
            return Err(Error::LsiClockInUse);
        }
        let events = [
            actual.flags.hsirdy(),
            actual.flags.hserdy(),
            actual.flags.lserdy(),
            actual.flags.lsirdy(),
        ];
        let entry_events = [
            self.entry.flags.hsirdy(),
            self.entry.flags.hserdy(),
            self.entry.flags.lserdy(),
            self.entry.flags.lsirdy(),
        ];
        let event_enabled = [
            actual.ier.hsirdy(),
            actual.ier.hserdy(),
            actual.ier.lserdy(),
            actual.ier.lsirdy(),
        ];
        for index in 0..4 {
            if (!self.event_owned[index] && events[index] != entry_events[index])
                || (self.events[index] && !events[index])
            {
                return Err(Error::LsiClockInUse);
            }
        }
        if actual.flags.pllrdy() != self.entry.flags.pllrdy()
            || (actual.flags.0
                & !(crate::RCC_L083_LSI_ISR_READY_MASK | crate::RCC_L083_LSI_ISR_EVENT_MASK))
                != (self.entry.flags.0
                    & !(crate::RCC_L083_LSI_ISR_READY_MASK | crate::RCC_L083_LSI_ISR_EVENT_MASK))
        {
            return Err(Error::LsiClockInUse);
        }
        if !self.requested && self.class != LsiEntryClass::LiveReady {
            // A prior owned HSI request cannot erase a cold SYSCTRL observer.
            if actual.pending != self.entry.pending
                || actual.lsi.stable() != self.entry.lsi.stable()
            {
                return Err(Error::LsiClockInUse);
            }
        } else if (self.pending_seen && !actual.pending)
            || (!self.pending_seen
                && actual.pending
                && !(0..4).any(|index| {
                    self.event_owned[index]
                        && events[index]
                        && !entry_events[index]
                        && event_enabled[index]
                }))
        {
            return Err(Error::LsiClockInUse);
        }
        self.events = events;
        Ok(())
    }

    #[cfg(rcc_cw32l083_v1)]
    fn hsi_divisor(&self) -> Result<u32, Error> {
        match self.entry.hsi.div() {
            5 => Ok(6),
            6 => Ok(1),
            8 => Ok(2),
            9 => Ok(4),
            11 => Ok(8),
            12 => Ok(10),
            13 => Ok(12),
            14 => Ok(14),
            15 => Ok(16),
            _ => Err(Error::InvalidHsiDivider),
        }
    }

    #[cfg(rcc_cw32l083_v1)]
    pub(super) fn validate_edge(
        config: Config,
        clocks: Clocks,
        source: crate::rcc::ClockBounds,
        clock: pac::sysctrl::regs::Cr0,
        wait: u8,
    ) -> Result<(), Error> {
        let ahb = 1u32 << clock.hclkprs();
        let apb = 1u32 << clock.pclkprs();
        crate::rcc::operating::validate(
            config.operating_conditions,
            Clocks {
                source,
                dividers: [config.hsi.div.divisor(), ahb, ahb * apb],
                ..clocks
            },
        )?;
        if wait > 2
            || source
                .divided_by(ahb)
                .maximum_exceeds((u32::from(wait) + 1) * crate::RCC_FLASH_WAIT_STEP_HZ)
        {
            return Err(Error::FlashLatencyTimeout);
        }
        Ok(())
    }

    #[cfg(rcc_cw32l083_v1)]
    fn fallback_bound(config: Config, clocks: Clocks) -> Result<(), Error> {
        crate::rcc::operating::validate(
            config.operating_conditions,
            Clocks {
                source: crate::rcc::ClockBounds::hsi(1),
                dividers: [config.hsi.div.divisor(), 1, 1],
                ..clocks
            },
        )
    }

    #[cfg(rcc_cw32l083_v1)]
    pub(super) fn admit_l083(&mut self, config: Config, clocks: Clocks) -> Result<(), Error> {
        use pac::sysctrl::vals::PllSource as Reference;
        let entry = self.entry;
        let pll = entry.pll;
        if pll.reserved_debug().to_bits() != crate::RCC_PLL_RESERVED_DEBUG_DEFAULT {
            return Err(Error::PllReservedConfiguration);
        }
        // Reconstruct only documented fields in a local value. All unnamed
        // RFU bits must retain their own documented reset default of zero;
        // no PLL register write occurs, and STABLE is checked separately.
        let mut defined = pac::sysctrl::regs::Pll::default();
        defined.set_source(pll.source());
        defined.set_freqin(pll.freqin());
        defined.set_mul(pll.mul());
        defined.set_freqout(pll.freqout());
        defined.set_waitcycle(pll.waitcycle());
        defined.set_reserved_debug(pll.reserved_debug());
        if pll.0 & crate::RCC_LSI_PLL_PARAMETERS_MASK != defined.0 {
            return Err(Error::PllReservedConfiguration);
        }
        let multiplier = pll.mul().to_bits();
        if !(crate::RCC_PLL_MULTIPLIER_RANGE.0..=crate::RCC_PLL_MULTIPLIER_RANGE.1)
            .contains(&multiplier)
        {
            return Err(Error::InvalidPllMultiplier);
        }
        if !matches!(
            pll.source(),
            Reference::Hsi | Reference::HseCrystal | Reference::HseBypass
        ) {
            return Err(Error::PllConfigurationTimeout);
        }
        if matches!(pll.source(), Reference::HseCrystal | Reference::HseBypass)
            && entry.hse.mode() != (pll.source() == Reference::HseBypass)
        {
            return Err(Error::PllConfigurationTimeout);
        }
        // Every encoded WAIT/input/output bin is documented. Inactive PLL
        // parameters remain defined without claiming they describe a live rate.
        let pll_bounds = if entry.sources.pllen() {
            let c = config.operating_conditions;
            if c.supply_mv < crate::RCC_PLL_SUPPLY_MV.0
                || c.supply_mv > crate::RCC_PLL_SUPPLY_MV.1
                || c.min_temperature_c < crate::RCC_PLL_TEMPERATURE_C.0
                || c.max_temperature_c > crate::RCC_PLL_TEMPERATURE_C.1
            {
                return Err(Error::PllConditionsOutsideQualifiedRange);
            }
            let input = match pll.source() {
                Reference::Hsi => {
                    if !entry.sources.hsien()
                        || !entry.hsi.stable()
                        || entry.hsi.trim() != self.factory_hsi
                    {
                        return Err(Error::HsiClockInUse);
                    }
                    crate::rcc::ClockBounds::hsi(self.hsi_divisor()?)
                }
                Reference::HseCrystal | Reference::HseBypass => {
                    let hse = config.hse.ok_or(Error::HseNotConfigured)?;
                    if !entry.sources.hseen()
                        || !entry.hse.stable()
                        || entry.hse.mode() != (pll.source() == Reference::HseBypass)
                        || entry.hse.mode() != (hse.mode == HseMode::Bypass)
                    {
                        return Err(Error::PllConfigurationTimeout);
                    }
                    hse.bounds()?
                }
                _ => return Err(Error::PllConfigurationTimeout),
            };
            if input.minimum_below(crate::RCC_PLL_INPUT_RANGE_HZ.0)
                || input.maximum_exceeds(crate::RCC_PLL_INPUT_RANGE_HZ.1)
            {
                return Err(Error::PllInputOutsideQualifiedRange);
            }
            let input_bin = crate::RCC_PLL_INPUT_BINS_HZ[usize::from(pll.freqin().to_bits())];
            if input.minimum_below(input_bin.0) || input.maximum_exceeds(input_bin.1) {
                return Err(Error::PllInputCrossesBin);
            }
            let output = input
                .multiplied_by(u32::from(multiplier))
                .ok_or(Error::PllArithmeticOverflow)?;
            if output.minimum_below(crate::RCC_PLL_OUTPUT_RANGE_HZ.0)
                || output.maximum_exceeds(crate::RCC_PLL_OUTPUT_RANGE_HZ.1)
            {
                return Err(Error::PllOutputOutsideQualifiedRange);
            }
            // Native 1xx encodings all denote the documented 48..72 MHz bin.
            let output_index = match pll.freqout().to_bits() {
                0..=3 => pll.freqout().to_bits(),
                4..=7 => 4,
                _ => return Err(Error::PllOutputCrossesBin),
            };
            let output_bin = crate::RCC_PLL_OUTPUT_BINS_HZ[usize::from(output_index)];
            if output.minimum_below(output_bin.0) || output.maximum_exceeds(output_bin.1) {
                return Err(Error::PllOutputCrossesBin);
            }
            Some(output)
        } else {
            None
        };
        let source = match entry.clock.sysclk() {
            ClockSource::Hsi if entry.hsi.trim() == self.factory_hsi => {
                Some(crate::rcc::ClockBounds::hsi(self.hsi_divisor()?))
            }
            // Preserve the caller's safe, unchanged non-factory HSI interval;
            // it receives no invented absolute factory-rate bound.
            ClockSource::Hsi => None,
            ClockSource::Hse => Some(config.hse.ok_or(Error::HseNotConfigured)?.bounds()?),
            ClockSource::Lse => Some(
                config
                    .lse
                    .ok_or(Error::LseNotConfigured)?
                    .bounds(config.operating_conditions)?,
            ),
            ClockSource::Lsi => Some(crate::rcc::ClockBounds::lsi()),
            ClockSource::Pll => Some(pll_bounds.ok_or(Error::PllConfigurationTimeout)?),
            _ => return Err(Error::InvalidClockSource),
        };
        if let Some(source) = source {
            Self::validate_edge(config, clocks, source, entry.clock, entry.cr2.flashwait())?;
        }
        let fallback = entry.sources.clkccs()
            && matches!(entry.clock.sysclk(), ClockSource::Hse | ClockSource::Lse);
        if fallback {
            Self::fallback_bound(config, clocks)?;
            if !entry.sources.hsien() || !entry.hsi.stable() || entry.hsi.trim() != self.factory_hsi
            {
                return Err(Error::HsiClockInUse);
            }
            if entry.cr2.flashwait() != 2 {
                return Err(Error::FlashLatencyTimeout);
            }
        }
        if entry.clock.sysclk() == ClockSource::Pll {
            let mut guard = entry.clock;
            guard.set_hclkprs(entry.clock.hclkprs().max(AHBPrescaler::Div4 as u8));
            guard.set_pclkprs(entry.clock.pclkprs().max(APBPrescaler::Div8 as u8));
            self.pll_bridge = if entry.hsi.trim() == self.factory_hsi {
                Self::validate_edge(
                    config,
                    clocks,
                    crate::rcc::ClockBounds::hsi(self.hsi_divisor()?),
                    guard,
                    2,
                )?;
                Some(ClockSource::Hsi)
            } else if matches!(pll.source(), Reference::HseCrystal | Reference::HseBypass) {
                let hse = config.hse.ok_or(Error::HseNotConfigured)?;
                Self::validate_edge(config, clocks, hse.bounds()?, guard, 2)?;
                if entry.sources.clkccs() {
                    Self::fallback_bound(config, clocks)?;
                    // An HSE escape cannot bypass unprovable HSI fallback.
                    return Err(Error::HsiClockInUse);
                }
                Some(ClockSource::Hse)
            } else {
                return Err(Error::HsiClockInUse);
            };
        }
        Ok(())
    }

    #[cfg(rcc_cw32l083_v1)]
    pub(super) fn stop_l083_pll(
        &mut self,
        config: Config,
        cs: critical_section::CriticalSection<'_>,
    ) -> Result<(), Error> {
        if self.pll_stopped {
            return self.check_all(LsiTransition::Steady, cs);
        }
        if let Some(bridge) = self.pll_bridge {
            if bridge == ClockSource::Hsi && !self.sources.hsien() {
                self.check_all(LsiTransition::Steady, cs)?;
                let mut enabled = self.sources;
                enabled.set_hsien(true);
                self.event_owned[0] = true;
                pac::SYSCTRL.cr1().modify(|w| {
                    w.set_key(0x5a5a);
                    w.set_hsien(true);
                });
                self.wait(
                    LsiTransition::HsiRequest(enabled),
                    config.timeout,
                    Error::ClockConfigurationTimeout,
                    cs,
                )?;
                self.wait(
                    LsiTransition::HsiReady,
                    config.timeout,
                    Error::HsiTimeout,
                    cs,
                )?;
            }
            self.check_all(LsiTransition::Steady, cs)?;
            let mut target = self.clock;
            target.set_sysclk(bridge);
            pac::SYSCTRL.cr0().modify(|w| {
                w.set_key(0x5a5a);
                w.set_sysclk(bridge);
            });
            self.wait(
                LsiTransition::Clock(target),
                config.timeout,
                Error::TemporaryClockSwitchTimeout,
                cs,
            )?;
            barrier();
        }
        // Immutable entry PLLEN makes every collection repeat all bonded and
        // unbonded PLL-output owner checks right up to the stop use edge.
        self.check_all(LsiTransition::Steady, cs)?;
        if self.clock.sysclk() == ClockSource::Pll {
            return Err(Error::PllStopTimeout);
        }
        let mut stopped = self.sources;
        stopped.set_pllen(false);
        pac::SYSCTRL.cr1().modify(|w| {
            w.set_key(0x5a5a);
            w.set_pllen(false);
        });
        self.wait(
            LsiTransition::PllStopping(stopped),
            config.timeout,
            Error::PllStopTimeout,
            cs,
        )?;
        self.check_all(LsiTransition::Steady, cs)
    }
}
