//! Ordinary interrupt-driven register reads on both L012 I2C controllers.
//! Wire and qualify both separate buses before running this firmware.
#![no_std]
#![no_main]

use cw32_l012_i2c_examples::{self as examples, REGISTER_POINTER, Readings, TARGET_ADDRESS};
use embassy_cw32::{
    self as hal,
    i2c::{Error, I2c, InterruptHandler, Operation},
    mode::Async,
};

hal::bind_interrupts!(struct Irqs {
    I2C1 => InterruptHandler<hal::peripherals::I2C1>;
    I2C2 => InterruptHandler<hal::peripherals::I2C2>;
});

async fn read_register_patterns(i2c: &mut I2c<'_, Async>) -> Result<Readings, Error> {
    let mut readings = Readings {
        direct: [0; 2],
        combined: [0; 2],
        adjacent: [0; 2],
    };
    // Two separate transactions: pointer write, STOP, then two-byte read, STOP.
    i2c.write(TARGET_ADDRESS, &REGISTER_POINTER).await?;
    i2c.read(TARGET_ADDRESS, &mut readings.direct).await?;
    // One transaction with a write-to-read repeated START and one final STOP.
    i2c.write_read(TARGET_ADDRESS, &REGISTER_POINTER, &mut readings.combined)
        .await?;
    let (first, last) = readings.adjacent.split_at_mut(1);
    // The real async trait receives adjacent buffers as one read run. One
    // receive command spans both bytes; only the final byte is NACKed.
    embedded_hal_async::i2c::I2c::transaction(
        i2c,
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
    let p = hal::init(Default::default()).unwrap();
    // Both exact packages bond PA4/PA5 (AF2) and PB0/PB1 (AF8).
    let mut primary = I2c::new_async(p.I2C1, p.PA4, p.PA5, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let mut secondary = I2c::new_async(p.I2C2, p.PB0, p.PB1, Irqs, examples::config())
        .unwrap_or_else(|_| examples::halt());

    loop {
        let readings = read_register_patterns(&mut primary)
            .await
            .unwrap_or_else(|_| examples::halt());
        core::hint::black_box(readings);
        let readings = read_register_patterns(&mut secondary)
            .await
            .unwrap_or_else(|_| examples::halt());
        core::hint::black_box(readings);
    }
}
