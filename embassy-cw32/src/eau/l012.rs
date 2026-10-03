//! CW32L012 extended arithmetic unit: 32-bit division and integer square root.
//!
//! Based on User Manual v1.4, sections 11.3--11.7 (printed pp.148--153), and
//! vendor `cw32l012_eau` SDK. Division starts on the DIVISOR write; sqrt starts
//! on the DIVIDEND write. All accesses are 32-bit. No interrupt or DMA is used.
//! Poll budgets bound register reads, not wall-clock time. Hardware execution
//! has not been verified on a board.
use core::marker::PhantomData;

use crate::{pac, rcc::PeripheralClock, Peri, PeripheralType};
use pac::eau::regs;

mod sealed {
    pub(crate) trait Instance {
        fn regs() -> crate::pac::eau::Eau;
    }
}

/// Audited EAU peripheral identity, generated from the selected chip metadata.
#[allow(private_bounds)]
pub trait Instance: sealed::Instance + PeripheralClock + PeripheralType + 'static {}

include!(concat!(env!("OUT_DIR"), "/_generated_eau.rs"));

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    SignedOverflow,
    Busy,
    /// The calculation may still be running; retain this instance and retry
    /// when idle, or call reset to discard the operation.
    Timeout,
    ZeroPollBudget,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Division<T> {
    pub quotient: T,
    /// For signed division this has the dividend's sign (truncation toward zero).
    pub remainder: T,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SquareRoot {
    /// Floor(sqrt(input)).
    pub root: u32,
    /// input - root*root.
    pub remainder: u32,
}

/// Exclusive accelerator owner. The final clock owner gates the accelerator on drop.
pub struct Eau<'d, T: Instance> {
    clock: crate::rcc::ClockGuard,
    token: Peri<'d, T>,
}
impl<'d, T: Instance> Eau<'d, T> {
    pub fn new(token: Peri<'d, T>) -> Self {
        let clock = T::acquire();
        Self { token, clock }
    }
    /// Reset only while no forgotten owner retains the same clock resource.
    pub fn reset(&mut self) -> bool {
        self.clock.reset()
    }
    pub fn release(self) -> Peri<'d, T> {
        self.clock.reset();
        self.token
    }
    pub fn is_busy(&self) -> bool {
        // SAFETY: exclusive clocked peripheral; status reads have no side effects.
        T::regs().csr().read().busy()
    }
    pub fn divide_unsigned(
        &mut self,
        dividend: u32,
        divisor: u32,
        poll_budget: u32,
    ) -> Result<Division<u32>, Error> {
        run(
            &mut Hardware::<T>(PhantomData),
            Operation::unsigned(dividend, divisor)?,
            poll_budget,
        )
    }
    pub fn divide_signed(
        &mut self,
        dividend: i32,
        divisor: i32,
        poll_budget: u32,
    ) -> Result<Division<i32>, Error> {
        let result = run(
            &mut Hardware::<T>(PhantomData),
            Operation::signed(dividend, divisor)?,
            poll_budget,
        )?;
        Ok(Division {
            quotient: result.quotient as i32,
            remainder: result.remainder as i32,
        })
    }
    pub fn sqrt(&mut self, value: u32, poll_budget: u32) -> Result<SquareRoot, Error> {
        let result = run(
            &mut Hardware::<T>(PhantomData),
            Operation::sqrt(value),
            poll_budget,
        )?;
        Ok(SquareRoot {
            root: result.quotient,
            remainder: result.remainder,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
enum Mode {
    Unsigned = 0,
    Signed = 1,
    Sqrt = 2,
}
struct Operation {
    mode: Mode,
    dividend: u32,
    divisor: Option<u32>,
}
impl Operation {
    fn unsigned(dividend: u32, divisor: u32) -> Result<Self, Error> {
        if divisor == 0 {
            return Err(Error::DivisionByZero);
        }
        Ok(Self {
            mode: Mode::Unsigned,
            dividend,
            divisor: Some(divisor),
        })
    }
    fn signed(dividend: i32, divisor: i32) -> Result<Self, Error> {
        if divisor == 0 {
            return Err(Error::DivisionByZero);
        }
        if dividend == i32::MIN && divisor == -1 {
            return Err(Error::SignedOverflow);
        }
        Ok(Self {
            mode: Mode::Signed,
            dividend: dividend as u32,
            divisor: Some(divisor as u32),
        })
    }
    fn sqrt(dividend: u32) -> Self {
        Self {
            mode: Mode::Sqrt,
            dividend,
            divisor: None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reg {
    Csr,
    Dividend,
    Divisor,
    Quotient,
    Remainder,
}
trait Backend {
    fn read(&mut self, reg: Reg) -> u32;
    fn write(&mut self, reg: Reg, word: u32);
}
struct Hardware<T: Instance>(PhantomData<T>);
impl<T: Instance> Backend for Hardware<T> {
    fn read(&mut self, reg: Reg) -> u32 {
        // Ownership/clock established by Eau. Results are read only when idle.
        match reg {
            Reg::Csr => T::regs().csr().read().0,
            Reg::Quotient => T::regs().quotient().read().0,
            Reg::Remainder => T::regs().remainder().read().0,
            _ => unreachable!(),
        }
    }
    fn write(&mut self, reg: Reg, word: u32) {
        // Explicit whole-operand writes retain the computation trigger order.
        match reg {
            Reg::Csr => T::regs().csr().write_value(regs::Csr(word)),
            Reg::Dividend => T::regs().dividend().write_value(regs::Dividend(word)),
            Reg::Divisor => T::regs().divisor().write_value(regs::Divisor(word)),
            _ => unreachable!(),
        }
    }
}

fn status_error(mode: Mode, status: u32) -> Result<(), Error> {
    let status = regs::Csr(status);
    if mode != Mode::Sqrt && status.zero() {
        return Err(Error::DivisionByZero);
    }
    if mode == Mode::Signed && status.ovr() {
        return Err(Error::SignedOverflow);
    }
    Ok(())
}
fn run(
    backend: &mut impl Backend,
    op: Operation,
    poll_budget: u32,
) -> Result<Division<u32>, Error> {
    if poll_budget == 0 {
        return Err(Error::ZeroPollBudget);
    }
    if regs::Csr(backend.read(Reg::Csr)).busy() {
        return Err(Error::Busy);
    }
    // Do not RMW status flags into a new configuration word.
    let mut control = regs::Csr(0);
    control.set_mode(op.mode as u8);
    backend.write(Reg::Csr, control.0);
    backend.write(Reg::Dividend, op.dividend);
    if let Some(divisor) = op.divisor {
        backend.write(Reg::Divisor, divisor);
    }
    for _ in 0..poll_budget {
        let status = backend.read(Reg::Csr);
        if !regs::Csr(status).busy() {
            status_error(op.mode, status)?;
            return Ok(Division {
                quotient: backend.read(Reg::Quotient),
                remainder: backend.read(Reg::Remainder),
            });
        }
        core::hint::spin_loop();
    }
    Err(Error::Timeout)
}
