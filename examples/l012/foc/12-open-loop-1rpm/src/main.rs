#![no_std]
#![no_main]

mod sensor;

pub mod board_resources {
    include!("../../common/board_resources.rs");
}
use board_resources::BoardResources;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::peripheral::{SYST, syst::SystClkSource};
use cw32_l012_foc_algorithms::{
    FocController, JD4631, PhaseCurrent, calibration::Calibration, sin_cos_q15_lut,
};
use defmt_rtt as _;
use embassy_cw32::{
    self as hal,
    adc::{
        Adc, AdcTrigger, Config as AdcConfig, SampleTime, adc_irq_clear, adc_irq_eos,
        adc_irq_result, configure_continuous, configure_timer_triggered,
    },
    gpio::Flex,
    opamp::{Config as OpaConfig, OpAmp},
    peripherals::{ADC1, ADC2, ATIM},
    rcc::HsiDiv,
    time::Hertz,
    timer::{
        Channel,
        complementary_pwm::{
            ComplementaryPwm, ComplementaryPwmPin, Config as PwmConfig, irq_cc4, irq_center,
            irq_moe,
        },
        low_level::CountingMode,
        simple_pwm::PwmPin,
    },
};

const RATE: u32 = 16_000;
const FIELD_CURRENT_COUNTS: i16 = 200;
const DEAD_TIME_NS: u32 = 2000;
const ALIGN_MS: u32 = 500;
static BRIDGE_READY: AtomicBool = AtomicBool::new(false);
defmt::timestamp!("{=u32}", 0u32);

fn stop() -> ! {
    if BRIDGE_READY.load(Ordering::Acquire) {
        irq_moe::<ATIM>(false);
    }
    loop {
        cortex_m::asm::wfi();
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    stop()
}

#[cortex_m_rt::exception]
unsafe fn HardFault(_: &cortex_m_rt::ExceptionFrame) -> ! {
    stop()
}

// Each EOS is one complete timer-triggered pair. SysTick counts down at HCLK.
fn sample(timeout_cycles: u32) -> Option<(i32, i32)> {
    let start = SYST::get_current();
    while !adc_irq_eos::<ADC1>() {
        if start.wrapping_sub(SYST::get_current()) & 0xff_ffff >= timeout_cycles {
            return None;
        }
    }
    let a = i32::from(adc_irq_result::<ADC1>(0)?);
    let b = i32::from(adc_irq_result::<ADC1>(1)?);
    adc_irq_clear::<ADC1>();
    Some((a, b))
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut sys = hal::Config::default();
    sys.rcc.operating_conditions.supply_mv = 5000;
    sys.rcc.hsi.div = HsiDiv::Div1;
    sys.rcc.allow_nominal_bus_overclock = true;
    unsafe extern "C" {
        static __foc_calibration: [u8; 12];
    }
    let addr = core::ptr::addr_of!(__foc_calibration).cast::<u8>();
    let record = core::array::from_fn(|i| unsafe { addr.add(i).read_volatile() });
    let Some(cal) = Calibration::decode(record) else {
        defmt::error!("missing calibration: run 01-open-loop");
        stop();
    };
    let board = BoardResources::from(hal::init_hsi(sys).unwrap());
    let mut encoder = sensor::connect(board.encoder);
    let mut syst = cortex_m::Peripherals::take().unwrap().SYST;
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(0xff_ffff);
    syst.clear_current();
    syst.enable_counter();
    let timeout = hal::rcc::clocks().hclk.0 / (RATE / 4);

    let mut opa_cfg = OpaConfig::default();
    opa_cfg.supply_mv = 5000;
    let mut opa1 = OpAmp::new(board.current_sense.opa_a, opa_cfg)
        .unwrap()
        .standalone_ext_adc(
            board.current_sense.positive_a,
            board.current_sense.negative_a,
        )
        .unwrap();
    let mut opa2 = OpAmp::new(board.current_sense.opa_b, opa_cfg)
        .unwrap()
        .standalone_ext_adc(
            board.current_sense.positive_b,
            board.current_sense.negative_b,
        )
        .unwrap();
    cortex_m::asm::delay(800);
    opa1.calibrate(100);
    opa2.calibrate(100);
    let mut bus = Flex::new(board.aux_adc.bus_voltage);
    bus.set_as_analog();
    let mut cfg = AdcConfig::default();
    cfg.vdda_mv = 5000;
    cfg.frequency = Hertz(24_000_000);
    let _adc1 = Adc::new(board.current_adc.adc, cfg).unwrap();
    let _adc2 = Adc::new(board.aux_adc.adc, cfg).unwrap();
    configure_continuous::<ADC2>([5, 5], SampleTime::Cycles518, 3);
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
        Hertz(RATE * 2),
        CountingMode::EdgeAlignedUp,
        PwmConfig {
            dead_time_ns: DEAD_TIME_NS,
        },
    )
    .unwrap();
    pwm.set_master_output_enable(false);
    BRIDGE_READY.store(true, Ordering::Release);
    let period = pwm.get_max_duty();
    // Center alignment gives two timer slopes per PWM period. One lost
    // high-side interval per period corresponds to half this many CCR ticks.
    let counter_hz = u64::from(period) * u64::from(pwm.get_frequency().0);
    let compensation_ticks =
        ((counter_hz * u64::from(DEAD_TIME_NS) + 1_000_000_000) / 2_000_000_000) as i32;
    defmt::info!(
        "deadtime A/B: ccr_trim={} period={} zero_band_counts=16",
        compensation_ticks,
        period
    );
    irq_center::<ATIM>();
    for ch in [Channel::Ch1, Channel::Ch2, Channel::Ch3] {
        pwm.set_duty_cycle(ch, period / 2);
        pwm.enable(ch);
    }
    configure_timer_triggered::<ADC1>([8, 9], SampleTime::Cycles30, 1, AdcTrigger::AtimCc4);
    irq_cc4::<ATIM>((period - 5) as u16);
    let (mut za, mut zb) = (0, 0);
    for _ in 0..512 {
        let Some((a, b)) = sample(timeout) else {
            defmt::error!("ADC startup timeout");
            stop()
        };
        za += a;
        zb += b;
    }
    za /= 512;
    zb /= 512;
    if !(512..=3584).contains(&za) || !(512..=3584).contains(&zb) {
        defmt::error!("invalid offsets");
        stop();
    }
    defmt::info!(
        "1 rpm forward/reverse: pairs={} direction={}",
        cal.pole_pairs,
        cal.direction
    );
    let cycles_per_ms = hal::rcc::clocks().hclk.0 / 1000;
    let mut previous = SYST::get_current();
    let mut cycles = 0;
    let mut motion_ms = 0u32;
    let mut running = false;
    let mut align_ms = 0;
    let mut angle = 0u16;
    let mut fraction = 0;
    let mut controller = FocController::new(JD4631);
    let mut field_trig = sin_cos_q15_lut(0);
    let mut report_ms = 0;
    let mut samples = [0u16; 20];
    let mut sample_count = 0;
    loop {
        let Some((a, b)) = sample(timeout) else {
            irq_moe::<ATIM>(false);
            defmt::error!("ADC timeout");
            stop();
        };
        let now = SYST::get_current();
        cycles += previous.wrapping_sub(now) & 0xff_ffff;
        previous = now;
        let ia = za - a;
        let ib = zb - b;
        if ia.abs().max(ib.abs()).max((ia + ib).abs()) >= 2000 {
            irq_moe::<ATIM>(false);
            defmt::error!("overcurrent ia={} ib={}", ia, ib);
            stop();
        }
        if cycles >= cycles_per_ms {
            cycles -= cycles_per_ms;
            let bus_mv = u32::from(adc_irq_result::<ADC2>(0).unwrap()) * 55_000 / 4095;
            if !(8000..=16000).contains(&bus_mv) {
                irq_moe::<ATIM>(false);
                defmt::error!("bus fault mv={}", bus_mv);
                stop();
            }
            controller.id_reference = if align_ms < ALIGN_MS {
                align_ms += 1;
                (i32::from(FIELD_CURRENT_COUNTS) * align_ms as i32 / ALIGN_MS as i32) as i16
            } else {
                if motion_ms == 240_000 {
                    irq_moe::<ATIM>(false);
                    defmt::info!("test complete: bridge off");
                    stop();
                }
                motion_ms += 1;
                // One mechanical turn per 60,000 ms. Carry the fractional electrical counts.
                fraction += cal.pole_pairs * 65536;
                let sign = if motion_ms <= 60_000 || (120_001..=180_000).contains(&motion_ms) {
                    cal.direction
                } else {
                    -cal.direction
                };
                angle = angle.wrapping_add((sign * (fraction / 60000)) as u16);
                fraction %= 60000;
                FIELD_CURRENT_COUNTS
            };
            field_trig = sin_cos_q15_lut(angle);
            if motion_ms > 0 {
                report_ms += 1;
            }
            if !running {
                running = true;
                defmt::info!(
                    "run: 1 mechanical rpm id_ref={} iq_ref=0",
                    FIELD_CURRENT_COUNTS
                );
            }
        }
        if !running {
            continue;
        }
        let duty = controller.update(
            PhaseCurrent {
                a: ia as i16,
                b: ib as i16,
            },
            field_trig.0,
            field_trig.1,
        );
        for (ch, d, current) in [
            (Channel::Ch1, duty.a, ia),
            (Channel::Ch2, duty.b, ib),
            (Channel::Ch3, duty.c, -ia - ib),
        ] {
            let nominal = ((u32::from(d) * period) >> 15) as i32;
            let trim = if motion_ms > 120_000 {
                compensation_ticks * current.clamp(-16, 16) / 16
            } else {
                0
            };
            pwm.set_duty_cycle(ch, (nominal + trim).clamp(1, period as i32 - 1) as u32);
        }
        irq_moe::<ATIM>(true);
        if report_ms == 10 {
            report_ms = 0;
            // 0xffff marks a failed read; every slot retains its original time.
            samples[sample_count] = encoder.read_angle().unwrap_or(u16::MAX);
            sample_count += 1;
            if sample_count == samples.len() {
                defmt::info!("end_ms={} mech={=?}", motion_ms, &samples);
                sample_count = 0;
            }
        }
    }
}
