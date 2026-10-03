//! Shared sequence lifecycle. IP backends select the real completion source
//! and preserve their own register, clock, watchdog and channel contracts.
use super::Error;
use crate::interrupt::EventState;
use core::task::{Context, Poll};

pub(super) trait SequenceIo {
    fn disarm(&mut self);
    fn busy(&mut self) -> bool;
    fn control(&mut self, word: u32);
    fn clear(&mut self);
}

pub(super) trait AsyncSequenceIo: SequenceIo {
    fn completion_interrupt_enabled(&mut self) -> bool;
    fn completion_pending(&mut self) -> bool;
    fn completion_interrupt(&mut self, enabled: bool);
    fn clear_completion(&mut self);
    /// Stopping must also reset the conversion position for the next operation.
    fn start(&mut self, enabled: bool);
    fn trigger(&mut self, mask: u32);
}

pub(super) fn disarm_idle(io: &mut impl SequenceIo) -> Result<(), Error> {
    // Hardware may start a sequence up to the disarm write. Inspect START afterwards.
    io.disarm();
    if io.busy() {
        Err(Error::Busy)
    } else {
        Ok(())
    }
}

pub(super) fn prepare_sequence(io: &mut impl SequenceIo, cr: u32) -> Result<(), Error> {
    disarm_idle(io)?;
    io.control(cr);
    io.clear();
    Ok(())
}

const SEQUENCE_COMPLETE: u32 = 1;

pub(super) fn poll_sequence(state: &EventState, cx: &mut Context<'_>) -> Poll<()> {
    // Register first, then consume the latch: completion before registration,
    // during registration, or after the check cannot be lost.
    state.register(cx.waker());
    if state.take() & SEQUENCE_COMPLETE != 0 {
        Poll::Ready(())
    } else {
        Poll::Pending
    }
}

fn start_async_sequence(
    io: &mut impl AsyncSequenceIo,
    state: &EventState,
    cr: u32,
    trigger: u32,
) -> Result<(), Error> {
    disarm_idle(io)?;
    io.completion_interrupt(false);
    state.reset();
    io.control(cr);
    io.clear_completion();
    io.completion_interrupt(true);
    if trigger == 0 {
        io.start(true);
    } else {
        io.trigger(trigger);
    }
    Ok(())
}

fn service_completion(io: &mut impl AsyncSequenceIo) -> bool {
    if !io.completion_interrupt_enabled() || !io.completion_pending() {
        return false;
    }
    io.disarm();
    // Clear BEFORE checking START. A late sequence can finish while the CPU
    // runs even inside a critical section. Clearing after reading busy could
    // erase its final completion and leave a sleeping future with no remaining IRQ.
    io.clear_completion();
    // Each backend uses a non-continuous mode that clears START on completion.
    // With the route disarmed, at most one operation remains.
    if io.busy() {
        return false;
    }
    io.completion_interrupt(false);
    true
}

pub(super) fn on_interrupt(io: &mut impl AsyncSequenceIo, state: &EventState) {
    let waker = critical_section::with(|_| {
        if service_completion(io) {
            // Keep completion publication indivisible with hardware cleanup
            // even if an enclosing executor can run in another interrupt.
            state.latch(SEQUENCE_COMPLETE)
        } else {
            None
        }
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}

pub(super) fn cancel_async_sequence(io: &mut impl AsyncSequenceIo, state: &EventState) {
    io.completion_interrupt(false);
    io.disarm();
    io.start(false);
    io.clear_completion();
    state.reset();
}

pub(super) struct ConversionGuard<'a, IO: AsyncSequenceIo> {
    io: IO,
    state: &'a EventState,
}

impl<'a, IO: AsyncSequenceIo> ConversionGuard<'a, IO> {
    pub(super) fn start(
        mut io: IO,
        state: &'a EventState,
        cr: u32,
        trigger: u32,
    ) -> Result<Self, Error> {
        critical_section::with(|_| start_async_sequence(&mut io, state, cr, trigger))?;
        Ok(Self { io, state })
    }
}

impl<IO: AsyncSequenceIo> Drop for ConversionGuard<'_, IO> {
    fn drop(&mut self) {
        critical_section::with(|_| cancel_async_sequence(&mut self.io, self.state));
    }
}

#[cfg(any(dma_l012, dma_f030))]
mod dma_support {
    use super::super::{Error, Instance, Sequence};
    use crate::{dma, Async, Mode};
    use core::{
        cell::Cell,
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };
    use critical_section::Mutex;

    #[derive(Clone, Copy)]
    enum Phase {
        Idle,
        Running(fn()),
        Poisoned,
    }

    /// Lives with the generated ADC identity, never with a forgettable owner.
    pub(crate) struct DmaState(Mutex<Cell<Phase>>);
    impl DmaState {
        pub(crate) const fn new() -> Self {
            Self(Mutex::new(Cell::new(Phase::Idle)))
        }
        pub(crate) fn check(&self) -> Result<(), Error> {
            critical_section::with(|cs| match self.0.borrow(cs).get() {
                Phase::Idle => Ok(()),
                Phase::Running(_) => Err(Error::Busy),
                Phase::Poisoned => Err(Error::DmaPoisoned),
            })
        }
        fn begin<C: dma::Instance>(&self) {
            critical_section::with(|cs| {
                self.0
                    .borrow(cs)
                    .set(Phase::Running(dma::cancel_channel::<C>))
            });
        }
        pub(crate) fn complete(&self, clean: bool) {
            critical_section::with(|cs| {
                self.0
                    .borrow(cs)
                    .set(if clean { Phase::Idle } else { Phase::Poisoned })
            });
        }
        pub(crate) fn cancel(&self) {
            critical_section::with(|cs| {
                if let Phase::Running(cancel) = self.0.borrow(cs).get() {
                    // The channel atomically removes its terminal hook before
                    // publishing completion. It can never refer to a later user.
                    cancel();
                }
            });
        }
    }

    /// An ADC setup failure or DMA launch validation failure.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DmaStartFailure {
        Adc(Error),
        Dma(dma::Error),
    }

    /// No transfer started: the sequence and destination remain owned and usable.
    pub struct DmaStartError<S> {
        pub error: DmaStartFailure,
        pub sequence: S,
        pub destination: &'static mut [u32],
    }

    /// A completed finite scan, including the owned static input resources.
    pub struct DmaBuffers<S> {
        pub sequence: S,
        /// One native 32-bit register word per slot; the ADC code is in bits 11:0.
        pub destination: &'static mut [u32],
    }

    /// A hardware-requested finite ADC read owning a static sequence and buffer.
    ///
    /// Clean DMA TC returns both. Drop/cancel/error/timeout quarantine the input
    /// resources and destination unless clean TC was already observed. Forgetting
    /// leaks them; persistent ADC and DMA leases still forbid competing operations.
    #[must_use = "dropping a started ADC DMA read quarantines its resources"]
    pub struct DmaTransfer<
        'a,
        'd,
        'c,
        'dc,
        I: Instance,
        M: Mode,
        C: dma::Instance,
        DM: Mode,
        const N: usize,
    > {
        sequence: Option<Sequence<'a, 'd, 'static, I, M, N>>,
        transfer: Option<dma::Transfer<'c, 'dc, C, DM, &'static mut [u32]>>,
    }

    impl<'a, 'd, I: Instance, M: Mode, const N: usize> Sequence<'a, 'd, 'static, I, M, N> {
        /// Consume the sequence for one software-started hardware-requested scan.
        /// The destination length must equal the number of slots. L012 supports
        /// 1–8 slots via EOS/BULK; F030 supports one MODE=0 conversion only.
        ///
        /// Static channel guards are required as well as a static destination:
        /// a forgotten or failed DMA read must not release a pin or internal
        /// analog resource still involved in the conversion. Async DMA channels
        /// require their own checked DMA interrupt binding; an ADC IRQ binding
        /// alone is insufficient. Blocking ADC owners may also use Async DMA.
        pub fn read_dma<'c, 'dc, C: dma::Instance, DM: Mode>(
            mut self,
            channel: &'c mut dma::Channel<'dc, C, DM>,
            destination: &'static mut [u32],
        ) -> Result<DmaTransfer<'a, 'd, 'c, 'dc, I, M, C, DM, N>, DmaStartError<Self>> {
            if destination.len() != N {
                return Err(DmaStartError {
                    error: DmaStartFailure::Dma(dma::Error::LengthMismatch),
                    sequence: self,
                    destination,
                });
            }
            if let Err(error) = self.prepare_dma() {
                return Err(DmaStartError {
                    error: DmaStartFailure::Adc(error),
                    sequence: self,
                    destination,
                });
            }
            let result = critical_section::with(|_| {
                I::dma_state().begin::<C>();
                // SAFETY: each backend proves its result bank width/stride,
                // finite conversion and generated request identity. This guard
                // owns all static input resources; persistent hooks preserve the
                // ADC lease through forgotten guards and owner reconstruction.
                let result = unsafe {
                    channel.read_peripheral(
                        I::regs().result(0).as_ptr() as *const u32,
                        destination,
                        Self::dma_config(),
                        Self::finish_dma,
                    )
                };
                match result {
                    Ok(transfer) => {
                        Self::start_dma();
                        Ok(transfer)
                    }
                    Err(error) => {
                        I::dma_state().complete(true); // validation never armed DMA
                        Err(error)
                    }
                }
            });
            match result {
                Ok(transfer) => Ok(DmaTransfer {
                    sequence: Some(self),
                    transfer: Some(transfer),
                }),
                Err(error) => Err(DmaStartError {
                    error: DmaStartFailure::Dma(error.error),
                    sequence: self,
                    destination: error.buffers,
                }),
            }
        }
    }

    impl<'a, 'd, I: Instance, M: Mode, C: dma::Instance, DM: Mode, const N: usize>
        DmaTransfer<'a, 'd, '_, '_, I, M, C, DM, N>
    {
        /// Wait for a bounded number of observations. Timeout quarantines both
        /// the ADC input resources and destination, unless the final check sees TC.
        pub fn blocking_wait(
            mut self,
            poll_budget: u32,
        ) -> Result<DmaBuffers<Sequence<'a, 'd, 'static, I, M, N>>, dma::Error> {
            let result = self.transfer.take().unwrap().blocking_wait(poll_budget);
            self.finish(result)
        }
        /// Request stop; ownership is returned only if clean TC already occurred.
        pub fn cancel(
            mut self,
        ) -> Result<DmaBuffers<Sequence<'a, 'd, 'static, I, M, N>>, dma::Error> {
            let result = self.transfer.take().unwrap().cancel();
            self.finish(result)
        }
        fn finish(
            &mut self,
            result: Result<&'static mut [u32], dma::Error>,
        ) -> Result<DmaBuffers<Sequence<'a, 'd, 'static, I, M, N>>, dma::Error> {
            let sequence = self
                .sequence
                .take()
                .expect("completed ADC DMA read polled again");
            match result {
                Ok(destination) => Ok(DmaBuffers {
                    sequence,
                    destination,
                }),
                Err(error) => {
                    // These may own internal reference/pin guards with Drop.
                    // Dropping the sequence would release those resources before
                    // an outstanding peripheral read is proven finished.
                    core::mem::forget(sequence);
                    Err(error)
                }
            }
        }
    }
    impl<I: Instance, M: Mode, C: dma::Instance, DM: Mode, const N: usize> Drop
        for DmaTransfer<'_, '_, '_, '_, I, M, C, DM, N>
    {
        fn drop(&mut self) {
            if let Some(transfer) = self.transfer.take() {
                let result = transfer.cancel();
                let sequence = self.sequence.take().unwrap();
                if result.is_err() {
                    core::mem::forget(sequence);
                }
            }
        }
    }
    impl<'a, 'd, I: Instance, M: Mode, C: dma::Instance, const N: usize> Future
        for DmaTransfer<'a, 'd, '_, '_, I, M, C, Async, N>
    {
        type Output = Result<DmaBuffers<Sequence<'a, 'd, 'static, I, M, N>>, dma::Error>;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            match Pin::new(
                this.transfer
                    .as_mut()
                    .expect("completed ADC DMA read polled again"),
            )
            .poll(cx)
            {
                Poll::Pending => Poll::Pending,
                Poll::Ready(result) => {
                    this.transfer.take();
                    Poll::Ready(this.finish(result))
                }
            }
        }
    }
}

#[cfg(any(dma_l012, dma_f030))]
pub(crate) use dma_support::DmaState;
#[cfg(any(dma_l012, dma_f030))]
pub use dma_support::{DmaBuffers, DmaStartError, DmaStartFailure, DmaTransfer};
