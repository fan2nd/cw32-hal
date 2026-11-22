//! Echoing transceiver and exclusive test-network publishing assignment required.
//! Application deadlines are demonstration values, not LIN timing qualification.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    usart::{self, LinBreakThreshold, LinChecksum, LinIdentifier, LinResponse, LinUart},
};
use embassy_time::{Duration, with_timeout};

hal::bind_interrupts!(struct Irqs {
    UART1 => usart::InterruptHandler<hal::peripherals::UART1>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // Preserve the default time-driver-qualified HSI/PCLK configuration.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 3_300;
    let p = hal::init(system).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let (tx_pin, rx_pin) = (p.PA1, p.PA0);
    #[cfg(any(feature = "cw32l011k8t6", feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    let (tx_pin, rx_pin) = (p.PA2, p.PA3);
    let mut config = usart::Config::default();
    config.baudrate = 19_200;
    let mut uart = LinUart::new(p.UART1, tx_pin, rx_pin, Irqs, config, LinBreakThreshold::Bits11)
        .unwrap();
    let responses = [LinResponse::new(
        LinIdentifier::new(0x12).unwrap(), &[0x10, 0x20], LinChecksum::Enhanced,
    ).unwrap()];

    // One deadline covers header selection, all echoes and final UART flush.
    // It is intentionally not restarted on every byte or unrelated identifier.
    let attempt = with_timeout(
        Duration::from_millis(100),
        uart.respond_once_with_echo(&responses),
    ).await;
    match attempt {
        Ok(Ok(id)) => { core::hint::black_box(id); }
        Ok(Err(error)) => { core::hint::black_box(error); }
        Err(_) => { core::hint::black_box("response attempt deadline elapsed"); }
    }

    // A cancelled write can leave a queued byte. Bound the wait for its UART
    // completion too; do not resend a possibly transmitted response prefix.
    let retirement = with_timeout(
        Duration::from_millis(10), uart.transmitter().flush(),
    ).await;
    let _ = core::hint::black_box(retirement);
    // The application must choose network recovery. Keep the owner and park;
    // do not infer peer acknowledgment or a safe new schedule from UART idle.
    core::future::pending::<()>().await;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}
