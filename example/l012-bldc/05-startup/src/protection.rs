#![deny(unsafe_code)]
//! ADC2 conversion and 100 ms protection checks from `compu.c`.
//!
//! Call [`Protection::sample`] once for each new 100 ms sample. Thirty
//! consecutive bad samples trip the current/voltage protections; twenty-five
//! valid hot NTC samples trip the temperature protection. These checks are not
//! PWM-cycle protection: the source's `HardFAULTAD = 403` and `NUMtimes = 2`
//! constants are unused and are deliberately not implemented here.
//!
//! Conversions use exact `u64` rational arithmetic and truncate only the final
//! result, rather than reproducing each intermediate single-precision rounding
//! in C. Values very close to an integer boundary can therefore differ from the
//! C float result. Out-of-range current measurements saturate instead of wrapping.

/// Source error codes, plus a fail-safe code for unusable ADC calibration.
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
    /// A motor-control ADC stream missed its explicit freshness deadline.
    AdcStale = 11,
    /// The async UI/task command heartbeat missed the board watchdog deadline.
    UiStale = 12,
}

/// The five ADC2 sequence results, in the original channel order.
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
    /// Bus voltage in 0.1 V units (`CanshuV`).
    pub bus_decivolts: u32,
    /// Bus current in mA (`CanshuI`).
    pub current_ma: u32,
}

/// Stateful debouncing for the source's `SampleVI()` and `SampleT()`.
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
    /// `calibration_mv` is the factory value at `ADC_BGR_VOL_ADDRESS`.
    /// The startup current offset is 884, as in `global.c`; replace it with an
    /// idle ADC reading using [`Self::set_current_offset`] before driving a motor.
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

    /// Clear the latched fault and all consecutive-sample counters.
    /// Calibration, current offset, and the latest valid measurements survive.
    /// The caller is responsible for keeping the bridge off during recovery.
    pub fn reset(&mut self) {
        self.fault = None;
        self.current_count = 0;
        self.high_voltage_count = 0;
        self.low_voltage_count = 0;
        self.temperature_count = 0;
    }

    /// Convert one ADC2 sample and run the checks in the original source order.
    ///
    /// The first fault stays latched until [`Self::reset`]. Valid voltage and
    /// current readings still update after a fault, as in `SampleVI()`. Zero
    /// or erased (0xffff) calibration, or zero reference, latches
    /// `InvalidCalibration` without dividing or
    /// changing the last valid measurements; it never replaces an earlier fault.
    pub fn sample(&mut self, sample: Adc2Sample) -> Result<Measurements, Fault> {
        if self.calibration_mv == 0 || self.calibration_mv == u16::MAX || sample.reference == 0 {
            return Err(self.latch(Fault::InvalidCalibration));
        }

        let calibration = u64::from(self.calibration_mv);
        let reference = u64::from(sample.reference);
        let current_delta = u64::from(sample.current.saturating_sub(self.current_offset));
        // 10x amplifier and 0.01 ohm shunt: measured millivolts * 10 = mA.
        let current_ma = calibration * current_delta * 10 / reference;
        // RV1 = 1, RV2 = 10: undo the 1:11 divider and convert mV to 0.1 V.
        let bus_decivolts = calibration * u64::from(sample.bus_voltage) * 11 / (reference * 100);
        self.measurements = Measurements {
            bus_decivolts: bus_decivolts as u32,
            current_ma: current_ma.min(u64::from(u32::MAX)) as u32,
        };

        if let Some(fault) = self.fault {
            return Err(fault);
        }

        // Source order matters: on sample 30 the sustained trip precedes even
        // a simultaneous 10 A trip, and electrical faults precede temperature.
        if consecutive(
            &mut self.current_count,
            self.measurements.current_ma >= 3_000,
            30,
        ) {
            return Err(self.latch(Fault::SustainedOvercurrent));
        }
        if self.measurements.current_ma >= 10_000 {
            return Err(self.latch(Fault::InstantOvercurrent));
        }
        if consecutive(
            &mut self.high_voltage_count,
            self.measurements.bus_decivolts >= 140,
            30,
        ) {
            return Err(self.latch(Fault::Overvoltage));
        }
        let voltage = self.measurements.bus_decivolts;
        if consecutive(
            &mut self.low_voltage_count,
            voltage < 66 || (voltage > 95 && voltage < 100),
            30,
        ) {
            return Err(self.latch(Fault::Undervoltage));
        }

        // Preserve C's paused counter below 50: an invalid/noisy reading cannot
        // erase already accumulated hot samples. A valid cool reading (>342)
        // resets it. Invalid readings do not prove a safe temperature or detect
        // an open/shorted NTC; sensor-integrity needs hardware validation.
        if sample.temperature < 50 {
            return Ok(self.measurements);
        }
        if consecutive(
            &mut self.temperature_count,
            (50..=342).contains(&sample.temperature),
            25,
        ) {
            return Err(self.latch(Fault::Overtemperature));
        }

        Ok(self.measurements)
    }

    fn latch(&mut self, fault: Fault) -> Fault {
        *self.fault.get_or_insert(fault)
    }
}

fn consecutive(count: &mut u8, condition: bool, limit: u8) -> bool {
    if condition {
        *count = count.saturating_add(1);
    } else {
        *count = 0;
    }
    *count >= limit
}
