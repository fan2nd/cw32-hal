#![no_std]
#![no_main]

use cw32f030_motor_tb6612::tb6612::{self, Brake};

#[cortex_m_rt::entry]
fn main() -> ! {
    tb6612::run(Brake::ExternalGate)
}
