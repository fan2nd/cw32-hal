use anyhow::{bail, ensure, Context, Result};
use hidapi::HidDevice;
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub const VID: u16 = 0x04d8;
pub const PID: u16 = 0x00dd;

// A report transport permits protocol tests without attaching or programming hardware.
pub trait Transport {
    fn exchange(&self, request: [u8; 64]) -> Result<[u8; 64]>;
}

impl Transport for HidDevice {
    fn exchange(&self, request: [u8; 64]) -> Result<[u8; 64]> {
        // hidapi requires a leading report ID even for an unnumbered output report.
        let mut output = [0; 65];
        output[1..].copy_from_slice(&request);
        ensure!(
            self.write(&output)? == 65,
            "short HID write; command outcome unknown"
        );
        let mut response = [0; 64];
        let len = self.read_timeout(&mut response, 1000)?;
        ensure!(
            len == 64,
            "HID timeout/short report ({len}/64); command outcome unknown"
        );
        Ok(response)
    }
}

pub struct Bridge<T> {
    pub transport: T,
    pub address: u8,
}

impl<T: Transport> Bridge<T> {
    fn command(&self, bytes: &[u8]) -> Result<[u8; 64]> {
        ensure!(
            !bytes.is_empty() && bytes.len() <= 64,
            "invalid HID command size"
        );
        let mut request = [0; 64];
        request[..bytes.len()].copy_from_slice(bytes);
        let r = self.transport.exchange(request)?;
        ensure!(
            r[0] == bytes[0],
            "unexpected HID echo 0x{:02X}, expected 0x{:02X}",
            r[0],
            bytes[0]
        );
        ensure!(
            r[1] == 0,
            "MCP command 0x{:02X} rejected: status=0x{:02X}, state=0x{:02X}",
            bytes[0],
            r[1],
            r[2]
        );
        Ok(r)
    }

    fn wait_complete(&self, reading: bool) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let r = self.command(&[0x10])?;
            // Firmware engine states/NAK flag follow the Linux MCP2221 driver.
            ensure!(
                r[20] & 0x40 == 0,
                "I2C address NACK; check address, MODE, power and wiring"
            );
            match r[8] {
                0 => return Ok(()),
                0x55 if reading => return Ok(()),
                0x25 => bail!("I2C address NACK"),
                0x12 | 0x23 | 0x44 | 0x62 => bail!("I2C engine timeout: 0x{:02X}", r[8]),
                _ => ensure!(
                    Instant::now() < deadline,
                    "I2C completion timeout, state=0x{:02X}; use recover after checking wiring",
                    r[8]
                ),
            }
            sleep(Duration::from_millis(2));
        }
    }

    fn prepare(&self) -> Result<()> {
        // 12 MHz / (divider + 2) = 100 kHz; never silently cancel an active transaction.
        let r = self.command(&[0x10, 0, 0, 0x20, 118])?;
        ensure!(
            r[3] == 0x20,
            "I2C speed change rejected (busy); use recover if needed"
        );
        Ok(())
    }

    pub fn recover(&self) -> Result<()> {
        self.command(&[0x10, 0, 0x10])?;
        self.wait_complete(false)
    }

    pub fn read_register(&self, register: u8) -> Result<u8> {
        self.prepare()?;
        self.command(&[0x94, 1, 0, self.address << 1, register])?;
        // Give the no-STOP register-pointer write time to finish before repeated START.
        sleep(Duration::from_millis(2));
        self.command(&[0x93, 1, 0, (self.address << 1) | 1])?;
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let r = self.command(&[0x40])?;
            ensure!(
                !matches!(r[2], 0x25 | 0x12 | 0x23 | 0x44 | 0x62),
                "I2C read failed, state=0x{:02X}",
                r[2]
            );
            match r[3] {
                1 => {
                    self.wait_complete(true)?;
                    return Ok(r[4]);
                }
                0 => ensure!(
                    Instant::now() < deadline,
                    "I2C read timed out at register 0x{register:02X}"
                ),
                n => bail!("invalid I2C read length {n}, register 0x{register:02X}"),
            }
            sleep(Duration::from_millis(2));
        }
    }

    pub fn write_register(&self, register: u8, value: u8) -> Result<()> {
        self.prepare()?;
        self.command(&[0x90, 2, 0, self.address << 1, register, value])?;
        self.wait_complete(false).with_context(|| {
            format!("writing register 0x{register:02X}; value may already have changed")
        })
    }

    pub fn program_eeprom(&self) -> Result<()> {
        self.write_register(0x09, 0xb3)?;
        let result = self.write_register(0x0a, 0x05);
        // Even if the final HID acknowledgement is lost, programming may have begun.
        // Always leave the sensor undisturbed for >600 ms before returning the outcome.
        sleep(Duration::from_millis(700));
        result.context(
            "EEPROM command outcome uncertain; do not retry blindly, power-cycle and verify",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::VecDeque};
    struct Fake(RefCell<VecDeque<(Vec<u8>, [u8; 64])>>);
    impl Transport for Fake {
        fn exchange(&self, request: [u8; 64]) -> Result<[u8; 64]> {
            let (expected, reply) = self.0.borrow_mut().pop_front().expect("unexpected command");
            assert_eq!(&request[..expected.len()], expected);
            assert!(request[expected.len()..].iter().all(|b| *b == 0));
            Ok(reply)
        }
    }
    fn response(command: u8, fields: &[(usize, u8)]) -> [u8; 64] {
        let mut r = [0; 64];
        r[0] = command;
        for (i, v) in fields {
            r[*i] = *v;
        }
        r
    }
    #[test]
    fn register_read_uses_repeated_start_and_read_address_bit() {
        let bridge = Bridge {
            address: 6,
            transport: Fake(RefCell::new(VecDeque::from([
                (vec![0x10, 0, 0, 0x20, 118], response(0x10, &[(3, 0x20)])),
                (vec![0x94, 1, 0, 0x0c, 0x31], response(0x94, &[])),
                (vec![0x93, 1, 0, 0x0d], response(0x93, &[])),
                (vec![0x40], response(0x40, &[])),
                (vec![0x40], response(0x40, &[(2, 0x55), (3, 1), (4, 0xff)])),
                (vec![0x10], response(0x10, &[(8, 0x55)])),
            ]))),
        };
        assert_eq!(bridge.read_register(0x31).unwrap(), 255);
        assert!(bridge.transport.0.borrow().is_empty());
    }
    #[test]
    fn nack_and_bad_echo_fail_without_resending_writes() {
        for reply in [
            response(0x10, &[(20, 0x40)]),
            response(0x10, &[(8, 0x25)]),
            response(0x11, &[]),
        ] {
            let bridge = Bridge {
                address: 6,
                transport: Fake(RefCell::new(VecDeque::from([
                    (vec![0x10, 0, 0, 0x20, 118], response(0x10, &[(3, 0x20)])),
                    (vec![0x90, 2, 0, 0x0c, 0x31, 0xff], response(0x90, &[])),
                    (vec![0x10], reply),
                ]))),
            };
            assert!(bridge.write_register(0x31, 255).is_err());
            assert!(bridge.transport.0.borrow().is_empty());
        }
    }
    #[test]
    fn invalid_read_payload_is_never_returned_as_register_data() {
        for reply in [
            response(0x40, &[(3, 127)]),
            response(0x40, &[(3, 2)]),
            response(0x40, &[(1, 0x41)]),
            response(0x40, &[(2, 0x25), (3, 1)]),
        ] {
            let bridge = Bridge {
                address: 0x46,
                transport: Fake(RefCell::new(VecDeque::from([
                    (vec![0x10, 0, 0, 0x20, 118], response(0x10, &[(3, 0x20)])),
                    (vec![0x94, 1, 0, 0x8c, 0x31], response(0x94, &[])),
                    (vec![0x93, 1, 0, 0x8d], response(0x93, &[])),
                    (vec![0x40], reply),
                ]))),
            };
            assert!(bridge.read_register(0x31).is_err());
            assert!(bridge.transport.0.borrow().is_empty());
        }
    }

    #[test]
    fn rejected_eeprom_reply_still_observes_quiet_time() {
        let bridge = Bridge {
            address: 6,
            transport: Fake(RefCell::new(VecDeque::from([
                (vec![0x10, 0, 0, 0x20, 118], response(0x10, &[(3, 0x20)])),
                (vec![0x90, 2, 0, 0x0c, 9, 0xb3], response(0x90, &[])),
                (vec![0x10], response(0x10, &[])),
                (vec![0x10, 0, 0, 0x20, 118], response(0x10, &[(3, 0x20)])),
                (vec![0x90, 2, 0, 0x0c, 10, 5], response(0x90, &[(1, 1)])),
            ]))),
        };
        let start = Instant::now();
        assert!(bridge.program_eeprom().is_err());
        assert!(start.elapsed() >= Duration::from_millis(700));
        assert!(bridge.transport.0.borrow().is_empty());
    }

    #[test]
    fn eeprom_sequence_and_quiet_time() {
        let mut steps = VecDeque::new();
        for (reg, value) in [(9, 0xb3), (10, 5)] {
            steps.push_back((vec![0x10, 0, 0, 0x20, 118], response(0x10, &[(3, 0x20)])));
            steps.push_back((vec![0x90, 2, 0, 0x0c, reg, value], response(0x90, &[])));
            steps.push_back((vec![0x10], response(0x10, &[])));
        }
        let bridge = Bridge {
            address: 6,
            transport: Fake(RefCell::new(steps)),
        };
        let start = Instant::now();
        bridge.program_eeprom().unwrap();
        assert!(start.elapsed() >= Duration::from_millis(700));
        assert!(bridge.transport.0.borrow().is_empty());
    }
}
