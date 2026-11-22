//! Ordinary interrupt-driven reads from each wired board target.
//! External pull-ups and target address/register assumptions are in README.md.
#![no_std]
#![no_main]

use cw32_classic_i2c_examples::{self as examples, REGISTER_POINTER, Readings, TARGET_ADDRESS};
use embassy_cw32::{
    self as hal,
    i2c::{Error, I2c, InterruptHandler, Operation},
    mode::Async,
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

async fn read_register_patterns(i2c: &mut I2c<'_, Async>) -> Result<Readings, Error> {
    let mut readings = Readings {
        direct: [0; 2],
        combined: [0; 2],
        adjacent: [0; 2],
    };
    // Separate transactions: select the register, STOP, then read two bytes.
    i2c.write(TARGET_ADDRESS, &REGISTER_POINTER).await?;
    i2c.read(TARGET_ADDRESS, &mut readings.direct).await?;
    // One transaction: pointer write, repeated START, two-byte read, STOP.
    i2c.write_read(TARGET_ADDRESS, &REGISTER_POINTER, &mut readings.combined)
        .await?;
    let (first, last) = readings.adjacent.split_at_mut(1);
    // Adjacent reads share one read address phase. Only the second byte is
    // NACKed; the preceding write-to-read direction change uses repeated START.
    i2c.transaction(
        TARGET_ADDRESS,
        &mut [
            Operation::Write(&REGISTER_POINTER),
            Operation::Read(first),
            Operation::Read(last),
        ],
    )
    .await?;
    Ok(readings)
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    #[cfg(not(feature = "cw32f002f3p7"))]
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32f002f3p7")]
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let (peripheral, scl, sda) = examples::primary_pins!(p);
    let mut primary = I2c::new_async(peripheral, scl, sda, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    #[cfg(any(
        feature = "cw32f030c8t7",
        feature = "cw32a030c8t7",
        feature = "cw32f020c6u7",
        feature = "cw32l052c8t6",
        feature = "cw32l083rct6"
    ))]
    let mut secondary = {
        let (peripheral, scl, sda) = examples::secondary_pins!(p);
        I2c::new_async(peripheral, scl, sda, Irqs, examples::config())
            .unwrap_or_else(|_| examples::halt())
    };

    loop {
        let readings = read_register_patterns(&mut primary)
            .await
            .unwrap_or_else(|_| examples::halt());
        core::hint::black_box(readings);
        #[cfg(any(
            feature = "cw32f030c8t7",
            feature = "cw32a030c8t7",
            feature = "cw32f020c6u7",
            feature = "cw32l052c8t6",
            feature = "cw32l083rct6"
        ))]
        {
            let readings = read_register_patterns(&mut secondary)
                .await
                .unwrap_or_else(|_| examples::halt());
            core::hint::black_box(readings);
        }
    }
}
