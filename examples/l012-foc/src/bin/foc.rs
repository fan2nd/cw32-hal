#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cw32_l012_foc::foc::{Duty, FocController, JD4631, PhaseCurrent, SpeedController};
use embassy_cw32::{
    self as hal,
    adc::{Adc, Common, Config as AdcConfig, SampleTime},
    cordic::{Cordic, Q1_31},
    gpio::{Flex, Speed},
    mode::Blocking,
    peripherals,
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

const PWM_HZ: u32 = 16_000;
const ADC_FULL_SCALE_COUNTS: i32 = 2_048;
const ALIGNMENT_PERIODS: u32 = 16_000 / 5; // 200 ms at 16 kHz.
const READY_ZERO_SAMPLES: u32 = 1_600; // 100 ms at the 16 kHz control tick.
const POT_ZERO_COUNTS: u16 = 40;
const POT_MAX_RPM: i32 = 800; // 68 rpm/V * 12 V, with headroom.
const SPEED_LOOP_DIV: u32 = 16; // 1 kHz speed loop from the 16 kHz current loop.
const DIRECTION_TEST_PERIODS: u32 = PWM_HZ / 50; // 20 ms, low-current point test.
const DIRECTION_TEST_IQ_Q15: i16 = 2_500; // About 8% of the normalized current.
const DIRECTION_MIN_DELTA_COUNTS: i32 = 4;
const BUS_MIN_MV: u32 = 8_000;
const BUS_MAX_MV: u32 = 16_000;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

struct Encoder<'d> {
    spi: Spi<'d, Blocking>,
    cs: Flex<'d>,
}

impl<'d> Encoder<'d> {
    fn read_angle(&mut self) -> Result<u16, spi::Error> {
        let mut frame = [0u8; 3];
        self.cs.set_low();
        let result = self.spi.blocking_transfer_in_place(&mut frame);
        self.cs.set_high();
        result?;
        let raw = (u16::from(frame[0]) << 6) | (u16::from(frame[1]) >> 2);
        Ok((JD4631.encoder_counts_per_rev - (raw & 0x3fff)) & 0x3fff)
    }
}

fn wait_period<T: hal::timer::OverflowInstance>(timer: &mut Timer<'_, T>) {
    while !timer.take_overflow() {
        core::hint::spin_loop();
    }
}

fn q15_from_adc(sample: u16, offset: u16) -> i16 {
    let signed = i32::from(offset) - i32::from(sample);
    (signed * 32_767 / ADC_FULL_SCALE_COUNTS).clamp(-32_767, 32_767) as i16
}

fn electrical_angle(mechanical: u16, offset: u16) -> u16 {
    let mechanical = u32::from(mechanical.wrapping_sub(offset) & 0x3fff);
    ((mechanical * u32::from(JD4631.pole_pairs)) & 0x3fff) as u16
}

fn angle_delta(current: u16, previous: u16) -> i32 {
    let delta = i32::from(current) - i32::from(previous);
    if delta > 8_192 {
        delta - 16_384
    } else if delta < -8_192 {
        delta + 16_384
    } else {
        delta
    }
}

fn sin_cos_q15(cordic: &mut Cordic<'_, peripherals::CORDIC>, angle: u16) -> (i16, i16) {
    // CORDIC accepts angle/pi in Q1.31.  One electrical revolution is 2*pi.
    let signed = if angle >= 8192 {
        i32::from(angle) - 16384
    } else {
        i32::from(angle)
    };
    let normalized = signed << 18;
    let waveform = cordic.sin_cos(Q1_31::from_bits(normalized), 1_024).unwrap();
    (
        (waveform.sin.to_bits() >> 16) as i16,
        (waveform.cos.to_bits() >> 16) as i16,
    )
}

fn apply_duty(pwm: &mut ComplementaryPwm<'_, peripherals::ATIM>, duty: Duty) {
    let max = pwm.get_max_duty();
    pwm.set_duty(Channel::Ch1, u32::from(duty.a) * max / 32_767);
    pwm.set_duty(Channel::Ch2, u32::from(duty.b) * max / 32_767);
    pwm.set_duty(Channel::Ch3, u32::from(duty.c) * max / 32_767);
}

#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default());
    let common = Common::new(p.BGR, 100_000).unwrap();
    let mut adc_config = AdcConfig::default();
    adc_config.vdda_mv = 3_300;
    adc_config.frequency = Hertz(6_000_000);
    let mut adc1 = Adc::new(p.ADC1, &common, adc_config).unwrap();
    let mut adc2 = Adc::new(p.ADC2, &common, adc_config).unwrap();
    let mut ia_pin = p.PB0;
    let mut ib_pin = p.PB1;
    let mut bus_pin = p.PA8;
    let mut pot_pin = p.PA10;

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
        pwm.set_duty(channel, pwm.get_max_duty() / 2);
        pwm.enable(channel);
    }
    // Keep the power stage off while current offsets are sampled and until the
    // operator performs the zero-pot arming gesture below.
    pwm.set_master_output_enable(false);

    let mut tick = Timer::new(p.BTIM1).unwrap();
    tick.set_frequency(Hertz(PWM_HZ));
    tick.start();

    let mut cs = Flex::new(p.PF0);
    cs.set_high();
    cs.set_as_output(Speed::Default);
    let mut spi_config = spi::Config::default();
    spi_config.mode = MODE_1;
    spi_config.frequency = Hertz(4_000_000);
    let spi = Spi::new_blocking(p.SPI3, p.PB8, p.PA11, p.PC14, spi_config).unwrap();
    let mut encoder = Encoder { spi, cs };
    let mut cordic = Cordic::new(p.CORDIC).unwrap();

    // The bridge stays disabled while offsets are measured and while the rotor
    // is held at the electrical zero used for encoder alignment.
    let mut offset_a = 0u32;
    let mut offset_b = 0u32;
    for _ in 0..128 {
        offset_a += u32::from(
            adc1.blocking_read(&mut ia_pin, SampleTime::Cycles30)
                .unwrap(),
        );
        offset_b += u32::from(
            adc1.blocking_read(&mut ib_pin, SampleTime::Cycles30)
                .unwrap(),
        );
    }
    let offset_a = (offset_a / 128) as u16;
    let offset_b = (offset_b / 128) as u16;

    // The power stage is kept disabled until the operator deliberately brings
    // the command pot to zero. This is the arming gesture required at power-on.
    let mut zero_samples: u32 = 0;
    loop {
        pwm.set_master_output_enable(false);
        let pot = adc2
            .blocking_read(&mut pot_pin, SampleTime::Cycles54)
            .unwrap();
        if pot <= POT_ZERO_COUNTS {
            zero_samples = zero_samples.saturating_add(1);
            if zero_samples >= READY_ZERO_SAMPLES {
                break;
            }
        } else {
            zero_samples = 0;
        }
        wait_period(&mut tick);
    }

    pwm.set_master_output_enable(true);
    let alignment = Duty {
        a: 20_000,
        b: 14_566,
        c: 14_566,
    };
    for _ in 0..ALIGNMENT_PERIODS {
        apply_duty(&mut pwm, alignment);
        wait_period(&mut tick);
    }
    let encoder_offset = encoder.read_angle().unwrap_or(0);
    let mut foc = FocController::new(JD4631);

    // Probe the sign of the motor/encoder combination with a small positive
    // torque request. The encoder delta tells us whether positive Iq produces
    // positive or negative mechanical motion. A stationary rotor falls back to
    // the board's nominal sign and can be retried after reducing friction/load.
    foc.iq_reference = DIRECTION_TEST_IQ_Q15;
    let mut probe_angle = encoder_offset;
    let mut probe_delta = 0i32;
    for _ in 0..DIRECTION_TEST_PERIODS {
        wait_period(&mut tick);
        let ia = adc1
            .blocking_read(&mut ia_pin, SampleTime::Cycles30)
            .unwrap();
        let ib = adc1
            .blocking_read(&mut ib_pin, SampleTime::Cycles30)
            .unwrap();
        let bus = adc2
            .blocking_read(&mut bus_pin, SampleTime::Cycles54)
            .unwrap();
        let bus_mv = u32::from(bus) * 3_300 * 11 / 4_095;
        if !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv) {
            pwm.set_master_output_enable(false);
            loop {
                cortex_m::asm::wfi();
            }
        }
        let mechanical_angle = encoder.read_angle().unwrap_or(probe_angle);
        probe_delta += angle_delta(mechanical_angle, probe_angle);
        probe_angle = mechanical_angle;
        let angle = electrical_angle(mechanical_angle, encoder_offset);
        let (sin, cos) = sin_cos_q15(&mut cordic, angle);
        let duty = foc.update(
            PhaseCurrent {
                a: q15_from_adc(ia, offset_a),
                b: q15_from_adc(ib, offset_b),
            },
            sin,
            cos,
        );
        apply_duty(&mut pwm, duty);
    }
    let direction_sign = if probe_delta >= DIRECTION_MIN_DELTA_COUNTS {
        1i16
    } else if probe_delta <= -DIRECTION_MIN_DELTA_COUNTS {
        -1i16
    } else {
        1i16
    };
    foc.iq_reference = 0;
    foc.reset();

    let mut speed = SpeedController::new(POT_MAX_RPM, 18_000);
    let mut last_angle = probe_angle;
    let mut speed_delta_sum = 0i32;
    let mut speed_tick = 0u32;

    loop {
        wait_period(&mut tick);
        let ia = adc1
            .blocking_read(&mut ia_pin, SampleTime::Cycles30)
            .unwrap();
        let ib = adc1
            .blocking_read(&mut ib_pin, SampleTime::Cycles30)
            .unwrap();
        let bus = adc2
            .blocking_read(&mut bus_pin, SampleTime::Cycles54)
            .unwrap();
        let bus_mv = u32::from(bus) * 3_300 * 11 / 4_095;
        if !(BUS_MIN_MV..=BUS_MAX_MV).contains(&bus_mv) {
            pwm.set_master_output_enable(false);
            loop {
                cortex_m::asm::wfi();
            }
        }
        let mechanical_angle = encoder.read_angle().unwrap_or(last_angle);
        speed_delta_sum += angle_delta(mechanical_angle, last_angle);
        last_angle = mechanical_angle;
        speed_tick += 1;
        if speed_tick >= SPEED_LOOP_DIV {
            let feedback_rpm = speed_delta_sum * 60 * PWM_HZ as i32
                / (i32::from(JD4631.encoder_counts_per_rev) * SPEED_LOOP_DIV as i32);
            let pot = adc2
                .blocking_read(&mut pot_pin, SampleTime::Cycles54)
                .unwrap();
            if pot <= POT_ZERO_COUNTS {
                // A zero command is an explicit stop request. Clear the speed
                // integrator so a previous positive command cannot restart the
                // motor while the pot is still at zero.
                speed.reset();
                foc.iq_reference = 0;
            } else {
                let target_rpm = i32::from(pot) * POT_MAX_RPM / 4_095;
                foc.iq_reference = speed.update(target_rpm, feedback_rpm) * direction_sign;
            }
            speed_delta_sum = 0;
            speed_tick = 0;
        }
        let angle = electrical_angle(mechanical_angle, encoder_offset);
        let (sin, cos) = sin_cos_q15(&mut cordic, angle);
        let duty = foc.update(
            PhaseCurrent {
                a: q15_from_adc(ia, offset_a),
                b: q15_from_adc(ib, offset_b),
            },
            sin,
            cos,
        );
        apply_duty(&mut pwm, duty);
    }
}
