//! Checked startup clocks and counted peripheral clock resources.
//!
//! Frequencies are fixed after initialization. Runtime clock switching and
//! DeepSleep/STOP recovery are deliberately unsupported.

use crate::pac;
use core::cell::Cell;

#[cfg(sysctrl_l012)]
mod l012;
#[cfg(sysctrl_l012)]
pub use l012::*;
#[cfg(sysctrl_f030)]
mod f030;
#[cfg(sysctrl_f030)]
pub use f030::*;

mod low_speed;
pub use low_speed::*;

/// AHB clock divider. Encodings follow SYSCTRL.CR0.HCLKPRS on both variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum HclkDivider {
    #[default]
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
    Div16 = 4,
    Div32 = 5,
    Div64 = 6,
    Div128 = 7,
}
impl HclkDivider {
    pub(crate) const fn register_value(self) -> crate::pac::sysctrl::vals::Cr0Hclkprs {
        match self {
            Self::Div1 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV1,
            Self::Div2 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV2,
            Self::Div4 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV4,
            Self::Div8 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV8,
            Self::Div16 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV16,
            Self::Div32 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV32,
            Self::Div64 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV64,
            Self::Div128 => crate::pac::sysctrl::vals::Cr0Hclkprs::DIV128,
        }
    }
}

/// APB clock divider. CW32 timer kernels use PCLK directly, without doubling.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum PclkDivider {
    #[default]
    Div1 = 0,
    Div2 = 1,
    Div4 = 2,
    Div8 = 3,
}

impl PclkDivider {
    pub(crate) const fn register_value(self) -> crate::pac::sysctrl::vals::Cr0Pclkprs {
        match self {
            Self::Div1 => crate::pac::sysctrl::vals::Cr0Pclkprs::DIV1,
            Self::Div2 => crate::pac::sysctrl::vals::Cr0Pclkprs::DIV2,
            Self::Div4 => crate::pac::sysctrl::vals::Cr0Pclkprs::DIV4,
            Self::Div8 => crate::pac::sysctrl::vals::Cr0Pclkprs::DIV8,
        }
    }
}

/// One physical gate, shared by every metadata alias and independently split
/// owner. A forgotten guard keeps its count; quarantine permanently pins it.
pub(crate) struct ClockResource {
    state: critical_section::Mutex<Cell<ResourceState>>,
    gate: fn(bool),
    reset: Option<fn()>,
}
#[derive(Clone, Copy)]
struct ResourceState {
    users: u16,
    pinned: bool,
}
impl ClockResource {
    pub(crate) const fn new(gate: fn(bool), reset: Option<fn()>) -> Self {
        Self {
            state: critical_section::Mutex::new(Cell::new(ResourceState {
                users: 0,
                pinned: false,
            })),
            gate,
            reset,
        }
    }
    /// IRQ dispatch can outlive the last driver when an NVIC vector stays
    /// shared/enabled. Check under the same critical section as ISR MMIO.
    pub(crate) fn is_enabled(&self) -> bool {
        critical_section::with(|cs| {
            let state = self.state.borrow(cs).get();
            state.users != 0 || state.pinned
        })
    }
    pub(crate) fn enable_pinned(&'static self) {
        let mut clock = self.acquire(false);
        clock.pin();
    }
    fn acquire(&'static self, reset: bool) -> ClockGuard {
        critical_section::with(|cs| {
            let mut state = self.state.borrow(cs).get();
            let users = state
                .users
                .checked_add(1)
                .expect("clock owner count exhausted");
            if state.users == 0 {
                (self.gate)(true);
                if reset && !state.pinned {
                    if let Some(reset) = self.reset {
                        reset();
                    }
                }
            }
            state.users = users;
            self.state.borrow(cs).set(state);
        });
        ClockGuard { resource: self }
    }
    /// Safe only as an internal ownership check: no reset is asserted while a
    /// sibling, forgotten owner, or quarantined transfer retains the resource.
    #[cfg_attr(not(any(cordic_l012, eau_l012)), allow(dead_code))]
    fn reset_if_exclusive(&self) -> bool {
        critical_section::with(|cs| {
            let state = self.state.borrow(cs).get();
            if state.users != 1 || state.pinned {
                return false;
            }
            if let Some(reset) = self.reset {
                reset();
                true
            } else {
                false
            }
        })
    }
}

/// Stored by each independently droppable hardware owner. Drivers stop their
/// own activity before this field drops and releases the final physical gate.
pub(crate) struct ClockGuard {
    resource: &'static ClockResource,
}
impl ClockGuard {
    /// Split an existing live resource without resetting any hardware.
    pub(crate) fn retain(&self) -> Self {
        self.resource.acquire(false)
    }
    /// Retain a gate indefinitely when outstanding bus activity is unproven,
    /// or when an unsafe motor takeover intentionally leaves hardware running.
    pub(crate) fn pin(&mut self) {
        critical_section::with(|cs| {
            let mut state = self.resource.state.borrow(cs).get();
            state.pinned = true;
            self.resource.state.borrow(cs).set(state);
        });
    }
    #[cfg_attr(not(any(cordic_l012, eau_l012)), allow(dead_code))]
    pub(crate) fn reset(&self) -> bool {
        self.resource.reset_if_exclusive()
    }
}
impl Drop for ClockGuard {
    fn drop(&mut self) {
        critical_section::with(|cs| {
            let mut state = self.resource.state.borrow(cs).get();
            state.users = state
                .users
                .checked_sub(1)
                .expect("unbalanced clock release");
            if state.users == 0 && !state.pinned {
                (self.resource.gate)(false);
            }
            self.resource.state.borrow(cs).set(state);
        });
    }
}

/// Generated metadata defines the real peripheral source and shared gate.
/// The frequency is the input kernel clock; local ADC/timer divisors apply later.
pub(crate) trait PeripheralClock {
    fn clock_resource() -> &'static ClockResource;
    fn bus_frequency() -> u32;
    fn acquire() -> ClockGuard {
        Self::clock_resource().acquire(true)
    }
    fn acquire_no_reset() -> ClockGuard {
        Self::clock_resource().acquire(false)
    }
}
pub(crate) trait KernelClock: PeripheralClock {
    fn frequency() -> u32;
}
/// Nominal peripheral input clock from its audited metadata source.
/// Local ADC/timer prescalers are applied by their drivers. Peripherals with
/// unresolved or configurable kernel sources do not implement this bound.
/// Panics before HAL initialization.
#[allow(private_bounds)]
pub fn frequency<T: KernelClock>() -> u32 {
    T::frequency()
}

/// Nominal register-interface bus clock from the peripheral's bus domain.
/// This is distinct from independently sourced peripheral kernel clocks.
/// Panics before HAL initialization.
#[allow(private_bounds)]
pub fn bus_frequency<T: PeripheralClock>() -> u32 {
    T::bus_frequency()
}

include!(concat!(env!("OUT_DIR"), "/_generated_peripheral_clocks.rs"));
