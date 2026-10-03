//! Owned analog building blocks, selected independently by peripheral IP.
//!
//! Signal pins and peripheral identities come from audited metadata. Drivers
//! retain every peripheral, pin and analog-source borrow for their full lifetime.
//! Shared clocks, reset lines and BGR are never disabled by another owner's drop.
//! Raw electrical codes and conservative startup delays are not calibrated volts
//! or a guarantee outside datasheet conditions. No board-level signal integrity,
//! calibration accuracy or motor safety has been validated.

use crate::gpio::Pin;
#[cfg(any(dac_l012, opa_l012, vc_l012, vc_f030, vcref_l012))]
use crate::peripherals;
#[cfg(any(opa_l012, vc_l012, vc_f030, vcref_l012))]
use crate::{pac, rcc::PeripheralClock, PeripheralType};

#[cfg(bgr_l012)]
#[path = "bgr/l012.rs"]
mod bgr;
#[cfg(bgr_l012)]
pub use bgr::*;

#[cfg(dac_l012)]
#[path = "dac/l012.rs"]
mod dac;
#[cfg(dac_l012)]
pub use dac::*;

#[cfg(vcref_l012)]
#[path = "vcref/l012.rs"]
mod vcref;
#[cfg(vcref_l012)]
pub use vcref::*;

// These owners require the BGR startup witness in every constructor. Other
// analog kinds remain usable independently of this coherent prerequisite.
#[cfg(all(opa_l012, bgr_l012))]
#[path = "opa/l012.rs"]
mod opa;
#[cfg(all(opa_l012, bgr_l012))]
pub use opa::*;

#[cfg(any(all(vc_l012, bgr_l012), vc_f030))]
mod vc;
#[cfg(any(all(vc_l012, bgr_l012), vc_f030))]
pub use vc::*;

mod sealed {
    #[cfg(opa_l012)]
    #[cfg_attr(not(bgr_l012), allow(dead_code))]
    pub(crate) trait OpaInstance {
        fn regs() -> crate::pac::opa::Opa;
    }
    #[cfg(vcref_l012)]
    pub(crate) trait RefInstance {
        fn regs() -> crate::pac::vcref::Vcref;
    }
    #[cfg(any(vc_l012, vc_f030))]
    #[cfg_attr(all(vc_l012, not(bgr_l012)), allow(dead_code))]
    pub(crate) trait VcInstance {
        fn regs() -> crate::pac::vc::Vc;
        fn state() -> &'static crate::async_support::EventState;
    }
    pub trait Pin<I, const S: u8> {}
}

/// Audited chip-level signal route; package/board bonding must be checked separately.
/// OPA: OUT=0, INP1..3=1..3, INN1..2=11..12; VC: CH0..3 (l012) or CH0..7
/// (f030); DAC: OUT1/OUT2=1/2.
pub trait SignalPin<I, const S: u8>: sealed::Pin<I, S> + Pin {}
/// Audited OPA peripheral identity. Register access is internal to the driver.
#[cfg(opa_l012)]
#[allow(private_bounds)]
pub trait OpaInstance: sealed::OpaInstance + PeripheralClock + PeripheralType {}
/// Audited pair-owned comparator reference identity.
#[cfg(vcref_l012)]
#[allow(private_bounds)]
pub trait RefInstance: sealed::RefInstance + PeripheralClock + PeripheralType {}
/// Audited comparator identity and physical interrupt association.
#[cfg(any(vc_l012, vc_f030))]
#[allow(private_bounds)]
pub trait VcInstance: sealed::VcInstance + PeripheralClock + PeripheralType {
    type Interrupt: crate::interrupt::typelevel::Interrupt;
    #[cfg(all(vc_l012, vcref_l012))]
    type Reference: RefInstance;
    const NUMBER: u8;
}
include!(concat!(env!("OUT_DIR"), "/_generated_analog.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[cfg(dac_l012)]
    InvalidCode,
    InvalidSignal,
    Disabled,
    #[cfg(vc_f030)]
    InvalidConfig,
    NotReady,
    #[cfg(vcref_l012)]
    InvalidDivider,
    #[cfg(opa_l012)]
    InvalidCalibration,
    #[cfg(opa_l012)]
    Busy,
    #[cfg(opa_l012)]
    Timeout,
    #[cfg(opa_l012)]
    ZeroPollBudget,
    #[cfg(all(vc_f030, atim_f030))]
    OutputsEnabled,
    #[cfg(all(vc_f030, atim_f030))]
    RouteInUse,
}
