#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal,
    mode::Async,
    timer::low_level::{Prescaler, Timer},
};

// The build script checks each selected bank's actual GLOBAL IRQ metadata.
// A shared-vector partner enabled elsewhere also needs its own handler here.
hal::bind_interrupts!(struct Irqs {
    BTIM1 => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM1>;
    #[cfg(not(any(example_l052, example_l083)))]
    BTIM2 => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM2>;
    #[cfg(example_l052)]
    BTIM2_LPTIM => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM2>;
    #[cfg(example_l083)]
    BTIM2_LPTIM1 => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM2>;
    #[cfg(not(example_l012))]
    BTIM3 => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM3>;
    #[cfg(example_l012)]
    BTIM3_HALLTIM => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM3>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut fast = Timer::<_, Async>::new_async(p.BTIM1, Irqs).unwrap();
    let mut medium = Timer::<_, Async>::new_async(p.BTIM2, Irqs).unwrap();
    let mut slow = Timer::<_, Async>::new_async(p.BTIM3, Irqs).unwrap();

    // Nominal event rates are PCLK / 16_000_000, / 32_000_000 and / 64_000_000.
    // Explicit periods avoid assuming a common PCLK rate across these packages.
    fast.set_period(Prescaler::Div256, 62_500).unwrap();
    medium.set_period(Prescaler::Div512, 62_500).unwrap();
    slow.set_period(Prescaler::Div1024, 62_500).unwrap();
    fast.start();
    medium.start();
    slow.start();

    loop {
        fast.wait_overflow().await;
        cortex_m::asm::nop(); // Put frequent run-mode housekeeping here.
        if medium.take_overflow() {
            cortex_m::asm::nop(); // Put less frequent housekeeping here.
        }
        if slow.take_overflow() {
            cortex_m::asm::nop(); // Put slow housekeeping here.
        }
        // Each consumed event means at least one overflow, never an exact count.
        // Stopping PCLK or IRQ service can leave wait_overflow pending forever.
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
