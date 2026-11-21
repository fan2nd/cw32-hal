//! L012 integer arithmetic accelerator. All operands and results are 32-bit.
//!
//! Poll budgets count status reads, not time. A timeout does not abort hardware.
//! Subsequent operations refuse to overwrite a busy engine. The access value
//! has no peripheral ownership and dropping it never changes the clock or
//! operation state; no reset pulse or abort is attempted.
//! See `docs/l012-math-evidence.md` for sources and verification boundaries.

use crate::{PeripheralType, pac, rcc::RccPeripheral};
use core::marker::PhantomData;
use pac::eau::vals::Mode;

/// Startup or calculation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// Peripheral is held in reset; the driver never releases or pulses reset.
    HeldInReset,
    /// Clock enable did not read back as enabled.
    ClockNotEnabled,
    /// An earlier operation is still running; no operands were written.
    Busy,
    /// The caller's status-read budget expired; hardware may still be running.
    Timeout,
    /// Hardware reported a zero divisor; result registers were not read.
    DivisionByZero,
    /// Hardware reported signed division overflow; result registers were not read.
    Overflow,
}

/// Quotient and remainder in the operand's integer format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Division<T> {
    /// Quotient, with signed division truncated toward zero.
    pub quotient: T,
    /// Remainder; a signed remainder has the dividend's sign (or is zero).
    pub remainder: T,
}

/// Integer square root and remainder: `radicand = root * root + remainder`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SquareRoot {
    /// Nonnegative integer square root, rounded down.
    pub root: u32,
    /// The portion left after subtracting the square of `root`.
    pub remainder: u32,
}

trait SealedInstance {
    fn regs() -> pac::eau::Eau;
}
/// A source-qualified EAU instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + RccPeripheral {}
impl SealedInstance for crate::peripherals::EAU {
    fn regs() -> pac::eau::Eau {
        pac::EAU
    }
}
impl Instance for crate::peripherals::EAU {}

/// Blocking access to the shared EAU accelerator. Each calculation holds a
/// critical section from the busy check through the final result read.
/// The poll budget therefore also bounds the number of completion polls while
/// interrupts are masked. A timed-out operation can leave the engine busy.
pub struct Eau<T: Instance> {
    _marker: PhantomData<T>,
}
impl<T: Instance> Eau<T> {
    /// Enable the gate, preserving reset and neighboring/shared resources.
    /// A busy startup leaves the gate enabled so existing work can finish.
    pub fn new() -> Result<Self, Error> {
        if T::RCC_INFO.reset_asserted() {
            return Err(Error::HeldInReset);
        }
        critical_section::with(|cs| T::RCC_INFO.enable_with_cs(cs))
            .map_err(|_| Error::ClockNotEnabled)?;
        if !T::RCC_INFO.is_enabled() {
            return Err(Error::ClockNotEnabled);
        }
        if T::regs().csr().read().busy() {
            return Err(Error::Busy);
        }
        Ok(Self {
            _marker: PhantomData,
        })
    }
    /// Observe busy without modifying hardware or reading result registers.
    pub fn is_busy(&self) -> bool {
        T::regs().csr().read().busy()
    }

    /// Unsigned 32-bit hardware division, including hardware divide-by-zero reporting.
    /// At most `polls` CSR reads follow submission; zero submits nothing.
    pub fn divide_unsigned(
        &mut self,
        dividend: u32,
        divisor: u32,
        polls: u32,
    ) -> Result<Division<u32>, Error> {
        critical_section::with(|_| {
            self.prepare(Mode::UnsignedDivision, polls)?;
            let r = T::regs();
            r.dividend().write(|w| w.set_dividend(dividend));
            r.divisor().write(|w| w.set_divisor(divisor)); // Trigger after dividend.
            let status = self.wait(polls)?;
            if status.zero() {
                return Err(Error::DivisionByZero);
            }
            Ok(Division {
                quotient: r.quotient().read().quotient(),
                remainder: r.remainder().read().remainder(),
            })
        })
    }
    /// Signed 32-bit division, with hardware zero-divisor and overflow reporting.
    /// `i32::MIN / -1` reports [`Error::Overflow`]. No result is read on error.
    pub fn divide_signed(
        &mut self,
        dividend: i32,
        divisor: i32,
        polls: u32,
    ) -> Result<Division<i32>, Error> {
        critical_section::with(|_| {
            self.prepare(Mode::SignedDivision, polls)?;
            let r = T::regs();
            r.dividend().write(|w| w.set_dividend(dividend as u32));
            r.divisor().write(|w| w.set_divisor(divisor as u32));
            let status = self.wait(polls)?;
            if status.zero() {
                return Err(Error::DivisionByZero);
            }
            if status.ovr() {
                return Err(Error::Overflow);
            }
            Ok(Division {
                quotient: r.quotient().read().quotient() as i32,
                remainder: r.remainder().read().remainder() as i32,
            })
        })
    }
    /// Unsigned integer square root. Writing DIVIDEND triggers this operation.
    /// ZERO/OVR are inapplicable in square-root mode and are not interpreted.
    pub fn square_root(&mut self, radicand: u32, polls: u32) -> Result<SquareRoot, Error> {
        critical_section::with(|_| {
            self.prepare(Mode::SquareRoot, polls)?;
            let r = T::regs();
            r.dividend().write(|w| w.set_dividend(radicand));
            self.wait(polls)?;
            Ok(SquareRoot {
                root: r.quotient().read().quotient(),
                remainder: r.remainder().read().remainder(),
            })
        })
    }
    fn prepare(&mut self, mode: Mode, polls: u32) -> Result<(), Error> {
        if polls == 0 {
            return Err(Error::Timeout);
        }
        if self.is_busy() {
            return Err(Error::Busy);
        }
        T::regs().csr().write(|w| w.set_mode(mode));
        Ok(())
    }
    fn wait(&self, polls: u32) -> Result<pac::eau::regs::Csr, Error> {
        for _ in 0..polls {
            let status = T::regs().csr().read();
            if !status.busy() {
                return Ok(status);
            }
            core::hint::spin_loop();
        }
        Err(Error::Timeout)
    }
}
