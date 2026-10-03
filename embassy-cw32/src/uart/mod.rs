//! Owned asynchronous serial ports, with blocking and interrupt-driven I/O.
//!
//! The unbuffered receiver retains the hardware's single-word holding register.
//! Service it promptly: CW32L012 reports overrun, while CW32F030 cannot detect
//! overwritten receive data. Async operations require a real interrupt binding;
//! they never lend application buffers to DMA or an interrupt handler.

use core::marker::PhantomData;

use crate::gpio::{AfType, Flex, OutputType, Pin, Pull};
use crate::interrupt::typelevel::Interrupt as _;
use crate::{Async, Blocking, Mode, Peri, PeripheralType};

mod r#async;
#[cfg(any(dma_l012, dma_f030))]
pub mod dma;
#[cfg(uart_f030)]
mod f030;
#[cfg(uart_l012)]
mod l012;
pub use r#async::InterruptHandler;

pub(crate) mod sealed {
    pub(crate) trait Instance: crate::rcc::PeripheralClock {
        fn regs() -> crate::pac::uart::Uart;
        fn state() -> &'static super::State;
    }
    pub trait TxPin<I> {}
    pub trait RxPin<I> {}
}

/// An audited UART instance and its physical interrupt vector.
#[allow(private_bounds)]
pub trait Instance: sealed::Instance + PeripheralType + 'static {
    type Interrupt: crate::interrupt::typelevel::Interrupt;
}
/// An audited transmitter pin route. Check bonding on the actual package.
#[allow(private_bounds)]
pub trait TxPin<I: Instance>: Pin + sealed::TxPin<I> {
    const AF: u8;
}
/// An audited receiver pin route. Check bonding on the actual package.
#[allow(private_bounds)]
pub trait RxPin<I: Instance>: Pin + sealed::RxPin<I> {
    const AF: u8;
}

pub(crate) struct State {
    tx: crate::interrupt::EventState,
    rx: crate::interrupt::EventState,
}
impl State {
    pub(crate) const fn new() -> Self {
        Self {
            tx: crate::interrupt::EventState::new(),
            rx: crate::interrupt::EventState::new(),
        }
    }
}
include!(concat!(env!("OUT_DIR"), "/_generated_uart.rs"));

/// Payload width, excluding any hardware-generated parity bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    DataBits8,
    /// Use the `u16` methods. Nine payload bits cannot be combined with parity.
    DataBits9,
}
/// Hardware parity selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    ParityNone,
    ParityEven,
    ParityOdd,
}
/// Stop-bit lengths implemented by both CW32 UART IPs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    STOP1,
    STOP1P5,
    STOP2,
}
/// Validated PCLK source encodings. Both select the same fixed HAL bus clock.
/// LSI/LSE are intentionally absent because the HAL does not own those clocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockSource {
    Pclk,
    PclkAlt,
}
/// Asynchronous serial configuration. Baud generation uses 16x oversampling.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    pub baudrate: u32,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub clock_source: ClockSource,
    pub rx_pull: Pull,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            data_bits: DataBits::DataBits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::STOP1,
            clock_source: ClockSource::Pclk,
            rx_pull: Pull::None,
        }
    }
}
/// Configuration is checked before the UART clock or registers are changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    BaudrateTooLow,
    BaudrateTooHigh,
    DataParityNotSupported,
}
/// Receive errors, plus a byte-API width mismatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    Framing,
    Parity,
    /// CW32L012 only; CW32F030 has no hardware overrun flag.
    Overrun,
    /// CW32L012 only; CW32F030 has no hardware noise flag.
    Noise,
    /// A byte method was called for nine-bit payloads; use its `u16` variant.
    DataBitsMismatch,
}

// Each backend maps its own frame format, source selection, errors and interrupt
// enables. Workflows below do not know hardware flag encodings or chip names.
trait UartRegisters {
    fn configure(self, config: &Config, divisor: u32);
    fn receive(self) -> Result<Option<u16>, Error>;
    fn rx_pending(self) -> bool;
    fn rx_interrupt(self, enable: bool);
    fn interrupt_pending(self) -> (bool, bool);
}

fn divisor<I: Instance>(config: &Config) -> Result<u32, ConfigError> {
    if config.data_bits == DataBits::DataBits9 && config.parity != Parity::ParityNone {
        return Err(ConfigError::DataParityNotSupported);
    }
    if config.baudrate == 0 {
        return Err(ConfigError::BaudrateTooLow);
    }
    let divisor = (u64::from(I::bus_frequency()) + u64::from(config.baudrate) / 2)
        / u64::from(config.baudrate);
    // Both reference manuals require BRRI in 1..=65535 and BRRF in 0..=15.
    if divisor < 16 {
        return Err(ConfigError::BaudrateTooHigh);
    }
    if divisor > 0xfffff {
        return Err(ConfigError::BaudrateTooLow);
    }
    Ok(divisor as u32)
}
fn initialize<I: Instance>(config: &Config, divisor: u32) -> crate::rcc::ClockGuard {
    let clock = I::acquire();
    critical_section::with(|_| {
        I::regs().configure(config, divisor);
        I::state().tx.reset();
        I::state().rx.reset();
    });
    clock
}
fn enable_interrupt<I: Instance>() {
    // Never unpend or disable a vector: another peripheral or a software
    // executor may share it. The constructor's Binding proves our dispatch.
    unsafe { I::Interrupt::enable() };
}
fn tx_pin<'d, I: Instance, P: TxPin<I>>(pin: Peri<'d, P>) -> Flex<'d> {
    let mut pin = Flex::new(pin);
    pin.set_high();
    pin.set_as_af_unchecked(
        P::AF,
        AfType::output(
            OutputType::PushPull,
            #[cfg(gpio_has_speed)]
            crate::gpio::Speed::High,
        ),
    );
    pin
}
fn rx_pin<'d, I: Instance, P: RxPin<I>>(pin: Peri<'d, P>, pull: Pull) -> Flex<'d> {
    let mut pin = Flex::new(pin);
    pin.set_as_af_unchecked(P::AF, AfType::input(pull));
    pin
}
fn byte_mode(nine_bits: bool) -> Result<(), Error> {
    if nine_bits {
        Err(Error::DataBitsMismatch)
    } else {
        Ok(())
    }
}
fn write_word<I: Instance>(word: u16) -> bool {
    if !I::regs().isr().read().txe() {
        return false;
    }
    // TC is sticky; clear it before each newly queued word so flush never
    // accepts an earlier transmission's completion. TXE clears on the TDR write.
    I::regs().icr().write(|w| w.set_tc(false));
    I::regs().tdr().write(|w| w.set_tdr(word));
    true
}

/// Full-duplex UART owner. Split halves retain independent clock and pin owners.
pub struct Uart<'d, I: Instance, M: Mode> {
    tx: UartTx<'d, I, M>,
    rx: UartRx<'d, I, M>,
}
/// Transmit half. Call `blocking_flush` or `flush` before dropping to finish
/// queued data; dropping immediately disables TX and may truncate a frame.
pub struct UartTx<'d, I: Instance, M: Mode> {
    _instance: Peri<'d, I>,
    _pin: Flex<'d>,
    _clock: crate::rcc::ClockGuard,
    nine_bits: bool,
    _mode: PhantomData<M>,
}
/// Receive half. The hardware holding register can be overwritten between reads.
pub struct UartRx<'d, I: Instance, M: Mode> {
    _instance: Peri<'d, I>,
    _pin: Flex<'d>,
    _clock: crate::rcc::ClockGuard,
    nine_bits: bool,
    _mode: PhantomData<M>,
}
impl<'d, I: Instance, M: Mode> Uart<'d, I, M> {
    fn new_inner<TX: TxPin<I>, RX: RxPin<I>>(
        instance: Peri<'d, I>,
        tx: Peri<'d, TX>,
        rx: Peri<'d, RX>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let divisor = divisor::<I>(&config)?;
        let clock = initialize::<I>(&config, divisor);
        let tx_pin = tx_pin::<I, _>(tx);
        let rx_pin = rx_pin::<I, _>(rx, config.rx_pull);
        let rx_clock = clock.retain();
        // The two handles access disjoint TX/RX data, enables, event state and
        // pins. Shared register RMWs are serialized, and neither half resets or
        // reconfigures shared frame/clock settings after this split.
        let rx_instance = unsafe { instance.clone_unchecked() };
        critical_section::with(|_| {
            I::regs().cr1().modify(|w| {
                w.set_txen(true);
                w.set_rxen(true);
            })
        });
        Ok(Self {
            tx: UartTx {
                _instance: instance,
                _pin: tx_pin,
                _clock: clock,
                nine_bits: config.data_bits == DataBits::DataBits9,
                _mode: PhantomData,
            },
            rx: UartRx {
                _instance: rx_instance,
                _pin: rx_pin,
                _clock: rx_clock,
                nine_bits: config.data_bits == DataBits::DataBits9,
                _mode: PhantomData,
            },
        })
    }
    /// Consume the owner into independently usable, independently droppable halves.
    pub fn split(self) -> (UartTx<'d, I, M>, UartRx<'d, I, M>) {
        (self.tx, self.rx)
    }
    /// Borrow both halves without duplicating peripheral or pin ownership.
    pub fn split_ref(&mut self) -> (&mut UartTx<'d, I, M>, &mut UartRx<'d, I, M>) {
        (&mut self.tx, &mut self.rx)
    }
    pub fn blocking_write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.tx.blocking_write(bytes)
    }
    pub fn blocking_write_u16(&mut self, words: &[u16]) -> Result<(), Error> {
        self.tx.blocking_write_u16(words)
    }
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        self.tx.blocking_flush()
    }
    pub fn blocking_read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        self.rx.blocking_read(bytes)
    }
    pub fn blocking_read_u16(&mut self, words: &mut [u16]) -> Result<(), Error> {
        self.rx.blocking_read_u16(words)
    }
    pub fn is_write_ready(&self) -> bool {
        self.tx.is_write_ready()
    }
    pub fn try_write(&mut self, byte: u8) -> Result<bool, Error> {
        self.tx.try_write(byte)
    }
    pub fn try_write_u16(&mut self, word: u16) -> bool {
        self.tx.try_write_u16(word)
    }
    pub fn try_read(&mut self) -> Result<Option<u8>, Error> {
        self.rx.try_read()
    }
    pub fn try_read_u16(&mut self) -> Result<Option<u16>, Error> {
        self.rx.try_read_u16()
    }
}
impl<'d, I: Instance> Uart<'d, I, Blocking> {
    pub fn new_blocking(
        instance: Peri<'d, I>,
        tx: Peri<'d, impl TxPin<I>>,
        rx: Peri<'d, impl RxPin<I>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::new_inner(instance, tx, rx, config)
    }
}
impl<'d, I: Instance> Uart<'d, I, Async> {
    pub fn new(
        instance: Peri<'d, I>,
        tx: Peri<'d, impl TxPin<I>>,
        rx: Peri<'d, impl RxPin<I>>,
        _irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let driver = Self::new_inner(instance, tx, rx, config)?;
        enable_interrupt::<I>();
        Ok(driver)
    }
    pub async fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.tx.write(bytes).await
    }
    pub async fn write_u16(&mut self, words: &[u16]) -> Result<(), Error> {
        self.tx.write_u16(words).await
    }
    pub async fn flush(&mut self) -> Result<(), Error> {
        self.tx.flush().await
    }
    pub async fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        self.rx.read(bytes).await
    }
    pub async fn read_u16(&mut self, words: &mut [u16]) -> Result<(), Error> {
        self.rx.read_u16(words).await
    }
}
impl<'d, I: Instance, M: Mode> UartTx<'d, I, M> {
    fn new_inner(
        instance: Peri<'d, I>,
        tx: Peri<'d, impl TxPin<I>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let divisor = divisor::<I>(&config)?;
        let clock = initialize::<I>(&config, divisor);
        let pin = tx_pin::<I, _>(tx);
        I::regs().cr1().modify(|w| w.set_txen(true));
        Ok(Self {
            _instance: instance,
            _pin: pin,
            _clock: clock,
            nine_bits: config.data_bits == DataBits::DataBits9,
            _mode: PhantomData,
        })
    }
    /// True when one word can be queued without waiting. It does not mean the
    /// last stop bit has left the pin; use flush for physical completion.
    pub fn is_write_ready(&self) -> bool {
        I::regs().isr().read().txe()
    }
    /// Queue one byte, returning false without writing when TDR is occupied.
    pub fn try_write(&mut self, byte: u8) -> Result<bool, Error> {
        byte_mode(self.nine_bits)?;
        Ok(self.try_write_u16(u16::from(byte)))
    }
    /// Queue the low eight or nine payload bits, according to the configuration.
    pub fn try_write_u16(&mut self, word: u16) -> bool {
        critical_section::with(|_| {
            write_word::<I>(word & if self.nine_bits { 0x1ff } else { 0xff })
        })
    }
    /// Queue all bytes. The final byte may still be on the wire at return.
    pub fn blocking_write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        byte_mode(self.nine_bits)?;
        for &byte in bytes {
            while !self.try_write_u16(u16::from(byte)) {
                core::hint::spin_loop();
            }
        }
        Ok(())
    }
    pub fn blocking_write_u16(&mut self, words: &[u16]) -> Result<(), Error> {
        for &word in words {
            while !self.try_write_u16(word) {
                core::hint::spin_loop();
            }
        }
        Ok(())
    }
    /// Wait until both TDR and the shift register are empty, including stop bits.
    pub fn blocking_flush(&mut self) -> Result<(), Error> {
        while I::regs().isr().read().txbusy() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}
impl<'d, I: Instance> UartTx<'d, I, Blocking> {
    pub fn new_blocking(
        instance: Peri<'d, I>,
        tx: Peri<'d, impl TxPin<I>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::new_inner(instance, tx, config)
    }
}
impl<'d, I: Instance> UartTx<'d, I, Async> {
    pub fn new(
        instance: Peri<'d, I>,
        tx: Peri<'d, impl TxPin<I>>,
        _irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let driver = Self::new_inner(instance, tx, config)?;
        enable_interrupt::<I>();
        Ok(driver)
    }
}
impl<'d, I: Instance, M: Mode> UartRx<'d, I, M> {
    fn new_inner(
        instance: Peri<'d, I>,
        rx: Peri<'d, impl RxPin<I>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let divisor = divisor::<I>(&config)?;
        let clock = initialize::<I>(&config, divisor);
        let pin = rx_pin::<I, _>(rx, config.rx_pull);
        I::regs().cr1().modify(|w| w.set_rxen(true));
        Ok(Self {
            _instance: instance,
            _pin: pin,
            _clock: clock,
            nine_bits: config.data_bits == DataBits::DataBits9,
            _mode: PhantomData,
        })
    }
    /// Return one byte if available. An error discards that received word,
    /// acknowledges RC and the observed error flags, and permits the next read.
    pub fn try_read(&mut self) -> Result<Option<u8>, Error> {
        byte_mode(self.nine_bits)?;
        Ok(self.try_read_u16()?.map(|word| word as u8))
    }
    pub fn try_read_u16(&mut self) -> Result<Option<u16>, Error> {
        critical_section::with(|_| {
            I::regs()
                .receive()
                .map(|word| word.map(|word| word & if self.nine_bits { 0x1ff } else { 0xff }))
        })
    }
    /// Fill the whole buffer or stop at the first receive error. A completed
    /// prefix remains in the buffer on error; its length is not returned.
    pub fn blocking_read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        byte_mode(self.nine_bits)?;
        for byte in bytes {
            loop {
                if let Some(word) = self.try_read_u16()? {
                    *byte = word as u8;
                    break;
                }
                core::hint::spin_loop();
            }
        }
        Ok(())
    }
    pub fn blocking_read_u16(&mut self, words: &mut [u16]) -> Result<(), Error> {
        for word in words {
            loop {
                if let Some(value) = self.try_read_u16()? {
                    *word = value;
                    break;
                }
                core::hint::spin_loop();
            }
        }
        Ok(())
    }
}
impl<'d, I: Instance> UartRx<'d, I, Blocking> {
    pub fn new_blocking(
        instance: Peri<'d, I>,
        rx: Peri<'d, impl RxPin<I>>,
        config: Config,
    ) -> Result<Self, ConfigError> {
        Self::new_inner(instance, rx, config)
    }
}
impl<'d, I: Instance> UartRx<'d, I, Async> {
    pub fn new(
        instance: Peri<'d, I>,
        rx: Peri<'d, impl RxPin<I>>,
        _irq: impl crate::interrupt::typelevel::Binding<I::Interrupt, InterruptHandler<I>> + 'd,
        config: Config,
    ) -> Result<Self, ConfigError> {
        let driver = Self::new_inner(instance, rx, config)?;
        enable_interrupt::<I>();
        Ok(driver)
    }
}
impl<I: Instance, M: Mode> Drop for UartTx<'_, I, M> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            I::regs().ier().modify(|w| {
                w.set_txe(false);
                w.set_tc(false);
            });
            I::regs().cr1().modify(|w| w.set_txen(false));
            I::state().tx.reset();
        });
        // Flex disconnects before ClockGuard releases this half's clock lease.
    }
}
impl<I: Instance, M: Mode> Drop for UartRx<'_, I, M> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            I::regs().rx_interrupt(false);
            I::regs().cr1().modify(|w| w.set_rxen(false));
            I::state().rx.reset();
        });
    }
}
