//! Pure RCC admission checks; never initializes clocks or accesses MMIO.
#![cfg(any(
    feature = "cw32l010f8p6",
    feature = "cw32l010f8u6",
    feature = "cw32l010y8m6"
))]
use embassy_cw32::{
    rcc::{self, Config, Error, Hse, HseDrive, HseMode, Sysclk},
    time::Hertz,
};

fn config(minimum: u32, nominal: u32, maximum: u32, retained: bool) -> Config {
    let conditions = rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = Config::default();
    config.operating_conditions = conditions;
    config.sys = if retained { Sysclk::HSI } else { Sysclk::HSE };
    config.hse = Some(Hse {
        freq: Hertz(nominal),
        min_freq: Hertz(minimum),
        max_freq: Hertz(maximum),
        operating_conditions: conditions,
        mode: HseMode::Bypass,
        drive: HseDrive::Level2,
    });
    config
}

#[test]
fn low_bypass_interval_and_retained_source_admission() {
    for retained in [false, true] {
        for frequency in 3_908_159..=4_000_000 {
            let clocks = config(frequency, frequency, frequency, retained)
                .frequencies()
                .unwrap();
            if !retained {
                assert_eq!(clocks.sys_bounds().minimum(), Hertz(frequency));
                assert_eq!(clocks.sys_bounds().maximum(), Hertz(frequency));
            }
        }
        assert!(matches!(
            config(3_908_158, 3_950_000, 4_000_000, retained).frequencies(),
            Err(Error::HseOutsideQualifiedRange)
        ));
        // The whole declared tolerance must fit, including with HSI as SYSCLK.
        assert!(matches!(
            config(0, 3_950_000, 4_000_000, retained).frequencies(),
            Err(Error::InvalidHseBounds)
        ));
        assert!(matches!(
            config(3_950_001, 3_950_000, 4_000_000, retained).frequencies(),
            Err(Error::InvalidHseBounds)
        ));
    }
}

#[test]
fn example_tolerance_is_preserved_and_crystal_is_not_extended() {
    for retained in [false, true] {
        let example = config(3_949_881, 3_950_000, 3_950_119, retained);
        let clocks = example.frequencies().unwrap();
        if !retained {
            assert_eq!(clocks.sys_bounds().minimum(), Hertz(3_949_881));
            assert_eq!(clocks.sys_bounds().maximum(), Hertz(3_950_119));
        }
        let mut crystal = example;
        crystal.hse.as_mut().unwrap().mode = HseMode::Oscillator;
        assert!(matches!(
            crystal.frequencies(),
            Err(Error::HseOutsideQualifiedRange)
        ));
    }
}
