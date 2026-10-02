//! Blocking GPIO API shared by the audited L012 and F030 subsets.
//! Variant-specific setup remains explicitly cfg-gated inside the implementation.

#[cfg(any(gpio_l012, gpio_f030))]
mod shared;
#[cfg(any(gpio_l012, gpio_f030))]
pub use shared::*;
