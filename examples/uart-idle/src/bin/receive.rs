#![no_std]
#![no_main]

use cw32_uart_idle_example as examples;
use embassy_cw32 as hal;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut uart = examples::uart(p);
    loop {
        let mut buffer = [0u8; examples::BUFFER_SIZE];
        match uart.read_until_idle(&mut buffer).await {
            Ok(count) => {
                uart.write(&examples::count_line(count)).await.unwrap();
                uart.write(&buffer[..count]).await.unwrap();
                uart.write(b"\r\n").await.unwrap();
            }
            Err(error) => {
                // An error has no successful count; discard this partial buffer.
                core::hint::black_box(error);
                uart.write(b"receive error\r\n").await.unwrap();
            }
        }
        uart.flush().await.unwrap();
    }
}
