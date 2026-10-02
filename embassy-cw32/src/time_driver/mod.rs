//! GTIM1 time-driver facade. Hardware algorithms remain variant-specific.

#[cfg(gtim_l012)]
#[path = "l012_core.rs"]
mod core;
#[cfg(gtim_f030)]
#[path = "f030_core.rs"]
mod core;
mod queue;

#[cfg(all(feature = "time-driver-gtim1", gtim_l012))]
mod l012;
#[cfg(all(feature = "time-driver-gtim1", gtim_l012))]
pub(crate) use l012::init;
#[cfg(all(feature = "time-driver-gtim1", gtim_f030))]
mod f030;
#[cfg(all(feature = "time-driver-gtim1", gtim_f030))]
pub(crate) use f030::init;
