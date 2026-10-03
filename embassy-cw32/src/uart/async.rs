//! Shared one-shot IRQ waits. No pointers into user buffers enter ISR state.

use super::{byte_mode, Error, Instance, UartRegisters, UartRx, UartTx};
use crate::Async;
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

/// Handler for the UART's physical vector. Each invocation examines only this
/// instance's enabled sources, including when a vector has multiple handlers.
pub struct InterruptHandler<I: Instance>(PhantomData<I>);
impl<I: Instance> crate::interrupt::typelevel::Handler<I::Interrupt> for InterruptHandler<I> {
    unsafe fn on_interrupt() {
        let (tx_waker, rx_waker) = critical_section::with(|_| {
            // A shared NVIC line may remain enabled after both halves drop.
            if !I::clock_resource().is_enabled() {
                return (None, None);
            }
            let (tx, rx) = I::regs().interrupt_pending();
            let tx_waker = if tx {
                I::regs().ier().modify(|w| {
                    w.set_txe(false);
                    w.set_tc(false);
                });
                I::state().tx.latch(1)
            } else {
                None
            };
            let rx_waker = if rx {
                I::regs().rx_interrupt(false);
                I::state().rx.latch(1)
            } else {
                None
            };
            // Leave receive data and error flags for the owner to consume in
            // one critical section. Masking the source prevents an IRQ storm.
            (tx_waker, rx_waker)
        });
        if let Some(waker) = tx_waker {
            waker.wake();
        }
        if let Some(waker) = rx_waker {
            waker.wake();
        }
    }
}
#[derive(Clone, Copy)]
enum WaitKind {
    TxReady,
    TxComplete,
    Rx,
}
impl WaitKind {
    fn state<I: Instance>(self) -> &'static crate::interrupt::EventState {
        match self {
            Self::Rx => &I::state().rx,
            _ => &I::state().tx,
        }
    }
    fn ready<I: Instance>(self) -> bool {
        match self {
            Self::TxReady => I::regs().isr().read().txe(),
            Self::TxComplete => !I::regs().isr().read().txbusy(),
            Self::Rx => I::regs().rx_pending(),
        }
    }
    fn arm<I: Instance>(self, enable: bool) {
        match self {
            Self::Rx => I::regs().rx_interrupt(enable),
            Self::TxReady => I::regs().ier().modify(|w| w.set_txe(enable)),
            Self::TxComplete => I::regs().ier().modify(|w| w.set_tc(enable)),
        }
    }
}
struct Wait<'a, I: Instance> {
    kind: WaitKind,
    armed: bool,
    _borrow: PhantomData<&'a mut I>,
}
impl<'a, I: Instance> Wait<'a, I> {
    fn new<T>(_owner: &'a mut T, kind: WaitKind) -> Self {
        Self {
            kind,
            armed: false,
            _borrow: PhantomData,
        }
    }
}
impl<I: Instance> Future for Wait<'_, I> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        critical_section::with(|_| {
            let state = this.kind.state::<I>();
            state.register(cx.waker());
            // Register, inspect, arm, and inspect again while IRQ dispatch is
            // excluded. Hardware may advance at any point; no edge can be lost.
            let _ = state.take();
            if this.kind.ready::<I>() {
                this.kind.arm::<I>(false);
                state.reset();
                this.armed = false;
                return Poll::Ready(());
            }
            this.kind.arm::<I>(true);
            this.armed = true;
            if this.kind.ready::<I>() {
                this.kind.arm::<I>(false);
                state.reset();
                this.armed = false;
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
    }
}
impl<I: Instance> Drop for Wait<'_, I> {
    fn drop(&mut self) {
        if self.armed {
            critical_section::with(|_| {
                self.kind.arm::<I>(false);
                self.kind.state::<I>().reset();
            });
        }
    }
}
impl<I: Instance> UartTx<'_, I, Async> {
    /// Queue all bytes using UART interrupts. Cancellation leaves an already
    /// queued prefix transmitting and cancels only the next readiness wait.
    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        byte_mode(self.nine_bits)?;
        for &byte in bytes {
            Wait::<I>::new(self, WaitKind::TxReady).await;
            // This half is exclusively borrowed; TXE cannot become false
            // without a TDR write from this owner.
            self.try_write_u16(u16::from(byte));
        }
        Ok(())
    }
    pub async fn write_u16(&mut self, words: &[u16]) -> Result<(), Error> {
        for &word in words {
            Wait::<I>::new(self, WaitKind::TxReady).await;
            self.try_write_u16(word);
        }
        Ok(())
    }
    /// Wait for physical completion. Cancellation leaves transmission running.
    pub async fn flush(&mut self) -> Result<(), Error> {
        Wait::<I>::new(self, WaitKind::TxComplete).await;
        Ok(())
    }
}
impl<I: Instance> UartRx<'_, I, Async> {
    /// Fill the whole buffer, or return the first receive error. Cancellation
    /// keeps the completed prefix and disables this receive wait's interrupts;
    /// a pending hardware word is left available to the next read. The completed
    /// prefix length is not returned, so protocol framing must allow recovery.
    pub async fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        byte_mode(self.nine_bits)?;
        for byte in bytes {
            loop {
                Wait::<I>::new(self, WaitKind::Rx).await;
                if let Some(word) = self.try_read_u16()? {
                    *byte = word as u8;
                    break;
                }
            }
        }
        Ok(())
    }
    pub async fn read_u16(&mut self, words: &mut [u16]) -> Result<(), Error> {
        for word in words {
            loop {
                Wait::<I>::new(self, WaitKind::Rx).await;
                if let Some(value) = self.try_read_u16()? {
                    *word = value;
                    break;
                }
            }
        }
        Ok(())
    }
}
