//! EOC-driven single reads, caller deadline, and reuse on every classic line.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config, InterruptHandler, SampleTime},
    bind_interrupts, peripherals,
};
use embassy_time::{Duration, Timer, with_timeout};

bind_interrupts!(struct Irqs {
    ADC => InterruptHandler<peripherals::ADC>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // Example board: actual supply 3.0–3.6 V, VDDA=VDD, qualified HSI range.
    // Defaults provide the existing nominal 8 MHz time-driver-compatible PCLK.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    let p = hal::init(system).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    let mut adc = Adc::new_async(p.ADC, Irqs, config).unwrap();
    let (mut input, _other_input) = include!(concat!(env!("OUT_DIR"), "/channel-pins.rs"));
    let mut supply = adc.enable_vdda();
    loop {
        match with_timeout(
            Duration::from_millis(100),
            adc.read(&mut input, SampleTime::Cycles10),
        )
        .await
        {
            Ok(result) => {
                let sample = result.unwrap();
                core::hint::black_box((sample, adc.timing()));
            }
            Err(_) => {
                // The dropped future requests logical stop. A subsequent read
                // reinitializes after successful cleanup, or reports Faulted.
                // Classic sources do not specify an independent drain handshake.
                core::hint::black_box("caller deadline elapsed");
            }
        }
        let sample = with_timeout(
            Duration::from_millis(100),
            adc.read(&mut supply, SampleTime::Cycles10),
        )
        .await;
        // The HAL selects the internal source's existing follower/rate limits.
        let _ = core::hint::black_box(sample);
        Timer::after_millis(10).await;
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
