//! Test network with an echoing LIN transceiver. Every TX byte must appear on RX.
//! This example parks on any failure; supply a caller deadline in a deployed system.
//! Inter-byte gaps and response timing require board qualification.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    usart::{self, LinBreakThreshold, LinChecksum, LinIdentifier, LinResponse, LinUart},
};
hal::bind_interrupts!(struct Irqs {
    UART1 => usart::InterruptHandler<hal::peripherals::UART1>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let (tx_pin, rx_pin) = (p.PA1, p.PA0);
    #[cfg(any(
        feature = "cw32l011k8t6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    ))]
    let (tx_pin, rx_pin) = (p.PA2, p.PA3);
    let mut config = usart::Config::default();
    config.baudrate = 19_200;
    let mut uart = LinUart::new(p.UART1, tx_pin, rx_pin, Irqs, config, LinBreakThreshold::Bits11)
        .unwrap();
    // The test network must assign both responses exclusively to this node.
    let responses = [
        LinResponse::new(
            LinIdentifier::new(0x12).unwrap(), &[0x10, 0x20], LinChecksum::Enhanced,
        ).unwrap(),
        LinResponse::new(
            LinIdentifier::new(0x13).unwrap(), &[0x30, 0x40, 0x50], LinChecksum::Enhanced,
        ).unwrap(),
    ];
    loop {
        match uart.respond_once_with_echo(&responses).await {
            Ok(id) => { core::hint::black_box(id); }
            Err(error) => {
                core::hint::black_box(error);
                // Do not retry a possibly transmitted prefix. The application
                // must decide how to recover its network schedule.
                core::future::pending::<()>().await;
            }
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
