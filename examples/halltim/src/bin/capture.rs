#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    gpio::Pull,
    halltim::{Config, Error, HallPin, HallTim, Prescaler},
};
#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    // Connect three push-pull Hall sensor outputs referenced to board ground.
    // PA3/PA4/PA5 are CH1/CH2/CH3 AF9 on both qualified 48-pin packages.
    let mut hall = HallTim::new(
        p.HALLTIM,
        HallPin::new(p.PA3, Pull::None),
        HallPin::new(p.PA4, Pull::None),
        HallPin::new(p.PA5, Pull::None),
        Config {
            prescaler: Prescaler::Div8,
            filter_length: 8,
            ..Default::default()
        },
    )
    .unwrap();
    let clock = hall.tick_clock();
    hall.restart();
    loop {
        match hall.wait_capture(100_000) {
            Ok(capture) => {
                // Process the latest observed sample. This is not lossless motor
                // commutation feedback: live levels are not latched with WIDTH.
                core::hint::black_box((capture, clock));
                if capture.overflow_since_restart {
                    hall.restart(); // Explicitly discard ambiguous elapsed intervals.
                }
            }
            Err(Error::Timeout) => {
                cortex_m::asm::nop();
            }
            Err(_) => {
                hall.stop();
                loop {
                    cortex_m::asm::bkpt();
                }
            }
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
