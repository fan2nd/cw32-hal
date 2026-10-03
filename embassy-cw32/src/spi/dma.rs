//! Finite full-duplex DMA with two owned channels and equal static buffers.
//!
//! Both channel TCs, absence of peripheral errors, an empty receive buffer, and
//! TXE with !BUSY are required before returning resources. Every other exit
//! quarantines both channels, buffers, all bus pins and both clock domains.

use super::{backend, common, Instance, Spi};
use crate::{dma as engine, Async, Mode};
use core::{
    future::Future,
    mem::ManuallyDrop,
    pin::Pin,
    task::{Context, Poll},
};

pub(crate) mod sealed {
    pub trait TxDma<I> {}
    pub trait RxDma<I> {}
}
/// A DMA channel with this SPI instance's generated transmit request.
#[allow(private_bounds)]
pub trait TxDma<I: Instance>: engine::Instance + sealed::TxDma<I> {
    const REQUEST: engine::Request;
}
/// A DMA channel with this SPI instance's generated receive request.
#[allow(private_bounds)]
pub trait RxDma<I: Instance>: engine::Instance + sealed::RxDma<I> {
    const REQUEST: engine::Request;
}
/// The audited 8-bit and 16-bit SPI/DMA word pairs.
pub trait Word: super::Word + engine::Word {}
impl Word for u8 {}
impl Word for u16 {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Dma(engine::Error),
    Spi(super::Error),
}
/// A failure before start returns every untouched owner and buffer.
pub struct StartError<R> {
    pub error: Error,
    pub resources: R,
}
/// Complete resource bundle returned only after a clean, fully drained transfer.
pub struct Resources<I: Instance, TX: TxDma<I>, RX: RxDma<I>, M: Mode, W: Word> {
    pub spi: Spi<'static, I, M>,
    pub tx_channel: engine::Channel<'static, TX, M>,
    pub rx_channel: engine::Channel<'static, RX, M>,
    pub write: &'static mut [W],
    pub read: &'static mut [W],
}
/// Started finite full-duplex exchange. Forgetting it retains all resources;
/// dropping it requests stop and permanently poisons both channels.
#[must_use = "dropping SPI DMA quarantines both channels, buffers and the bus"]
pub struct Transfer<I: Instance, TX: TxDma<I>, RX: RxDma<I>, M: Mode, W: Word> {
    resources: Option<ManuallyDrop<Resources<I, TX, RX, M, W>>>,
}
impl<I: Instance, TX: TxDma<I>, RX: RxDma<I>, M: Mode, W: Word> Unpin
    for Transfer<I, TX, RX, M, W>
{
}

fn requests<I: Instance>(tx: Option<bool>, rx: Option<bool>) {
    #[cfg(spi_l012)]
    I::regs().cr2().modify(|w| {
        if let Some(enable) = tx {
            w.set_dmatx(enable);
        }
        if let Some(enable) = rx {
            w.set_dmarx(enable);
        }
    });
    #[cfg(spi_f030)]
    I::regs().cr1().modify(|w| {
        if let Some(enable) = tx {
            w.set_dmatx(enable);
        }
        if let Some(enable) = rx {
            w.set_dmarx(enable);
        }
    });
}
fn tx_terminal<I: Instance>(complete: bool) {
    requests::<I>(Some(false), if complete { None } else { Some(false) });
}
fn rx_terminal<I: Instance>(complete: bool) {
    requests::<I>(if complete { None } else { Some(false) }, Some(false));
}

impl<I: Instance, M: Mode> Spi<'static, I, M> {
    /// Exchange equal nonzero buffers of at most 65535 words. `u8` selects
    /// eight-bit frames; `u16` selects sixteen-bit frames. RX DMA is armed before
    /// TX requests, and every transmitted frame has a receive destination.
    ///
    /// Use explicit dummy transmit/receive buffers for read-only or write-only
    /// protocols. In-place DMA and unequal lengths are intentionally absent.
    /// The existing Config::poll_limit bounds the final TXE/!BUSY drain. External
    /// chip select remains the caller's responsibility, including cancel paths.
    pub fn transfer_dma<TX: TxDma<I>, RX: RxDma<I>, W: Word>(
        self,
        tx_channel: engine::Channel<'static, TX, M>,
        rx_channel: engine::Channel<'static, RX, M>,
        write: &'static mut [W],
        read: &'static mut [W],
    ) -> Result<Transfer<I, TX, RX, M, W>, StartError<Resources<I, TX, RX, M, W>>> {
        let resources = Resources {
            spi: self,
            tx_channel,
            rx_channel,
            write,
            read,
        };
        let result = critical_section::with(|_| {
            if TX::INDEX == RX::INDEX {
                return Err(engine::Error::ChannelBusy);
            }
            if resources.write.len() != resources.read.len() {
                return Err(engine::Error::LengthMismatch);
            }
            let dr = I::regs().dr().as_ptr().cast::<W>();
            let tx = resources.tx_channel.prepare_endpoint(
                resources.write.as_ptr(),
                dr,
                resources.write.len(),
                engine::RawConfig {
                    trigger: engine::Trigger::Hardware(TX::REQUEST),
                    mode: engine::TransferMode::Block,
                    source_increment: true,
                    destination_increment: false,
                },
            )?;
            let rx = resources.rx_channel.prepare_endpoint(
                dr,
                resources.read.as_mut_ptr(),
                resources.read.len(),
                engine::RawConfig {
                    trigger: engine::Trigger::Hardware(RX::REQUEST),
                    mode: engine::TransferMode::Block,
                    source_increment: false,
                    destination_increment: true,
                },
            )?;
            Ok((tx, rx))
        });
        let (tx, rx) = match result {
            Ok(configs) => configs,
            Err(error) => {
                return Err(StartError {
                    error: Error::Dma(error),
                    resources,
                })
            }
        };
        let mut transfer = Transfer {
            resources: Some(ManuallyDrop::new(resources)),
        };
        critical_section::with(|_| {
            let r = transfer.resources.as_mut().unwrap();
            // Safely terminate any forgotten CPU-driven single-frame operation.
            // No earlier DMA operation can return this owner without full drain.
            r.spi.stop();
            backend::configure(I::regs(), &r.spi.config, r.spi.br, W::BITS);
            I::state().reset();
            if M::ASYNC {
                I::regs()
                    .ier()
                    .write_value(crate::pac::spi::regs::Ier(0xf0));
            }
            // SAFETY: all endpoint owners and static buffers are now retained
            // in the transfer. Both channels were validated before either start.
            unsafe {
                r.rx_channel.start_endpoint(rx, rx_terminal::<I>);
                r.tx_channel.start_endpoint(tx, tx_terminal::<I>);
            }
            // RX request logic must be live before the first TX can clock data.
            requests::<I>(None, Some(true));
            requests::<I>(Some(true), None);
        });
        Ok(transfer)
    }
}

impl<I: Instance, TX: TxDma<I>, RX: RxDma<I>, M: Mode, W: Word> Transfer<I, TX, RX, M, W> {
    fn check(&self) -> Option<Result<(), Error>> {
        let ready = critical_section::with(|_| {
            if let Err(e) = common::error(I::regs().isr().read().0) {
                return Some(Err(Error::Spi(e)));
            }
            // Inspect both every time: never wait exclusively on TX while RX
            // has already failed, or vice versa. A failure wins over peer TC.
            let tx = engine::completion::<TX>();
            let rx = engine::completion::<RX>();
            if let Some(Err(e)) = tx {
                return Some(Err(Error::Dma(e)));
            }
            if let Some(Err(e)) = rx {
                return Some(Err(Error::Dma(e)));
            }
            if tx == Some(Ok(())) && rx == Some(Ok(())) {
                Some(Ok(()))
            } else {
                None
            }
        });
        match ready? {
            Err(e) => Some(Err(e)),
            Ok(()) => {
                let budget = self.resources.as_ref().unwrap().spi.config.poll_limit;
                if let Err(e) = common::drain::<I>(budget) {
                    return Some(Err(Error::Spi(e)));
                }
                // RXNE must be empty after one matched receive per transmitted
                // frame. Do not silently discard an unexpected residual word.
                if I::regs().isr().read().rxne() {
                    return Some(Err(Error::Spi(super::Error::Overrun)));
                }
                Some(Ok(()))
            }
        }
    }
    fn quarantine(&mut self) {
        critical_section::with(|_| {
            requests::<I>(Some(false), Some(false));
            common::disarm::<I>();
            engine::poison_endpoint::<TX>();
            engine::poison_endpoint::<RX>();
            // Peripheral flush is documented, but DMA abort-drain is not. Even
            // after this stop, retain pins, both channels and both clocks.
            common::stop::<I>();
        });
    }
    fn finish(&mut self, result: Result<(), Error>) -> Result<Resources<I, TX, RX, M, W>, Error> {
        if result.is_err() {
            self.quarantine();
        } else {
            critical_section::with(|_| {
                common::disarm::<I>();
                I::state().reset();
            });
        }
        let resources = self
            .resources
            .take()
            .expect("SPI DMA transfer completed twice");
        match result {
            Ok(()) => Ok(ManuallyDrop::into_inner(resources)),
            Err(e) => Err(e),
        }
    }
    /// Bounded register observations, plus the configured final wire-drain
    /// budget. Timeout retains all owners; it never returns a partial buffer.
    pub fn blocking_wait(mut self, poll_budget: u32) -> Result<Resources<I, TX, RX, M, W>, Error> {
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
    /// Return resources only when both channels and the bus already completed.
    pub fn cancel(mut self) -> Result<Resources<I, TX, RX, M, W>, Error> {
        let result = self
            .check()
            .unwrap_or(Err(Error::Dma(engine::Error::Cancelled)));
        self.finish(result)
    }
}
impl<I: Instance, TX: TxDma<I>, RX: RxDma<I>, W: Word> Future for Transfer<I, TX, RX, Async, W> {
    type Output = Result<Resources<I, TX, RX, Async, W>, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(this.resources.is_some(), "SPI DMA transfer completed twice");
        engine::register_endpoint::<TX>(cx.waker());
        engine::register_endpoint::<RX>(cx.waker());
        I::state().register(cx.waker());
        match this.check() {
            Some(result) => Poll::Ready(this.finish(result)),
            None => Poll::Pending,
        }
    }
}
impl<I: Instance, TX: TxDma<I>, RX: RxDma<I>, M: Mode, W: Word> Drop for Transfer<I, TX, RX, M, W> {
    fn drop(&mut self) {
        if self.resources.is_some() {
            self.quarantine();
        }
    }
}
