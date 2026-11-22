#![no_std]
#![no_main]

use cw32_uart_idle_example as examples;
use embassy_cw32 as hal;
use embassy_futures::join::join;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let uart = examples::uart(p);
    let (mut tx, mut rx) = uart.split();
    loop {
        let mut response = [0u8; examples::BUFFER_SIZE];
        // Poll RX first so it can await a response while the request is sent.
        let (received, sent) = join(rx.read_until_idle(&mut response), async {
            tx.write(b"read?\r\n").await?;
            tx.flush().await
        })
        .await;
        sent.unwrap();
        match received {
            Ok(count) => {
                tx.write(&examples::count_line(count)).await.unwrap();
                tx.write(&response[..count]).await.unwrap();
                tx.write(b"\r\n").await.unwrap();
            }
            Err(error) => {
                core::hint::black_box(error);
                tx.write(b"receive error\r\n").await.unwrap();
            }
        }
        tx.flush().await.unwrap();
    }
}
