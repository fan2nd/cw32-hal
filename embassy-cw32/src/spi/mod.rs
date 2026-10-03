//! Owned full-duplex SPI controller buses, with blocking or interrupt-driven I/O.
//!
//! The generated pin traits check each SCK/MOSI/MISO route. Chip select belongs
//! to an external GPIO and an `embedded-hal` / `embedded-hal-async` `SpiDevice`
//! adapter. There is no hardware-CS, target, half-duplex or DMA API here.
//!
//! Both modes send one frame at a time and consume its receive word before
//! sending another. Successful operations wait for TXE and !BUSY. Async frame
//! waits use peripheral interrupts; the final BUSY tail has a finite polling
//! budget because these IPs have no idle-completion interrupt.

use crate::{
    gpio::{AfType, Flex, Level, OutputType, Pin, Pull},
    interrupt,
    rcc::KernelClock,
    Async, Blocking, Mode as DriverMode, Peri, PeripheralType,
};
use core::{future::poll_fn, marker::PhantomData};
use interrupt::typelevel::Interrupt as _;

pub use embedded_hal::spi::{Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};

#[cfg(spi_l012)]
#[path = "l012.rs"]
mod backend;
#[cfg(spi_f030)]
#[path = "f030.rs"]
mod backend;
mod common;

mod sealed {
    pub(crate) trait Instance {
        fn regs() -> crate::pac::spi::Spi;
        fn state() -> &'static crate::interrupt::EventState;
    }
    pub trait SckPin<I> {}
    pub trait MosiPin<I> {}
    pub trait MisoPin<I> {}
    pub trait Word {
        const BITS: u8;
        fn into_u16(self) -> u16;
        fn from_u16(word: u16) -> Self;
    }
}

/// Audited generated peripheral identity, kernel clock and interrupt route.
#[allow(private_bounds)]
pub trait Instance: sealed::Instance + KernelClock + PeripheralType + 'static {
    type Interrupt: interrupt::typelevel::Interrupt;
}
/// Pin with a documented SPI controller clock route.
pub trait SckPin<I: Instance>: sealed::SckPin<I> + Pin {
    const AF: u8;
}
/// Pin with a documented SPI controller output route.
pub trait MosiPin<I: Instance>: sealed::MosiPin<I> + Pin {
    const AF: u8;
}
/// Pin with a documented SPI controller input route.
pub trait MisoPin<I: Instance>: sealed::MisoPin<I> + Pin {
    const AF: u8;
}

include!(concat!(env!("OUT_DIR"), "/_generated_spi.rs"));

/// Supported frame words: `u8` selects 8 bits and `u16` selects 16 bits.
/// Other documented 4..16-bit widths are not exposed by this version.
#[allow(private_bounds)]
pub trait Word: sealed::Word + Copy + Default + 'static {}
impl sealed::Word for u8 {
    const BITS: u8 = 8;
    fn into_u16(self) -> u16 {
        u16::from(self)
    }
    fn from_u16(word: u16) -> Self {
        word as u8
    }
}
impl Word for u8 {}
impl sealed::Word for u16 {
    const BITS: u8 = 16;
    fn into_u16(self) -> u16 {
        self
    }
    fn from_u16(word: u16) -> Self {
        word
    }
}
impl Word for u16 {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitOrder {
    MsbFirst,
    LsbFirst,
}

/// Configuration shared by both audited SPI IPs.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    pub mode: Mode,
    pub bit_order: BitOrder,
    /// Maximum nominal SCK rate in Hz. The fastest supported rate no greater
    /// than this value is selected. Above PCLK/2 selects PCLK/2; too low fails.
    pub frequency: u32,
    /// Maximum register observations for each blocking wait and final BUSY
    /// drain (including async transfers). This is not an elapsed-time deadline.
    /// Async TXE/RXNE waits have no timeout; cancel externally when needed.
    pub poll_limit: u32,
    /// Programmable output slew rate exists only on GPIO IPs with SPEED.
    #[cfg(gpio_has_speed)]
    pub gpio_speed: crate::gpio::Speed,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            mode: MODE_0,
            bit_order: BitOrder::MsbFirst,
            frequency: 1_000_000,
            poll_limit: 100_000,
            #[cfg(gpio_has_speed)]
            gpio_speed: crate::gpio::Speed::High,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    ZeroFrequency,
    /// Even the largest documented divisor would exceed the requested rate.
    FrequencyTooLow,
    ZeroPollBudget,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    ModeFault,
    Overrun,
    Underrun,
    Select,
    /// A bounded blocking wait or final BUSY drain exhausted its allowance.
    Timeout,
}
impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        use embedded_hal::spi::ErrorKind;
        match self {
            Self::ModeFault => ErrorKind::ModeFault,
            Self::Overrun => ErrorKind::Overrun,
            Self::Select => ErrorKind::ChipSelectFault,
            Self::Underrun | Self::Timeout => ErrorKind::Other,
        }
    }
}

/// Full-duplex controller owning its token, three pins and counted clock.
///
/// Blocking methods work in either mode. Async methods require `Async`, whose
/// constructor checks the interrupt binding. Cancel/error/Drop stop the module
/// and clear both TX and receive state before releasing any owned resource.
pub struct Spi<'d, I: Instance, M: DriverMode> {
    _instance: Peri<'d, I>,
    _sck: Flex<'d>,
    _mosi: Flex<'d>,
    _miso: Flex<'d>,
    // After pin fields: pin Drop runs before the final clock release.
    _clock: crate::rcc::ClockGuard,
    #[cfg(gpio_has_speed)]
    sck_af: u8,
    #[cfg(gpio_has_speed)]
    mosi_af: u8,
    config: Config,
    br: u8,
    divider: u16,
    clock_hz: u32,
    in_flight: bool,
    _mode: PhantomData<M>,
}

impl<'d, I: Instance> Spi<'d, I, Blocking> {
    pub fn new_blocking<S: SckPin<I>, O: MosiPin<I>, N: MisoPin<I>>(
        instance: Peri<'d, I>,
        sck: Peri<'d, S>,
        mosi: Peri<'d, O>,
        miso: Peri<'d, N>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::new_inner(instance, sck, mosi, miso, config)
    }
}

impl<'d, I: Instance> Spi<'d, I, Async> {
    pub fn new<S: SckPin<I>, O: MosiPin<I>, N: MisoPin<I>>(
        instance: Peri<'d, I>,
        sck: Peri<'d, S>,
        mosi: Peri<'d, O>,
        miso: Peri<'d, N>,
        _irq: impl interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let bus = Self::new_inner(instance, sck, mosi, miso, config)?;
        // Do not unpend or disable a possibly shared vector. Each handler masks
        // and services only its own peripheral, including after driver Drop.
        unsafe { I::Interrupt::enable() };
        Ok(bus)
    }

    /// Receive words while transmitting zero words.
    pub async fn read<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.transfer(words, &[]).await
    }

    /// Transmit words and consume every received word, preventing overrun.
    pub async fn write<W: Word>(&mut self, words: &[W]) -> Result<(), Error> {
        self.transfer(&mut [], words).await
    }

    /// Exchange max(read.len(), write.len()) frames. A shorter write slice is
    /// padded with zeros; receive words beyond the read slice are discarded.
    pub async fn transfer<W: Word>(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        let mut op = Operation::begin::<W>(self);
        for index in 0..read.len().max(write.len()) {
            let tx = write.get(index).copied().unwrap_or_default();
            let rx = op.exchange_async(tx).await?;
            if let Some(word) = read.get_mut(index) {
                *word = rx;
            }
        }
        op.finish()
    }

    pub async fn transfer_in_place<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        let mut op = Operation::begin::<W>(self);
        for word in words {
            *word = op.exchange_async(*word).await?;
        }
        op.finish()
    }

    /// Finish any outstanding frame and verify the bus is idle. Normal methods
    /// already finish completely. Also handles a deliberately forgotten future.
    pub async fn flush(&mut self) -> Result<(), Error> {
        let op = Operation::existing(self);
        if op.bus.in_flight {
            poll_fn(|cx| common::poll_flag::<I>(cx, common::RXNE)).await?;
            let _ = I::regs().dr().read();
            op.bus.in_flight = false;
        }
        op.finish()
    }
}

impl<'d, I: Instance, M: DriverMode> Spi<'d, I, M> {
    fn new_inner<S: SckPin<I>, O: MosiPin<I>, N: MisoPin<I>>(
        instance: Peri<'d, I>,
        sck: Peri<'d, S>,
        mosi: Peri<'d, O>,
        miso: Peri<'d, N>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let clock_hz = I::frequency();
        let (br, divider) = validate(clock_hz, &config)?;
        let clock = I::acquire();
        common::stop::<I>();
        // Establish the idle peripheral level before attaching its outputs.
        backend::configure(I::regs(), &config, br, 8);
        let mut sck = Flex::new(sck);
        let mut mosi = Flex::new(mosi);
        let mut miso = Flex::new(miso);
        sck.set_level(match config.mode.polarity {
            Polarity::IdleLow => Level::Low,
            Polarity::IdleHigh => Level::High,
        });
        mosi.set_level(Level::Low);
        let af = output_af(&config);
        sck.set_as_af_unchecked(S::AF, af);
        mosi.set_as_af_unchecked(O::AF, af);
        miso.set_as_af_unchecked(N::AF, AfType::input(Pull::None));
        Ok(Self {
            _instance: instance,
            _sck: sck,
            _mosi: mosi,
            _miso: miso,
            _clock: clock,
            #[cfg(gpio_has_speed)]
            sck_af: S::AF,
            #[cfg(gpio_has_speed)]
            mosi_af: O::AF,
            config,
            br,
            divider,
            clock_hz,
            in_flight: false,
            _mode: PhantomData,
        })
    }

    /// Actual nominal SCK in Hz, rounded down for a fractional result.
    pub fn frequency(&self) -> u32 {
        self.clock_hz / u32::from(self.divider)
    }
    /// The exact nominal rate is `kernel_clock_hz() / clock_divider()`.
    pub fn kernel_clock_hz(&self) -> u32 {
        self.clock_hz
    }
    pub fn clock_divider(&self) -> u16 {
        self.divider
    }
    pub fn config(&self) -> Config {
        self.config
    }

    /// Reconfigure an idle bus; validate first, then abort any forgotten
    /// operation. The caller must have deasserted every external chip select.
    pub fn set_config(&mut self, config: Config) -> Result<(), ConfigError> {
        let (br, divider) = validate(self.clock_hz, &config)?;
        self.stop();
        self.config = config;
        self.br = br;
        self.divider = divider;
        backend::configure(I::regs(), &config, br, 8);
        #[cfg(gpio_has_speed)]
        {
            // Reapply the owned output pins with the same generated routes.
            self._sck
                .set_as_af_unchecked(self.sck_af, output_af(&config));
            self._mosi
                .set_as_af_unchecked(self.mosi_af, output_af(&config));
        }
        Ok(())
    }

    pub fn blocking_read<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.blocking_transfer(words, &[])
    }
    pub fn blocking_write<W: Word>(&mut self, words: &[W]) -> Result<(), Error> {
        self.blocking_transfer(&mut [], words)
    }
    pub fn blocking_transfer<W: Word>(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        let mut op = Operation::begin::<W>(self);
        for index in 0..read.len().max(write.len()) {
            let tx = write.get(index).copied().unwrap_or_default();
            let rx = op.exchange_blocking(tx)?;
            if let Some(word) = read.get_mut(index) {
                *word = rx;
            }
        }
        op.finish()
    }
    pub fn blocking_transfer_in_place<W: Word>(&mut self, words: &mut [W]) -> Result<(), Error> {
        let mut op = Operation::begin::<W>(self);
        for word in words {
            *word = op.exchange_blocking(*word)?;
        }
        op.finish()
    }
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        let op = Operation::existing(self);
        if op.bus.in_flight {
            common::wait_blocking::<I>(common::RXNE, op.bus.config.poll_limit)?;
            let _ = I::regs().dr().read();
            op.bus.in_flight = false;
        }
        op.finish()
    }

    fn stop(&mut self) {
        common::stop::<I>();
        self.in_flight = false;
    }
}

fn validate(clock: u32, config: &Config) -> Result<(u8, u16), ConfigError> {
    if config.poll_limit == 0 {
        return Err(ConfigError::ZeroPollBudget);
    }
    backend::divider(clock, config.frequency)
}

fn output_af(config: &Config) -> AfType {
    #[cfg(not(gpio_has_speed))]
    let _ = config;
    AfType::output(
        OutputType::PushPull,
        #[cfg(gpio_has_speed)]
        config.gpio_speed,
    )
}

impl<I: Instance, M: DriverMode> Drop for Spi<'_, I, M> {
    fn drop(&mut self) {
        self.stop();
        // Flex fields subsequently disconnect before ClockGuard drops.
    }
}

/// Checked peripheral ISR. List both handlers when using the SPI23 vector.
pub struct InterruptHandler<I: Instance>(PhantomData<I>);
impl<I: Instance> interrupt::typelevel::Handler<I::Interrupt> for InterruptHandler<I> {
    unsafe fn on_interrupt() {
        common::on_interrupt::<I>();
    }
}

/// A future owns this guard on first poll, before sending any data. Dropping
/// it cannot leave background activity or a borrowed-buffer reference in ISR.
struct Operation<'a, 'd, I: Instance, M: DriverMode> {
    bus: &'a mut Spi<'d, I, M>,
    complete: bool,
}
impl<'a, 'd, I: Instance, M: DriverMode> Operation<'a, 'd, I, M> {
    fn existing(bus: &'a mut Spi<'d, I, M>) -> Self {
        Self {
            bus,
            complete: false,
        }
    }
    fn begin<W: Word>(bus: &'a mut Spi<'d, I, M>) -> Self {
        // Safe recovery if an earlier future was forgotten: stop its single
        // pending frame and clear stale IRQ/RX state before restoring CR1.
        bus.stop();
        backend::configure(I::regs(), &bus.config, bus.br, W::BITS);
        Self::existing(bus)
    }
    fn send<W: Word>(&mut self, word: W) {
        I::regs()
            .dr()
            .write_value(crate::pac::spi::regs::Dr(u32::from(word.into_u16())));
        self.bus.in_flight = true;
    }
    fn receive<W: Word>(&mut self) -> W {
        let word = I::regs().dr().read().dr();
        self.bus.in_flight = false;
        W::from_u16(word)
    }
    fn exchange_blocking<W: Word>(&mut self, word: W) -> Result<W, Error> {
        common::wait_blocking::<I>(common::TXE, self.bus.config.poll_limit)?;
        self.send(word);
        common::wait_blocking::<I>(common::RXNE, self.bus.config.poll_limit)?;
        Ok(self.receive())
    }
    fn finish(mut self) -> Result<(), Error> {
        common::drain::<I>(self.bus.config.poll_limit)?;
        common::disarm::<I>();
        I::state().reset();
        self.complete = true;
        Ok(())
    }
}
impl<I: Instance> Operation<'_, '_, I, Async> {
    async fn exchange_async<W: Word>(&mut self, word: W) -> Result<W, Error> {
        poll_fn(|cx| common::poll_flag::<I>(cx, common::TXE)).await?;
        self.send(word);
        poll_fn(|cx| common::poll_flag::<I>(cx, common::RXNE)).await?;
        Ok(self.receive())
    }
}
impl<I: Instance, M: DriverMode> Drop for Operation<'_, '_, I, M> {
    fn drop(&mut self) {
        if !self.complete {
            self.bus.stop();
        }
    }
}

impl<I: Instance, M: DriverMode> embedded_hal::spi::ErrorType for Spi<'_, I, M> {
    type Error = Error;
}
impl<I: Instance, M: DriverMode, W: Word> embedded_hal::spi::SpiBus<W> for Spi<'_, I, M> {
    fn read(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.blocking_read(words)
    }
    fn write(&mut self, words: &[W]) -> Result<(), Error> {
        self.blocking_write(words)
    }
    fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        self.blocking_transfer(read, write)
    }
    fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.blocking_transfer_in_place(words)
    }
    fn flush(&mut self) -> Result<(), Error> {
        self.blocking_flush()
    }
}
impl<I: Instance, W: Word> embedded_hal_async::spi::SpiBus<W> for Spi<'_, I, Async> {
    async fn read(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.read(words).await
    }
    async fn write(&mut self, words: &[W]) -> Result<(), Error> {
        self.write(words).await
    }
    async fn transfer(&mut self, read: &mut [W], write: &[W]) -> Result<(), Error> {
        self.transfer(read, write).await
    }
    async fn transfer_in_place(&mut self, words: &mut [W]) -> Result<(), Error> {
        self.transfer_in_place(words).await
    }
    async fn flush(&mut self) -> Result<(), Error> {
        self.flush().await
    }
}
