//! PAC and metadata generated from the layered cw32-data YAML sources.
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
include!(concat!(env!("OUT_DIR"), "/pac.rs"));

// SAFETY: the enum discriminants come from the validated, audited IRQ metadata.
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    fn number(self) -> u16 {
        self as u16
    }
}

#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
/// Device interrupts, also used to validate the cortex-m-rt interrupt attribute.
pub use Interrupt as interrupt;

// Keep the hardware vector table out of host tools that consume metadata.
#[cfg(all(feature = "rt", target_arch = "arm", target_os = "none"))]
mod runtime {
    include!(concat!(env!("OUT_DIR"), "/rt.rs"));
}

#[cfg(feature = "metadata")]
pub mod metadata {
    include!(concat!(env!("OUT_DIR"), "/metadata.rs"));
}
