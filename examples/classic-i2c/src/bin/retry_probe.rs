//! Explicit bounded address-probe replay; target must permit repeated probes.
//! External pull-ups and target address/register assumptions are in README.md.
#![no_std]
#![no_main]

use cw32_classic_i2c_examples::{self as examples, TARGET_ADDRESS};
use embassy_cw32::{
    self as hal,
    i2c::{I2c, InterruptHandler, Operation, RetryOn, RetryPolicy},
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
    let mut primary = I2c::new_async(peripheral, scl, sda, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let result = primary.transaction_with_retries(
        TARGET_ADDRESS,
        &mut [Operation::Write(&[])],
        RetryPolicy { retries: 2, on: RetryOn::AddressNack },
    ).await;
    core::hint::black_box(result); // At most three attempts; no retry on BusBusy.
    core::future::pending::<()>().await;
}
