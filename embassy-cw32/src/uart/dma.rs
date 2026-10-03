//! Finite DMA with complete, static ownership of the UART half and channel.
//!
//! Success returns every resource. Error, cancellation, timeout and Drop retain
//! the UART half (including pin and clock), channel and buffer permanently.
//! Forgetting a transfer also retains them. No borrowed-buffer abort proof is
//! assumed. Eight-bit payloads only; see `docs/bus-dma.md`.

use super::{Instance, UartRx, UartTx};
use crate::{dma as engine, Async, Mode};
use core::{
    future::Future,
    marker::PhantomData,
    mem::ManuallyDrop,
    pin::Pin,
    task::{Context, Poll},
};

pub(crate) mod sealed {
    pub trait TxDma<I> {}
    pub trait RxDma<I> {}
}
/// A channel with the generated transmit request for this UART instance.
#[allow(private_bounds)]
pub trait TxDma<I: Instance>: engine::Instance + sealed::TxDma<I> {
    const REQUEST: engine::Request;
}
/// A channel with the generated receive request for this UART instance.
#[allow(private_bounds)]
pub trait RxDma<I: Instance>: engine::Instance + sealed::RxDma<I> {
    const REQUEST: engine::Request;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Dma(engine::Error),
    Uart(super::Error),
    /// Finish earlier queued TX data, or consume pending RX data, before start.
    Busy,
    /// RX DMA is restricted to one frame until RC acknowledgement is proven.
    ReceiveLength,
}
/// A validation failure returns the untouched owners and buffer.
pub struct StartError<R> {
    pub error: Error,
    pub resources: R,
}
/// All resources consumed by a finite UART DMA operation.
pub struct Resources<U, C: engine::Instance, M: Mode> {
    pub uart: U,
    pub channel: engine::Channel<'static, C, M>,
    pub buffer: &'static mut [u8],
}

pub(crate) trait Endpoint<I: Instance> {
    const RX: bool;
    fn nine_bits(&self) -> bool;
}
impl<I: Instance, M: Mode> Endpoint<I> for UartTx<'static, I, M> {
    const RX: bool = false;
    fn nine_bits(&self) -> bool {
        self.nine_bits
    }
}
impl<I: Instance, M: Mode> Endpoint<I> for UartRx<'static, I, M> {
    const RX: bool = true;
    fn nine_bits(&self) -> bool {
        self.nine_bits
    }
}

/// A started finite operation. Only success releases any consumed resource.
#[must_use = "dropping a UART DMA transfer quarantines its owners and buffer"]
#[allow(private_bounds)]
pub struct Transfer<U: Endpoint<I>, I: Instance, C: engine::Instance, M: Mode> {
    resources: Option<ManuallyDrop<Resources<U, C, M>>>,
    drain_budget: u32,
    _instance: PhantomData<I>,
}
impl<U: Endpoint<I>, I: Instance, C: engine::Instance, M: Mode> Unpin for Transfer<U, I, C, M> {}

pub type TxTransfer<I, C, M> = Transfer<UartTx<'static, I, M>, I, C, M>;
pub type RxTransfer<I, C, M> = Transfer<UartRx<'static, I, M>, I, C, M>;

impl<I: Instance, M: Mode> UartTx<'static, I, M> {
    /// Transmit 1..=65535 eight-bit payloads. DMA TC is followed by a bounded
    /// TXBUSY drain before owners are returned. The budget counts observations,
    /// not elapsed time. Requires a fully owned static UART half and channel.
    pub fn write_dma<C: TxDma<I>>(
        self,
        channel: engine::Channel<'static, C, M>,
        buffer: &'static mut [u8],
        drain_budget: u32,
    ) -> Result<TxTransfer<I, C, M>, StartError<Resources<Self, C, M>>> {
        Transfer::start(
            Resources {
                uart: self,
                channel,
                buffer,
            },
            C::REQUEST,
            drain_budget,
        )
    }
}
impl<I: Instance, M: Mode> UartRx<'static, I, M> {
    /// Receive exactly one eight-bit payload. A longer buffer is rejected before
    /// starting: the manuals do not explain DMA-specific RC acknowledgement.
    /// F030 cannot detect overwritten receive data; the sender must respect the
    /// finite receive window. Receive errors quarantine the complete operation.
    pub fn read_dma<C: RxDma<I>>(
        self,
        channel: engine::Channel<'static, C, M>,
        buffer: &'static mut [u8],
    ) -> Result<RxTransfer<I, C, M>, StartError<Resources<Self, C, M>>> {
        Transfer::start(
            Resources {
                uart: self,
                channel,
                buffer,
            },
            C::REQUEST,
            1,
        )
    }
}

fn receive_error<I: Instance>() -> Option<super::Error> {
    let s = I::regs().isr().read();
    #[cfg(uart_l012)]
    {
        if s.ore() {
            return Some(super::Error::Overrun);
        }
        if s.ne() {
            return Some(super::Error::Noise);
        }
    }
    if s.fe() {
        Some(super::Error::Framing)
    } else if s.pe() {
        Some(super::Error::Parity)
    } else {
        None
    }
}
fn error_interrupt<I: Instance>(enable: bool) {
    I::regs().ier().modify(|w| {
        w.set_rc(false); // RC stays owned by the DMA handshake, never the ISR.
        w.set_fe(enable);
        w.set_pe(enable);
        #[cfg(uart_l012)]
        {
            w.set_ore(enable);
            w.set_ne(enable);
        }
    });
}
fn terminal<U: Endpoint<I>, I: Instance>(_complete: bool) {
    I::regs().cr2().modify(|w| {
        if U::RX {
            w.set_dmarx(false);
        } else {
            w.set_dmatx(false);
        }
    });
    if U::RX {
        // Stop acceptance at the terminal boundary. This does not establish DMA
        // quiescence on an error: those owners remain permanently retained.
        I::regs().cr1().modify(|w| w.set_rxen(false));
        error_interrupt::<I>(false);
    }
}

#[allow(private_bounds)]
impl<U: Endpoint<I>, I: Instance, C: engine::Instance, M: Mode> Transfer<U, I, C, M> {
    fn start(
        resources: Resources<U, C, M>,
        request: engine::Request,
        drain_budget: u32,
    ) -> Result<Self, StartError<Resources<U, C, M>>> {
        let result = critical_section::with(|_| {
            if resources.uart.nine_bits() {
                return Err(Error::Uart(super::Error::DataBitsMismatch));
            }
            if U::RX && resources.buffer.len() != 1 {
                return Err(Error::ReceiveLength);
            }
            if drain_budget == 0 {
                return Err(Error::Dma(engine::Error::Timeout));
            }
            if U::RX {
                if let Some(e) = receive_error::<I>() {
                    return Err(Error::Uart(e));
                }
                if I::regs().isr().read().rc() {
                    return Err(Error::Busy);
                }
            } else if I::regs().isr().read().txbusy() {
                return Err(Error::Busy);
            }
            let (source, destination) = if U::RX {
                (
                    I::regs().rdr().as_ptr().cast::<u8>() as *const u8,
                    resources.buffer.as_mut_ptr(),
                )
            } else {
                (
                    resources.buffer.as_ptr(),
                    I::regs().tdr().as_ptr().cast::<u8>(),
                )
            };
            resources
                .channel
                .prepare_endpoint(
                    source,
                    destination,
                    resources.buffer.len(),
                    engine::RawConfig {
                        trigger: engine::Trigger::Hardware(request),
                        mode: engine::TransferMode::Block,
                        source_increment: !U::RX,
                        destination_increment: U::RX,
                    },
                )
                .map_err(Error::Dma)
        });
        let prepared = match result {
            Ok(config) => config,
            Err(error) => return Err(StartError { error, resources }),
        };
        let mut transfer = Self {
            resources: Some(ManuallyDrop::new(resources)),
            drain_budget,
            _instance: PhantomData,
        };
        critical_section::with(|_| {
            if U::RX {
                I::state().rx.reset();
                error_interrupt::<I>(M::ASYNC);
            } else {
                I::state().tx.reset();
                I::regs().ier().modify(|w| {
                    w.set_txe(false);
                    w.set_tc(false);
                });
                I::regs().icr().write(|w| w.set_tc(false));
            }
            // SAFETY: the transfer already owns all static endpoint resources.
            // No safe alias can start this exclusive channel between validation
            // and start. Peripheral requests are still disabled.
            unsafe {
                transfer
                    .resources
                    .as_mut()
                    .unwrap()
                    .channel
                    .start_endpoint(prepared, terminal::<U, I>);
            }
            I::regs().cr2().modify(|w| {
                if U::RX {
                    w.set_dmarx(true);
                } else {
                    w.set_dmatx(true);
                }
            });
        });
        Ok(transfer)
    }
    fn check(&self) -> Option<Result<(), Error>> {
        let ready = critical_section::with(|_| {
            if U::RX {
                if let Some(e) = receive_error::<I>() {
                    return Some(Err(Error::Uart(e)));
                }
            }
            match engine::completion::<C>()? {
                Err(e) => Some(Err(Error::Dma(e))),
                Ok(()) => {
                    if U::RX {
                        // The terminal hook has stopped RX, so no new frame can
                        // race this last error snapshot or RC acknowledgement.
                        if let Some(e) = receive_error::<I>() {
                            return Some(Err(Error::Uart(e)));
                        }
                        I::regs().icr().write(|w| w.set_rc(false));
                    }
                    Some(Ok(()))
                }
            }
        });
        match ready? {
            Err(error) => Some(Err(error)),
            Ok(()) if U::RX => Some(Ok(())),
            Ok(()) => {
                // Requests are disabled and this operation exclusively owns TX.
                // Leave unrelated interrupts enabled during the bounded wire
                // drain. Sticky UART TC cannot replace the TXBUSY observation.
                for _ in 0..self.drain_budget {
                    if !I::regs().isr().read().txbusy() {
                        return Some(Ok(()));
                    }
                    core::hint::spin_loop();
                }
                Some(Err(Error::Dma(engine::Error::Timeout)))
            }
        }
    }
    fn quarantine(&mut self) {
        critical_section::with(|_| {
            terminal::<U, I>(false);
            engine::poison_endpoint::<C>();
        });
    }
    fn finish(&mut self, result: Result<(), Error>) -> Result<Resources<U, C, M>, Error> {
        if result.is_err() {
            self.quarantine();
        }
        let resources = self
            .resources
            .take()
            .expect("UART DMA transfer completed twice");
        match result {
            Ok(()) => {
                if U::RX {
                    // CR1 also belongs to a safely split TX half.
                    critical_section::with(|_| I::regs().cr1().modify(|w| w.set_rxen(true)));
                }
                Ok(ManuallyDrop::into_inner(resources))
            }
            Err(e) => Err(e), // ManuallyDrop retains every resource, including clocks.
        }
    }
    pub fn blocking_wait(mut self, poll_budget: u32) -> Result<Resources<U, C, M>, Error> {
        for _ in 0..poll_budget {
            if let Some(result) = self.check() {
                return self.finish(result);
            }
            core::hint::spin_loop();
        }
        let result = self
            .check()
            .unwrap_or(Err(Error::Dma(engine::Error::Timeout)));
        self.finish(result)
    }
    /// Reclaims resources only if DMA and wire completion are already proven.
    pub fn cancel(mut self) -> Result<Resources<U, C, M>, Error> {
        let result = self
            .check()
            .unwrap_or(Err(Error::Dma(engine::Error::Cancelled)));
        self.finish(result)
    }
}
impl<U: Endpoint<I>, I: Instance, C: engine::Instance> Future for Transfer<U, I, C, Async> {
    type Output = Result<Resources<U, C, Async>, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(
            this.resources.is_some(),
            "UART DMA transfer completed twice"
        );
        engine::register_endpoint::<C>(cx.waker());
        if U::RX {
            I::state().rx.register(cx.waker());
        }
        match this.check() {
            Some(result) => Poll::Ready(this.finish(result)),
            None => Poll::Pending,
        }
    }
}
impl<U: Endpoint<I>, I: Instance, C: engine::Instance, M: Mode> Drop for Transfer<U, I, C, M> {
    fn drop(&mut self) {
        if self.resources.is_some() {
            self.quarantine();
        }
    }
}
