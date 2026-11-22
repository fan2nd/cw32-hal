//! One encoded LIN transmission; timing and network scheduling remain unqualified.
//! Connect TX to a LIN transceiver configured for the actual board and network.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    usart::{self, LinBreakBits, LinChecksum, LinIdentifier, LinResponse, LinTransmitter},
};

hal::bind_interrupts!(struct Irqs {
    UART1 => usart::InterruptHandler<hal::peripherals::UART1>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let pin = p.PA1;
    #[cfg(any(
        feature = "cw32l011k8t6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    ))]
    let pin = p.PA2;
    let mut config = usart::Config::default();
    config.baudrate = 19_200;
    let mut tx = LinTransmitter::new(p.UART1, pin, Irqs, config).unwrap();
    // Configure a test network that assigns ID 0x12 to this publisher, with
    // two payload bytes and enhanced checksum. This is not device discovery.
    let response = LinResponse::new(
        LinIdentifier::new(0x12).unwrap(),
        &[0x10, 0x20],
        LinChecksum::Enhanced,
    )
    .unwrap();
    tx.send_break(LinBreakBits::default()).await.unwrap();
    // Explicit full idle frame, not a claim of a minimal LIN delimiter.
    tx.send_idle().await.unwrap();
    tx.write_sync_identifier(response.identifier()).await.unwrap();
    tx.write_response(&response).await.unwrap();
    tx.flush().await.unwrap();
    #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    {
        // A separate transceiver is required for this second bus.
        let mut second = LinTransmitter::new_blocking(p.UART2, p.PA7, config).unwrap();
        second.blocking_send_break(LinBreakBits::default()).unwrap();
        second.blocking_send_idle().unwrap();
        second
            .blocking_write_sync_identifier(response.identifier())
            .unwrap();
        second.blocking_write_response(&response).unwrap();
        second.blocking_flush().unwrap();
    }
    // No retries or repeated transmissions; this does not establish bus timing.
    core::future::pending::<()>().await;
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
