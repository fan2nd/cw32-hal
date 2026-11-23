//! Calculate a hardware CRC and leave the result available to a debugger.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    crc::{Config, Crc},
};

/// Last checksum, zero-extended when the selected preset is CRC16.
#[unsafe(no_mangle)]
pub static CRC_CHECKSUM: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    let mut crc = Crc::new(p.CRC, Config::default()).unwrap();

    // All selections calculate the checksum of the bytes "CW32 CRC".
    crc.reset();
    crc.feed_byte(b'C');
    crc.feed_bytes(b"W");
    #[cfg(feature = "cw32f002f3p7")]
    crc.feed_bytes(b"32 CRC");
    #[cfg(any(feature = "cw32f020c6u7", feature = "cw32f030c8t7"))]
    {
        // Native halfword/word feeds process their low byte first.
        crc.feed_halfword(u16::from_le_bytes(*b"32"));
        crc.feed_word(u32::from_le_bytes(*b" CRC"));
    }
    CRC_CHECKSUM.store(crc.read(), Ordering::Relaxed);

    // The driver retains the exclusive CRC borrow while the checksum is used.
    loop {
        core::hint::black_box(&crc);
        cortex_m::asm::nop();
    }
}
