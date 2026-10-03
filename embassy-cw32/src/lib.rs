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
#[cfg(spi)]
pub mod spi;
#[cfg(feature = "time-driver-gtim1")]
mod time_driver;
#[cfg(any(atim, gtim))]
pub mod timer;
#[cfg(uart)]
pub mod uart;
#[cfg(iwdt)]
pub mod wdg;

use core::cell::Cell;
static TAKEN: critical_section::Mutex<Cell<bool>> = critical_section::Mutex::new(Cell::new(false));

/// Initialization validates direct-reset HSI, then applies the requested HSI/bus dividers.
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
    Clock(rcc::ClockError),
    /// Selected GTIM1 cannot produce the fixed 1 MHz Embassy timebase.
    #[cfg(feature = "time-driver-gtim1")]
    UnsupportedTimeClock,
}

// All device/peripheral/pin identities come from cw32-metapac metadata.
include!(concat!(env!("OUT_DIR"), "/_generated.rs"));

/// Initialize after direct reset boot and acquire singleton ownership once.
/// A bootloader-modified clock is rejected rather than silently misreported.
/// The binary must execute on the selected chip and board wiring must match its pin use.
pub fn init(config: Config) -> Peripherals {
    try_init(config).expect("CW32 initialization failed")
}

/// Fallible initialization for applications that report unsupported boot clocks.
pub fn try_init(config: Config) -> Result<Peripherals, InitError> {
    critical_section::with(|cs| {
        let taken = TAKEN.borrow(cs);
        if taken.get() {
            return Err(InitError::AlreadyTaken);
        }
        #[cfg(feature = "time-driver-gtim1")]
        if !time_driver::valid_clock(config.rcc.clocks()) {
            return Err(InitError::UnsupportedTimeClock);
        }
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
        taken.set(true);
        // SAFETY: the single global acquisition is guarded by TAKEN above.
        Ok(unsafe { take_generated() })
    })
}

/// Audited associations for driver and board inspection. Empty tables mean no verified data,
/// not absence of hardware capability. These tables do not install ISRs or AFIO.
#[cfg(feature = "metadata")]
pub mod metadata {
    include!(concat!(env!("OUT_DIR"), "/_generated_associations.rs"));
}
