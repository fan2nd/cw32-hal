//! Single ADC producer. A delayed transmitter gets the latest complete block.
use crate::protocol::crc;
use core::{
    cell::RefCell,
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};
use cortex_m::interrupt::{Mutex, free};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

pub static GENERATED: AtomicU32 = AtomicU32::new(0);
pub static OVERWRITTEN: AtomicU32 = AtomicU32::new(0);
pub static SENT: AtomicU32 = AtomicU32::new(0);

static FORMAT: AtomicU8 = AtomicU8::new(0);
pub static READY: Channel<CriticalSectionRawMutex, Batch, 4> = Channel::new();
#[derive(Clone, Copy)]
pub struct Batch {
    first: u32,
    valid: u16,
    pwm: bool,
    period: u16,
    samples: [Sample; 16],
}
#[derive(Clone, Copy, Default)]
pub struct Sample {
    pub currents: [i16; 4],
    pub ccr: [u16; 3],
}
pub const PWM_FRAME: usize = 236;
impl Batch {
    const EMPTY: Self = Self {
        first: 0,
        valid: 0,
        pwm: false,
        period: 0,
        samples: [Sample {
            currents: [0; 4],
            ccr: [0; 3],
        }; 16],
    };
    pub fn encode<'a>(&self, out: &'a mut [u8; PWM_FRAME]) -> &'a [u8] {
        out[..2].copy_from_slice(if self.pwm { b"SP" } else { b"SW" });
        out[2..6].copy_from_slice(&self.first.to_le_bytes());
        out[6..8].copy_from_slice(&self.valid.to_le_bytes());
        let mut pos = 8;
        if self.pwm {
            out[8..10].copy_from_slice(&self.period.to_le_bytes());
            pos = 10;
        }
        for sample in &self.samples {
            for current in sample.currents {
                out[pos..pos + 2].copy_from_slice(&current.to_le_bytes());
                pos += 2;
            }
            if self.pwm {
                for compare in sample.ccr {
                    out[pos..pos + 2].copy_from_slice(&compare.to_le_bytes());
                    pos += 2;
                }
            }
        }
        let check = crc(&out[..pos]);
        out[pos..pos + 2].copy_from_slice(&check.to_le_bytes());
        &out[..pos + 2]
    }
}
struct Capture {
    block: Batch,
    used: usize,
}
static CAPTURE: Mutex<RefCell<Capture>> = Mutex::new(RefCell::new(Capture {
    block: Batch::EMPTY,
    used: 0,
}));
pub fn enabled() -> bool {
    FORMAT.load(Ordering::Relaxed) != 0
}
pub fn subscribe(format: u8) {
    free(|cs| {
        if FORMAT.load(Ordering::Relaxed) != format {
            FORMAT.store(format, Ordering::Relaxed);
            let mut c = CAPTURE.borrow(cs).borrow_mut();
            c.used = 0;
            c.block.pwm = format == 7;
            c.block.first = 0;
            c.block.valid = 0;
            // A completed block may still be waiting for TX when sampling stops.
            if format != 0 {
                while READY.try_receive().is_ok() {
                    OVERWRITTEN.store(
                        OVERWRITTEN.load(Ordering::Relaxed).wrapping_add(1),
                        Ordering::Relaxed,
                    );
                }
            }
        }
    });
}
pub fn push(value: Option<(Sample, u16)>) {
    free(|cs| {
        let mut c = CAPTURE.borrow(cs).borrow_mut();
        let i = c.used;
        if let Some((v, period)) = value {
            c.block.period = period;
            c.block.samples[i] = v;
            c.block.valid |= 1 << i;
        } else {
            c.block.samples[i] = Sample::default();
        }
        c.used += 1;
        if c.used == 16 {
            GENERATED.store(
                GENERATED.load(Ordering::Relaxed).wrapping_add(1),
                Ordering::Relaxed,
            );
            if READY.try_send(c.block).is_err() {
                OVERWRITTEN.store(
                    OVERWRITTEN.load(Ordering::Relaxed).wrapping_add(1),
                    Ordering::Relaxed,
                );
            }

            c.block.first = c.block.first.wrapping_add(16);
            c.block.valid = 0;
            c.used = 0;
        }
    });
}
