#![no_std]

pub mod board;
/// Fixed-point PID, Clarke/Park and SVPWM algorithms.
pub use cw32_l012_foc_algorithms as foc;
pub use cw32_l012_foc_algorithms::control;
