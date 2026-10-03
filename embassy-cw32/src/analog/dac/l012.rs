//! Independently owned channels of the l012 DAC IP.

use core::marker::PhantomData;

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
/// [`Self::split`] transfers both channels into independent owners. Each keeps
/// the peripheral borrow alive and shuts down only its own channel on drop.
pub struct Dac<'d> {
    one: DacChannel<'d, 1>,
    two: DacChannel<'d, 2>,
}
impl<'d> Dac<'d> {
    pub fn new(_token: Peri<'d, peripherals::DAC>, delay: &mut impl DelayNs) -> Self {
        let clock = <peripherals::DAC as PeripheralClock>::acquire();
        // Exclusive whole-DAC token; triggers/DMA/interrupts/waves off.
        pac::DAC.cr0().write_value(pac::dac::regs::Cr0(0));
        pac::DAC.cr1().write_value(pac::dac::regs::Cr1(0));
        pac::DAC.dhr12r(0).write_value(pac::dac::regs::Dhr12r(0));
        pac::DAC.dhr12r(1).write_value(pac::dac::regs::Dhr12r(0));
        pac::DAC.cr0().write(|w| {
            w.set_en(0, true);
            w.set_en(1, true);
        });
        // Datasheet tSTART is typically 3us, not a characterized maximum.
        delay.delay_us(10);
        // Consuming the only DAC token grants two disjoint channel capabilities.
        // Both retain its lifetime; neither exposes the whole peripheral token.
        Self {
            one: DacChannel {
                _clock: clock.retain(),
                output: None,
                _borrow: PhantomData,
            },
            two: DacChannel {
                _clock: clock,
                output: None,
                _borrow: PhantomData,
            },
        }
    }
    pub fn with_output1<P: SignalPin<peripherals::DAC, 1>>(mut self, pin: Peri<'d, P>) -> Self {
        self.one = self.one.with_output(pin);
        self
    }
    pub fn with_output2<P: SignalPin<peripherals::DAC, 2>>(mut self, pin: Peri<'d, P>) -> Self {
        self.two = self.two.with_output(pin);
        self
    }
    /// Transfer both channels and any attached pins without resetting hardware.
    /// Dropping either channel leaves the other channel and shared gate intact.
    pub fn split(self) -> (DacChannel<'d, 1>, DacChannel<'d, 2>) {
        (self.one, self.two)
    }
    /// Write a 12-bit right-aligned code. TEN=0 transfers it to DOR after one
    /// peripheral clock; analog settling takes additional time. No blocking wait.
    pub fn set(&mut self, channel: Channel, code: u16) -> Result<(), Error> {
        match channel {
            Channel::One => self.one.set(code),
            Channel::Two => self.two.set(code),
        }
    }
    /// Set both holding registers with one 32-bit write. Available only while
    /// both channels remain exclusively owned together.
    pub fn set_pair(&mut self, one: u16, two: u16) -> Result<(), Error> {
        let one = dac_code(one)?;
        let two = dac_code(two)?;
        pac::DAC.dhr12rd().write(|w| {
            w.set_data(0, one);
            w.set_data(1, two);
        });
        Ok(())
    }
    pub fn output_code(&self, channel: Channel) -> u16 {
        match channel {
            Channel::One => self.one.output_code(),
            Channel::Two => self.two.output_code(),
        }
    }
}

/// One enabled DAC channel, owning only its own optional output pin.
/// `C` is 1 or 2; only [`Dac::split`] can create these capabilities.
/// Shared configuration is changed with critical-section RMWs, and dropping a
/// channel never resets or gates its sibling.
pub struct DacChannel<'d, const C: u8> {
    _clock: crate::rcc::ClockGuard,
    output: Option<Peri<'d, AnyPin>>,
    _borrow: PhantomData<&'d mut peripherals::DAC>,
}
impl<'d, const C: u8> DacChannel<'d, C> {
    /// Attach this channel's audited external output pin.
    pub fn with_output<P: SignalPin<peripherals::DAC, C>>(mut self, pin: Peri<'d, P>) -> Self {
        let pin: Peri<'d, AnyPin> = pin.into();
        pin.configure_analog();
        critical_section::with(|_| {
            pac::DAC.cr1().modify(|w| match C {
                1 => w.set_out(0, true),
                2 => w.set_out(1, true),
                _ => unreachable!(),
            });
        });
        if let Some(previous) = self.output.replace(pin) {
            previous.disconnect();
        }
        self
    }
    /// Write this channel's holding register without waiting for analog settling.
    pub fn set(&mut self, code: u16) -> Result<(), Error> {
        write_channel::<C>(code)
    }
    pub fn output_code(&self) -> u16 {
        pac::DAC.dor(usize::from(C - 1)).read().data()
    }
    /// Reserve this enabled channel as an internal OPA/comparator source.
    /// The guard permits code updates while consumers borrow it, but keeps this
    /// owner and its pin unavailable for reconfiguration or drop.
    pub fn source(&mut self) -> DacSource<'_, C> {
        DacSource {
            _borrow: PhantomData,
        }
    }
}
impl<const C: u8> Drop for DacChannel<'_, C> {
    fn drop(&mut self) {
        critical_section::with(|_| {
            // Disconnect before disabling; retain every sibling/reserved field.
            pac::DAC.cr1().modify(|w| match C {
                1 => w.set_out(0, false),
                2 => w.set_out(1, false),
                _ => unreachable!(),
            });
            pac::DAC.cr0().modify(|w| match C {
                1 => w.set_en(0, false),
                2 => w.set_en(1, false),
                _ => unreachable!(),
            });
        });
        if let Some(pin) = &self.output {
            pin.disconnect();
        }
    }
}

/// A lifetime-bound, updatable analog source from one enabled DAC channel.
///
/// OPA and VC dependencies borrow this guard. Updating its code is deliberately
/// allowed during that borrow and changes every connected consumer's signal.
/// The caller must allow DAC and downstream analog settling before interpreting
/// results. This guard cannot disable, reroute, or release the channel or its pin.
pub struct DacSource<'a, const C: u8> {
    _borrow: PhantomData<&'a mut peripherals::DAC>,
}
impl<const C: u8> DacSource<'_, C> {
    /// Update this source without ending its consumers' dependency borrows.
    /// The write is single-channel; it does not alter enable/route registers.
    pub fn set(&self, code: u16) -> Result<(), Error> {
        write_channel::<C>(code)
    }
    pub fn output_code(&self) -> u16 {
        pac::DAC.dor(usize::from(C - 1)).read().data()
    }
}
// Erase only the channel number in a consumer's stored shared reference. The
// actual DacSource borrow, including its exclusive channel lifetime, is retained.
#[cfg(all(bgr_l012, any(opa_l012, vc_l012)))]
pub(crate) trait DacDependency: Sync {}
#[cfg(all(bgr_l012, any(opa_l012, vc_l012)))]
impl<const C: u8> DacDependency for DacSource<'_, C> {}

fn write_channel<const C: u8>(code: u16) -> Result<(), Error> {
    let code = dac_code(code)?;
    pac::DAC
        .dhr12r(usize::from(C - 1))
        .write(|w| w.set_data(code));
    Ok(())
}
fn dac_code(code: u16) -> Result<u16, Error> {
    if code > 4095 {
        Err(Error::InvalidCode)
    } else {
        Ok(code)
    }
}
