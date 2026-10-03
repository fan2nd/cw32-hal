//! Shared single-owner update/break waits; register policy stays in the IP backend.
use super::backend::{
    service_events, set_event_enabled, BreakFlags, HardwareEvents, ThreePhasePwm,
};
use crate::interrupt::{self, EventState, InterruptExt};

static UPDATE_STATE: EventState = EventState::new();
static BREAK_STATE: EventState = EventState::new();

#[derive(Clone, Copy)]
pub(super) enum WaitEvent {
    Update,
    Break,
}
impl WaitEvent {
    fn state(self) -> &'static EventState {
        match self {
            Self::Update => &UPDATE_STATE,
            Self::Break => &BREAK_STATE,
        }
    }
}
pub(super) trait EventIo {
    fn enables(&mut self) -> u32;
    fn status(&mut self) -> u32;
    fn set_enables(&mut self, value: u32);
    fn clear_update(&mut self);
}
// Hardware access is always serialized by the caller's critical section.
// Only update preparation clears a status flag; break latches remain intact.
fn prepare_event(io: &mut impl EventIo, event: WaitEvent) {
    set_event_enabled(io, event, false);
    if matches!(event, WaitEvent::Update) {
        io.clear_update();
    }
    set_event_enabled(io, event, true);
}

/// Bind this handler to the real ATIM interrupt with `bind_interrupts!`.
/// Only enabled update/break subscriptions are serviced. Other ATIM sources
/// remain available to additional handlers in the same binding.
pub struct InterruptHandler;
impl interrupt::typelevel::Handler<interrupt::typelevel::ATIM> for InterruptHandler {
    unsafe fn on_interrupt() {
        let (update, fault) = critical_section::with(|_| {
            let (update, fault) = service_events(&mut HardwareEvents);
            let update = if update != 0 {
                UPDATE_STATE.latch(update)
            } else {
                None
            };
            let fault = if fault != 0 {
                BREAK_STATE.latch(fault)
            } else {
                None
            };
            (update, fault)
        });
        // Hardware service and latch publication are one cancellation-atomic
        // transaction; only owning Wakers leave the CS, never unlatchable bits.
        if let Some(waker) = update {
            waker.wake();
        }
        if let Some(waker) = fault {
            waker.wake();
        }
    }
}
/// IRQ-driven event waits plus the original explicit PWM controls.
/// The wrapper owns the complete ATIM, not just the two subscriptions.
pub struct AsyncThreePhasePwm<'d> {
    inner: ThreePhasePwm<'d>,
}
impl<'d> ThreePhasePwm<'d> {
    /// Install a proved IRQ binding without changing the counter or power state.
    /// Construction and waits never enable phase outputs or acknowledge faults.
    pub fn into_async(
        self,
        _irq: impl interrupt::typelevel::Binding<interrupt::typelevel::ATIM, InterruptHandler>,
    ) -> AsyncThreePhasePwm<'d> {
        critical_section::with(|_| {
            UPDATE_STATE.reset();
            BREAK_STATE.reset();
            set_event_enabled(&mut HardwareEvents, WaitEvent::Update, false);
            set_event_enabled(&mut HardwareEvents, WaitEvent::Break, false);
            // SAFETY: caller supplied the type-level proof for this real vector.
            unsafe {
                interrupt::ATIM.enable();
            }
        });
        AsyncThreePhasePwm { inner: self }
    }
}
impl<'d> core::ops::Deref for AsyncThreePhasePwm<'d> {
    type Target = ThreePhasePwm<'d>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AsyncThreePhasePwm<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
struct WaitGuard(WaitEvent);
impl WaitGuard {
    fn new(event: WaitEvent) -> Self {
        critical_section::with(|_| {
            event.state().reset();
            prepare_event(&mut HardwareEvents, event);
        });
        Self(event)
    }
}
impl Drop for WaitGuard {
    fn drop(&mut self) {
        critical_section::with(|_| {
            set_event_enabled(&mut HardwareEvents, self.0, false);
            self.0.state().reset();
        });
        // Do not disable/unpend the shared vector, clear a fault, or touch MOE.
    }
}
impl AsyncThreePhasePwm<'_> {
    async fn wait_event(&mut self, event: WaitEvent) -> u32 {
        let _guard = WaitGuard::new(event);
        core::future::poll_fn(|cx| {
            event.state().register(cx.waker());
            let latched = event.state().take();
            // Covers hardware completion before IRQ entry (including historical
            // break latches). ISR clears UIF only after copying into EventState.
            let flags = critical_section::with(|_| HardwareEvents.status()) & event.flags();
            let result = latched | flags;
            if result != 0 {
                core::task::Poll::Ready(result)
            } else {
                core::task::Poll::Pending
            }
        })
        .await
    }
    /// Wait for the next update after first polling. Does not start the counter;
    /// call start_counter() explicitly. Updates coalesce, this is not a count.
    /// Cancellation masks only UIE, leaving the counter and outputs unchanged.
    pub async fn wait_update(&mut self) {
        self.wait_event(WaitEvent::Update).await;
    }
    /// Wait for a break, or return an already latched break immediately. Leaves
    /// all break latches and MOE untouched. Only acknowledge_fault() may clear
    /// latches; enable_outputs() remains a separate explicit operation.
    /// Cancellation masks only BIE, never the hardware brake protection.
    pub async fn wait_break(&mut self) -> BreakFlags {
        BreakFlags(self.wait_event(WaitEvent::Break).await)
    }
}
impl Drop for AsyncThreePhasePwm<'_> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            set_event_enabled(&mut HardwareEvents, WaitEvent::Update, false);
            set_event_enabled(&mut HardwareEvents, WaitEvent::Break, false);
            UPDATE_STATE.reset();
            BREAK_STATE.reset();
        });
        // inner Drop safely disables power outputs and counter as before.
    }
}
