//! Fixed-baud receive-only observations through an external LIN transceiver.
#![no_std]
#![no_main]
use embassy_cw32::{
    self as hal,
    usart::{
        self, LinBreakThreshold, LinChecksum, LinFrameDefinition, LinFrameDispatcher,
        LinIdentifier, LinReceiver,
    },
};

hal::bind_interrupts!(struct Irqs {
    UART1 => usart::InterruptHandler<hal::peripherals::UART1>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let pin = p.PA0;
    #[cfg(any(
        feature = "cw32l011k8t6",
        feature = "cw32l012c8t6",
        feature = "cw32l012c8u6"
    ))]
    let pin = p.PA3;
    let mut config = usart::Config::default();
    config.baudrate = 19_200;
    let mut rx = LinReceiver::new(p.UART1, pin, Irqs, config, LinBreakThreshold::Bits11)
        .unwrap();
    // ID 0x12 matches the transmit example. This monitor also accepts ID 0x13
    // only on a network that defines it with this eight-byte enhanced layout.
    let definitions = [
        LinFrameDefinition::new(LinIdentifier::new(0x12).unwrap(), 2, LinChecksum::Enhanced)
            .unwrap(),
        LinFrameDefinition::new(LinIdentifier::new(0x13).unwrap(), 8, LinChecksum::Enhanced)
            .unwrap(),
    ];
    let mut frames = LinFrameDispatcher::new(&definitions).unwrap();
    loop {
        match rx.read_event().await {
            Ok(event) => {
                let _ = core::hint::black_box(frames.push(event));
            }
            Err(_) => frames.reset(),
        }
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
