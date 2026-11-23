use super::*;

/// SPI configuration error. Invalid configurations do not touch hardware.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ConfigError {
    /// Call [`crate::init`] successfully before constructing a driver.
    ClockNotInitialized,
    /// The requested frequency must be nonzero.
    FrequencyZero,
    /// The request exceeds the family master limit or minimum clock divisor.
    /// x030: 16 MHz; F002/F003/F020 and L031-derived IP: 12 MHz;
    /// L010/L011/L012: 24 MHz. L031-derived IP additionally requires a divisor
    /// of at least 4; all other supported IP requires a divisor of at least 2.
    /// The clock-relative request range uses nominal PCLK, separately from
    /// actual-clock safety.
    FrequencyTooHigh,
    /// The largest divider (/128 on old IP, /256 on L010/L011/L012) is too fast.
    FrequencyTooLow,
    /// This IP does not implement the requested sampling-delay semantics.
    UnsupportedSampleDelay,
    /// The requested weak pull is unavailable on the peripheral input routes.
    UnsupportedInputPull,
    /// The bus must be idle before changing its configuration.
    Busy,
    /// Invalid private DMA staging; constructor inputs are consumed on error.
    #[cfg(spi_dma)]
    DmaBuffer(dma::ConfigError),
    /// The physical DMA channel was not admitted by clean HAL initialization.
    #[cfg(spi_dma)]
    DmaChannel(dma::CopyChannelError),
    /// DMA resources were permanently retained after an unconfirmed transfer.
    #[cfg(spi_dma)]
    DmaQuarantined,
}

impl core::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::ClockNotInitialized => "SPI peripheral clock is not initialized",
            Self::FrequencyZero => "SPI frequency must be nonzero",
            Self::FrequencyTooHigh => "SPI frequency exceeds the supported maximum",
            Self::FrequencyTooLow => "SPI frequency is below the supported divider range",
            Self::UnsupportedSampleDelay => "SPI sampling delay is unsupported on this IP",
            Self::UnsupportedInputPull => "SPI input pull is unsupported on this IP",
            Self::Busy => "SPI bus is busy",
            #[cfg(spi_dma)]
            Self::DmaBuffer(_) => "invalid SPI DMA staging buffer",
            #[cfg(spi_dma)]
            Self::DmaChannel(_) => "SPI DMA channel admission failed",
            #[cfg(spi_dma)]
            Self::DmaQuarantined => "SPI DMA resources are quarantined",
        })
    }
}
impl core::error::Error for ConfigError {}

/// SPI transfer error.
///
/// PIO errors clear buffers/flags and re-enable the configured controller.
/// DMA errors permanently quarantine the paired resources without resetting or
/// flushing hardware. Earlier completed receive chunks may already be visible.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// A bounded blocking transfer exhausted its polling budget.
    PollTimeout,
    /// Another master was detected on the bus.
    ModeFault,
    /// A received frame overwrote unread data.
    Overrun,
    /// A transmit frame was not available (normally a slave-only error).
    Underrun,
    /// Chip select changed during a frame (normally a slave-only error).
    ChipSelectFault,
    /// A synchronous call cannot settle a cancelled/forgotten async PIO frame.
    /// Keep the original CS selected and await a successful async flush.
    #[cfg(spi_async_pio)]
    PioBusy,
    /// A DMA channel reported an error or ambiguous terminal state.
    #[cfg(spi_dma)]
    Dma(dma::Error),
    /// A blocking/configuration call cannot settle an outstanding DMA lease.
    #[cfg(spi_dma)]
    DmaBusy,
    /// The paired resources are permanently retained after an error.
    #[cfg(spi_dma)]
    DmaQuarantined,
    /// The selected DMA backend only accepts byte frames. No transfer or
    /// settlement occurs on this rejection, including for empty buffers.
    #[cfg(spi_dma)]
    UnsupportedWordSize,
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::PollTimeout => "SPI polling timeout",
            Self::ModeFault => "SPI mode fault",
            Self::Overrun => "SPI receive overrun",
            Self::Underrun => "SPI transmit underrun",
            Self::ChipSelectFault => "SPI chip-select fault",
            #[cfg(spi_async_pio)]
            Self::PioBusy => "SPI async frame is still pending",
            #[cfg(spi_dma)]
            Self::Dma(_) => "SPI DMA transfer failed",
            #[cfg(spi_dma)]
            Self::DmaBusy => "SPI DMA transfer is still running",
            #[cfg(spi_dma)]
            Self::DmaQuarantined => "SPI DMA resources are quarantined",
            #[cfg(spi_dma)]
            Self::UnsupportedWordSize => "SPI DMA only supports byte frames",
        })
    }
}
impl core::error::Error for Error {}
impl embedded_hal::spi::Error for Error {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        use embedded_hal::spi::ErrorKind;
        match self {
            Self::ModeFault => ErrorKind::ModeFault,
            Self::Overrun => ErrorKind::Overrun,
            Self::ChipSelectFault => ErrorKind::ChipSelectFault,
            Self::Underrun | Self::PollTimeout => ErrorKind::Other,
            #[cfg(spi_async_pio)]
            Self::PioBusy => ErrorKind::Other,
            #[cfg(spi_dma)]
            Self::Dma(_) | Self::DmaBusy | Self::DmaQuarantined | Self::UnsupportedWordSize => {
                ErrorKind::Other
            }
        }
    }
}

/// SPI bit order.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitOrder {
    /// Least significant bit first.
    LsbFirst,
    /// Most significant bit first.
    MsbFirst,
}

/// CW32 master input sampling point.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SampleDelay {
    /// Sample at the edge selected by CPOL and CPHA.
    None,
    /// Delay input sampling by half an SCK period (CR1.SMP=1).
    ///
    /// Use only when the slave's output timing and board timing require it.
    /// Unsupported on L010/L011/L012: their SMP bit does not promise half a
    /// period. These backends accept only [`Self::None`].
    HalfPeriod,
}

/// SPI bus configuration. Frame width is selected by the transfer's word type.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Config {
    /// Clock polarity and phase.
    pub mode: Mode,
    /// On-wire bit order.
    pub bit_order: BitOrder,
    /// Maximum actual SCK frequency under the selected RCC source bounds. The
    /// fastest supported divisor whose entire envelope does not exceed this
    /// request is selected; unsupported limits return an error. The bounds
    /// require the documented ambient temperature, supply and factory trim.
    /// Nominal SCK may therefore be lower than with an ideal input clock.
    pub frequency: Hertz,
    /// Weak pull resistor on MISO. L012 has no pull-down on any SPI input route.
    pub input_pull: Pull,
    /// Optional CW32-specific delayed sampling.
    pub sample_delay: SampleDelay,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            mode: MODE_0,
            bit_order: BitOrder::MsbFirst,
            frequency: Hertz(1_000_000),
            input_pull: Pull::None,
            sample_delay: SampleDelay::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Prescaler {
    pub(super) bits: u8,
    pub(super) frequency_bounds: ClockBounds,
}

impl Prescaler {
    // Encodings are source-backed, not inferred from the field width.
    pub(super) const MAX_BITS: u8 = if cfg!(spi_cw32l012_v1) {
        127
    } else if cfg!(spi_cw32l010_v1) {
        7
    } else {
        6
    };
    pub(super) const fn divisor(bits: u8) -> u32 {
        if cfg!(spi_cw32l012_v1) {
            2 * (bits as u32 + 1)
        } else {
            2 << bits
        }
    }
}

impl Config {
    pub(super) fn sck_pull(&self) -> Pull {
        match self.mode.polarity {
            #[cfg(not(any(spi_cw32l010_v1, spi_cw32l012_v1)))]
            Polarity::IdleLow => Pull::Down,
            // L010/L011 have no pull-down; L012 supports it only on PF3, which
            // has no serial route. The peripheral actively drives idle-low SCK.
            #[cfg(any(spi_cw32l010_v1, spi_cw32l012_v1))]
            Polarity::IdleLow => Pull::None,
            Polarity::IdleHigh => Pull::Up,
        }
    }
}
