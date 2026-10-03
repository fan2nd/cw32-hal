//! Owned, single-input timer capture with polling or checked interrupt waits.
//!
//! A capture owns the whole timer and one physically routed input. A count is a
//! 16-bit timestamp, not elapsed time; subtraction is modulo 65536. Each request
//! arms on its first poll and gates its channel after observing the capture.
//! Multiple edges before service can overwrite the capture register. L012 and
//! F030 ATIM report this; F030 GTIM has no hardware loss indication.

use super::{
    low_level::{Config as TimerConfig, Timer},
    CoreInstance,
};
pub use super::{Ch1, Ch2, Ch3, Ch4};
use crate::{
    gpio::{AfType, Flex, Pin, Pull},
    interrupt::typelevel::{Binding, Handler, Interrupt},
    Async, Blocking, Mode, Peri,
};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin as FuturePin,
    task::{Context, Poll, Waker},
};

pub(crate) mod sealed {
    pub(crate) trait Instance {
        fn state() -> &'static crate::interrupt::EventState;
        /// Generated external mux selection, if it lives outside the timer IP.
        fn select_external_input(channel: usize);
    }
    pub trait Channel {}
    pub trait Pin<T, C> {}
}

/// An audited timer with its real capture interrupt and input selector.
#[allow(private_bounds)]
pub trait Instance: CoreInstance + sealed::Instance {
    type Interrupt: Interrupt;
}
/// A physical capture channel. Only generated pin routes make it constructible.
#[allow(private_bounds)]
pub trait CaptureChannel: sealed::Channel {
    #[doc(hidden)]
    const INDEX: usize;
}
macro_rules! channel {
    ($($name:ident = $index:literal),* $(,)?) => {$ (
        impl sealed::Channel for $name {}
        impl CaptureChannel for $name { const INDEX: usize = $index; }
    )*};
}
channel!(Ch1 = 0, Ch2 = 1, Ch3 = 2, Ch4 = 3);
macro_rules! ab_channel {
    ($($name:ident = $index:literal),* $(,)?) => {$ (
        #[doc = concat!("F030 ATIM physical capture input ", stringify!($name), ".")]
        pub enum $name {}
        channel!($name=$index);
    )*};
}
ab_channel!(Ch1A = 0, Ch1B = 1, Ch2A = 2, Ch2B = 3, Ch3A = 4, Ch3B = 5);

/// Metadata-proved external capture route, distinct from output-only routes.
/// Confirm that the board/package exposes the chosen pin.
#[allow(private_bounds)]
pub trait CapturePin<T: Instance, C: CaptureChannel>: Pin + sealed::Pin<T, C> {
    const AF: u8;
}

/// A pin owned for one timer's physical input. Creation selects digital input AF.
/// Drop disconnects the pin; no output or timer is enabled by this wrapper.
pub struct CaptureInput<'d, T: Instance, C: CaptureChannel> {
    pub(super) pin: Flex<'d>,
    _phantom: PhantomData<(T, C)>,
}
impl<'d, T: Instance, C: CaptureChannel> CaptureInput<'d, T, C> {
    pub fn from_pin<P: CapturePin<T, C>>(pin: Peri<'d, P>, pull: Pull) -> Self {
        let mut pin = Flex::new(pin);
        pin.set_as_af_unchecked(P::AF, AfType::input(pull));
        Self {
            pin,
            _phantom: PhantomData,
        }
    }
}

/// Input filter requirements expressed as physical samples, not IP encodings.
/// Unsupported selections are rejected before any timer register writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter {
    None,
    /// L012 ATIM/GTIM and F030 GTIM: two consecutive PCLK samples.
    PclkSamples2,
    /// F030 ATIM: three consecutive PCLK samples.
    PclkSamples3,
    /// L012 ATIM/GTIM and F030 GTIM: four consecutive PCLK samples.
    PclkSamples4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Rising,
    Falling,
    Both,
}

/// A capture register snapshot, taken with the channel gated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capture {
    pub count: u16,
    /// `Some(true)` means one or more earlier captures were overwritten.
    /// `None` means the IP cannot report loss (F030 GTIM), never "no loss".
    pub overcapture: Option<bool>,
}
impl Capture {
    fn encode(self) -> u32 {
        u32::from(self.count)
            | (1 << 16)
            | if self.overcapture == Some(true) {
                1 << 17
            } else {
                0
            }
            | if self.overcapture.is_some() {
                1 << 18
            } else {
                0
            }
    }
    fn decode(bits: u32) -> Option<Self> {
        (bits & (1 << 16) != 0).then_some(Self {
            count: bits as u16,
            overcapture: if bits & (1 << 18) != 0 {
                Some(bits & (1 << 17) != 0)
            } else {
                None
            },
        })
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Timer(super::Error),
    UnsupportedFilter,
    /// Finite polling allowance exhausted; not an elapsed-time deadline.
    Timeout,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    /// Physical PCLK divider; legal values are determined by the timer IP.
    pub prescaler_divisor: u32,
    pub filter: Filter,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            prescaler_divisor: 1,
            filter: Filter::None,
        }
    }
}

/// Capture IRQ dispatcher. List both handlers when GTIM3/GTIM4 share GTIM34.
/// It services only an enabled, pending capture source, never the global NVIC
/// pending state or another timer's peripheral flags.
pub struct InterruptHandler<T: Instance>(PhantomData<T>);
impl<T: Instance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|_| service_interrupt::<T>());
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}
fn service_interrupt<T: Instance>() -> Option<Waker> {
    // A statically bound handler remains installed after its owner drops. A
    // shared-vector peer may interrupt after this timer's RCC gate is released.
    if !T::clock_resource().is_enabled() {
        return None;
    }
    let r = T::regs();
    for channel in 0..r.capture_channels() {
        if r.capture_interrupt_enabled(channel) && r.capture_pending(channel) {
            // Capture remains possible on F030 ATIM even with CNT stopped.
            // Gate the source before reading status/CCR to freeze this sample.
            r.capture_interrupt(channel, false);
            r.capture_edge(channel, None);
            return T::state().latch(r.capture_read(channel).encode());
        }
    }
    None
}

/// Whole-timer owner with one capture input. No DMA or motor output is enabled.
/// The counter starts at construction; the input stays gated until a request.
/// Drop masks/gates the input, stops the counter, disconnects the pin and then
/// releases the metadata-owned RCC clock. GTIM1 remains reserved by time-driver.
pub struct InputCapture<'d, T: Instance, C: CaptureChannel, M: Mode = Blocking> {
    timer: Timer<'d, T>,
    input: CaptureInput<'d, T, C>,
    _mode: PhantomData<M>,
}
impl<'d, T: Instance, C: CaptureChannel> InputCapture<'d, T, C, Blocking> {
    pub fn new_blocking(
        timer: Peri<'d, T>,
        input: CaptureInput<'d, T, C>,
        config: Config,
    ) -> Result<Self, Error> {
        Self::new_inner(timer, input, config)
    }
    /// Arm and poll at most `poll_limit` times. A zero limit performs no arming.
    /// Timeout gates/acknowledges this input and leaves the counter running.
    pub fn blocking_capture(&mut self, edge: Edge, poll_limit: u32) -> Result<Capture, Error> {
        if poll_limit == 0 {
            return Err(Error::Timeout);
        }
        self.arm(edge);
        for _ in 0..poll_limit {
            if let Some(capture) = self.try_capture() {
                return Ok(capture);
            }
        }
        self.cancel_capture();
        Err(Error::Timeout)
    }
}
impl<'d, T: Instance, C: CaptureChannel> InputCapture<'d, T, C, Async> {
    pub fn new(
        timer: Peri<'d, T>,
        input: CaptureInput<'d, T, C>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        let this = Self::new_inner(timer, input, config)?;
        // SAFETY: exact timer handler binding supplied. Never clear/disable a
        // shared vector, which could have pending work for a live peer.
        unsafe { T::Interrupt::enable() };
        Ok(this)
    }
    /// Arm on first poll. Cancelling the future gates/masks/acknowledges only
    /// this source; the counter and any shared-vector peer continue running.
    pub async fn wait_for_edge(&mut self, edge: Edge) -> Capture {
        CaptureFuture {
            driver: self,
            edge,
            armed: false,
        }
        .await
    }
    pub async fn wait_for_rising_edge(&mut self) -> Capture {
        self.wait_for_edge(Edge::Rising).await
    }
    pub async fn wait_for_falling_edge(&mut self) -> Capture {
        self.wait_for_edge(Edge::Falling).await
    }
    /// The capture value does not retain which of the two edges occurred.
    pub async fn wait_for_any_edge(&mut self) -> Capture {
        self.wait_for_edge(Edge::Both).await
    }
}
impl<'d, T: Instance, C: CaptureChannel, M: Mode> InputCapture<'d, T, C, M> {
    fn new_inner(
        timer: Peri<'d, T>,
        input: CaptureInput<'d, T, C>,
        config: Config,
    ) -> Result<Self, Error> {
        let timing = TimerConfig {
            prescaler_divisor: config.prescaler_divisor,
            period_ticks: 65536,
        };
        timing
            .validate(T::regs().prescalers())
            .map_err(Error::Timer)?;
        let filter = T::regs()
            .capture_filter(config.filter)
            .ok_or(Error::UnsupportedFilter)?;
        let mut timer = Timer::new(timer);
        timer.set_config(timing).map_err(Error::Timer)?;
        critical_section::with(|_| {
            T::select_external_input(C::INDEX);
            T::regs().configure_capture(C::INDEX, filter);
            T::regs().capture_interrupt(C::INDEX, false);
            T::regs().capture_clear(C::INDEX);
            T::state().reset();
        });
        timer.start();
        Ok(Self {
            timer,
            input,
            _mode: PhantomData,
        })
    }
    /// Exact tick frequency, numerator/denominator hertz.
    pub fn tick_frequency(&self) -> (u32, u32) {
        (
            self.timer.frequency().clock_hz(),
            self.timer.prescaler_divisor(),
        )
    }
    pub fn counter(&self) -> u16 {
        self.timer.counter()
    }
    /// Start a polling capture, discarding any previous request/result.
    pub fn arm(&mut self, edge: Edge) {
        self.arm_inner(edge, false);
    }
    fn arm_inner(&mut self, edge: Edge, interrupt: bool) {
        critical_section::with(|_| {
            self.cancel_inner();
            T::regs().capture_interrupt(C::INDEX, interrupt);
            T::regs().capture_edge(C::INDEX, Some(edge));
        });
    }
    /// Return and disarm a completed request. Never waits for hardware.
    pub fn try_capture(&mut self) -> Option<Capture> {
        critical_section::with(|_| {
            if let Some(capture) = Capture::decode(T::state().take()) {
                return Some(capture);
            }
            let r = T::regs();
            if !r.capture_pending(C::INDEX) {
                return None;
            }
            r.capture_interrupt(C::INDEX, false);
            r.capture_edge(C::INDEX, None);
            Some(r.capture_read(C::INDEX))
        })
    }
    /// Gate/mask/acknowledge this input. Does not reset phase or shared NVIC state.
    pub fn cancel_capture(&mut self) {
        critical_section::with(|_| self.cancel_inner());
    }
    fn cancel_inner(&mut self) {
        T::regs().capture_interrupt(C::INDEX, false);
        T::regs().capture_edge(C::INDEX, None);
        T::regs().capture_clear(C::INDEX);
        T::state().reset();
    }
}
impl<T: Instance, C: CaptureChannel, M: Mode> Drop for InputCapture<'_, T, C, M> {
    fn drop(&mut self) {
        self.cancel_capture();
        self.timer.stop();
        self.input.pin.set_as_disconnected();
    }
}

struct CaptureFuture<'a, 'd, T: Instance, C: CaptureChannel> {
    driver: &'a mut InputCapture<'d, T, C, Async>,
    edge: Edge,
    armed: bool,
}
impl<T: Instance, C: CaptureChannel> Future for CaptureFuture<'_, '_, T, C> {
    type Output = Capture;
    fn poll(self: FuturePin<&mut Self>, cx: &mut Context<'_>) -> Poll<Capture> {
        let this = self.get_mut();
        if !this.armed {
            this.driver.arm_inner(this.edge, true);
            this.armed = true;
        }
        T::state().register(cx.waker());
        // Inspect after registration, closing the race with an already-serviced
        // IRQ or an edge whose vector has not yet executed.
        match this.driver.try_capture() {
            Some(capture) => {
                T::state().reset();
                Poll::Ready(capture)
            }
            None => Poll::Pending,
        }
    }
}
impl<T: Instance, C: CaptureChannel> Drop for CaptureFuture<'_, '_, T, C> {
    fn drop(&mut self) {
        if self.armed {
            self.driver.cancel_capture();
        }
    }
}
