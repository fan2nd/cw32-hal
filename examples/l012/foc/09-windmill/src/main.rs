#![no_std]
#![no_main]

pub mod board_resources {
    include!("../../common/board_resources.rs");
}
pub mod adc;
mod current_loop;
mod sensor;
mod timing;
use crate::{board_resources::BoardResources, current_loop::*, sensor::connect, timing::Timing};
use cw32_l012_foc_algorithms as foc;
const PWM_HZ: u32 = 16_000;
use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m_rt::{ExceptionFrame, exception};
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config as AdcConfig},
    gpio::{Flex, Level, Output, Speed},
    peripherals::ATIM,
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig, irq_moe},
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};
use rtt_target::{ChannelMode, rtt_init, set_defmt_channel};

static BRIDGE_CONFIGURED: AtomicBool = AtomicBool::new(false);

defmt::timestamp!("{=u32}", Timing::micros());

#[cortex_m_rt::entry]
fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: { size: 1024, mode: ChannelMode::NoBlockSkip, name: "defmt" }
            1: { size: 512, mode: ChannelMode::NoBlockSkip, name: "live" }
        }
    };
    set_defmt_channel(channels.up.0);
    let mut live = channels.up.1;
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
    let mut timing = Timing::new(led, hal::rcc::clocks().hclk.0, board.speed_clock.timer);
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
    // is stationary. Match the C
    // calibration: 16 samples, spaced by 2 ms, taken from the timer-triggered
    // ADC results after OPA auto-zero has completed.
    defmt::info!("adc: offset sampling start");
    let mut offset_a = 0u32;
    let mut offset_b = 0u32;
    let start_us = Timing::micros();
    let start_speed = timing::SPEED_TICKS.load(Ordering::Relaxed);
    let start_eos = CURRENT_LOOP_COUNT.load(Ordering::Relaxed);
    for _ in 0..16 {
        timing.delay_us(2000);
        offset_a += u32::from(adc1.latest_result(0).unwrap_or(0));
        offset_b += u32::from(adc1.latest_result(1).unwrap_or(0));
    }
    let eos = CURRENT_LOOP_COUNT
        .load(Ordering::Relaxed)
        .wrapping_sub(start_eos);
    let speed_ticks = timing::SPEED_TICKS
        .load(Ordering::Relaxed)
        .wrapping_sub(start_speed);
    let sample_us = Timing::micros().wrapping_sub(start_us);
    defmt::info!(
        "ADC1 offset window: eos={=u16} btim1_ticks={=u32} elapsed_us={=u32}",
        eos,
        speed_ticks,
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
    if !(450..=570).contains(&eos)
        || !(512..=3584).contains(&offset_a)
        || !(512..=3584).contains(&offset_b)
    {
        defmt::error!("invalid current offsets; check 02-current-loop sampling");
        halt();
    }
    init_eau();
    defmt::info!("windmill: return pot to zero, then raise to start");
    READY.store(true, Ordering::Release);
    let mut last_idle_fault = None;
    // Keep a short trace only in thread mode. Neither ISR consumes encoder data.
    let mut motion = [None::<(u32, u16, i32)>; 5];
    let mut next_motion = 0;
    let mut last_motion = Timing::micros();
    let mut last_live = Timing::micros();
    let mut last_report = Timing::micros();
    loop {
        timing.heartbeat();
        let now = Timing::micros();
        let stage = STATE.load(Ordering::Acquire);
        if stage == 8 {
            motion.fill(None);
            next_motion = 0;
        } else if stage == 3 && now.wrapping_sub(last_motion) >= 2000 {
            last_motion = now;
            let raw = encoder.read_angle().ok();
            if STATE.load(Ordering::Acquire) == 3 && FAULT.load(Ordering::Acquire) == 0 {
                motion[next_motion] = raw.map(|angle| (now, angle, RPM.load(Ordering::Relaxed)));
                next_motion = (next_motion + 1) % motion.len();
            }
        }
        if now.wrapping_sub(last_report) >= 100_000 {
            last_report = now;
            let state = STATE.load(Ordering::Acquire);
            let target = TARGET.load(Ordering::Acquire);
            if state != 0 || target != 0 {
                last_idle_fault = None;
                if matches!(state, 3 | 9) && now.wrapping_sub(last_live) >= 500_000 {
                    last_live = now;
                    let rpm = RPM.load(Ordering::Relaxed);
                    let observer = match (state, OBSERVER.load(Ordering::Relaxed)) {
                        (9, _) => "capture",
                        (_, 1) => "emf",
                        (_, 2) => "relock",
                        _ => "hfi",
                    };
                    let wind = WIND_RESULT.load(Ordering::Relaxed);
                    let _ = writeln!(live, "{observer} target={target} rpm={rpm} wind={wind}");
                }
                continue;
            }
            let fault = FAULT.load(Ordering::Acquire);
            if last_idle_fault == Some(fault) {
                continue;
            }
            last_idle_fault = Some(fault);
            if fault == 0 {
                defmt::info!("idle");
            } else {
                defmt::warn!(
                    "stop fault={=u16} state={=u16} target={=i32} rpm={=i32} iqref={=i16}",
                    fault,
                    FAULT_STATE.load(Ordering::Relaxed),
                    FAULT_TARGET.load(Ordering::Relaxed),
                    FAULT_RPM.load(Ordering::Relaxed),
                    FAULT_IQ.load(Ordering::Relaxed)
                );
                if fault == 8 {
                    let newest = (next_motion + motion.len() - 1) % motion.len();
                    if let (Some((start, first, _)), Some((end, last, hfi))) =
                        (motion[next_motion], motion[newest])
                    {
                        let us = end.wrapping_sub(start);
                        if (1..=20_000).contains(&us) {
                            // 03 mechanical positive is decreasing raw MT6701 angle.
                            let delta = (i32::from(first) - i32::from(last) + 8192)
                                .rem_euclid(16384)
                                - 8192;
                            defmt::info!(
                                "before stop: encoder_avg_rpm={=i32} hfi_end_rpm={=i32} dt_us={=u32}",
                                delta * 3662 / us as i32,
                                hfi,
                                us
                            );
                        }
                    }
                } else if fault == 5 || fault == 6 || fault == 11 {
                    defmt::info!(
                        "axis_contrast={=i32} align_error_q16={=i16}",
                        CONTRAST.load(Ordering::Relaxed),
                        ALIGN_ERROR.load(Ordering::Relaxed)
                    );
                } else if fault == 10 {
                    defmt::info!("overrun_cycles={=i32}", ISR_CYCLES.load(Ordering::Relaxed));
                }
            }
        }
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
