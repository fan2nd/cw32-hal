//! Stateful hardware CRC using the algorithms provided by the selected IP.
//!
//! Writing the algorithm selector initializes a new calculation. `feed_*`
//! continues the existing calculation; reading does not reset it. There is no
//! configurable polynomial or initial value on these CW32 implementations.

use crate::{peripherals::CRC, rcc::PeripheralClock, Peri};

#[cfg(crc_l012)]
#[path = "l012.rs"]
mod hardware;
#[cfg(crc_f030)]
#[path = "f030.rs"]
mod hardware;

/// Complete hardware algorithm, including initialization and reflection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Algorithm {
    /// CRC-16/ARC (IBM): polynomial 0x8005, initial value 0, reflected.
    Ibm,
    /// CRC-16/MAXIM-DOW: IBM with final XOR 0xffff.
    Maxim,
    /// CRC-16/USB: polynomial 0x8005, initial/final XOR 0xffff, reflected.
    Usb,
    /// CRC-16/MODBUS: polynomial 0x8005, initial 0xffff, reflected.
    Modbus,
    /// Vendor CRC16_CCITT, also known as CRC-16/KERMIT: initial 0, reflected.
    #[default]
    Ccitt,
    /// CRC-16/IBM-3740 (CCITT-FALSE): initial 0xffff, unreflected.
    CcittFalse,
    /// CRC-16/IBM-SDLC (X-25): initial/final XOR 0xffff, reflected.
    X25,
    /// CRC-16/XMODEM: initial 0, unreflected.
    Xmodem,
    /// CRC-32/ISO-HDLC: initial/final XOR 0xffffffff, reflected.
    #[cfg(crc_has_crc32)]
    Crc32,
    /// CRC-32/MPEG-2: initial 0xffffffff, unreflected, final XOR 0.
    #[cfg(crc_has_crc32)]
    Mpeg2,
}

impl Algorithm {
    fn register_value(self) -> crate::pac::crc::vals::CrMode {
        use crate::pac::crc::vals::CrMode;
        match self {
            Self::Ibm => CrMode::IBM,
            Self::Maxim => CrMode::MAXIM,
            Self::Usb => CrMode::USB,
            Self::Modbus => CrMode::MODBUS,
            Self::Ccitt => CrMode::CCITT,
            Self::CcittFalse => CrMode::CCITT_FALSE,
            Self::X25 => CrMode::X25,
            Self::Xmodem => CrMode::XMODEM,
            #[cfg(crc_has_crc32)]
            Self::Crc32 => CrMode::CRC32,
            #[cfg(crc_has_crc32)]
            Self::Mpeg2 => CrMode::CRC32_MPEG2,
        }
    }
}

/// Exclusive owner of the CRC engine and its running calculation.
pub struct Crc<'d> {
    _peripheral: Peri<'d, CRC>,
    _clock: crate::rcc::ClockGuard,
    algorithm: Algorithm,
}

impl<'d> Crc<'d> {
    /// Acquire the engine and initialize the selected algorithm.
    pub fn new(peripheral: Peri<'d, CRC>, algorithm: Algorithm) -> Self {
        let clock = CRC::acquire();
        let mut crc = Self {
            _peripheral: peripheral,
            _clock: clock,
            algorithm,
        };
        crc.reset();
        crc
    }

    /// Reset the running calculation to this algorithm's initial value.
    pub fn reset(&mut self) {
        // CR is a command register: writing MODE initializes the accumulator,
        // even when the selected algorithm is unchanged.
        crate::pac::CRC
            .cr()
            .write(|w| w.set_mode(self.algorithm.register_value()));
    }

    /// Select another algorithm and initialize a new calculation.
    pub fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
        self.reset();
    }

    /// The currently selected algorithm.
    pub fn algorithm(&self) -> Algorithm {
        self.algorithm
    }

    /// Read the current checksum without resetting or feeding data.
    /// A 16-bit algorithm returns its zero-extended result.
    pub fn read(&self) -> u32 {
        hardware::read(self.algorithm)
    }

    /// Feed one byte into the existing calculation.
    pub fn feed_byte(&mut self, byte: u8) {
        hardware::feed_byte(byte);
    }

    /// Feed bytes in slice order into the existing calculation.
    pub fn feed_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.feed_byte(byte);
        }
    }

    /// Feed a halfword, low byte first, using a 16-bit bus write.
    #[cfg(crc_has_wide_data)]
    pub fn feed_halfword(&mut self, halfword: u16) {
        hardware::feed_halfword(halfword);
    }

    /// Feed halfwords in slice order, low byte first within each halfword.
    #[cfg(crc_has_wide_data)]
    pub fn feed_halfwords(&mut self, halfwords: &[u16]) {
        for &halfword in halfwords {
            self.feed_halfword(halfword);
        }
    }

    /// Feed a word, low byte first, using a 32-bit bus write.
    #[cfg(crc_has_wide_data)]
    pub fn feed_word(&mut self, word: u32) {
        hardware::feed_word(word);
    }

    /// Feed words in slice order, low byte first within each word.
    #[cfg(crc_has_wide_data)]
    pub fn feed_words(&mut self, words: &[u32]) {
        for &word in words {
            self.feed_word(word);
        }
    }
}
