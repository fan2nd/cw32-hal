//! Owned, seven-bit I2C controller with blocking and interrupt-driven transfers.
//!
//! Consecutive operations in the same direction are concatenated. Direction
//! changes generate repeated START; only the last read byte is NACKed. See
//! `docs/i2c.md` for timing assumptions, IP differences and recovery limits.

#[cfg(i2c_f030)]
#[path = "f030.rs"]
mod backend;
#[cfg(i2c_l012)]
#[path = "l012.rs"]
mod backend;

use crate::{
    gpio::{AfType, Flex, OutputType, Pin, Pull},
    interrupt,
    rcc::{ClockGuard, PeripheralClock},
    Async, Blocking, Mode, Peri, PeripheralType,
};
use core::{future::poll_fn, marker::PhantomData, task::Poll};
use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource, Operation};

mod sealed {
    pub(crate) trait Sealed {
        fn regs() -> crate::pac::i2c::I2c;
        fn state() -> &'static crate::interrupt::EventState;
    }
    pub trait SclPin<I> {}
    pub trait SdaPin<I> {}
}
/// Metadata-generated controller and physical interrupt identity.
#[allow(private_bounds)]
pub trait Instance: sealed::Sealed + PeripheralClock + PeripheralType + 'static {
    type Interrupt: interrupt::typelevel::Interrupt;
}
/// A documented SCL alternate-function route for this controller.
pub trait SclPin<I: Instance>: sealed::SclPin<I> + Pin {
    const AF: u8;
}
/// A documented SDA alternate-function route for this controller.
pub trait SdaPin<I: Instance>: sealed::SdaPin<I> + Pin {
    const AF: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_i2c.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Bus,
    Arbitration,
    /// F030 identifies Address or Data. L012's single sticky flag reports Unknown.
    NoAcknowledge(NoAcknowledgeSource),
    /// A polling allowance was exhausted, or hardware detected pin-low timeout.
    Timeout,
    Overrun,
    InvalidAddress,
    EmptyRead,
    TransferTooLong,
    InvalidConfig,
    /// The peripheral could not be reset exclusively after a failed transfer.
    RecoveryFailed,
}
impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> ErrorKind {
        match *self {
            Self::Bus => ErrorKind::Bus,
            Self::Arbitration => ErrorKind::ArbitrationLoss,
            Self::NoAcknowledge(source) => ErrorKind::NoAcknowledge(source),
            Self::Overrun => ErrorKind::Overrun,
            _ => ErrorKind::Other,
        }
    }
}

/// Controller timing and finite polling allowances. External pull-ups are normally required.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    /// Maximum requested SCL frequency, 1..=400_000 Hz. Actual frequency can be lower.
    pub frequency: u32,
    /// Board's maximum SDA/SCL rise time. Standard mode: <=1000 ns; fast: <=300 ns.
    pub rise_time_ns: u32,
    /// Board's maximum SDA/SCL fall time, <=300 ns.
    pub fall_time_ns: u32,
    /// Enable optional weak GPIO pull-ups. These do not guarantee compliant rise times.
    pub internal_pullups: bool,
    /// Maximum consecutive unsuccessful register observations in blocking calls.
    /// This is a CPU-dependent observation count, not an elapsed-time deadline.
    pub poll_limit: u32,
    /// Maximum observations when waiting for STOP or aborting a cancelled transfer.
    pub recovery_poll_limit: u32,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 100_000,
            rise_time_ns: 300,
            fall_time_ns: 100,
            internal_pullups: false,
            poll_limit: 100_000,
            recovery_poll_limit: 10_000,
        }
    }
}
impl Config {
    fn validate(&self) -> Result<(), Error> {
        if self.frequency == 0
            || self.frequency > 400_000
            || self.poll_limit == 0
            || self.recovery_poll_limit == 0
            || self.fall_time_ns > 300
            || self.rise_time_ns > if self.frequency <= 100_000 { 1000 } else { 300 }
        {
            return Err(Error::InvalidConfig);
        }
        Ok(())
    }
}

/// Controller, two exclusive pin owners, and the physical peripheral clock lease.
pub struct I2c<'d, T: Instance, M: Mode> {
    _peri: Peri<'d, T>,
    _scl: Flex<'d>,
    _sda: Flex<'d>,
    clock: ClockGuard,
    timing: backend::Timing,
    config: Config,
    poisoned: bool,
    in_flight: bool,
    _mode: PhantomData<M>,
}
impl<'d, T: Instance> I2c<'d, T, Blocking> {
    pub fn new_blocking<SCL: SclPin<T>, SDA: SdaPin<T>>(
        peri: Peri<'d, T>,
        scl: Peri<'d, SCL>,
        sda: Peri<'d, SDA>,
        config: Config,
    ) -> Result<Self, Error> {
        Self::new_inner(peri, scl, sda, config)
    }
}
impl<'d, T: Instance> I2c<'d, T, Async> {
    /// Construct with proof that the physical IRQ dispatches this controller's handler.
    pub fn new<SCL: SclPin<T>, SDA: SdaPin<T>>(
        peri: Peri<'d, T>,
        scl: Peri<'d, SCL>,
        sda: Peri<'d, SDA>,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'd,
        config: Config,
    ) -> Result<Self, Error> {
        Self::new_inner(peri, scl, sda, config)
    }
    /// IRQ-driven transfer. Use a timer/select at the application level for an
    /// elapsed deadline. Dropping the future runs bounded controller recovery.
    pub async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        let mut transfer = Transfer::new(self, address, operations)?;
        poll_fn(|cx| {
            T::state().register(cx.waker());
            // Registration precedes hardware inspection and IRQ arming. Sticky
            // hardware flags survive ISR masking; no buffer pointer lives in ISR state.
            loop {
                backend::disarm::<T>();
                match transfer.step() {
                    Ok(Step::Progress) => continue,
                    Ok(Step::Done) => return Poll::Ready(transfer.finish(Ok(()))),
                    Err(error) => return Poll::Ready(transfer.finish(Err(error))),
                    Ok(Step::Wait(mask)) => {
                        backend::arm::<T>(mask);
                        // Close the check/enable race without self-waking polling.
                        if backend::ready::<T>(mask) {
                            continue;
                        }
                        return Poll::Pending;
                    }
                }
            }
        })
        .await
    }
    pub async fn read(&mut self, address: u8, bytes: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Read(bytes)])
            .await
    }
    pub async fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.transaction(address, &mut [Operation::Write(bytes)])
            .await
    }
    pub async fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Error> {
        self.transaction(
            address,
            &mut [Operation::Write(write), Operation::Read(read)],
        )
        .await
    }
}
impl<'d, T: Instance, M: Mode> I2c<'d, T, M> {
    fn new_inner<SCL: SclPin<T>, SDA: SdaPin<T>>(
        peri: Peri<'d, T>,
        scl: Peri<'d, SCL>,
        sda: Peri<'d, SDA>,
        config: Config,
    ) -> Result<Self, Error> {
        config.validate()?;
        // L012 explicitly selects PCLK in MCR0; F030 is fixed to PCLK.
        let timing = backend::Timing::new(T::bus_frequency(), &config)?;
        let clock = T::acquire();
        backend::disarm::<T>();
        T::state().reset();
        let mut scl = Flex::new(scl);
        let mut sda = Flex::new(sda);
        scl.set_high();
        sda.set_high();
        let af = AfType::output_pull(
            OutputType::OpenDrain,
            #[cfg(gpio_has_speed)]
            crate::gpio::Speed::High,
            if config.internal_pullups {
                Pull::Up
            } else {
                Pull::None
            },
        );
        scl.set_as_af_unchecked(SCL::AF, af);
        sda.set_as_af_unchecked(SDA::AF, af);
        backend::configure::<T>(&timing);
        Ok(Self {
            _peri: peri,
            _scl: scl,
            _sda: sda,
            clock,
            timing,
            config,
            poisoned: false,
            in_flight: false,
            _mode: PhantomData,
        })
    }
    /// Configured nominal upper bound, excluding external stretching and added bus delay.
    pub fn frequency(&self) -> u32 {
        self.timing.frequency()
    }
    /// Selected controller kernel frequency, before the local divider.
    pub fn kernel_frequency(&self) -> u32 {
        T::bus_frequency()
    }
    pub fn blocking_transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        let limit = self.config.poll_limit;
        let mut remaining = limit;
        let mut transfer = Transfer::new(self, address, operations)?;
        loop {
            match transfer.step() {
                Ok(Step::Progress) => remaining = limit,
                Ok(Step::Done) => return transfer.finish(Ok(())),
                Err(error) => return transfer.finish(Err(error)),
                Ok(Step::Wait(_)) => {
                    remaining -= 1;
                    if remaining == 0 {
                        return transfer.finish(Err(Error::Timeout));
                    }
                    core::hint::spin_loop();
                }
            }
        }
    }
    pub fn blocking_read(&mut self, address: u8, bytes: &mut [u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Read(bytes)])
    }
    pub fn blocking_write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.blocking_transaction(address, &mut [Operation::Write(bytes)])
    }
    pub fn blocking_write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Error> {
        self.blocking_transaction(
            address,
            &mut [Operation::Write(write), Operation::Read(read)],
        )
    }
    /// Reset local controller state after bounded STOP cleanup. This does not
    /// generate GPIO recovery clocks or fix a target holding SDA/SCL low.
    pub fn recover(&mut self) -> Result<(), Error> {
        backend::abort::<T>(self.config.recovery_poll_limit);
        if !self.clock.reset() {
            self.poisoned = true;
            return Err(Error::RecoveryFailed);
        }
        backend::configure::<T>(&self.timing);
        T::state().reset();
        self.poisoned = false;
        self.in_flight = false;
        Ok(())
    }
}
impl<T: Instance, M: Mode> Drop for I2c<'_, T, M> {
    fn drop(&mut self) {
        backend::abort::<T>(self.config.recovery_poll_limit);
        // Release pins before the clock lease is dropped. No borrowed buffers
        // or DMA remain associated with the peripheral.
        self._scl.set_as_disconnected();
        self._sda.set_as_disconnected();
        T::state().reset();
    }
}

/// IRQ handler. It masks the event and wakes the task, leaving status/data for
/// the single owning task. F030 has no peripheral interrupt-enable register;
/// its dedicated NVIC vector is masked while the task consumes SI.
pub struct InterruptHandler<T: Instance>(PhantomData<T>);
impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let wake = critical_section::with(|_| {
            if !T::clock_resource().is_enabled() || !backend::interrupt_pending::<T>() {
                return None;
            }
            backend::disarm::<T>();
            T::state().latch(1)
        });
        if let Some(wake) = wake {
            wake.wake();
        }
    }
}

pub(super) enum Step {
    Progress,
    Wait(u32),
    Done,
}

/// Buffer traversal is common to both real IPs; bus state transitions are not.
pub(super) struct Cursor<'a, 'b> {
    operations: &'a mut [Operation<'b>],
    index: usize,
    offset: usize,
    end: usize,
    pub read: bool,
    pub remaining: usize,
}
impl<'a, 'b> Cursor<'a, 'b> {
    fn new(operations: &'a mut [Operation<'b>]) -> Result<Self, Error> {
        let mut total = 0usize;
        for op in operations.iter() {
            let len = match op {
                Operation::Read(bytes) if bytes.is_empty() => return Err(Error::EmptyRead),
                Operation::Read(bytes) => bytes.len(),
                Operation::Write(bytes) => bytes.len(),
            };
            total = total.checked_add(len).ok_or(Error::TransferTooLong)?;
        }
        let mut this = Self {
            operations,
            index: 0,
            offset: 0,
            end: 0,
            read: false,
            remaining: 0,
        };
        this.next_group();
        Ok(this)
    }
    pub fn done(&self) -> bool {
        self.index == self.operations.len()
    }
    pub fn next_group(&mut self) {
        self.index = self.end;
        self.offset = 0;
        self.remaining = 0;
        if self.done() {
            return;
        }
        self.read = matches!(self.operations[self.index], Operation::Read(_));
        while self.end < self.operations.len()
            && matches!(self.operations[self.end], Operation::Read(_)) == self.read
        {
            self.remaining += match &self.operations[self.end] {
                Operation::Read(b) => b.len(),
                Operation::Write(b) => b.len(),
            };
            self.end += 1;
        }
    }
    pub fn take_write(&mut self) -> u8 {
        loop {
            let Operation::Write(bytes) = &self.operations[self.index] else {
                unreachable!()
            };
            if self.offset == bytes.len() {
                self.index += 1;
                self.offset = 0;
                continue;
            }
            let value = bytes[self.offset];
            self.offset += 1;
            self.remaining -= 1;
            return value;
        }
    }
    pub fn put_read(&mut self, value: u8) {
        loop {
            let Operation::Read(bytes) = &mut self.operations[self.index] else {
                unreachable!()
            };
            if self.offset == bytes.len() {
                self.index += 1;
                self.offset = 0;
                continue;
            }
            bytes[self.offset] = value;
            self.offset += 1;
            self.remaining -= 1;
            return;
        }
    }
}
struct Transfer<'a, 'b, 'd, T: Instance, M: Mode> {
    driver: &'a mut I2c<'d, T, M>,
    cursor: Cursor<'a, 'b>,
    engine: backend::Engine,
    active: bool,
}
impl<'a, 'b, 'd, T: Instance, M: Mode> Transfer<'a, 'b, 'd, T, M> {
    fn new(
        driver: &'a mut I2c<'d, T, M>,
        address: u8,
        operations: &'a mut [Operation<'b>],
    ) -> Result<Self, Error> {
        if address > 0x7f {
            return Err(Error::InvalidAddress);
        }
        let cursor = Cursor::new(operations)?;
        if driver.poisoned {
            return Err(Error::RecoveryFailed);
        }
        // A safely forgotten future need not run its Drop. No buffer pointer
        // survives in IRQ state, but the controller can still be on the bus.
        if driver.in_flight {
            driver.recover()?;
        }
        backend::disarm::<T>();
        T::state().reset();
        let active = !cursor.done();
        driver.in_flight = active;
        Ok(Self {
            driver,
            cursor,
            engine: backend::Engine::new(address),
            active,
        })
    }
    fn step(&mut self) -> Result<Step, Error> {
        if !self.active {
            return Ok(Step::Done);
        }
        self.engine
            .step::<T>(&mut self.cursor, self.driver.config.recovery_poll_limit)
    }
    fn finish(&mut self, result: Result<(), Error>) -> Result<(), Error> {
        backend::disarm::<T>();
        if result.is_err() {
            let _ = self.driver.recover();
        } else {
            self.driver.in_flight = false;
        }
        self.active = false;
        T::state().reset();
        result
    }
}
impl<T: Instance, M: Mode> Drop for Transfer<'_, '_, '_, T, M> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.driver.recover();
        }
    }
}
impl<T: Instance, M: Mode> embedded_hal::i2c::ErrorType for I2c<'_, T, M> {
    type Error = Error;
}
impl<T: Instance, M: Mode> embedded_hal::i2c::I2c for I2c<'_, T, M> {
    fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Error> {
        self.blocking_transaction(address, operations)
    }
}
impl<T: Instance> embedded_hal_async::i2c::I2c for I2c<'_, T, Async> {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Error> {
        self.transaction(address, operations).await
    }
}

fn ceil_div(numerator: u64, denominator: u64) -> u64 {
    numerator / denominator + u64::from(numerator % denominator != 0)
}
