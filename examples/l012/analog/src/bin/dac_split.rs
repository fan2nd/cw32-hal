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
    let (ch1, ch2) = dac.split();
    let mut ch1 = ch1.unwrap();
    let mut ch2 = ch2.unwrap();
    loop {
        for code in [0, 1024, 2048, 3072, 4095] {
            ch1.set(hal::dac::Value::Bit12(code)).unwrap();
            ch2.set(hal::dac::Value::Bit8((code >> 4) as u8)).unwrap();
            core::hint::black_box((ch1.read(), ch2.read()));
            // Independent direct-loading views; no analog settling guarantee.
            cortex_m::asm::delay(800_000);
        }
    }
}
