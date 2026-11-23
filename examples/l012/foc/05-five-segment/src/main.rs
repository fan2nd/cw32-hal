#![no_std]
#![no_main]

pub mod board_resources {
    include!("../../common/board_resources.rs");
}
pub mod adc;
pub mod config;
pub mod current_loop;
pub mod global;
pub mod hardware;
pub mod param;
pub mod sensor;
pub mod timing;
pub use cw32_l012_foc_algorithms as foc;
pub use foc::{calibration, control};
pub mod speed;
pub mod telemetry;

use crate::{
    board_resources::BoardResources, calibration::Calibration, global::*, hardware::*,
    sensor::connect, timing::Timing,
};
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::{ExceptionFrame, exception};
use defmt_rtt as _;
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config as AdcConfig},
    gpio::{Flex, Level, Output, Speed},
    opamp::OpAmpAdc,
    peripherals::{ADC1, ADC2, ATIM, OPA1, OPA2},
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig, irq_moe},
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};

static BRIDGE_CONFIGURED: AtomicBool = AtomicBool::new(false);

pub type Bridge = ComplementaryPwm<'static, ATIM>;
defmt::timestamp!("{=u32}", 0u32);

/// Held by the outer interrupt for its entire lifetime.
pub(crate) struct Analog {
    _adc1: Adc<'static, ADC1>,
    _adc2: Adc<'static, ADC2>,
    _opa1: OpAmpAdc<'static, OPA1>,
    _opa2: OpAmpAdc<'static, OPA2>,
    _bus: Flex<'static>,
    _pot: Flex<'static>,
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // The board supplies the MCU/VDDA from 5 V. The motor-control application
    // deliberately opts into the calibrated nominal 96 MHz HSI bus path.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 5_000;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div1;
    system.rcc.allow_nominal_bus_overclock = true;
    let p = match hal::init_hsi(system) {
        Ok(p) => p,
        Err(error) => {
            defmt::error!("entry: RCC init failed: {:?}", error);
            halt()
        }
    };
    let board = BoardResources::from(p);
    // The reference board LED is active-low on PC13, matching LEDON/LEDOFF
    // in the C project: waiting blinks, alignment is solid, and run heartbeats.
    let led = Output::new(board.status.led, Level::High, Speed::Default);
    let mut timing = Timing::new(led, hal::rcc::clocks().hclk.0);
    defmt::info!("five-segment SVPWM: PWM={=u32}Hz", PWM_HZ);
    let mut adc_config = AdcConfig::default();
    adc_config.vdda_mv = 5_000;
    adc_config.frequency = Hertz(24_000_000);
    // The current-sense amplifier and its analog routing are established
    // before ADC1 is acquired, matching the C startup order.
    let _current_inputs = (board.current_adc.phase_a, board.current_adc.phase_b);
    let mut bus_pin = Flex::new(board.aux_adc.bus_voltage);
    bus_pin.set_as_analog();
    let mut pot_pin = Flex::new(board.aux_adc.potentiometer);
    pot_pin.set_as_analog();

    // Configure both current-sense amplifiers through the HAL. Their fixed PB0/PB1
    // outputs remain available for ADC1 sampling.
    let mut opa_config = hal::opamp::Config::default();
    opa_config.supply_mv = 5_000;
    let opa1_ctrl = hal::opamp::OpAmp::new(board.current_sense.opa_a, opa_config).unwrap();
    let opa2_ctrl = hal::opamp::OpAmp::new(board.current_sense.opa_b, opa_config).unwrap();
    let mut opa1 = opa1_ctrl
        .standalone_ext_adc(
            board.current_sense.positive_a,
            board.current_sense.negative_a,
        )
        .unwrap();
    let mut opa2 = opa2_ctrl
        .standalone_ext_adc(
            board.current_sense.positive_b,
            board.current_sense.negative_b,
        )
        .unwrap();
    cortex_m::asm::delay(800);
    opa1.calibrate(100);
    opa2.calibrate(100);
    defmt::info!(
        "init: opa current amplifiers enabled opa1={=bool} opa2={=bool} cal1={=bool} cal2={=bool}",
        opa1.is_enabled(),
        opa2.is_enabled(),
        opa1.calibration_enabled(),
        opa2.calibration_enabled()
    );

    // Keep ADC1 owned for the lifetime of the firmware. Its sequence is
    // reprogrammed below to the C project's ATIM-CC4 triggered scan.
    let adc1 = Adc::new_motor(board.current_adc.adc, current_loop::Irqs, adc_config).unwrap();
    let _adc2 = Adc::new(board.aux_adc.adc, adc_config).unwrap();
    adc::auxiliary();

    let mut pwm = ComplementaryPwm::new(
        board.bridge.timer,
        Some(PwmPin::new(board.bridge.ch1)),
        Some(ComplementaryPwmPin::new(board.bridge.ch1n)),
        Some(PwmPin::new(board.bridge.ch2)),
        Some(ComplementaryPwmPin::new(board.bridge.ch2n)),
        Some(PwmPin::new(board.bridge.ch3)),
        Some(ComplementaryPwmPin::new(board.bridge.ch3n)),
        None,
        None,
        None,
        // Program half the requested period, then switch ATIM to CMS=01.
        // The resulting up/down cycle is 16 kHz; at the 24 MHz bring-up clock
        // the ARR is 750, leaving ample resolution for the current loop.
        Hertz(PWM_HZ * 2),
        CountingMode::EdgeAlignedUp,
        PwmConfig {
            dead_time_ns: 2_000,
        },
    )
    .unwrap();
    BRIDGE_CONFIGURED.store(true, Ordering::Release);
    hal::timer::complementary_pwm::irq_center::<hal::peripherals::ATIM>();
    for channel in [Channel::Ch1, Channel::Ch2, Channel::Ch3] {
        pwm.set_duty_cycle(channel, pwm.get_max_duty() / 2);
        pwm.enable(channel);
    }
    // Keep the power stage off while current offsets are sampled and until the
    // operator performs the zero-pot arming gesture below.
    pwm.set_master_output_enable(false);

    let pwm_period = pwm.get_max_duty();
    // Arm the same ADC1 EOS path used by the C project before offset
    // calibration. MOE is still low, so the ISR only drains conversions and
    // cannot apply any voltage to the motor.
    adc::current_loop(pwm_period);

    let mut encoder = connect(board.encoder);

    // The bridge stays disabled while offsets are measured and while the rotor
    // is held at the electrical zero used for encoder alignment. Match the C
    // calibration: 16 samples, spaced by 2 ms, taken from the timer-triggered
    // ADC results after OPA auto-zero has completed.
    defmt::info!("adc: offset sampling start");
    let mut offset_a = 0u32;
    let mut offset_b = 0u32;
    let start_us = Timing::micros();
    let start_eos = CURRENT_LOOP_COUNT.load(Ordering::Relaxed);
    for _ in 0..16 {
        timing.delay_us(2000);
        offset_a += u32::from(adc1.latest_result(0).unwrap_or(0));
        offset_b += u32::from(adc1.latest_result(1).unwrap_or(0));
    }
    let eos = CURRENT_LOOP_COUNT
        .load(Ordering::Relaxed)
        .wrapping_sub(start_eos);
    let sample_us = Timing::micros().wrapping_sub(start_us);
    defmt::info!(
        "ADC1 offset window: eos={=u16} elapsed_us={=u32}",
        eos,
        sample_us
    );
    let offset_a = (offset_a / 16) as u16;
    let offset_b = (offset_b / 16) as u16;
    CURRENT_OFFSET_A.store(offset_a, Ordering::Relaxed);
    CURRENT_OFFSET_B.store(offset_b, Ordering::Relaxed);
    defmt::info!(
        "adc: offset sampling done ia={=u16} ib={=u16}",
        offset_a,
        offset_b
    );

    // Missing conversions or a railed amplifier cannot establish a current zero.
    if !(512..=3584).contains(&offset_a) || !(512..=3584).contains(&offset_b) {
        defmt::error!("invalid current offsets; check 02-current-loop sampling");
        halt();
    }
    unsafe extern "C" {
        static __foc_calibration: [u8; 12];
    }
    let address = core::ptr::addr_of!(__foc_calibration).cast::<u8>();
    // Read-only mapping of the linker-reserved page; these images have no writer.
    let bytes = core::array::from_fn(|i| unsafe { address.wrapping_add(i).read_volatile() });
    let Some(calibration) = Calibration::decode(bytes) else {
        defmt::error!(
            "invalid calibration at {=u32:x}: {=[u8; 12]:x}; need 01 FLASH save OK",
            address as u32,
            &bytes
        );
        halt();
    };
    cortex_m::interrupt::free(|cs| {
        CURRENT_CONTROLLER.borrow(cs).borrow_mut().motor.pole_pairs = calibration.pole_pairs as u16;
    });
    defmt::info!(
        "using 01 calibration: pairs={=i32} direction={=i32} zero={=u16}",
        calibration.pole_pairs,
        calibration.direction,
        calibration.mechanical_zero
    );
    let Ok(angle) = encoder.read_angle() else {
        defmt::error!("SSI transfer failed");
        halt();
    };
    CURRENT_ELECTRICAL_ANGLE.store(u32::from(calibration.angle(angle)), Ordering::Relaxed);
    CURRENT_RESET.store(1, Ordering::Release);
    defmt::info!("state=wait-zero: move potentiometer to zero, then raise it to enable");
    let analog = Analog {
        _adc1: adc1,
        _adc2,
        _opa1: opa1,
        _opa2: opa2,
        _bus: bus_pin,
        _pot: pot_pin,
    };
    crate::speed::install(pwm, encoder, calibration, angle, analog);
    loop {
        timing.heartbeat();
        telemetry::report();
        cortex_m::asm::wfi();
    }
}

fn halt() -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

// A terminal fault can interrupt the controller between changing its software
// gate and writing MOE. Use the hardware gate before attempting RTT output.
fn shutdown_on_fault() {
    cortex_m::interrupt::disable();
    if BRIDGE_CONFIGURED.load(Ordering::Acquire) {
        irq_moe::<ATIM>(false);
        cortex_m::asm::dsb();
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    shutdown_on_fault();
    defmt::error!("panic: firmware panic handler entered");
    halt()
}

#[exception]
unsafe fn HardFault(frame: &ExceptionFrame) -> ! {
    shutdown_on_fault();
    defmt::error!(
        "HardFault pc={=u32:x} lr={=u32:x} xpsr={=u32:x}",
        frame.pc(),
        frame.lr(),
        frame.xpsr()
    );
    halt()
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) -> ! {
    shutdown_on_fault();
    defmt::error!("fault: unhandled irq={=i16}", irqn);
    halt()
}
