//! Independent ADC1/ADC2 ordered scans, cancellation/reuse and live DAC outputs.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, AdcChannel, Config, InterruptHandler, SampleTime},
    bind_interrupts,
    dac::{Channel, Dac, Value},
    peripherals,
    time::Hertz,
};
use embassy_futures::join::join;
use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Timer, with_timeout};

bind_interrupts!(struct Irqs {
    ADC1 => InterruptHandler<peripherals::ADC1>;
    ADC2_DAC => InterruptHandler<peripherals::ADC2>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // Example board: nominal 3.3 V, guaranteed 3.0–3.6 V, ambient -40..85 C.
    // VDDA=VDD; PA0/PA1/PA8 stay within 0..VDDA. Qualify the PB0/PB1 DAC loads.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.min_supply_mv = 3000;
    system.rcc.operating_conditions.max_supply_mv = 3600;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div8;
    let p = hal::init(system);
    let mut dac_config = hal::dac::Config::default();
    dac_config.supply_mv = 3300;
    let mut dac = Dac::new(p.DAC, p.PB0, p.PB1, dac_config).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(24_000_000);
    let mut adc1 = Adc::new_async(p.ADC1, Irqs, config).unwrap();
    let mut adc2 = Adc::new_async(p.ADC2, Irqs, config).unwrap();
    let mut input2 = p.PA8;
    let mut first = p.PA0;
    let mut second = p.PA1;
    let temperature1 = adc1.enable_temperature().degrade_adc();
    let bandgap1 = adc1.enable_vrefint().degrade_adc();
    let temperature2 = adc2.enable_temperature().degrade_adc();
    let bandgap2 = adc2.enable_vrefint().degrade_adc();
    let mut code = 0;
    loop {
        dac.set(Channel::Ch1, Value::Bit12(code)).unwrap();
        dac.set(Channel::Ch2, Value::Bit12(4095 - code)).unwrap();
        {
            let a = first.reborrow_adc();
            let b = second.reborrow_adc();
            let c = input2.reborrow_adc();
            let sequence1 = [
                (&a, SampleTime::Cycles54),
                (&b, SampleTime::Cycles102),
                (&temperature1, SampleTime::Cycles518),
                (&bandgap1, SampleTime::Cycles518),
                (&a, SampleTime::Cycles518),
                (&b, SampleTime::Cycles54),
                (&temperature1, SampleTime::Cycles518),
                (&bandgap1, SampleTime::Cycles518),
            ];
            let sequence2 = [
                (&c, SampleTime::Cycles54),
                (&temperature2, SampleTime::Cycles518),
                (&bandgap2, SampleTime::Cycles518),
                (&c, SampleTime::Cycles102),
                (&temperature2, SampleTime::Cycles518),
                (&bandgap2, SampleTime::Cycles518),
                (&c, SampleTime::Cycles262),
                (&c, SampleTime::Cycles518),
            ];
            let mut samples1 = [0xffff; 8];
            let mut samples2 = [0xffff; 8];
            // Both eight-slot scans can be active while the DAC drives PB0/PB1.
            // Each deadline independently drops its losing scan future. Either
            // select outcome is valid; this is not a cancellation/latency test.
            let (first_scan, second_scan) = join(
                select(
                    adc1.read_sequence(&sequence1, &mut samples1),
                    Timer::after_micros(50),
                ),
                select(
                    adc2.read_sequence(&sequence2, &mut samples2),
                    Timer::after_micros(50),
                ),
            )
            .await;
            for result in [first_scan, second_scan] {
                let timing = match result {
                    Either::First(result) => Some(result.unwrap()),
                    Either::Second(()) => None,
                };
                core::hint::black_box(timing);
            }
            // The losing future has been dropped. A canceled scan leaves every
            // output element unchanged; completed results retain caller order.
            core::hint::black_box((samples1, samples2));

            // Reuse the same owner, channels and output after completion or
            // successful logical cancellation. The HAL reestablishes fresh state.
            let (first_scan, second_scan) = join(
                with_timeout(
                    Duration::from_millis(100),
                    adc1.read_sequence(&sequence1, &mut samples1),
                ),
                with_timeout(
                    Duration::from_millis(100),
                    adc2.read_sequence(&sequence2, &mut samples2),
                ),
            )
            .await;
            for result in [first_scan, second_scan] {
                match result {
                    Ok(result) => {
                        core::hint::black_box(result.unwrap());
                    }
                    Err(_) => {
                        core::hint::black_box("caller deadline elapsed");
                    }
                }
            }
            core::hint::black_box((samples1, samples2));
        }
        // Pin/sequence borrows ended. Single conversion uses one slot again.
        let (first_read, second_read) = join(
            with_timeout(
                Duration::from_millis(100),
                adc1.read(&mut first, SampleTime::Cycles102),
            ),
            with_timeout(
                Duration::from_millis(100),
                adc2.read(&mut input2, SampleTime::Cycles102),
            ),
        )
        .await;
        for result in [first_read, second_read] {
            match result {
                Ok(sample) => {
                    core::hint::black_box(sample.unwrap());
                }
                Err(_) => {
                    core::hint::black_box("caller deadline elapsed");
                }
            }
        }
        code = (code + 256) & 4095;
        Timer::after_millis(10).await;
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
