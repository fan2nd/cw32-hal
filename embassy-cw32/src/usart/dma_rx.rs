use super::*;

// RX staging is never reborrowed while DMA may write it. The exclusive static
// reference is consumed once; only a positively retired Ready chunk is copied.
#[cfg(uart_dma_rx)]
#[derive(Clone, Copy)]
pub(super) enum RxDmaPhase {
    Idle,
    InFlight {
        len: usize,
        line_error: Option<Error>,
        error_reported: bool,
    },
    Ready {
        len: usize,
        offset: usize,
    },
    Quarantined,
}

#[cfg(uart_dma_rx)]
pub(super) struct RxDma {
    channel: crate::dma::Channel<'static>,
    staging: *mut u8,
    capacity: usize,
    _staging: PhantomData<&'static mut [u8]>,
    request: crate::dma::Request,
    #[cfg(uart_cw32l012_v1)]
    checked_frames: bool,
    pub(super) phase: RxDmaPhase,
}

#[cfg(uart_dma_rx)]
impl RxDma {
    fn new(
        channel: crate::dma::Channel<'static>,
        staging: &'static mut [u8],
        request: crate::dma::Request,
        #[cfg(uart_cw32l012_v1)] checked_frames: bool,
    ) -> Self {
        let capacity = staging.len();
        let staging = staging.as_mut_ptr();
        Self {
            channel,
            staging,
            capacity,
            _staging: PhantomData,
            request,
            #[cfg(uart_cw32l012_v1)]
            checked_frames,
            phase: RxDmaPhase::Idle,
        }
    }

    pub(super) fn line_error(status: regs::Isr) -> Option<Error> {
        #[cfg(uart_cw32l012_v1)]
        if status.ore() { return Some(Error::Overrun); }
        if status.pe() {
            Some(Error::Parity)
        } else if status.fe() {
            Some(Error::Framing)
        } else {
            #[cfg(uart_cw32l012_v1)]
            if status.ne() { return Some(Error::Noise); }
            None
        }
    }

    fn in_flight_error(&self, status: regs::Isr) -> Option<Error> {
        #[cfg(uart_cw32l012_v1)]
        let status = {
            let mut observed = status;
            if self.checked_frames { observed.set_ore(false); }
            observed
        };
        Self::line_error(status)
    }

    fn merge_error(previous: Option<Error>, observed: Option<Error>) -> Option<Error> {
        match (previous, observed) {
            #[cfg(uart_cw32l012_v1)]
            (Some(Error::Overrun), _) | (_, Some(Error::Overrun)) => Some(Error::Overrun),
            (Some(Error::Parity), _) | (_, Some(Error::Parity)) => Some(Error::Parity),
            (Some(error), _) | (_, Some(error)) => Some(error),
            _ => None,
        }
    }

    // RC is software-cleared, even when DMA has already read RDR. Never treat
    // stale RC as an extra byte or read RDR on a DMA completion/error path.
    fn acknowledge(regs: pac::uart::Uart, status: regs::Isr) {
        let mut command = regs::Icr::write_noop();
        #[cfg(uart_cw32l012_v1)]
        { command.set_ne(!status.ne()); command.set_ore(!status.ore()); }
        command.set_rc(!status.rc());
        command.set_pe(!status.pe());
        command.set_fe(!status.fe());
        regs.icr().write_value(command);
    }

    fn poll_complete(
        &mut self,
        regs: pac::uart::Uart,
        state: &State,
        mut cx: Option<&mut Context<'_>>,
    ) -> Poll<Result<(), Error>> {
        let (len, mut line_error, mut error_reported) = match self.phase {
            RxDmaPhase::Idle | RxDmaPhase::Ready { .. } => return Poll::Ready(Ok(())),
            RxDmaPhase::Quarantined => return Poll::Ready(Err(Error::DmaQuarantined)),
            RxDmaPhase::InFlight {
                len,
                line_error,
                error_reported,
            } => (len, line_error, error_reported),
        };
        if let Some(cx) = cx.as_mut() {
            state.rx_waker.register(cx.waker());
        }
        critical_section::with(|_| {
            // Register both wakers before final observations. The nonblocking
            // second observation services this channel's flags even if the
            // caller has masked CPU interrupts, and makes observed TE win.
            if let Some(cx) = cx.as_deref_mut() {
                let _ = self.channel.poll_hardware_complete(Some(cx));
            }
            if cx.is_some() && !error_reported {
                regs.ier()
                    .modify(|w| {
                        WaitEvent::ReceiveErrors.set_enabled(w, true);
                        #[cfg(uart_cw32l012_v1)]
                        if self.checked_frames { w.set_ore(false); }
                    });
            }
            line_error = Self::merge_error(line_error, self.in_flight_error(regs.isr().read()));
            match self.channel.poll_hardware_complete(None) {
                Poll::Ready(Err(error)) => {
                    regs.ier()
                        .modify(|w| WaitEvent::Receive.set_enabled(w, false));
                    self.phase = RxDmaPhase::Quarantined;
                    Poll::Ready(Err(Error::Dma(error)))
                }
                Poll::Ready(Ok(())) => {
                    // x030 p350 and each qualified own RX example: EN before DMARX; close only
                    // after clean DMA TC. A UART error alone is never a drain.
                    regs.cr2().modify(|w| w.set_dmarx(false));
                    if regs.cr2().read().dmarx() {
                        regs.ier()
                            .modify(|w| WaitEvent::Receive.set_enabled(w, false));
                        self.phase = RxDmaPhase::Quarantined;
                        return Poll::Ready(Err(Error::DmaQuarantined));
                    }
                    let status = regs.isr().read();
                    line_error = Self::merge_error(line_error, self.in_flight_error(status));
                    regs.ier()
                        .modify(|w| WaitEvent::Receive.set_enabled(w, false));
                    Self::acknowledge(regs, status);
                    // SAFETY: this persistent lease retained the destination,
                    // endpoint and clocks; clean completion and request closure
                    // now precede the DMA fence and retirement.
                    unsafe { self.channel.retire_hardware() };
                    self.phase = if line_error.is_some() {
                        RxDmaPhase::Idle
                    } else {
                        RxDmaPhase::Ready { len, offset: 0 }
                    };
                    if !error_reported {
                        if let Some(error) = line_error {
                            return Poll::Ready(Err(error));
                        }
                    }
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => {
                    let report = if !error_reported { line_error } else { None };
                    if report.is_some() {
                        error_reported = true;
                        regs.ier()
                            .modify(|w| WaitEvent::Receive.set_enabled(w, false));
                    }
                    self.phase = RxDmaPhase::InFlight {
                        len,
                        line_error,
                        error_reported,
                    };
                    match report {
                        Some(error) => Poll::Ready(Err(error)),
                        None => Poll::Pending,
                    }
                }
            }
        })
    }

    pub(super) fn check_complete(
        &mut self,
        regs: pac::uart::Uart,
        state: &State,
    ) -> Result<(), Error> {
        match self.poll_complete(regs, state, None) {
            Poll::Ready(result) => result,
            Poll::Pending => Err(Error::DmaBusy),
        }
    }

    async fn complete(&mut self, regs: pac::uart::Uart, state: &State) -> Result<(), Error> {
        // Only waiter masking uses Drop. The DMA lease and all error state
        // remain valid even if this guard and the whole future are forgotten.
        let _waiter = Wait::new(regs, &state.rx_waker, WaitEvent::ReceiveErrors);
        core::future::poll_fn(|cx| self.poll_complete(regs, state, Some(cx))).await
    }

    pub(super) fn copy_ready(&mut self, buffer: &mut [u8]) -> usize {
        let RxDmaPhase::Ready { len, offset } = self.phase else {
            return 0;
        };
        let count = buffer.len().min(len - offset);
        if count != 0 {
            // SAFETY: Ready requires clean retirement. The static destination
            // is exclusively owned and disjoint from this new caller borrow.
            unsafe {
                core::ptr::copy_nonoverlapping(self.staging.add(offset), buffer.as_mut_ptr(), count)
            };
        }
        self.phase = if offset + count == len {
            RxDmaPhase::Idle
        } else {
            RxDmaPhase::Ready {
                len,
                offset: offset + count,
            }
        };
        count
    }

    fn start(&mut self, regs: pac::uart::Uart, len: usize) -> Result<(), Error> {
        // Called only from Idle. Initialize only RX flags. Boundary frames may
        // be lost: RC neither counts bytes nor proves an unread DMA-era frame.
        let error = critical_section::with(|_| {
            regs.cr2().modify(|w| w.set_dmarx(false));
            regs.ier()
                .modify(|w| WaitEvent::Receive.set_enabled(w, false));
            let status = regs.isr().read();
            Self::acknowledge(regs, status);
            Self::line_error(status)
        });
        if let Some(error) = error {
            return Err(error);
        }
        // Publish before EN, including any panic/configuration failure after it.
        self.phase = RxDmaPhase::InFlight {
            len,
            line_error: None,
            error_reported: false,
        };
        // SAFETY: static private staging, typed RDR/request and this owner's
        // channel, pins and clocks are retained through every unconfirmed path.
        let result = unsafe {
            self.channel.start_read(
                self.request,
                regs.rdr().as_ptr().cast::<u8>(),
                self.staging,
                len,
            )
        };
        if result.is_err() {
            self.phase = RxDmaPhase::Quarantined;
            return Err(Error::DmaQuarantined);
        }
        critical_section::with(|_| regs.cr2().modify(|w| w.set_dmarx(true)));
        Ok(())
    }

    pub(super) async fn read(
        &mut self,
        regs: pac::uart::Uart,
        state: &State,
        mut buffer: &mut [u8],
    ) -> Result<(), Error> {
        // Empty inherent reads can settle an old lease but never consume Ready.
        self.complete(regs, state).await?;
        while !buffer.is_empty() {
            let copied = self.copy_ready(buffer);
            buffer = &mut buffer[copied..];
            if buffer.is_empty() {
                break;
            }
            self.start(regs, self.capacity.min(buffer.len()))?;
            self.complete(regs, state).await?;
        }
        Ok(())
    }
}

#[cfg(uart_dma_rx)]
impl UartRx<'static, Async> {
    /// Create a safe finite-chunk DMA receiver without RTS.
    ///
    /// Qualified families are F030/A030/L083 and F020/L031/L052/R031;
    /// each selected package still requires safe RX pins and SRAM metadata.
    ///
    /// Consumes static UART, RX pin, an admitted compatible channel and private
    /// SRAM staging of 1..=65535 bytes. Rejected inputs are consumed. Ordinary
    /// read buffers are CPU-copy destinations and are never passed to DMA.
    /// Cancelling/forgetting a read preserves its current chunk and clean tails.
    /// FE/PE returns promptly, but recovery must await clean DMA completion;
    /// errors or ambiguous DMA completion permanently quarantine resources.
    /// Dropping an unresolved owner retains its pins, channel, staging and clocks.
    /// This is not continuous reception: gaps can lose frames, with no overrun
    /// indication, lossless guarantee, early abort or bounded completion time.
    pub fn new_with_dma<T, RX, D>(
        _peri: Peri<'static, T>,
        rx: Peri<'static, RX>,
        channel: Peri<'static, D>,
        irq: impl Binding<T::Interrupt, T::InterruptHandler>
        + Binding<D::Interrupt, crate::dma::InterruptHandler<D>>
        + 'static,
        staging: &'static mut [u8],
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        T: Instance,
        RX: RxPin<T>,
        D: crate::dma::RequestRoute<T, crate::dma::signal::RX>,
    {
        let baud = configured_baud::<T>(config)?;
        require_byte_config(config)?;
        TxDma::validate_staging(staging).map_err(ConfigError::DmaBuffer)?;
        let channel = crate::dma::Channel::new_admitted(channel, irq)
            .map_err(|error| ConfigError::DmaChannel(error.error))?;
        let info = T::info();
        info.prepare::<T>(config, baud);
        let mut rx =
            Self::from_pin::<T, RX>(rx, info, baud.actual, config.rx_pull, config.data_bits);
        rx.dma = Some(RxDma::new(channel, staging, D::REQUEST, #[cfg(uart_cw32l012_v1)] config.dma_rx_checked_frames));
        critical_section::with(|_| info.regs.cr1().modify(|w| w.set_rxen(true)));
        unsafe { T::Interrupt::enable() };
        Ok(rx)
    }
}

#[cfg(all(uart_dma_tx, uart_dma_rx))]
impl Uart<'static, Async> {
    /// Create both DMA directions with one UART configuration, then use split
    /// or split_ref for concurrent writes and finite-chunk reads.
    ///
    /// Both channel capabilities and private static staging allocations are
    /// consumed. Shared DMA vectors must bind both channel handlers. The UART
    /// is prepared once after all validation and atomic pair admission. Each
    /// half retains only its own unresolved lease on Drop, preserving its peer.
    /// RX has the loss/cancellation limits of [`UartRx::new_with_dma`]; TX has
    /// the retained-resource limits of [`UartTx::new_with_dma`]. No RTS/CTS.
    pub fn new_with_dma<T, TX, RX, DT, DR>(
        _peri: Peri<'static, T>,
        tx: Peri<'static, TX>,
        rx: Peri<'static, RX>,
        tx_channel: Peri<'static, DT>,
        rx_channel: Peri<'static, DR>,
        irq: impl Binding<T::Interrupt, T::InterruptHandler>
        + Binding<DT::Interrupt, crate::dma::InterruptHandler<DT>>
        + Binding<DR::Interrupt, crate::dma::InterruptHandler<DR>>
        + 'static,
        tx_staging: &'static mut [u8],
        rx_staging: &'static mut [u8],
        config: Config,
    ) -> Result<Self, ConfigError>
    where
        T: Instance,
        TX: TxPin<T>,
        RX: RxPin<T>,
        DT: crate::dma::RequestRoute<T, crate::dma::signal::TX>,
        DR: crate::dma::RequestRoute<T, crate::dma::signal::RX>,
    {
        let baud = configured_baud::<T>(config)?;
        require_byte_config(config)?;
        TxDma::validate_staging(tx_staging).map_err(ConfigError::DmaBuffer)?;
        TxDma::validate_staging(rx_staging).map_err(ConfigError::DmaBuffer)?;
        let (tx_channel, rx_channel) =
            crate::dma::Channel::new_admitted_pair(tx_channel, rx_channel, irq)
                .map_err(ConfigError::DmaChannel)?;
        let info = T::info();
        info.prepare::<T>(config, baud);
        let mut tx = UartTx::from_pin::<T, TX>(tx, info, baud.actual, config.data_bits);
        let mut rx =
            UartRx::from_pin::<T, RX>(rx, info, baud.actual, config.rx_pull, config.data_bits);
        tx.dma = Some(TxDma {
            channel: tx_channel,
            staging: tx_staging,
            request: DT::REQUEST,
            phase: TxDmaPhase::Idle,
        });
        rx.dma = Some(RxDma::new(rx_channel, rx_staging, DR::REQUEST, #[cfg(uart_cw32l012_v1)] config.dma_rx_checked_frames));
        critical_section::with(|_| {
            info.regs.cr1().modify(|w| {
                w.set_txen(true);
                w.set_rxen(true);
            })
        });
        unsafe { T::Interrupt::enable() };
        Ok(Self { tx, rx })
    }
}
