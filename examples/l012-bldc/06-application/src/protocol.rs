#![deny(unsafe_code)]
//! 来自 `init.c::SendDataToUsart()` 的七字节 UART 帧。

/// 构造与原源码一致的遥测帧，不向外设发送。
///
/// 字节依次为 `43 57 04`、速度档位、以 0.1 V 为单位的母线电压
/// 低字节、关闭标志，以及前六字节总和对 256 取模的结果。
/// 原源码先用完整位宽的电压求和，再发送其低
/// 字节；先截取低字节再求和会得到完全相同的校验和。
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
