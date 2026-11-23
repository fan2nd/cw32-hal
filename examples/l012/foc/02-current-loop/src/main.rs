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
pub mod tasks;
pub use cw32_l012_foc_algorithms as foc;
pub use foc::{calibration, control};

use crate::{
    board_resources::BoardResources, calibration::Calibration, control::ArmingGate, global::*,
    hardware::*, sensor::connect,
};
use core::sync::atomic::Ordering;
use cortex_m_rt::{ExceptionFrame, exception};
use defmt_rtt as _;
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config as AdcConfig, adc_irq_result},
    gpio::{Flex, Level, Output, Speed},
    peripherals::ADC2,
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig},
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Ticker};

defmt::timestamp!("{=u32}", 0u32);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // The board supplies the MCU/VDDA from 5 V. The motor-control application
    // deliberately opts into the calibrated nominal 96 MHz HSI bus path.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 5_000;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div1;
    system.rcc.allow_nominal_bus_overclock = true;
    let p = match hal::init(system) {
        Ok(p) => p,
        Err(error) => {
            defmt::error!("entry: RCC init failed: {:?}", error);
            loop {
                cortex_m::asm::wfi();
            }
        }
    };
    let board = BoardResources::from(p);
    // The reference board LED is active-low on PC13, matching LEDON/LEDOFF
    // in the C project: waiting blinks, alignment is solid, and run heartbeats.
    let led = Output::new(board.status.led, Level::High, Speed::Default);
    spawner.spawn(tasks::led(led).unwrap());
    defmt::info!("FOC boot: PWM={=u32}Hz", PWM_HZ);
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
    hal::timer::complementary_pwm::irq_center::<hal::peripherals::ATIM>();
    for channel in [Channel::Ch1, Channel::Ch2, Channel::Ch3] {
        pwm.set_duty_cycle(channel, pwm.get_max_duty() / 2);
        pwm.enable(channel);
    }
    // Keep the power stage off while current offsets are sampled and until the
    // operator performs the zero-pot arming gesture below.
    pwm.set_master_output_enable(false);

    let mut ticker = Ticker::every(Duration::from_micros(1_000_000 / u64::from(PWM_HZ)));
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
    for _ in 0..16 {
        for _ in 0..(PWM_HZ / 500) {
            ticker.next().await;
        }
        offset_a += u32::from(adc1.latest_result(0).unwrap_or(0));
        offset_b += u32::from(adc1.latest_result(1).unwrap_or(0));
    }
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
    let offsets_valid = (512..=3584).contains(&offset_a) && (512..=3584).contains(&offset_b);
    if !offsets_valid {
        defmt::error!("invalid current offsets: sampling only");
    }
    unsafe extern "C" {
        static __foc_calibration: [u8; 12];
    }
    let address = core::ptr::addr_of!(__foc_calibration).cast::<u8>();
    // Read-only mapping of the linker-reserved page; these images have no writer.
    let bytes = core::array::from_fn(|i| unsafe { address.wrapping_add(i).read_volatile() });
    let calibration = Calibration::decode(bytes);
    if calibration.is_none() {
        defmt::error!(
            "invalid calibration at {=u32:x}: {=[u8; 12]:x}; need 01 FLASH save OK",
            address as u32,
            &bytes
        );
    }
    let mut gate = ArmingGate::new(READY_ZERO_SAMPLES);
    let mut armed = false;
    let mut next_log = Instant::now();
    let calibration = loop {
        let now = Instant::now();
        if now >= next_log {
            let a = CURRENT_ADC_A.load(Ordering::Relaxed);
            let b = CURRENT_ADC_B.load(Ordering::Relaxed);
            defmt::info!(
                "ADC a/b={=u16}/{=u16} zero={=u16}/{=u16} current_mA={=i32}/{=i32}",
                a,
                b,
                offset_a,
                offset_b,
                current_ma_from_counts(q15_from_adc(a, offset_a)),
                current_ma_from_counts(q15_from_adc(b, offset_b))
            );
            next_log = now + Duration::from_millis(100);
        }
        if let Some(cal) = calibration.filter(|_| offsets_valid) {
            if let Some(pot) = adc_irq_result::<ADC2>(1) {
                if armed && pot > POT_MIN_COUNTS {
                    break cal;
                }
                if !armed && gate.observe(pot, POT_ZERO_COUNTS) {
                    armed = true;
                    defmt::info!("armed: raise pot for current control");
                }
            }
        }
        ticker.next().await;
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
    let Some(bus) = adc_irq_result::<ADC2>(0) else {
        return;
    };
    if !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_voltage_mv(bus)) {
        return;
    }
    let Ok(angle) = encoder.read_angle() else {
        defmt::error!("SSI transfer failed");
        return;
    };
    CURRENT_ELECTRICAL_ANGLE.store(u32::from(calibration.angle(angle)), Ordering::Relaxed);
    CURRENT_RESET.store(1, Ordering::Release);

    let mut ticker = Ticker::every(Duration::from_millis(1));
    let zero = calibration.mechanical_zero;
    let mut previous = zero;
    let mut last = Instant::now();
    let mut log_time = last;
    let mut active = false;
    let mut rpm = 0;
    loop {
        ticker.next().await;
        let now = Instant::now();
        let us = (now - last).as_micros().min(u64::from(u32::MAX)) as u32;
        last = now;
        let sample = encoder.read_angle();
        let pot = adc_irq_result::<ADC2>(1);
        let bus = adc_irq_result::<ADC2>(0).map(bus_voltage_mv);
        let mut delta = 0;
        if let Ok(angle) = sample {
            delta = angle_delta(angle, previous);
            previous = angle;
        }
        let stopped = pot.is_some_and(|p| p <= POT_ZERO_COUNTS);
        let valid = sample.is_ok()
            && pot.is_some()
            && bus.is_some_and(|v| (BUS_MIN_MV..=BUS_MAX_MV).contains(&v))
            && (!active || (delta.abs() <= MAX_SAMPLE_DELTA_COUNTS && us <= 10_000));
        if !stopped && !valid {
            CURRENT_FAULT.store(1, Ordering::Release);
        }
        if stopped || CURRENT_FAULT.load(Ordering::Acquire) != 0 {
            CURRENT_OUTPUT_ENABLE.store(0, Ordering::Release);
            pwm.set_master_output_enable(false);
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_RESET.store(1, Ordering::Release);
            if active && !stopped {
                defmt::error!("control fault: return pot to zero");
            }
            if stopped {
                CURRENT_FAULT.store(0, Ordering::Release);
            }
            active = false;
            rpm = 0;
            continue;
        }
        if !active && pot.unwrap() <= POT_MIN_COUNTS {
            continue;
        }
        // 60e6 / 16384 = 234375 / 64. Delta is bounded while active.
        if active {
            let measured = delta * 234_375 / (64 * us.max(1) as i32);
            rpm = (7 * rpm + measured) / 8;
        }
        let command = pot.unwrap().clamp(POT_MIN_COUNTS, POT_MAX_COUNTS) - POT_MIN_COUNTS;
        // Limit open-shaft acceleration. Iq is expressed in calibrated ADC counts.
        let iq = if rpm.abs() >= 240 {
            0
        } else {
            (i32::from(command) * 200 / i32::from(POT_MAX_COUNTS - POT_MIN_COUNTS)) as i16
        };
        CURRENT_ELECTRICAL_ANGLE.store(u32::from(calibration.angle(previous)), Ordering::Relaxed);
        CURRENT_ANGLE_STEP.store(calibration.step_for_rpm(rpm, PWM_HZ), Ordering::Relaxed);
        let iq = calibration.electrical_iq(iq);
        CURRENT_IQ_REFERENCE.store(iq, Ordering::Relaxed);
        cortex_m::interrupt::free(|_| {
            if CURRENT_FAULT.load(Ordering::Relaxed) == 0 {
                CURRENT_OUTPUT_ENABLE.store(1, Ordering::Release);
                pwm.set_master_output_enable(true);
            }
        });
        active = true;
        if (now - log_time).as_millis() >= 250 {
            log_time = now;
            defmt::info!(
                "pos_counts={=i32} rpm={=i32} iq_ref={=i16} id={=i16} iq={=i16}",
                angle_delta(previous, zero),
                rpm,
                iq,
                CURRENT_ID.load(Ordering::Relaxed),
                CURRENT_IQ.load(Ordering::Relaxed)
            );
        }
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    defmt::error!("panic: firmware panic handler entered");
    loop {
        cortex_m::asm::wfi();
    }
}

#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    defmt::error!("fault: hard fault handler entered");
    loop {
        cortex_m::asm::wfi();
    }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) -> ! {
    defmt::error!("fault: unhandled irq={=i16}", irqn);
    loop {
        cortex_m::asm::wfi();
    }
}
