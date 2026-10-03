//! Register lifecycle shared only after verifying the common CW DMA subset.
use super::{Error, RawConfig, TransferMode, Trigger};

pub(super) trait Io {
    fn csr(&mut self) -> u32;
    fn status(&mut self) -> u32;
    fn control(&mut self, value: u32);
    fn count(&mut self, value: u32);
    fn source(&mut self, value: u32);
    fn destination(&mut self, value: u32);
    fn trigger(&mut self, value: u32);
    /// Clear just this channel's TC/TE, preserving peer and reserved ICR bits.
    fn clear(&mut self);
    /// Compiler and CPU ordering only. Does NOT assert DMA-bus quiescence.
    fn fence(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Config {
    source: u32,
    destination: u32,
    count: u16,
    control: u32,
    trigger: u32,
}

pub(super) fn validate(
    source: usize,
    destination: usize,
    count: usize,
    size: u8,
    raw: RawConfig,
) -> Result<Config, Error> {
    if count == 0 {
        return Err(Error::Empty);
    }
    if count > u16::MAX as usize {
        return Err(Error::TooLong);
    }
    let bytes = 1usize << size;
    if source % bytes != 0 || destination % bytes != 0 {
        return Err(Error::Unaligned);
    }
    for (address, increment) in [
        (source, raw.source_increment),
        (destination, raw.destination_increment),
    ] {
        let span = if increment { count } else { 1 } * bytes;
        if address
            .checked_add(span - 1)
            .is_none_or(|end| end > u32::MAX as usize)
        {
            return Err(Error::AddressOverflow);
        }
    }
    let control = ((size as u32) << 6)
        | ((raw.destination_increment as u32) << 5)
        | ((raw.source_increment as u32) << 4)
        | ((matches!(raw.mode, TransferMode::Block) as u32) << 3);
    let trigger = match raw.trigger {
        Trigger::Software => 0,
        Trigger::Hardware(request) => 1 | ((request as u32) << 2),
    };
    Ok(Config {
        source: source as u32,
        destination: destination as u32,
        count: count as u16,
        control,
        trigger,
    })
}

pub(super) fn start(io: &mut impl Io, config: Config, interrupts: bool, repeating: bool) {
    // Only called for an idle or fully completed channel. A poisoned channel
    // cannot use this sequence to pretend an earlier abort was drained.
    io.control(0);
    io.trigger(0);
    io.clear();
    io.source(config.source);
    io.destination(config.destination);
    // REPEAT=1, not zero. F030 destructively decrements both count fields;
    // always program a complete new word. L012 keeps separate current fields.
    io.count(u32::from(config.count) | (1 << 16));
    io.trigger(config.trigger); // SOFTSRC deliberately zero, never a TRIG RMW.
    let mut control = config.control;
    if interrupts {
        control |= (1 << 1) | (1 << 2);
    }
    #[cfg(dmachannel_l012)]
    if repeating {
        control |= 1 << 11;
    } // no per-block SRCLOAD/DSTLOAD
    #[cfg(not(dmachannel_l012))]
    debug_assert!(!repeating);
    io.fence(); // publish source data before either trigger can launch DMA
    io.control(control | 1);
    if config.trigger & 1 == 0 {
        io.trigger(1 << 1);
    }
}

/// This is only a disable request. Callers must quarantine on non-TC paths.
pub(super) fn disable(io: &mut impl Io) {
    // Construct a writable control word; never replay STATUS or L012 TC/TE.
    io.control(0);
    io.trigger(0);
    io.fence();
    io.clear();
}

pub(super) fn complete(io: &mut impl Io, from_irq: bool) -> Option<Result<(), Error>> {
    let pending = io.status() & 3; // common ISR bits normalized for this channel
    if pending == 0 {
        return None;
    }
    let csr = io.csr();
    if from_irq && (pending & ((csr >> 1) & 3)) == 0 {
        return None;
    }
    // Error wins when flags coincide. Never return a possibly corrupt buffer
    // based on TC while any error indication is asserted.
    let result = if pending & 2 != 0 {
        Err(decode_error(((csr >> 8) & 7) as u8))
    } else {
        Ok(())
    };
    disable(io);
    // A clean TC means all requested data transferred correctly (both RMs 8.6).
    // The barrier orders later CPU buffer access after that hardware proof.
    io.fence();
    Some(result)
}

#[cfg(dmachannel_l012)]
pub(super) fn error(io: &mut impl Io) -> Option<Error> {
    if io.status() & 2 != 0 {
        Some(decode_error(((io.csr() >> 8) & 7) as u8))
    } else {
        None
    }
}

fn decode_error(status: u8) -> Error {
    match status {
        1 => Error::AddressRange,
        2 => Error::Aborted,
        3 => Error::SourceAccess,
        4 => Error::DestinationAccess,
        other => Error::UnknownHardwareStatus(other),
    }
}
