//! Finite continuous acquisition with a caller deadline and subsequent reuse.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config, InterruptHandler, SampleTime},
    bind_interrupts, peripherals, time::Hertz,
};
use embassy_time::{Duration, with_timeout};

bind_interrupts!(struct Irqs {
    ADC => InterruptHandler<peripherals::ADC>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // Actual board supply 3.0–3.6 V, VDDA=VDD, qualified HSI conditions.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    let p = hal::init(system).unwrap();
    let mut config = Config::default();
    config.vdda_mv = 3000;
    config.frequency = Hertz(125_000);
    let mut adc = Adc::new_async(p.ADC, Irqs, config).unwrap();
    let (mut input, _other_input) = include!(concat!(env!("OUT_DIR"), "/channel-pins.rs"));
    let mut samples = [0; 32];
    match with_timeout(
        Duration::from_millis(100),
        adc.read_continuous(&mut input, SampleTime::Cycles10, &mut samples),
    ).await {
        Ok(Ok(timing)) => { core::hint::black_box((samples, timing)); }
        Ok(Err(error)) => { core::hint::black_box(error); }
        Err(_) => { core::hint::black_box("caller deadline elapsed"); }
    }
    // Error/timeout leaves samples invalid. A new read establishes fresh state,
    // or reports Faulted if the previous retirement could not be confirmed.
    let mut supply = adc.enable_vdda();
    let next = with_timeout(
        Duration::from_millis(100),
        adc.read(&mut supply, SampleTime::Cycles10),
    ).await;
    let _ = core::hint::black_box(next);
    drop(adc);
    core::future::pending::<()>().await;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}
