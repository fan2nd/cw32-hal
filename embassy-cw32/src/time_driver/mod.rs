//! GTIM1 time-driver facade. Hardware algorithms remain variant-specific.

#[cfg(gtim_l012)]
#[path = "l012_core.rs"]
mod core;
#[cfg(gtim_f030)]
#[path = "f030_core.rs"]
mod core;
#[cfg(feature = "time-driver-gtim1")]
mod driver;
mod queue;

#[cfg(all(feature = "time-driver-gtim1", gtim_l012))]
mod l012;
#[cfg(all(feature = "time-driver-gtim1", gtim_l012))]
pub(crate) use l012::init;
#[cfg(all(feature = "time-driver-gtim1", gtim_f030))]
mod f030;
#[cfg(all(feature = "time-driver-gtim1", gtim_f030))]
pub(crate) use f030::init;

/// Reject an unsupported 1 MHz timebase before any RCC register is changed.
pub(crate) fn valid_clock(clocks: crate::rcc::Clocks) -> bool {
    let hz = embassy_time_driver::TICK_HZ as u32;
    let pclk = clocks.pclk_hz();
    if pclk < hz || pclk % hz != 0 {
        return false;
    }
    #[cfg(gtim_l012)]
    {
        pclk / hz <= 65536
    }
    #[cfg(gtim_f030)]
    {
        hz == 1_000_000 && core::prescaler(pclk / hz).is_some()
    }
}
