//! Board-selected pin operations without fabricated singleton ownership.
use crate::{gpio::Port, pac};
use core::marker::PhantomData;
#[derive(Clone, Copy, Debug)]
pub struct PinId {
    pub(super) port: Port,
    pub(super) number: u8,
}
impl PinId {
    pub const fn new(port: Port, number: u8) -> Self {
        assert!(number < 16 && port.implemented_mask() & (1 << number) != 0);
        Self { port, number }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum PinMode {
    OutputLow,
    OutputHigh,
    InputPullUp,
    Analog,
}
/// A caller-owned pin lease. Shared port RMW operations are critical-section protected.
pub struct MotorPin {
    id: PinId,
    _domain: PhantomData<*mut ()>,
}
impl MotorPin {
    /// # Safety
    /// Verify bonding, mux and wiring. Exclude all other users of this pin for
    /// the lease lifetime, including safe GPIO/peripheral owners and interrupts.
    /// Other pins may be used concurrently only through coordinated port RMWs.
    pub unsafe fn acquire(id: PinId) -> Self {
        Self {
            id,
            _domain: PhantomData,
        }
    }
    fn regs(&self) -> pac::gpio::Gpio {
        // SAFETY: address comes from generated port metadata, not the caller.
        unsafe { pac::gpio::Gpio::from_ptr(self.id.port.base() as *mut ()) }
    }
    pub fn configure(&mut self, mode: PinMode, alternate_function: u8) {
        assert!(alternate_function < 16);
        critical_section::with(|_| {
            self.id.port.enable_clock();
            let r = self.regs();
            let n = self.id.number as usize;
            r.dir().modify(|w| w.set_pin(n, true));
            r.afr(n / 8)
                .modify(|w| w.set_afr(n % 8, alternate_function));
            r.riseie().modify(|w| w.set_pin(n, false));
            r.fallie().modify(|w| w.set_pin(n, false));
            r.opendrain().modify(|w| w.set_pin(n, false));
            r.pur()
                .modify(|w| w.set_pin(n, matches!(mode, PinMode::InputPullUp)));
            r.analog()
                .modify(|w| w.set_pin(n, matches!(mode, PinMode::Analog)));
            if matches!(mode, PinMode::OutputLow | PinMode::OutputHigh) {
                self.set_high(matches!(mode, PinMode::OutputHigh));
                r.dir().modify(|w| w.set_pin(n, false));
            }
        });
    }
    /// Change the mux only, retaining the output direction and latch.
    pub fn alternate_function(&mut self, af: u8) {
        assert!(af < 16);
        critical_section::with(|_| {
            let n = self.id.number as usize;
            self.regs().afr(n / 8).modify(|w| w.set_afr(n % 8, af));
        });
    }
    pub fn set_high(&mut self, high: bool) {
        let n = self.id.number as usize;
        if high {
            self.regs().bsrr().write(|w| w.set_bss(n, true));
        } else {
            self.regs().brr().write(|w| w.set_brr(n, true));
        }
    }
    pub fn is_high(&mut self) -> bool {
        self.regs().idr().read().pin(self.id.number as usize)
    }
}
