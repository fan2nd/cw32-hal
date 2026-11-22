#![no_std]
#![no_main]

use cw32f030_examples as _;
use embassy_cw32::{
    self as hal, exti,
    gpio::{Level, Output, Pull, Speed},
    interrupt,
};
use embassy_executor::Spawner;

hal::bind_interrupts!(struct Irqs {
    GPIOA => exti::InterruptHandler<interrupt::typelevel::GPIOA>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut led = Output::new(p.PB0, Level::Low, Speed::Low);
    let mut input = exti::ExtiInput::new(p.PA0, Pull::Up, Irqs);

    loop {
        // A falling edge wakes the real Embassy thread-mode executor via GPIOA.
        // No polling task, timer shim, or embassy-time driver is involved.
        input.wait_for_falling_edge().await;
        led.toggle();
    }
}
