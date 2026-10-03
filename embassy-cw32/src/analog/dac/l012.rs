//! Two-channel DAC owner for the l012 DAC IP.

use super::{Error, SignalPin};
use crate::{gpio::AnyPin, pac, peripherals, rcc::PeripheralClock, Peri};
use embedded_hal::delay::DelayNs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    One,
    Two,
}
/// Two internally enabled channels, initially zero. External outputs remain
/// disconnected until explicitly attached with `with_output1/with_output2`.
/// Reference is VDDA. Owning the external PB0/PB1 pin prevents OPA/DAC conflict.
///
/// A driver retains the exclusive borrow of its peripheral.
///
/// Attaching an output retains its borrow even after pin type erasure.
pub struct Dac<'d> {
    _token: Peri<'d, peripherals::DAC>,
    output1: Option<Peri<'d, AnyPin>>,
    output2: Option<Peri<'d, AnyPin>>,
    route: pac::dac::regs::Cr1,
}
impl<'d> Dac<'d> {
    pub fn new(token: Peri<'d, peripherals::DAC>, delay: &mut impl DelayNs) -> Self {
        <peripherals::DAC as PeripheralClock>::enable_and_reset();
        // Exclusive whole-DAC token; triggers/DMA/interrupts/waves off.
        pac::DAC.cr0().write_value(pac::dac::regs::Cr0(0));
        pac::DAC.cr1().write_value(pac::dac::regs::Cr1(0));
        pac::DAC.dhr12r(0).write_value(pac::dac::regs::Dhr12r(0));
        pac::DAC.dhr12r(1).write_value(pac::dac::regs::Dhr12r(0));
        pac::DAC.cr0().write(|w| {
            w.set_en1(true);
            w.set_en2(true);
        });
        delay.delay_us(10); // datasheet tSTART typical 3us, not a characterized max.
        Self {
            _token: token,
            output1: None,
            output2: None,
            route: pac::dac::regs::Cr1(0),
        }
    }
    pub fn with_output1<P: SignalPin<peripherals::DAC, 1>>(mut self, pin: Peri<'d, P>) -> Self {
        let pin: Peri<'d, AnyPin> = pin.into();
        pin.configure_analog();
        self.route.set_c1out(true);
        pac::DAC.cr1().write_value(self.route);
        self.output1 = Some(pin);
        self
    }
    pub fn with_output2<P: SignalPin<peripherals::DAC, 2>>(mut self, pin: Peri<'d, P>) -> Self {
        let pin: Peri<'d, AnyPin> = pin.into();
        pin.configure_analog();
        self.route.set_c2out(true);
        pac::DAC.cr1().write_value(self.route);
        self.output2 = Some(pin);
        self
    }
    /// Write a 12-bit right-aligned code. TEN=0 transfers it to DOR after one
    /// peripheral clock; analog settling takes additional time. No blocking wait.
    pub fn set(&mut self, channel: Channel, code: u16) -> Result<(), Error> {
        let code = dac_code(code)? as u16;
        let n = match channel {
            Channel::One => 0,
            Channel::Two => 1,
        };
        pac::DAC.dhr12r(n).write(|w| w.set_data(code));
        Ok(())
    }
    /// Set both holding registers with one 32-bit write.
    pub fn set_pair(&mut self, one: u16, two: u16) -> Result<(), Error> {
        let one = dac_code(one)? as u16;
        let two = dac_code(two)? as u16;
        pac::DAC.dhr12rd().write(|w| {
            w.set_c1data(one);
            w.set_c2data(two);
        });
        Ok(())
    }
    pub fn output_code(&self, channel: Channel) -> u16 {
        let n = match channel {
            Channel::One => 0,
            Channel::Two => 1,
        };
        pac::DAC.dor(n).read().data()
    }
}
fn dac_code(code: u16) -> Result<u32, Error> {
    if code > 4095 {
        Err(Error::InvalidCode)
    } else {
        Ok(u32::from(code))
    }
}
impl Drop for Dac<'_> {
    fn drop(&mut self) {
        pac::DAC.cr1().write_value(pac::dac::regs::Cr1(0));
        pac::DAC.cr0().write_value(pac::dac::regs::Cr0(0));
        if let Some(pin) = &self.output1 {
            pin.disconnect();
        }
        if let Some(pin) = &self.output2 {
            pin.disconnect();
        }
    }
}
