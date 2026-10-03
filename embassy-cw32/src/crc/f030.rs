//! RM2.5 10.3/10.6: bus access width selects how many input bytes are fed.
use super::Algorithm;

pub(super) fn feed_byte(byte: u8) {
    crate::pac::CRC.dr8().write(|w| w.set_dr8(byte));
}

pub(super) fn feed_halfword(halfword: u16) {
    crate::pac::CRC.dr16().write(|w| w.set_dr16(halfword));
}

pub(super) fn feed_word(word: u32) {
    crate::pac::CRC.dr32().write(|w| w.set_dr32(word));
}

pub(super) fn read(algorithm: Algorithm) -> u32 {
    match algorithm {
        Algorithm::Crc32 | Algorithm::Mpeg2 => crate::pac::CRC.result32().read().result32(),
        _ => u32::from(crate::pac::CRC.result16().read().result16()),
    }
}
