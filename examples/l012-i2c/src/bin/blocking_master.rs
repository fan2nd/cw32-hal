//! The same two L012 buses and register patterns through the blocking API.
//! Wire and qualify both separate buses before running this firmware.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cw32_l012_i2c_examples::{self as examples, REGISTER_POINTER, Readings, TARGET_ADDRESS};
use embassy_cw32::{
    self as hal,
    i2c::{Error, I2c, Operation},
    mode::Blocking,
};

fn read_register_patterns(i2c: &mut I2c<'_, Blocking>) -> Result<Readings, Error> {
    let mut readings = Readings {
        direct: [0; 2],
        combined: [0; 2],
        adjacent: [0; 2],
    };
    i2c.blocking_write(TARGET_ADDRESS, &REGISTER_POINTER)?;
    i2c.blocking_read(TARGET_ADDRESS, &mut readings.direct)?;
    i2c.blocking_write_read(TARGET_ADDRESS, &REGISTER_POINTER, &mut readings.combined)?;
    let (first, last) = readings.adjacent.split_at_mut(1);
    // Repeated START changes direction; adjacent reads use one receive command.
    i2c.blocking_transaction(
        TARGET_ADDRESS,
        &mut [
            Operation::Write(&REGISTER_POINTER),
            Operation::Read(first),
            Operation::Read(last),
        ],
    )?;
    Ok(readings)
}

#[entry]
fn main() -> ! {
    let p = hal::init(Default::default()).unwrap();
    let mut primary = I2c::new_blocking(p.I2C1, p.PA4, p.PA5, examples::config())
        .unwrap_or_else(|_| examples::halt());
    let mut secondary = I2c::new_blocking(p.I2C2, p.PB0, p.PB1, examples::config())
        .unwrap_or_else(|_| examples::halt());

    loop {
        let readings = read_register_patterns(&mut primary).unwrap_or_else(|_| examples::halt());
        core::hint::black_box(readings);
        let readings = read_register_patterns(&mut secondary).unwrap_or_else(|_| examples::halt());
        core::hint::black_box(readings);
    }
}
