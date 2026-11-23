#![no_std]
use embassy_cw32::{
    self as hal,
    rcc::{Lse, LseAmplitude, LseDrive, LseMode, LseWait, OperatingConditions},
    time::Hertz,
};
/// Example declarations, not measurements. Replace them with qualified board data.
pub fn config(mode: LseMode) -> hal::Config {
    let board = OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = board;
    config.rcc.lse = Some(Lse {
        min_freq: Hertz(32766),
        max_freq: Hertz(32770),
        operating_conditions: board,
        mode,
        drive: LseDrive::Strong,
        amplitude: LseAmplitude::Normal,
        #[cfg(lse_startup_analog)]
        startup_drive: LseDrive::Normal,
        #[cfg(lse_startup_analog)]
        startup_amplitude: LseAmplitude::Normal,
        wait: LseWait::Cycles16384,
        // Attempts, not a guaranteed startup duration. POR-retained source on timeout.
        poll_budget: 20_000_000,
    });
    config
}
pub fn initial_time() -> hal::rtc::DateTime {
    hal::rtc::DateTime::from(2026, 10, 9, hal::rtc::DayOfWeek::Friday, 12, 0, 0, 0).unwrap()
}
pub fn display(rtc: hal::rtc::Rtc<'_>, pin: hal::Peri<'_, hal::peripherals::PA4>) -> ! {
    #[cfg(gpio_has_speed)]
    let speed = hal::gpio::Speed::Low;
    #[cfg(not(gpio_has_speed))]
    let speed = hal::gpio::Speed::Default;
    let mut indicator = hal::gpio::Output::new(pin, hal::gpio::Level::Low, speed);
    let mut previous = None;
    loop {
        let now = rtc.now().unwrap();
        if previous != Some(now.second()) {
            indicator.toggle();
            previous = Some(now.second());
        }
        core::hint::black_box((now, rtc.calendar_tick_bounds()));
    }
}
