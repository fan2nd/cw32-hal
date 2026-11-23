//! Full-length classic ordered scans, caller cancellation, then fresh reuse.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, AdcChannel, Config, InterruptHandler, MAX_SEQUENCE_LEN, SampleTime},
    bind_interrupts, peripherals,
};
use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Timer, with_timeout};

bind_interrupts!(struct Irqs {
    ADC => InterruptHandler<peripherals::ADC>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // Board-qualified low-impedance external inputs, within 0..VDDA.
    // Supply is 3.0–3.6 V with VDDA=VDD and qualified HSI conditions.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    let p = hal::init(system).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    let mut adc = Adc::new_async(p.ADC, Irqs, config).unwrap();
    let (mut first_pin, mut second_pin) = include!(concat!(env!("OUT_DIR"), "/channel-pins.rs"));
    let mut supply = adc.enable_vdda();
    loop {
        {
            let first = first_pin.reborrow_adc();
            let second = second_pin.reborrow_adc();
            // The hardware has four or eight slots. Every slot must use the
            // same acquisition setting and an unbuffered external source.
            let sequence: [_; MAX_SEQUENCE_LEN] = core::array::from_fn(|slot| {
                (
                    if slot % 2 == 0 { &first } else { &second },
                    SampleTime::Cycles10,
                )
            });
            let mut samples = [0xffff; MAX_SEQUENCE_LEN];
            // Either outcome is valid: this is ordinary future composition,
            // with no promised IRQ latency or deterministic timeout winner.
            let timing = match select(
                adc.read_sequence(&sequence, &mut samples),
                Timer::after_micros(50),
            )
            .await
            {
                Either::First(result) => Some(result.unwrap()),
                Either::Second(()) => None,
            };
            // The losing future has dropped. Cancellation leaves all elements
            // unchanged; completion commits the full sequence in caller order.
            core::hint::black_box((samples, timing));
            match with_timeout(
                Duration::from_millis(100),
                adc.read_sequence(&sequence, &mut samples),
            )
            .await
            {
                Ok(result) => core::hint::black_box((samples, Some(result.unwrap()))),
                Err(_) => core::hint::black_box((samples, None)),
            };
        }
        // Internal sources retain the single-read/follower qualification.
        // The next scan restores the independently qualified external mode.
        let result = with_timeout(
            Duration::from_millis(100),
            adc.read(&mut supply, SampleTime::Cycles10),
        )
        .await;
        let _ = core::hint::black_box(result);
        Timer::after_millis(10).await;
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
