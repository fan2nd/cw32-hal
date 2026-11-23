#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    gpio::{Level, Output, Speed},
    rcc, usart,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(cw32_pll_clock_examples::config()).unwrap();
    #[cfg(gpio_speed)]
    let speed = Speed::Low;
    #[cfg(not(gpio_speed))]
    let speed = Speed::Default;
    let mut led = Output::new(p.PB0, Level::Low, speed);
    let mut uart = usart::UartTx::new_blocking(p.UART1, p.PA8, usart::Config::default()).unwrap();
    // Exact bounds remain available separately from rounded nominal Hertz.
    core::hint::black_box(rcc::clocks().pll_bounds());
    loop {
        uart.blocking_write(b"CW32 qualified PLL: UART1 115200 8N1\r\n")
            .unwrap();
        uart.blocking_flush().unwrap();
        led.toggle();
        // Activity indication only, not a precise wall-clock delay.
        cortex_m::asm::delay(rcc::clocks().hclk.0 / 4);
    }
}
