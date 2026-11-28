use super::*;

/// Eight data bits are supported with no, even, or odd parity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parity {
    /// Eight data bits without a parity bit.
    ParityNone,
    /// Eight data bits followed by even parity.
    ParityEven,
    /// Eight data bits followed by odd parity.
    ParityOdd,
}

/// Stop-bit length supported by the hardware.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopBits {
    /// One stop bit.
    STOP1,
    /// One and a half stop bits.
    STOP1P5,
    /// Two stop bits.
    STOP2,
}

/// Receiver oversampling. Lower factors permit higher baud rates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oversampling {
    /// Sixteen samples, with a four-bit fractional divider.
    Oversampling16,
    /// Eight samples, with an integer divider.
    Oversampling8,
    /// Four samples, with an integer divider.
    Oversampling4,
}

/// Payload size, excluding an optional parity bit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DataBits {
    /// Eight data bits, with optional parity.
    DataBits8,
    /// Nine data bits without parity. Qualified only for L010/L011/L012 ordinary PIO.
    DataBits9,
}

impl DataBits {
    pub(super) fn check_byte(self) -> Result<(), Error> {
        if self == Self::DataBits8 {
            Ok(())
        } else {
            Err(Error::WordSizeMismatch)
        }
    }
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    pub(super) fn mask(self) -> u16 {
        match self {
            Self::DataBits8 => 0xff,
            Self::DataBits9 => 0x1ff,
        }
    }
}

/// UART configuration. The clock source is the PCLK frozen by HAL initialization.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    /// Requested baud rate in bits per second.
    pub baudrate: u32,
    /// Accept ORE during an active L012 RX DMA block. DMA reads RDR but RC
    /// stays set across bytes. Only use with self-synchronizing, checked frames:
    /// an actual overwritten byte cannot be distinguished from sticky RC.
    /// Framing, parity, noise, pre-transfer overruns and DMA errors still fail.
    #[cfg(all(uart_dma_rx, uart_cw32l012_v1))]
    pub dma_rx_checked_frames: bool,
    /// Data bits. Nine-bit payloads require the L010/L011/L012 ordinary PIO u16 APIs.
    pub data_bits: DataBits,
    /// Parity bit configuration. Nine data bits plus parity are unsupported.
    pub parity: Parity,
    /// Stop-bit length.
    pub stop_bits: StopBits,
    /// Sampling mode.
    pub oversampling: Oversampling,
    /// Maximum divider-rounding error in parts per million, at most 1,000,000.
    /// This excludes the physical oscillator's frequency tolerance.
    pub max_error_ppm: u32,
    /// RX weak pull resistor. The manual recommends pull-up for idle-high UART.
    pub rx_pull: Pull,
    /// CTS weak pull resistor. CTS is active low; pull-up pauses an undriven peer.
    /// Used only by constructors that take a CTS pin.
    pub cts_pull: Pull,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            #[cfg(all(uart_dma_rx, uart_cw32l012_v1))]
            dma_rx_checked_frames: false,
            data_bits: DataBits::DataBits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::STOP1,
            oversampling: Oversampling::Oversampling16,
            max_error_ppm: 20_000,
            rx_pull: Pull::Up,
            cts_pull: Pull::None,
        }
    }
}

/// Configuration was rejected before the UART or pins were changed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigError {
    /// LIN owners require eight data bits, no parity and one stop bit.
    #[cfg(any(cw32l010, cw32l011, cw32l012))]
    InvalidLinFrame,
    /// Native RS485 DE assertion/deassertion delay must fit the five-bit field.
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    InvalidDriverEnableDelay,
    /// The selected family or buffered/DMA backend only qualifies eight data bits.
    UnsupportedDataBits,
    /// Nine data bits cannot be combined with a parity bit.
    InvalidParity,
    /// Successful HAL clock initialization is required first.
    ClockNotConfigured,
    /// A zero baud rate is invalid.
    BaudrateZero,
    /// The selected sampling mode requires a divider below its minimum.
    BaudrateTooHigh,
    /// The baud-rate divider exceeds its sixteen-bit integer part.
    BaudrateTooLow,
    /// Divider rounding exceeds the configured tolerance.
    BaudrateError,
    /// `max_error_ppm` must not exceed 1,000,000.
    InvalidTolerance,
    /// The selected CTS pin does not support the requested pull resistor.
    UnsupportedCtsPull,
    /// The supplied software receive ring has zero capacity.
    RxBufferEmpty,
    /// DMA staging must be nonempty, bounded and wholly inside qualified SRAM.
    #[cfg(any(uart_dma_tx, uart_dma_rx))]
    DmaBuffer(crate::dma::ConfigError),
    /// Normal HAL startup did not admit this one-time safe DMA channel.
    #[cfg(any(uart_dma_tx, uart_dma_rx))]
    DmaChannel(crate::dma::CopyChannelError),
    /// Pull-down is unavailable on every reviewed L012 UART RX route.
    #[cfg(cw32l012)]
    UnsupportedRxPull,
}

/// UART receive errors and safe DMA lease errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A byte API cannot represent the configured nine-bit payload.
    WordSizeMismatch,
    /// u16-container operations are not provided by the selected DMA backend.
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    UnsupportedWordTransfer,
    /// Received parity did not match.
    Parity,
    /// Invalid stop bit / frame structure.
    Framing,
    /// A clean ISR-observed byte could not enter the full software receive ring.
    BufferOverflow,
    /// DMA failed or its completion could not be verified; resources are retained.
    #[cfg(any(uart_dma_tx, uart_dma_rx))]
    Dma(crate::dma::Error),
    /// A previous staged chunk is running. An inherent async operation can settle it.
    #[cfg(any(uart_dma_tx, uart_dma_rx))]
    DmaBusy,
    /// A previous error permanently quarantined this direction's DMA resources.
    #[cfg(any(uart_dma_tx, uart_dma_rx))]
    DmaQuarantined,
    /// Noise was detected. L010/L011/L012 report it only at 16x/8x sampling.
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    Noise,
    /// An unread frame was overwritten by a newer frame; receive data was lost.
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    Overrun,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::WordSizeMismatch => "UART byte API cannot represent nine-bit data",
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::UnsupportedWordTransfer => "UART u16 API requires ordinary PIO",
            Self::Parity => "UART parity error",
            Self::Framing => "UART frame error",
            Self::BufferOverflow => "UART software receive buffer overflow",
            #[cfg(any(uart_dma_tx, uart_dma_rx))]
            Self::Dma(_) => "UART DMA transfer failed",
            #[cfg(any(uart_dma_tx, uart_dma_rx))]
            Self::DmaBusy => "UART DMA chunk is still running",
            #[cfg(any(uart_dma_tx, uart_dma_rx))]
            Self::DmaQuarantined => "UART DMA resources are quarantined",

            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::Noise => "UART noise error",
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::Overrun => "UART receive overrun",
        })
    }
}
impl core::error::Error for Error {}
impl core::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::InvalidDriverEnableDelay => "UART DE delay exceeds 31 sample periods",
            #[cfg(any(cw32l010, cw32l011, cw32l012))]
            Self::InvalidLinFrame => "LIN requires eight data bits, no parity and one stop bit",
            Self::UnsupportedDataBits => {
                "UART data bits are not qualified for this family or backend"
            }
            Self::InvalidParity => "UART nine data bits cannot include parity",
            Self::ClockNotConfigured => "HAL clocks are not initialized",
            Self::BaudrateZero => "UART baud rate must be nonzero",
            Self::BaudrateTooHigh => "UART baud rate exceeds the sampling limit",
            Self::BaudrateTooLow => "UART baud rate requires an excessive divider",
            Self::BaudrateError => "UART baud-rate rounding exceeds the configured tolerance",
            Self::InvalidTolerance => "UART baud-rate tolerance exceeds 100 percent",
            Self::UnsupportedCtsPull => "UART CTS pin does not support the requested pull",
            Self::RxBufferEmpty => "UART receive buffer is empty",
            #[cfg(any(uart_dma_tx, uart_dma_rx))]
            Self::DmaBuffer(_) => "UART DMA staging buffer is invalid",
            #[cfg(any(uart_dma_tx, uart_dma_rx))]
            Self::DmaChannel(_) => "UART DMA channel admission failed",
            #[cfg(cw32l012)]
            Self::UnsupportedRxPull => "UART RX pull-down is unavailable on this device",
        })
    }
}
impl core::error::Error for ConfigError {}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            Self::BufferOverflow | Self::WordSizeMismatch => embedded_io::ErrorKind::Other,
            #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
            Self::UnsupportedWordTransfer => embedded_io::ErrorKind::Other,
            #[cfg(any(uart_dma_tx, uart_dma_rx))]
            Self::DmaBusy | Self::DmaQuarantined | Self::Dma(_) => embedded_io::ErrorKind::Other,
            _ => embedded_io::ErrorKind::InvalidData,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Baud {
    integer: u16,
    fraction: u8,
    over: vals::Over,
    pub(super) actual: u32,
}
pub(super) fn configured_baud<T: Instance>(config: Config) -> Result<Baud, ConfigError> {
    validate_config(config)?;
    // Configuration selects SOURCE=0 (PCLK) after preflight. Do not read a
    // peripheral-local mux before its clock has been enabled.
    let clock = crate::rcc::bus_frequency::<T>().ok_or(ConfigError::ClockNotConfigured)?;
    calculate_baud(clock.0, config)
}
fn validate_config(config: Config) -> Result<(), ConfigError> {
    if config.data_bits == DataBits::DataBits9 {
        #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1)))]
        return Err(ConfigError::UnsupportedDataBits);
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        if config.parity != Parity::ParityNone {
            return Err(ConfigError::InvalidParity);
        }
    }

    #[cfg(cw32l012)]
    if config.rx_pull == Pull::Down {
        return Err(ConfigError::UnsupportedRxPull);
    }
    let _ = config;
    Ok(())
}
pub(super) fn require_byte_config(config: Config) -> Result<(), ConfigError> {
    if config.data_bits == DataBits::DataBits8 {
        Ok(())
    } else {
        Err(ConfigError::UnsupportedDataBits)
    }
}

pub(super) fn validate_cts_pull(pin: Option<&AnyPin>, pull: Pull) -> Result<(), ConfigError> {
    #[cfg(not(any(gpio_cw32l010_v1, gpio_cw32l011_v1)))]
    if let Some(pin) = pin {
        if pull == Pull::Down
            && crate::GPIO_PULL_DOWN_MASKS[pin.port() as usize] & (1 << pin.pin()) == 0
        {
            return Err(ConfigError::UnsupportedCtsPull);
        }
    }
    let _ = (pin, pull);
    Ok(())
}

fn calculate_baud(clock: u32, config: Config) -> Result<Baud, ConfigError> {
    if config.baudrate == 0 {
        return Err(ConfigError::BaudrateZero);
    }
    if config.max_error_ppm > 1_000_000 {
        return Err(ConfigError::InvalidTolerance);
    }
    let clock = u64::from(clock);
    let baud = u64::from(config.baudrate);
    let (factor, over) = match config.oversampling {
        Oversampling::Oversampling16 => (16, vals::Over::Over16),
        Oversampling::Oversampling8 => (8, vals::Over::Over8),
        Oversampling::Oversampling4 => (4, vals::Over::Over4),
    };
    // Check the requested frequency itself, before rounding into a legal divider.
    if baud * factor > clock {
        return Err(ConfigError::BaudrateTooHigh);
    }
    let (integer, fraction, divisor) = if factor == 16 {
        let divisor = (clock + baud / 2) / baud;
        (divisor / 16, divisor % 16, divisor)
    } else {
        let integer = (clock + baud * factor / 2) / (baud * factor);
        (integer, 0, integer * factor)
    };
    if integer > 65_535 {
        return Err(ConfigError::BaudrateTooLow);
    }
    if integer == 0 {
        return Err(ConfigError::BaudrateTooHigh);
    }
    // Once the rounded divider is in range, baud * divisor is near `clock`,
    // so these u64 products are bounded even for a u32 clock input.
    let requested = baud * divisor;
    if clock.abs_diff(requested) * 1_000_000 > requested * u64::from(config.max_error_ppm) {
        return Err(ConfigError::BaudrateError);
    }
    Ok(Baud {
        integer: integer as u16,
        fraction: fraction as u8,
        over,
        actual: ((clock + divisor / 2) / divisor) as u32,
    })
}

pub(super) fn configure(regs: pac::uart::Uart, config: Config, baud: Baud) {
    regs.ier().write(|w| {
        w.set_txe(false);
        w.set_tc(false);
        w.set_rc(false);
        w.set_fe(false);
        w.set_pe(false);
        w.set_cts(false);
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        {
            w.set_rxidle(false);
            w.set_rxbrk(false);
            w.set_baud(false);
            w.set_timov(false);
            w.set_ne(false);
            w.set_ore(false);
            w.set_rxmatch(false);
        }
        #[cfg(any(uart_cw32l031_v1, uart_cw32l052_v1))]
        {
            w.set_timov(false);
            w.set_baud(false);
            w.set_rxbrk(false);
        }
    });
    regs.cr2().write(|w| {
        w.set_addren(false);
        w.set_ctsen(false);
        w.set_rtsen(false);
        w.set_rxinv(false);
        w.set_txinv(false);
        #[cfg(not(any(uart_cw32l010_v1, uart_cw32f002_v1)))]
        {
            w.set_dmarx(false);
            w.set_dmatx(false);
        }
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        {
            w.set_rxmatchen(false);
            w.set_swap(false);
            w.set_adcrx(false);
            w.set_adctx(false);
            w.set_loop_(false);
            w.set_rxsrc(0);
        }
        #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1)))]
        {
            w.set_signal(false);
            #[cfg(not(uart_cw32l052_v1))]
            w.set_source(vals::Source::Pclk);
            #[cfg(uart_cw32l052_v1)]
            w.set_sorce(vals::Source::Pclk);
        }
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1, uart_cw32l031_v1, uart_cw32l052_v1))]
        w.set_timcr(0);
    });
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    regs.cr3().write(|w| {
        w.set_dem(false);
        w.set_dep(false);
        w.set_detime(0);
        w.set_lin(false);
        w.set_brkl(false);
    });
    regs.brri().write(|w| w.set_brri(baud.integer));
    regs.brrf().write(|w| w.set_brrf(baud.fraction));
    regs.cr1().write(|w| {
        w.set_txen(false);
        w.set_rxen(false);
        w.set_sync(false);
        w.set_start(false);
        w.set_over(baud.over);
        #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
        {
            // CHLEN counts the parity bit. Eight data bits with parity need
            // nine total bits; CHLEN=0 automatically clears PARITYEN.
            w.set_chlen(
                config.data_bits == DataBits::DataBits9 || config.parity != Parity::ParityNone,
            );
            w.set_parityen(config.parity != Parity::ParityNone);
            w.set_parity(if config.parity == Parity::ParityOdd {
                vals::Parity::Odd
            } else {
                vals::Parity::Even
            });
            w.set_msbf(false);
            w.set_signal(false);
            w.set_source(vals::Source::Pclk);
        }
        #[cfg(not(any(uart_cw32l010_v1, uart_cw32l012_v1)))]
        w.set_parity(match config.parity {
            Parity::ParityNone => vals::Parity::None,
            Parity::ParityEven => vals::Parity::Even,
            Parity::ParityOdd => vals::Parity::Odd,
        });
        #[cfg(uart_cw32l031_v1)]
        {
            w.set_lin(false);
            w.set_txbrk(false);
            w.set_brkl(false);
        }
        #[cfg(uart_cw32l052_v1)]
        {
            w.set_linen(false);
            w.set_txbrk(false);
            w.set_brklen(false);
        }
        w.set_stop(match config.stop_bits {
            StopBits::STOP1 => vals::Stop::Stop1,
            StopBits::STOP1P5 => vals::Stop::Stop1p5,
            StopBits::STOP2 => vals::Stop::Stop2,
        });
    });
    // Initialization clears all implemented event commands. The generated
    // no-op preserves reserved bits at their own-family documented values.
    let mut command = regs::Icr::write_noop();
    command.set_tc(false);
    command.set_rc(false);
    command.set_fe(false);
    command.set_pe(false);
    command.set_cts(false);
    #[cfg(any(uart_cw32l010_v1, uart_cw32l012_v1))]
    {
        command.set_rxidle(false);
        command.set_rxbrk(false);
        command.set_baud(false);
        command.set_timov(false);
        command.set_ne(false);
        command.set_ore(false);
        command.set_rxmatch(false);
    }
    #[cfg(any(uart_cw32l031_v1, uart_cw32l052_v1))]
    {
        command.set_timov(false);
        command.set_baud(false);
        command.set_rxbrk(false);
    }
    regs.icr().write_value(command);
}
