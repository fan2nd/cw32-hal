#![no_std]
#![no_main]
use core::num::NonZeroU32;
use embassy_cw32::{
    bind_interrupts,
    lcd::{self, Bias, BiasSource, Config, Duty, ElectricalConfig, Lcd, LcdPin, ScanFrequency},
    peripherals,
};
bind_interrupts!(struct Irqs { AUTOTRIM_LCD => lcd::InterruptHandler<peripherals::LCD>; });

// Board assumptions: VDD is 3.3 V nominal and never exceeds 3.6 V, and the
// attached quarter-duty, third-bias glass tolerates a 3.6 V peak waveform.
// Wire COM0..3 to PA9..12; SEG0/5/6/7 to PA8/PB15/PB14/PB13. These pads are
// bonded on every exact L052/L083 feature offered by this example.
#[cortex_m_rt::entry]
fn main() -> ! {
    let board = embassy_cw32::rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let mut config = embassy_cw32::Config::default();
    config.rcc.operating_conditions = board;
    config.rcc.lse = Some(embassy_cw32::rcc::Lse {
        min_freq: embassy_cw32::time::Hertz(32766),
        max_freq: embassy_cw32::time::Hertz(32770),
        operating_conditions: board,
        mode: embassy_cw32::rcc::LseMode::Oscillator,
        drive: embassy_cw32::rcc::LseDrive::Strong,
        amplitude: embassy_cw32::rcc::LseAmplitude::Normal,
        #[cfg(lse_startup_analog)]
        startup_drive: embassy_cw32::rcc::LseDrive::Normal,
        #[cfg(lse_startup_analog)]
        startup_amplitude: embassy_cw32::rcc::LseAmplitude::Normal,
        wait: embassy_cw32::rcc::LseWait::Cycles16384,
        poll_budget: 20_000_000,
    });
    // Example board declaration, not measured crystal accuracy.
    let p = embassy_cw32::init(config).unwrap();
    let clock = embassy_cw32::rcc::LseClock::new(p.SYSCTRL, p.PC14, p.PC15).unwrap();
    let pins = [
        LcdPin::com::<0>(p.PA9),
        LcdPin::com::<1>(p.PA10),
        LcdPin::com::<2>(p.PA11),
        LcdPin::com::<3>(p.PA12),
        LcdPin::segment::<0>(p.PA8),
        LcdPin::segment::<5>(p.PB15),
        LcdPin::segment::<6>(p.PB14),
        LcdPin::segment::<7>(p.PB13),
    ];
    let mut lcd = Lcd::new_with_lse(
        p.LCD,
        pins,
        Irqs,
        Config {
            duty: Duty::Quarter,
            bias: Bias::Third,
            drive: BiasSource::InternalMedium,
            scan: ScanFrequency::Hz256,
            contrast: 0,
            electrical: ElectricalConfig {
                maximum_supply_mv: 3600,
                panel_maximum_drive_mv: 3600,
            },
            poll_budget: NonZeroU32::new(1_000_000).unwrap(),
        },
        clock,
    )
    .unwrap();
    lcd.set_blink(true, 32).unwrap();
    lcd.fill(true);
    // Per-frame APIs explicitly reject the longer shared event interval.
    assert_eq!(lcd.present(), Err(lcd::Error::EventIntervalNotOne));
    loop {
        lcd.present_on_event().unwrap();
        lcd.wait_next_event().unwrap();
        core::hint::black_box((lcd.typical_source_frequency(), lcd.source_bounds()));
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
