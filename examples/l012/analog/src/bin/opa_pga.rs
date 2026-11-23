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
    let mut config = hal::opamp::Config::default();
    config.supply_mv = 3300;
    let mut opamp = hal::opamp::OpAmp::new(p.OPA2, config).unwrap();
    // Bring-up margin only. Qualify BGR stability on this board first;
    // no source-backed maximum startup delay is claimed by this example.
    cortex_m::asm::delay(80_000);
    let _output = opamp
        .pga_ext(p.PA4, p.PB1, hal::opamp::OpAmpGain::Mul8)
        .unwrap();
    // Observe with a high-impedance scope. No ready/accuracy assertion.
    loop {
        cortex_m::asm::nop();
    }
}
