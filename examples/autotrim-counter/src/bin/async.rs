#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    autotrim::{self, AutotrimTimer, Config, Prescaler},
    bind_interrupts, peripherals,
};

bind_interrupts!(struct Irqs {
    AUTOTRIM_LCD => autotrim::InterruptHandler<peripherals::AUTOTRIM>,
        hal::lcd::InterruptHandler<peripherals::LCD>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    // 1500 prescaled cycles * 32768 raw HSIOSC cycles. This is a qualified
    // source-cycle period, not an exact one-second wall-clock deadline.
    let mut timer = AutotrimTimer::new_async(
        p.AUTOTRIM,
        Irqs,
        Config {
            prescaler: Prescaler::Div32768,
            reload: 1499,
        },
    )
    .unwrap();
    timer.start().unwrap();
    loop {
        // Coalesced IRQ event, not an exact count of elapsed periods.
        timer.wait().await;
        cortex_m::asm::nop(); // Perform periodic application work here.
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
