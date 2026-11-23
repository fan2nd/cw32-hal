#![no_std]
#![no_main]

#[cfg(any(
    feature = "cw32f020c6u7",
    feature = "cw32f030c8t7",
    feature = "cw32a030c8t7",
    feature = "cw32l031c8t6",
    feature = "cw32l031c8u6",
    feature = "cw32l031f8u6",
    feature = "cw32r031c8u6",
    feature = "cw32l052c8t6",
    feature = "cw32l052r8s6",
    feature = "cw32l052r8t6",
    feature = "cw32l083rbt6",
    feature = "cw32l083rct6",
    feature = "cw32l083rcs6",
    feature = "cw32l083mct6",
    feature = "cw32l083vct6"
))]
use embassy_cw32::rtc::{DateTime, DayOfWeek, Rtc};
use embassy_cw32::{self as hal, rcc};

#[cortex_m_rt::entry]
fn main() -> ! {
    // This example assumes a 3.3 V board.
    let mut config = hal::Config::default();
    config.rcc.operating_conditions = rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -40,
        max_temperature_c: if cfg!(any(
            feature = "cw32l010f8p6",
            feature = "cw32l010f8u6",
            feature = "cw32l010y8m6",
            feature = "cw32l011k8t6",
            feature = "cw32l011k8u6",
            feature = "cw32l012c8t6",
            feature = "cw32l012c8u6",
            feature = "cw32l031c8t6",
            feature = "cw32l031c8u6",
            feature = "cw32l031f8u6",
            feature = "cw32r031c8u6",
            feature = "cw32l052c8t6",
            feature = "cw32l052r8s6",
            feature = "cw32l052r8t6",
            feature = "cw32l083rbt6",
            feature = "cw32l083rct6",
            feature = "cw32l083rcs6",
            feature = "cw32l083mct6",
            feature = "cw32l083vct6"
        )) {
            85
        } else {
            105
        },
    };
    config.rcc.sys = rcc::Sysclk::LSI;
    #[cfg(any(
        feature = "cw32l010f8p6",
        feature = "cw32l010f8u6",
        feature = "cw32l010y8m6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    ))]
    {
        // This does not prepare LSI. Ordinary POR is expected to return an error.
        // Only an already software-requested, stable, factory-matching source
        // qualifies; its existing TRIM and WAIT remain unchanged.
        config.rcc.lse = None;
    }
    #[cfg(any(feature = "cw32l011k8t6", feature = "cw32l011k8u6"))]
    {
        // This requires an already live factory-matching LSI; POR returns an error.
        // Inherited HSE and both CCS monitors must also be disabled.
        // No HSE, LSE or watchdog preparation is performed by this example.
        config.rcc.hse = None;
        config.rcc.lse = None;
    }
    let p = hal::init(config).unwrap_or_else(|error| stop(error));
    run(p)
}

#[cfg(any(
    feature = "cw32f002f3p7",
    feature = "cw32f002f3u7",
    feature = "cw32f003f4p7",
    feature = "cw32f003f4u7",
    feature = "cw32f003e4p7"
))]
fn run(_p: hal::Peripherals) -> ! {
    // F002/F003 have no RTC. Successful init exposes factory LSI rate bounds for
    // SYSCLK and both buses, while HSI keeps its independent qualification.
    let clocks = rcc::clocks();
    loop {
        core::hint::black_box((
            clocks.sys_bounds(),
            clocks.hclk_bounds(),
            clocks.pclk_bounds(),
            clocks.hsi_bounds(),
        ));
    }
}

#[cfg(any(
    feature = "cw32f020c6u7",
    feature = "cw32f030c8t7",
    feature = "cw32a030c8t7",
    feature = "cw32l031c8t6",
    feature = "cw32l031c8u6",
    feature = "cw32l031f8u6",
    feature = "cw32r031c8u6",
    feature = "cw32l052c8t6",
    feature = "cw32l052r8s6",
    feature = "cw32l052r8t6",
    feature = "cw32l083rbt6",
    feature = "cw32l083rct6",
    feature = "cw32l083rcs6",
    feature = "cw32l083mct6",
    feature = "cw32l083vct6"
))]
fn run(p: hal::Peripherals) -> ! {
    let lsi = rcc::LsiClock::new(p.SYSCTRL, 100_000).unwrap_or_else(|error| stop(error));
    core::hint::black_box((rcc::clocks().sys_bounds(), lsi.bounds()));
    let clock = rcc::CalendarClock::Lsi(lsi);
    core::hint::black_box(clock.bounds());
    // An unset calendar is provisioned with an explicit demonstration epoch.
    // Replace it with the intended time before using this as a clock product.
    // An already-running compatible calendar retains its existing date/time.
    let epoch = DateTime::from(2026, 10, 9, DayOfWeek::Friday, 0, 0, 0, 0).unwrap();
    let rtc = Rtc::initialize_if_unset(p.RTC, clock, Default::default(), epoch)
        .unwrap_or_else(|error| stop(error));
    loop {
        // Bounds remain rate-only through the calendar's exact 32768 divisor.
        // No strict minimum/maximum-duration helper or 1 MHz time driver is used.
        core::hint::black_box((
            rtc.now().unwrap_or_else(|error| stop(error)),
            rtc.source_clock_bounds(),
            rtc.calendar_tick_bounds(),
        ));
    }
}

#[cfg(any(
    feature = "cw32l010f8p6",
    feature = "cw32l010f8u6",
    feature = "cw32l010y8m6"
))]
fn run(p: hal::Peripherals) -> ! {
    // HSIOSC keeps its independent strict 48 MHz source envelope. Acquiring
    // this existing capability only reads readiness; it does not alter RTC.
    let hsi = rcc::HsiOscClock::new(p.SYSCTRL).unwrap_or_else(|error| stop(error));
    let clocks = rcc::clocks();
    loop {
        core::hint::black_box((
            clocks.sys_bounds(),
            clocks.hclk_bounds(),
            clocks.pclk_bounds(),
            clocks.hsi_bounds(),
            hsi.bounds(),
        ));
    }
}

#[cfg(any(
    feature = "cw32l011k8t6",
    feature = "cw32l011k8u6",
    feature = "cw32l012c8t6",
    feature = "cw32l012c8u6"
))]
fn run(p: hal::Peripherals) -> ! {
    // This observes the existing raw 96 MHz HSIOSC capability. Its strict
    // bounds stay independent of rate-only LSI SYSCLK and the HSI divider.
    // No LSI preparation, watchdog operation or RTC initialization is hidden here.
    let hsi = rcc::HsiOscClock::new(p.SYSCTRL).unwrap_or_else(|error| stop(error));
    let clocks = rcc::clocks();
    loop {
        core::hint::black_box((
            clocks.sys_bounds(),
            clocks.hclk_bounds(),
            clocks.pclk_bounds(),
            clocks.hsi_bounds(),
            hsi.bounds(),
        ));
    }
}

fn stop<E>(error: E) -> ! {
    core::hint::black_box(error);
    loop {
        cortex_m::asm::bkpt();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
