//! ATIM API with shared async ownership and IP-specific register control.

#[cfg_attr(atim_l012, path = "l012.rs")]
#[cfg_attr(atim_f030, path = "f030.rs")]
mod backend;
mod events;

pub use backend::*;
pub use events::{AsyncThreePhasePwm, InterruptHandler};
