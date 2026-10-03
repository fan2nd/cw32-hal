//! Explicit ownership-bypass operations for a serialized motor-control domain.
//!
//! These handles do not take singleton tokens, claim pins, install interrupts,
//! serialize callers, or prove exclusivity. Every `acquire` is unsafe: the caller
//! must exclude safe HAL owners, other handles, PAC access and interrupt/DMA
//! configuration changes for the entire handle lifetime. Retained but unused
//! singleton tokens are permitted; using them concurrently is not.
//!
//! A handle is a short-lived exclusive hardware lease. Drop does not stop the
//! hardware or restore a prior HAL driver configuration. Dormant configured
//! drivers must not resume unless their invariants have been restored; normally
//! drop and reinitialize them. Keep configuration and any DMA storage valid
//! after the lease ends.
//! Examples use non-nesting equal-priority motor interrupts; thread-mode access
//! additionally requires a critical section. No lease or software-state borrow
//! may survive an await if an interrupt can access the same resource.
//!
//! Pin routing, clocks/VDDA limits, settling delays, gate-driver polarity, dead
//! time and physical protection remain board responsibilities. These operations
//! are not a motor controller or a certified safety layer. Ordinary owned HAL
//! drivers remain the default for code that does not need this escape hatch.

#[cfg(adc_l012)]
mod adc;
#[cfg(adc_l012)]
pub use adc::*;
#[cfg(atim_l012)]
mod pwm;
#[cfg(atim_l012)]
pub use pwm::*;
#[cfg(btim_l012)]
mod timer;
#[cfg(btim_l012)]
pub use timer::*;
#[cfg(gpio_l012)]
mod pins;
#[cfg(gpio_l012)]
pub use pins::*;
#[cfg(all(opa_l012, bgr_l012))]
mod analog;
#[cfg(all(opa_l012, bgr_l012))]
pub use analog::*;
