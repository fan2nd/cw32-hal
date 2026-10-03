//! Shared full-duplex lifecycle for the two documented CW32 SPI IPs.
use super::{backend, Error, Instance};
use crate::pac::spi::regs;
use core::task::{Context, Poll};

pub(super) const TXE: u32 = 1;
pub(super) const RXNE: u32 = 1 << 1;
const BUSY: u32 = 1 << 8;
const ERRORS: u32 = 0xf0;

pub(super) fn error(status: u32) -> Result<(), Error> {
    if status & (1 << 7) != 0 {
        Err(Error::ModeFault)
    } else if status & (1 << 5) != 0 {
        Err(Error::Overrun)
    } else if status & (1 << 6) != 0 {
        Err(Error::Select)
    } else if status & (1 << 4) != 0 {
        Err(Error::Underrun)
    } else {
        Ok(())
    }
}

pub(super) fn disarm<I: Instance>() {
    I::regs().ier().write_value(regs::Ier(0));
}

/// Finite abort, including the one frame a cancelled/forgotten future queued.
/// Caller keeps the counted clock and pins alive until this finishes.
pub(super) fn stop<I: Instance>() {
    critical_section::with(|_| {
        let r = I::regs();
        disarm::<I>();
        backend::disable(r);
        // Both RMs explicitly define FLUSH=0 as clearing TX buffer AND shift
        // register. Clearing all eight W0C bits also discards RX and errors;
        // all reserved bits reset to zero. This is an abort, never normal flush.
        r.icr().write_value(regs::Icr(0));
        r.ssi().write_value(regs::Ssi(1));
    });
    I::state().reset();
}

pub(super) fn wait_blocking<I: Instance>(flag: u32, poll_limit: u32) -> Result<(), Error> {
    for _ in 0..poll_limit {
        let status = I::regs().isr().read().0;
        error(status)?;
        if status & flag != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(Error::Timeout)
}

/// No CW32 BUSY-clear interrupt exists. After the final RXNE, finish the final
/// clock edge with a finite polling budget. Never report a busy bus as flushed.
pub(super) fn drain<I: Instance>(poll_limit: u32) -> Result<(), Error> {
    for _ in 0..poll_limit {
        let status = I::regs().isr().read().0;
        error(status)?;
        if status & (TXE | BUSY) == TXE {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(Error::Timeout)
}

pub(super) fn poll_flag<I: Instance>(cx: &mut Context<'_>, flag: u32) -> Poll<Result<(), Error>> {
    // Register before checking the latch or hardware. The register check and
    // interrupt arming share a critical section with dispatch/cancellation.
    I::state().register(cx.waker());
    critical_section::with(|_| {
        let r = I::regs();
        let status = I::state().take() | r.isr().read().0;
        if let Err(e) = error(status) {
            disarm::<I>();
            return Poll::Ready(Err(e));
        }
        if status & flag != 0 {
            disarm::<I>();
            return Poll::Ready(Ok(()));
        }
        // A level which asserts after the check still interrupts when enabled.
        r.ier().write_value(regs::Ier(flag | ERRORS));
        Poll::Pending
    })
}

pub(super) fn on_interrupt<I: Instance>() {
    let wake = critical_section::with(|_| {
        // SPI2/SPI3 share a vector on one IP. Late/spurious dispatch after Drop
        // must not touch the register bus once the final clock owner is gone.
        if !I::clock_resource().is_enabled() {
            return None;
        }
        let r = I::regs();
        let pending = r.isr().read().0 & r.ier().read().0;
        if pending == 0 {
            return None;
        }
        // Mask the level source before wake. The future consumes DR and arms
        // its next state; no buffer pointer or lifetime is held by this ISR.
        disarm::<I>();
        I::state().latch(pending)
    });
    if let Some(wake) = wake {
        wake.wake();
    }
}
