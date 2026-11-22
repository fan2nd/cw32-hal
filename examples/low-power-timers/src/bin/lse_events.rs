#![no_std]
#![no_main]

#[cfg(not(example_lse_awt))]
compile_error!("lse_events requires an exact package qualified for owned LSE AWT");

use embassy_cw32::{
    self as hal,
    awt::{Awt, Config, Prescaler},
    rcc::{Lse, LseAmplitude, LseClock, LseDrive, LseMode, LseWait, OperatingConditions},
    time::Hertz,
};

hal::bind_interrupts!(struct Irqs { AWT => hal::awt::InterruptHandler<hal::peripherals::AWT>; });

// Illustrative declarations, not measured or qualified board facts. Replace the
// conditions and LSE settings below with the actual board's per-cycle/electrical
// qualification. Each part uses its own family authority, even where values agree.

// F020 own LSE limits: 1650..=5500 mV, -40..=105 C.
#[cfg(feature = "cw32f020c6u7")]
const BOARD: OperatingConditions = OperatingConditions {
    supply_mv: 3_300,
    min_temperature_c: -20,
    max_temperature_c: 70,
};
// F030 own LSE limits: 1650..=5500 mV, -40..=105 C.
#[cfg(feature = "cw32f030c8t7")]
const BOARD: OperatingConditions = OperatingConditions {
    supply_mv: 3_300,
    min_temperature_c: -20,
    max_temperature_c: 70,
};
// A030 own LSE limits: 1650..=5500 mV, -40..=105 C.
#[cfg(feature = "cw32a030c8t7")]
const BOARD: OperatingConditions = OperatingConditions {
    supply_mv: 3_300,
    min_temperature_c: -20,
    max_temperature_c: 70,
};
// L031 own LSE limits: 1650..=5500 mV, -40..=85 C.
#[cfg(feature = "cw32l031c8t6")]
const BOARD: OperatingConditions = OperatingConditions {
    supply_mv: 3_300,
    min_temperature_c: -20,
    max_temperature_c: 70,
};
// R031 own LSE limits: 2200..=3600 mV, -40..=85 C.
#[cfg(feature = "cw32r031c8u6")]
const BOARD: OperatingConditions = OperatingConditions {
    supply_mv: 3_300,
    min_temperature_c: -20,
    max_temperature_c: 70,
};

fn board_config() -> hal::Config {
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = BOARD;
    config.rcc.lse = Some(Lse {
        min_freq: Hertz(32766),
        max_freq: Hertz(32770),
        operating_conditions: BOARD,
        mode: LseMode::Oscillator,
        drive: LseDrive::Strong,
        amplitude: LseAmplitude::Normal,
        wait: LseWait::Cycles16384,
        // Attempts, not a guaranteed startup duration. Timeout retains LSE/pads.
        poll_budget: 20_000_000,
    });
    config
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let mut p = hal::init(board_config()).unwrap();
    let period = Config {
        prescaler: Prescaler::Div2,
        reload: 1023,
    };

    // Acquisition can temporarily open all of GPIOC to inspect PC14/PC15.
    // Retained bank filters/armed edges can advance, even if acquisition fails.
    let clock = LseClock::new(p.SYSCTRL.reborrow(), p.PC14.reborrow(), p.PC15.reborrow()).unwrap();
    let mut timer = Awt::new_lse(p.AWT.reborrow(), clock, period).unwrap();
    timer.start();
    let mut observed_event = false;
    // An iteration limit, not a time deadline or a promise of an elapsed event.
    // This phase terminates without relying on further LSE edges.
    for _ in 0..100_000 {
        let elapsed = timer.take_elapsed();
        let ready = timer.is_clock_ready();
        observed_event |= elapsed;
        core::hint::black_box((elapsed, ready, timer.counter()));
        if elapsed || !ready {
            break;
        }
        core::hint::spin_loop();
    }
    core::hint::black_box(observed_event);
    // Stops/masks AWT, but retains LSE, monitoring LSI and oscillator pads.
    drop(timer);

    // Reacquisition repeats the whole-GPIOC inspection window. It does not clear
    // source faults or recover a failed LSE; either constructor can still fail.
    let clock = LseClock::new(p.SYSCTRL.reborrow(), p.PC14.reborrow(), p.PC15.reborrow()).unwrap();
    let mut timer = Awt::new_async_lse(p.AWT.reborrow(), Irqs, clock, period).unwrap();
    timer.start();
    loop {
        // A ready snapshot does not guarantee another edge. Source loss produces
        // no wake: absent a retained event, wait can remain pending forever.
        // A hard deadline needs a separately valid, independently waking path.
        core::hint::black_box(timer.is_clock_ready());
        timer.wait().await;
        // Event retention and current readiness are separate observations.
        // An event can predate source loss; these are not wall-clock readings.
        core::hint::black_box(timer.is_clock_ready());
        cortex_m::asm::nop();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
