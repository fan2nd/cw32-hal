//! Owned blocking GPIO and checked interrupt-backed async inputs.
//! Variant-specific setup remains explicitly cfg-gated inside the implementation.

#[cfg(any(gpio_l012, gpio_f030))]
mod shared;
#[cfg(any(gpio_l012, gpio_f030))]
pub use shared::*;
#[cfg(any(gpio_l012, gpio_f030))]
pub(crate) mod interrupt_input;
#[cfg(any(gpio_l012, gpio_f030))]
pub use interrupt_input::{InterruptHandler, InterruptInput, InterruptPin};
