#![no_std]
#![no_main]

#[cfg(not(any(example_l012, example_l052, example_l083)))]
compile_error!("btim_shared requires an admitted L012, L052 or L083 shared vector");

use embassy_cw32::{
    self as hal,
    mode::Async,
    timer::low_level::{Prescaler, Timer},
};

#[cfg(any(example_l052, example_l083))]
hal::bind_interrupts!(struct Irqs {
    #[cfg(example_l052)]
    BTIM2_LPTIM =>
        hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM2>,
        hal::lptim::InterruptHandler<hal::peripherals::LPTIM>;
    #[cfg(example_l083)]
    BTIM2_LPTIM1 =>
        hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM2>,
        hal::lptim::InterruptHandler<hal::peripherals::LPTIM>;
});

#[cfg(example_l012)]
hal::bind_interrupts!(struct Irqs {
    BTIM3_HALLTIM => hal::timer::OverflowInterruptHandler<hal::peripherals::BTIM3>;
});

#[cfg(any(example_l052, example_l083))]
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut btim = Timer::<_, Async>::new_async(p.BTIM2, Irqs).unwrap();
    let mut lptim = hal::lptim::Lptim::new_async(p.LPTIM, Irqs, Default::default()).unwrap();
    btim.set_period(Prescaler::Div256, 62_500).unwrap();
    btim.start();
    lptim.start().unwrap();

    loop {
        // Both real handlers run on each vector entry and service only their
        // own active source. Each wait consumes its owner's retained event.
        btim.wait_overflow().await;
        cortex_m::asm::nop(); // BTIM-paced housekeeping.
        lptim.wait().await;
        cortex_m::asm::nop(); // LPTIM-paced housekeeping.
        // Sequential waits can coalesce events. Neither imposes a deadline.
    }
}

#[cfg(example_l012)]
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    use hal::{
        gpio::Pull,
        halltim::{Config, HallPin, HallTim},
    };

    let p = hal::init(Default::default()).unwrap();
    // Connect push-pull Hall CH1/CH2/CH3 to PA3/PA4/PA5 (AF9) and common ground.
    // Construct the polling owner first: its own constructor masks all Hall
    // requests before BTIM3 enables the shared vector. No Hall handler is added.
    let mut hall = HallTim::new(
        p.HALLTIM,
        HallPin::new(p.PA3, Pull::None),
        HallPin::new(p.PA4, Pull::None),
        HallPin::new(p.PA5, Pull::None),
        Config {
            filter_length: 8,
            ..Default::default()
        },
    )
    .unwrap();
    let mut btim = Timer::<_, Async>::new_async(p.BTIM3, Irqs).unwrap();
    btim.set_period(Prescaler::Div256, 62_500).unwrap();
    hall.start();
    btim.start();

    loop {
        btim.wait_overflow().await;
        // Latest capture only: intervening Hall edges can be overwritten, and
        // live input levels are not sampled atomically with the captured width.
        if let Some(capture) = hall.try_capture() {
            core::hint::black_box(capture);
            cortex_m::asm::nop(); // Process the latest observed Hall sample.
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
