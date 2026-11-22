//! Ten-bit address-mode example, not an assumed attached device protocol.
//! Configure compatible targets on both buses before running. No hardware run.
#![no_std]
#![no_main]
use cw32_l012_i2c_examples as examples;
use embassy_cw32::{
    self as hal,
    i2c::{Address, I2c, InterruptHandler, Operation},
};
const TARGET: u16 = 0x2a5;
hal::bind_interrupts!(struct Irqs {
    I2C1 => InterruptHandler<hal::peripherals::I2C1>;
});
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut asynchronous = I2c::new_async(p.I2C1, p.PA4, p.PA5, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let mut blocking = I2c::new_blocking(p.I2C2, p.PB0, p.PB1, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let mut first = [0u8; 2];
    let mut second = [0u8; 2];
    loop {
        // A pure read must select both address bytes in write direction first.
        embedded_hal_async::i2c::I2c::<embedded_hal::i2c::TenBitAddress>::read(
            &mut asynchronous,
            TARGET,
            &mut first,
        )
        .await
        .unwrap_or_else(|_| examples::halt());
        embedded_hal::i2c::I2c::<embedded_hal::i2c::TenBitAddress>::read(
            &mut blocking,
            TARGET,
            &mut second,
        )
        .unwrap_or_else(|_| examples::halt());
        // Explicit typed-mode interfaces: empty writes are address probes.
        asynchronous
            .transaction_with_address(Address::TenBit(TARGET), &mut [Operation::Write(&[])])
            .await
            .unwrap_or_else(|_| examples::halt());
        blocking
            .blocking_transaction_with_address(
                Address::TenBit(TARGET),
                &mut [Operation::Write(&[])],
            )
            .unwrap_or_else(|_| examples::halt());
        core::hint::black_box((first, second));
    }
}
