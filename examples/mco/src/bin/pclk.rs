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
    // APB /2 makes this distinct from HCLK. MCO /8 is nominally 500 kHz.
    // SYSCTRL stays owned here; no live HAL RTC clock owner can coexist.
    let mco = Mco::new(
        p.SYSCTRL,
        p.PA8,
        McoSource::Pclk,
        McoPrescaler::Div8,
        board::BOARD,
    )
    .unwrap();
    loop {
        core::hint::black_box(mco.clock_bounds());
        cortex_m::asm::nop();
    }
}
