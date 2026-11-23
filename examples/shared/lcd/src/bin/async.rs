#![no_std]
#![no_main]
use core::num::NonZeroU32;
use embassy_cw32::{
    autotrim, bind_interrupts,
    lcd::{self, Bias, BiasSource, Config, Duty, ElectricalConfig, Lcd, LcdPin, ScanFrequency},
    peripherals,
};
bind_interrupts!(struct Irqs { AUTOTRIM_LCD => lcd::InterruptHandler<peripherals::LCD>, autotrim::InterruptHandler<peripherals::AUTOTRIM>; });

// Board assumptions: VDD is 3.3 V nominal and never exceeds 3.6 V, and the
// attached quarter-duty, third-bias glass tolerates a 3.6 V peak waveform.
// Wire COM0..3 to PA9..12; SEG0/5/6/7 to PA8/PB15/PB14/PB13. These pads are
// bonded on every exact L052/L083 feature offered by this example.
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = embassy_cw32::init(Default::default()).unwrap();
    // Both owners share one NVIC vector but acknowledge only their own source.
    let mut timer =
        autotrim::AutotrimTimer::new_async(p.AUTOTRIM, Irqs, Default::default()).unwrap();
    timer.start().unwrap();
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
    let mut lcd = Lcd::new(
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
    )
    .unwrap();
    loop {
        // Walk a real display crosspoint pattern, holding each for 32 frames.
        for segment in [0, 5, 6, 7] {
            lcd.fill(false);
            for common in 0..4 {
                lcd.set_pixel(common, segment, true).unwrap();
            }
            lcd.present_async().await.unwrap();
            timer.wait().await;
            for _ in 0..32 {
                lcd.wait_next_frame_async().await.unwrap();
            }
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
