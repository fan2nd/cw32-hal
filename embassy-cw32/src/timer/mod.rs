//! Owned PCLK timers and single-ended PWM on audited timer IPs.
//!
//! Only edge-aligned upcounting is exposed. Constructors stop the timer; DMA,
//! interrupts, external/slave clocks and ADC triggering are not enabled. The
//! separate [`crate::atim`] driver owns complementary/break-protected PWM.

use crate::{gpio::Pin, pac, peripherals, rcc::PeripheralClock, PeripheralType};

#[cfg(all(atim_f030, gtim_f030))]
mod f030;
#[cfg(all(atim_l012, gtim_l012))]
mod l012;
pub mod low_level;
pub mod simple_pwm;

#[derive(Clone, Copy)]
pub(crate) enum Registers {
    Atim(pac::atim::Atim),
    Gtim(pac::gtim::Gtim),
}

pub(crate) mod sealed {
    pub(crate) trait Instance {
        fn regs() -> super::Registers;
    }
    pub trait PwmInstance {}
    pub trait Channel {}
    pub trait Pin<T, C> {}
}

/// A metadata-generated, exclusively owned 16-bit PCLK timer instance.
#[allow(private_bounds)]
pub trait CoreInstance: PeripheralType + PeripheralClock + sealed::Instance {}
/// A timer with an audited single-ended output implementation.
#[allow(private_bounds)]
pub trait PwmInstance: CoreInstance + sealed::PwmInstance {}

/// A hardware comparison channel. A channel needs a verified pin to be enabled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Channel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
}
impl Channel {
    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}
/// Sealed type-level channel identity.
#[allow(private_bounds)]
pub trait TimerChannel: sealed::Channel {
    const CHANNEL: Channel;
}
macro_rules! channel {
    ($($name:ident),*) => {$ (
        #[doc = concat!("Timer ", stringify!($name), " marker.")]
        pub enum $name {}
        impl sealed::Channel for $name {}
        impl TimerChannel for $name { const CHANNEL: Channel = Channel::$name; }
    )*};
}
channel!(Ch1, Ch2, Ch3, Ch4);

/// An audited chip-level main-output AF route for this timer and channel.
/// The board must also confirm that its package exposes the selected pin.
#[allow(private_bounds)]
pub trait TimerPin<T: PwmInstance, C: TimerChannel>: Pin + sealed::Pin<T, C> {
    const AF: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_timer.rs"));

/// Invalid requests are rejected before changing hardware.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidFrequency,
    InvalidPrescaler,
    InvalidPeriod,
    CounterOutOfRange,
    DutyOutOfRange,
    ChannelUnavailable,
}
impl embedded_hal::pwm::Error for Error {
    fn kind(&self) -> embedded_hal::pwm::ErrorKind {
        embedded_hal::pwm::ErrorKind::Other
    }
}

/// Actual timing, using the internally validated PCLK and exact integer divisors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frequency {
    clock_hz: u32,
    config: low_level::Config,
}
impl Frequency {
    /// Validated timer input clock, in hertz.
    pub const fn clock_hz(self) -> u32 {
        self.clock_hz
    }
    pub const fn config(self) -> low_level::Config {
        self.config
    }
    /// Exact frequency: numerator / denominator hertz, including rates below 1 Hz.
    pub const fn ratio(self) -> (u32, u64) {
        (
            self.clock_hz,
            self.config.prescaler_divisor as u64 * self.config.period_ticks as u64,
        )
    }
    /// Whole hertz, rounded downward. Use `ratio` for lossless reporting.
    pub const fn hz(self) -> u32 {
        (self.clock_hz as u64 / self.ratio().1) as u32
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Prescalers {
    #[cfg(atim_l012)]
    Linear,
    #[cfg(atim_f030)]
    AtimF030,
    #[cfg(gtim_f030)]
    GtimF030,
}
impl Prescalers {
    fn validate(self, divisor: u32) -> bool {
        match self {
            #[cfg(atim_l012)]
            Self::Linear => (1..=65536).contains(&divisor),
            #[cfg(atim_f030)]
            Self::AtimF030 => matches!(divisor, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 256),
            #[cfg(gtim_f030)]
            Self::GtimF030 => divisor.is_power_of_two() && divisor <= 32768,
        }
    }
    fn at_least(self, minimum: u32) -> Option<u32> {
        match self {
            #[cfg(atim_l012)]
            Self::Linear => (minimum <= 65536).then_some(minimum.max(1)),
            #[cfg(atim_f030)]
            Self::AtimF030 => [1, 2, 4, 8, 16, 32, 64, 256]
                .into_iter()
                .find(|&d| d >= minimum),
            #[cfg(gtim_f030)]
            Self::GtimF030 => (0..=15).map(|n| 1 << n).find(|&d| d >= minimum),
        }
    }
}
