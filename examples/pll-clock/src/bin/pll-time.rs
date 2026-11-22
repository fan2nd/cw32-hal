#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    usart,
};
use embassy_executor::Spawner;
use embassy_time::{Instant, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Defaults: classic /6 x4 = 32 MHz, L083 /6 x7 = 56 MHz. Each
    // is an exact 1 MHz division supported by its own GTIM prescaler.
    // Both HSE modes use 8 MHz x4 = nominal 32 MHz, divided by 32 for
    // nominal 1 MHz ticks (or divided by 8 with low-voltage AHB /4).
    // Fractional-MHz PLL choices are not necessarily admitted by this driver.
    let p = hal::init(cw32_pll_clock_examples::config()).unwrap();
    #[cfg(gpio_speed)]
    let speed = Speed::Low;
    #[cfg(not(gpio_speed))]
    let speed = Speed::Default;
    let mut led = Output::new(p.PB0, Level::Low, speed);
    let mut uart = usart::UartTx::new_blocking(p.UART1, p.PA8, usart::Config::default()).unwrap();
    core::hint::black_box(hal::time_driver::tick_bounds());
    loop {
        Timer::after_millis(250).await;
        core::hint::black_box(Instant::now());
        led.toggle();
        uart.blocking_write(b"PLL clock: Embassy timer tick\r\n")
            .unwrap();
        uart.blocking_flush().unwrap();
    }
}
