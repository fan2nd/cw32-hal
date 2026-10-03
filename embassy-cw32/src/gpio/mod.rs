//! Owned blocking GPIO and checked interrupt-backed async inputs.
//! Optional register capabilities are selected from metadata inside the shared implementation.

mod shared;
pub use shared::*;
pub(crate) mod interrupt_input;
pub use interrupt_input::{InterruptHandler, InterruptInput, InterruptPin};
