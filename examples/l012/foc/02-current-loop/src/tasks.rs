//! Independent Embassy tasks for the FOC example.
use embassy_cw32::gpio::Output;
use embassy_time::{Duration, Ticker};

#[embassy_executor::task]
pub async fn led(mut led: Output<'static>) {
    let mut ticker = Ticker::every(Duration::from_millis(250));
    loop {
        ticker.next().await;
        led.toggle();
    }
}
