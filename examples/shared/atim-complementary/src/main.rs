//! Scope/logic-analyzer firmware: 1 kHz complementary output, at least 500 ns dead time.
//! L011/L012: external active-high BK1 on PA0. L010: software break only.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config},
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};
#[cfg(not(feature = "cw32l010f8p6"))]
use hal::{
    gpio::Pull,
    timer::complementary_pwm::{BreakInput, BreakInputPolarity},
};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    #[cfg(feature = "cw32l010f8p6")]
    let (main, complementary, break_input) =
        (PwmPin::new(p.PB4), ComplementaryPwmPin::new(p.PA4), None);
    #[cfg(not(feature = "cw32l010f8p6"))]
    let (main, complementary, break_input) = (
        PwmPin::new(p.PA5),
        ComplementaryPwmPin::new(p.PA7),
        Some(BreakInput::new(
            p.PA0,
            BreakInputPolarity::ActiveHigh,
            Pull::None,
        )),
    );
    let mut pwm = ComplementaryPwm::new(
        p.ATIM,
        Some(main),
        Some(complementary),
        None,
        None,
        None,
        None,
        None,
        None,
        break_input,
        Hertz(1_000),
        CountingMode::EdgeAlignedUp,
        Config { dead_time_ns: 500 },
    )
    .unwrap();
    pwm.set_duty(Channel::Ch1, pwm.get_max_duty() / 4);
    pwm.enable(Channel::Ch1);
    pwm.set_master_output_enable(true);
    core::hint::black_box((pwm.frequency_bounds(), pwm.dead_time_bounds_ns()));
    loop {
        // An external BK1 event clears MOE in hardware. This application leaves
        // outputs disabled; it deliberately has no automatic restart policy.
        if pwm.is_break_pending() {
            pwm.set_master_output_enable(false);
            core::hint::black_box(pwm.get_master_output_enable());
        }
        core::hint::spin_loop();
    }
}
