//! The same board register reads using the ordinary blocking API.
//! External pull-ups and target address/register assumptions are in README.md.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cw32_classic_i2c_examples::{self as examples, REGISTER_POINTER, Readings, TARGET_ADDRESS};
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
    // The direction change uses repeated START; adjacent reads remain one run.
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
    #[cfg(not(feature = "cw32f002f3p7"))]
    let p = hal::init(Default::default()).unwrap();
    #[cfg(feature = "cw32f002f3p7")]
    let p = hal::init(Default::default()).unwrap_or_else(|_| examples::halt());
    let (peripheral, scl, sda) = examples::primary_pins!(p);
    let mut primary = I2c::new_blocking(peripheral, scl, sda, examples::config())
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
        I2c::new_blocking(peripheral, scl, sda, examples::config())
            .unwrap_or_else(|_| examples::halt())
    };

    loop {
        let readings = read_register_patterns(&mut primary).unwrap_or_else(|_| examples::halt());
        core::hint::black_box(readings);
        #[cfg(any(
            feature = "cw32f030c8t7",
            feature = "cw32a030c8t7",
            feature = "cw32f020c6u7",
            feature = "cw32l052c8t6",
            feature = "cw32l083rct6"
        ))]
        {
            let readings =
                read_register_patterns(&mut secondary).unwrap_or_else(|_| examples::halt());
            core::hint::black_box(readings);
        }
    }
}
