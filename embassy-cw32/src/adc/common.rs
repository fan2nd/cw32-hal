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
