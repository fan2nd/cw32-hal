#![deny(unsafe_code)]
//! The seven-byte UART frame from `init.c::SendDataToUsart()`.

/// Build the source's telemetry frame without sending it to a peripheral.
///
/// Bytes are `43 57 04`, the speed level, the low byte of bus voltage in
/// decivolts, the powered-off flag, and the modulo-256 sum of the first six
/// bytes. The source sums its full-width voltage before transmitting its low
/// byte; reducing before summing produces exactly the same checksum.
pub const fn telemetry_frame(speed_level: u8, bus_decivolts: u32, powered_off: bool) -> [u8; 7] {
    let voltage = bus_decivolts as u8;
    let off = powered_off as u8;
    let checksum = 0x43_u8
        .wrapping_add(0x57)
        .wrapping_add(0x04)
        .wrapping_add(speed_level)
        .wrapping_add(voltage)
        .wrapping_add(off);
    [0x43, 0x57, 0x04, speed_level, voltage, off, checksum]
}
