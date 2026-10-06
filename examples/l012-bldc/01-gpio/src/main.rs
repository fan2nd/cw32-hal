//! 仅进行 GPIO 初步调试：只使用 LED 和按键，不操作电机引脚。
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use embassy_cw32::gpio::{Input, Level, Output, Pull};

#[entry]
fn main() -> ! {
    defmt::info!("01 GPIO boot; motor gate pins untouched");
    // 使用默认的 4 MHz 复位配置，未启用时间驱动特性。
    let p = embassy_cw32::init(Default::default());
    let clocks = embassy_cw32::rcc::clocks();
    defmt::info!(
        "nominal clocks: sysclk={} Hz hclk={} Hz pclk={} Hz",
        clocks.sysclk_hz(),
        clocks.hclk_hz(),
        clocks.pclk_hz()
    );
    let mut led = Output::new(p.PC13, Level::High);
    let key = Input::new(p.PA3, Pull::Up);
    let mut logged_pressed = key.is_low();
    defmt::info!("PA3 key_pressed={} (active low)", logged_pressed);
    // 仅限制诊断输出频率，LED/按键仍保持即时响应。这里采用的是
    // 循环计数分频，不是校准后的时基，也不用于按键消抖。
    let mut log_poll = 0u32;
    loop {
        let pressed = key.is_low();
        led.set_level(if pressed { Level::Low } else { Level::High });
        log_poll += 1;
        if log_poll == 100_000 {
            log_poll = 0;
            if pressed != logged_pressed {
                logged_pressed = pressed;
                defmt::info!("PA3 key_pressed={} PC13 led_on={}", pressed, pressed);
            }
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}
