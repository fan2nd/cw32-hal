use core::fmt::{self, Write};
use core::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use cortex_m::peripheral::{Peripherals, SYST, syst::SystClkSource};
use embassy_cw32::{
    self as hal,
    gpio::{Input, Level, Output, Pull, Speed},
    pac::{ATIM, GPIOA, GTIM2, gtim::vals::CcMode},
    rcc,
    time::Hertz,
    timer::{
        low_level::CountingMode,
        qei::{Config, Qei},
        simple_pwm::{PwmPin, SimplePwm},
    },
};
use hal::{
    mode::Blocking,
    usart::{Config as UartConfig, UartTx},
};

const DISTANCE_UM: u32 = 10_000;
const ENCODER_CYCLES_PER_MOTOR_REV: i64 = 7;
const COUNTS_PER_SCREW_REV: i64 = ENCODER_CYCLES_PER_MOTOR_REV * 4 * 50;
const LEAD_UM: i64 = 500;
const fn counts(um: u32) -> i64 {
    (um as i64 * COUNTS_PER_SCREW_REV + LEAD_UM / 2) / LEAD_UM
}
const TARGET: i64 = counts(DISTANCE_UM);
const _: () = assert!(TARGET < 32_768);

// Inspect after stopping. Counts are signed relative to the starting position.
#[unsafe(no_mangle)]
static FINAL_COUNTS: AtomicI32 = AtomicI32::new(0);
#[unsafe(no_mangle)]
static MOVE_STATUS: AtomicU32 = AtomicU32::new(0);

fn wait_ms(tick: &mut SYST) {
    while !tick.has_wrapped() {}
}

pub enum Brake {
    Atim,
    ExternalGate,
}

struct Log<'d>(UartTx<'d, Blocking>);
impl Write for Log<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.0
            .blocking_write(text.as_bytes())
            .map_err(|_| fmt::Error)
    }
}

fn halt() -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

// Extend the 16-bit hardware count using signed differences at each 1 ms sample.
fn sample(raw: u16, previous: &mut u16, travelled: &mut i64) {
    *travelled += i64::from(raw.wrapping_sub(*previous) as i16);
    *previous = raw;
}

pub fn run(brake: Brake) -> ! {
    let p = hal::init(Default::default());
    let mut standby = Output::new(p.PB2, Level::Low, Speed::Low);
    let mut in1 = Output::new(p.PB0, Level::Low, Speed::Low);
    let _in2 = Output::new(p.PB1, Level::Low, Speed::Low);
    let mut pwm = SimplePwm::new3(
        p.ATIM,
        Some(PwmPin::new(p.PA5)),
        None,
        None,
        Hertz(20_000),
        CountingMode::EdgeAlignedUp,
    )
    .unwrap();
    let mut drive = pwm.ch1();
    drive.set_duty_cycle_fully_off();
    drive.enable();
    let encoder = Qei::new(p.GTIM2, p.PA0, p.PA1, Config::default()).unwrap();
    let _compare_pin = Output::new(p.PA2, Level::Low, Speed::High);
    let feedback = Input::new(p.PA6, Pull::Down);
    let mut log = Log(UartTx::new_blocking(p.UART1, p.PA8, UartConfig::default()).unwrap());
    GTIM2.cmmr().modify(|w| w.set_cc3m(CcMode::ForceLow));
    GPIOA.afrl().modify(|w| w.set_afr2(6));
    if matches!(brake, Brake::Atim) {
        GPIOA.afrl().modify(|w| w.set_afr6(7));
        ATIM.ch1cr().modify(|w| w.set_bksa(2)); // force CH1A low on break
        ATIM.fltr().modify(|w| {
            w.set_bkp(false);
            w.set_fltbk(0);
        });
        ATIM.dtr().modify(|w| {
            w.set_aoe(false);
            w.set_bke(true);
        });
    }
    let mut tick = Peripherals::take().unwrap().SYST;
    tick.set_clock_source(SystClkSource::Core);
    tick.set_reload(rcc::clocks().hclk.0 / 1000 - 1);
    tick.clear_current();
    tick.enable_counter();

    let mut cycle = 0u64;
    loop {
        // Run each interlock check and clear latched events with STBY low.
        let ready = match brake {
            Brake::Atim => {
                GTIM2.cmmr().modify(|w| w.set_cc3m(CcMode::ForceLow));
                wait_ms(&mut tick);
                ATIM.icr().write(|_| {});
                GTIM2.cmmr().modify(|w| w.set_cc3m(CcMode::ForceHigh));
                wait_ms(&mut tick);
                let break_seen = ATIM.isr().read().bif();
                GTIM2.cmmr().modify(|w| w.set_cc3m(CcMode::ForceLow));
                wait_ms(&mut tick);
                ATIM.icr().write(|_| {});
                ATIM.dtr().modify(|w| w.set_moe(true));
                wait_ms(&mut tick);
                break_seen && !ATIM.isr().read().bif() && ATIM.dtr().read().moe()
            }
            Brake::ExternalGate => {
                let mut gate_ok = true;
                for permit in [false, true] {
                    let mode = if permit {
                        CcMode::ForceHigh
                    } else {
                        CcMode::ForceLow
                    };
                    GTIM2.cmmr().modify(|w| w.set_cc3m(mode));
                    for pwm_high in [false, true] {
                        if pwm_high {
                            drive.set_duty_cycle_fully_on();
                        } else {
                            drive.set_duty_cycle_fully_off();
                        }
                        wait_ms(&mut tick);
                        gate_ok &= feedback.is_high() == (permit && pwm_high);
                    }
                }

                gate_ok
            }
        };
        if !ready {
            MOVE_STATUS.store(2, Ordering::Relaxed);
            drive.set_duty_cycle_fully_off();
            writeln!(log, "fault=brake_self_test\r").unwrap();
            halt();
        }
        let origin = encoder.count() as u16;
        GTIM2
            .ccr3()
            .write(|w| w.set_ccr(origin.wrapping_add(TARGET as u16)));
        GTIM2.icr().write(|_| {});
        GTIM2.cmmr().modify(|w| {
            w.set_cc3m(match brake {
                Brake::Atim => CcMode::CompareHigh,
                Brake::ExternalGate => CcMode::CompareLow,
            })
        });
        wait_ms(&mut tick);
        let armed = match brake {
            Brake::Atim => !ATIM.isr().read().bif() && ATIM.dtr().read().moe(),
            Brake::ExternalGate => feedback.is_high(),
        };
        if !armed || GTIM2.isr().read().cc3() {
            MOVE_STATUS.store(2, Ordering::Relaxed);
            drive.set_duty_cycle_fully_off();
            writeln!(log, "fault=brake_arm\r").unwrap();
            halt();
        }

        cycle += 1;
        MOVE_STATUS.store(0, Ordering::Relaxed);
        let mut previous = origin;
        let mut travelled = 0;
        let mut best = 0;
        let mut last_progress = 0;
        let mut approach = false;
        let mut status = 3;
        in1.set_high();
        drive.set_duty_cycle_percent(55);
        standby.set_high();
        for ms in 0..180_000 {
            wait_ms(&mut tick);
            sample(encoder.count() as u16, &mut previous, &mut travelled);
            let event = match brake {
                Brake::Atim => ATIM.isr().read().bif(),
                Brake::ExternalGate => GTIM2.isr().read().cc3(),
            };
            if event || travelled >= TARGET {
                status = if event && GTIM2.isr().read().cc3() {
                    1
                } else {
                    2
                };
                break;
            }
            if travelled > best {
                best = travelled;
                last_progress = ms;
            }
            if ms - last_progress >= 500 || travelled < -counts(2) {
                status = 4;
                break;
            }
            if !approach && TARGET - travelled <= counts(20) {
                drive.set_duty_cycle_percent(25);
                approach = true;
            }
        }
        drive.set_duty_cycle_fully_off();
        // Short brake for the first 100 ms of the 2 s pause, then standby.
        for ms in 1..=2000 {
            wait_ms(&mut tick);
            sample(encoder.count() as u16, &mut previous, &mut travelled);
            if ms == 100 {
                standby.set_low();
            }
        }
        FINAL_COUNTS.store(travelled as i32, Ordering::Relaxed);
        MOVE_STATUS.store(status, Ordering::Relaxed);
        // Round the encoder estimate to the nearest nm before printing.
        let numerator = travelled * LEAD_UM * 1000;
        let measured_nm =
            (numerator + numerator.signum() * (COUNTS_PER_SCREW_REV / 2)) / COUNTS_PER_SCREW_REV;
        let error_nm = measured_nm - i64::from(DISTANCE_UM) * 1000;
        writeln!(
            log,
            "cycle={} target_um={} counts={} measured_nm={} error_nm={:+} status={}\r",
            cycle, DISTANCE_UM, travelled, measured_nm, error_nm, status
        )
        .unwrap();
        if status != 1 {
            halt();
        }
    }
}
