use super::*;

/// Handler for a UART with its own vector. Bind it with `bind_interrupts!`.
/// L083 constructors instead require `SharedInterruptHandler`, which dispatches
/// both instances on the shared vector. A partial binding cannot construct one.
pub struct InterruptHandler<T: Instance>(PhantomData<T>);
#[cfg(not(uart_cw32l083_v1))]
impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        dispatch_interrupt(T::info().regs, T::state());
    }
}

/// L083 shared-vector handler. The type argument is the type-level vector:
/// `SharedInterruptHandler<interrupt::typelevel::UART1_UART4>` (or
/// `UART2_UART5`, `UART3_UART6`). Bind that handler once; it safely services both
/// UARTs and is accepted by either instance's async constructor.
///
/// An unconstructed or dropped partner is skipped before any clock-gated MMIO.
/// The NVIC line stays enabled after first use, including after the last drop;
/// each HAL-owned inactive UART's local interrupts are masked and its guard is
/// false. A bootloader/external owner must quiesce an unowned partner's source
/// (or provide its own compatible handling) before enabling this vector. This
/// driver never reads, resets or clears an inactive unowned UART to do that.
#[cfg(uart_cw32l083_v1)]
pub struct SharedInterruptHandler<I: Interrupt>(PhantomData<I>);

#[cfg(uart_cw32l083_v1)]
macro_rules! group {
    ($irq:ident, $first:ident, $second:ident) => {
        impl Handler<crate::interrupt::typelevel::$irq>
            for SharedInterruptHandler<crate::interrupt::typelevel::$irq>
        {
            unsafe fn on_interrupt() {
                dispatch_group(
                    <crate::peripherals::$first as sealed::Instance>::info().regs,
                    <crate::peripherals::$first as sealed::Instance>::state(),
                    <crate::peripherals::$second as sealed::Instance>::info().regs,
                    <crate::peripherals::$second as sealed::Instance>::state(),
                );
            }
        }
    };
}
#[cfg(uart_cw32l083_v1)]
group!(UART1_UART4, UART1, UART4);
#[cfg(uart_cw32l083_v1)]
group!(UART2_UART5, UART2, UART5);
#[cfg(uart_cw32l083_v1)]
group!(UART3_UART6, UART3, UART6);

#[cfg(uart_cw32l083_v1)]
fn dispatch_group(a: pac::uart::Uart, a_state: &State, b: pac::uart::Uart, b_state: &State) {
    dispatch_interrupt(a, a_state);
    dispatch_interrupt(b, b_state);
}

#[cfg(not(uart_cw32l083_v1))]
pub(crate) fn disable_interrupt<I: Interrupt>() {
    I::disable();
}

fn dispatch_interrupt(regs: pac::uart::Uart, state: &State) {
    let (wake_rx, wake_tx) = critical_section::with(|cs| {
        let owners = state.lifecycle.borrow(cs).get();
        if !owners.live() {
            return (false, false);
        }
        on_interrupt(regs, state, owners, cs)
    });
    // No waker runs under the lifecycle lock or an outstanding ring borrow.
    // A stale wake after intervening teardown grants no register/storage access.
    if wake_rx {
        state.rx_waker.wake();
    }
    if wake_tx {
        state.tx_waker.wake();
    }
}

// These are software wait conditions, not hardware bit positions. Status,
// enables and clear commands use their distinct generated PAC fieldsets.
#[derive(Clone, Copy)]
pub(super) enum WaitEvent {
    TxReady,
    TxComplete,
    Receive,
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    ReceiveUntilIdle,
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    ReceiveLin,
    #[cfg(uart_dma_rx)]
    ReceiveErrors,
}

impl WaitEvent {
    pub(super) fn set_enabled(self, ier: &mut regs::Ier, enabled: bool) {
        match self {
            Self::TxReady => ier.set_txe(enabled),
            Self::TxComplete => ier.set_tc(enabled),
            #[cfg(uart_dma_rx)]
            Self::ReceiveErrors => {
                #[cfg(uart_cw32l012_v1)]
                { ier.set_ne(enabled); ier.set_ore(enabled); }
                ier.set_rc(false);
                ier.set_fe(enabled);
                ier.set_pe(enabled);
            }
            Self::Receive => {
                ier.set_rc(enabled);
                ier.set_fe(enabled);
                ier.set_pe(enabled);
                #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
                {
                    ier.set_ne(enabled);
                    ier.set_ore(enabled);
                    ier.set_rxidle(false);
                    ier.set_rxbrk(false);
                }
            }
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::ReceiveUntilIdle => {
                Self::Receive.set_enabled(ier, enabled);
                ier.set_rxidle(enabled);
            }
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::ReceiveLin => {
                Self::Receive.set_enabled(ier, enabled);
                ier.set_rxbrk(enabled);
            }
        }
    }

    fn ready(self, status: regs::Isr) -> bool {
        match self {
            Self::TxReady => status.txe(),
            Self::TxComplete => !status.txbusy(),
            Self::Receive => receive_event(status),
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::ReceiveUntilIdle => receive_event(status) || status.rxidle(),
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::ReceiveLin => receive_event(status) || status.rxbrk(),
            #[cfg(uart_dma_rx)]
            Self::ReceiveErrors => RxDma::line_error(status).is_some(),
        }
    }
}

#[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
pub(super) fn retire_receive_wait(regs: pac::uart::Uart) {
    critical_section::with(|_| {
        regs.ier()
            .modify(|w| WaitEvent::Receive.set_enabled(w, false));
    });
}

fn receive_event(status: regs::Isr) -> bool {
    let pending = status.rc() || status.fe() || status.pe();
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    let pending = pending || status.ne() || status.ore();
    pending
}

pub(super) fn transmit(regs: pac::uart::Uart, byte: u8) {
    transmit_word(regs, u16::from(byte));
}

pub(super) fn transmit_word(regs: pac::uart::Uart, word: u16) {
    let mut command = regs::Icr::write_noop();
    command.set_tc(false);
    regs.icr().write_value(command);
    regs.tdr().write(|w| w.set_tdr(word));
}

pub(super) fn receive(regs: pac::uart::Uart) -> Result<Option<u8>, Error> {
    let status = regs.isr().read();
    receive_observed(regs, status, false)
}

pub(super) fn receive_observed(
    regs: pac::uart::Uart,
    status: regs::Isr,
    acknowledge_idle: bool,
) -> Result<Option<u8>, Error> {
    receive_observed_word(regs, status, acknowledge_idle).map(|word| word.map(|value| value as u8))
}

pub(super) fn receive_observed_word(
    regs: pac::uart::Uart,
    status: regs::Isr,
    acknowledge_idle: bool,
) -> Result<Option<u16>, Error> {
    let pending = receive_event(status);
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    let pending = pending || (acknowledge_idle && status.rxidle());
    #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1)))]
    let _ = acknowledge_idle;
    if !pending {
        return Ok(None);
    }
    let data = if status.rc() {
        Some(regs.rdr().read().rdr())
    } else {
        None
    };
    // R1W0: start from the generated no-op command, then clear only observed
    // receive events. Unrelated bits and events absent from the snapshot remain
    // untouched, including reserved reset-one bits. A repeated assertion of an
    // already observed bit can coalesce before acknowledgment; no ordering or
    // lossless guarantee follows. Never read-modify-write ICR.
    let mut command = regs::Icr::write_noop();
    command.set_rc(!status.rc());
    command.set_fe(!status.fe());
    command.set_pe(!status.pe());
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    {
        command.set_ne(!status.ne());
        command.set_ore(!status.ore());
        command.set_rxidle(!(acknowledge_idle && status.rxidle()));
    }
    regs.icr().write_value(command);
    // Data-loss errors have priority, then parity, framing and noise. Clear
    // every observed receive event even when reporting only the highest one.
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    if status.ore() {
        return Err(Error::Overrun);
    }
    if status.pe() {
        return Err(Error::Parity);
    }
    if status.fe() {
        return Err(Error::Framing);
    }
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    if status.ne() {
        return Err(Error::Noise);
    }
    Ok(data)
}

fn on_interrupt(
    regs: pac::uart::Uart,
    state: &State,
    owners: Lifecycle,
    cs: critical_section::CriticalSection<'_>,
) -> (bool, bool) {
    let enabled = regs.ier().read();
    let status = regs.isr().read();
    let rx_data_pending = (status.rc() && enabled.rc())
        || (status.fe() && enabled.fe())
        || (status.pe() && enabled.pe());
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    let rx_data_pending =
        rx_data_pending || (status.ne() && enabled.ne()) || (status.ore() && enabled.ore());
    let rx_pending = rx_data_pending;
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    let rx_pending = rx_pending
        || (status.rxidle() && enabled.rxidle())
        || (status.rxbrk() && enabled.rxbrk());
    let tx_pending = (status.txe() && enabled.txe()) || (status.tc() && enabled.tc());
    // Reuse the sampled IER. Buffered RX stays armed, while ordinary/DMA RX
    // keeps its one-shot masking and leaves RDR/status intact for its waiter.
    let mut next = enabled;
    let mut ring = state.rx_ring.borrow(cs).borrow_mut();
    let wake_rx = if let Some(ring) = ring.as_mut() {
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        let idle_observed = owners.rx_live && status.rxidle() && enabled.rxidle();
        #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1)))]
        let idle_observed = false;
        if owners.rx_live && (rx_data_pending || idle_observed) {
            // Exactly one receive observation per dispatch, even when full or
            // an error is pending. Drain/acknowledge before applying the barrier.
            let observation = receive_observed(regs, status, idle_observed);
            ring.observe(observation);
        }
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        if idle_observed {
            // Latch after receive observation so simultaneous data/errors are
            // represented. Later appends cannot extend this unread prefix.
            let snapshot = BufferedIdle {
                buffered_bytes: ring.len,
                error: ring.error,
            };
            if ring.idle_capture.active {
                ring.idle_capture.push(snapshot);
            } else {
                ring.idle_snapshot = Some(snapshot);
                next.set_rxidle(false);
            }
        }
        if owners.rx_live && ring.waiting && (ring.len != 0 || ring.error.is_some()) {
            ring.waiting = false;
            true
        } else {
            idle_observed
        }
    } else {
        if rx_pending {
            WaitEvent::Receive.set_enabled(&mut next, false);
        }
        rx_pending
    };
    if tx_pending {
        next.set_txe(false);
        next.set_tc(false);
    }
    regs.ier().write_value(next);
    (wake_rx, tx_pending)
}

pub(super) struct Wait<'a> {
    regs: pac::uart::Uart,
    waker: &'a AtomicWaker,
    event: WaitEvent,
}
impl<'a> Wait<'a> {
    pub(super) fn new(regs: pac::uart::Uart, waker: &'a AtomicWaker, event: WaitEvent) -> Self {
        Self { regs, waker, event }
    }
}
impl Future for Wait<'_> {
    type Output = ();
    fn poll(self: FuturePin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Register before arming, then recheck after arming. Both an interrupt
        // before first poll and a hardware event during arming are retained.
        self.waker.register(cx.waker());
        critical_section::with(|_| {
            if let WaitEvent::TxComplete = self.event {
                if !self.regs.isr().read().txbusy() {
                    return Poll::Ready(());
                }
                // TC may describe an earlier frame while a later one is queued.
                let mut command = regs::Icr::write_noop();
                command.set_tc(false);
                self.regs.icr().write_value(command);
            }
            self.regs.ier().modify(|w| self.event.set_enabled(w, true));
            if self.event.ready(self.regs.isr().read()) {
                self.regs.ier().modify(|w| self.event.set_enabled(w, false));
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
    }
}
impl Drop for Wait<'_> {
    fn drop(&mut self) {
        // No pointer into the caller's buffer is installed in the ISR. Cancelling
        // disables only this half's interrupts and leaves unread RX flags intact.
        critical_section::with(|_| {
            self.regs.ier().modify(|w| self.event.set_enabled(w, false));
        });
    }
}
