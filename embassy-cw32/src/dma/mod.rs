//! Direct memory access with exclusive channels and interrupt-bound transfers.
//!
//! CW DMA is not STM32 DMA: BLOCK/BULK are arbitration modes, one SIZE controls
//! both addresses, REPEAT is fixed to one here, and interrupt clearing is R1W0.
//! Only a normal transfer-complete indication proves that buffers can be returned.
//! The manuals do not promise that clearing EN drains outstanding bus accesses.
//! Safe transfers therefore own **static** buffers. Cancellation, errors and
//! timeouts quarantine those buffers and permanently poison the channel. See
//! `docs/dma.md` for the deliberate cancellation and `mem::forget` contract.

use crate::{
    interrupt::{
        typelevel::{Binding, Handler, Interrupt},
        EventState,
    },
    mode::{Async, Blocking, Mode},
    Peri, PeripheralType,
};
use core::{
    cell::Cell,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use critical_section::Mutex;

mod engine;
mod hardware;
use hardware::Hardware;

/// A validated driver-private endpoint configuration. This is not a buffer
/// lease: the bus driver must retain its complete static resources separately.
pub(crate) struct PreparedEndpoint(engine::Config);

pub(crate) mod sealed {
    pub(crate) trait Instance {
        const INDEX: usize;
        fn state() -> &'static super::ChannelState;
    }
    pub(crate) trait Word {}
}

/// An exclusively owned channel identity, generated from audited DMA topology.
#[allow(private_bounds)]
pub trait Instance: sealed::Instance + PeripheralType + 'static {
    type Interrupt: Interrupt;
}

/// The three equal source/destination widths supported by these controllers.
#[allow(private_bounds)]
pub trait Word: sealed::Word + Copy + Unpin + 'static {
    #[doc(hidden)]
    const SIZE: u8;
}
macro_rules! word {
    ($ty:ty, $size:expr) => {
        impl sealed::Word for $ty {}
        impl Word for $ty {
            const SIZE: u8 = $size;
        }
    };
}
word!(u8, 0);
word!(u16, 1);
word!(u32, 2);

/// DMA arbitration mode, not a circular/streaming or buffer-layout setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransferMode {
    /// Insert an arbitration gap after each word. A hardware trigger transfers
    /// one word (REPEAT=1); a software trigger starts the entire finite copy.
    Block,
    /// One trigger transfers the entire count without arbitration gaps. May
    /// stall CPU/other channels accessing the same target until it finishes.
    Bulk,
}

/// This controller's actual trigger selection. It has no STM32-style DMAMUX.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigger {
    Software,
    Hardware(Request),
}

/// Configuration for an unsafe raw transfer. Both endpoints use the Word width.
/// Peripheral bus accesses have the peripheral's native width; SIZE determines
/// memory width and address increment. The caller must audit both endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawConfig {
    pub trigger: Trigger,
    pub mode: TransferMode,
    pub source_increment: bool,
    pub destination_increment: bool,
}
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            trigger: Trigger::Software,
            mode: TransferMode::Block,
            source_increment: true,
            destination_increment: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Empty,
    LengthMismatch,
    TooLong,
    Unaligned,
    AddressOverflow,
    ChannelBusy,
    /// Cancellation or an error left bus quiescence unproven. Only a device
    /// reset restores the safe channel; dropping/recreating owners does not.
    ChannelPoisoned,
    AddressRange,
    Aborted,
    SourceAccess,
    DestinationAccess,
    UnknownHardwareStatus(u8),
    Cancelled,
    Timeout,
}

/// Ownership is returned intact when validation rejects a transfer before start.
#[derive(Debug)]
pub struct StartError<B> {
    pub error: Error,
    pub buffers: B,
}

/// Static buffers consumed by a safe copy, returned only after clean completion.
#[derive(Debug)]
pub struct CopyBuffers<W: Word> {
    pub source: &'static [W],
    pub destination: &'static mut [W],
}

/// Exclusive source and destination consumed by [`Channel::copy_mut`].
/// Both mutable references are returned only after clean completion.
#[derive(Debug)]
pub struct MutableCopyBuffers<W: Word> {
    pub source: &'static mut [W],
    pub destination: &'static mut [W],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Idle,
    Running,
    Repeating,
    Complete,
    Failed(Error),
    Poisoned,
}

pub(crate) struct ChannelState {
    event: EventState,
    phase: Mutex<Cell<Phase>>,
    endpoint: Mutex<Cell<Option<fn(bool)>>>,
}
impl ChannelState {
    pub(crate) const fn new() -> Self {
        Self {
            event: EventState::new(),
            phase: Mutex::new(Cell::new(Phase::Idle)),
            endpoint: Mutex::new(Cell::new(None)),
        }
    }
    fn phase(&self) -> Phase {
        critical_section::with(|cs| self.phase.borrow(cs).get())
    }
    fn set_phase(&self, phase: Phase) {
        critical_section::with(|cs| self.phase.borrow(cs).set(phase));
    }
    fn finish_endpoint(&self, complete: bool) {
        // Remove the old endpoint before publishing a reusable channel. A
        // forgotten guard must never cancel or observe a later channel user.
        let endpoint = critical_section::with(|cs| self.endpoint.borrow(cs).take());
        if let Some(endpoint) = endpoint {
            endpoint(complete);
        }
    }
}

/// Preserve quarantine even if a borrowed controller token is split again after
/// channel handles were forgotten. A controller reset is not a bus-drain proof.
pub(crate) fn quarantine_for_controller_reset(state: &ChannelState) {
    critical_section::with(|_| {
        match state.phase() {
            Phase::Idle | Phase::Complete => state.set_phase(Phase::Idle),
            _ => state.set_phase(Phase::Poisoned),
        }
        state.finish_endpoint(state.phase() == Phase::Idle);
        state.event.reset();
    });
}

include!(concat!(env!("OUT_DIR"), "/_generated_dma.rs"));

/// Consume exclusive controller ownership and produce disjoint channel tokens.
/// Each constructed channel retains the shared clock; unused channel tokens
/// retain no gate. The final clean owner releases it, while quarantine pins it
/// permanently. No channel may reset the controller or disable another channel's
/// IRQ. Existing software poison is never cleared here.
pub fn split<'d>(_controller: Peri<'d, Controller>) -> Channels<'d> {
    initialize_controller();
    // SAFETY: the sole controller token covers every generated descendant;
    // returned channel lifetimes cannot outlive its exclusive borrow.
    unsafe { split_tokens() }
}

/// Exclusive DMA channel owner, analogous to Embassy's Channel/Transfer model.
/// Blocking mode does not require or enable NVIC. Async mode requires the exact
/// generated channel/IRQ binding, including every used channel on shared vectors.
pub struct Channel<'d, C: Instance, M: Mode = Blocking> {
    _clock: crate::rcc::ClockGuard,
    _token: Peri<'d, C>,
    _mode: PhantomData<M>,
}
impl<'d, C: Instance> Channel<'d, C, Blocking> {
    pub fn new_blocking(channel: Peri<'d, C>) -> Self {
        Self {
            _clock: <Controller as crate::rcc::PeripheralClock>::acquire_no_reset(),
            _token: channel,
            _mode: PhantomData,
        }
    }
}
impl<'d, C: Instance> Channel<'d, C, Async> {
    pub fn new(
        channel: Peri<'d, C>,
        _irq: impl Binding<C::Interrupt, InterruptHandler<C>> + 'd,
    ) -> Self {
        // Do not unpend or reprioritize a shared vector: a peer may be active.
        // SAFETY: Binding proves dispatch to the channel-specific handler.
        unsafe { C::Interrupt::enable() };
        Self {
            _clock: <Controller as crate::rcc::PeripheralClock>::acquire_no_reset(),
            _token: channel,
            _mode: PhantomData,
        }
    }
}

impl<'d, C: Instance, M: Mode> Channel<'d, C, M> {
    /// Validate without starting, so a two-channel peripheral can reject both
    /// endpoints before either channel can access memory.
    pub(crate) fn prepare_endpoint<W: Word>(
        &self,
        source: *const W,
        destination: *mut W,
        count: usize,
        config: RawConfig,
    ) -> Result<PreparedEndpoint, Error> {
        match C::state().phase() {
            Phase::Running | Phase::Repeating => return Err(Error::ChannelBusy),
            Phase::Failed(_) | Phase::Poisoned => return Err(Error::ChannelPoisoned),
            Phase::Idle | Phase::Complete => {}
        }
        validate(source, destination, count, config).map(PreparedEndpoint)
    }

    /// # Safety
    /// The caller owns the peripheral, request and static endpoint buffers and
    /// retains all of them until clean completion, or forever on any other exit.
    /// Retain exclusive channel ownership from validation through this call.
    /// Start inside a critical section, before peripheral request enable.
    pub(crate) unsafe fn start_endpoint(&mut self, config: PreparedEndpoint, endpoint: fn(bool)) {
        self.begin(config.0, false, Some(endpoint))
            .expect("prepared endpoint channel changed before start");
    }

    /// A completion already observed by polling or IRQ service permits reuse;
    /// a forgotten transfer with unobserved completion remains busy. Dropping
    /// the owner performs a final completion check. Abort and error poison
    /// survives all owner reconstruction and controller splits.
    pub fn is_poisoned(&self) -> bool {
        matches!(C::state().phase(), Phase::Poisoned | Phase::Failed(_))
    }

    /// Start a safe memory copy. Both slices must have equal, nonzero length at
    /// most 65535 words. Static ownership makes forgetting the future safe.
    ///
    /// On clean completion the buffers are returned. Dropping, cancelling,
    /// timing out or failing a started transfer consumes the references forever
    /// and poisons this channel; EN=0 is not treated as proof of bus quiescence.
    pub fn copy<W: Word>(
        &mut self,
        source: &'static [W],
        destination: &'static mut [W],
    ) -> Result<Transfer<'_, 'd, C, M, CopyBuffers<W>>, StartError<CopyBuffers<W>>> {
        let buffers = CopyBuffers {
            source,
            destination,
        };
        if buffers.source.len() != buffers.destination.len() {
            return Err(StartError {
                error: Error::LengthMismatch,
                buffers,
            });
        }
        let result = validate::<W>(
            buffers.source.as_ptr(),
            buffers.destination.as_ptr(),
            buffers.source.len(),
            RawConfig::default(),
        )
        .and_then(|config| self.begin(config, false, None));
        match result {
            Ok(()) => Ok(Transfer {
                _channel: self,
                buffers: Some(buffers),
                finished: false,
            }),
            Err(error) => Err(StartError { error, buffers }),
        }
    }

    /// Copy while retaining exclusive ownership of a mutable source, so it can
    /// be changed and copied again after completion. The source is not exposed
    /// while DMA reads it. Validation returns both references; cancellation,
    /// error, timeout and forgetting quarantine them exactly as for [`Self::copy`].
    pub fn copy_mut<W: Word>(
        &mut self,
        source: &'static mut [W],
        destination: &'static mut [W],
    ) -> Result<Transfer<'_, 'd, C, M, MutableCopyBuffers<W>>, StartError<MutableCopyBuffers<W>>>
    {
        let buffers = MutableCopyBuffers {
            source,
            destination,
        };
        if buffers.source.len() != buffers.destination.len() {
            return Err(StartError {
                error: Error::LengthMismatch,
                buffers,
            });
        }
        let result = validate::<W>(
            buffers.source.as_ptr(),
            buffers.destination.as_ptr(),
            buffers.source.len(),
            RawConfig::default(),
        )
        .and_then(|config| self.begin(config, false, None));
        match result {
            Ok(()) => Ok(Transfer {
                _channel: self,
                buffers: Some(buffers),
                finished: false,
            }),
            Err(error) => Err(StartError { error, buffers }),
        }
    }

    /// Audited driver-only endpoint: the terminal hook owns the peripheral's
    /// persistent lease and must retain it on every unproven stop path.
    ///
    /// # Safety
    /// The source/request/mode and native access width must match that endpoint.
    /// Retain its static resources through errors, cancellation and forgetting.
    /// The hook must disarm it, mark clean completion or permanent poison, and
    /// never recursively cancel this channel. All setup occurs in one critical
    /// section before enabling the peripheral request generator.
    pub(crate) unsafe fn read_peripheral<W: Word>(
        &mut self,
        source: *const W,
        destination: &'static mut [W],
        config: RawConfig,
        endpoint: fn(bool),
    ) -> Result<Transfer<'_, 'd, C, M, &'static mut [W]>, StartError<&'static mut [W]>> {
        let result = validate::<W>(source, destination.as_ptr(), destination.len(), config)
            .and_then(|config| self.begin(config, false, Some(endpoint)));
        match result {
            Ok(()) => Ok(Transfer {
                _channel: self,
                buffers: Some(destination),
                finished: false,
            }),
            Err(error) => Err(StartError {
                error,
                buffers: destination,
            }),
        }
    }

    /// Start a finite raw transfer. This is an unsafe endpoint interface, not an
    /// audited peripheral driver. Hardware BLOCK consumes one word per request;
    /// BULK consumes the complete count per request. REPEAT is always one.
    ///
    /// # Safety
    /// The addresses, width, alignment, request and side effects must be valid
    /// for the actual peripheral/memory and compatible with all other owners.
    /// DMA/FLASH-controller/RAM-controller registers are forbidden endpoints.
    /// The source must be readable and destination writable over the entire
    /// incremented range. Keep source memory immutable while DMA reads it. No
    /// normal references or nonvolatile CPU accesses may observe a destination
    /// while DMA writes it. Concurrent raw volatile CPU observations require a
    /// caller-audited, target-specific protocol for aligned native-width loads;
    /// they must not write the destination or conflict with other DMA writers.
    /// Volatile alone does not establish that protocol or a coherent snapshot.
    /// Keep memory allocated and endpoint ownership valid until clean TC proves
    /// completion, **even if this transfer is dropped or forgotten**. On timeout,
    /// cancellation or error, disabling EN does not establish quiescence: retain
    /// the endpoints for the device lifetime or establish quiescence externally.
    /// A typed raw pointer alone does not satisfy any of these requirements.
    pub unsafe fn transfer_raw<W: Word>(
        &mut self,
        source: *const W,
        destination: *mut W,
        count: usize,
        config: RawConfig,
    ) -> Result<Transfer<'_, 'd, C, M>, Error> {
        let config = validate::<W>(source, destination, count, config)?;
        self.begin(config, false, None)?;
        Ok(Transfer {
            _channel: self,
            buffers: Some(()),
            finished: false,
        })
    }

    /// Start L012 hardware auto-retransmission with initial-address reload at
    /// each full transfer. Per-word SRCLOAD/DSTLOAD are kept disabled. This is
    /// not an Embassy circular buffer: no producer index, coherent snapshot,
    /// overrun detection or safe concurrent CPU access is provided.
    ///
    /// # Safety
    /// All `transfer_raw` obligations apply indefinitely. In particular, neither
    /// a TC flag from one iteration nor Drop proves quiescence. Buffers and
    /// peripheral ownership must survive until independently proven quiescent.
    #[cfg(dmachannel_l012)]
    pub unsafe fn start_repeating_raw<W: Word>(
        &mut self,
        source: *const W,
        destination: *mut W,
        count: usize,
        config: RawConfig,
    ) -> Result<RepeatingTransfer<'_, 'd, C, M>, Error> {
        let config = validate::<W>(source, destination, count, config)?;
        self.begin(config, true, None)?;
        Ok(RepeatingTransfer { _channel: self })
    }

    fn begin(
        &mut self,
        config: engine::Config,
        repeating: bool,
        endpoint: Option<fn(bool)>,
    ) -> Result<(), Error> {
        critical_section::with(|_| {
            let state = C::state();
            match state.phase() {
                Phase::Running | Phase::Repeating => return Err(Error::ChannelBusy),
                Phase::Failed(_) | Phase::Poisoned => return Err(Error::ChannelPoisoned),
                Phase::Idle | Phase::Complete => {}
            }
            state.event.reset();
            critical_section::with(|cs| state.endpoint.borrow(cs).set(endpoint));
            state.set_phase(if repeating {
                Phase::Repeating
            } else {
                Phase::Running
            });
            engine::start(
                &mut Hardware(C::INDEX),
                config,
                M::ASYNC && !repeating,
                repeating,
            );
            Ok(())
        })
    }
}

impl<C: Instance, M: Mode> Drop for Channel<'_, C, M> {
    fn drop(&mut self) {
        cancel_channel::<C>();
        if C::state().phase() != Phase::Idle && C::state().phase() != Phase::Complete {
            self._clock.pin();
        }
    }
}

fn validate<W: Word>(
    source: *const W,
    destination: *const W,
    count: usize,
    config: RawConfig,
) -> Result<engine::Config, Error> {
    engine::validate(
        source as usize,
        destination as usize,
        count,
        W::SIZE,
        config,
    )
}

/// Shared-vector handler. It services only this channel's enabled TC/TE flags,
/// publishes under the same critical section as cleanup, and wakes outside it.
pub struct InterruptHandler<C: Instance>(PhantomData<C>);
impl<C: Instance> Handler<C::Interrupt> for InterruptHandler<C> {
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|_| service::<C>(true));
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}

fn service<C: Instance>(from_irq: bool) -> Option<core::task::Waker> {
    let state = C::state();
    if state.phase() != Phase::Running {
        return None;
    }
    let Some(result) = engine::complete(&mut Hardware(C::INDEX), from_irq) else {
        return None;
    };
    state.finish_endpoint(result.is_ok());
    state.set_phase(match result {
        Ok(()) => Phase::Complete,
        Err(error) => Phase::Failed(error),
    });
    state.event.latch(1)
}

pub(crate) fn completion<C: Instance>() -> Option<Result<(), Error>> {
    let (result, waker) = critical_section::with(|_| {
        let waker = service::<C>(false);
        let result = match C::state().phase() {
            Phase::Complete => Some(Ok(())),
            Phase::Failed(error) => Some(Err(error)),
            Phase::Poisoned => Some(Err(Error::ChannelPoisoned)),
            _ => None,
        };
        (result, waker)
    });
    if let Some(waker) = waker {
        waker.wake();
    }
    result
}

/// Register before inspecting hardware, using the same state as DMA IRQ service.
pub(crate) fn register_endpoint<C: Instance>(waker: &core::task::Waker) {
    C::state().event.register(waker);
}

/// A peripheral error or undrained wire poisons the complete operation, even
/// if one channel independently reached TC. Never turn partial success into a
/// reusable channel or release one side of a two-channel bus transaction.
pub(crate) fn poison_endpoint<C: Instance>() {
    critical_section::with(|_| {
        engine::disable(&mut Hardware(C::INDEX));
        C::state().finish_endpoint(false);
        C::state().set_phase(Phase::Poisoned);
        C::state().event.reset();
    });
}

pub(crate) fn cancel_channel<C: Instance>() {
    let waker = critical_section::with(|_| {
        let state = C::state();
        // Observe a genuine terminal flag before deciding whether reclamation
        // is possible. An IRQ may have been masked or not yet dispatched.
        let waker = service::<C>(false);
        match state.phase() {
            Phase::Running | Phase::Repeating | Phase::Failed(_) | Phase::Poisoned => {
                engine::disable(&mut Hardware(C::INDEX));
                state.finish_endpoint(false);
                state.set_phase(Phase::Poisoned);
            }
            Phase::Complete | Phase::Idle => {}
        }
        state.event.reset();
        waker
    });
    drop(waker);
}

/// An in-flight transfer, owning safe buffers or carrying unsafe raw obligations.
/// Dropping it requests channel disable. Only clean completion returns buffers;
/// other exits quarantine them and poison the channel. `mem::forget` can leak
/// static buffers and leave the channel busy, but cannot release their memory.
#[must_use = "dropping a started transfer cancels and quarantines its buffers"]
pub struct Transfer<'a, 'd, C: Instance, M: Mode, B = ()> {
    _channel: &'a mut Channel<'d, C, M>,
    buffers: Option<B>,
    finished: bool,
}
impl<C: Instance, M: Mode, B> Transfer<'_, '_, C, M, B> {
    /// Bound the wait loop to `poll_budget` checks, followed by one final
    /// completion check during cancellation. A timeout disables and poisons the
    /// channel and returns no buffers. This is not a bus-drain deadline.
    pub fn blocking_wait(mut self, poll_budget: u32) -> Result<B, Error> {
        for _ in 0..poll_budget {
            if let Some(result) = completion::<C>() {
                return self.finish(result);
            }
            core::hint::spin_loop();
        }
        cancel_channel::<C>();
        // Cancellation races with a valid completion; reclaim only if that
        // completion was actually observed, never based merely on EN=0.
        let result = if C::state().phase() == Phase::Complete {
            Ok(())
        } else {
            Err(Error::Timeout)
        };
        self.finish(result)
    }
    /// Request cancellation. Buffers are returned only if clean TC had already
    /// occurred; otherwise Err(Cancelled) consumes them and poisons the channel.
    pub fn cancel(mut self) -> Result<B, Error> {
        cancel_channel::<C>();
        let result = if C::state().phase() == Phase::Complete {
            Ok(())
        } else {
            Err(Error::Cancelled)
        };
        self.finish(result)
    }
    fn finish(&mut self, result: Result<(), Error>) -> Result<B, Error> {
        if result.is_err() {
            cancel_channel::<C>();
        }
        self.finished = true;
        let buffers = self
            .buffers
            .take()
            .expect("completed DMA transfer polled again");
        // On error dropping static references leaks ownership; it does not free
        // memory. No safe API can recreate the consumed exclusive reference.
        result.map(|_| buffers)
    }
}
impl<C: Instance, M: Mode, B> Drop for Transfer<'_, '_, C, M, B> {
    fn drop(&mut self) {
        if !self.finished {
            cancel_channel::<C>();
        }
    }
}
impl<C: Instance, B: Unpin> Future for Transfer<'_, '_, C, Async, B> {
    type Output = Result<B, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        assert!(!this.finished, "completed DMA transfer polled again");
        C::state().event.register(cx.waker());
        match completion::<C>() {
            Some(result) => Poll::Ready(this.finish(result)),
            None => Poll::Pending,
        }
    }
}

/// Unsafe L012 repeating engine guard. Drop disables and poisons the channel;
/// it does not return buffers or claim that outstanding bus accesses drained.
#[cfg(dmachannel_l012)]
#[must_use = "dropping the repeating transfer requests disable and poisons its channel"]
pub struct RepeatingTransfer<'a, 'd, C: Instance, M: Mode> {
    _channel: &'a mut Channel<'d, C, M>,
}
#[cfg(dmachannel_l012)]
impl<C: Instance, M: Mode> RepeatingTransfer<'_, '_, C, M> {
    /// Inspect an error without claiming an atomic memory snapshot or clearing
    /// it. Successful iterations do not terminate a repeating transfer.
    pub fn error(&self) -> Option<Error> {
        engine::error(&mut Hardware(C::INDEX))
    }
    /// Explicit spelling of Drop; endpoint lifetime obligations remain in force.
    pub fn request_stop(self) {
        drop(self);
    }
}
#[cfg(dmachannel_l012)]
impl<C: Instance, M: Mode> Drop for RepeatingTransfer<'_, '_, C, M> {
    fn drop(&mut self) {
        cancel_channel::<C>();
    }
}
