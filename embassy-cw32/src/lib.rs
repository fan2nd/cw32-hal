//! Metadata-driven CW32L012C8 and CW32F030C8 HAL, backed by the generated cw32-metapac crate.
//! Peripheral and pin identities are emitted from metadata by build.rs; the PAC consumes chip/register
//! JSON compiled from layered source data by the separate data generator.
//! No STM32 register compatibility is assumed.
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(adc)]
pub mod adc;
#[cfg(any(bgr, dac, vcref, opa, vc))]
pub mod analog;
#[cfg(atim)]
pub mod atim;
#[cfg(cordic)]
pub mod cordic;
#[cfg(crc)]
pub mod crc;
#[cfg(dma)]
pub mod dma;
#[cfg(eau)]
pub mod eau;
#[cfg(flash)]
pub mod flash;
#[cfg(gpio)]
pub mod gpio;
#[cfg(i2c)]
pub mod i2c;
pub mod interrupt;
#[cfg(any(adc_l012, atim_l012, btim_l012, gpio_l012, all(opa_l012, bgr_l012)))]
pub mod motor;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "unstable-pac")]
pub use cw32_metapac as pac;
#[cfg(not(feature = "unstable-pac"))]
pub(crate) use cw32_metapac as pac;
pub use embassy_hal_internal::{Peri, PeripheralType};

/// Driver operation modes. Modes are sealed so interrupt requirements cannot be bypassed.
pub mod mode {
    mod sealed {
        pub trait Sealed {}
    }
    /// Driver mode selected by its constructor.
    #[allow(private_bounds)]
    pub trait Mode: sealed::Sealed {
        #[doc(hidden)]
        const ASYNC: bool;
    }
    /// Busy-waiting driver without a required interrupt binding.
    pub struct Blocking;
    /// Interrupt-driven driver with a checked interrupt binding.
    pub struct Async;
    impl sealed::Sealed for Blocking {}
    impl sealed::Sealed for Async {}
    impl Mode for Blocking {
        const ASYNC: bool = false;
    }
    impl Mode for Async {
        const ASYNC: bool = true;
    }
}
pub use mode::{Async, Blocking, Mode};
#[cfg(sysctrl)]
pub mod rcc;
#[cfg(rtc)]
pub mod rtc;
#[cfg(spi)]
pub mod spi;
#[cfg(feature = "time-driver-gtim1")]
mod time_driver;
#[cfg(any(atim, gtim))]
pub mod timer;
#[cfg(uart)]
pub mod uart;
#[cfg(any(iwdt, wwdt))]
pub mod wdg;

use core::cell::Cell;
#[derive(Clone, Copy)]
enum InitState {
    Available,
    Failed,
    Ready,
}
static TAKEN: critical_section::Mutex<Cell<InitState>> =
    critical_section::Mutex::new(Cell::new(InitState::Available));

/// Validated direct-reset startup clock configuration. External oscillator pins
/// remain reserved in the returned metadata-generated peripheral collection.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct Config {
    pub rcc: rcc::Config,
    /// GTIM1 IRQ priority. Service must never be blocked for a complete 65.536 ms wrap.
    #[cfg(feature = "time-driver-gtim1")]
    pub time_interrupt_priority: interrupt::Priority,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rcc: rcc::Config::default(),
            #[cfg(feature = "time-driver-gtim1")]
            time_interrupt_priority: interrupt::Priority::P0,
        }
    }
}

#[derive(Debug)]
pub enum InitError {
    AlreadyTaken,
    /// A prior hardware initialization failed. Reset before trying again;
    /// partially started clocks or reserved oscillator pads are not reclaimed.
    PreviousInitFailed,
    Clock(rcc::ClockError),
    /// Selected GTIM1 cannot produce the fixed 1 MHz Embassy timebase.
    #[cfg(feature = "time-driver-gtim1")]
    UnsupportedTimeClock,
}

// All device/peripheral/pin identities come from cw32-metapac metadata.
include!(concat!(env!("OUT_DIR"), "/_generated.rs"));

/// Initialize after direct reset boot and acquire singleton ownership once.
/// A bootloader-modified clock or retained HSE/PLL is rejected. HSE-used pads
/// are `None` in the returned collection; bypass keeps its unused output pad.
/// The binary must execute on the selected chip and board wiring must match its pin use.
pub fn init(config: Config) -> Peripherals {
    try_init(config).expect("CW32 initialization failed")
}

/// Fallible initialization. Invalid requested profiles and unsupported Embassy
/// time clocks fail before MMIO and leave initialization available. Once hardware
/// initialization starts, any failure consumes this boot's initialization right;
/// reset is required before retrying and no peripheral/pin token is returned.
pub fn try_init(config: Config) -> Result<Peripherals, InitError> {
    critical_section::with(|cs| {
        let taken = TAKEN.borrow(cs);
        match taken.get() {
            InitState::Ready => return Err(InitError::AlreadyTaken),
            InitState::Failed => return Err(InitError::PreviousInitFailed),
            InitState::Available => {}
        }
        let requested = config.rcc.validate().map_err(InitError::Clock)?;
        #[cfg(feature = "time-driver-gtim1")]
        if !time_driver::valid_clock(requested) {
            return Err(InitError::UnsupportedTimeClock);
        }
        #[cfg(not(feature = "time-driver-gtim1"))]
        let _ = requested;
        // No safe tokens have escaped. An oscillator can remain active after a
        // timeout, so never let a failed attempt later return its pads as GPIO.
        taken.set(InitState::Failed);
        // SAFETY: exclusive global initialization, critical section held; the
        // clock routine validates reset state before applying vendor trim.
        let clocks = unsafe { rcc::init(config.rcc) }.map_err(InitError::Clock)?;
        rcc::set_clocks(clocks);
        #[cfg(feature = "time-driver-gtim1")]
        // SAFETY: GTIM1 is reserved from the generated safe singleton set;
        // clocks were validated and initialization is globally exclusive.
        unsafe {
            time_driver::init(clocks, config.time_interrupt_priority)
        };
        taken.set(InitState::Ready);
        // SAFETY: the single global acquisition is guarded by TAKEN above.
        Ok(unsafe { take_generated(config.rcc.uses_hse(), config.rcc.uses_hse_output()) })
    })
}

/// Audited associations for driver and board inspection. Empty tables mean no verified data,
/// not absence of hardware capability. These tables do not install ISRs or AFIO.
#[cfg(feature = "metadata")]
pub mod metadata {
    include!(concat!(env!("OUT_DIR"), "/_generated_associations.rs"));
}
