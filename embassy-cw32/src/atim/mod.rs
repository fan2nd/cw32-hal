//! atim API, selected by the generated atim IP variant.

#[cfg(atim_l012)]
mod l012;
#[cfg(atim_l012)]
pub use l012::*;

#[cfg(atim_f030)]
mod f030;
#[cfg(atim_f030)]
pub use f030::*;
