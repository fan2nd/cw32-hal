#![deny(unsafe_code)]
//! 按原有 100 ms 周期，先调用 `compu.c` 的 SampleVI，再调用 SampleT。
//!
//! 保留 C 源码的单精度表达式求值顺序、unsigned-char 计数器以及
//! ErrorCode 条件控制。参考值为零，或转换值非有限/超出 u32 范围时，
//! C 语言没有定义对应的整数结果；仅在这些算术边界使用显式的
//! InvalidCalibration 防护。不会仅因校准值为 0 或 0xffff
//! 就拒绝它们。此防护是已记录的差异，不声称与原源码一致。

#[derive(Clone, Copy, Debug, defmt::Format, Eq, PartialEq)]
#[repr(u8)]
pub enum Fault {
    TooFast = 2,
    StartupFailed = 3,
    SustainedOvercurrent = 4,
    InstantOvercurrent = 5,
    Overvoltage = 6,
    Stall = 7,
    Undervoltage = 8,
    Overtemperature = 9,
    InvalidCalibration = 10,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Adc2Sample {
    pub current: u16,
    pub bus_voltage: u16,
    pub potentiometer: u16,
    pub temperature: u16,
    pub reference: u16,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Measurements {
    /// CanshuV：单位为 0.1 V。
    pub bus_decivolts: u32,
    /// CanshuI：单位为 mA。
    pub current_ma: u32,
}

#[derive(Clone, Debug)]
pub struct Protection {
    calibration_mv: u16,
    current_offset: u16,
    measurements: Measurements,
    fault: Option<Fault>,
    current_count: u8,
    high_voltage_count: u8,
    low_voltage_count: u8,
    temperature_count: u8,
}

impl Protection {
    pub const fn new(calibration_mv: u16) -> Self {
        Self {
            calibration_mv,
            current_offset: 884,
            measurements: Measurements {
                bus_decivolts: 0,
                current_ma: 0,
            },
            fault: None,
            current_count: 0,
            high_voltage_count: 0,
            low_voltage_count: 0,
            temperature_count: 0,
        }
    }

    pub fn set_current_offset(&mut self, offset: u16) {
        self.current_offset = offset;
    }

    pub const fn measurements(&self) -> Measurements {
        self.measurements
    }

    pub const fn fault(&self) -> Option<Fault> {
        self.fault
    }

    /// C 源码的按键路径只清除 ErrorCode，不清除这些函数的静态计数器。
    pub fn reset(&mut self) {
        self.fault = None;
    }

    /// `external_fault` 为当前共享的 C ErrorCode，包含自上次调用以来的电机
    /// 错误或按键清错结果。已有错误会阻止新的保护触发，
    /// 但不会跳过 SampleVI 的测量值更新和计数器重置。
    pub fn sample_vi(
        &mut self,
        sample: Adc2Sample,
        external_fault: Option<Fault>,
    ) -> Result<Measurements, Fault> {
        self.fault = external_fault;
        let calibration = f32::from(self.calibration_mv);
        let reference = f32::from(sample.reference);
        let mut t = f32::from(sample.current);
        if t <= f32::from(self.current_offset) {
            self.measurements.current_ma = 0;
        } else {
            t -= f32::from(self.current_offset);
            t = calibration * t / reference;
            t *= 10.0;
            let Some(current) = c_unsigned_int(t) else {
                return Err(self.latch(Fault::InvalidCalibration));
            };
            self.measurements.current_ma = current;
        }

        if self.measurements.current_ma >= 3_000 && self.fault.is_none() {
            self.current_count = self.current_count.wrapping_add(1);
            if self.current_count >= 30 {
                self.fault = Some(Fault::SustainedOvercurrent);
                self.current_count = 0;
            }
        } else {
            self.current_count = 0;
        }
        if self.measurements.current_ma >= 10_000 && self.fault.is_none() {
            self.fault = Some(Fault::InstantOvercurrent);
        }

        t = f32::from(sample.bus_voltage);
        t = calibration * t / reference;
        t = t / 1.0 * (1.0 + 10.0) / 100.0;
        let Some(voltage) = c_unsigned_int(t) else {
            return Err(self.latch(Fault::InvalidCalibration));
        };
        self.measurements.bus_decivolts = voltage;
        if voltage >= 140 && self.fault.is_none() {
            self.high_voltage_count = self.high_voltage_count.wrapping_add(1);
            if self.high_voltage_count >= 30 {
                self.fault = Some(Fault::Overvoltage);
            }
        } else {
            self.high_voltage_count = 0;
        }
        if (voltage < 66 || (voltage < 100 && voltage > 95)) && self.fault.is_none() {
            self.low_voltage_count = self.low_voltage_count.wrapping_add(1);
            if self.low_voltage_count >= 30 {
                self.fault = Some(Fault::Undervoltage);
            }
        } else {
            self.low_voltage_count = 0;
        }

        match self.fault {
            Some(fault) => Err(fault),
            None => Ok(self.measurements),
        }
    }

    /// 原有周期调用顺序：先 SampleVI，再 SampleT。
    pub fn sample(
        &mut self,
        sample: Adc2Sample,
        external_fault: Option<Fault>,
    ) -> Result<Measurements, Fault> {
        self.sample_vi(sample, external_fault)?;
        // 存在错误或 st < 50 时，SampleT 会在访问 ntcCount 前返回。
        // 这两个条件都不会清除此前累计的高温样本计数。
        if self.fault.is_none() && sample.temperature >= 50 {
            if sample.temperature <= 342 {
                self.temperature_count = self.temperature_count.wrapping_add(1);
                if self.temperature_count >= 25 {
                    self.fault = Some(Fault::Overtemperature);
                }
            } else {
                self.temperature_count = 0;
            }
        }
        match self.fault {
            Some(fault) => Err(fault),
            None => Ok(self.measurements),
        }
    }

    fn latch(&mut self, fault: Fault) -> Fault {
        *self.fault.get_or_insert(fault)
    }
}

fn c_unsigned_int(value: f32) -> Option<u32> {
    // 不包含上界：2^32 无法转换为 C 的 uint32_t。
    // 超出 C 定义域时，不得悄悄使用 Rust 的饱和浮点转换。
    (value.is_finite() && (0.0..4_294_967_296.0).contains(&value)).then(|| value as u32)
}
