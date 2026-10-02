//! analog API, selected by the generated vc IP variant.

#[cfg(vc_l012)]
mod l012;
#[cfg(vc_l012)]
pub use l012::*;

#[cfg(vc_f030)]
mod f030;
#[cfg(vc_f030)]
pub use f030::*;
