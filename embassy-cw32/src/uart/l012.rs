//! CW32L012 RM1.4 21.3.3 and 21.9: CR1 clock source, CHLEN/PARITYEN,
//! independent RC/PE/FE/NE/ORE flags, and R1W0 acknowledgement.

use super::{ClockSource, Config, DataBits, Error, Parity, StopBits, UartRegisters};
use crate::pac::{self, uart::vals};

impl UartRegisters for pac::uart::Uart {
    fn configure(self, config: &Config, divisor: u32) {
        self.ier().write(|_| {});
        self.cr1().write(|w| {
            w.set_source(match config.clock_source {
                ClockSource::Pclk => vals::Cr1Source::PCLK,
                ClockSource::PclkAlt => vals::Cr1Source::PCLK_ALT,
            });
            w.set_over(vals::Cr1Over::OVERSAMPLE16);
            w.set_chlen(
                config.data_bits == DataBits::DataBits9 || config.parity != Parity::ParityNone,
            );
            w.set_parityen(config.parity != Parity::ParityNone);
            w.set_parity(match config.parity {
                Parity::ParityNone | Parity::ParityEven => vals::Cr1Parity::EVEN,
                Parity::ParityOdd => vals::Cr1Parity::ODD,
            });
            w.set_stop(match config.stop_bits {
                StopBits::STOP1 => vals::Cr1Stop::STOP1,
                StopBits::STOP1P5 => vals::Cr1Stop::STOP1P5,
                StopBits::STOP2 => vals::Cr1Stop::STOP2,
            });
        });
        self.cr2().write(|_| {});
        self.cr3().write(|_| {});
        self.brri().write(|w| w.set_brri((divisor / 16) as u16));
        self.brrf().write(|w| w.set_brrf((divisor % 16) as u8));
        self.icr().write(|w| {
            w.set_rc(false);
            w.set_tc(false);
            w.set_fe(false);
            w.set_pe(false);
            w.set_ne(false);
            w.set_ore(false);
        });
    }
    fn receive(self) -> Result<Option<u16>, Error> {
        let status = self.isr().read();
        let error = if status.ore() {
            Some(Error::Overrun)
        } else if status.ne() {
            Some(Error::Noise)
        } else if status.fe() {
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
        // RDR does not acknowledge RC. Clear only flags observed in this
        // snapshot, preserving unselected R1W0 flags and reserved reset bits.
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
            if status.ne() {
                w.set_ne(false);
            }
            if status.ore() {
                w.set_ore(false);
            }
        });
        match error {
            Some(error) => Err(error),
            None => Ok(word),
        }
    }
    fn rx_pending(self) -> bool {
        let s = self.isr().read();
        s.rc() || s.fe() || s.pe() || s.ne() || s.ore()
    }
    fn rx_interrupt(self, enable: bool) {
        self.ier().modify(|w| {
            w.set_rc(enable);
            w.set_fe(enable);
            w.set_pe(enable);
            w.set_ne(enable);
            w.set_ore(enable);
        });
    }
    fn interrupt_pending(self) -> (bool, bool) {
        let s = self.isr().read();
        let e = self.ier().read();
        (
            (s.txe() && e.txe()) || (s.tc() && e.tc()),
            (s.rc() && e.rc())
                || (s.fe() && e.fe())
                || (s.pe() && e.pe())
                || (s.ne() && e.ne())
                || (s.ore() && e.ore()),
        )
    }
}
