//! cordic API for the audited L012 IP variant.

#[cfg(cordic_l012)]
mod l012;
#[cfg(cordic_l012)]
pub use l012::*;
