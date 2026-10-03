//! Comparator versions share event ownership and one-shot wait semantics;
//! register programming, startup checks and resource routes remain IP-specific.

#[cfg(all(vc_l012, bgr_l012))]
mod l012;
#[cfg(all(vc_l012, bgr_l012))]
use l012::VcHardware;
#[cfg(all(vc_l012, bgr_l012))]
pub use l012::*;

#[cfg(vc_f030)]
mod f030;
#[cfg(vc_f030)]
use f030::VcHardware;
#[cfg(vc_f030)]
pub use f030::*;

mod r#async;
pub use r#async::InterruptHandler;
