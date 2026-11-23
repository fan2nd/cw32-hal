#![no_std]
#![no_main]

//! Open-loop field sweep for identifying encoder direction and motor pole pairs.
mod tasks;
use open_loop::{
    actuator::apply_duty,
    board_resources::BoardResources,
    foc::{Duty, svpwm},
    hardware::{
        BUS_MAX_MV, BUS_MIN_MV, POT_MIN_COUNTS, POT_ZERO_COUNTS, PWM_HZ, angle_delta,
        bus_voltage_mv, sin_cos_q15_lut,
    },
    sensor::connect,
};
use defmt_rtt as _;
use embassy_cw32::flash::{ERASE_SIZE, Flash, OperatingConditions, ReservedRegion};
unsafe extern "C" {
    static __foc_calibration: u8;
}
use open_loop::calibration::Calibration;
use embassy_cw32::{
    self as hal,
    adc::{Adc, Config as AdcConfig, SampleTime},
    gpio::{Level, Output, Speed},
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig},
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Ticker};

use defmt::info;
defmt::timestamp!("{=u32}", 0u32);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

// One electrical revolution is 65536 counts. Hold the initial field on phase A.
fn sweep_duty(angle: u16) -> Duty {
    let (sin, cos) = sin_cos_q15_lut(angle);
    // Apply one quarter of the full-scale stationary-frame voltage vector.
    svpwm(i32::from(cos) / 4, i32::from(sin) / 4)
}

#[inline(never)]
fn save_calibration(flash: &mut Flash<'_>, calibration: Calibration) -> Result<(), &'static str> {
    let record = calibration.record();
    let mut bytes = [0u8; 12];
    flash.blocking_read(0, &mut bytes).map_err(|_| "read")?;
    if bytes != record {
        flash
            .blocking_erase(0, ERASE_SIZE as u32)
            .map_err(|_| "erase")?;
        flash
            .blocking_write(4, &record[4..])
            .map_err(|_| "write payload")?;
        flash
            .blocking_write(0, &record[..4])
            .map_err(|_| "write marker")?;
        flash.blocking_read(0, &mut bytes).map_err(|_| "readback")?;
    }
    if bytes != record {
        return Err("verify");
    }
    Ok(())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut system = hal::Config::default();
    system.rcc.operating_conditions.supply_mv = 5_000;
    system.rcc.hsi.div = hal::rcc::HsiDiv::Div2;
    // The 5 V board and /2 HSI keep the qualified HCLK below the programming ceiling.
    let conditions = unsafe {
        OperatingConditions::from_rcc(
            system.rcc.operating_conditions,
            system.rcc.frequencies().unwrap(),
        )
    }
    .unwrap();
    let p = hal::init(system).unwrap();
    let mut board = BoardResources::from(p);
    let address = core::ptr::addr_of!(__foc_calibration) as u32;
    // All FOC linker scripts exclude this entire page. Only 01 owns its writer.
    let region = unsafe { ReservedRegion::new(address..address + ERASE_SIZE as u32) }.unwrap();
    let mut flash = Flash::new_blocking(board.storage.flash, region, conditions).unwrap();
    let led = Output::new(board.status.led, Level::High, Speed::Default);
    spawner.spawn(tasks::led(led).unwrap());
    let mut adc_config = AdcConfig::default();
    adc_config.vdda_mv = 5_000;
    adc_config.frequency = Hertz(24_000_000);
    let mut adc1 = Adc::new(board.current_adc.adc, adc_config).unwrap();
    let mut adc2 = Adc::new(board.aux_adc.adc, adc_config).unwrap();

    // Configure the current-sense amplifiers through the HAL; PB0/PB1 remain
    // available as ADC channels while the input pins are retained by the guards.
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
    info!(
        "init: opa current amplifiers enabled opa1={=bool} opa2={=bool}",
        opa1.is_enabled(),
        opa2.is_enabled()
    );

    let mut offset_a = 0u32;
    let mut offset_b = 0u32;
    for _ in 0..64 {
        offset_a += u32::from(
            adc1.blocking_read(&mut board.current_adc.phase_a, SampleTime::Cycles30)
                .unwrap(),
        );
        offset_b += u32::from(
            adc1.blocking_read(&mut board.current_adc.phase_b, SampleTime::Cycles30)
                .unwrap(),
        );
    }
    let offset_a = (offset_a / 64) as i32;
    let offset_b = (offset_b / 64) as i32;
    info!("current offsets: ia={=i32} ib={=i32}", offset_a, offset_b);

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
    info!(
        "pwm: moe={=bool} break={=bool}",
        pwm.get_master_output_enable(),
        pwm.is_break_pending()
    );

    const STEPS_PER_TURN: i32 = 66;
    const ALIGN_POINTS: usize = 6;
    const ALIGN_STEP: i32 = STEPS_PER_TURN / ALIGN_POINTS as i32;
    const TARGETS: [u16; ALIGN_POINTS] = [0, 10922, 21845, 32768, 43690, 54613];
    const TURNS: i32 = 12;
    let mut encoder = connect(board.encoder);
    let mut hold_ticks = 125;
    'arm: loop {
        let mut ticker = Ticker::every(Duration::from_millis(4));
        pwm.set_master_output_enable(false);
        info!(
            "sweep: voltage=25% hold_ms={=i32}; set pot to zero, then raise it to enable",
            hold_ticks * 4
        );
        let mut zero_seen = false;
        loop {
            ticker.next().await;
            let Ok(pot) =
                adc2.blocking_read(&mut board.aux_adc.potentiometer, SampleTime::Cycles518)
            else {
                continue;
            };
            zero_seen |= pot <= POT_ZERO_COUNTS;
            if zero_seen && pot > POT_MIN_COUNTS {
                break;
            }
        }
        let mut previous = 0;
        let mut displacement = [0i32; 2];
        let mut hold_mean = [0u16; ALIGN_POINTS];
        let mut hold_span = [0i32; ALIGN_POINTS];
        for leg in 0..2 {
            let mut anchor = 0;
            let mut first = 0;
            let mut sum = 0;
            let mut low = 0;
            let mut high = 0;
            for step in 0..=TURNS * STEPS_PER_TURN {
                let alignment = leg == 0 && step < STEPS_PER_TURN && step % ALIGN_STEP == 0;
                let dwell = if alignment || step == 0 || step == TURNS * STEPS_PER_TURN {
                    hold_ticks
                } else {
                    0
                };
                let phase = ((step % STEPS_PER_TURN) * 65536 / STEPS_PER_TURN) as u16;
                let electrical = if leg == 0 {
                    phase
                } else {
                    phase.wrapping_neg()
                };
                // Sample only after the commanded vector has been held for this tick.
                for elapsed in 0..=dwell {
                    ticker.next().await;
                    let pot =
                        adc2.blocking_read(&mut board.aux_adc.potentiometer, SampleTime::Cycles518);
                    let bus =
                        adc2.blocking_read(&mut board.aux_adc.bus_voltage, SampleTime::Cycles518);
                    let ia =
                        adc1.blocking_read(&mut board.current_adc.phase_a, SampleTime::Cycles30);
                    let ib =
                        adc1.blocking_read(&mut board.current_adc.phase_b, SampleTime::Cycles30);
                    let sample = encoder.read_angle();
                    if !pot.is_ok_and(|v| v > POT_ZERO_COUNTS)
                        || !bus
                            .is_ok_and(|v| (BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_voltage_mv(v)))
                        || !ia.is_ok_and(|v| (i32::from(v) - offset_a).abs() < 2000)
                        || !ib.is_ok_and(|v| (i32::from(v) - offset_b).abs() < 2000)
                        || sample.is_err()
                    {
                        pwm.set_master_output_enable(false);
                        info!("sweep aborted: check pot, bus, phase current and SSI");
                        continue 'arm;
                    }
                    let mechanical = sample.unwrap();
                    if alignment && elapsed > dwell - 32 {
                        let hold = (step / ALIGN_STEP) as usize;
                        let n = elapsed - (dwell - 31);
                        if n == 0 {
                            first = mechanical;
                            sum = 0;
                            low = 0;
                            high = 0;
                        }
                        let delta = angle_delta(mechanical, first);
                        sum += delta;
                        low = low.min(delta);
                        high = high.max(delta);
                        if n == 31 {
                            hold_mean[hold] = ((i32::from(first) + sum / 32) & 0x3fff) as u16;
                            hold_span[hold] = high - low;
                        }
                    }
                    if step == 0 && elapsed == dwell {
                        anchor = mechanical;
                        info!("alignment: electrical=0 mechanical_count={=u16}", anchor);
                    }
                    if step > 0 {
                        let delta = angle_delta(mechanical, previous);
                        if delta.abs() > 512 {
                            pwm.set_master_output_enable(false);
                            info!("sweep aborted: encoder discontinuity");
                            continue 'arm;
                        }
                        displacement[leg] += delta;
                        if step % 80 == 0 && elapsed == dwell {
                            info!(
                                "field_q16={=u16} mech_q14={=u16} from_zero={=i32}",
                                electrical,
                                mechanical,
                                angle_delta(mechanical, anchor)
                            );
                        }
                    }
                    previous = mechanical;
                    if elapsed == 0 {
                        apply_duty(&mut pwm, sweep_duty(electrical));
                        pwm.set_master_output_enable(true);
                    }
                }
            }
        }
        pwm.set_master_output_enable(false);
        let [forward, reverse] = displacement;
        let span = (forward.abs() + reverse.abs()) / 2;
        let travel = TURNS * 16384;
        if span > 0 && forward.signum() == -reverse.signum() {
            let pairs = (travel + span / 2) / span;
            if (1..=64).contains(&pairs)
                && (forward + reverse).abs() <= span / 20
                && (span * pairs - travel).abs() <= travel / 20
            {
                let sign = forward.signum();
                let scale = sign * pairs * 4;
                let initial = Calibration::from_zero(sign, pairs, hold_mean[0]);
                // Unwrap each phase residual around the first mechanical anchor.
                let sum: i32 = hold_mean
                    .iter()
                    .zip(TARGETS)
                    .map(|(&m, target)| i32::from(initial.error(m, target)))
                    .sum();
                let zero = ((i32::from(hold_mean[0]) + sum / (ALIGN_POINTS as i32 * scale))
                    & 0x3fff) as u16;
                let calibration = Calibration::from_zero(sign, pairs, zero);
                let errors: [i16; ALIGN_POINTS] =
                    core::array::from_fn(|i| calibration.error(hold_mean[i], TARGETS[i]));
                info!("six-point residual q16 (0,60,120,180,240,300)={:?}", errors);
                // Allow at most 1/32 electrical turn of jitter or disagreement.
                const MAX_ERROR: i32 = 65536 / 32;
                if errors.iter().any(|&e| i32::from(e).abs() > MAX_ERROR)
                    || hold_span.iter().any(|span| span * scale.abs() > MAX_ERROR)
                {
                    info!(
                        "inconclusive zero: pole_pairs_est={=i32} hold_counts={:?}",
                        pairs, hold_mean
                    );

                    info!(
                        "hold window span_counts(0,60,120,180,240,300)={:?}",
                        hold_span
                    );
                    hold_ticks = 250;
                    info!(
                        "01 NOT saved: zero check failed; next sweep uses 4 ms steps and 1 s holds"
                    );
                    continue;
                }
                if let Err(stage) = save_calibration(&mut flash, calibration) {
                    defmt::error!("01 FLASH failed during {=str}", stage);
                    continue;
                }
                hold_ticks = 125;
                info!(
                    "01 FLASH save OK: pole_pairs={=i32} direction={=i32} mechanical_zero_q14={=u16} electrical_offset_q16={=u16}",
                    pairs, sign, zero, calibration.offset
                );

                continue;
            }
        }
        info!(
            "inconclusive: forward_counts={=i32} reverse_counts={=i32}",
            forward, reverse
        );
    }
}
