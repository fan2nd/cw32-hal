#![deny(unsafe_code)]
//! `compu.c` SampleVI then SampleT, called at the original 100 ms cadence.
//!
//! Preserve C's single-precision expression order, unsigned-char counters and
//! ErrorCode gating. Zero reference or a nonfinite/out-of-u32 conversion has no
//! defined C integer result; only that arithmetic boundary uses the explicit
//! InvalidCalibration guard. Calibration 0 and 0xffff are not rejected merely
//! for their values. This guard is a documented difference, not source parity.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// CanshuV: 0.1 V units.
    pub bus_decivolts: u32,
    /// CanshuI: mA.
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

    /// The C key path clears ErrorCode, not the functions' static counters.
    pub fn reset(&mut self) {
        self.fault = None;
    }

    /// `external_fault` is the current shared C ErrorCode, including motor
    /// errors or a key clear since the preceding call. Existing errors inhibit
    /// trips but do not skip SampleVI's measurement updates and counter resets.
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

    /// Original periodic sequence: SampleVI followed by SampleT.
    pub fn sample(
        &mut self,
        sample: Adc2Sample,
        external_fault: Option<Fault>,
    ) -> Result<Measurements, Fault> {
        self.sample_vi(sample, external_fault)?;
        // SampleT returns before touching ntcCount for an existing error or
        // st < 50. Neither condition clears previously accumulated hot samples.
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
    // The upper endpoint is exclusive: 2^32 cannot be converted to C uint32_t.
    // Do not silently use Rust's saturating float cast outside C's domain.
    (value.is_finite() && (0.0..4_294_967_296.0).contains(&value)).then(|| value as u32)
}
