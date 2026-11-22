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
    let dac = hal::dac::Dac::new(p.DAC, p.PB0, p.PB1, config).unwrap();
    let (ch1, ch2) = dac.into_owned_channels();
    let mut ch1 = ch1.unwrap();
    let mut ch2 = ch2.unwrap();
    ch1.set(hal::dac::Value::Bit12(2048)).unwrap();
    drop(ch1); // Disable PB0 only; PB1 stays independently usable.
    loop {
        for code in [0, 1024, 2048, 3072, 4095] {
            ch2.set(hal::dac::Value::Bit8((code >> 4) as u8)).unwrap();
            core::hint::black_box(ch2.read());
            // Channel 2 remains owned after channel 1 Drop; no settling guarantee.
            cortex_m::asm::delay(800_000);
        }
    }
}
