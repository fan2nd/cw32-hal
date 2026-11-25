#![no_std]
#![no_main]

pub mod board_resources {
    include!("../../common/board_resources.rs");
}
use board_resources::BoardResources;
use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::peripheral::{SYST, syst::SystClkSource};
use cw32_l012_foc_algorithms::svpwm;
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
const AMPLITUDE_Q15: i32 = 2700;
// The East Is Red, opening eight bars in 2/4. Durations are eighth notes.
const G4: u32 = 392;
const A4: u32 = 440;
const C5: u32 = 523;
const D5: u32 = 587;
#[rustfmt::skip]
const SCORE: &[(u32, u32)] = &[
    (G4, 2), (G4, 1), (A4, 1),
    (D5, 4),
    (C5, 2), (C5, 1), (A4, 1),
    (D5, 4),
    (G4, 2), (G4, 2),
    (A4, 1), (C5, 1), (A4, 1), (G4, 1),
    (C5, 2), (C5, 1), (A4, 1),
    (D5, 4),
];
const NOTE_COUNT: usize = SCORE.len() * 2;
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
    let board = BoardResources::from(hal::init_hsi(sys).unwrap());
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
        PwmConfig { dead_time_ns: 2000 },
    )
    .unwrap();
    pwm.set_master_output_enable(false);
    BRIDGE_READY.store(true, Ordering::Release);
    irq_center::<ATIM>();
    let period = pwm.get_max_duty();
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
    defmt::info!("melody start offsets={}/{}", za, zb);
    let mut peak = 0;
    let mut ripple = [0i32; NOTE_COUNT];
    let mut note_ms = [0u32; NOTE_COUNT];
    let cycles_per_ms = hal::rcc::clocks().hclk.0 / 1000;
    for (i, &(hz, length)) in SCORE.iter().cycle().take(NOTE_COUNT).enumerate() {
        let ticks = length * (RATE * 5 / 16);
        let sounding = ticks - 480;
        let step = (hz * 65536 + RATE / 2) / RATE;
        let mut phase = 0u16;
        let mut low = i32::MAX;
        let mut high = i32::MIN;
        let mut elapsed = 0u32;
        let mut previous = SYST::get_current();
        for t in 0..ticks {
            let Some((a, b)) = sample(timeout) else {
                irq_moe::<ATIM>(false);
                defmt::error!("ADC timeout");
                stop()
            };
            let now = SYST::get_current();
            elapsed += previous.wrapping_sub(now) & 0xff_ffff;
            previous = now;
            let (ia, ib) = (za - a, zb - b);
            if (64..sounding - 64).contains(&t) {
                low = low.min(ia);
                high = high.max(ia);
            }
            peak = peak.max(ia.abs()).max(ib.abs()).max((ia + ib).abs());
            let bus_mv = u32::from(adc_irq_result::<ADC2>(0).unwrap()) * 55_000 / 4095;
            if peak > 128 || !(8000..=16000).contains(&bus_mv) {
                irq_moe::<ATIM>(false);
                defmt::error!("bridge off peak={} bus_mv={}", peak, bus_mv);
                stop();
            }
            if t == 0 {
                pwm.set_master_output_enable(true);
            }
            if t == sounding {
                pwm.set_master_output_enable(false);
            }
            let envelope = t.min(sounding.saturating_sub(t)).min(64) as i32;
            let magnitude = AMPLITUDE_Q15 * envelope / 64;
            let alpha = if phase < 32768 { magnitude } else { -magnitude };
            phase = phase.wrapping_add(step as u16);
            let duty = svpwm(alpha, 0);
            for (ch, d) in [
                (Channel::Ch1, duty.a),
                (Channel::Ch2, duty.b),
                (Channel::Ch3, duty.c),
            ] {
                pwm.set_duty_cycle(ch, (u32::from(d) * period) >> 15);
            }
        }
        ripple[i] = high - low;
        note_ms[i] = elapsed / cycles_per_ms;
    }
    pwm.set_master_output_enable(false);
    for (pp, ms) in ripple.chunks(16).zip(note_ms.chunks(16)) {
        defmt::info!("A phase peak-to-peak={=?} note_ms={=?}", pp, ms);
    }
    defmt::info!(
        "melody complete notes={} elapsed_ms={} peak_counts={} bridge off",
        NOTE_COUNT,
        note_ms.iter().sum::<u32>(),
        peak
    );
    stop()
}
