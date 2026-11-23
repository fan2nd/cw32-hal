#![no_std]
#![no_main]

use cw32_mco_examples as board;
use embassy_cw32::{
    self as hal,
    rcc::{Mco, McoPrescaler, McoSource},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = hal::init(board::config()).unwrap();
    // PA8 AF4: C8T7 LQFP48 pin 29, F030K8T7/K8U7 pin 18.
    // /8 gives a nominal 1 MHz logical clock on each exact selection.
    // This rate check does not qualify the loaded waveform or its duty cycle.
    let mco = Mco::new(
        p.SYSCTRL,
        p.PA8,
        McoSource::Hclk,
        McoPrescaler::Div8,
        board::BOARD,
    )
    .unwrap();
    loop {
        core::hint::black_box(mco.clock_bounds());
        cortex_m::asm::nop();
    }
}
