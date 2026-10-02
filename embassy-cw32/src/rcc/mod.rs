//! rcc API, selected by the generated sysctrl IP variant.

#[cfg(sysctrl_l012)]
mod l012;
#[cfg(sysctrl_l012)]
pub use l012::*;

#[cfg(sysctrl_f030)]
mod f030;
#[cfg(sysctrl_f030)]
pub use f030::*;
