#![no_std]

use embassy_cw32::{self as hal, rcc};

/// Demonstration board declarations, not measurements or a qualified board BOM.
/// Replace these with limits established for the actual board and loaded PA8.
/// Total load includes routing, the receiver and any measurement equipment.
pub const BOARD: rcc::McoBoardConditions = rcc::McoBoardConditions {
    operating_conditions: rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    },
    max_load_pf: 50,
};

/// Keep the declaration identical at initialization and MCO acquisition.
/// Nominal HCLK is 8 MHz and PCLK is 4 MHz; actual bounds include HSI error.
pub fn config() -> hal::Config {
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = BOARD.operating_conditions;
    config.rcc.sys = rcc::Sysclk::HSI;
    config.rcc.hsi.div = rcc::HsiDiv::Div6;
    config.rcc.ahb_pre = rcc::AHBPrescaler::Div1;
    config.rcc.apb_pre = rcc::APBPrescaler::Div2;
    config
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
