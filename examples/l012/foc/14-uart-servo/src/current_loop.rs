//! ADC1 current-loop interrupt and the power-stage output gate.
use crate::config::{CURRENT_TRIP_COUNTS, PROBE_VOLTAGE_LIMIT_Q15};
use crate::global::*;
use crate::hardware::sin_cos_q15_lut;
use core::sync::atomic::Ordering;
use embassy_cw32 as hal;
use embassy_cw32::adc::{adc_irq_clear, adc_irq_enable_eos, adc_irq_eos, adc_irq_result};
use embassy_cw32::interrupt::typelevel::{ADC1, Handler};
use embassy_cw32::timer::Channel;

/// The outer loop requests enable; the ADC ISR primes CCR before setting MOE.
pub struct Adc1CurrentLoop;

impl Handler<ADC1> for Adc1CurrentLoop {
    unsafe fn on_interrupt() {
        if !adc_irq_eos::<hal::peripherals::ADC1>() {
            return;
        }
        // EOS is a level request on this ADC. Mask it before touching the
        // current-loop state so a pending/sticky flag cannot immediately
        // re-enter the handler and starve the outer loop.
        adc_irq_enable_eos::<hal::peripherals::ADC1>(false);
        let ia = adc_irq_result::<hal::peripherals::ADC1>(0).unwrap_or(0);
        let ib = adc_irq_result::<hal::peripherals::ADC1>(1).unwrap_or(0);
        CURRENT_ADC_A.store(ia, Ordering::Relaxed);
        CURRENT_ADC_B.store(ib, Ordering::Relaxed);
        adc_irq_clear::<hal::peripherals::ADC1>();

        let count = CURRENT_LOOP_COUNT.load(Ordering::Relaxed);
        CURRENT_LOOP_COUNT.store(count.wrapping_add(1), Ordering::Relaxed);

        // The ADC trigger remains armed while the outer-loop state machine is
        // waiting for a nonzero speed command. Keep the power stage hard-off
        // in that state instead of allowing stale PI output to reach CCRx.
        // During alignment the startup code owns the fixed duty vector; the
        // current ISR must drain EOS without clearing MOE or replacing it.
        let capture = count & 7 == 0 && crate::wave::enabled();
        if CURRENT_ALIGNMENT_ACTIVE.load(Ordering::Relaxed) != 0 {
            if capture {
                crate::wave::push(None);
            }
            adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
            return;
        }
        let output = CURRENT_OUTPUT_ENABLE.load(Ordering::Acquire);
        if output == 0 || CURRENT_FAULT.load(Ordering::Relaxed) != 0 {
            hal::timer::complementary_pwm::irq_moe::<hal::peripherals::ATIM>(false);
            if capture {
                crate::wave::push(None);
            }
            adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
            return;
        }

        // Predict the angle at the exact current-loop instant. The SysTick
        // SSI read periodically re-anchors this accumulator; between reads the
        // ISR advances it by the measured electrical speed.
        let step = CURRENT_ANGLE_STEP.load(Ordering::Relaxed) as u16;
        let angle = CURRENT_ELECTRICAL_ANGLE
            .load(Ordering::Relaxed)
            .wrapping_add(u32::from(step)) as u16;
        CURRENT_ELECTRICAL_ANGLE.store(u32::from(angle), Ordering::Relaxed);
        CURRENT_APPLIED_ANGLE.store(angle, Ordering::Relaxed);
        let (sin, cos) = sin_cos_q15_lut(angle);
        let iq_reference = CURRENT_IQ_REFERENCE.load(Ordering::Relaxed);
        let duty = cortex_m::interrupt::free(|cs| {
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
            if currents.a.unsigned_abs() > CURRENT_TRIP_COUNTS
                || currents.b.unsigned_abs() > CURRENT_TRIP_COUNTS
            {
                record_stop(StopReason::PhaseCurrent);
                CURRENT_FAULT.store(1, Ordering::Release);
                hal::timer::complementary_pwm::irq_moe::<hal::peripherals::ATIM>(false);
                return None;
            }
            controller.iq_reference = iq_reference;
            controller.voltage_limit = if CURRENT_PROBE_ACTIVE.load(Ordering::Relaxed) != 0 {
                PROBE_VOLTAGE_LIMIT_Q15
            } else {
                27_000
            };
            let duty = controller.update(currents, sin, cos);
            CURRENT_ID.store(controller.measured_id, Ordering::Relaxed);
            CURRENT_IQ.store(controller.measured_iq, Ordering::Relaxed);
            Some((
                duty,
                [
                    currents.a,
                    currents.b,
                    controller.measured_id,
                    controller.measured_iq,
                ],
            ))
        });
        let Some((duty, currents)) = duty else {
            if capture {
                crate::wave::push(None);
            }
            adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
            return;
        };
        let total = CURRENT_LOOP_TOTAL.load(Ordering::Relaxed);
        CURRENT_LOOP_TOTAL.store(total.wrapping_add(1), Ordering::Relaxed);
        let period = hal::timer::complementary_pwm::irq_arr::<hal::peripherals::ATIM>();
        let ccr =
            [duty.a, duty.b, duty.c].map(|d| ((u32::from(d) * period) >> 15).min(65_535) as u16);
        for (channel, compare) in [Channel::Ch1, Channel::Ch2, Channel::Ch3]
            .into_iter()
            .zip(ccr)
        {
            hal::timer::complementary_pwm::irq_ccr::<hal::peripherals::ATIM>(channel, compare);
        }
        if capture {
            let sample = crate::wave::Sample { currents, ccr };
            crate::wave::push((output == OUTPUT_RUNNING).then_some((sample, period as u16)));
        }
        // CCR writes are preloaded. Wait one full ADC trigger interval for
        // this start's duties to latch before connecting the power stage.
        if CURRENT_FAULT.load(Ordering::Relaxed) == 0 {
            if output == OUTPUT_PRIME {
                CURRENT_OUTPUT_ENABLE.store(OUTPUT_ENABLE_NEXT, Ordering::Release);
            } else if output == OUTPUT_ENABLE_NEXT {
                hal::timer::complementary_pwm::irq_moe::<hal::peripherals::ATIM>(true);
                CURRENT_OUTPUT_ENABLE.store(OUTPUT_RUNNING, Ordering::Release);
            }
        }
        adc_irq_enable_eos::<hal::peripherals::ADC1>(true);
    }
}

hal::bind_interrupts!(pub struct Irqs {
    ADC1 => Adc1CurrentLoop;
});
