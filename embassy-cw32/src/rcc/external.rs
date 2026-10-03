//! Board-supplied high-speed oscillator parameters.

/// Electrical connection to the documented OSC_IN/OSC_OUT pads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HseMode {
    /// A 4–32 MHz crystal/resonator. Both pads are reserved in analog mode.
    Crystal,
    /// A 4–32 MHz external input with 40–60% duty cycle on OSC_IN.
    /// OSC_OUT remains available as GPIO.
    Bypass,
}

/// HSE parameters applied before the oscillator is enabled.
///
/// Frequency, electrical levels, duty cycle, crystal loading and drive must
/// match the board. The HAL validates numerical limits; it cannot measure the
/// input, supply voltage or oscillator quality. All poll limits count register
/// reads, not elapsed time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HseConfig {
    pub frequency_hz: u32,
    pub mode: HseMode,
    /// Crystal drive strength: 0–7 on L012, 0–3 on F030.
    pub drive: u8,
    /// L012 drive strength before stabilization, 0–7.
    #[cfg(sysctrl_l012)]
    pub startup_drive: u8,
    /// 0, 1, 2, 3 select 8192, 32768, 131072, 262144 HSE cycles.
    pub wait_cycles: u8,
    /// Maximum readiness polls. Zero is rejected before MMIO.
    pub stabilization_limit: u32,
}

impl HseConfig {
    /// A crystal with default drive 2 and 131072 stabilization cycles.
    /// Board-specific drive and timeout values may need adjustment.
    pub const fn crystal(frequency_hz: u32) -> Self {
        Self {
            frequency_hz,
            mode: HseMode::Crystal,
            drive: 2,
            #[cfg(sysctrl_l012)]
            startup_drive: 2,
            wait_cycles: 2,
            stabilization_limit: 0xffff,
        }
    }

    /// A continuously supplied external clock on OSC_IN.
    pub const fn bypass(frequency_hz: u32) -> Self {
        Self {
            mode: HseMode::Bypass,
            ..Self::crystal(frequency_hz)
        }
    }

    pub(crate) const fn valid(self) -> bool {
        #[cfg(sysctrl_l012)]
        let drive_valid = self.drive <= 7 && self.startup_drive <= 7;
        #[cfg(sysctrl_f030)]
        let drive_valid = self.drive <= 3;
        self.frequency_hz >= 4_000_000
            && self.frequency_hz <= 32_000_000
            && drive_valid
            && self.wait_cycles <= 3
            && self.stabilization_limit != 0
    }

    pub(crate) const fn detection_cycles(self) -> u16 {
        // RM: DETCNT = 8000 / fHSE(MHz), including margin for clock error.
        // Round up so a fractional division cannot shorten that interval.
        8_000_000_000u64.div_ceil(self.frequency_hz as u64) as u16
    }
}
