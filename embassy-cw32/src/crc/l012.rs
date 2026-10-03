//! RM1.4 10.3/10.6: eight 16-bit algorithms; DR consumes one byte per word write.
use super::Algorithm;

pub(super) fn feed_byte(byte: u8) {
    crate::pac::CRC.dr().write(|w| w.set_dr(byte));
}

pub(super) fn read(_: Algorithm) -> u32 {
    u32::from(crate::pac::CRC.result().read().result())
}
