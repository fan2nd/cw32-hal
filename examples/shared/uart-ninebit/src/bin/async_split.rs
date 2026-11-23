#![no_std]
#![no_main]

use cw32_uart_ninebit_example as examples;
use embassy_cw32::{self as hal, usart};
use embassy_futures::join::join;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let (peri, tx, rx) = (p.UART2, p.PA3, p.PA4);
    #[cfg(feature = "cw32l011k8t6")]
    let (peri, tx, rx) = (p.UART3, p.PA4, p.PA5);
    #[cfg(any(feature = "cw32l012c8t6", feature = "cw32l012c8u6"))]
    let (peri, tx, rx) = (p.UART1, p.PA2, p.PA3);
    let uart = usart::Uart::new(peri, tx, rx, examples::Irqs, examples::config()).unwrap();
    let (mut tx, mut rx) = uart.split();
    // Physical loopback: connect the selected TX -> RX. Poll RX and TX together.
    loop {
        let sent = [0x000u16, 0x100, 0x1ff, 0x055];
        let mut received = [0u16; 4];
        let (read, write) = join(rx.read_u16(&mut received), async {
            tx.write_u16(&sent).await?;
            tx.flush().await
        })
        .await;
        read.unwrap();
        write.unwrap();
        core::hint::black_box(received);
    }
}
