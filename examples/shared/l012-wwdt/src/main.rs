#![no_std]
#![no_main]
//! Notification-only demonstration: intentionally does not feed, so WWDT resets.
use embassy_cw32::{
    self as hal, bind_interrupts,
    wdg::{WindowConfig, WindowPrescaler, WindowWatchdog},
};

bind_interrupts!(struct Irqs { WDT => hal::wdg::WindowInterruptHandler; });
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let config = WindowConfig::from_counts(WindowPrescaler::Div524288, 0x7f, 0x60).unwrap();
    let mut watchdog = WindowWatchdog::new(p.WWDT, config).unwrap();
    watchdog.try_unleash().unwrap();
    watchdog.enable_early_warning(Irqs).unwrap();
    // Only one counter tick remains at the event. Delivery here is not guaranteed.
    watchdog.wait_early_warning().await.unwrap();
    let _ = core::hint::black_box(watchdog.try_status());
    core::future::pending::<()>().await;
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::nop();
    }
}
