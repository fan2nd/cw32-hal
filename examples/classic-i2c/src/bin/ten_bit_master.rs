//! One ten-bit write/read transaction on an explicitly configured test target.
//! Address 0x2a5 and register 0 are illustrative; configure the target beforehand.
#![no_std]
#![no_main]

use cw32_classic_i2c_examples as examples;
use embassy_cw32::{
    self as hal,
    i2c::{Address, I2c, InterruptHandler, Operation},
};

#[cfg(any(
    feature = "cw32f030c8t7",
    feature = "cw32a030c8t7",
    feature = "cw32f020c6u7",
    feature = "cw32l052c8t6",
    feature = "cw32l083rct6"
))]
hal::bind_interrupts!(struct Irqs {
    I2C1 => InterruptHandler<hal::peripherals::I2C1>;
    I2C2 => InterruptHandler<hal::peripherals::I2C2>;
});

#[cfg(any(
    feature = "cw32l010f8p6",
    feature = "cw32l031c8t6",
    feature = "cw32r031c8u6",
))]
hal::bind_interrupts!(struct Irqs {
    I2C1 => InterruptHandler<hal::peripherals::I2C1>;
});

#[cfg(any(
    feature = "cw32f002f3p7",
    feature = "cw32f003e4p7",
    feature = "cw32l011k8t6"
))]
hal::bind_interrupts!(struct Irqs {
    I2C => InterruptHandler<hal::peripherals::I2C>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    #[cfg(not(feature = "cw32f002f3p7"))]
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32f002f3p7")]
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let (peripheral, scl, sda) = examples::primary_pins!(p);
    let mut bus = I2c::new_async(peripheral, scl, sda, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let mut bytes = [0; 2];
    let result = bus.transaction_with_address(
        Address::TenBit(0x2a5),
        &mut [Operation::Write(&[0]), Operation::Read(&mut bytes)],
    ).await;
    let _ = core::hint::black_box((result, bytes));
    core::future::pending::<()>().await;
}
