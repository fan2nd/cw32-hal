//! CW32L012 CORDIC, bounded polling in Q1.31 (not the STM32 CORDIC interface).
//!
//! The device has separate X/Y/Z registers. Writing the final input starts a
//! calculation; there is no START bit. A two-input operation writes X then Y.
//! Inputs and results are signed Q1.31; angles are in units of pi, not radians.
//! Blocking access owns the CORDIC token; `into_async` adds verified IRQ ownership.
//!
//! Evidence: CW32L012 User Manual v1.4, sections 12.3--12.6 (printed pp.156--161),
//! and the vendor `cw32l012_cordic` driver/examples. The overview's "6--66"
//! iterations conflicts with the CSR table and SDK; this driver follows their
//! 4-bit encoding of 2,4,...,32 iterations. COMP=0 requests hardware gain
//! compensation, as the CSR table specifies. The SDK sqrt example uses COMP=1
//! despite labelling its output as compensated; that setting is not copied.
//! Hardware numerical accuracy and endpoint behavior still require board validation.
//! Q1.15 and DMA are intentionally not exposed.

use crate::{pac, peripherals::CORDIC, rcc::PeripheralClock, Peri};
use pac::cordic::regs;

/// Signed fixed point: the mathematical value is `bits / 2^31`, in [-1, 1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Q31(i32);

impl Q31 {
    pub const ZERO: Self = Self(0);
    pub const MIN: Self = Self(i32::MIN);
    pub const MAX: Self = Self(i32::MAX);

    pub const fn from_bits(bits: i32) -> Self {
        Self(bits)
    }
    pub const fn to_bits(self) -> i32 {
        self.0
    }
    /// Convert a fraction, truncating toward zero. Rejects division by zero and
    /// values outside [-1,1); +1 is not representable and is never wrapped.
    pub fn from_ratio(numerator: i32, denominator: u32) -> Result<Self, Error> {
        let n = i64::from(numerator);
        let d = i64::from(denominator);
        if d == 0 || n < -d || n >= d {
            return Err(Error::InputOutOfRange);
        }
        Ok(Self(((n * (1i64 << 31)) / d) as i32))
    }
    /// Approximate floating-point view. No floating-point operation is needed
    /// by the hardware driver itself; Cortex-M0+ uses software for this helper.
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 2_147_483_648.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidIterations,
    InvalidScale,
    InputOutOfRange,
    /// The angle of (0,0) is undefined.
    UndefinedPhase,
    /// A previous timed-out calculation is still running. No registers written.
    Busy,
    /// Polling allowance exhausted; the hardware operation is not cancelled.
    Timeout,
    /// A zero allowance is rejected before accessing hardware.
    ZeroPollBudget,
}

/// CSR ITER encoding: 0 means 2 iterations and 15 means 32.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Iterations(u8);
impl Iterations {
    pub const fn new(count: u8) -> Result<Self, Error> {
        if count < 2 || count > 32 || count % 2 != 0 {
            Err(Error::InvalidIterations)
        } else {
            Ok(Self(count / 2 - 1))
        }
    }
    pub const fn count(self) -> u8 {
        (self.0 + 1) * 2
    }
}
impl Default for Iterations {
    fn default() -> Self {
        Self(15)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    /// The manual recommends 24--32 iterations for Q1.31. Default: 32.
    pub iterations: Iterations,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SinCos {
    pub sin: Q31,
    pub cos: Q31,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Polar {
    /// atan2(y,x) / pi, signed Q1.31.
    pub angle_pi: Q31,
    /// sqrt(x*x + y*y) / 2. The division by TWO is part of the hardware API.
    pub half_magnitude: Q31,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hyperbolic {
    /// sinh(2*z) / 2.
    pub half_sinh: Q31,
    /// cosh(2*z) / 2.
    pub half_cosh: Q31,
}

/// Exclusive, synchronous access to the hardware accelerator.
///
/// Dropping this object consumes its token and does not gate the shared bus or
/// enable an interrupt. `release` explicitly resets and returns the token.
/// After a timeout, call `reset` or try another operation once BUSY is clear.
pub struct Cordic<'d> {
    token: Peri<'d, CORDIC>,
    config: Config,
}

impl<'d> Cordic<'d> {
    pub fn new(token: Peri<'d, CORDIC>) -> Self {
        Self::with_config(token, Config::default())
    }
    pub fn with_config(token: Peri<'d, CORDIC>, config: Config) -> Self {
        <CORDIC as PeripheralClock>::enable_and_reset();
        Self { token, config }
    }
    /// Abort any previous operation and discard its result via peripheral reset.
    pub fn reset(&mut self) {
        <CORDIC as PeripheralClock>::enable_and_reset();
    }
    pub fn release(self) -> Peri<'d, CORDIC> {
        <CORDIC as PeripheralClock>::enable_and_reset();
        self.token
    }
    pub fn is_busy(&self) -> bool {
        // SAFETY: owned token and clock enabled by construction; CSR is RO for status.
        pac::CORDIC.csr().read().busy()
    }
    /// Return both sin(angle*pi) and cos(angle*pi) in one hardware operation.
    /// `poll_budget` bounds CSR polls, not elapsed time or HCLK cycles.
    pub fn sin_cos(&mut self, angle_pi: Q31, poll_budget: u32) -> Result<SinCos, Error> {
        let r = self.calculate(
            Operation::single(Function::Cos, 0, Reg::Z, angle_pi, 3)?,
            poll_budget,
        )?;
        Ok(SinCos {
            cos: r[0],
            sin: r[1],
        })
    }
    /// Return the phase and HALF magnitude. Rejects the undefined phase at (0,0).
    pub fn polar(&mut self, x: Q31, y: Q31, poll_budget: u32) -> Result<Polar, Error> {
        let r = self.calculate(Operation::pair(Function::Atan2, x, y)?, poll_budget)?;
        Ok(Polar {
            angle_pi: r[2],
            half_magnitude: r[0],
        })
    }
    /// Return sqrt(x*x+y*y)/2, including a zero vector. Does not return full hypot.
    pub fn half_magnitude(&mut self, x: Q31, y: Q31, poll_budget: u32) -> Result<Q31, Error> {
        Ok(self.calculate(Operation::pair(Function::Hypot, x, y)?, poll_budget)?[0])
    }
    /// atan(2^scale * y) / pi. Valid scale: 0..=7.
    pub fn atan(&mut self, y: Q31, scale: u8, poll_budget: u32) -> Result<Q31, Error> {
        Ok(self.calculate(
            Operation::single(Function::Atan, scale, Reg::Y, y, 4)?,
            poll_budget,
        )?[2])
    }
    /// Return cosh(2*z)/2 and sinh(2*z)/2. Input z must lie in [-0.559,0.559].
    pub fn hyperbolic(&mut self, z: Q31, poll_budget: u32) -> Result<Hyperbolic, Error> {
        let r = self.calculate(
            Operation::single(Function::Cosh, 1, Reg::Z, z, 3)?,
            poll_budget,
        )?;
        Ok(Hyperbolic {
            half_cosh: r[0],
            half_sinh: r[1],
        })
    }
    /// atanh(2*y)/2. Input y must lie in [-0.403,0.403].
    pub fn atanh(&mut self, y: Q31, poll_budget: u32) -> Result<Q31, Error> {
        Ok(self.calculate(
            Operation::single(Function::Atanh, 1, Reg::Y, y, 4)?,
            poll_budget,
        )?[2])
    }
    /// Scaled natural logarithm, using the exact manual's argument convention:
    ///
    /// - scale 1: x in [0.0535,0.5), result ln(2*x)/4
    /// - scale 2: x in [0.25,0.75), result ln(4*x)/2
    /// - scale 3: x in [0.375,0.875), result ln(8*x)/2
    /// - scale 4: x in [0.4375,0.584), result ln(16*x)/4
    pub fn ln_scaled(&mut self, x: Q31, scale: u8, poll_budget: u32) -> Result<Q31, Error> {
        Ok(self.calculate(
            Operation::single(Function::Ln, scale, Reg::X, x, 4)?,
            poll_budget,
        )?[2])
    }
    /// Scaled square root, with hardware gain compensation enabled:
    ///
    /// - scale 0: x in [0.027,0.75), result sqrt(x)
    /// - scale 1: x in [0.375,0.875), result sqrt(2*x)/2
    /// - scale 2: x in [0.4375,0.585], result sqrt(4*x)/2
    ///
    /// These are hardware convergence domains; this is not a general-purpose
    /// sqrt routine. Values outside them are rejected before any MMIO.
    pub fn sqrt_scaled(&mut self, x: Q31, scale: u8, poll_budget: u32) -> Result<Q31, Error> {
        Ok(self.calculate(
            Operation::single(Function::Sqrt, scale, Reg::X, x, 1)?,
            poll_budget,
        )?[0])
    }
    fn calculate(&mut self, op: Operation, poll_budget: u32) -> Result<[Q31; 3], Error> {
        run(&mut Hardware, self.config, op, poll_budget)
    }
}

static EVENT: crate::async_support::EventState = crate::async_support::EventState::new();
static RESULTS: critical_section::Mutex<core::cell::Cell<[Q31; 3]>> =
    critical_section::Mutex::new(core::cell::Cell::new([Q31::ZERO; 3]));

/// Completion handler for the CORDIC IRQ (RM 12.6.1).
pub struct InterruptHandler;
impl crate::interrupt::typelevel::Handler<crate::interrupt::typelevel::CORDIC>
    for InterruptHandler
{
    unsafe fn on_interrupt() {
        let waker = critical_section::with(|cs| {
            if let Some(result) = complete_async(&mut Hardware) {
                RESULTS.borrow(cs).set(result);
                EVENT.latch(1)
            } else {
                None
            }
        });
        if let Some(waker) = waker {
            waker.wake();
        }
    }
}
fn complete_async(hw: &mut impl Backend) -> Option<[Q31; 3]> {
    let mut status = regs::Csr(hw.read(Reg::Csr));
    if !status.ie() || !status.eoc() || status.busy() {
        return None;
    }
    status.set_ie(false);
    hw.write(Reg::Csr, status.0);
    // The first read clears EOC, not the result bank. No new operation can start
    // while the future holds the exclusive owner borrow.
    Some([Reg::X, Reg::Y, Reg::Z].map(|r| Q31::from_bits(hw.read(r) as i32)))
}
fn begin_async(hw: &mut impl Backend, config: Config, op: &Operation) -> Result<(), Error> {
    if regs::Csr(hw.read(Reg::Csr)).busy() {
        return Err(Error::Busy);
    }
    EVENT.reset();
    // A result read drains stale EOC; final input triggers the new operation.
    for reg in [Reg::X, Reg::Y, Reg::Z] {
        let _ = hw.read(reg);
    }
    let mut control = regs::Csr(control_word(config, op));
    control.set_ie(true);
    hw.write(Reg::Csr, control.0);
    for (index, reg) in [Reg::X, Reg::Y, Reg::Z].iter().enumerate() {
        if let Some(value) = op.inputs[index] {
            hw.write(*reg, value.to_bits() as u32);
        }
    }
    Ok(())
}
impl<'d> Cordic<'d> {
    /// Convert the existing exclusive driver into an IRQ-driven owner.
    ///
    pub fn into_async(
        self,
        _irq: impl crate::interrupt::typelevel::Binding<
            crate::interrupt::typelevel::CORDIC,
            InterruptHandler,
        >,
    ) -> AsyncCordic<'d> {
        use crate::interrupt::typelevel::Interrupt;
        critical_section::with(|_| {
            self.reset_irq_state();
            // Do not unpend/disable NVIC: peripheral flags/IE are sufficient.
            unsafe {
                crate::interrupt::typelevel::CORDIC::enable();
            }
        });
        AsyncCordic { inner: Some(self) }
    }
    fn reset_irq_state(&self) {
        <CORDIC as PeripheralClock>::enable_and_reset();
        EVENT.reset();
    }
}
/// IRQ-driven Q1.31 operations. Every method is lazy until first poll. Dropping
/// an in-flight future masks IE and resets the dedicated accelerator, discarding
/// its result. The next operation can be started immediately. No DMA is used.
pub struct AsyncCordic<'d> {
    inner: Option<Cordic<'d>>,
}
impl<'d> AsyncCordic<'d> {
    pub fn into_blocking(mut self) -> Cordic<'d> {
        let inner = self.inner.take().unwrap();
        inner.reset_irq_state();
        inner
    }
    async fn calculate(&mut self, op: Operation) -> Result<[Q31; 3], Error> {
        let guard = critical_section::with(|_| {
            begin_async(&mut Hardware, self.inner.as_ref().unwrap().config, &op)?;
            Ok(Cancel)
        })?;
        let result = core::future::poll_fn(|cx| {
            EVENT.register(cx.waker());
            if EVENT.take() != 0 {
                core::task::Poll::Ready(critical_section::with(|cs| RESULTS.borrow(cs).get()))
            } else {
                core::task::Poll::Pending
            }
        })
        .await;
        drop(guard);
        Ok(result)
    }
    /// Return both sin(angle*pi) and cos(angle*pi) in one hardware operation.
    pub async fn sin_cos(&mut self, angle_pi: Q31) -> Result<SinCos, Error> {
        let r = self
            .calculate(Operation::single(Function::Cos, 0, Reg::Z, angle_pi, 3)?)
            .await?;
        Ok(SinCos {
            cos: r[0],
            sin: r[1],
        })
    }
    /// Return the phase and HALF magnitude. Rejects the undefined phase at (0,0).
    pub async fn polar(&mut self, x: Q31, y: Q31) -> Result<Polar, Error> {
        let r = self
            .calculate(Operation::pair(Function::Atan2, x, y)?)
            .await?;
        Ok(Polar {
            angle_pi: r[2],
            half_magnitude: r[0],
        })
    }
    /// Return sqrt(x*x+y*y)/2, including a zero vector. Does not return full hypot.
    pub async fn half_magnitude(&mut self, x: Q31, y: Q31) -> Result<Q31, Error> {
        Ok(self
            .calculate(Operation::pair(Function::Hypot, x, y)?)
            .await?[0])
    }
    /// atan(2^scale * y) / pi. Valid scale: 0..=7.
    pub async fn atan(&mut self, y: Q31, scale: u8) -> Result<Q31, Error> {
        Ok(self
            .calculate(Operation::single(Function::Atan, scale, Reg::Y, y, 4)?)
            .await?[2])
    }
    /// Return cosh(2*z)/2 and sinh(2*z)/2. Input z must lie in [-0.559,0.559].
    pub async fn hyperbolic(&mut self, z: Q31) -> Result<Hyperbolic, Error> {
        let r = self
            .calculate(Operation::single(Function::Cosh, 1, Reg::Z, z, 3)?)
            .await?;
        Ok(Hyperbolic {
            half_cosh: r[0],
            half_sinh: r[1],
        })
    }
    /// atanh(2*y)/2. Input y must lie in [-0.403,0.403].
    pub async fn atanh(&mut self, y: Q31) -> Result<Q31, Error> {
        Ok(self
            .calculate(Operation::single(Function::Atanh, 1, Reg::Y, y, 4)?)
            .await?[2])
    }
    /// Scaled natural logarithm, using the exact manual's argument convention:
    ///
    /// - scale 1: x in [0.0535,0.5), result ln(2*x)/4
    /// - scale 2: x in [0.25,0.75), result ln(4*x)/2
    /// - scale 3: x in [0.375,0.875), result ln(8*x)/2
    /// - scale 4: x in [0.4375,0.584), result ln(16*x)/4
    pub async fn ln_scaled(&mut self, x: Q31, scale: u8) -> Result<Q31, Error> {
        Ok(self
            .calculate(Operation::single(Function::Ln, scale, Reg::X, x, 4)?)
            .await?[2])
    }
    /// Scaled square root, with hardware gain compensation enabled:
    ///
    /// - scale 0: x in [0.027,0.75), result sqrt(x)
    /// - scale 1: x in [0.375,0.875), result sqrt(2*x)/2
    /// - scale 2: x in [0.4375,0.585], result sqrt(4*x)/2
    ///
    /// These are hardware convergence domains; this is not a general-purpose
    /// sqrt routine. Values outside them are rejected before any MMIO.
    pub async fn sqrt_scaled(&mut self, x: Q31, scale: u8) -> Result<Q31, Error> {
        Ok(self
            .calculate(Operation::single(Function::Sqrt, scale, Reg::X, x, 1)?)
            .await?[0])
    }
}
impl Drop for AsyncCordic<'_> {
    fn drop(&mut self) {
        if let Some(inner) = &self.inner {
            critical_section::with(|_| inner.reset_irq_state());
        }
    }
}

struct Cancel;
impl Drop for Cancel {
    fn drop(&mut self) {
        critical_section::with(|_| {
            // Dedicated AHBRST.CORDIC reset cancels hardware, including a
            // computation that has not reached EOC. No shared reset is asserted.
            let mut hw = Hardware;
            let mut status = regs::Csr(hw.read(Reg::Csr));
            status.set_ie(false);
            hw.write(Reg::Csr, status.0);
            <CORDIC as PeripheralClock>::enable_and_reset();
            EVENT.reset();
        });
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
enum Function {
    Cos = 0,
    Atan2 = 2,
    Hypot = 3,
    Atan = 4,
    Cosh = 5,
    Atanh = 7,
    Ln = 8,
    Sqrt = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reg {
    Csr,
    X,
    Y,
    Z,
}

struct Operation {
    function: Function,
    scale: u8,
    inputs: [Option<Q31>; 3],
    outputs: u8,
}
impl Operation {
    fn pair(function: Function, x: Q31, y: Q31) -> Result<Self, Error> {
        if function == Function::Atan2 && x == Q31::ZERO && y == Q31::ZERO {
            return Err(Error::UndefinedPhase);
        }
        Ok(Self {
            function,
            scale: 0,
            inputs: [Some(x), Some(y), None],
            outputs: if function == Function::Atan2 { 5 } else { 1 },
        })
    }
    fn single(
        function: Function,
        scale: u8,
        reg: Reg,
        value: Q31,
        outputs: u8,
    ) -> Result<Self, Error> {
        validate_input(function, scale, value)?;
        let mut inputs = [None; 3];
        let index = match reg {
            Reg::X => 0,
            Reg::Y => 1,
            Reg::Z => 2,
            Reg::Csr => unreachable!(),
        };
        inputs[index] = Some(value);
        Ok(Self {
            function,
            scale,
            inputs,
            outputs,
        })
    }
}

// Exact decimal rational comparisons prevent rounded Q31 thresholds from
// admitting one-LSB-outside inputs. The largest product is < 2^48.
fn in_domain(value: Q31, low: i32, high: i32, denominator: i32, high_inclusive: bool) -> bool {
    let value = i64::from(value.0) * i64::from(denominator);
    let low = i64::from(low) * (1i64 << 31);
    let high = i64::from(high) * (1i64 << 31);
    value >= low
        && if high_inclusive {
            value <= high
        } else {
            value < high
        }
}
fn validate_input(function: Function, scale: u8, value: Q31) -> Result<(), Error> {
    let valid = match (function, scale) {
        (Function::Cos, 0) | (Function::Atan, 0..=7) => true,
        (Function::Cosh, 1) => in_domain(value, -559, 559, 1000, true),
        (Function::Atanh, 1) => in_domain(value, -403, 403, 1000, true),
        (Function::Ln, 1) => in_domain(value, 535, 5000, 10000, false),
        (Function::Ln, 2) => in_domain(value, 1, 3, 4, false),
        (Function::Ln, 3) => in_domain(value, 3, 7, 8, false),
        (Function::Ln, 4) => in_domain(value, 4375, 5840, 10000, false),
        (Function::Sqrt, 0) => in_domain(value, 27, 750, 1000, false),
        (Function::Sqrt, 1) => in_domain(value, 3, 7, 8, false),
        (Function::Sqrt, 2) => in_domain(value, 4375, 5850, 10000, true),
        _ => return Err(Error::InvalidScale),
    };
    if valid {
        Ok(())
    } else {
        Err(Error::InputOutOfRange)
    }
}

fn control_word(config: Config, op: &Operation) -> u32 {
    let mut word = regs::Csr(0);
    word.set_func(op.function as u8);
    word.set_scale(op.scale);
    word.set_format(true); // Q1.31, unlike STM32 encoding.
    word.set_iter(config.iterations.0);
    // COMP=0: hardware gain compensation; IE/DMAEOC/DMAIDLE remain disabled.
    word.set_comp(false);
    word.0
}

trait Backend {
    fn read(&mut self, reg: Reg) -> u32;
    fn write(&mut self, reg: Reg, word: u32);
}
struct Hardware;
impl Backend for Hardware {
    fn read(&mut self, reg: Reg) -> u32 {
        // Owned and clocked Cordic; CSR polling does not drain result flags.
        // Each result access remains exactly one read, which clears EOC.
        match reg {
            Reg::Csr => pac::CORDIC.csr().read().0,
            Reg::X => pac::CORDIC.x().read().0,
            Reg::Y => pac::CORDIC.y().read().0,
            Reg::Z => pac::CORDIC.z().read().0,
        }
    }
    fn write(&mut self, reg: Reg, word: u32) {
        // Validated configuration and inputs cross the raw algorithm boundary
        // exactly once, retaining the hardware's final-input trigger ordering.
        match reg {
            Reg::Csr => pac::CORDIC.csr().write_value(regs::Csr(word)),
            Reg::X => pac::CORDIC.x().write_value(regs::X(word)),
            Reg::Y => pac::CORDIC.y().write_value(regs::Y(word)),
            Reg::Z => pac::CORDIC.z().write_value(regs::Z(word)),
        }
    }
}

fn run(
    backend: &mut impl Backend,
    config: Config,
    op: Operation,
    poll_budget: u32,
) -> Result<[Q31; 3], Error> {
    if poll_budget == 0 {
        return Err(Error::ZeroPollBudget);
    }
    if regs::Csr(backend.read(Reg::Csr)).busy() {
        return Err(Error::Busy);
    }
    backend.write(Reg::Csr, control_word(config, &op));
    let registers = [Reg::X, Reg::Y, Reg::Z];
    for (index, reg) in registers.iter().enumerate() {
        if let Some(value) = op.inputs[index] {
            backend.write(*reg, value.to_bits() as u32);
        }
    }
    for _ in 0..poll_budget {
        let status = regs::Csr(backend.read(Reg::Csr));
        if !status.busy() && status.eoc() {
            let mut results = [Q31::ZERO; 3];
            // The first data read clears EOC, but all results remain valid while
            // idle. Do not wait for EOC again between two results of one operation.
            for (index, reg) in registers.iter().enumerate() {
                if op.outputs & (1 << index) != 0 {
                    results[index] = Q31::from_bits(backend.read(*reg) as i32);
                }
            }
            return Ok(results);
        }
        core::hint::spin_loop();
    }
    Err(Error::Timeout)
}
