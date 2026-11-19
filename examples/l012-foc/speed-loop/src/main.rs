#![no_std]
#![no_main]

use core::cell::RefCell;
mod adc;
use core::sync::atomic::{AtomicI16, AtomicI32, AtomicU16, AtomicU32, Ordering};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::{exception, ExceptionFrame};
use cw32_l012_foc_speed_loop::{
    hardware::{
        ALIGNMENT_PERIODS, BUS_MAX_MV, BUS_MIN_MV, EncoderAngleTracker, POT_MAX_COUNTS, POT_MAX_RPM,
        POT_MIN_COUNTS, POT_MIN_RPM, POT_ZERO_COUNTS, PWM_HZ, READY_ZERO_SAMPLES, SPEED_LOOP_DIV,
        angle_delta, bus_voltage_mv, electrical_angle,
        electrical_angle_step_for_rpm, sin_cos_q15_lut, speed_rpm_from_delta,
    },
    control::{
        ArmingGate, MotorState, combined_torque_reference, holding_torque_reference,
        target_rpm_from_pot,
    },
    foc::{CurrentSampleFilter, Duty, FocController, JD4631, SpeedController},
};
use cw32_l012_foc_speed_loop::{actuator::apply_duty, sensor::Encoder};
use embassy_time::{Duration, Ticker};
use embassy_cw32::interrupt::typelevel::{ADC1, Handler};
use embassy_cw32::{
    self as hal,
    adc::{Adc, Common, Config as AdcConfig, adc_irq_clear,
        adc_irq_enable_eos, adc_irq_eos, adc_irq_result},
    gpio::{Flex, Level, Output, Speed},
    spi::{self, Spi},
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig},
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};
use embedded_hal::spi::MODE_1;

use defmt_rtt as _;

defmt::timestamp!("{=u32}", 0u32);

// Bring-up mode: after the zero-pot arming gesture, run a fixed low speed so
// the alignment/encoder/current-loop path can be verified independently of
// the command potentiometer. Set to zero after the mechanical loop is proven.
const NO_MOTION_TIMEOUT_SPEED_TICKS: u32 = 2_000; // 2 s at the 1 kHz speed loop.
const SPEED_RAMP_STEP_RPM: i32 = 2; // Smooth command slew: 2 rpm per elapsed ms.
const PROBE_VOLTAGE_LIMIT_Q15: i16 = 6_000;
// The validated open-loop phase sequence produces a positive SSI-count delta
// on this board with the present electrical-angle convention. Keep the speed
// feedback sign positive for that direction.
const ENCODER_DIRECTION_SIGN: i16 = 1;
// Keep the software trip at the reference C current-loop range. Bench-supply
// current limiting remains the external protection during bring-up.
const CURRENT_TRIP_COUNTS: i32 = 2_000;
// Temporary bench-test command. Keep the normal potentiometer path below;
// this lets a hardware run start without asking the operator to touch the
// command knob while the alignment handoff is being verified.
const AUTO_START_TEST_RPM: i32 = 0;

// The controller is owned by the ADC EOS ISR for its whole lifetime. Keeping
// the value in place avoids taking an Option and moving the PI state on every
// 16 kHz sample. The main loop only publishes atomics to this fast path.
static CURRENT_CONTROLLER: Mutex<RefCell<FocController>> =
    Mutex::new(RefCell::new(FocController::new(JD4631)));
static CURRENT_SAMPLE_FILTER: Mutex<RefCell<CurrentSampleFilter>> =
    Mutex::new(RefCell::new(CurrentSampleFilter::new()));
static CURRENT_OFFSET_A: AtomicU16 = AtomicU16::new(0);
static CURRENT_OFFSET_B: AtomicU16 = AtomicU16::new(0);
// Raw samples are retained for hardware bring-up diagnostics. They let the
// log distinguish a bad ADC trigger/analog path from a control-law problem.
static CURRENT_ADC_A: AtomicU16 = AtomicU16::new(0);
static CURRENT_ADC_B: AtomicU16 = AtomicU16::new(0);
static CURRENT_ELECTRICAL_ANGLE: AtomicU32 = AtomicU32::new(0);
static CURRENT_ANGLE_STEP: AtomicI32 = AtomicI32::new(0);
static CURRENT_IQ_REFERENCE: AtomicI16 = AtomicI16::new(0);
static CURRENT_PHASE_A: AtomicI16 = AtomicI16::new(0);
static CURRENT_PHASE_B: AtomicI16 = AtomicI16::new(0);
static CURRENT_ID: AtomicI16 = AtomicI16::new(0);
static CURRENT_IQ: AtomicI16 = AtomicI16::new(0);
static CURRENT_RESET: AtomicU16 = AtomicU16::new(0);
static CURRENT_LOOP_COUNT: AtomicU16 = AtomicU16::new(0);
static CURRENT_LOOP_TOTAL: AtomicU32 = AtomicU32::new(0);
static CURRENT_PROBE_ACTIVE: AtomicU16 = AtomicU16::new(0);
static CURRENT_FAULT: AtomicU16 = AtomicU16::new(0);
static CURRENT_OUTPUT_ENABLE: AtomicU16 = AtomicU16::new(0);
static CURRENT_ALIGNMENT_ACTIVE: AtomicU16 = AtomicU16::new(0);

/// The foreground owns the PWM handle, while the ADC ISR may asynchronously
/// force MOE low.  Always publish the gate before asserting MOE so the ISR can
/// never immediately undo a legitimate nonzero-pot command with the stale
/// zero state.
fn set_current_output(pwm: &mut ComplementaryPwm<'_, hal::peripherals::ATIM>, enable: bool) {
    let previous = CURRENT_OUTPUT_ENABLE.load(Ordering::Acquire) != 0;
    CURRENT_OUTPUT_ENABLE.store(u16::from(enable), Ordering::Release);
    if previous == enable {
        return;
    }
    defmt::info!("moe: request enable={=bool}", enable);
    pwm.set_master_output_enable(enable);
    defmt::info!(
        "moe: applied request={=bool} readback={=bool}",
        enable,
        pwm.get_master_output_enable()
    );
}

struct Adc1CurrentLoop;

impl Handler<ADC1> for Adc1CurrentLoop {
    unsafe fn on_interrupt() {
        if !adc_irq_eos::<hal::peripherals::ADC1>() {
            return;
        }
        // EOS is a level request on this ADC. Mask it before touching the
        // current-loop state so a pending/sticky flag cannot immediately
        // re-enter the handler and starve the foreground probe/speed loop.
        adc_irq_enable_eos::<hal::peripherals::ADC1>(false);
        let ia = adc_irq_result::<hal::peripherals::ADC1>(0).unwrap_or(0);
        let ib = adc_irq_result::<hal::peripherals::ADC1>(1).unwrap_or(0);
        CURRENT_ADC_A.store(ia, Ordering::Relaxed);
        CURRENT_ADC_B.store(ib, Ordering::Relaxed);
        adc_irq_clear::<hal::peripherals::ADC1>();

        // The ADC trigger remains armed while the foreground state machine is
        // waiting for a nonzero speed command. Keep the power stage hard-off
        // in that state instead of allowing stale PI output to reach CCRx.
        // During alignment the foreground owns the fixed duty vector; the
        // current ISR must drain EOS without clearing MOE or replacing it.
        if CURRENT_ALIGNMENT_ACTIVE.load(Ordering::Relaxed) != 0 {
            adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
            return;
        }
        if CURRENT_OUTPUT_ENABLE.load(Ordering::Relaxed) == 0 {
            hal::timer::complementary_pwm::irq_moe(false);
            adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
            return;
        }

        // Predict the angle at the exact current-loop instant. The foreground
        // SSI read periodically re-anchors this accumulator; between reads the
        // ISR advances it by the measured electrical speed.
        let step = CURRENT_ANGLE_STEP.load(Ordering::Relaxed) as u16;
        let angle = CURRENT_ELECTRICAL_ANGLE
            .load(Ordering::Relaxed)
            .wrapping_add(u32::from(step)) as u16;
        CURRENT_ELECTRICAL_ANGLE.store(u32::from(angle), Ordering::Relaxed);
        let (sin, cos) = sin_cos_q15_lut(angle);
        let iq_reference = CURRENT_IQ_REFERENCE.load(Ordering::Relaxed);
        let (currents, duty) = cortex_m::interrupt::free(|cs| {
            let mut filter = CURRENT_SAMPLE_FILTER.borrow(cs).borrow_mut();
            let mut controller = CURRENT_CONTROLLER.borrow(cs).borrow_mut();
            if CURRENT_RESET.load(Ordering::Relaxed) != 0 {
                CURRENT_RESET.store(0, Ordering::Relaxed);
                controller.reset();
                filter.reset();
            }
            let currents = filter.sample(
                ia,
                ib,
                CURRENT_OFFSET_A.load(Ordering::Relaxed),
                CURRENT_OFFSET_B.load(Ordering::Relaxed),
            );
            controller.iq_reference = iq_reference;
            controller.voltage_limit = if CURRENT_PROBE_ACTIVE.load(Ordering::Relaxed) != 0 {
                PROBE_VOLTAGE_LIMIT_Q15
            } else {
                27_000
            };
            (currents, controller.update(currents, sin, cos))
        });
        CURRENT_PHASE_A.store(currents.a, Ordering::Relaxed);
        CURRENT_PHASE_B.store(currents.b, Ordering::Relaxed);
        cortex_m::interrupt::free(|cs| {
            let controller = CURRENT_CONTROLLER.borrow(cs).borrow();
            CURRENT_ID.store(controller.measured_id, Ordering::Relaxed);
            CURRENT_IQ.store(controller.measured_iq, Ordering::Relaxed);
        });
        let ia_q15 = i32::from(CURRENT_PHASE_A.load(Ordering::Relaxed));
        let ib_q15 = i32::from(CURRENT_PHASE_B.load(Ordering::Relaxed));
        if ia_q15.abs() > CURRENT_TRIP_COUNTS || ib_q15.abs() > CURRENT_TRIP_COUNTS {
            CURRENT_FAULT.store(1, Ordering::Relaxed);
            hal::timer::complementary_pwm::irq_moe(false);
        }
        let count = CURRENT_LOOP_COUNT.load(Ordering::Relaxed);
        CURRENT_LOOP_COUNT.store(count.wrapping_add(1), Ordering::Relaxed);
        let total = CURRENT_LOOP_TOTAL.load(Ordering::Relaxed);
        CURRENT_LOOP_TOTAL.store(total.wrapping_add(1), Ordering::Relaxed);
        let period = hal::timer::complementary_pwm::irq_arr();
        hal::timer::complementary_pwm::irq_ccr(Channel::Ch1, ((u32::from(duty.a) * period) >> 15).min(65_535) as u16);
        hal::timer::complementary_pwm::irq_ccr(Channel::Ch2, ((u32::from(duty.b) * period) >> 15).min(65_535) as u16);
        hal::timer::complementary_pwm::irq_ccr(Channel::Ch3, ((u32::from(duty.c) * period) >> 15).min(65_535) as u16);
        adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
    }
}

hal::bind_interrupts!(struct Irqs {
    ADC1 => Adc1CurrentLoop;
});

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

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // The board supplies the MCU/VDDA from 5 V. The motor-control application
    // deliberately opts into the calibrated nominal 96 MHz HSI bus path.
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.min_supply_mv = 5_000;
    system.rcc.operating_conditions.max_supply_mv = 5_000;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div1;
    system.rcc.allow_nominal_bus_overclock = true;
    let p = match hal::try_init(system) {
        Ok(p) => p,
        Err(error) => {
            defmt::error!("entry: RCC init failed: {:?}", error);
            loop {
                cortex_m::asm::wfi();
            }
        }
    };
    // The reference board LED is active-low on PC13, matching LEDON/LEDOFF
    // in the C project: waiting blinks, alignment is solid, and run heartbeats.
    let mut led = Output::new(p.PC13, Level::High, Speed::Default);
    defmt::info!("JD4631 FOC boot: PWM={=u32}Hz", PWM_HZ);
    let common = Common::new(p.BGR, 100_000).unwrap();
    let mut adc_config = AdcConfig::default();
    adc_config.vdda_mv = 5_000;
    // The default L012 clock tree is 8 MHz PCLK. 24 MHz is the requested
    // ceiling; the HAL then selects the qualified /1 ADC clock instead of
    // falling into the unqualified 4 MHz lower band.
    adc_config.frequency = Hertz(24_000_000);
    // The current-sense amplifier and its analog routing are established
    // before ADC1 is acquired, matching the C startup order.
    let mut bus_pin = Flex::new(p.PA8);
    bus_pin.set_as_analog();
    let mut pot_pin = Flex::new(p.PA10);
    pot_pin.set_as_analog();

    // Configure both current-sense amplifiers through the HAL. Their fixed PB0/PB1
    // outputs remain available for ADC1 sampling.
    let mut opa_config = hal::opamp::Config::default();
    opa_config.supply_mv = 5_000;
    let mut opa1_ctrl = hal::opamp::OpAmp::new(p.OPA1, opa_config).unwrap();
    let mut opa2_ctrl = hal::opamp::OpAmp::new(p.OPA2, opa_config).unwrap();
    let mut opa1 = opa1_ctrl.standalone_ext_adc(p.PA6, p.PA7).unwrap();
    let mut opa2 = opa2_ctrl.standalone_ext_adc(p.PA4, p.PA5).unwrap();
    cortex_m::asm::delay(800);
    opa1.calibrate(100);
    opa2.calibrate(100);
    defmt::info!("init: opa current amplifiers enabled opa1={=bool} opa2={=bool} cal1={=bool} cal2={=bool}",
        opa1.is_enabled(), opa2.is_enabled(), opa1.calibration_enabled(), opa2.calibration_enabled());

    // Keep ADC1 owned for the lifetime of the firmware. Its sequence is
    // reprogrammed below to the C project's ATIM-CC4 triggered scan.
    let adc1 = Adc::new_motor(p.ADC1, &common, Irqs, adc_config).unwrap();
    let _adc2 = Adc::new(p.ADC2, &common, adc_config).unwrap();
    adc::auxiliary();

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
    hal::timer::complementary_pwm::irq_center();
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

    let mut cs = Flex::new(p.PF0);
    cs.set_high();
    cs.set_as_output(Speed::Default);
    let mut spi_config = spi::Config::default();
    spi_config.mode = MODE_1;
    spi_config.frequency = Hertz(4_000_000);
    let spi = Spi::new_blocking(p.SPI3, p.PB8, p.PA11, p.PC14, spi_config).unwrap();
    let mut encoder = Encoder::new(spi, cs);

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

    // The power stage is kept disabled until the operator deliberately brings
    // the command pot to zero. This is the arming gesture required at power-on.
    let mut state = ArmingGate::new(READY_ZERO_SAMPLES);
    let mut wait_led_ticks = 0u32;
    let mut wait_zero_log_ticks = 0u32;
    defmt::info!("state=wait-zero: move potentiometer to zero");
    loop {
        pwm.set_master_output_enable(false);
        let pot = adc_irq_result::<hal::peripherals::ADC2>(1).unwrap_or(0);
        if wait_zero_log_ticks >= PWM_HZ / 2 {
            defmt::info!("wait-zero: pot={=u16}", pot);
            wait_zero_log_ticks = 0;
        }
        wait_zero_log_ticks += 1;
        if state.observe(pot, POT_ZERO_COUNTS) {
            break;
        }
        wait_led_ticks += 1;
        if wait_led_ticks >= PWM_HZ / 4 {
            led.toggle();
            wait_led_ticks = 0;
        }
        ticker.next().await;
    }
    led.set_low();
    defmt::info!("state=align: potentiometer zero accepted");
    state.set_state(MotorState::Aligning);

    let alignment = Duty {
        // Same static A-axis alignment vector as the C controller.  The
        // encoder sample taken after this hold is therefore electrical angle
        // zero; no guessed quarter-turn offset is needed in the Park transform.
        a: 18_156,
        b: 14_611,
        c: 14_611,
    };
    // The first interior duty write changes each channel from the disabled
    // endpoint. The HAL deliberately clears MOE for that transition, so write
    // all three alignment duties first and assert MOE afterwards.
    apply_duty(&mut pwm, alignment);
    CURRENT_ALIGNMENT_ACTIVE.store(1, Ordering::Release);
    pwm.set_master_output_enable(true);
    for _ in 0..ALIGNMENT_PERIODS {
        ticker.next().await;
    }
    // Keep the proven static alignment vector applied while the command
    // potentiometer remains in its zero deadband. This is the C startup
    // handoff: the rotor cannot drift before its encoder zero is captured.
    defmt::info!("state=aligned: waiting for nonzero command");
    let mut pot_wait_log_ticks = 0u32;
    loop {
        let pot = adc_irq_result::<hal::peripherals::ADC2>(1).unwrap_or(0);
        if pot_wait_log_ticks >= PWM_HZ / 2 {
            defmt::info!("wait-nonzero: pot={=u16}", pot);
            pot_wait_log_ticks = 0;
        }
        pot_wait_log_ticks += 1;
        if AUTO_START_TEST_RPM > 0 || pot > POT_MIN_COUNTS {
            break;
        }
        ticker.next().await;
    }
    let encoder_offset = encoder.read_angle().unwrap_or(0);
    // Keep the static alignment vector and MOE asserted until the first
    // nonzero command is handed to the current loop.  Dropping into a zero
    // q-reference current loop here leaves the rotor unpowered for the
    // blocking SSI/speed-loop handoff; static friction can then move it by an
    // electrical sector and make the first torque vector appear with either
    // sign.  The command edge below performs the atomic handoff.
    let direction_sign = ENCODER_DIRECTION_SIGN;
    CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
    CURRENT_PROBE_ACTIVE.store(0, Ordering::Relaxed);
    CURRENT_FAULT.store(0, Ordering::Relaxed);
    // The electrical direction was already validated in the open-loop bring-up.
    // Do not apply a second point-test torque here: it can disturb the aligned
    // rotor and blocks the foreground SSI handoff while the bridge is active.
    let aligned_angle = electrical_angle(encoder_offset, encoder_offset);
    CURRENT_ELECTRICAL_ANGLE.store(u32::from(aligned_angle), Ordering::Relaxed);
    CURRENT_ANGLE_STEP.store(0, Ordering::Relaxed);
    set_current_output(&mut pwm, false);
    CURRENT_ALIGNMENT_ACTIVE.store(0, Ordering::Release);
    CURRENT_RESET.store(1, Ordering::Release);
    CURRENT_IQ_REFERENCE.store(0, Ordering::Release);
    set_current_output(&mut pwm, true);
    let torque_direction_sign = ENCODER_DIRECTION_SIGN;
    defmt::info!("state=aligned: fixed torque_sign={=i16}", torque_direction_sign);

    state.set_state(MotorState::Running);
    led.set_low();
    defmt::info!("state=run: speed control enabled");
    let mut speed = SpeedController::new(POT_MAX_RPM);
    let initial_angle = encoder.read_angle().unwrap_or(encoder_offset);
    let mut angle_tracker = EncoderAngleTracker::new(initial_angle);
    let mut last_angle = initial_angle;
    let mut last_speed_loop_total = CURRENT_LOOP_TOTAL.load(Ordering::Relaxed);
    let mut speed_delta_sum = 0i32;
    let mut speed_tick = 0u32;
    // C: ENC_CalcAvrgMecSpeed keeps an 8-sample IIR average. Without this
    // filter the 16 ms SSI count window quantizes low-speed feedback between
    // positive and negative values and makes the speed PI pump iqref.
    let mut speed_feedback_rpm = 0i32;
    let mut speed_window_has_sample = false;
    let mut log_elapsed_pwm_periods = 0u32;
    let mut command_active = true;
    let mut no_motion_ticks = 0u32;
    let mut speed_reference_rpm = 0i32;
    // Match the C SampleSpeed hysteresis: once a nonzero command is accepted
    // above NMINVD, the 50-count band down to NMINVD-50 does not clear it.
    // Without this latch a low-speed command near ADC count 200 can alternate
    // between 50 rpm and zero from ordinary potentiometer noise.
    let mut command_rpm_latched = 0i32;

    loop {
        let pot_now = adc_irq_result::<hal::peripherals::ADC2>(1).unwrap_or(0);
        let auto_command = AUTO_START_TEST_RPM > 0;
        if !auto_command && pot_now <= POT_ZERO_COUNTS {
            // Zero is also the operator's fault-clear command. Drop the
            // bridge immediately and clear both PI states so the next command
            // starts from a known condition.
            let was_active = command_active || CURRENT_FAULT.load(Ordering::Relaxed) != 0;
            speed.reset();
            speed_feedback_rpm = 0;
            speed_reference_rpm = 0;
            command_rpm_latched = 0;
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            CURRENT_FAULT.store(0, Ordering::Relaxed);
            CURRENT_RESET.store(1, Ordering::Release);
            command_active = false;
            no_motion_ticks = 0;
            state.set_state(MotorState::Running);
            set_current_output(&mut pwm, false);
            if was_active {
                defmt::info!("run: potentiometer zero, output and faults cleared");
            }
        } else if CURRENT_FAULT.load(Ordering::Relaxed) != 0 {
            // Hold the bridge off until the operator returns the pot to zero.
            // This prevents the speed integrator from continuing to request
            // torque after a current fault.
            CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
            set_current_output(&mut pwm, false);
            continue;
        }
        let bus = adc_irq_result::<hal::peripherals::ADC2>(0).unwrap_or(0);
        let bus_mv = bus_voltage_mv(bus);
        if (auto_command || pot_now > POT_ZERO_COUNTS)
            && !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv)
        {
            set_current_output(&mut pwm, false);
            CURRENT_FAULT.store(1, Ordering::Relaxed);
            led.set_high();
            state.set_state(MotorState::Fault);
            defmt::error!(
                "state=fault: bus voltage={=u32}mV ia={=i16} ib={=i16} idq={=i16}/{=i16} iqref={=i16} angle={=u16} ccr={=u16}/{=u16}/{=u16} loops={=u16}",
                bus_mv,
                CURRENT_PHASE_A.load(Ordering::Relaxed),
                CURRENT_PHASE_B.load(Ordering::Relaxed),
                CURRENT_ID.load(Ordering::Relaxed),
                CURRENT_IQ.load(Ordering::Relaxed),
                CURRENT_IQ_REFERENCE.load(Ordering::Relaxed),
                last_angle,
                hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch1),
                hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch2),
                hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch3),
                CURRENT_LOOP_COUNT.load(Ordering::Relaxed)
            );
            continue;
        }
        let (mechanical_angle, sample_valid) = match encoder.read_angle() {
            Ok(encoder_sample) => angle_tracker.update_checked(encoder_sample),
            Err(_) => (last_angle, false),
        };
        speed_window_has_sample |= sample_valid;
        speed_delta_sum += angle_delta(mechanical_angle, last_angle);
        last_angle = mechanical_angle;
        speed_tick += 1;
        if speed_tick >= SPEED_LOOP_DIV {
            // Use the elapsed ADC/current-loop periods as the clock. SSI and
            // CORDIC are blocking operations, so a foreground pass is about
            // 1.9 ms on this build rather than the nominal 62.5 us PWM tick.
            let speed_loop_total = CURRENT_LOOP_TOTAL.load(Ordering::Relaxed);
            let elapsed_pwm_periods = speed_loop_total.wrapping_sub(last_speed_loop_total);
            last_speed_loop_total = speed_loop_total;
            let elapsed_ms = (elapsed_pwm_periods / (PWM_HZ / 1_000)).clamp(1, 100);
            // A blocking SSI transfer can fail while the ADC current ISR keeps
            // running. Treat a window with no accepted encoder sample as
            // missing data; feeding its zero displacement to the speed PI
            // creates the observed full-speed/full-stop oscillation.
            let feedback_rpm_raw = if speed_window_has_sample {
                speed_rpm_from_delta(speed_delta_sum, elapsed_pwm_periods)
            } else {
                speed_feedback_rpm
            };
            // Match C's ENC_CalcAvrgMecSpeed: last*7 + new, then /8.
            // Filtering is essential at low speed because a 16 ms SSI window
            // contains only a handful of encoder counts.
            speed_feedback_rpm = (speed_feedback_rpm
                .saturating_mul(7)
                .saturating_add(feedback_rpm_raw))
                / 8;
            let feedback_rpm = speed_feedback_rpm * i32::from(direction_sign);
            // electrical_angle() uses the inverted mechanical displacement, so
            // negate the physical signed speed when advancing the predicted
            // electrical angle in the ISR.
            CURRENT_ANGLE_STEP.store(
                electrical_angle_step_for_rpm(-feedback_rpm),
                Ordering::Relaxed,
            );
            let pot = pot_now;
            let mapped_command_rpm = if auto_command {
                AUTO_START_TEST_RPM
            } else {
                target_rpm_from_pot(
                    pot,
                    POT_ZERO_COUNTS,
                    POT_MIN_COUNTS,
                    POT_MAX_COUNTS,
                    POT_MIN_RPM,
                    POT_MAX_RPM,
                )
            };
            let command_rpm = if auto_command {
                mapped_command_rpm
            } else if pot <= POT_ZERO_COUNTS {
                command_rpm_latched = 0;
                0
            } else if pot <= POT_MIN_COUNTS {
                // Keep the previous nonzero command in the C deadband. A
                // genuine stop still requires returning to POT_ZERO_COUNTS.
                command_rpm_latched
            } else {
                command_rpm_latched = mapped_command_rpm;
                mapped_command_rpm
            };
            // Keep the speed reference equal to the operator command. The
            // static-friction feed-forward below supplies starting torque;
            // raising a 30 rpm command to an artificial 120 rpm kick makes
            // the motor overshoot before the speed loop can take control.
            let target_rpm = if command_rpm == 0 {
                speed_reference_rpm = 0;
                0
            } else if speed_reference_rpm < command_rpm {
                let ramp_step = SPEED_RAMP_STEP_RPM.saturating_mul(elapsed_ms as i32);
                speed_reference_rpm = (speed_reference_rpm + ramp_step).min(command_rpm);
                speed_reference_rpm
            } else if speed_reference_rpm > command_rpm {
                let ramp_step = SPEED_RAMP_STEP_RPM.saturating_mul(elapsed_ms as i32);
                speed_reference_rpm = (speed_reference_rpm - ramp_step).max(command_rpm);
                speed_reference_rpm
            } else {
                speed_reference_rpm
            };
            // Keep the bridge off throughout the complete command deadband.
            // Previously pot=151..200 entered the current loop with a zero
            // speed reference, so ADC offset/noise was converted into holding
            // current even though the operator had not requested motion.
            let mut speed_iq_reference = 0i16;
            let mut holding_iq_reference = 0i16;
            if target_rpm == 0 {
                // A zero command is an explicit stop request. Clear the speed
                // integrator so a previous positive command cannot restart the
                // motor while the pot is still at zero, and remove MOE so the
                // current loop cannot turn ADC noise into holding torque.
                speed.reset();
                CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
                if command_active {
                    defmt::info!("run: command deadband, bridge off pot={=u16}", pot);
                }
                command_active = false;
                no_motion_ticks = 0;
                set_current_output(&mut pwm, false);
            } else {
                speed_iq_reference = speed.update_for_ticks(target_rpm, feedback_rpm, elapsed_ms);
                holding_iq_reference = holding_torque_reference(target_rpm, POT_MAX_RPM);
                let iq_reference = i32::from(combined_torque_reference(
                    speed_iq_reference,
                    holding_iq_reference,
                    target_rpm,
                    feedback_rpm,
                ));
                let signed_iq_reference = iq_reference
                    .saturating_mul(i32::from(torque_direction_sign))
                    .clamp(i32::from(i16::MIN), i32::from(i16::MAX))
                    as i16;
                CURRENT_IQ_REFERENCE.store(signed_iq_reference, Ordering::Relaxed);
                if feedback_rpm.abs() >= 5 {
                    no_motion_ticks = 0;
                } else {
                    no_motion_ticks = no_motion_ticks.saturating_add(elapsed_ms);
                }
                if no_motion_ticks >= NO_MOTION_TIMEOUT_SPEED_TICKS {
                    let timeout_iq_reference = CURRENT_IQ_REFERENCE.load(Ordering::Relaxed);
                    CURRENT_IQ_REFERENCE.store(0, Ordering::Relaxed);
                    // Latch the startup failure with the bridge off.  Keep
                    // the foreground state machine alive so returning the
                    // potentiometer to zero can clear the fault and retry.
                    CURRENT_FAULT.store(1, Ordering::Release);
                    command_active = false;
                    no_motion_ticks = 0;
                    set_current_output(&mut pwm, false);
                    defmt::error!(
                        "startup timeout: no motion for 2s, output off pot={=u16} target={=i32} feedback={=i32} iqref_before={=i16} idq={=i16}/{=i16} ia={=i16} ib={=i16} ccr={=u16}/{=u16}/{=u16}",
                        pot,
                        target_rpm,
                        feedback_rpm,
                        timeout_iq_reference,
                        CURRENT_ID.load(Ordering::Relaxed),
                        CURRENT_IQ.load(Ordering::Relaxed),
                        CURRENT_PHASE_A.load(Ordering::Relaxed),
                        CURRENT_PHASE_B.load(Ordering::Relaxed),
                        hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch1),
                        hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch2),
                        hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch3)
                    );
                    continue;
                }
                if !command_active {
                    // Break the static alignment vector only at the same edge
                    // that enables the first nonzero current-loop request.
                    // This removes the unpowered gap between encoder capture
                    // and q-axis control, which otherwise makes the startup
                    // direction depend on rotor settling and load friction.
                    set_current_output(&mut pwm, false);
                    CURRENT_ALIGNMENT_ACTIVE.store(0, Ordering::Release);
                    CURRENT_RESET.store(1, Ordering::Release);
                    defmt::info!(
                        "run: command active, bridge on pot={=u16} target={=i32}",
                        pot,
                        target_rpm
                    );
                    command_active = true;
                    set_current_output(&mut pwm, true);
                    for _ in 0..4 {
                        ticker.next().await;
                    }
                    defmt::info!(
                        "run: command sample loops={=u16} idq={=i16}/{=i16} ref={=i16} ia={=i16} ib={=i16} angle={=u16} ccr={=u16}/{=u16}/{=u16}",
                        CURRENT_LOOP_COUNT.load(Ordering::Relaxed),
                        CURRENT_ID.load(Ordering::Relaxed),
                        CURRENT_IQ.load(Ordering::Relaxed),
                        CURRENT_IQ_REFERENCE.load(Ordering::Relaxed),
                        CURRENT_PHASE_A.load(Ordering::Relaxed),
                        CURRENT_PHASE_B.load(Ordering::Relaxed),
                        mechanical_angle,
                        hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch1),
                        hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch2),
                        hal::timer::complementary_pwm::irq_ccr_read(Channel::Ch3)
                    );
                } else {
                    set_current_output(&mut pwm, true);
                }
            }
            log_elapsed_pwm_periods = log_elapsed_pwm_periods.saturating_add(elapsed_pwm_periods);
            if log_elapsed_pwm_periods >= PWM_HZ {
                led.toggle();
                defmt::info!(
                    "run: pot={=u16} target={=i32} fb={=i32} iqref={=i16} idq={=i16}/{=i16} adc={=u16}/{=u16} off={=u16}/{=u16} bus={=u32}",
                    pot,
                    target_rpm,
                    feedback_rpm,
                    CURRENT_IQ_REFERENCE.load(Ordering::Relaxed),
                    CURRENT_ID.load(Ordering::Relaxed),
                    CURRENT_IQ.load(Ordering::Relaxed),
                    CURRENT_ADC_A.load(Ordering::Relaxed),
                    CURRENT_ADC_B.load(Ordering::Relaxed),
                    CURRENT_OFFSET_A.load(Ordering::Relaxed),
                    CURRENT_OFFSET_B.load(Ordering::Relaxed),
                    bus_mv
                );
                CURRENT_LOOP_COUNT.store(0, Ordering::Relaxed);
                log_elapsed_pwm_periods -= PWM_HZ;
            }
            speed_delta_sum = 0;
            speed_tick = 0;
            speed_window_has_sample = false;
        }
        let angle = electrical_angle(mechanical_angle, encoder_offset);
        CURRENT_ELECTRICAL_ANGLE.store(u32::from(angle), Ordering::Relaxed);
        // Keep the hardware gate synchronized with the command transition.
        // The explicit helper above handles the edge; this write also repairs
        // MOE if the ISR had cleared it for a transient current fault.
        if CURRENT_ALIGNMENT_ACTIVE.load(Ordering::Relaxed) != 0 {
            pwm.set_master_output_enable(true);
        } else if command_active && CURRENT_FAULT.load(Ordering::Relaxed) == 0 {
            pwm.set_master_output_enable(true);
        } else if !command_active {
            set_current_output(&mut pwm, false);
        }
    }
}
