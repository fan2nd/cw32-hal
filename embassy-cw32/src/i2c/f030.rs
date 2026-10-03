//! Byte/status controller: CW32x030 RM Rev2.5 §20.4.2–3, §20.4.10, §20.7.
use super::{ceil_div, Config, Cursor, Error, Instance, Step};
use crate::{interrupt::typelevel::Interrupt as _, pac::i2c::regs};
use embedded_hal::i2c::NoAcknowledgeSource;

#[derive(Clone, Copy)]
pub(super) struct Timing {
    brr: u8,
    frequency: u32,
}
impl Timing {
    pub fn new(pclk: u32, config: &Config) -> Result<Self, Error> {
        if pclk == 0 {
            return Err(Error::InvalidConfig);
        }
        // The manual proves the total period, not the high/low split. Use
        // an assumed symmetric nominal split to add divider margin; actual
        // tLOW/tHIGH and data timing still require board measurement.
        let low_ns = if config.frequency <= 100_000 {
            4700
        } else {
            1300
        };
        let high_ns = if config.frequency <= 100_000 {
            4000
        } else {
            600
        };
        let half_ns = (low_ns + config.fall_time_ns).max(high_ns + config.rise_time_ns);
        let divider = ceil_div(u64::from(pclk), 8 * u64::from(config.frequency))
            .max(ceil_div(
                u64::from(pclk) * u64::from(half_ns),
                4_000_000_000,
            ))
            .max(2);
        // BRR=0 is forbidden even if the arithmetic would otherwise fit.
        if divider > 256 {
            return Err(Error::InvalidConfig);
        }
        Ok(Self {
            brr: (divider - 1) as u8,
            frequency: pclk / (8 * divider as u32),
        })
    }
    pub fn frequency(&self) -> u32 {
        self.frequency
    }
}
pub(super) fn configure<T: Instance>(timing: &Timing) {
    let r = T::regs();
    r.cr().write_value(regs::Cr(0));
    r.brren().write(|w| w.set_en(false));
    r.brr().write(|w| w.set_brr(timing.brr));
    r.brren().write(|w| w.set_en(true));
    // RM20.4.3 specifically chooses simple filtering when BRR<=9.
    // AA=0 prevents target addressing; slave address registers are unused.
    r.cr()
        .write_value(regs::Cr((1 << 6) | u32::from(timing.brr <= 9)));
}
/// Explicit complete CR word: never RMW the W0C SI command/status register.
fn control<T: Instance>(start: bool, stop: bool, ack: bool) {
    let filter = T::regs().cr().read().flt();
    T::regs().cr().write_value(regs::Cr(
        (1 << 6)
            | (u32::from(start) << 5)
            | (u32::from(stop) << 4)
            | (u32::from(ack) << 2)
            | u32::from(filter),
    ));
}
pub(super) fn disarm<T: Instance>() {
    T::Interrupt::disable();
}
pub(super) fn arm<T: Instance>(_: u32) {
    T::Interrupt::unpend();
    // SAFETY: only Async's binding-proven constructor exposes this path.
    unsafe {
        T::Interrupt::enable();
    }
}
pub(super) fn ready<T: Instance>(_: u32) -> bool {
    T::regs().cr().read().si()
}
pub(super) fn interrupt_pending<T: Instance>() -> bool {
    ready::<T>(0)
}

fn check_status(status: u8) -> Result<(), Error> {
    match status {
        0x00 => Err(Error::Bus),
        0x38 | 0x68 | 0x78 | 0xb0 => Err(Error::Arbitration),
        0x20 | 0x48 => Err(Error::NoAcknowledge(NoAcknowledgeSource::Address)),
        0x30 => Err(Error::NoAcknowledge(NoAcknowledgeSource::Data)),
        _ => Ok(()),
    }
}
fn expected<T: Instance>(expected: u8) -> Result<bool, Error> {
    if !ready::<T>(0) {
        return Ok(false);
    }
    let status = T::regs().stat().read().stat();
    check_status(status)?;
    if status != expected {
        return Err(Error::Bus);
    }
    Ok(true)
}
fn wait_stop<T: Instance>(limit: u32) -> Result<(), Error> {
    // No STOP-complete interrupt exists on this IP (F8 never asserts SI).
    for _ in 0..limit {
        let cr = T::regs().cr().read();
        if cr.si() {
            check_status(T::regs().stat().read().stat())?;
        }
        if !cr.sto() {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(Error::Timeout)
}
pub(super) fn abort<T: Instance>(limit: u32) {
    disarm::<T>();
    let r = T::regs();
    // Wait only a bounded interval for a meaningful state. F8 is not proof
    // that this controller owns the bus, so never issue STOP on F8 alone.
    for _ in 0..limit {
        let cr = r.cr().read();
        if !cr.en() {
            break;
        }
        if cr.si() {
            let status = r.stat().read().stat();
            if matches!(
                status,
                0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x40 | 0x48 | 0x50 | 0x58
            ) {
                control::<T>(false, true, false);
                let _ = wait_stop::<T>(limit);
            }
            // 38/68/78/B0 lost arbitration: release without driving STOP.
            // 00 is the bus-error state: RM says STO would not generate STOP.
            break;
        }
        if cr.sto() {
            let _ = wait_stop::<T>(limit);
            break;
        }
        core::hint::spin_loop();
    }
    r.cr().write_value(regs::Cr(0));
    r.brren().write(|w| w.set_en(false));
    T::Interrupt::unpend();
}

#[derive(Clone, Copy)]
enum Stage {
    Begin,
    Start,
    Address,
    Write,
    Read,
    End,
    Done,
}
pub(super) struct Engine {
    address: u8,
    stage: Stage,
    first: bool,
}
impl Engine {
    pub fn new(address: u8) -> Self {
        Self {
            address,
            stage: Stage::Begin,
            first: true,
        }
    }
    pub fn step<T: Instance>(
        &mut self,
        cursor: &mut Cursor<'_, '_>,
        stop_limit: u32,
    ) -> Result<Step, Error> {
        match self.stage {
            Stage::Begin => {
                control::<T>(true, false, false);
                self.stage = Stage::Start;
            }
            Stage::Start => {
                if !expected::<T>(if self.first { 0x08 } else { 0x10 })? {
                    return Ok(Step::Wait(1));
                }
                T::regs().dr().write_value(regs::Dr(u32::from(
                    (self.address << 1) | u8::from(cursor.read),
                )));
                control::<T>(false, false, false);
                self.stage = Stage::Address;
                self.first = false;
            }
            Stage::Address => {
                if !expected::<T>(if cursor.read { 0x40 } else { 0x18 })? {
                    return Ok(Step::Wait(1));
                }
                self.next_byte::<T>(cursor);
            }
            Stage::Write => {
                if !expected::<T>(0x28)? {
                    return Ok(Step::Wait(1));
                }
                self.next_byte::<T>(cursor);
            }
            Stage::Read => {
                if !expected::<T>(if cursor.remaining == 1 { 0x58 } else { 0x50 })? {
                    return Ok(Step::Wait(1));
                }
                cursor.put_read(T::regs().dr().read().dr());
                self.next_byte::<T>(cursor);
            }
            Stage::End => {
                cursor.next_group();
                if cursor.done() {
                    control::<T>(false, true, false);
                    wait_stop::<T>(stop_limit)?;
                    self.stage = Stage::Done;
                    return Ok(Step::Done);
                }
                // SI remains set until Begin issues repeated START.
                self.stage = Stage::Begin;
            }
            Stage::Done => return Ok(Step::Done),
        }
        Ok(Step::Progress)
    }
    fn next_byte<T: Instance>(&mut self, cursor: &mut Cursor<'_, '_>) {
        if cursor.remaining == 0 {
            self.stage = Stage::End;
        } else if cursor.read {
            // AA is set before SI is cleared, including single-byte reads.
            control::<T>(false, false, cursor.remaining > 1);
            self.stage = Stage::Read;
        } else {
            T::regs()
                .dr()
                .write_value(regs::Dr(u32::from(cursor.take_write())));
            control::<T>(false, false, false);
            self.stage = Stage::Write;
        }
    }
}
