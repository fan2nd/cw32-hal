use super::*;

impl<'d> UartRx<'d, Blocking> {
    /// Create a blocking receive-only UART with active-low hardware RTS.
    pub fn new_blocking_with_rts<T: Instance, RX: RxPin<T>, RTS: RtsPin<T>>(
        _peri: Peri<'d, T>,
        rx: Peri<'d, RX>,
        rts: Peri<'d, RTS>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let rx = Self::new_inner::<T, RX>(
            rx,
            Some((rts.into(), <RTS as sealed::Pin<T, sealed::Rts>>::AF)),
            config,
        )?;
        Ok(rx)
    }

    /// Create a blocking receive-only UART.
    pub fn new_blocking<T: Instance, RX: RxPin<T>>(
        _peri: Peri<'d, T>,
        rx: Peri<'d, RX>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::new_inner::<T, RX>(rx, None, config)
    }
}
impl<'d> UartRx<'d, Async> {
    /// Create an async receive-only UART with active-low hardware RTS.
    pub fn new_with_rts<T: Instance, RX: RxPin<T>, RTS: RtsPin<T>>(
        _peri: Peri<'d, T>,
        rx: Peri<'d, RX>,
        rts: Peri<'d, RTS>,
        _irq: impl Binding<T::Interrupt, T::InterruptHandler>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let rx = Self::new_inner::<T, RX>(
            rx,
            Some((rts.into(), <RTS as sealed::Pin<T, sealed::Rts>>::AF)),
            config,
        )?;
        unsafe { T::Interrupt::enable() };
        Ok(rx)
    }

    /// Create an interrupt-driven receive-only UART.
    pub fn new<T: Instance, RX: RxPin<T>>(
        _peri: Peri<'d, T>,
        rx: Peri<'d, RX>,
        _irq: impl Binding<T::Interrupt, T::InterruptHandler>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let rx = Self::new_inner::<T, RX>(rx, None, config)?;
        unsafe { T::Interrupt::enable() };
        Ok(rx)
    }
    /// Receive exactly the requested bytes, using private staging with DMA.
    /// Cancellation keeps completed caller bytes and the current DMA chunk;
    /// a later read reaps it and preserves any clean surplus for another read.
    /// An empty inherent read may await an old chunk without consuming it.
    /// FE/PE returns promptly but recovery may wait forever for that chunk.
    /// Finite chunks and rearm/copy gaps do not provide lossless reception.
    /// On L010/L011/L012, even an empty supported byte read masks an abandoned RX/idle wait's
    /// interrupt enables; it does not clear receive flags or consume data.
    pub async fn read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        self.data_bits.check_byte()?;
        #[cfg(uart_dma_rx)]
        if let Some(dma) = self.dma.as_mut() {
            return dma.read(self.info.regs, self.state, buffer).await;
        }
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        retire_receive_wait(self.info.regs);
        for byte in buffer {
            loop {
                Wait::new(self.info.regs, &self.state.rx_waker, WaitEvent::Receive).await;
                if let Some(received) = receive(self.info.regs)? {
                    *byte = received;
                    break;
                }
            }
        }
        Ok(())
    }

    /// Receive a finite prefix until the buffer fills or native RXIDLE is observed.
    /// Nine-bit owners reject this byte API before any action, including empty calls.
    ///
    /// Available on L010/L011/L012. On an eight-bit owner, an empty slice returns `Ok(0)` on its first
    /// poll without MMIO, waker registration, interrupt changes or stale-idle
    /// acknowledgment. Constructing or dropping an unpolled future has no effect.
    /// An empty call does not retire an abandoned earlier wait.
    ///
    /// A nonempty call starts one idle-observation window on its first poll:
    /// it masks old RX wait interrupts and clears only the then-stale RXIDLE
    /// indication, preserving unread RC/RDR and line errors. It does not flush
    /// input or recover an earlier packet boundary. Each successfully consumed
    /// byte increments the returned count; bytes beyond that prefix are untouched.
    /// A new idle indication can return zero before any byte is collected. This
    /// is an inherent API, not the embedded-io short-read contract.
    ///
    /// If RC and RXIDLE are observed together, the observed byte is consumed
    /// before completing on idle, unless an observed error takes precedence.
    /// Interrupt/executor delay makes the byte's position relative to an earlier
    /// idle boundary best-effort: these flags provide no packet timestamp/order.
    /// Errors take priority in the order ORE, PE, FE, NE. A byte sampled with an
    /// error is consumed/acknowledged but not appended to the successful prefix,
    /// matching exact reads. Already written bytes remain on error/cancellation;
    /// an error carries no partial count.
    ///
    /// Hardware begins idle detection only after a completed received character.
    /// There is no software timer, and this call can wait indefinitely without
    /// the necessary receive/idle event. A preserved pre-start RC byte can be
    /// collected after its old idle was discarded; clearing that flag does not
    /// guarantee another idle event for the same character. More traffic, a later
    /// idle indication, buffer completion or external cancellation may be needed.
    /// There is no continuous reception/storage between calls. Service delays,
    /// caller gaps and coalesced hardware flags can lose data or boundary history.
    ///
    /// Cancelling a pending wait masks RX/error/idle enables while preserving
    /// unread data and flags, TX and unrelated interrupt enables. Forgetting the
    /// future skips this cleanup; a later enabled RX event, ordinary read,
    /// nonempty idle read or RX-owner drop retires the old wait. The ISR owns no
    /// caller-buffer pointer and cannot write caller memory after cancellation
    /// or forget. Neither action guarantees data or boundary recovery.
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    pub async fn read_until_idle(&mut self, buffer: &mut [u8]) -> Result<usize, Error> {
        self.data_bits.check_byte()?;
        if buffer.is_empty() {
            return Ok(0);
        }
        #[cfg(uart_dma_rx)]
        if let Some(dma) = self.dma.as_mut() {
            dma.check_complete(self.info.regs, self.state)?;
            let copied = dma.copy_ready(buffer);
            if copied != 0 { return Ok(copied); }
        }
        let regs = self.info.regs;
        critical_section::with(|_| {
            regs.ier()
                .modify(|w| WaitEvent::Receive.set_enabled(w, false));
            // This clear completes the new observation-window boundary. Keep
            // unread RC/RDR and all line errors for the consumption snapshot.
            let mut command = regs::Icr::write_noop();
            command.set_rxidle(false);
            regs.icr().write_value(command);
        });
        let mut received = 0;
        loop {
            Wait::new(regs, &self.state.rx_waker, WaitEvent::ReceiveUntilIdle).await;
            // The wait only established readiness. Use one fresh snapshot for
            // data eligibility, acknowledgment, error priority and completion.
            let status = regs.isr().read();
            if let Some(byte) = receive_observed(regs, status, true)? {
                buffer[received] = byte;
                received += 1;
            }
            if received == buffer.len() || status.rxidle() {
                return Ok(received);
            }
        }
    }
}
impl<'d, M: Mode> UartRx<'d, M> {
    pub(super) fn from_pin<T: Instance, RX: RxPin<T>>(
        pin: Peri<'d, RX>,
        info: Info,
        actual: u32,
        pull: Pull,
        data_bits: DataBits,
    ) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_af(<RX as sealed::Pin<T, sealed::Rx>>::AF, false, pull);
        T::state().activate(Direction::Rx);
        Self {
            info,
            state: T::state(),
            _pin: Some(pin),
            _rts: None,
            #[cfg(uart_dma_rx)]
            dma: None,
            actual_baudrate: actual,
            data_bits,
            _mode: PhantomData,
        }
    }

    fn new_inner<T: Instance, RX: RxPin<T>>(
        rx: Peri<'d, RX>,
        rts: Option<(Peri<'d, AnyPin>, u8)>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let baud = configured_baud::<T>(config)?;
        let info = T::info();
        info.prepare::<T>(config, baud);
        let mut rx =
            Self::from_pin::<T, RX>(rx, info, baud.actual, config.rx_pull, config.data_bits);
        rx._rts = rts.map(|(pin, af)| {
            let mut pin = Flex::new(pin);
            pin.set_as_af(af, true, Pull::None);
            pin
        });
        critical_section::with(|_| info.regs.cr2().modify(|w| w.set_rtsen(rx._rts.is_some())));
        critical_section::with(|_| info.regs.cr1().modify(|v| v.set_rxen(true)));
        Ok(rx)
    }
    /// Baud rate after divider rounding.
    pub fn actual_baudrate(&self) -> u32 {
        self.actual_baudrate
    }
    // Trait reads must not await an abandoned multi-byte DMA chunk just to
    // return their first byte. Empty trait reads use this same observation.
    // Native ordinary reads also retire old RX/idle enables without clearing
    // receive flags, including blocking and empty trait entry points.
    pub(super) fn check_read(&mut self) -> Result<(), Error> {
        self.data_bits.check_byte()?;
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        retire_receive_wait(self.info.regs);
        #[cfg(uart_dma_rx)]
        if let Some(dma) = self.dma.as_mut() {
            dma.check_complete(self.info.regs, self.state)?;
        }
        Ok(())
    }
    /// Read exactly the requested bytes. First observe any DMA chunk once:
    /// pending returns DmaBusy, clean staged bytes are copied before CPU reads.
    pub fn blocking_read(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        self.check_read()?;
        #[cfg(uart_dma_rx)]
        let buffer = if let Some(dma) = self.dma.as_mut() {
            let copied = dma.copy_ready(buffer);
            &mut buffer[copied..]
        } else {
            buffer
        };
        for byte in buffer {
            loop {
                if let Some(received) = receive(self.info.regs)? {
                    *byte = received;
                    break;
                }
                core::hint::spin_loop();
            }
        }
        Ok(())
    }
}

impl<M: Mode> Drop for UartRx<'_, M> {
    fn drop(&mut self) {
        #[cfg(uart_dma_rx)]
        if let Some(dma) = self.dma.as_mut() {
            // An error return alone does not decide lifetime: a line-error
            // chunk may already have retired cleanly, whereas an unresolved
            // line error retains exactly the same DMA memory lease.
            let _ = dma.check_complete(self.info.regs, self.state);
            if matches!(
                dma.phase,
                RxDmaPhase::InFlight { .. } | RxDmaPhase::Quarantined
            ) {
                self.info.shutdown(self.state, Direction::Rx, true);
                core::mem::forget(self.dma.take());
                core::mem::forget(self._pin.take());
                core::mem::forget(self._rts.take());
                return;
            }
        }
        self.info.shutdown(self.state, Direction::Rx, false);
    }
}
