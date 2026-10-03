//! Command/FIFO controller: CW32L012 RM1.4 §23.2.1, §23.4, §23.8.
use super::{ceil_div, Config, Cursor, Error, Instance, Step};
use crate::{interrupt::typelevel::Interrupt as _, pac::i2c::regs};
use embedded_hal::i2c::NoAcknowledgeSource;

const TXE: u32 = 1;
const RXNE: u32 = 1 << 1;
const STOP: u32 = 1 << 9;
const ERRORS: u32 = (1 << 10) | (1 << 11) | (1 << 12) | (1 << 13);
#[derive(Clone, Copy)]
pub(super) struct Timing {
    prescale: u8,
    clklo: u8,
    clkhi: u8,
    sethold: u8,
    datavd: u8,
    filter: u8,
    frequency: u32,
}
impl Timing {
    pub fn new(pclk: u32, config: &Config) -> Result<Self, Error> {
        if pclk == 0 {
            return Err(Error::InvalidConfig);
        }
        let fast = config.frequency > 100_000;
        let (low_ns, high_ns, hold_ns, valid_ns) = if fast {
            (1300u64, 600u64, 600u64, 900u64)
        } else {
            (4700u64, 4000u64, 4700u64, 3450u64)
        };
        let clock = u64::from(pclk);
        let rise = u64::from(config.rise_time_ns);
        let fall = u64::from(config.fall_time_ns);
        // A >=50 ns digital filter; its clock is before the prescaler.
        let filter = ceil_div(clock * 50, 1_000_000_000);
        if filter > 15 {
            return Err(Error::InvalidConfig);
        }
        for prescale in 0..=7 {
            let div = 1u64 << prescale;
            let cycles = |ns| ceil_div(clock * ns, div * 1_000_000_000);
            let latency = (2 + filter + ceil_div(clock * rise, 1_000_000_000)) / div;
            let min_latency = (2 + filter) / div;
            // Meet low/high minima without relying on board-dependent
            // synchronizer latency to supply a minimum high period.
            let mut low = cycles(low_ns + fall).max(4);
            let mut high = cycles(high_ns + rise).max(2);
            let hold = cycles(hold_ns + rise + fall).max(3);
            let valid = cycles(300 + fall).max(2);
            let period = ceil_div(clock, div * u64::from(config.frequency));
            // Do not use added synchronizer delay to exceed the requested rate.
            if low + high < period {
                low += period - low - high;
            }
            // Balance oversized low counts into high, preserving both minima.
            if low > 64 {
                high += low - 64;
                low = 64;
            }
            if low > 64 || high > 64 || hold > 64 || valid > 64
                || low < cycles(low_ns + fall) || high < cycles(high_ns + rise)
                || (low - 1) * div <= latency || (hold - 1) * div <= latency
                || low <= latency + 1 || valid > low - latency - 1
                || filter > (low - 1) * div - 3
                // The configured rise time is a maximum. Do not rely on it
                // to satisfy the minimum data setup time (§23.4.3.1).
                || (min_latency + 1) * div * 1_000_000_000 < clock * if fast { 100 } else { 250 }
                || valid * div * 1_000_000_000 + clock * rise > clock * valid_ns
                || clock / div < 8 * u64::from(config.frequency)
            {
                continue;
            }
            return Ok(Self {
                prescale,
                clklo: (low - 1) as u8,
                clkhi: (high - 1) as u8,
                sethold: (hold - 1) as u8,
                datavd: (valid - 1) as u8,
                filter: filter as u8,
                frequency: (clock / (div * (low + high + min_latency))) as u32,
            });
        }
        Err(Error::InvalidConfig)
    }
    pub fn frequency(&self) -> u32 {
        self.frequency
    }
}
pub(super) fn configure<T: Instance>(timing: &Timing) {
    let r = T::regs();
    r.mier().write_value(regs::Mier(0));
    r.mder().write_value(regs::Mder(0));
    r.sier().write_value(regs::Sier(0));
    r.sder().write_value(regs::Sder(0));
    r.scr0().write_value(regs::Scr0(0));
    // RESET resets all master state except MCR0 itself (§23.4.1.1).
    r.mcr0().write_value(regs::Mcr0(1 << 1));
    r.mcr0().write_value(regs::Mcr0(0));
    // Select GPIO bus inputs and PCLK explicitly. No configurable-kernel
    // KernelClock implementation or caller-supplied frequency is assumed.
    r.insel().write_value(regs::Insel(0));
    r.mcr1().write_value(regs::Mcr1(0));
    r.mcr2().write(|w| w.set_prescale(timing.prescale));
    r.mcr3().write(|w| {
        w.set_fltscl(timing.filter);
        w.set_fltsda(timing.filter);
        // Bus-idle detection disabled: don't infer idle from a high phase on
        // somebody else's long/slow transaction.
        w.set_busidle(0);
    });
    r.mcr4().write_value(regs::Mcr4(0));
    r.mccr().write(|w| {
        w.set_clklo(timing.clklo);
        w.set_clkhi(timing.clkhi);
        w.set_sethold(timing.sethold);
        w.set_datavd(timing.datavd);
    });
    r.mfifocr().write_value(regs::Mfifocr(0));
    clear::<T>(0x7f00);
    // CLKSRC=00 PCLK; MEN set only after all timing writes.
    r.mcr0().write_value(regs::Mcr0(1));
}
fn clear<T: Instance>(flags: u32) {
    // R1W0: keep every other implemented flag at one, including documented
    // reserved reset bit 2. Do not RMW this acknowledgement register.
    T::regs().micr().write_value(regs::Micr(0x7f04 & !flags));
}
fn command<T: Instance>(cmd: u8, data: u8) {
    T::regs()
        .mtdr()
        .write_value(regs::Mtdr((u32::from(cmd) << 8) | u32::from(data)));
}
pub(super) fn disarm<T: Instance>() {
    T::regs().mier().write_value(regs::Mier(0));
}
pub(super) fn arm<T: Instance>(mask: u32) {
    T::regs().mier().write_value(regs::Mier(mask | ERRORS));
    // SAFETY: only Async's binding-proven constructor exposes this path.
    unsafe {
        T::Interrupt::enable();
    }
}
pub(super) fn ready<T: Instance>(mask: u32) -> bool {
    T::regs().misr().read().0 & (mask | ERRORS) != 0
}
pub(super) fn interrupt_pending<T: Instance>() -> bool {
    T::regs().misr().read().0 & T::regs().mier().read().0 != 0
}
fn check<T: Instance>() -> Result<regs::Misr, Error> {
    let status = T::regs().misr().read();
    if status.arbi() {
        return Err(Error::Arbitration);
    }
    if status.nack() {
        return Err(Error::NoAcknowledge(NoAcknowledgeSource::Unknown));
    }
    if status.fifo() {
        return Err(Error::Bus);
    }
    if status.pinlow() {
        return Err(Error::Timeout);
    }
    Ok(status)
}
pub(super) fn abort<T: Instance>(limit: u32) {
    disarm::<T>();
    let r = T::regs();
    let status = r.misr().read();
    if !status.arbi() && status.mstbusy() {
        // Flush queued user commands first. MEN=0 alone would continue
        // draining them and is not an immediate cancellation (§23.4.4).
        r.mcr0().write_value(regs::Mcr0(1 | (1 << 8) | (1 << 9)));
        command::<T>(2, 0);
        // With MEN clear, hardware must not stretch on RX-full/TX-empty.
        r.mcr0().write_value(regs::Mcr0(0));
        for _ in 0..limit {
            let status = r.misr().read();
            if status.arbi() || !status.mstbusy() {
                break;
            }
            core::hint::spin_loop();
        }
    }
    // Hard local reset is finite even if the target holds the bus low.
    // After arbitration loss, no STOP command is inserted.
    r.mcr0().write_value(regs::Mcr0(1 << 1));
    r.mcr0().write_value(regs::Mcr0(0));
}

#[derive(Clone, Copy)]
enum Stage {
    Begin,
    Write,
    ReadStart,
    ReadQueue,
    Read,
    End,
    Stop,
    Done,
}
pub(super) struct Engine {
    address: u8,
    stage: Stage,
    chunk_remaining: usize,
    unqueued: usize,
    queued: usize,
}
impl Engine {
    pub fn new(address: u8) -> Self {
        Self {
            address,
            stage: Stage::Begin,
            chunk_remaining: 0,
            unqueued: 0,
            queued: 0,
        }
    }
    pub fn step<T: Instance>(
        &mut self,
        cursor: &mut Cursor<'_, '_>,
        _: u32,
    ) -> Result<Step, Error> {
        let status = check::<T>()?;
        match self.stage {
            Stage::Begin => {
                if !status.txe() {
                    return Ok(Step::Wait(TXE));
                }
                clear::<T>((1 << 8) | STOP);
                command::<T>(4, (self.address << 1) | u8::from(cursor.read));
                self.stage = if cursor.read {
                    Stage::ReadStart
                } else {
                    Stage::Write
                };
            }
            Stage::Write => {
                if !status.txe() {
                    return Ok(Step::Wait(TXE));
                }
                if cursor.remaining == 0 {
                    self.stage = Stage::End;
                } else {
                    command::<T>(0, cursor.take_write());
                }
            }
            Stage::ReadStart => {
                if !status.txe() {
                    return Ok(Step::Wait(TXE));
                }
                self.chunk_remaining = cursor.remaining.min(256);
                self.unqueued = cursor.remaining - self.chunk_remaining;
                command::<T>(1, (self.chunk_remaining - 1) as u8);
                self.stage = Stage::ReadQueue;
            }
            Stage::ReadQueue => {
                if self.unqueued != 0 {
                    if !status.txe() {
                        return Ok(Step::Wait(TXE));
                    }
                    self.queued = self.unqueued.min(256);
                    self.unqueued -= self.queued;
                    command::<T>(1, (self.queued - 1) as u8);
                }
                // Queue the next command BEFORE consuming this chunk's RX
                // data. Every non-final chunk is 256 bytes, while the RX FIFO
                // holds one byte: RX-full stretches SCL and prevents reaching
                // a non-final boundary with an empty command FIFO, irrespective
                // of executor latency. The last command naturally emits NACK.
                self.stage = Stage::Read;
            }
            Stage::Read => {
                if !status.rxne() {
                    return Ok(Step::Wait(RXNE));
                }
                let data = T::regs().mrdr().read();
                if data.empty() {
                    return Err(Error::Overrun);
                }
                cursor.put_read(data.data());
                self.chunk_remaining -= 1;
                if self.chunk_remaining == 0 {
                    if self.queued != 0 {
                        self.chunk_remaining = self.queued;
                        self.queued = 0;
                        self.stage = Stage::ReadQueue;
                    } else {
                        self.stage = Stage::End;
                    }
                }
            }
            Stage::End => {
                if !status.txe() {
                    return Ok(Step::Wait(TXE));
                }
                cursor.next_group();
                if cursor.done() {
                    clear::<T>(STOP);
                    command::<T>(2, 0);
                    self.stage = Stage::Stop;
                } else {
                    self.stage = Stage::Begin;
                }
            }
            Stage::Stop => {
                if !status.stop() {
                    return Ok(Step::Wait(STOP));
                }
                // STOP is the completion fence, not TXE (FIFO empty is not
                // proof that the final byte/address has been ACKed).
                clear::<T>(STOP | (1 << 8));
                self.stage = Stage::Done;
                return Ok(Step::Done);
            }
            Stage::Done => return Ok(Step::Done),
        }
        Ok(Step::Progress)
    }
}
