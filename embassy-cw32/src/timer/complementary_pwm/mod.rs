//! Owned complementary PWM using each qualified ATIM register architecture.
//!
//! Buffered L010/L011/L012 provide four pairs and bounded BK1 handling.
//! Classic F030/A030 provide three optional complete A+B pairs, interior duty,
//! constructor-only dead time, and explicit global output enable. Classic brake,
//! individual pair disable and endpoint duty are outside this API.

/// Output polarity used by the complementary PWM owner.
pub use super::low_level::OutputPolarity;
#[cfg(atim_buffered)]
mod buffered;
#[cfg(atim_buffered)]
pub use buffered::*;
#[cfg(atim_classic_complementary)]
mod classic;
#[cfg(atim_classic_complementary)]
pub use classic::*;
