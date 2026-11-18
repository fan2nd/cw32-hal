#![no_std]
#![no_main]

//! Low-modulation open-loop bring-up for the 4621 power stage.
//!
//! This binary deliberately bypasses the encoder and current loop. It is for
//! checking phase order and basic commutation with a conservative modulation
//! limit before enabling closed-loop FOC.

use cortex_m_rt::entry;
use cw32_l012_foc_open_loop::{
    board::{
        ALIGNMENT_PERIODS, BUS_MAX_MV, BUS_MIN_MV, Encoder, POT_MAX_COUNTS, POT_MAX_RPM,
        POT_MIN_COUNTS, POT_ZERO_COUNTS, PWM_HZ, READY_ZERO_SAMPLES, angle_delta, apply_duty,
        bus_voltage_mv, sin_cos_q15, wait_period,
    },
    control::target_rpm_from_pot,
    foc::{Duty, JD4631},
};
use defmt_rtt as _;
use embassy_cw32::{
    self as hal,
    adc::{Adc, Common, Config as AdcConfig, SampleTime},
    cordic::Cordic,
    gpio::{Flex, Level, Output, Speed},
    spi::{self, Spi},
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig},
        low_level::{CountingMode, Timer},
        simple_pwm::PwmPin,
    },
};
use embedded_hal::spi::MODE_1;

use defmt::info;
const OPEN_LOOP_MIN_MODULATION_Q15: i32 = 6_000; // approximately 18% at low speed.
const OPEN_LOOP_MAX_MODULATION_Q15: i32 = 12_000; // approximately 37% at 1000 rpm.
const ALIGNMENT_ELECTRICAL_ANGLE: u16 = 0; // A high, B/C low alignment vector.
const OPEN_LOOP_LOG_PERIODS: u32 = 137; // Avoid sampling exactly once per 11 Hz electrical cycle.
const FORCE_OPEN_LOOP_RPM: i32 = 100; // Temporary alignment/phase-order test.
const HALF_Q15: i32 = 16_384;
const SQRT3_OVER_2_Q15: i32 = 28_346;

fn phase_step_for_rpm(target_rpm: u32, pole_pairs: u16) -> u32 {
    // phase_step = rpm * pole_pairs * 2^30 / (60 * PWM_HZ).
    // Use the constant quotient so every intermediate remains u32.
    const DENOMINATOR: u32 = 60 * PWM_HZ;
    const SCALE: u32 = 1 << 30;
    let turns = target_rpm.saturating_mul(u32::from(pole_pairs));
    let quotient = SCALE / DENOMINATOR;
    // The discarded remainder contributes less than 0.1% at the 1000 rpm
    // ceiling and avoiding that product keeps the worst case below u32::MAX.
    turns.saturating_mul(quotient).max(1)
}

defmt::timestamp!("{=u32}", 0u32);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

fn open_loop_duty(
    angle: u16,
    modulation_q15: i32,
    cordic: &mut Cordic<'_, hal::peripherals::CORDIC>,
) -> Duty {
    // The open-loop phase accumulator is kept in the convenient 14-bit
    // electrical-angle domain; the shared CORDIC helper uses the C reference's
    // 16-bit full-turn domain.
    let (sin, cos) = sin_cos_q15(cordic, angle << 2);
    let sin = i64::from(sin);
    let cos = i64::from(cos);
    // Inverse Clarke: b/c are -alpha/2 +/- sqrt(3)*beta/2.
    // Keeping the half factor is essential; -alpha would destroy the 120°
    // phase spacing and only shake the rotor while drawing current.
    let beta = ((-sin * i64::from(HALF_Q15))
        + (cos * i64::from(SQRT3_OVER_2_Q15))) >> 15;
    let gamma = ((-sin * i64::from(HALF_Q15))
        - (cos * i64::from(SQRT3_OVER_2_Q15))) >> 15;
    Duty {
        a: centered_duty(sin, modulation_q15),
        b: centered_duty(beta, modulation_q15),
        c: centered_duty(gamma, modulation_q15),
    }
}

const Q15_ONE: i32 = 32_767;

fn centered_duty(value: i64, modulation_q15: i32) -> u16 {
    (16_384_i64 + value * i64::from(modulation_q15) / i64::from(Q15_ONE))
        .clamp(1, 32_766) as u16
}

fn modulation_for_rpm(target_rpm: i32) -> i32 {
    OPEN_LOOP_MIN_MODULATION_Q15
        + (target_rpm.clamp(0, POT_MAX_RPM)
            * (OPEN_LOOP_MAX_MODULATION_Q15 - OPEN_LOOP_MIN_MODULATION_Q15)
            / POT_MAX_RPM)
}

#[entry]
fn main() -> ! {
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.min_supply_mv = 5_000;
    system.rcc.operating_conditions.max_supply_mv = 5_000;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div1;
    system.rcc.allow_nominal_bus_overclock = true;
    let p = hal::try_init(system).unwrap();
    let mut led = Output::new(p.PC13, Level::High, Speed::Default);
    info!(
        "4621 open-loop bring-up: modulation={=i32}..{=i32}/32767",
        OPEN_LOOP_MIN_MODULATION_Q15, OPEN_LOOP_MAX_MODULATION_Q15
    );

    let common = Common::new(p.BGR, 100_000).unwrap();
    let mut adc_config = AdcConfig::default();
    adc_config.vdda_mv = 5_000;
    adc_config.frequency = Hertz(24_000_000);
    let mut adc1 = Adc::new(p.ADC1, &common, adc_config).unwrap();
    let mut adc2 = Adc::new(p.ADC2, &common, adc_config).unwrap();
    let mut ia_pin = p.PB0;
    let mut ib_pin = p.PB1;
    let mut bus_pin = p.PA8;
    let mut pot_pin = p.PA10;

    // Match the C board's current-sense path: OPA1 PA6/PA7 -> PB0 and
    // OPA2 PA4/PA5 -> PB1. Without this setup ADC1 reads zero even while the
    // bridge is switching.
    let mut opa_config = hal::opamp::Config::default();
    opa_config.supply_mv = 5_000;
    let opa1 = hal::opamp::OpAmp::new(p.OPA1, opa_config).unwrap();
    let opa2 = hal::opamp::OpAmp::new(p.OPA2, opa_config).unwrap();
    core::mem::forget(opa1);
    core::mem::forget(opa2);
    let mut opa1_inp = Flex::new(p.PA6);
    opa1_inp.set_as_analog();
    let mut opa1_inn = Flex::new(p.PA7);
    opa1_inn.set_as_analog();
    let mut opa2_inp = Flex::new(p.PA4);
    opa2_inp.set_as_analog();
    let mut opa2_inn = Flex::new(p.PA5);
    opa2_inn.set_as_analog();
    hal::pac::OPA1.cr().write(|w| {
        w.set_bias(hal::pac::opa::vals::Bias::Ua8);
        w.set_mode(hal::pac::opa::vals::Mode::External);
        w.set_inp2en(true);
        w.set_inn2en(true);
        w.set_en(true);
    });
    hal::pac::OPA2.cr().write(|w| {
        w.set_bias(hal::pac::opa::vals::Bias::Ua8);
        w.set_mode(hal::pac::opa::vals::Mode::External);
        w.set_inp1en(true);
        w.set_inn1en(true);
        w.set_en(true);
    });
    cortex_m::asm::delay(800);
    info!("init: opa current amplifiers enabled");

    let mut offset_a = 0u32;
    let mut offset_b = 0u32;
    for _ in 0..64 {
        offset_a += u32::from(
            adc1.blocking_read(&mut ia_pin, SampleTime::Cycles30)
                .unwrap(),
        );
        offset_b += u32::from(
            adc1.blocking_read(&mut ib_pin, SampleTime::Cycles30)
                .unwrap(),
        );
    }
    let offset_a = (offset_a / 64) as i32;
    let offset_b = (offset_b / 64) as i32;
    info!("current offsets: ia={=i32} ib={=i32}", offset_a, offset_b);

    let mut pwm = ComplementaryPwm::new(
        p.ATIM,
        Some(PwmPin::new(p.PB5)),
        Some(ComplementaryPwmPin::new(p.PA15)),
        Some(PwmPin::new(p.PB6)),
        Some(ComplementaryPwmPin::new(p.PB3)),
        Some(PwmPin::new(p.PB7)),
        Some(ComplementaryPwmPin::new(p.PB4)),
        None,
        None,
        None,
        Hertz(PWM_HZ),
        CountingMode::EdgeAlignedUp,
        PwmConfig {
            dead_time_ns: 2_000,
        },
    )
    .unwrap();
    for channel in [Channel::Ch1, Channel::Ch2, Channel::Ch3] {
        pwm.set_duty_cycle(channel, pwm.get_max_duty() / 2);
        pwm.enable(channel);
    }
    pwm.set_master_output_enable(false);
    let ccer = hal::pac::ATIM.ccer().read();
    let ccmr1 = hal::pac::ATIM.ccmr1cmp().read();
    let bdtr = hal::pac::ATIM.bdtr().read();
    let af1 = hal::pac::ATIM.af1().read();
    info!(
        "pwm regs: ccer e={=bool}/{=bool}/{=bool} p={=bool}/{=bool}/{=bool} np={=bool}/{=bool}/{=bool} oc={=u8}/{=u8} bke={=bool} bkp={=bool} moe={=bool} bif={=bool} bkine={=bool}",
        ccer.cc1e(),
        ccer.cc2e(),
        ccer.cc3e(),
        ccer.cc1p(),
        ccer.cc2p(),
        ccer.cc3p(),
        ccer.cc1np(),
        ccer.cc2np(),
        ccer.cc3np(),
        ccmr1.oc1m(),
        ccmr1.oc2m(),
        bdtr.bke(),
        bdtr.bkp(),
        bdtr.moe(),
        pwm.is_break_pending(),
        af1.bkine(),
    );

    let mut tick = Timer::new(p.BTIM1).unwrap();
    tick.set_frequency(Hertz(PWM_HZ));
    tick.start();
    let mut cordic = Cordic::new(p.CORDIC).unwrap();
    let mut cs = Flex::new(p.PF0);
    cs.set_high();
    cs.set_as_output(Speed::Default);
    let mut spi_config = spi::Config::default();
    spi_config.mode = MODE_1;
    spi_config.frequency = Hertz(4_000_000);
    let spi = Spi::new_blocking(p.SPI3, p.PB8, p.PA11, p.PC14, spi_config).unwrap();
    let mut encoder = Encoder::new(spi, cs);
    info!("encoder: SSI3 monitor enabled");

    info!("state=wait-zero: move potentiometer to zero");
    let mut zero_samples = 0u32;
    let mut wait_led_ticks = 0u32;
    loop {
        let pot = adc2
            .blocking_read(&mut pot_pin, SampleTime::Cycles518)
            .unwrap();
        if pot <= POT_ZERO_COUNTS {
            zero_samples = zero_samples.saturating_add(1);
            if zero_samples >= READY_ZERO_SAMPLES {
                break;
            }
        } else {
            zero_samples = 0;
        }
        wait_led_ticks += 1;
        if wait_led_ticks >= PWM_HZ / 4 {
            led.toggle();
            wait_led_ticks = 0;
        }
        wait_period(&mut tick);
    }

    info!("state=align: open-loop zero accepted");
    let align = Duty {
        a: 18_750,
        b: 14_000,
        c: 14_000,
    };
    apply_duty(&mut pwm, align);
    pwm.set_master_output_enable(true);
    led.set_low();
    for _ in 0..ALIGNMENT_PERIODS {
        wait_period(&mut tick);
    }
    pwm.set_master_output_enable(false);

    info!("state=open-loop: turn pot above zero to start");
    // Continue from the same electrical vector used during alignment.
    let mut phase_accum = u32::from(ALIGNMENT_ELECTRICAL_ANGLE) << 16;
    let mut speed_tick = 0u32;
    let mut log_tick = 0u32;
    let mut command_active = false;
    let mut phase_step = 0u32;
    let mut target_rpm_active = 0i32;
    let mut encoder_angle = encoder.read_angle().unwrap_or(0);
    let mut encoder_delta = 0i32;
    loop {
        wait_period(&mut tick);
        speed_tick += 1;
        if speed_tick >= 16 {
            let next_encoder_angle = encoder.read_angle().unwrap_or(encoder_angle);
            encoder_delta += angle_delta(next_encoder_angle, encoder_angle);
            encoder_angle = next_encoder_angle;
            let bus = adc2
                .blocking_read(&mut bus_pin, SampleTime::Cycles518)
                .unwrap();
            let pot = adc2
                .blocking_read(&mut pot_pin, SampleTime::Cycles518)
                .unwrap();
            let bus_mv = bus_voltage_mv(bus);
            let target_rpm = if FORCE_OPEN_LOOP_RPM > 0 {
                FORCE_OPEN_LOOP_RPM
            } else {
                target_rpm_from_pot(
                    pot,
                    POT_ZERO_COUNTS,
                    POT_MIN_COUNTS,
                    POT_MAX_COUNTS,
                    0,
                    POT_MAX_RPM,
                )
            };
            command_active = target_rpm > 0 && (BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv);
            target_rpm_active = if command_active { target_rpm } else { 0 };
            if !command_active {
                phase_accum = u32::from(ALIGNMENT_ELECTRICAL_ANGLE) << 16;
                phase_step = 0;
                pwm.set_master_output_enable(false);
                if bus_mv < BUS_MIN_MV || bus_mv > BUS_MAX_MV {
                    info!("open-loop inhibited: bus={=u32}mV", bus_mv);
                }
            } else {
                // The phase accumulator advances on every PWM period below.
                // Keep the denominator at the actual 16 kHz update rate even
                // though the command/potentiometer is sampled every 16 ticks.
                // The board's phase order may be opposite the encoder's
                // positive convention; start with the reverse scan for the
                // bring-up binary and make the direction explicit here.
                phase_step = phase_step_for_rpm(target_rpm as u32, JD4631.pole_pairs);
                let modulation_q15 = modulation_for_rpm(target_rpm);
                pwm.set_master_output_enable(true);
                if log_tick == 0 {
                    let ia_adc = i32::from(
                        adc1.blocking_read(&mut ia_pin, SampleTime::Cycles30)
                            .unwrap(),
                    );
                    let ib_adc = i32::from(
                        adc1.blocking_read(&mut ib_pin, SampleTime::Cycles30)
                            .unwrap(),
                    );
                    info!(
                        "pwm active: angle={=u16} step={=u32} mod={=i32} ia_adc={=i32} ib_adc={=i32} ia={=i32} ib={=i32} moe={=bool} bif={=bool} ccr={=u32}/{=u32}/{=u32}",
                        (phase_accum >> 16) as u16 & 0x3fff,
                        phase_step,
                        modulation_q15,
                        ia_adc,
                        ib_adc,
                        offset_a - ia_adc,
                        offset_b - ib_adc,
                        pwm.get_master_output_enable(),
                        pwm.is_break_pending(),
                        hal::pac::ATIM.ccr1().read().ccr1() as u32,
                        hal::pac::ATIM.ccr2().read().ccr2() as u32,
                        hal::pac::ATIM.ccr3().read().ccr3() as u32,
                    );
                }
                log_tick += 1;
                if log_tick >= OPEN_LOOP_LOG_PERIODS {
                    info!(
                        "open-loop: pot={=u16} target={=i32} bus={=u32}mV encoder={=u16} delta={=i32}",
                        pot, target_rpm, bus_mv, encoder_angle, encoder_delta
                    );
                    led.toggle();
                    log_tick = 0;
                }
            }
            speed_tick = 0;
        }
        let angle = (phase_accum >> 16) as u16 & 0x3fff;
        let modulation_q15 = modulation_for_rpm(target_rpm_active);
        apply_duty(&mut pwm, open_loop_duty(angle, modulation_q15, &mut cordic));
        pwm.set_master_output_enable(command_active);
        if command_active {
            phase_accum = phase_accum.wrapping_sub(phase_step);
        }
    }
}
