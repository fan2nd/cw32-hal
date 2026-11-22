//! Independent ADC1/ADC2 EOS-driven single reads with live direct DAC outputs.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config, InterruptHandler, SampleTime},
    bind_interrupts,
    dac::{Channel, Dac, Value},
    peripherals,
    time::Hertz,
};
use embassy_futures::join::join;
use embassy_time::{Duration, Timer, with_timeout};

bind_interrupts!(struct Irqs {
    ADC1 => InterruptHandler<peripherals::ADC1>;
    ADC2_DAC => InterruptHandler<peripherals::ADC2>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // Example board: nominal 3.3 V, guaranteed 3.0–3.6 V, ambient -40..85 C.
    // VDDA must equal VDD; PA0/PA8 stay within 0..VDDA. Qualify the DAC loads.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div8;
    let p = hal::init(system).unwrap();
    let mut dac_config = hal::dac::Config::default();
    dac_config.supply_mv = 3300;
    let mut dac = Dac::new(p.DAC, p.PB0, p.PB1, dac_config).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(24_000_000);
    // Config.timeout only bounds synchronous register polls, not the await.
    let mut adc1 = Adc::new_async(p.ADC1, Irqs, config).unwrap();
    // Admission reads actual DAC interrupt enables. The direct-loading DAC
    // stays owned and active; ADC2 never changes its controls, data or flags.
    let mut adc2 = Adc::new_async(p.ADC2, Irqs, config).unwrap();
    let mut input1 = p.PA0;
    let mut input2 = p.PA8;
    let mut code = 0;
    loop {
        dac.set(Channel::Ch1, Value::Bit12(code)).unwrap();
        dac.set(Channel::Ch2, Value::Bit12(4095 - code)).unwrap();
        // Both conversions can be pending together while PB0/PB1 keep driving.
        // join polls ordinary futures; neither converter waits for the other IRQ.
        let (first, second) = join(
            with_timeout(
                Duration::from_millis(100),
                adc1.read(&mut input1, SampleTime::Cycles102),
            ),
            with_timeout(
                Duration::from_millis(100),
                adc2.read(&mut input2, SampleTime::Cycles262),
            ),
        )
        .await;
        for result in [first, second] {
            match result {
                Ok(sample) => {
                    // A HAL error is distinct from the caller's deadline.
                    core::hint::black_box(sample.unwrap());
                }
                Err(_) => {
                    // Drop masks local completion, stops/resets the sequence,
                    // and preserves the sibling and DAC. The next loop reuses
                    // the owner; terminal cleanup faults surface on that call.
                    core::hint::black_box("caller deadline elapsed");
                }
            }
        }
        core::hint::black_box((adc1.timing(), adc2.timing()));
        code = (code + 256) & 4095;
        // This pacing delay is not an analog-settling qualification.
        Timer::after_millis(10).await;
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
