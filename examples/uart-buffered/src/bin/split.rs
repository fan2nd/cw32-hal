#![no_std]
#![no_main]

use cw32_uart_buffered_example as examples;
use embassy_cw32::{
    self as hal,
    mode::Async,
    usart::{BufferedUartRx, UartTx},
};
use embassy_futures::join::join;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let ring =
        cortex_m::singleton!(: [u8; examples::RING_SIZE] = [0; examples::RING_SIZE]).unwrap();
    let uart = examples::uart(p, ring);
    #[cfg(feature = "cw32l012c8t6")]
    {
        let mut uart = uart;
        let (tx, rx) = uart.split_ref();
        exchange(tx, rx).await;
    }
    #[cfg(not(feature = "cw32l012c8t6"))]
    {
        let (mut tx, mut rx) = uart.split();
        exchange(&mut tx, &mut rx).await;
    }
}

async fn exchange(tx: &mut UartTx<'_, Async>, rx: &mut BufferedUartRx<'_>) -> ! {
    loop {
        // RX lends only an occupied segment; IRQs may fill other free slots.
        let (received, sent) = join(embedded_io_async::BufRead::fill_buf(rx), async {
            tx.write(b"send data\r\n").await?;
            tx.flush().await
        })
        .await;
        sent.unwrap();
        match received {
            Ok(prefix) => {
                let count = prefix.len();
                // The borrowed RX prefix stays valid across the separate TX wait.
                tx.write(prefix).await.unwrap();
                embedded_io_async::BufRead::consume(rx, count);
            }
            Err(error) => {
                core::hint::black_box(error);
                tx.write(b"receive error\r\n").await.unwrap();
            }
        }
        tx.flush().await.unwrap();
    }
}
