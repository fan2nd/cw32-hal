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
    let mut dac = dac.into_software_triggered().unwrap();
    loop {
        for code in [0, 1024, 2048, 3072, 4095] {
            dac.stage(hal::dac::Channel::Ch1, hal::dac::Value::Bit12(code))
                .unwrap();
            dac.stage(
                hal::dac::Channel::Ch2,
                hal::dac::Value::Bit8((code >> 4) as u8),
            )
            .unwrap();
            dac.trigger(hal::dac::Channel::Ch1, 1024).unwrap();
            dac.trigger(hal::dac::Channel::Ch2, 1024).unwrap();
            // Commands are sequential; completion means only digital loading.
            // Neither simultaneous analog outputs nor settling is guaranteed.
            core::hint::black_box((
                dac.read(hal::dac::Channel::Ch1),
                dac.read(hal::dac::Channel::Ch2),
            ));
            cortex_m::asm::delay(800_000);
        }
    }
}
