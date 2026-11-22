#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32 as hal;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    let mut config = hal::dac::Config::default();
    config.supply_mv = 3300;
    let mut dac = hal::dac::Dac::new(p.DAC, p.PB0, p.PB1, config).unwrap();
    loop {
        for code in [0, 1024, 2048, 3072, 4095] {
            dac.set(hal::dac::Channel::Ch1, hal::dac::Value::Bit12(code))
                .unwrap();
            dac.set(
                hal::dac::Channel::Ch2,
                hal::dac::Value::Bit8((code >> 4) as u8),
            )
            .unwrap();
            // Slow scope-visible steps, not a guarantee of analog settling.
            cortex_m::asm::delay(800_000);
        }
    }
}
