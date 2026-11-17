//! L012 blocking and async CORDIC operations in signed Q1.31, with checked input domains.
//!
//! Angles for circular functions use units of pi. Hyperbolic arguments do not.
//! Every method documents the hardware's scaled output; no floating-point
//! conversion, software fallback, range extension or post-correction is hidden.
//! The maximum ITER table encoding is selected. The manual's ITER prose conflicts
//! with its table; no execution-time or numeric accuracy guarantee is made.
//!
//! Budgets count CSR reads, not time. Timeout does not abort. A subsequent call
//! refuses busy hardware; once idle, it may discard an unread timed-out result.
//! Drop never waits or resets and retains the gate while busy. DMA and
//! Q1.15 are outside this driver's scope. See `docs/l012-math-evidence.md`.

use crate::{
    Peri, PeripheralType,
    interrupt::typelevel::{Binding, CORDIC, Handler, Interrupt},
    pac,
    rcc::RccPeripheral,
};
use core::{
    cell::{Cell, RefCell},
    future::poll_fn,
    marker::PhantomData,
    task::{Poll, Waker},
};
use critical_section::Mutex;
use pac::cordic::vals::{Comp, Format, Func, Iter, Scale};
mod domains {
    include!(concat!(env!("OUT_DIR"), "/_cordic_domains.rs"));
}

/// Signed Q1.31 bit pattern used by the CORDIC peripheral.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct Q1_31(i32);
impl Q1_31 {
    pub const fn from_bits(bits: i32) -> Self { Self(bits) }
    pub const fn to_bits(self) -> i32 { self.0 }
}

/// Signed Q1.15 value shared with motor-control fixed-point code.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(transparent)]
pub struct Q1_15(i16);
impl Q1_15 {
    pub const fn from_bits(bits: i16) -> Self { Self(bits) }
    pub const fn to_bits(self) -> i16 { self.0 }
}

pub const Q15_ONE: i32 = 32_767;

#[inline(always)]
pub const fn q15_mul_wide(lhs: i64, rhs: i64) -> i64 { (lhs * rhs) >> 15 }

/// Q1.15 multiply for bounded real-time paths. Q15 operands and the FOC
/// current ranges fit in a signed 32-bit product; keep this separate from the
/// widened helper so Cortex-M0 ISRs do not pay for software 64-bit multiply.
#[inline(always)]
pub const fn q15_mul(lhs: i32, rhs: i32) -> i32 { (lhs * rhs) >> 15 }

#[inline(always)]
pub const fn q15_clamp(value: i64) -> Q1_15 {
    let value = if value < -32_767 { -32_767 } else if value > 32_767 { 32_767 } else { value };
    Q1_15(value as i16)
}

/// Startup, input-domain or bounded-wait failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// Peripheral reset is asserted; no reset is released or pulsed.
    HeldInReset,
    /// Clock enable did not read back as enabled.
    ClockNotEnabled,
    /// An earlier operation is busy; no control or operand was written.
    Busy,
    /// Status-read budget expired; the operation may still be running.
    Timeout,
    /// Operand is outside this API's documented domain; no operation was started.
    OutOfDomain,
}

/// Circular sine/cosine pair, both in signed Q1.31.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SinCos {
    /// sin(angle * pi), from Y.
    pub sin: Q1_31,
    /// cos(angle * pi), from X.
    pub cos: Q1_31,
}
/// Scaled hyperbolic outputs, both in signed Q1.31.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SinhCosh {
    /// sinh(2 * input) / 2, from Y.
    pub sinh_half: Q1_31,
    /// cosh(2 * input) / 2, from X.
    pub cosh_half: Q1_31,
}

/// Input multiplier for hardware arctangent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AtanScale {
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
    X64,
    X128,
}
impl AtanScale {
    fn value(self) -> Scale {
        match self {
            Self::X1 => Scale::Scale0,
            Self::X2 => Scale::Scale1,
            Self::X4 => Scale::Scale2,
            Self::X8 => Scale::Scale3,
            Self::X16 => Scale::Scale4,
            Self::X32 => Scale::Scale5,
            Self::X64 => Scale::Scale6,
            Self::X128 => Scale::Scale7,
        }
    }
}
/// Natural-logarithm input domain and output scaling, as specified by hardware.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LogScale {
    /// x in [0.to_bits()535, 0.5): return ln(2*x)/4.
    X2Over4,
    /// x in [0.25, 0.75): return ln(4*x)/2.
    X4Over2,
    /// x in [0.375, 0.875): return ln(8*x)/2.
    X8Over2,
    /// x in [0.4375, 0.584): return ln(16*x)/4.
    X16Over4,
}
impl LogScale {
    fn parameters(self) -> (Scale, (i32, i32)) {
        match self {
            Self::X2Over4 => (Scale::Scale1, domains::LN1),
            Self::X4Over2 => (Scale::Scale2, domains::LN2),
            Self::X8Over2 => (Scale::Scale3, domains::LN3),
            Self::X16Over4 => (Scale::Scale4, domains::LN4),
        }
    }
}
/// Fixed-point square-root input domain and output scaling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SqrtScale {
    /// x in [0.to_bits()27, 0.75): return sqrt(x).
    X1,
    /// x in [0.375, 0.875): return sqrt(2*x)/2.
    X2Over2,
    /// x in [0.4375, 0.585]: return sqrt(4*x)/2.
    X4Over2,
}
impl SqrtScale {
    fn parameters(self) -> (Scale, (i32, i32)) {
        match self {
            Self::X1 => (Scale::Scale0, domains::SQRT0),
            Self::X2Over2 => (Scale::Scale1, domains::SQRT1),
            Self::X4Over2 => (Scale::Scale2, domains::SQRT2),
        }
    }
}

trait SealedInstance {
    fn regs() -> pac::cordic::Cordic;
}
/// A source-qualified CORDIC instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + RccPeripheral {}
impl SealedInstance for crate::peripherals::CORDIC {
    fn regs() -> pac::cordic::Cordic {
        pac::CORDIC
    }
}
impl Instance for crate::peripherals::CORDIC {}

struct State {
    waiting: Mutex<Cell<bool>>,
    waker: Mutex<RefCell<Option<Waker>>>,
}
static STATE: State = State {
    waiting: Mutex::new(Cell::new(false)),
    waker: Mutex::new(RefCell::new(None)),
};
/// Completion handler; masks IE and wakes without reading results or aborting.
pub struct InterruptHandler<T: Instance>(PhantomData<T>);
impl<T: Instance> Handler<CORDIC> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let wake = critical_section::with(|cs| {
            if !STATE.waiting.borrow(cs).get() {
                return None;
            }
            let r = T::regs();
            let status = r.csr().read();
            if status.ie() && !status.busy() && status.eoc() {
                r.csr().modify(|w| w.set_ie(false));
                STATE.waker.borrow(cs).borrow_mut().take()
            } else {
                None
            }
        });
        if let Some(waker) = wake {
            waker.wake();
        }
    }
}
fn retire_wait<T: Instance>() {
    let stale = critical_section::with(|cs| {
        T::regs().csr().modify(|w| w.set_ie(false));
        STATE.waiting.borrow(cs).set(false);
        STATE.waker.borrow(cs).borrow_mut().take()
    });
    drop(stale);
}
struct CalculationWait<T: Instance>(PhantomData<T>);
impl<T: Instance> Drop for CalculationWait<T> {
    fn drop(&mut self) {
        retire_wait::<T>();
    }
}

/// Exclusive CORDIC owner with blocking and IRQ-backed operations.
pub struct Cordic<'d, T: Instance> {
    _peripheral: Peri<'d, T>,
}
impl<'d, T: Instance> Cordic<'d, T> {
    /// Enable the clock while preserving resets and shared resources.
    /// Busy startup leaves the clock running without writing CORDIC controls.
    pub fn new(peripheral: Peri<'d, T>) -> Result<Self, Error> {
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
        retire_wait::<T>();
        // No operand is written; disable inherited request sources while idle.
        Self::configure(Func::Cos, Scale::Scale0);
        Ok(Self {
            _peripheral: peripheral,
        })
    }
    /// Read BUSY without consuming a result or modifying hardware.
    pub fn is_busy(&self) -> bool {
        T::regs().csr().read().busy()
    }

    /// sin/cos of `angle * pi`, for angle in [-1, 1).
    /// Hardware quantization of exact +1 endpoints is not specified; this method
    /// returns the hardware bits without promising saturation or rounding.
    /// At most `polls` CSR reads follow submission; zero submits nothing.
    pub fn sin_cos(&mut self, angle: Q1_31, polls: u32) -> Result<SinCos, Error> {
        self.prepare(Func::Cos, Scale::Scale0, polls)?;
        let r = T::regs();
        r.z().write(|w| w.set_z(angle.to_bits() as u32));
        self.wait(polls)?;
        let cos = Q1_31::from_bits(r.x().read().x() as i32);
        let sin = Q1_31::from_bits(r.y().read().y() as i32);
        Ok(SinCos { sin, cos })
    }
    /// atan(y/x)/pi, using the hardware phase function. Requires x > 0.
    /// This intentionally excludes zero/negative-x quadrant and origin cases,
    /// whose branch-cut behavior is not specified in the manual's atan(y/x) table.
    pub fn phase(&mut self, x: Q1_31, y: Q1_31, polls: u32) -> Result<Q1_31, Error> {
        if x.to_bits() <= 0 {
            return Err(Error::OutOfDomain);
        }
        self.prepare(Func::Atan2, Scale::Scale0, polls)?;
        let r = T::regs();
        r.x().write(|w| w.set_x(x.to_bits() as u32));
        r.y().write(|w| w.set_y(y.to_bits() as u32));
        self.wait(polls)?;
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// sqrt(x*x + y*y)/2, for both inputs in [-1, 1), with hardware K compensation.
    /// The factor 1/2 is part of the documented hardware result.
    pub fn hypot_half(&mut self, x: Q1_31, y: Q1_31, polls: u32) -> Result<Q1_31, Error> {
        self.prepare(Func::Hypot, Scale::Scale0, polls)?;
        let r = T::regs();
        r.x().write(|w| w.set_x(x.to_bits() as u32));
        r.y().write(|w| w.set_y(y.to_bits() as u32));
        self.wait(polls)?;
        Ok(Q1_31::from_bits(r.x().read().x() as i32))
    }
    /// atan(multiplier*y)/pi, for y in [-1, 1), with multiplier chosen by `scale`.
    pub fn atan(&mut self, y: Q1_31, scale: AtanScale, polls: u32) -> Result<Q1_31, Error> {
        self.prepare(Func::Atan, scale.value(), polls)?;
        let r = T::regs();
        r.y().write(|w| w.set_y(y.to_bits() as u32));
        self.wait(polls)?;
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// Return sinh(2*z)/2 and cosh(2*z)/2; z must be in [-0.559, 0.559].
    pub fn sinh_cosh_half(&mut self, z: Q1_31, polls: u32) -> Result<SinhCosh, Error> {
        check_domain(z, domains::HYPERBOLIC)?;
        self.prepare(Func::Cosh, Scale::Scale1, polls)?;
        let r = T::regs();
        r.z().write(|w| w.set_z(z.to_bits() as u32));
        self.wait(polls)?;
        let cosh_half = Q1_31::from_bits(r.x().read().x() as i32);
        let sinh_half = Q1_31::from_bits(r.y().read().y() as i32);
        Ok(SinhCosh {
            sinh_half,
            cosh_half,
        })
    }
    /// atanh(2*y)/2, for y in [-0.403, 0.403].
    pub fn atanh_half(&mut self, y: Q1_31, polls: u32) -> Result<Q1_31, Error> {
        check_domain(y, domains::ATANH)?;
        self.prepare(Func::Atanh, Scale::Scale1, polls)?;
        let r = T::regs();
        r.y().write(|w| w.set_y(y.to_bits() as u32));
        self.wait(polls)?;
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// Natural logarithm with the exact input domain/output scaling in [`LogScale`].
    pub fn ln(&mut self, x: Q1_31, scale: LogScale, polls: u32) -> Result<Q1_31, Error> {
        let (scale, domain) = scale.parameters();
        check_domain(x, domain)?;
        self.prepare(Func::Ln, scale, polls)?;
        let r = T::regs();
        r.x().write(|w| w.set_x(x.to_bits() as u32));
        self.wait(polls)?;
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// Fixed-point root with [`SqrtScale`] domain/scaling and hardware K compensation.
    /// For full-range unsigned integer roots, use [`crate::eau::Eau::square_root`].
    pub fn square_root(&mut self, x: Q1_31, scale: SqrtScale, polls: u32) -> Result<Q1_31, Error> {
        let (scale, domain) = scale.parameters();
        check_domain(x, domain)?;
        self.prepare(Func::Sqrt, scale, polls)?;
        let r = T::regs();
        r.x().write(|w| w.set_x(x.to_bits() as u32));
        self.wait(polls)?;
        Ok(Q1_31::from_bits(r.x().read().x() as i32))
    }
    /// IRQ-backed variant of [`Self::sin_cos`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn sin_cos_async(
        &mut self,
        angle: Q1_31,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<SinCos, Error> {
        self.prepare(Func::Cos, Scale::Scale0, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.z().write(|w| w.set_z(angle.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        let cos = Q1_31::from_bits(r.x().read().x() as i32);
        let sin = Q1_31::from_bits(r.y().read().y() as i32);
        Ok(SinCos { sin, cos })
    }
    /// IRQ-backed variant of [`Self::phase`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn phase_async(
        &mut self,
        x: Q1_31,
        y: Q1_31,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<Q1_31, Error> {
        if x.to_bits() <= 0 {
            return Err(Error::OutOfDomain);
        }
        self.prepare(Func::Atan2, Scale::Scale0, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.x().write(|w| w.set_x(x.to_bits() as u32));
            r.y().write(|w| w.set_y(y.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// IRQ-backed variant of [`Self::hypot_half`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn hypot_half_async(
        &mut self,
        x: Q1_31,
        y: Q1_31,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<Q1_31, Error> {
        self.prepare(Func::Hypot, Scale::Scale0, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.x().write(|w| w.set_x(x.to_bits() as u32));
            r.y().write(|w| w.set_y(y.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        Ok(Q1_31::from_bits(r.x().read().x() as i32))
    }
    /// IRQ-backed variant of [`Self::atan`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn atan_async(
        &mut self,
        y: Q1_31,
        scale: AtanScale,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<Q1_31, Error> {
        self.prepare(Func::Atan, scale.value(), 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.y().write(|w| w.set_y(y.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// IRQ-backed variant of [`Self::sinh_cosh_half`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn sinh_cosh_half_async(
        &mut self,
        z: Q1_31,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<SinhCosh, Error> {
        check_domain(z, domains::HYPERBOLIC)?;
        self.prepare(Func::Cosh, Scale::Scale1, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.z().write(|w| w.set_z(z.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        let cosh_half = Q1_31::from_bits(r.x().read().x() as i32);
        let sinh_half = Q1_31::from_bits(r.y().read().y() as i32);
        Ok(SinhCosh {
            sinh_half,
            cosh_half,
        })
    }
    /// IRQ-backed variant of [`Self::atanh_half`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn atanh_half_async(
        &mut self,
        y: Q1_31,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<Q1_31, Error> {
        check_domain(y, domains::ATANH)?;
        self.prepare(Func::Atanh, Scale::Scale1, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.y().write(|w| w.set_y(y.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// IRQ-backed variant of [`Self::ln`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn ln_async(
        &mut self,
        x: Q1_31,
        scale: LogScale,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<Q1_31, Error> {
        let (scale, domain) = scale.parameters();
        check_domain(x, domain)?;
        self.prepare(Func::Ln, scale, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.x().write(|w| w.set_x(x.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        Ok(Q1_31::from_bits(r.z().read().z() as i32))
    }
    /// IRQ-backed variant of [`Self::square_root`], with identical numeric domain.
    /// Cancellation masks IE but does not abort; hardware may remain busy.
    pub async fn square_root_async(
        &mut self,
        x: Q1_31,
        scale: SqrtScale,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
    ) -> Result<Q1_31, Error> {
        let (scale, domain) = scale.parameters();
        check_domain(x, domain)?;
        self.prepare(Func::Sqrt, scale, 1)?;
        let _wait = self.submit_async(_irq, |r| {
            r.x().write(|w| w.set_x(x.to_bits() as u32));
        });
        self.wait_async().await;
        let r = T::regs();
        Ok(Q1_31::from_bits(r.x().read().x() as i32))
    }
    fn submit_async(
        &mut self,
        _irq: impl Binding<CORDIC, InterruptHandler<T>>,
        submit: impl FnOnce(pac::cordic::Cordic),
    ) -> CalculationWait<T> {
        critical_section::with(|cs| {
            STATE.waiting.borrow(cs).set(true);
            // Arm before trigger while the ISR cannot observe a stale EOC.
            T::regs().csr().modify(|w| w.set_ie(true));
            submit(T::regs());
        });
        unsafe {
            CORDIC::enable();
        }
        CalculationWait(PhantomData)
    }
    async fn wait_async(&mut self) {
        poll_fn(|cx| {
            let candidate = cx.waker().clone();
            let (ready, previous) = critical_section::with(|cs| {
                let status = T::regs().csr().read();
                if !status.busy() && status.eoc() {
                    (true, Some(candidate))
                } else {
                    (
                        false,
                        STATE.waker.borrow(cs).borrow_mut().replace(candidate),
                    )
                }
            });
            drop(previous);
            if ready {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
    }
    fn prepare(&mut self, function: Func, scale: Scale, polls: u32) -> Result<(), Error> {
        if polls == 0 {
            return Err(Error::Timeout);
        }
        if self.is_busy() {
            return Err(Error::Busy);
        }
        retire_wait::<T>();
        Self::configure(function, scale);
        Ok(())
    }
    fn configure(function: Func, scale: Scale) {
        // CSR reset seed is zero. RO flags are never acknowledged by writes;
        // the documented trigger clears EOC. All DMA/IRQ sources remain disabled.
        T::regs().csr().write(|w| {
            w.set_func(function);
            w.set_scale(scale);
            w.set_format(Format::Q131);
            w.set_iter(Iter::Iter32);
            w.set_comp(Comp::Compensated);
            w.set_ie(false);
            w.set_dmaeoc(false);
            w.set_dmaidle(false);
        });
    }
    fn wait(&self, polls: u32) -> Result<(), Error> {
        for _ in 0..polls {
            let status = T::regs().csr().read();
            if !status.busy() && status.eoc() {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(Error::Timeout)
    }
}
fn check_domain(value: Q1_31, (minimum, maximum): (i32, i32)) -> Result<(), Error> {
    if (minimum..=maximum).contains(&value.to_bits()) {
        Ok(())
    } else {
        Err(Error::OutOfDomain)
    }
}
impl<T: Instance> Drop for Cordic<'_, T> {
    fn drop(&mut self) {
        retire_wait::<T>();
        if !self.is_busy() {
            critical_section::with(|cs| {
                let _ = T::RCC_INFO.disable_with_cs(cs);
            });
        }
    }
}
