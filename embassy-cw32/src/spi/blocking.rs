use super::*;

impl<'d> Spi<'d, Blocking, Master> {
    /// Transfer in place with one retry budget shared by all idle, TX and RX waits.
    /// On timeout the PIO controller is cleared and re-enabled before returning.
    pub fn blocking_transfer_in_place_bounded<W: Word>(
        &mut self, words: &mut [W], mut retries: u32,
    ) -> Result<(), Error> {
        let result = self.transfer_in_place_with(words, &mut || {
            retries = retries.checked_sub(1).ok_or(Error::PollTimeout)?;
            Ok(())
        });
        if matches!(result, Err(Error::PollTimeout)) {
            self.peripheral.info.enable(false);
            self.peripheral.info.clear();
            self.peripheral.info.enable(true);
        }
        result
    }

    /// Create a blocking bus, returning an error before any hardware changes if
    /// the clock or requested frequency is invalid.
    ///
    /// Pins and the peripheral are consumed even on error; pass `reborrow()`
    /// tokens if they must remain available to the caller after failure.
    pub fn new_blocking<T: Instance>(
        peripheral: Peri<'d, T>,
        sck: Peri<'d, impl SckPin<T>>,
        mosi: Peri<'d, impl MosiPin<T>>,
        miso: Peri<'d, impl MisoPin<T>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::new_inner(peripheral, sck, mosi, miso, config)
    }
}

impl<'d, M: PeriMode> Spi<'d, M, Master> {
    pub(super) fn new_inner<T: Instance>(
        peripheral: Peri<'d, T>,
        sck: Peri<'d, impl SckPin<T>>,
        mosi: Peri<'d, impl MosiPin<T>>,
        miso: Peri<'d, impl MisoPin<T>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let kernel_clock =
            crate::rcc::bus_clock_bounds::<T>().ok_or(ConfigError::ClockNotInitialized)?;
        let sck_af = sealed::SckPin::<T>::af(&*sck);
        let mosi_af = sealed::MosiPin::<T>::af(&*mosi);
        let miso_af = sealed::MisoPin::<T>::af(&*miso);
        let peripheral: Peri<'d, AnySpi> = peripheral.into();
        let info = peripheral.info;
        let prescaler = info.validate_config(kernel_clock, &config)?;

        // A safely reborrowed token can replace a forgotten async owner. Fence
        // its static ISR before Flex changes pins or RCC resets the controller.
        #[cfg(spi_async_pio)]
        info.pio_state().deactivate(info.regs);

        // GPIO Flex begins disconnected. Configure the SPI idle level before
        // connecting the SCK alternate function to the physical output.
        let mut sck = Flex::new(sck);
        let mut mosi = Flex::new(mosi);
        let mut miso = Flex::new(miso);
        critical_section::with(|cs| info.rcc.enable_and_reset_with_cs(cs))
            .expect("SPI clock gate did not enable");
        info.configure(&config, prescaler, 8);
        sck.set_as_af(sck_af, true, config.sck_pull());
        mosi.set_as_af(mosi_af, true, Pull::None);
        miso.set_as_af(miso_af, false, config.input_pull);

        Ok(Self {
            peripheral,
            sck: Some(sck),
            _mosi: Some(mosi),
            miso: Some(miso),
            #[cfg(spi_dma)]
            dma: None,
            #[cfg(spi_async_pio)]
            pio: None,
            sck_af,
            miso_af,
            word_bits: 8,
            kernel_clock,
            config,
            frequency_bounds: prescaler.frequency_bounds,
            _mode: PhantomData,
        })
    }
}

impl<M: PeriMode> Spi<'_, M, Master> {
    /// Nominal configured SCK in whole hertz, rounded down if fractional.
    /// This is not a measured rate; see [`Self::get_current_frequency_bounds`].
    pub fn get_current_frequency(&self) -> Hertz {
        self.frequency_bounds.nominal()
    }

    /// Qualified actual SCK envelope, propagated through the selected divider.
    ///
    /// The whole-hertz endpoints round outward. The guarantee requires the
    /// factory trim and temperature/supply conditions documented by RCC; it
    /// does not certify board signal integrity or the slave's timing.
    pub fn get_current_frequency_bounds(&self) -> ClockBounds {
        self.frequency_bounds
    }

    /// Current requested configuration, including the requested frequency ceiling.
    pub fn get_current_config(&self) -> Config {
        self.config
    }

    /// Reconfigure an idle bus. Invalid settings leave the previous setup intact.
    ///
    /// Changing CPOL can change the physical SCK level. Keep all external chip
    /// selects inactive while changing configuration.
    pub fn set_config(&mut self, config: &Config) -> Result<(), ConfigError> {
        #[cfg(spi_async_pio)]
        if self.pio.is_some() {
            // Configuration cannot consume a hardware error through recovery.
            // Keep it sticky for a transfer/flush and leave configuration intact.
            if !matches!(self.poll_pio_settle(None, &mut false), Poll::Ready(Ok(()))) {
                return Err(ConfigError::Busy);
            }
        }
        #[cfg(spi_dma)]
        self.check_dma().map_err(|error| match error {
            Error::DmaBusy => ConfigError::Busy,
            _ => ConfigError::DmaQuarantined,
        })?;
        let prescaler = self
            .peripheral
            .info
            .validate_config(self.kernel_clock, config)?;
        #[cfg(spi_async_pio)]
        let status = if self.pio.is_some() {
            // Recheck after validation too: a newly observed error must remain
            // available to flush, not be erased by configure's disabled clear.
            self.pio_observe(None, PioWait::Idle)
                .map_err(|_| ConfigError::Busy)?
        } else {
            self.peripheral.info.regs.isr().read()
        };
        #[cfg(not(spi_async_pio))]
        let status = self.peripheral.info.regs.isr().read();
        if status.busy() || !status.txe() {
            return Err(ConfigError::Busy);
        }
        self.peripheral
            .info
            .configure(config, prescaler, self.word_bits);
        // Reapply only the owned pin modes; the AF number is preserved by
        // deriving it from the original generated route at construction.
        self.set_input_pulls(config);
        self.config = *config;
        self.frequency_bounds = prescaler.frequency_bounds;
        Ok(())
    }

    fn set_input_pulls(&mut self, config: &Config) {
        self.sck
            .as_mut()
            .unwrap()
            .set_as_af(self.sck_af, true, config.sck_pull());
        self.miso
            .as_mut()
            .unwrap()
            .set_as_af(self.miso_af, false, config.input_pull);
    }

    /// Write all words, receiving and discarding one frame per transmitted word.
    /// Returns only when TXE is set and BUSY is clear.
    pub fn blocking_write<W: Word>(&mut self, words: &[W]) -> Result<(), Error> {
        self.pio_transfer(&mut [], words)
    }

    /// Read words while transmitting zero-valued words to generate SCK.
    pub fn blocking_read<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.pio_transfer(words, &[])
    }

    /// Simultaneously transmit and receive `max(read.len(), write.len())` frames.
    /// A shorter write buffer is padded with zeroes. Extra received frames are
    /// discarded, including during write-only operations.
    pub fn blocking_transfer<W: Word>(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        self.pio_transfer(read, write)
    }

    /// Simultaneously transmit and replace each word with its received word.
    pub fn blocking_transfer_in_place<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.pio_transfer_in_place(words)
    }

    /// Wait for the transmit buffer and shift register to become empty.
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        #[cfg(spi_async_pio)]
        self.check_pio()?;
        #[cfg(spi_dma)]
        self.check_dma()?;
        self.wait_idle()
    }
}

impl<M: PeriMode, CM: CommunicationMode> Drop for Spi<'_, M, CM> {
    fn drop(&mut self) {
        // No ISR may read this bank after its clock is gated. PIO owns no DMA
        // memory lease; unfinished wire work may be truncated by this teardown.
        #[cfg(spi_async_pio)]
        self.peripheral
            .info
            .pio_state()
            .deactivate(self.peripheral.info.regs);
        #[cfg(spi_dma)]
        if let Some(dma) = self.dma.as_mut() {
            let settled = matches!(
                dma.poll_complete(self.peripheral.info.regs, None),
                Poll::Ready(Ok(()))
            );
            dma.deactivate(self.peripheral.info.regs);
            if !settled {
                // Neither request-disable nor EN=0 proves a pending DMA access
                // drained. Retain both channels/storage, SPI clock and AF pins.
                core::mem::forget(self.dma.take());
                core::mem::forget(self.sck.take());
                core::mem::forget(self._mosi.take());
                core::mem::forget(self.miso.take());
                return;
            }
        }
        self.peripheral.info.enable(false);
        self.peripheral.info.regs.ier().write(|v| {
            v.set_txe(false);
            v.set_rxne(false);
            v.set_ssf(false);
            v.set_ssr(false);
            v.set_ud(false);
            v.set_ov(false);
            v.set_sserr(false);
            v.set_modf(false);
        });
        critical_section::with(|cs| self.peripheral.info.rcc.disable_with_cs(cs))
            .expect("SPI clock gate did not disable");
    }
}

impl Info {
    fn validate_config(
        &self,
        clock: ClockBounds,
        config: &Config,
    ) -> Result<Prescaler, ConfigError> {
        #[cfg(spi_cw32l012_v1)]
        if config.input_pull == Pull::Down {
            return Err(ConfigError::UnsupportedInputPull);
        }
        if cfg!(any(spi_cw32l010_v1, spi_cw32l012_v1)) && config.sample_delay != SampleDelay::None {
            return Err(ConfigError::UnsupportedSampleDelay);
        }
        self.select_prescaler(clock, config.frequency)
    }
    fn select_prescaler(
        &self,
        clock: ClockBounds,
        requested: Hertz,
    ) -> Result<Prescaler, ConfigError> {
        if clock.nominal().0 == 0 {
            return Err(ConfigError::ClockNotInitialized);
        }
        if requested.0 == 0 {
            return Err(ConfigError::FrequencyZero);
        }
        let minimum_divisor = u64::from(self.minimum_divisor);
        if requested.0 > self.maximum_frequency
            || u64::from(requested.0) * minimum_divisor > u64::from(clock.nominal().0)
        {
            return Err(ConfigError::FrequencyTooHigh);
        }
        for bits in 0..=Prescaler::MAX_BITS {
            let divisor = Prescaler::divisor(bits);
            if divisor < u32::from(self.minimum_divisor) {
                continue;
            }
            // Keep the exact source numerator and cumulative divider. Rounded
            // nominal Hertz is unsuitable for a maximum actual SCK constraint.
            let frequency_bounds = clock.divided_by(divisor);
            if !frequency_bounds.maximum_exceeds(requested.0)
                && !frequency_bounds.maximum_exceeds(self.maximum_frequency)
            {
                return Ok(Prescaler {
                    bits,
                    frequency_bounds,
                });
            }
        }
        Err(ConfigError::FrequencyTooLow)
    }

    fn configure(&self, config: &Config, prescaler: Prescaler, word_bits: u8) {
        let regs = self.regs;
        let width = word_bits - 1;
        self.enable(false);
        regs.ier().write(|v| {
            v.set_txe(false);
            v.set_rxne(false);
            v.set_ssf(false);
            v.set_ssr(false);
            v.set_ud(false);
            v.set_ov(false);
            v.set_sserr(false);
            v.set_modf(false);
        });
        regs.cr2().write(|v| {
            #[cfg(not(any(spi_cw32l010_v1, spi_cw32l012_v1)))]
            v.set_hdoe(false);
            #[cfg(any(spi_cw32l010_v1, spi_cw32l012_v1))]
            v.set_en(false);
            #[cfg(spi_cw32l010_v1)]
            {
                v.set_adcrx(false);
                v.set_adctx(false);
            }
            #[cfg(spi_cw32l012_v1)]
            {
                v.set_dmarx(false);
                v.set_dmatx(false);
            }
        });
        // Low-power IP moves HDOE to CR3 and EN to CR2. Clear CR3 deliberately;
        // CR2=0 also disables DMA/ADC triggers where implemented.
        #[cfg(any(spi_cw32l010_v1, spi_cw32l012_v1))]
        regs.cr3().write(|v| v.set_hdoe(false));
        // SSM=1 makes the unused hardware CS an output controlled by SSI. No
        // physical CS pin is configured here; an external GPIO owns chip select.
        regs.ssi().write(|v| v.set_ssi(true));
        regs.cr1().write(|v| {
            v.set_mstr(true);
            v.set_ssm(true);
            v.set_cpol(config.mode.polarity == Polarity::IdleHigh);
            v.set_cpha(config.mode.phase == Phase::CaptureOnSecondTransition);
            v.set_lsbf(config.bit_order == BitOrder::LsbFirst);
            #[cfg(not(any(spi_cw32l010_v1, spi_cw32l012_v1)))]
            v.set_smp(config.sample_delay == SampleDelay::HalfPeriod);
            // The new low-power SMP bit has different semantics: leave it zero.
            v.set_br(prescaler.bits);
            v.set_width(width);
        });
        // ICR is R1W0, including FLUSH. Reset every owned flag and the
        // transmit/shift buffer only while the controller is disabled.
        self.clear();
        self.enable(true);
    }

    pub(super) fn enable(&self, enable: bool) {
        #[cfg(not(any(spi_cw32l010_v1, spi_cw32l012_v1)))]
        self.regs.cr1().modify(|v| v.set_en(enable));
        #[cfg(any(spi_cw32l010_v1, spi_cw32l012_v1))]
        self.regs.cr2().modify(|v| v.set_en(enable));
    }
    pub(super) fn clear(&self) {
        // This deliberately clears every SPI flag and flushes both transmit
        // buffers during disabled initialization/recovery. Begin with the
        // generated R1W0 no-op, then issue each command explicitly; reserved
        // bits retain their documented zero value. Never read-modify-write ICR.
        let mut command = pac::spi::regs::Icr::write_noop();
        command.set_flush(false);
        command.set_rxne(false);
        command.set_ssf(false);
        command.set_ssr(false);
        command.set_ud(false);
        command.set_ov(false);
        command.set_sserr(false);
        command.set_modf(false);
        self.regs.icr().write_value(command);
    }
}

impl<M: PeriMode> Spi<'_, M, Master> {
    fn status(&mut self) -> Result<pac::spi::regs::Isr, Error> {
        let status = self.peripheral.info.regs.isr().read();
        let error = decode_error(status);
        if let Some(error) = error {
            self.peripheral.info.enable(false);
            self.peripheral.info.clear();
            self.peripheral.info.enable(true);
            return Err(error);
        }
        Ok(status)
    }

    fn wait_idle(&mut self) -> Result<(), Error> {
        self.wait_idle_with(&mut || Ok(()))
    }

    fn wait_idle_with(&mut self, retry: &mut impl FnMut() -> Result<(), Error>) -> Result<(), Error> {
        loop {
            let status = self.status()?;
            if status.txe() && !status.busy() {
                return Ok(());
            }
            retry()?;
            core::hint::spin_loop();
        }
    }

    pub(super) fn prepare<W: Word>(&mut self) -> Result<(), Error> {
        self.prepare_with::<W>(&mut || Ok(()))
    }

    fn prepare_with<W: Word>(&mut self, retry: &mut impl FnMut() -> Result<(), Error>) -> Result<(), Error> {
        self.wait_idle_with(retry)?;
        if self.word_bits != W::BITS {
            self.peripheral.info.enable(false);
            self.peripheral
                .info
                .regs
                .cr1()
                .modify(|v| v.set_width(W::BITS - 1));
            self.peripheral.info.enable(true);
            self.word_bits = W::BITS;
        }
        // There is exactly one receive buffer. Read only when it is nonempty;
        // do not clear all ICR flags and accidentally hide a pending error.
        if self.status()?.rxne() {
            let _ = self.peripheral.info.regs.dr().read().dr();
        }
        Ok(())
    }

    fn exchange<W: Word>(&mut self, word: W, retry: &mut impl FnMut() -> Result<(), Error>) -> Result<W, Error> {
        while !self.status()?.txe() {
            retry()?;
            core::hint::spin_loop();
        }
        self.peripheral
            .info
            .regs
            .dr()
            .write(|v| v.set_dr(word.into_u16()));
        while !self.status()?.rxne() {
            retry()?;
            core::hint::spin_loop();
        }
        Ok(W::from_u16(self.peripheral.info.regs.dr().read().dr()))
    }

    fn pio_transfer<W: Word>(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        #[cfg(spi_async_pio)]
        self.check_pio()?;
        #[cfg(spi_dma)]
        self.check_dma()?;
        let count = read.len().max(write.len());
        if count == 0 {
            return Ok(());
        }
        self.prepare::<W>()?;
        for index in 0..count {
            let received = self.exchange(write.get(index).copied().unwrap_or_default(), &mut || Ok(()))?;
            if let Some(word) = read.get_mut(index) {
                *word = received;
            }
        }
        self.wait_idle()
    }

    fn pio_transfer_in_place<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        #[cfg(spi_async_pio)]
        self.check_pio()?;
        #[cfg(spi_dma)]
        self.check_dma()?;
        self.transfer_in_place_with(words, &mut || Ok(()))
    }

    fn transfer_in_place_with<W: Word>(&mut self, words: &mut [W], retry: &mut impl FnMut() -> Result<(), Error>) -> Result<(), Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.prepare_with::<W>(retry)?;
        for word in words {
            *word = self.exchange(*word, retry)?;
        }
        self.wait_idle_with(retry)
    }
}

pub(super) fn decode_error(status: pac::spi::regs::Isr) -> Option<Error> {
    if status.modf() {
        Some(Error::ModeFault)
    } else if status.ov() {
        Some(Error::Overrun)
    } else if status.sserr() {
        Some(Error::ChipSelectFault)
    } else if status.ud() {
        Some(Error::Underrun)
    } else {
        None
    }
}
