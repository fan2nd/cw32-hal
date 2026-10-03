//! Shared one-shot comparator IRQ service, wait and cancellation state machine.

use super::super::{Error, VcInstance};
use super::{Comp, VcHardware};
use crate::Async;

/// Bind every async comparator to its physical IRQ. Each handler checks only
/// its own IE/INTF and never clears, disables or unpends a shared-vector sibling.
pub struct InterruptHandler<I: VcInstance>(core::marker::PhantomData<I>);
impl<I: VcInstance> crate::interrupt::typelevel::Handler<I::Interrupt> for InterruptHandler<I> {
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|_| {
            if !I::clock_resource().is_enabled() {
                return None;
            }
            service_vc_interrupt(&mut VcHardware::<I>(core::marker::PhantomData), I::state())
        });
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// IRQ-backed, cancel-safe one-shot waits. Edges are relative to the first poll,
/// after stale flags are cleared and interrupt selection is enabled. Hardware
/// coalesces edges; these methods are notifications, not an edge counter.
///
/// Enable the comparator first. A disabled comparator returns `Error::Disabled`
/// rather than enabling analog circuitry without its required startup checks.
/// Cancellation clears only this VC's event source; it leaves the comparator
/// enabled and never disables or unpends a shared NVIC vector.
impl<I: VcInstance> Comp<'_, I, Async> {
    /// Wait for a new rising edge of the filtered, configured-polarity output.
    pub async fn wait_for_rising_edge(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::Rising).await
    }
    /// Wait for a new falling edge. Multiple edges may coalesce in hardware.
    pub async fn wait_for_falling_edge(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::Falling).await
    }
    /// Wait for either edge; INTF does not retain which edge occurred.
    pub async fn wait_for_any_edge(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::AnyEdge).await
    }
    /// Complete immediately if already high, otherwise await a high event.
    /// The level can change again before the awaiting task resumes.
    pub async fn wait_for_high(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::High).await
    }
    /// Complete immediately if already low, otherwise await a falling event.
    /// The level can change again before the awaiting task resumes.
    pub async fn wait_for_low(&mut self) -> Result<(), Error> {
        self.wait(WaitKind::Low).await
    }
    async fn wait(&mut self, kind: WaitKind) -> Result<(), Error> {
        self.output_level()?;
        VcWait::new(self, kind).await;
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub(super) enum WaitKind {
    Rising,
    Falling,
    AnyEdge,
    High,
    Low,
}
impl WaitKind {
    fn satisfied(self, high: bool) -> bool {
        matches!((self, high), (Self::High, true) | (Self::Low, false))
    }
}
const VC_EVENT: u32 = 1;
// The state machine deliberately knows no register layout or event bit encoding.
// The selected VC IP implements each hardware operation separately.
pub(super) trait VcIo {
    fn disable(&mut self);
    fn clear(&mut self);
    fn arm(&mut self, kind: WaitKind);
    fn pending(&mut self) -> bool;
    fn high(&mut self) -> bool;
}
fn service_vc_interrupt(
    io: &mut impl VcIo,
    state: &crate::interrupt::EventState,
) -> Option<core::task::Waker> {
    if io.pending() {
        // Disable only this source before RW0 acknowledgment, especially for
        // HIGHIE whose level could otherwise immediately reassert INTF.
        io.disable();
        io.clear();
        state.latch(VC_EVENT)
    } else {
        None
    }
}
struct VcWaitCore {
    armed: bool,
    done: bool,
    kind: WaitKind,
}
impl VcWaitCore {
    fn new(kind: WaitKind) -> Self {
        Self {
            armed: false,
            done: false,
            kind,
        }
    }
    fn poll(
        &mut self,
        io: &mut impl VcIo,
        state: &crate::interrupt::EventState,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        use core::task::Poll;
        assert!(!self.done, "completed comparator future polled again");
        critical_section::with(|_| {
            if !self.armed {
                io.disable();
                io.clear();
                state.reset();
                state.register(cx.waker());
                // Current-level waits also perform a post-enable check below.
                io.arm(self.kind);
                self.armed = true;
            } else {
                state.register(cx.waker());
            }
            // Registration precedes every latch read. A hardware event between
            // arm and this check is either observed here or remains IRQ-pending.
            let signaled = state.take() & VC_EVENT != 0;
            if signaled || io.pending() || self.kind.satisfied(io.high()) {
                io.disable();
                io.clear();
                state.reset();
                self.armed = false;
                self.done = true;
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
    }
    fn cancel(&mut self, io: &mut impl VcIo, state: &crate::interrupt::EventState) {
        if self.armed {
            critical_section::with(|_| {
                io.disable();
                io.clear();
                state.reset();
                self.armed = false;
            });
        }
    }
}
struct VcWait<'a, 'd, I: VcInstance> {
    _driver: &'a mut Comp<'d, I, Async>,
    core: VcWaitCore,
}
impl<'a, 'd, I: VcInstance> VcWait<'a, 'd, I> {
    fn new(driver: &'a mut Comp<'d, I, Async>, kind: WaitKind) -> Self {
        Self {
            _driver: driver,
            core: VcWaitCore::new(kind),
        }
    }
}
impl<I: VcInstance> core::future::Future for VcWait<'_, '_, I> {
    type Output = ();
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<()> {
        self.get_mut().core.poll(
            &mut VcHardware::<I>(core::marker::PhantomData),
            I::state(),
            cx,
        )
    }
}
impl<I: VcInstance> Drop for VcWait<'_, '_, I> {
    fn drop(&mut self) {
        self.core
            .cancel(&mut VcHardware::<I>(core::marker::PhantomData), I::state());
    }
}
