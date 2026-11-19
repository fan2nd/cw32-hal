//! L012 operational amplifiers: external buffer, PGA and external feedback.
//!
//! API ownership follows Embassy STM32's `OpAmp` / `OpAmpOutput` pattern. Every
//! active positive input, negative input (external feedback only), and output pin
//! is retained for the complete output lifetime. Pins cannot simultaneously be
//! used for DAC/ADC/GPIO. Internal DAC/ADC interconnects are not exposed yet.
//!
//! `new` enables shared BGR but leaves the amplifier disabled. The caller must
//! establish BGR stability before selecting a mode. Its documented ~30 µs startup
//! is approximate, not a maximum. Enabling a mode does not establish readiness:
//! the OPA datasheet's 2.5 µs startup is a minimum and there is no ready flag.
//! Board-specific startup and signal/load settling must be qualified separately.
//!
//! VDDA must equal VDD and remain within the selected supply range. Inputs must
//! remain in 0..VDDA and linear output in 0.1..VDDA−0.1 V; load resistance must be
//! at least 8 kΩ for <1% output deviation. The board must also respect the 6 mA
//! maximum output current and all datasheet conditions. Gain is nominal. This
//! Calibration is exposed explicitly because current-sense front ends commonly
//! need the vendor soft auto-zero sequence before the ADC is started. See
//! `docs/dac-opa.md`.
//!
//! Shared BGR, ADC, VC and clock gates are never disabled/reset. The DAC gate is
//! retained so a preexisting competing output can be inspected safely. No other
//! amplifier is configured. Bias is the manual's 8 µA default; the unexplained
//! datasheet LPMODE reference never authorizes a reserved-bit write.

use crate::gpio::{Flex, Pin};
use crate::pac::opa::vals;
use crate::rcc::{RccPeripheral, SealedRccPeripheral};
use crate::{Peri, PeripheralType, pac, peripherals};

/// Nominal positive gain from the internal feedback network.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpAmpGain {
    /// Two times.
    Mul2,
    /// Four times.
    Mul4,
    /// Eight times.
    Mul8,
    /// Sixteen times.
    Mul16,
    /// Thirty-two times.
    Mul32,
}
impl OpAmpGain {
    fn raw(self) -> vals::Gain {
        match self {
            Self::Mul2 => vals::Gain::Mul2,
            Self::Mul4 => vals::Gain::Mul4,
            Self::Mul8 => vals::Gain::Mul8,
            Self::Mul16 => vals::Gain::Mul16,
            Self::Mul32 => vals::Gain::Mul32,
        }
    }
}
/// Board declaration, not measured rail voltage.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Config {
    /// VDDA in mV. Must equal VDD and remain in the instance's supply range.
    pub supply_mv: u16,
}
impl Default for Config {
    fn default() -> Self {
        Self { supply_mv: 3300 }
    }
}
/// Configuration or output conflict.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Invalid declared supply.
    InvalidSupplyVoltage,
    /// A needed gate did not acknowledge.
    Clock,
    /// A needed block is held in reset; this driver does not release it.
    HeldInReset,
    /// A matching DAC channel is enabled and connected to the same output pad.
    OutputBusy,
}
/// Disabled op-amp controller. Modes return a scoped active output.
pub struct OpAmp<'d, T: Instance> {
    _peri: Peri<'d, T>,
}
impl<'d, T: Instance> OpAmp<'d, T> {
    /// Enable clocks and shared BGR, configuring this amplifier disabled.
    ///
    /// Before calling a mode method, wait for BGR stability using a board-qualified
    /// interval. This method never claims BGR or the amplifier has settled.
    /// Calibration remains disabled. No sibling or shared reset is touched.
    pub fn new(peri: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        let (min, max) = T::SUPPLY_RANGE_MV;
        if !(min..=max).contains(&config.supply_mv) {
            return Err(Error::InvalidSupplyVoltage);
        }
        critical_section::with(|cs| {
            T::RCC_INFO.enable_with_cs(cs).map_err(|_| Error::Clock)?;
            peripherals::DAC::RCC_INFO
                .enable_with_cs(cs)
                .map_err(|_| Error::Clock)?;
            if T::RCC_INFO.reset_asserted() || peripherals::DAC::RCC_INFO.reset_asserted() {
                return Err(Error::HeldInReset);
            }
            // Additive enable only; preserve the temperature sensor and never undo
            // automatic or software reference requests from ADC/VC/sibling OPA.
            pac::BGR.cr().modify(|w| w.set_bgren(true));
            Ok(())
        })?;
        T::regs().cr().write(|w| w.set_bias(vals::Bias::Ua8));
        // START/CALEN=0, SOFTTRIG=0. AZRUN is read-only and RFU stays at reset.
        T::regs().cal().write(|_| {});
        Ok(Self { _peri: peri })
    }
    /// Source-qualified VDDA operating range in mV.
    pub const fn supply_voltage_range_mv() -> (u16, u16) {
        T::SUPPLY_RANGE_MV
    }
    /// Linear output must stay this far from both analog rails, in mV.
    pub const fn output_headroom_mv() -> u16 {
        T::OUTPUT_HEADROOM_MV
    }
    fn check_output() -> Result<(), Error> {
        let cr0 = pac::DAC.cr0().read();
        let cr1 = pac::DAC.cr1().read();
        let busy = if T::CHANNEL == 1 {
            cr0.en1() && cr1.c1out()
        } else {
            cr0.en2() && cr1.c2out()
        };
        if busy { Err(Error::OutputBusy) } else { Ok(()) }
    }
    /// Enable an internal unity-gain follower with one external positive input.
    /// BGR must already be stable. Return does not qualify analog readiness.
    pub fn buffer_ext<'a, P: NonInvertingPin<T>, O: OutputPin<T>>(
        &'a mut self,
        input: Peri<'a, P>,
        output: Peri<'a, O>,
    ) -> Result<OpAmpOutput<'a, 'd, T>, Error> {
        self.internal(input, output, vals::Mode::Follower, vals::Gain::Mul2)
    }
    /// Enable nominal positive gain from the internal network.
    /// All external negative-input switches are off. BGR must already be stable;
    /// the input, multiplied output and load must fit the analog voltage limits.
    pub fn pga_ext<'a, P: NonInvertingPin<T>, O: OutputPin<T>>(
        &'a mut self,
        input: Peri<'a, P>,
        output: Peri<'a, O>,
        gain: OpAmpGain,
    ) -> Result<OpAmpOutput<'a, 'd, T>, Error> {
        self.internal(input, output, vals::Mode::Pga, gain.raw())
    }
    fn internal<'a, P: NonInvertingPin<T>, O: OutputPin<T>>(
        &'a mut self,
        input: Peri<'a, P>,
        output: Peri<'a, O>,
        mode: vals::Mode,
        gain: vals::Gain,
    ) -> Result<OpAmpOutput<'a, 'd, T>, Error> {
        Self::check_output()?;
        let mut input = Flex::new(input);
        let mut output = Flex::new(output);
        input.set_as_analog();
        output.set_as_analog();
        T::regs().cr().write(|w| {
            w.set_bias(vals::Bias::Ua8);
            w.set_mode(mode);
            w.set_amp(gain);
            P::connect(w);
            w.set_en(true);
        });
        Ok(OpAmpOutput {
            _inner: self,
            _positive: input,
            _negative: None,
            _output: output,
        })
    }
    /// Enable external-feedback amplification. The board must supply a stable
    /// feedback network and keep both inputs, output and load in range.
    /// BGR must already be stable. Return does not establish analog readiness.
    pub fn standalone_ext<'a, P: NonInvertingPin<T>, N: InvertingPin<T>, O: OutputPin<T>>(
        &'a mut self,
        positive: Peri<'a, P>,
        negative: Peri<'a, N>,
        output: Peri<'a, O>,
    ) -> Result<OpAmpOutput<'a, 'd, T>, Error> {
        Self::check_output()?;
        let mut positive = Flex::new(positive);
        let mut negative = Flex::new(negative);
        let mut output = Flex::new(output);
        positive.set_as_analog();
        negative.set_as_analog();
        output.set_as_analog();
        T::regs().cr().write(|w| {
            w.set_bias(vals::Bias::Ua8);
            w.set_mode(vals::Mode::External);
            P::connect(w);
            N::connect(w);
            w.set_en(true);
        });
        Ok(OpAmpOutput {
            _inner: self,
            _positive: positive,
            _negative: Some(negative),
            _output: output,
        })
    }

    /// Enable the external-feedback amplifier while leaving its fixed package
    /// output available to the ADC driver.
    ///
    /// L012 current-sense routes have a fixed OPA output (PB0/PB1). This
    /// variant consumes only the two input pins; the output pin is intentionally
    /// acquired by `Adc` afterwards, so the safe ownership model can represent
    /// the analog amplifier output and ADC channel at the same time.
    pub fn standalone_ext_adc<P: NonInvertingPin<T>, N: InvertingPin<T>>(
        self,
        positive: Peri<'d, P>,
        negative: Peri<'d, N>,
    ) -> Result<OpAmpAdc<'d, T>, Error> {
        Self::check_output()?;
        let mut positive = Flex::new(positive);
        let mut negative = Flex::new(negative);
        positive.set_as_analog();
        negative.set_as_analog();
        T::regs().cr().write(|w| {
            w.set_bias(vals::Bias::Ua8);
            w.set_mode(vals::Mode::External);
            P::connect(w);
            N::connect(w);
            w.set_en(true);
        });
        Ok(OpAmpAdc {
            _inner: self,
            _positive: positive,
            _negative: negative,
        })
    }
}
impl<T: Instance> Drop for OpAmp<'_, T> {
    fn drop(&mut self) {
        T::regs().cr().modify(|w| w.set_en(false));
    }
}
/// Active analog output retaining controller borrow and all participating pins.
/// Dropping this guard disables only this amplifier before disconnecting its pins.
/// It is deliberately not an ADC input: shared internal routing is deferred.
pub struct OpAmpOutput<'a, 'd, T: Instance> {
    _inner: &'a mut OpAmp<'d, T>,
    _positive: Flex<'a>,
    _negative: Option<Flex<'a>>,
    _output: Flex<'a>,
}

/// Active amplifier whose fixed output is sampled by an ADC peripheral.
pub struct OpAmpAdc<'d, T: Instance> {
    _inner: OpAmp<'d, T>,
    _positive: Flex<'d>,
    _negative: Flex<'d>,
}
impl<T: Instance> OpAmpAdc<'_, T> {
    /// Run the L012 level soft auto-zero sequence and retain calibration mode.
    /// `delay_cycles` is the post-start calibration interval at the current
    /// core clock; the method also includes the required settling interval.
    pub fn calibrate(&mut self, delay_cycles: u32) {
        T::regs().cal().write(|w| {
            w.set_calen(true);
            w.set_calmode(vals::CalibrationMode::Level);
            w.set_start(true);
            w.set_softtrig(false);
        });
        cortex_m::asm::delay(delay_cycles);
        T::regs().cal().modify(|w| {
            w.set_start(false);
            w.set_calmode(vals::CalibrationMode::Triggered);
            w.set_softtrig(false);
        });
        cortex_m::asm::delay(delay_cycles.saturating_mul(10));
    }
    /// Whether the amplifier is currently enabled.
    pub fn is_enabled(&self) -> bool {
        T::regs().cr().read().en()
    }
    /// Whether calibration remains enabled for the calibrated offset path.
    pub fn calibration_enabled(&self) -> bool {
        T::regs().cal().read().calen()
    }
}
impl<T: Instance> Drop for OpAmpAdc<'_, T> {
    fn drop(&mut self) {
        T::regs().cr().modify(|w| w.set_en(false));
    }
}
impl<T: Instance> Drop for OpAmpOutput<'_, '_, T> {
    fn drop(&mut self) {
        T::regs().cr().modify(|w| w.set_en(false));
    }
}
pub(crate) trait SealedInstance {
    fn regs() -> pac::opa::Opa;
    const CHANNEL: u8;
    const SUPPLY_RANGE_MV: (u16, u16);
    const OUTPUT_HEADROOM_MV: u16;
}
/// Source-qualified amplifier instance.
#[allow(private_bounds)]
pub trait Instance: PeripheralType + RccPeripheral + SealedInstance + 'static {}
pub(crate) trait SealedNonInvertingPin<T> {
    fn connect(w: &mut pac::opa::regs::Cr);
}
pub(crate) trait SealedInvertingPin<T> {
    fn connect(w: &mut pac::opa::regs::Cr);
}
pub(crate) trait SealedOutputPin<T> {}
/// Package-qualified positive input switch.
#[allow(private_bounds)]
pub trait NonInvertingPin<T>: Pin + SealedNonInvertingPin<T> {}
/// Package-qualified negative input switch.
#[allow(private_bounds)]
pub trait InvertingPin<T>: Pin + SealedInvertingPin<T> {}
/// Package-qualified analog output.
#[allow(private_bounds)]
pub trait OutputPin<T>: Pin + SealedOutputPin<T> {}
macro_rules! impl_instance {
    ($p:ident,$ch:literal,$range:expr,$headroom:literal) => {
        impl $crate::opamp::SealedInstance for $crate::peripherals::$p {
            fn regs() -> $crate::pac::opa::Opa {
                $crate::pac::$p
            }
            const CHANNEL: u8 = $ch;
            const SUPPLY_RANGE_MV: (u16, u16) = $range;
            const OUTPUT_HEADROOM_MV: u16 = $headroom;
        }
        impl $crate::opamp::Instance for $crate::peripherals::$p {}
    };
}
pub(crate) use impl_instance;
macro_rules! impl_pin {
    ($p:ident,$pin:ident,INP1)=>{$crate::opamp::impl_pin!(@positive $p,$pin,set_inp1en);};
    ($p:ident,$pin:ident,INP2)=>{$crate::opamp::impl_pin!(@positive $p,$pin,set_inp2en);};
    ($p:ident,$pin:ident,INP3)=>{$crate::opamp::impl_pin!(@positive $p,$pin,set_inp3en);};
    ($p:ident,$pin:ident,INN1)=>{$crate::opamp::impl_pin!(@negative $p,$pin,set_inn1en);};
    ($p:ident,$pin:ident,INN2)=>{$crate::opamp::impl_pin!(@negative $p,$pin,set_inn2en);};
    (@positive $p:ident,$pin:ident,$set:ident)=>{
        impl $crate::opamp::SealedNonInvertingPin<$crate::peripherals::$p> for $crate::peripherals::$pin { fn connect(w:&mut $crate::pac::opa::regs::Cr){w.$set(true);} }
        impl $crate::opamp::NonInvertingPin<$crate::peripherals::$p> for $crate::peripherals::$pin {}
    };
    (@negative $p:ident,$pin:ident,$set:ident)=>{
        impl $crate::opamp::SealedInvertingPin<$crate::peripherals::$p> for $crate::peripherals::$pin { fn connect(w:&mut $crate::pac::opa::regs::Cr){w.$set(true);} }
        impl $crate::opamp::InvertingPin<$crate::peripherals::$p> for $crate::peripherals::$pin {}
    };
    ($p:ident,$pin:ident,OUT)=>{
        impl $crate::opamp::SealedOutputPin<$crate::peripherals::$p> for $crate::peripherals::$pin {}
        impl $crate::opamp::OutputPin<$crate::peripherals::$p> for $crate::peripherals::$pin {}
    };
}
pub(crate) use impl_pin;
