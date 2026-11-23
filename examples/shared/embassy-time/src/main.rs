#![no_std]
#![no_main]
use embassy_cw32 as hal;
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};

#[embassy_executor::task]
async fn slow_work() {
    loop {
        // Longer than a full 16-bit wrap: the queue retains absolute deadlines.
        Timer::after_millis(100).await;
        core::hint::black_box(Instant::now());
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Before HAL init, the global clock safely reads zero without timer MMIO.
    core::hint::black_box(Instant::now());
    let _peripherals = hal::init(Default::default()).unwrap();
    core::hint::black_box(hal::time_driver::tick_bounds());
    spawner.spawn(slow_work().unwrap());
    loop {
        Timer::after_millis(1).await;
        let next = Instant::now() + Duration::from_millis(33);
        Timer::at(next).await;
        core::hint::black_box(Instant::now());
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
