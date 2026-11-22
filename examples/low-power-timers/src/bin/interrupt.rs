#![no_std]
#![no_main]
use embassy_cw32 as hal;
#[cfg(example_awt)]
hal::bind_interrupts!(struct Irqs { AWT => hal::awt::InterruptHandler<hal::peripherals::AWT>; });
#[cfg(example_l052)]
hal::bind_interrupts!(struct Irqs { BTIM2_LPTIM => hal::lptim::InterruptHandler<hal::peripherals::LPTIM>; });
#[cfg(example_l083)]
hal::bind_interrupts!(struct Irqs { BTIM2_LPTIM1 => hal::lptim::InterruptHandler<hal::peripherals::LPTIM>; });
#[cfg(not(any(example_awt, example_l052, example_l083)))]
hal::bind_interrupts!(struct Irqs { LPTIM => hal::lptim::InterruptHandler<hal::peripherals::LPTIM>; });
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(example_awt)]
    let mut timer = hal::awt::Awt::new_async(p.AWT, Irqs, Default::default()).unwrap();
    #[cfg(not(example_awt))]
    let mut timer = hal::lptim::Lptim::new_async(p.LPTIM, Irqs, Default::default()).unwrap();
    #[cfg(example_awt)]
    timer.start();
    #[cfg(not(example_awt))]
    timer.start().unwrap();
    loop {
        timer.wait().await;
        cortex_m::asm::nop(); // Put run-mode periodic work here.
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
