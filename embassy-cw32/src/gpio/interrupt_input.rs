//! Per-pin interrupt-backed GPIO inputs. CW32 has one IRQ per GPIO port.
use core::{convert::Infallible, marker::PhantomData, task::Poll};

use crate::{
    async_support::EventState,
    interrupt::typelevel::{Binding, Handler, Interrupt},
    pac, Peri,
};

use super::{Input, Level, Pin, Port, Pull};

pub(crate) mod sealed {
    pub(crate) trait InterruptPin {
        const PORT: super::Port;
        const NUMBER: u8;
        fn state() -> &'static crate::async_support::EventState;
    }
}

/// A concrete pin with its generated port IRQ and private event state.
///
/// This sealed trait is implemented only for concrete singleton pin tokens.
/// Erased [`super::AnyPin`] identities cannot prove an interrupt binding.
#[allow(private_bounds)]
pub trait InterruptPin: Pin + sealed::InterruptPin {
    type Interrupt: Interrupt;
}

/// Service one pin on a shared GPIO port interrupt.
///
/// List every active pin handler on that port in the same `bind_interrupts!`
/// declaration, for example `GPIOA => InterruptHandler<PA0>,
/// InterruptHandler<PA1>;`. Each handler services only its enabled pending bit.
pub struct InterruptHandler<P: InterruptPin>(PhantomData<P>);
impl<P: InterruptPin> Handler<P::Interrupt> for InterruptHandler<P> {
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|_| {
            service_interrupt(&mut GpioHardware::for_pin::<P>(), P::state())
        });
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

/// An owned input with a checked pin-specific interrupt binding.
///
/// Unlike STM32 EXTI, distinct pins with the same number on different ports do
/// not share a line token. Pins on the same CW32 port share an NVIC vector; bind
/// all active pin handlers to that vector together.
///
/// Waits arm on their first poll. Hardware coalesces events, so these waits are
/// notifications, not an edge counter, and do not retain the direction of an
/// any-edge event. An edge before arming is discarded. The level may change
/// again before the awaiting task resumes. GPIO interrupt filtering is preserved
/// as configured; this driver does not configure debounce or a filter clock.
///
/// Cancelling a wait masks and clears only this pin before releasing the pin
/// borrow. Dropping the driver also cleans its interrupt state, then disconnects
/// the owned input. Neither action disables/unpends the shared NVIC vector or
/// switches off the shared port clock.
pub struct InterruptInput<'d> {
    pin: Input<'d>,
    hardware: GpioHardware,
    state: &'static EventState,
}
impl<'d> InterruptInput<'d> {
    /// Acquire an input and enable its port vector using the binding proof.
    ///
    /// The proof must bind this exact concrete pin's [`InterruptHandler`].
    /// Unsupported pulls panic before MMIO, as with [`Input::new`].
    pub fn new<P: InterruptPin>(
        pin: Peri<'d, P>,
        _irq: impl Binding<P::Interrupt, InterruptHandler<P>>,
        pull: Pull,
    ) -> Self {
        let pin = Input::new(pin, pull);
        let mut hardware = GpioHardware::for_pin::<P>();
        let state = P::state();
        critical_section::with(|_| {
            hardware.disable();
            hardware.clear();
            state.reset();
            // SAFETY: the supplied Binding proves this exact pin's handler is
            // installed. Leave peer sources and shared NVIC pending untouched.
            unsafe { P::Interrupt::enable() };
        });
        Self {
            pin,
            hardware,
            state,
        }
    }

    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }
    pub fn level(&self) -> Level {
        self.pin.level()
    }

    /// Complete immediately if high, otherwise wait for a high event.
    /// F030 uses HIGHIE; L012 arms the rising edge and rechecks the input level.
    pub async fn wait_for_high(&mut self) {
        self.wait(WaitKind::High).await;
    }
    /// Complete immediately if low, otherwise wait for a low event.
    /// F030 uses LOWIE; L012 arms the falling edge and rechecks the input level.
    pub async fn wait_for_low(&mut self) {
        self.wait(WaitKind::Low).await;
    }
    /// Wait for a new rising edge, even if the input is already high.
    pub async fn wait_for_rising_edge(&mut self) {
        self.wait(WaitKind::Rising).await;
    }
    /// Wait for a new falling edge, even if the input is already low.
    pub async fn wait_for_falling_edge(&mut self) {
        self.wait(WaitKind::Falling).await;
    }
    /// Wait for either edge; the GPIO flag does not retain its direction.
    pub async fn wait_for_any_edge(&mut self) {
        self.wait(WaitKind::AnyEdge).await;
    }
    async fn wait(&mut self, kind: WaitKind) {
        InputWait {
            input: self,
            core: WaitCore::new(kind),
        }
        .await;
    }
}
impl Drop for InterruptInput<'_> {
    fn drop(&mut self) {
        // Also covers a forgotten future whose borrow has been released without
        // running its Drop. The Input/Flex field disconnects the pin afterward.
        critical_section::with(|_| {
            self.hardware.disable();
            self.hardware.clear();
            self.state.reset();
        });
    }
}

impl embedded_hal::digital::ErrorType for InterruptInput<'_> {
    type Error = Infallible;
}
impl embedded_hal::digital::InputPin for InterruptInput<'_> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(InterruptInput::is_high(self))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(InterruptInput::is_low(self))
    }
}
impl embedded_hal_async::digital::Wait for InterruptInput<'_> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        InterruptInput::wait_for_high(self).await;
        Ok(())
    }
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        InterruptInput::wait_for_low(self).await;
        Ok(())
    }
    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        InterruptInput::wait_for_rising_edge(self).await;
        Ok(())
    }
    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        InterruptInput::wait_for_falling_edge(self).await;
        Ok(())
    }
    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        InterruptInput::wait_for_any_edge(self).await;
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum WaitKind {
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

trait GpioIo {
    fn disable(&mut self);
    fn clear(&mut self);
    fn arm(&mut self, kind: WaitKind);
    fn pending(&mut self) -> bool;
    fn high(&mut self) -> bool;
}
struct GpioHardware {
    port: Port,
    number: u8,
}
impl GpioHardware {
    fn for_pin<P: InterruptPin>() -> Self {
        Self {
            port: P::PORT,
            number: P::NUMBER,
        }
    }
    fn regs(&self) -> pac::gpio::Gpio {
        // SAFETY: generated sealed pin metadata supplies a real GPIO address.
        unsafe { pac::gpio::Gpio::from_ptr(self.port.base() as *mut ()) }
    }
}
impl GpioIo for GpioHardware {
    fn disable(&mut self) {
        let r = self.regs();
        let n = usize::from(self.number);
        r.riseie().modify(|w| w.set_pin(n, false));
        r.fallie().modify(|w| w.set_pin(n, false));
        #[cfg(gpio_f030)]
        {
            r.highie().modify(|w| w.set_pin(n, false));
            r.lowie().modify(|w| w.set_pin(n, false));
        }
    }
    fn clear(&mut self) {
        // ICR is R1W0. Ones preserve every other implemented pin's pending flag;
        // zero clears ours. Reserved/unimplemented bits retain their zero value.
        // Do not RMW ICR or write an unrestricted !mask into its reserved bits.
        let preserve = u32::from(self.port.implemented_mask()) & !(1u32 << self.number);
        self.regs()
            .icr()
            .write_value(pac::gpio::regs::Icr(preserve));
    }
    fn arm(&mut self, kind: WaitKind) {
        let r = self.regs();
        let n = usize::from(self.number);
        match kind {
            WaitKind::Rising => r.riseie().modify(|w| w.set_pin(n, true)),
            WaitKind::Falling => r.fallie().modify(|w| w.set_pin(n, true)),
            WaitKind::AnyEdge => {
                r.riseie().modify(|w| w.set_pin(n, true));
                r.fallie().modify(|w| w.set_pin(n, true));
            }
            WaitKind::High => {
                #[cfg(gpio_f030)]
                r.highie().modify(|w| w.set_pin(n, true));
                #[cfg(gpio_l012)]
                r.riseie().modify(|w| w.set_pin(n, true));
            }
            WaitKind::Low => {
                #[cfg(gpio_f030)]
                r.lowie().modify(|w| w.set_pin(n, true));
                #[cfg(gpio_l012)]
                r.fallie().modify(|w| w.set_pin(n, true));
            }
        }
    }
    fn pending(&mut self) -> bool {
        let r = self.regs();
        let n = usize::from(self.number);
        let enabled = r.riseie().read().pin(n) || r.fallie().read().pin(n);
        #[cfg(gpio_f030)]
        let enabled = enabled || r.highie().read().pin(n) || r.lowie().read().pin(n);
        enabled && r.isr().read().pin(n)
    }
    fn high(&mut self) -> bool {
        self.regs().idr().read().pin(usize::from(self.number))
    }
}

fn service_interrupt(io: &mut impl GpioIo, state: &EventState) -> Option<core::task::Waker> {
    if io.pending() {
        // Mask before clearing: a still-active F030 level must not reassert.
        // Publish while hardware servicing is in the same critical section, so
        // cancellation/rearm cannot receive a completion from the old wait.
        io.disable();
        io.clear();
        state.latch(1)
    } else {
        None
    }
}

struct WaitCore {
    armed: bool,
    done: bool,
    kind: WaitKind,
}
impl WaitCore {
    fn new(kind: WaitKind) -> Self {
        Self {
            armed: false,
            done: false,
            kind,
        }
    }
    fn poll(
        &mut self,
        io: &mut impl GpioIo,
        state: &EventState,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<()> {
        assert!(!self.done, "completed GPIO future polled again");
        critical_section::with(|_| {
            if !self.armed {
                io.disable();
                io.clear();
                // Reset precedes registration, including after a forgotten
                // previous wait. Never erase the newly registered waker.
                state.reset();
                state.register(cx.waker());
                io.arm(self.kind);
                self.armed = true;
            } else {
                state.register(cx.waker());
            }
            // Registration precedes every latch read; the post-arm hardware
            // and level checks close the check/enable window. A later event
            // remains pending and will be serviced once this section exits.
            let signaled = state.take() != 0;
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
    fn cancel(&mut self, io: &mut impl GpioIo, state: &EventState) {
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
struct InputWait<'a, 'd> {
    input: &'a mut InterruptInput<'d>,
    core: WaitCore,
}
impl core::future::Future for InputWait<'_, '_> {
    type Output = ();
    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<()> {
        let this = self.get_mut();
        this.core
            .poll(&mut this.input.hardware, this.input.state, cx)
    }
}
impl Drop for InputWait<'_, '_> {
    fn drop(&mut self) {
        self.core.cancel(&mut self.input.hardware, self.input.state);
    }
}
