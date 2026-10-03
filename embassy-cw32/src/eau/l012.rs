//! CW32L012 extended arithmetic unit: 32-bit division and integer square root.
//!
//! Based on User Manual v1.4, sections 11.3--11.7 (printed pp.148--153), and
//! vendor `cw32l012_eau` SDK. Division starts on the DIVISOR write; sqrt starts
//! on the DIVIDEND write. All accesses are 32-bit. No interrupt or DMA is used.
//! Poll budgets bound register reads, not wall-clock time. Hardware execution
//! has not been verified on a board.
use crate::{pac, peripherals::EAU, rcc::PeripheralClock, Peri};

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

/// Exclusive accelerator owner. Drop does not disable a shared bus clock.
pub struct Eau<'d> {
    token: Peri<'d, EAU>,
}
impl<'d> Eau<'d> {
    pub fn new(token: Peri<'d, EAU>) -> Self {
        <EAU as PeripheralClock>::enable_and_reset();
        Self { token }
    }
    pub fn reset(&mut self) {
        <EAU as PeripheralClock>::enable_and_reset();
    }
    pub fn release(self) -> Peri<'d, EAU> {
        <EAU as PeripheralClock>::enable_and_reset();
        self.token
    }
    pub fn is_busy(&self) -> bool {
        // SAFETY: exclusive clocked peripheral; status reads have no side effects.
        pac::eau::fields::csr::BUSY.read(unsafe { pac::EAU.csr().read() })
    }
    pub fn divide_unsigned(
        &mut self,
        dividend: u32,
        divisor: u32,
        poll_budget: u32,
    ) -> Result<Division<u32>, Error> {
        run(
            &mut Hardware,
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
            &mut Hardware,
            Operation::signed(dividend, divisor)?,
            poll_budget,
        )?;
        Ok(Division {
            quotient: result.quotient as i32,
            remainder: result.remainder as i32,
        })
    }
    pub fn sqrt(&mut self, value: u32, poll_budget: u32) -> Result<SquareRoot, Error> {
        let result = run(&mut Hardware, Operation::sqrt(value), poll_budget)?;
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
struct Hardware;
impl Backend for Hardware {
    fn read(&mut self, reg: Reg) -> u32 {
        // SAFETY: ownership/clock established by Eau. Results read only when idle.
        unsafe {
            match reg {
                Reg::Csr => pac::EAU.csr().read(),
                Reg::Quotient => pac::EAU.quotient().read(),
                Reg::Remainder => pac::EAU.remainder().read(),
                _ => unreachable!(),
            }
        }
    }
    fn write(&mut self, reg: Reg, word: u32) {
        // SAFETY: exclusively owned, valid operands/mode, BUSY checked clear.
        unsafe {
            match reg {
                Reg::Csr => pac::EAU.csr().write_value(word),
                Reg::Dividend => pac::EAU.dividend().write_value(word),
                Reg::Divisor => pac::EAU.divisor().write_value(word),
                _ => unreachable!(),
            }
        }
    }
}
fn status_error(mode: Mode, status: u32) -> Result<(), Error> {
    use pac::eau::fields::csr;
    if mode != Mode::Sqrt && csr::ZERO.read(status) {
        return Err(Error::DivisionByZero);
    }
    if mode == Mode::Signed && csr::OVR.read(status) {
        return Err(Error::SignedOverflow);
    }
    Ok(())
}
fn run(
    backend: &mut impl Backend,
    op: Operation,
    poll_budget: u32,
) -> Result<Division<u32>, Error> {
    use pac::eau::fields::csr;
    if poll_budget == 0 {
        return Err(Error::ZeroPollBudget);
    }
    if csr::BUSY.read(backend.read(Reg::Csr)) {
        return Err(Error::Busy);
    }
    // Do not RMW status flags into a new configuration word.
    backend.write(Reg::Csr, csr::MODE.write(0, op.mode as u32));
    backend.write(Reg::Dividend, op.dividend);
    if let Some(divisor) = op.divisor {
        backend.write(Reg::Divisor, divisor);
    }
    for _ in 0..poll_budget {
        let status = backend.read(Reg::Csr);
        if !csr::BUSY.read(status) {
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
