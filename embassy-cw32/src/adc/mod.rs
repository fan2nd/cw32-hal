//! adc API, selected by the generated adc IP variant.

#[cfg(adc_l012)]
mod l012;
#[cfg(adc_l012)]
pub use l012::*;

#[cfg(adc_f030)]
mod f030;
#[cfg(adc_f030)]
pub use f030::*;

mod channel;
mod common;
mod internal;
pub use channel::{AdcChannel, BorrowedAdcChannel, BorrowedChannel};
#[cfg(any(dma_l012, dma_f030))]
pub use common::{DmaBuffers, DmaStartError, DmaStartFailure, DmaTransfer};
pub use internal::*;
