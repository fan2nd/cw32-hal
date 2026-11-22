#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal, bind_interrupts,
    gpio::Pull,
    halltim::{self, Config, HallPin, HallTim, Prescaler},
    peripherals,
};
bind_interrupts!(struct Irqs {
    BTIM3_HALLTIM => halltim::InterruptHandler<peripherals::HALLTIM>,
        hal::timer::OverflowInterruptHandler<peripherals::BTIM3>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    // Connect three push-pull Hall sensor outputs referenced to board ground.
    // PA3/PA4/PA5 are CH1/CH2/CH3 AF9 on both qualified 48-pin packages.
    let mut hall = HallTim::new(
        p.HALLTIM,
        HallPin::new(p.PA3, Pull::None),
        HallPin::new(p.PA4, Pull::None),
        HallPin::new(p.PA5, Pull::None),
        Config {
            prescaler: Prescaler::Div8,
            filter_length: 8,
            ..Default::default()
        },
    )
    .unwrap();
    let clock = hall.tick_clock();
    hall.restart();
    hall.start_history(Irqs).unwrap();
    loop {
        match hall.next_history().await {
            Ok(capture) => {
                // Oldest retained software observation, not a physical edge log.
                core::hint::black_box((capture, clock));
                if capture.overflow_since_restart {
                    hall.restart(); // Discard queue and ambiguous elapsed intervals.
                    hall.start_history(Irqs).unwrap();
                }
            }
            Err(halltim::Error::HistoryOverflow) => {
                // Queue prefix has been drained; collection can resume.
                // The next transition is FirstSample rather than spanning the gap.
                core::hint::black_box("software observations lost");
            }
            Err(_) => {
                hall.stop();
                loop {
                    cortex_m::asm::bkpt();
                }
            }
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
