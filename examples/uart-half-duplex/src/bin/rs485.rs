#![no_std]
#![no_main]

use cw32_uart_half_duplex_example as examples;
use embassy_cw32::{self as hal, usart};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let (peri, tx, rx, de) = (p.UART2, p.PA3, p.PA4, p.PB1);
    #[cfg(feature = "cw32l011k8t6")]
    let (peri, tx, rx, de) = (p.UART3, p.PA4, p.PA5, p.PB4);
    #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    let (peri, tx, rx, de) = (p.UART1, p.PA2, p.PA3, p.PA1);
    let mut uart = usart::Rs485Uart::new(
        peri, tx, rx, de, examples::Irqs, examples::config(),
        usart::DriverEnable {
            polarity: usart::DriverEnablePolarity::ActiveHigh,
            delay_samples: 16,
        },
    ).unwrap();
    // Send once. External transceiver/termination are required. DE timing must
    // be selected for that transceiver; flush does not prove external bus release.
    uart.write(b"CW32\r\n").await.unwrap();
    uart.flush().await.unwrap();
    // Retain the owner and its automatic DE service without repeatedly sending.
    core::future::pending::<()>().await;
}
