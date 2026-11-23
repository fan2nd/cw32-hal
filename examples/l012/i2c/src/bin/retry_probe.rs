//! Requires a target that permits repeated ten-bit address probes.
//! L012 cannot distinguish address NACK from data NACK. Not hardware verified.
#![no_std]
#![no_main]
use cw32_l012_i2c_examples as examples;
use embassy_cw32::{
    self as hal,
    i2c::{Address, I2c, InterruptHandler, Operation, RetryOn, RetryPolicy},
};
hal::bind_interrupts!(struct Irqs {
    I2C1 => InterruptHandler<hal::peripherals::I2C1>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut bus = I2c::new_async(p.I2C1, p.PA4, p.PA5, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let result = bus.transaction_with_address_and_retries(
        Address::TenBit(0x2a5),
        &mut [Operation::Write(&[])],
        RetryPolicy { retries: 2, on: RetryOn::UnclassifiedNack },
    ).await;
    core::hint::black_box(result);
    core::future::pending::<()>().await;
}
