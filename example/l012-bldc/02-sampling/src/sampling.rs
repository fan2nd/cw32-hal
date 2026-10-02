#![deny(unsafe_code)]
//! Stage 02 adds only raw sampling and its clock contract to the GPIO lesson.
//! The hardware owner never offers an API to energize a bridge gate.

pub const CPU_HZ: u32 = 96_000_000;
pub const PCLK_HZ: u32 = 48_000_000;
pub const SAMPLING_HZ: u32 = 20_000;
pub const SAMPLING_PERIOD: u16 = (PCLK_HZ / SAMPLING_HZ) as u16;
pub const ADC_HZ: u32 = PCLK_HZ / 8;
pub const ADC1_SAMPLE_CYCLES: u32 = 18;
pub const ADC2_SAMPLE_CYCLES: u32 = 518;
pub const ADC_CONVERSION_CYCLES: u32 = 15;
pub const ADC2_INTERVAL_MS: u8 = 5;
pub const ADC1_SEQUENCE_NS: u32 =
    4 * (ADC1_SAMPLE_CYCLES + ADC_CONVERSION_CYCLES) * 1_000 / (ADC_HZ / 1_000_000);
pub const ADC2_SEQUENCE_NS: u32 =
    5 * (ADC2_SAMPLE_CYCLES + ADC_CONVERSION_CYCLES) * 1_000 / (ADC_HZ / 1_000_000);

/// ADC1 slots: current, phase A, phase B, phase C. These are sequential samples.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Adc1Sample {
    pub current: u16,
    pub phase_a: u16,
    pub phase_b: u16,
    pub phase_c: u16,
}
impl From<[u16; 4]> for Adc1Sample {
    fn from(raw: [u16; 4]) -> Self {
        Self {
            current: raw[0],
            phase_a: raw[1],
            phase_b: raw[2],
            phase_c: raw[3],
        }
    }
}

/// Raw observations only. No calibration, protection, motor state or protocol.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SamplingDiagnostics {
    pub adc1: [u16; 4],
    /// Current, bus voltage, potentiometer, temperature, internal reference.
    pub adc2: [u16; 5],
    pub milliseconds: u32,
    pub adc1_sequences: u32,
    pub adc2_sequences: u32,
    pub sampling_fault: bool,
}
impl SamplingDiagnostics {
    pub fn record_adc1(&mut self, raw: [u16; 4]) {
        self.adc1 = raw;
        self.adc1_sequences = self.adc1_sequences.wrapping_add(1);
    }
    pub fn record_adc2(&mut self, raw: [u16; 5]) {
        self.adc2 = raw;
        self.adc2_sequences = self.adc2_sequences.wrapping_add(1);
    }
}

/// Divides the 1 ms event independently of the wrapping diagnostic timestamp.
#[derive(Default)]
pub struct SamplingClock {
    elapsed: u8,
}
impl SamplingClock {
    pub fn tick_1ms(&mut self) -> bool {
        self.elapsed += 1;
        if self.elapsed == ADC2_INTERVAL_MS {
            self.elapsed = 0;
            true
        } else {
            false
        }
    }
}
