//! One-shot, interrupt-driven DMA for CW32F030/CW32A030, CW32L083, CW32F020,
//! CW32L031, CW32L052, CW32R031 and CW32L012.
//!
//! The implementation follows CW32x030 User Manual EN V1.0 §§8.4–8.8
//! (also checked against CN V2.5), CW32L083 CN V2.0 §§8.4–8.8,
//! CW32F020 CN V1.4 §§8.4–8.8 (native two-channel controller), and each native
//! four-channel family's own manual: L031 CN V1.6 pp119–134, L052 CN V1.5
//! pp124–139, R031 CN V1.3 pp121–136 and W031 CN V1.4 pp120–135. Native L012
//! follows CN V1.4 §§8.1–8.8 / EN V1.0 §§8.1–8.8 and the UART TX example in
//! CN §21.7.1.5 (printed 474) / EN §21.7.1.5 (printed 520).
//! L083 also exposes safe staged UART/SPI requests; unsafe borrowed hardware
//! constructors remain x030-only. F020/L031/L052/R031 add safe owned copy
//! and staged UART TX/RX; L012 adds owned copy, staged UART TX and qualified
//! native SPI DR TX/RX. L012 UART RX remains unqualified.
//! Unsafe software-copy APIs retain their existing contracts.
//! L031/L052/R031 share DMACH23 between CH2/3 and use independent DMACH4
//! for CH4. L012 pairs CH1/2 on DMACH12 and CH3/4 on DMACH34; IRQ11 is CORDIC.
//! Only the selected channel's terminal flags are cleared.
//! It uses BLOCK transfers with REPEAT=1,
//! equal source/destination word widths and the channel's fixed priority.
//! L012 explicitly disables RESTART, SRCLOAD and DSTLOAD for each one-shot
//! launch. There is no circular mode, programmable priority, packing or burst API.
//!
//! # Cancellation limitation
//!
//! These manuals, including L012, do not document an acknowledgment that clearing
//! CSR.EN has drained an in-flight bus access. This driver therefore does **not** treat EN readback
//! as a cancellation guarantee. Dropping a transfer waits for its completion or
//! transfer-error indication before disabling the channel and releasing its
//! buffer borrow. A hardware-request transfer may block forever in Drop if its
//! peripheral stops supplying requests. There is deliberately no early-abort
//! method claiming otherwise. The staged UART/SPI wrappers instead own private
//! static staging and retain it across cancellation; see [`crate::usart`] and
//! [`crate::spi`].
//!
//! [`OwnedCopy`] owns static SRAM buffers and returns them only after clean
//! software-copy completion; errors permanently consume its resources. Its
//! safe entry is [`CopyChannel::copy`], using a capability admitted once during
//! normal HAL initialization. [`OwnedCopy::new`] remains the unsafe raw entry.
//! Direct PAC access is a low-level ownership escape; see [`crate::pac`].
//! The borrowed [`Transfer`] constructors
//! are also unsafe and additionally require retaining forgotten borrows. Awaiting
//! uses the real DMA interrupt; only explicit blocking waits and Drop poll.

use core::cell::RefCell;
use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::sync::atomic::{Ordering, fence};
use core::task::{Context, Poll, Waker};

use critical_section::Mutex;

use crate::interrupt::typelevel::{Binding, Handler, Interrupt};
use crate::{Peri, PeripheralType, pac};

/// An unshifted, verified hardware request selector.
///
/// Named constants are generated from the selected chip's request metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Request(pub(crate) u8);

impl Request {
    /// Raw HARDSRC value, before the register's left shift by two.
    pub const fn number(self) -> u8 {
        self.0
    }
}

/// Type-level request signals, using the verified metadata's spelling.
#[allow(non_camel_case_types)]
pub mod signal {
    /// Receive data request.
    pub enum RX {}
    /// Transmit data request.
    pub enum TX {}
    /// ADC conversion complete request.
    pub enum COMPLETE {}
    /// Timer update request.
    pub enum UP {}
    /// Timer trigger request.
    pub enum TRIG {}
    /// Timer channel 1 request.
    pub enum CH1 {}
    /// Timer channel 2 request.
    pub enum CH2 {}
    /// Timer channel 3 request.
    pub enum CH3 {}
    /// Timer channel 4 request.
    pub enum CH4 {}
    /// ATIM grouped A-channel and update request.
    pub enum CH1A2A3A4_UP {}
    /// ATIM grouped B-channel and update request.
    pub enum CH1B2B3B_UP {}
}

pub(crate) mod sealed {
    pub trait Channel {
        const INDEX: u8;
    }
    pub trait Route<P, S> {}
    pub trait Word {
        const SIZE: crate::pac::dmachannel::vals::Width;
    }
}

/// A singleton for one physical DMA channel.
#[allow(private_bounds)]
pub trait ChannelInstance: sealed::Channel + PeripheralType + 'static {
    /// Interrupt vector containing this channel.
    type Interrupt: Interrupt;
}

/// Verified request route from peripheral `P`, signal `S`, to this channel.
///
/// Qualified x030/L083 requests apply to their five channels; qualified F020
/// UART TX/RX requests apply to its two physical channels. Qualified
/// L031/L052/R031 UART TX/RX and L012 UART TX plus qualified SPI1/2/3
/// DR TX/RX requests apply to their four physical channels.
/// This trait is sealed so downstream code cannot introduce an unverified
/// request or channel route.
#[allow(private_bounds)]
pub trait RequestRoute<P, S>: ChannelInstance + sealed::Route<P, S> {
    /// Hardware trigger selector.
    const REQUEST: Request;
}

macro_rules! impl_channel {
    ($name:ident, $index:expr, $irq:ident) => {
        impl $crate::dma::sealed::Channel for $crate::peripherals::$name {
            const INDEX: u8 = $index;
        }
        impl $crate::dma::ChannelInstance for $crate::peripherals::$name {
            type Interrupt = $crate::interrupt::typelevel::$irq;
        }
    };
}
pub(crate) use impl_channel;

#[cfg(any(uart_dma_tx, uart_dma_rx, spi_dma))]
macro_rules! impl_request {
    ($name:ident, $peripheral:ident, $signal:ident, $number:expr) => {
        impl $crate::dma::Request {
            #[doc = concat!(stringify!($peripheral), " ", stringify!($signal), " request.")]
            pub const $name: Self = Self($number);
        }
        impl<C: $crate::dma::ChannelInstance>
            $crate::dma::sealed::Route<
                $crate::peripherals::$peripheral,
                $crate::dma::signal::$signal,
            > for C
        {
        }
        impl<C: $crate::dma::ChannelInstance>
            $crate::dma::RequestRoute<
                $crate::peripherals::$peripheral,
                $crate::dma::signal::$signal,
            > for C
        {
            const REQUEST: $crate::dma::Request = $crate::dma::Request::$name;
        }
    };
}
#[cfg(any(uart_dma_tx, uart_dma_rx, spi_dma))]
pub(crate) use impl_request;

/// A supported DMA word. Only `u8`, `u16` and `u32` implement this trait.
///
#[allow(private_bounds)]
pub trait Word: sealed::Word + Copy + 'static {}
macro_rules! impl_word {
    ($($ty:ty => $size:expr),+) => { $(
        impl sealed::Word for $ty { const SIZE: pac::dmachannel::vals::Width = $size; }
        impl Word for $ty {}
    )+ };
}
impl_word!(u8 => pac::dmachannel::vals::Width::Byte, u16 => pac::dmachannel::vals::Width::HalfWord, u32 => pac::dmachannel::vals::Width::Word);

/// Transfer setup failure; no transfer is started on these errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// Transfers must contain 1 through 65535 words.
    InvalidLength,
    /// Memory-to-memory source and destination lengths differ.
    LengthMismatch,
    /// Source or destination is null, exceeds 32 bits, or would wrap.
    AddressOutOfRange,
    /// Source or destination is not aligned for the selected word width.
    Misaligned,
    /// An earlier transfer still owns the channel (including a leaked future).
    Busy,
    /// The selected profile lacks a verified SRAM extent for owned copies.
    MissingMemoryMetadata,
    /// An owned-copy buffer is outside the selected part's SRAM.
    NotInSram,
    /// The owned-copy buffers overlap.
    OverlappingBuffers,
}

/// Reported DMA transfer error, decoded from CSR.STATUS.
///
/// An error flag alone does not establish that bus accesses have drained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// DMA reported an address outside its addressing range.
    AddressOutOfRange,
    /// A peripheral stop request aborted the transfer.
    Stopped,
    /// Source bus access failed.
    Source,
    /// Destination bus access failed.
    Destination,
    /// The error flag was set with an unexpected STATUS value.
    Unknown(u8),
    /// TC was observed but STATUS or software-start readback did not confirm completion.
    UnconfirmedCompletion,
}

/// Owned DMA channel handle, following Embassy's type-erased channel API.
pub struct Channel<'d> {
    index: u8,
    _borrow: PhantomData<&'d mut ()>,
}

impl<'d> Channel<'d> {
    /// Claim a channel, enable its shared clock and its correctly bound vector.
    ///
    /// Shared vectors require all used channel handlers in `bind_interrupts!`:
    /// `DMACH23 => dma::InterruptHandler<DMA_CH2>, dma::InterruptHandler<DMA_CH3>;`
    /// On L012, bind CH1/2 together on `DMACH12` and CH3/4 on `DMACH34` when
    /// both siblings are used.
    /// This is the raw integration handle: acquiring it permanently revokes
    /// safe-copy admission for this channel. Use [`CopyChannel::new`] for safe
    /// owned SRAM copies. All transfer constructors on this raw path are unsafe.
    /// Low-level runtime integration must already own and silence inherited
    /// interrupt sources before acquiring this handle: it unmasks the vector,
    /// but the HAL does not acknowledge flags of a transfer it did not start.
    /// Creating or dropping a channel never resets the DMA controller, disables
    /// its shared clock, or clears another channel's pending interrupt.
    pub fn new<T: ChannelInstance>(
        _channel: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
    ) -> Self {
        use crate::rcc::{Readback, SealedRccPeripheral};
        let rcc = crate::peripherals::DMA::RCC_INFO;
        critical_section::with(|cs| {
            // Raw acquisition irrevocably revokes safe-copy admission. In
            // particular, raw Transfer::drop returning to Idle cannot restore it.
            STATES[T::INDEX as usize]
                .inner
                .borrow(cs)
                .borrow_mut()
                .admission = Admission::Raw;
            rcc.enable_with_cs_readback(cs, Readback::None)
                .expect("unpolled DMA gate write cannot fail");
        });
        let _ = rcc.is_enabled();
        unsafe { T::Interrupt::enable() };
        Self {
            index: T::INDEX,
            _borrow: PhantomData,
        }
    }

    /// Reborrow without creating another owner of the physical channel.
    pub fn reborrow(&mut self) -> Channel<'_> {
        Channel {
            index: self.index,
            _borrow: PhantomData,
        }
    }

    /// Zero-based channel index; lower indices have higher hardware priority.
    pub fn number(&self) -> u8 {
        self.index
    }
}

/// Why normal HAL startup could not admit a safe owned-copy channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CopyChannelError {
    /// Normal HAL initialization has not admitted the controller.
    NotInitialized,
    /// The selected generic profile has no qualified SRAM extent.
    MissingMemoryMetadata,
    /// DMA was already clocked, held in reset, or had non-default registers/flags.
    InheritedState,
    /// The shared clock enable did not read back within the initialization budget.
    ClockNotReady,
    /// This channel's one-shot safe capability has already been claimed.
    AlreadyClaimed,
    /// A low-level channel acquisition or downgrade permanently revoked admission.
    RawTakeover,
}

/// Rejected safe-channel acquisition, retaining the unused physical singleton.
pub struct CopyChannelConfigError<T: ChannelInstance> {
    /// Reason admission failed. No channel registers or interrupt flags changed.
    pub error: CopyChannelError,
    /// Unused singleton; it may still be passed to the unsafe low-level path.
    pub channel: Peri<'static, T>,
}

/// A one-shot, HAL-admitted capability for safe static SRAM copies.
///
/// Obtain it from a physical singleton returned by normal [`crate::init`].
/// Initialization admits only a reset/clean-runtime controller with its clock
/// initially off, reset released, qualified SRAM and default DMA registers.
/// Dirty state is rejected, never reset or cleared to manufacture quiescence.
/// The register check occurs after clock enable and leaves the gate enabled.
/// Clean runtime entry excludes armed/gated transfers and pending requests that
/// could resume when the gate opens, as well as outstanding bus accesses.
/// These checks diagnose unsupported handovers; they do not authenticate an
/// arbitrary bootloader's outstanding bus accesses. See [`crate::init`].
///
/// There is no conversion from a raw [`Channel`] and no reborrow or token-release
/// API. Clean completion returns this same capability; error and forget retain
/// it permanently. Shared DMA clock/reset control remains with the HAL.
pub struct CopyChannel {
    channel: Channel<'static>,
}

impl CopyChannel {
    /// Consume an unused physical singleton and bind its real DMA interrupt.
    ///
    /// This uses admission recorded by normal HAL initialization, never a fresh
    /// EN/STATUS snapshot. Rejection returns the singleton without touching DMA
    /// registers, flags, its clock/reset, or the interrupt enable state.
    /// Bind every used sibling on a shared vector; L012 pairs CH1/2 on
    /// `DMACH12` and CH3/4 on `DMACH34`.
    pub fn new<T: ChannelInstance>(
        channel: Peri<'static, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'static,
    ) -> Result<Self, CopyChannelConfigError<T>> {
        Channel::new_admitted(channel, _irq).map(|channel| Self { channel })
    }

    /// Zero-based physical channel index.
    pub fn number(&self) -> u8 {
        self.channel.index
    }

    /// Start a safe software copy, moving both exclusive static SRAM buffers.
    ///
    /// Invalid configuration returns every owner before DMA starts. Success
    /// returns them after documented clean completion. Error, forget and Drop
    /// never expose buffers that hardware might still access. Drop can block
    /// indefinitely; this is not an early-abort or borrowed-buffer API.
    pub fn copy<W: Word>(
        self,
        source: &'static mut [W],
        destination: &'static mut [W],
    ) -> Result<OwnedCopy<W, Self>, CopyConfigError<W, Self>> {
        let index = self.channel.index;
        OwnedCopy::start(self, index, source, destination)
    }

    /// Permanently give up safe-copy admission and enter the low-level API.
    ///
    /// This consumes the capability without starting a transfer. The returned
    /// raw channel has unsafe transfer constructors; even a completed raw copy
    /// or a raw error followed by Drop cannot be upgraded to a CopyChannel.
    pub fn into_raw(self) -> Channel<'static> {
        critical_section::with(|cs| {
            STATES[self.channel.index as usize]
                .inner
                .borrow(cs)
                .borrow_mut()
                .admission = Admission::Raw;
        });
        self.channel
    }
}

impl Channel<'static> {
    // Shared one-time admission for safe owners. Never call Channel::new here:
    // raw acquisition or downgrade must remain permanently ineligible.
    pub(crate) fn new_admitted<T: ChannelInstance>(
        channel: Peri<'static, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'static,
    ) -> Result<Self, CopyChannelConfigError<T>> {
        let result = critical_section::with(|cs| {
            let mut inner = STATES[T::INDEX as usize].inner.borrow(cs).borrow_mut();
            match inner.admission {
                Admission::Available => {
                    inner.admission = Admission::Owned;
                    Ok(())
                }
                Admission::Unavailable(error) => Err(error),
                Admission::Owned => Err(CopyChannelError::AlreadyClaimed),
                Admission::Raw => Err(CopyChannelError::RawTakeover),
            }
        });
        if let Err(error) = result {
            return Err(CopyChannelConfigError { error, channel });
        }
        unsafe { T::Interrupt::enable() };
        Ok(Self {
            index: T::INDEX,
            _borrow: PhantomData,
        })
    }

    // Both directions must be admitted before either is claimed. Only the
    // startup-owned software capability is consulted; no fresh hardware
    // snapshot or partial admission can manufacture a safe owner.
    #[cfg(any(all(uart_dma_tx, uart_dma_rx), spi_dma))]
    pub(crate) fn new_admitted_pair<T: ChannelInstance, R: ChannelInstance>(
        _tx: Peri<'static, T>,
        _rx: Peri<'static, R>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>
        + Binding<R::Interrupt, InterruptHandler<R>>
        + 'static,
    ) -> Result<(Self, Self), CopyChannelError> {
        critical_section::with(|cs| {
            if T::INDEX == R::INDEX {
                return Err(CopyChannelError::AlreadyClaimed);
            }
            for index in [T::INDEX, R::INDEX] {
                match STATES[index as usize].inner.borrow(cs).borrow().admission {
                    Admission::Available => {}
                    Admission::Unavailable(error) => return Err(error),
                    Admission::Owned => return Err(CopyChannelError::AlreadyClaimed),
                    Admission::Raw => return Err(CopyChannelError::RawTakeover),
                }
            }
            for index in [T::INDEX, R::INDEX] {
                STATES[index as usize]
                    .inner
                    .borrow(cs)
                    .borrow_mut()
                    .admission = Admission::Owned;
            }
            Ok(())
        })?;
        unsafe {
            T::Interrupt::enable();
            R::Interrupt::enable();
        }
        Ok((
            Self {
                index: T::INDEX,
                _borrow: PhantomData,
            },
            Self {
                index: R::INDEX,
                _borrow: PhantomData,
            },
        ))
    }

    // Internal hardware-request entry. The peripheral owner must retain its
    // static source, endpoint, clocks and channel before this call, including
    // on panic/forget, until a clean completion has been retired. A live owner
    // leaves a cancelled operation running; no task-local Transfer is created.
    // L012's qualified UART TDR TX and SPI DR TX profiles use SIZE for SRAM
    // byte width. The pointer carries a fixed typed-register endpoint address,
    // not a byte-MMIO access license; native peripheral accesses remain 32-bit.
    #[cfg(any(uart_dma_tx, spi_dma))]
    pub(crate) unsafe fn start_write(
        &mut self,
        request: Request,
        source: &[u8],
        destination: *mut u8,
    ) -> Result<(), ConfigError> {
        let plan = Plan::new(
            source.as_ptr() as usize,
            destination as usize,
            source.len(),
            pac::dmachannel::vals::Width::Byte,
            true,
            false,
            Some(request),
        )?;
        start_transfer(self.index, plan)
    }

    // Internal fixed-endpoint byte RX entry. Before calling, the peripheral
    // owner must retain the exclusively owned static destination allocation,
    // endpoint, channel and clocks, including on panic/forget, until a clean
    // completion has been retired. `destination` must cover `len` writable
    // bytes in previously validated SRAM; no CPU access or reference to those
    // bytes is allowed while DMA may write. Launch only from an idle lease.
    // Qualified native L012 SPI DR RX uses SIZE for SRAM byte width and this
    // pointer only for the fixed typed-register address, not byte MMIO. This
    // does not qualify the separate native UART RX endpoint.
    #[cfg(any(uart_dma_rx, spi_dma))]
    pub(crate) unsafe fn start_read(
        &mut self,
        request: Request,
        source: *const u8,
        destination: *mut u8,
        len: usize,
    ) -> Result<(), ConfigError> {
        let plan = Plan::new(
            source as usize,
            destination as usize,
            len,
            pac::dmachannel::vals::Width::Byte,
            false,
            true,
            Some(request),
        )?;
        start_transfer(self.index, plan)
    }

    // None is a single nonblocking observation for blocking methods and Drop;
    // async callers register with the same critical-section protocol as Transfer.
    #[cfg(any(uart_dma_tx, uart_dma_rx, spi_dma))]
    pub(crate) fn poll_hardware_complete(
        &mut self,
        cx: Option<&mut Context<'_>>,
    ) -> Poll<Result<(), Error>> {
        if cx.is_none() {
            service_interrupt(self.index);
        }
        critical_section::with(|cs| {
            let mut inner = STATES[self.index as usize].inner.borrow(cs).borrow_mut();
            match inner.phase {
                Phase::Finished(result) => {
                    let result = result.and_then(|()| {
                        if clean_terminal_readback(self.index) {
                            Ok(())
                        } else {
                            Err(Error::UnconfirmedCompletion)
                        }
                    });
                    // Ambiguous completion remains an error even if later
                    // hardware observations change. SOFTSRC is not evidence
                    // for a hardware-triggered transfer.
                    inner.phase = Phase::Finished(result);
                    fence(Ordering::Acquire);
                    Poll::Ready(result)
                }
                Phase::Running => {
                    if let Some(cx) = cx {
                        if !inner
                            .waker
                            .as_ref()
                            .is_some_and(|w| w.will_wake(cx.waker()))
                        {
                            inner.waker = Some(cx.waker().clone());
                        }
                    }
                    Poll::Pending
                }
                Phase::Idle => Poll::Ready(Err(Error::UnconfirmedCompletion)),
            }
        })
    }

    // Only after poll_hardware_complete returned clean success and the owner
    // closed the peripheral request. Error/abandoned owners never call this.
    #[cfg(any(uart_dma_tx, uart_dma_rx, spi_dma))]
    pub(crate) unsafe fn retire_hardware(&mut self) {
        fence(Ordering::SeqCst);
        critical_section::with(|cs| {
            let mut inner = STATES[self.index as usize].inner.borrow(cs).borrow_mut();
            assert_eq!(inner.phase, Phase::Finished(Ok(())));
            inner.waker = None;
            inner.phase = Phase::Idle;
        });
    }
}

/// Type-level interrupt handler for one channel.
pub struct InterruptHandler<T: ChannelInstance>(PhantomData<T>);

impl<T: ChannelInstance> Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        service_interrupt(T::INDEX);
    }
}

/// A running one-shot DMA transfer.
///
/// Construction immediately starts the hardware. Awaiting is interrupt-driven
/// and returns a terminal completion/error result. Drop drains the transfer;
/// it is not an early cancellation operation and can block indefinitely.
///
/// The destination remains exclusively borrowed until the transfer is dropped.
/// A channel cannot be reused by a second concurrent transfer.
#[must_use = "DMA is already running; await, blocking_wait, or Drop must drain it"]
pub struct Transfer<'a> {
    index: u8,
    _borrow: PhantomData<&'a mut ()>,
}

impl<'a> Transfer<'a> {
    /// Start a peripheral-to-memory transfer, incrementing only the destination.
    ///
    /// # Safety
    /// `peripheral` must be a readable, DMA-accessible register of width `W`
    /// with the given hardware request. Its peripheral must stay configured and
    /// supply requests until completion (or a terminal transfer error). No other
    /// agent may access the destination buffer, reset the DMA controller, or
    /// reconfigure this channel while running. If this transfer is forgotten,
    /// the buffer and channel must remain reserved and valid until hardware has
    /// been independently proven stopped. Drop may block forever on missing
    /// requests; it cannot establish early-abort quiescence by clearing EN.
    /// TE alone does not document bus quiescence: if an error occurs, callers
    /// must independently establish that before reusing the buffer/channel.
    #[cfg(cw32x030)]
    pub unsafe fn new_read<W: Word>(
        channel: &'a mut Channel<'_>,
        request: Request,
        peripheral: *const W,
        buffer: &'a mut [W],
    ) -> Result<Self, ConfigError> {
        let plan = Plan::new(
            peripheral as usize,
            buffer.as_mut_ptr() as usize,
            buffer.len(),
            W::SIZE,
            false,
            true,
            Some(request),
        )?;
        Self::start(channel, plan)
    }

    /// Start a memory-to-peripheral transfer, incrementing only the source.
    ///
    /// # Safety
    /// The obligations of [`Self::new_read`] apply with directions reversed:
    /// `peripheral` must accept DMA writes of width `W` for this request, and no
    /// other agent may mutate the source buffer until the transfer has stopped.
    /// Forgetting a running transfer does not release these obligations. Drop
    /// drains to completion/error and can block indefinitely.
    #[cfg(cw32x030)]
    pub unsafe fn new_write<W: Word>(
        channel: &'a mut Channel<'_>,
        request: Request,
        buffer: &'a [W],
        peripheral: *mut W,
    ) -> Result<Self, ConfigError> {
        let plan = Plan::new(
            buffer.as_ptr() as usize,
            peripheral as usize,
            buffer.len(),
            W::SIZE,
            true,
            false,
            Some(request),
        )?;
        Self::start(channel, plan)
    }

    /// Start a software-triggered memory copy, incrementing both addresses.
    ///
    /// # Safety
    /// Both buffers must be accessible by the DMA bus, disjoint, and remain
    /// valid until terminal completion/error. No other agent may access the
    /// destination or mutate the source while running. Do not reset/reconfigure
    /// the DMA controller or this channel. Forgetting the transfer requires
    /// retaining both buffers and channel until hardware is independently
    /// proven stopped. Drop waits for the transfer; it does not abort it.
    /// TE alone does not document bus quiescence: if an error occurs, callers
    /// must independently establish that before reusing either buffer/channel.
    pub unsafe fn new_copy<W: Word>(
        channel: &'a mut Channel<'_>,
        source: &'a [W],
        destination: &'a mut [W],
    ) -> Result<Self, ConfigError> {
        if source.len() != destination.len() {
            return Err(ConfigError::LengthMismatch);
        }
        let plan = Plan::new(
            source.as_ptr() as usize,
            destination.as_mut_ptr() as usize,
            source.len(),
            W::SIZE,
            true,
            true,
            None,
        )?;
        Self::start(channel, plan)
    }

    fn start(channel: &'a mut Channel<'_>, plan: Plan) -> Result<Self, ConfigError> {
        Self::start_index(channel.index, plan)
    }

    fn start_index(index: u8, plan: Plan) -> Result<Self, ConfigError> {
        start_transfer(index, plan)?;
        Ok(Self {
            index,
            _borrow: PhantomData,
        })
    }

    /// Whether the interrupt handler has yet to observe completion or an error.
    pub fn is_running(&self) -> bool {
        self.outcome().is_none()
    }

    /// Number of words not yet transferred, sampled from native CCNT on L012
    /// and CNT on the other supported controllers. With REPEAT=1 this is a
    /// sampled word count, not proof of completion or drained bus accesses.
    pub fn get_remaining_transfers(&self) -> u16 {
        #[cfg(dma_cw32l012_v1)]
        {
            channel_regs(self.index).ccnt().read().ccnt()
        }
        #[cfg(not(dma_cw32l012_v1))]
        {
            channel_regs(self.index).cnt().read().cnt()
        }
    }

    /// Drain to completion/error, including with CPU interrupts masked.
    /// A hardware-triggered transfer can wait indefinitely for its peripheral.
    pub fn blocking_wait(self) -> Result<(), Error> {
        self.wait_terminal()
    }

    fn outcome(&self) -> Option<Result<(), Error>> {
        critical_section::with(|cs| {
            match STATES[self.index as usize].inner.borrow(cs).borrow().phase {
                Phase::Finished(result) => Some(result),
                _ => None,
            }
        })
    }

    fn wait_terminal(&self) -> Result<(), Error> {
        loop {
            if let Some(result) = self.outcome() {
                return result;
            }
            // Explicit drain must work even if the IRQ is masked.
            service_interrupt(self.index);
            core::hint::spin_loop();
        }
    }
}

impl Future for Transfer<'_> {
    type Output = Result<(), Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        critical_section::with(|cs| {
            let mut inner = STATES[self.index as usize].inner.borrow(cs).borrow_mut();
            match inner.phase {
                Phase::Finished(result) => {
                    fence(Ordering::Acquire);
                    Poll::Ready(result)
                }
                Phase::Running => {
                    // ISR and first-poll registration share this critical section.
                    if !inner
                        .waker
                        .as_ref()
                        .is_some_and(|w| w.will_wake(cx.waker()))
                    {
                        inner.waker = Some(cx.waker().clone());
                    }
                    Poll::Pending
                }
                Phase::Idle => unreachable!("live transfer lost its channel ownership"),
            }
        })
    }
}

impl Drop for Transfer<'_> {
    fn drop(&mut self) {
        // Preserve the unsafe low-level protocol: wait for TC/TE, never clear
        // EN to manufacture an early cancellation acknowledgment.
        let _ = self.wait_terminal();
        fence(Ordering::SeqCst);
        critical_section::with(|cs| {
            let mut inner = STATES[self.index as usize].inner.borrow(cs).borrow_mut();
            inner.waker = None;
            inner.phase = Phase::Idle;
        });
    }
}

/// The channel and static SRAM buffers returned after a clean owned copy.
///
/// Source and destination retain their original lengths. They are returned only
/// after TC with no TE, completed STATUS and completed software-trigger readback.
pub struct CopyResources<W: Word, C = Channel<'static>> {
    /// Exclusive physical channel owner, available for another transfer.
    pub channel: C,
    /// Original source, available for modification or reuse.
    pub source: &'static mut [W],
    /// Original destination, containing the copied words on success.
    pub destination: &'static mut [W],
}

/// Rejected owned copy; no DMA transfer started and every resource is returned.
pub struct CopyConfigError<W: Word, C = Channel<'static>> {
    /// Reason validation or channel reservation failed.
    pub error: ConfigError,
    /// Unmodified owners, available to retry or use directly.
    pub resources: CopyResources<W, C>,
}

/// A software-triggered copy between two owned static SRAM buffers.
///
/// [`CopyChannel::copy`] is the safe entry, retaining its admitted capability.
/// [`OwnedCopy::new`] is the unsafe raw entry for an arbitrary [`Channel`].
/// The channel type `C` is preserved in configuration errors and successful
/// resources, so the shared transfer engine cannot upgrade a raw channel.
///
/// The channel and both exclusive static buffers move into this future. Only a
/// clean completion returns them. A DMA error consumes and permanently reserves
/// all three resources: the manuals do not prove error-time bus quiescence.
/// Error quarantine prevents reclaiming memory that hardware might still access;
/// the raw entry additionally requires its documented integration contract.
///
/// Forgetting this future leaks all owners; hardware may continue accessing its
/// reserved buffers. Dropping it drains until a completion/error indication and
/// discards the owners. Drop can wait forever if hardware stalls or is starved;
/// clearing EN is never treated as a drain acknowledgment. Await or use
/// [`Self::blocking_wait`] to recover the owners after successful completion.
///
/// Both full byte ranges must lie in the selected part's generated SRAM extent.
/// Profiles without memory metadata reject construction. Only u8/u16/u32 are
/// supported; there is no borrowed-buffer cancellation or peripheral wrapper.
#[must_use = "the copy has started; await it to recover its resources"]
pub struct OwnedCopy<W: Word, C = Channel<'static>> {
    transfer: Option<Transfer<'static>>,
    resources: Option<CopyResources<W, C>>,
}

impl<W: Word> OwnedCopy<W> {
    /// Validate and immediately start a copy of all words between equal slices.
    ///
    /// # Safety
    /// The channel must be quiescent on entry, with no earlier transfer or
    /// other bus master holding outstanding accesses to either buffer.
    /// The caller must enforce exclusive control of this channel's hardware.
    /// No other code, interrupt, debugger or bus master may reconfigure it,
    /// write its status/clear/trigger fields, or reset/disable its DMA controller
    /// or clock. In particular, public PAC writes can bypass the HAL owner and
    /// must not invalidate this transfer's configuration or completion evidence.
    /// Other DMA channels may use disjoint buffers if they preserve this
    /// channel's fields and the shared controller/clock state.
    /// No other agent may access the destination or mutate the source while
    /// this DMA operation might access either buffer.
    ///
    /// Hardware exclusivity must hold before and throughout construction and,
    /// if it succeeds, until clean completion returns the resources. It also
    /// persists if this future is forgotten, dropped without returning owners,
    /// or returns an error:
    /// keep the channel and buffers reserved unless hardware quiescence has
    /// been independently established. Error flags and clearing EN do not prove
    /// that condition. This API provides no recovery of quarantined resources.
    /// A configuration error starts nothing and returns every supplied owner.
    pub unsafe fn new(
        channel: Channel<'static>,
        source: &'static mut [W],
        destination: &'static mut [W],
    ) -> Result<Self, CopyConfigError<W>> {
        let index = channel.index;
        Self::start(channel, index, source, destination)
    }
}

impl<W: Word, C> OwnedCopy<W, C> {
    fn start(
        channel: C,
        index: u8,
        source: &'static mut [W],
        destination: &'static mut [W],
    ) -> Result<Self, CopyConfigError<W, C>> {
        let resources = CopyResources {
            channel,
            source,
            destination,
        };
        let plan = (|| {
            if resources.source.len() != resources.destination.len() {
                return Err(ConfigError::LengthMismatch);
            }
            let source = resources.source.as_ptr() as usize;
            let destination = resources.destination.as_mut_ptr() as usize;
            let len = resources.source.len();
            let plan = Plan::new(source, destination, len, W::SIZE, true, true, None)?;
            let (base, size) = crate::DMA_COPY_SRAM.ok_or(ConfigError::MissingMemoryMetadata)?;
            let bytes = len * core::mem::size_of::<W>();
            let limit = (base as usize)
                .checked_add(size as usize)
                .ok_or(ConfigError::AddressOutOfRange)?;
            let source_end = source
                .checked_add(bytes)
                .ok_or(ConfigError::AddressOutOfRange)?;
            let destination_end = destination
                .checked_add(bytes)
                .ok_or(ConfigError::AddressOutOfRange)?;
            if source < base as usize
                || source_end > limit
                || destination < base as usize
                || destination_end > limit
            {
                return Err(ConfigError::NotInSram);
            }
            if source < destination_end && destination < source_end {
                return Err(ConfigError::OverlappingBuffers);
            }
            Ok(plan)
        })();
        let transfer = match plan.and_then(|plan| Transfer::start_index(index, plan)) {
            Ok(transfer) => transfer,
            Err(error) => return Err(CopyConfigError { error, resources }),
        };
        Ok(Self {
            transfer: Some(transfer),
            resources: Some(resources),
        })
    }

    /// Wait even with interrupts masked, returning owners only after clean TC.
    /// On error, all owners are leaked and the physical channel stays reserved.
    pub fn blocking_wait(mut self) -> Result<CopyResources<W, C>, Error> {
        let result = self.transfer.as_ref().unwrap().wait_terminal();
        self.finish(result)
    }

    fn finish(&mut self, result: Result<(), Error>) -> Result<CopyResources<W, C>, Error> {
        let transfer = self.transfer.take().expect("owned copy already completed");
        let resources = self.resources.take().unwrap();
        let regs = channel_regs(transfer.index);
        let result = result.and_then(|()| {
            if clean_terminal_readback(transfer.index) && !regs.trig().read().softsrc() {
                Ok(())
            } else {
                Err(Error::UnconfirmedCompletion)
            }
        });
        match result {
            Ok(()) => {
                // TC means all words were correctly transferred (§8.6). The
                // extra readbacks corroborate software completion (§8.8.4).
                fence(Ordering::SeqCst);
                drop(transfer);
                Ok(resources)
            }
            Err(error) => {
                // Do not run Transfer::drop, which resets the software phase
                // for its unsafe callers. Keep this physical channel reserved
                // forever, even if TC later arrives after an ambiguous error.
                core::mem::forget(transfer);
                core::mem::forget(resources);
                Err(error)
            }
        }
    }
}

impl<W: Word, C: Unpin> Future for OwnedCopy<W, C> {
    type Output = Result<CopyResources<W, C>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let transfer = self
            .transfer
            .as_mut()
            .expect("owned copy already completed");
        match Pin::new(transfer).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => Poll::Ready(self.finish(result)),
        }
    }
}

impl<W: Word, C> Drop for OwnedCopy<W, C> {
    fn drop(&mut self) {
        if let Some(transfer) = self.transfer.as_ref() {
            let result = transfer.wait_terminal();
            // Static allocations are not reclaimed by dropping their owners.
            // Success discards the recovered owners; error leaks the reservation.
            let _ = self.finish(result);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Plan {
    source: u32,
    destination: u32,
    count: u16,
    size: pac::dmachannel::vals::Width,
    source_inc: bool,
    destination_inc: bool,
    request: Option<Request>,
}

impl Plan {
    fn new(
        source: usize,
        destination: usize,
        len: usize,
        size: pac::dmachannel::vals::Width,
        source_inc: bool,
        destination_inc: bool,
        request: Option<Request>,
    ) -> Result<Self, ConfigError> {
        if len == 0 || len > u16::MAX as usize {
            return Err(ConfigError::InvalidLength);
        }
        // Only the three sealed Word implementations supply this width.
        let width = 1usize << size.to_bits();
        let check = |address: usize, increment: bool| {
            if address == 0 || address > u32::MAX as usize {
                return Err(ConfigError::AddressOutOfRange);
            }
            if address & (width - 1) != 0 {
                return Err(ConfigError::Misaligned);
            }
            let bytes = if increment { len * width } else { width };
            if address
                .checked_add(bytes - 1)
                .filter(|&last| last <= u32::MAX as usize)
                .is_none()
            {
                return Err(ConfigError::AddressOutOfRange);
            }
            Ok(())
        };
        check(source, source_inc)?;
        check(destination, destination_inc)?;
        Ok(Self {
            source: source as u32,
            destination: destination as u32,
            count: len as u16,
            size,
            source_inc,
            destination_inc,
            request,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Phase {
    Idle,
    Running,
    Finished(Result<(), Error>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Admission {
    Unavailable(CopyChannelError),
    Available,
    Owned,
    Raw,
}
struct Inner {
    admission: Admission,
    phase: Phase,
    waker: Option<Waker>,
}
struct ChannelState {
    inner: Mutex<RefCell<Inner>>,
}
impl ChannelState {
    const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(Inner {
                admission: Admission::Unavailable(CopyChannelError::NotInitialized),
                phase: Phase::Idle,
                waker: None,
            })),
        }
    }
}
static STATES: [ChannelState; crate::DMA_CHANNEL_COUNT] =
    [const { ChannelState::new() }; crate::DMA_CHANNEL_COUNT];

// Called exactly once after successful RCC initialization, before returning
// peripheral tokens. Hardware reset/clean runtime entry is the platform basis;
// this conservative observation rejects dirty state, it does not drain it.
pub(crate) fn init(attempts: u32) {
    use crate::rcc::{Readback, SealedRccPeripheral};
    critical_section::with(|cs| {
        let rcc = crate::peripherals::DMA::RCC_INFO;
        let admission = if crate::DMA_COPY_SRAM.is_none() {
            Admission::Unavailable(CopyChannelError::MissingMemoryMetadata)
        } else if rcc.is_enabled() || rcc.reset_asserted() {
            // Do not clock, reset, acknowledge flags, or unmask unknown work.
            Admission::Unavailable(CopyChannelError::InheritedState)
        } else if rcc
            .enable_with_cs_readback(
                cs,
                Readback::Poll {
                    attempts,
                    spin: true,
                },
            )
            .is_err()
        {
            Admission::Unavailable(CopyChannelError::ClockNotReady)
        } else {
            // Only supported reset/clean runtime entry permits enabling this
            // gate. Clock-gating alone is not evidence of DMA bus quiescence.
            let default = pac::DMA.isr().read().0 == 0
                && (0..crate::DMA_CHANNEL_COUNT as u8).all(|index| {
                    let regs = channel_regs(index);
                    let count = regs.cnt().read().0;
                    // L012 silicon can power up with REPEAT=1 and CNT=0.
                    // Keep every other descriptor, flag and ownership check.
                    let count_reset = count == 0 || (cfg!(dma_cw32l012_v1) && count == 0x0001_0000);
                    let descriptor_default = regs.csr().read().0 == 0
                        && count_reset
                        && regs.srcaddr().read().srcaddr() == 0
                        && regs.dstaddr().read().dstaddr() == 0
                        && regs.trig().read().0 == 0;
                    #[cfg(dma_cw32l012_v1)]
                    let descriptor_default = descriptor_default
                        && regs.ccnt().read().0 == 0
                        && regs.csrcaddr().read().csrcaddr() == 0
                        && regs.cdstaddr().read().cdstaddr() == 0;
                    descriptor_default
                });
            if default {
                Admission::Available
            } else {
                Admission::Unavailable(CopyChannelError::InheritedState)
            }
        };
        for state in &STATES {
            let mut inner = state.inner.borrow(cs).borrow_mut();
            // Do not restore eligibility after a pre-init raw takeover through
            // an unsafe fabricated token, even if register state looks default.
            if inner.admission == Admission::Unavailable(CopyChannelError::NotInitialized) {
                inner.admission = admission;
            }
        }
    });
}

fn channel_regs(index: u8) -> pac::dmachannel::Dmachannel {
    match index {
        0 => pac::DMACHANNEL1,
        1 => pac::DMACHANNEL2,
        #[cfg(any(dma_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
        2 => pac::DMACHANNEL3,
        #[cfg(any(dma_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
        3 => pac::DMACHANNEL4,
        #[cfg(dma_v1)]
        4 => pac::DMACHANNEL5,
        _ => unreachable!("unverified DMA channel"),
    }
}

fn clear_flags(index: u8, complete: bool, error: bool) {
    // ICR is R1W0. Its source-qualified seed preserves reserved bits and all
    // other channels, including channels sharing this interrupt vector.
    let mut value = pac::dma::regs::Icr::write_noop();
    match index {
        0 => {
            value.set_tc1(!complete);
            value.set_te1(!error);
        }
        1 => {
            value.set_tc2(!complete);
            value.set_te2(!error);
        }
        #[cfg(any(dma_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
        2 => {
            value.set_tc3(!complete);
            value.set_te3(!error);
        }
        #[cfg(any(dma_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
        3 => {
            value.set_tc4(!complete);
            value.set_te4(!error);
        }
        #[cfg(dma_v1)]
        4 => {
            value.set_tc5(!complete);
            value.set_te5(!error);
        }
        _ => unreachable!("unverified DMA channel"),
    }
    pac::DMA.icr().write_value(value);
}

fn start_transfer(index: u8, plan: Plan) -> Result<(), ConfigError> {
    critical_section::with(|cs| {
        let mut inner = STATES[index as usize].inner.borrow(cs).borrow_mut();
        if inner.phase != Phase::Idle {
            return Err(ConfigError::Busy);
        }
        let regs = channel_regs(index);
        // Safe copies reach Idle only from admitted startup or clean TC.
        // Raw callers separately prove quiescence at their unsafe entry.
        // This initialization write is never an abort operation.
        regs.csr().write(|_| {});
        clear_flags(index, true, true);
        regs.srcaddr().write(|v| v.set_srcaddr(plan.source));
        regs.dstaddr().write(|v| v.set_dstaddr(plan.destination));
        regs.cnt().write(|v| {
            v.set_cnt(plan.count);
            // Reload REPEAT=1 before every transfer (§8.5.2).
            v.set_repeat(1);
        });
        regs.trig().write(|v| {
            v.set_type_(if plan.request.is_some() {
                pac::dmachannel::vals::Trigger::Hardware
            } else {
                pac::dmachannel::vals::Trigger::Software
            });
            if let Some(request) = plan.request {
                v.set_hardsrc(request.number());
            }
        });
        inner.waker = None;
        inner.phase = Phase::Running;
        fence(Ordering::SeqCst);
        regs.csr().write(|v| {
            v.set_trans(pac::dmachannel::vals::TransferMode::Block);
            v.set_tcie(true);
            v.set_teie(true);
            v.set_size(plan.size);
            v.set_srcinc(plan.source_inc);
            v.set_dstinc(plan.destination_inc);
            #[cfg(dma_cw32l012_v1)]
            {
                // Native one-shot profile: no automatic restart or address reload.
                v.set_restart(false);
                v.set_srcload(false);
                v.set_dstload(false);
            }
            v.set_en(true);
        });
        if plan.request.is_none() {
            regs.trig().write(|v| v.set_softsrc(true));
        }
        Ok(())
    })
}

// Only corroborates an already recorded TC without TE. Disabled EN alone is
// never evidence that outstanding accesses have drained.
fn clean_terminal_readback(index: u8) -> bool {
    let status = channel_regs(index).csr().read();
    status.status() == pac::dmachannel::vals::Status::Complete
        && !status.en()
        && !status.tcie()
        && !status.teie()
}

fn decode_error(status: pac::dmachannel::vals::Status) -> Error {
    use pac::dmachannel::vals::Status;
    match status {
        Status::AddressOutOfRange => Error::AddressOutOfRange,
        Status::Stopped => Error::Stopped,
        Status::SourceError => Error::Source,
        Status::DestinationError => Error::Destination,
        other => Error::Unknown(other.to_bits()),
    }
}

fn service_interrupt(index: u8) {
    let waker = critical_section::with(|cs| {
        let mut inner = STATES[index as usize].inner.borrow(cs).borrow_mut();
        if inner.phase != Phase::Running {
            return None;
        }
        let pending = pac::DMA.isr().read();
        let (complete, error) = match index {
            0 => (pending.tc1(), pending.te1()),
            1 => (pending.tc2(), pending.te2()),
            #[cfg(any(dma_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
            2 => (pending.tc3(), pending.te3()),
            #[cfg(any(dma_v1, dma_cw32l031_v1, dma_cw32l012_v1))]
            3 => (pending.tc4(), pending.te4()),
            #[cfg(dma_v1)]
            4 => (pending.tc5(), pending.te5()),
            _ => unreachable!("unverified DMA channel"),
        };
        if !complete && !error {
            return None;
        }
        {
            let regs = channel_regs(index);
            let result = if error {
                Err(decode_error(regs.csr().read().status()))
            } else {
                Ok(())
            };
            // TC/TE ends the existing unsafe transfer protocol. TE is NOT used
            // by OwnedCopy to prove drained bus accesses or return resources.
            regs.csr().modify(|v| {
                v.set_en(false);
                v.set_tcie(false);
                v.set_teie(false);
            });
            fence(Ordering::SeqCst);
            inner.phase = Phase::Finished(result);
        }
        clear_flags(index, complete, error);
        inner.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}
