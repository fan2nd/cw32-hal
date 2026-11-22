//! Declared board conditions qualifying factory-HSI clocks and bus limits.
use super::{Clocks, Error, HSI_BOUND_SUPPLY_MV, HSI_BOUND_TEMPERATURE_C};

/// Declared board supply voltage and ambient-temperature range.
///
/// These are declarations, not measurements. The board must sustain the operating conditions
/// required by the selected clock configuration throughout use.
/// Thermal/junction, analog-supply and peripheral-specific limits still apply.
/// A narrower interval does not tighten the factory HSI error bound.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OperatingConditions {
    /// Board supply voltage in millivolts.
    pub supply_mv: u16,
    /// Lowest guaranteed ambient TA in degrees Celsius.
    pub min_temperature_c: i16,
    /// Highest guaranteed ambient TA in degrees Celsius.
    pub max_temperature_c: i16,
}
impl OperatingConditions {
    /// Default to the lowest qualified VDD and full ambient range of this family.
    pub const fn new() -> Self {
        Self {
            supply_mv: HSI_BOUND_SUPPLY_MV.0,
            min_temperature_c: HSI_BOUND_TEMPERATURE_C.0,
            max_temperature_c: HSI_BOUND_TEMPERATURE_C.1,
        }
    }
}
impl Default for OperatingConditions {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub(crate) fn validate(c: OperatingConditions, clocks: Clocks) -> Result<(), Error> {
    validate_with_bus_policy(c, clocks, false)
}

/// Validate source/supply conditions, optionally using the requested nominal
/// bus rate instead of the worst-case HSI tolerance bound. The opt-in is used
/// by motor-control applications that run the factory-calibrated nominal HSI
/// clock and accept responsibility for its frequency envelope.
pub(crate) fn validate_with_bus_policy(
    c: OperatingConditions,
    clocks: Clocks,
    allow_nominal_bus_overclock: bool,
) -> Result<(), Error> {
    if c.min_temperature_c > c.max_temperature_c {
        return Err(Error::InvalidTemperatureRange);
    }
    if c.supply_mv < HSI_BOUND_SUPPLY_MV.0 || c.supply_mv > HSI_BOUND_SUPPLY_MV.1 {
        return Err(Error::SupplyOutsideQualifiedRange);
    }
    if c.min_temperature_c < HSI_BOUND_TEMPERATURE_C.0
        || c.max_temperature_c > HSI_BOUND_TEMPERATURE_C.1
    {
        return Err(Error::TemperatureOutsideQualifiedRange);
    }
    let maximum = if c.supply_mv < crate::RCC_LOW_VOLTAGE_THRESHOLD_MV {
        crate::RCC_LOW_VOLTAGE_BUS_MAX_HZ
    } else {
        crate::RCC_HIGH_VOLTAGE_BUS_MAX_HZ
    };
    if !allow_nominal_bus_overclock && clocks.hclk_bounds().maximum_exceeds(maximum) {
        return Err(Error::HclkTooHigh);
    }
    if !allow_nominal_bus_overclock && clocks.pclk_bounds().maximum_exceeds(maximum) {
        return Err(Error::PclkTooHigh);
    }
    Ok(())
}
