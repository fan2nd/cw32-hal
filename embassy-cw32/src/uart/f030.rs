//! CW32x030 RM2.5 18.3.3 and 18.9: CR2 clock source, integrated PARITY
//! encoding, no overrun/noise detector, and separately acknowledged RC.

use super::{ClockSource, Config, DataBits, Error, Parity, StopBits, UartRegisters};
use crate::pac::{self, uart::vals};

impl UartRegisters for pac::uart::Uart {
    fn configure(self, config: &Config, divisor: u32) {
        self.ier().write(|_| {});
        self.cr1().write(|w| {
            w.set_over(vals::Cr1Over::OVERSAMPLE16);
            w.set_parity(match (config.data_bits, config.parity) {
                (DataBits::DataBits9, Parity::ParityNone) => vals::Cr1Parity::CUSTOM,
                (_, Parity::ParityNone) => vals::Cr1Parity::NONE,
                (_, Parity::ParityEven) => vals::Cr1Parity::EVEN,
                (_, Parity::ParityOdd) => vals::Cr1Parity::ODD,
            });
            w.set_stop(match config.stop_bits {
                StopBits::STOP1 => vals::Cr1Stop::STOP1,
                StopBits::STOP1P5 => vals::Cr1Stop::STOP1P5,
                StopBits::STOP2 => vals::Cr1Stop::STOP2,
            });
        });
        self.cr2().write(|w| {
            w.set_source(match config.clock_source {
                ClockSource::Pclk => vals::Cr2Source::PCLK,
                ClockSource::PclkAlt => vals::Cr2Source::PCLK_ALT,
            })
        });
        self.brri().write(|w| w.set_brri((divisor / 16) as u16));
        self.brrf().write(|w| w.set_brrf((divisor % 16) as u8));
        self.icr().write(|w| {
            w.set_rc(false);
            w.set_tc(false);
            w.set_fe(false);
            w.set_pe(false);
        });
    }
    fn receive(self) -> Result<Option<u16>, Error> {
        let status = self.isr().read();
        let error = if status.fe() {
            Some(Error::Framing)
        } else if status.pe() {
            Some(Error::Parity)
        } else {
            None
        };
        if !status.rc() && error.is_none() {
            return Ok(None);
        }
        let word = if status.rc() {
            Some(self.rdr().read().rdr())
        } else {
            None
        };
        self.icr().write(|w| {
            if status.rc() {
                w.set_rc(false);
            }
            if status.fe() {
                w.set_fe(false);
            }
            if status.pe() {
                w.set_pe(false);
            }
        });
        match error {
            Some(error) => Err(error),
            None => Ok(word),
        }
    }
    fn rx_pending(self) -> bool {
        let s = self.isr().read();
        s.rc() || s.fe() || s.pe()
    }
    fn rx_interrupt(self, enable: bool) {
        self.ier().modify(|w| {
            w.set_rc(enable);
            w.set_fe(enable);
            w.set_pe(enable);
        });
    }
    fn interrupt_pending(self) -> (bool, bool) {
        let s = self.isr().read();
        let e = self.ier().read();
        (
            (s.txe() && e.txe()) || (s.tc() && e.tc()),
            (s.rc() && e.rc()) || (s.fe() && e.fe()) || (s.pe() && e.pe()),
        )
    }
}
