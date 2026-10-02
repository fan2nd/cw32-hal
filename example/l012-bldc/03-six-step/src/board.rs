//! Sampling-only board: OPA1/BGR, two ADCs, ATIM CH4 and the 1 ms timer.
//!
//! ATIM is only an internal 20 kHz ADC trigger. CH1–3 are disconnected,
//! MOE remains clear. The six gate pins are untouched; there is no arm API.
//! The ADC clock is 6 MHz, with 22 us / 444.167 us sequential conversions.
//! An EOS arriving while the next ADC1 sequence is busy latches a sampling
//! fault and stops that stream. One EOS flag cannot count completely missed
//! sequences: IRQ latency and analog settling still need hardware validation.

use crate::sampling::{
    ADC1_SEQUENCE_NS, ADC2_SEQUENCE_NS, ADC_HZ, CPU_HZ, SAMPLING_HZ, SAMPLING_PERIOD,
};
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    gpio::AnyPin,
    interrupt::{self, InterruptExt},
    pac, peripherals, rcc, Peri,
};

const _: () = assert!(SAMPLING_PERIOD == 2400 && ADC1_SEQUENCE_NS == 22_000);
const _: () = assert!(ADC1_SEQUENCE_NS < 1_000_000_000 / SAMPLING_HZ);
const _: () = assert!(ADC2_SEQUENCE_NS < 5_000_000 && ADC_HZ / 33 <= 200_000);
const ADC_ICR_MASK: u32 = 0xf;
const BTIM_ICR_MASK: u32 = 0x41;
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Owns exactly the sampling peripherals, analog pins and inherited LED/key.
/// No drive permission can be acquired through this type.
pub struct SamplingBoard {
    _atim: Peri<'static, peripherals::ATIM>,
    _adc1: Peri<'static, peripherals::ADC1>,
    _adc2: Peri<'static, peripherals::ADC2>,
    _opa1: Peri<'static, peripherals::OPA1>,
    _bandgap: Peri<'static, peripherals::BGR>,
    _millisecond: Peri<'static, peripherals::BTIM1>,
    _pins: [Peri<'static, AnyPin>; 12],
    sampling_fault: bool,
}

impl SamplingBoard {
    pub fn new() -> Result<Self, embassy_cw32::InitError> {
        let mut config = embassy_cw32::Config::default();
        config.rcc.hsi_frequency = rcc::HsiFrequency::Mhz96;
        config.rcc.pclk_divider = rcc::PclkDivider::Div2;
        let p = embassy_cw32::try_init(config)?;
        for irq in [interrupt::ADC1, interrupt::ADC2_DAC, interrupt::BTIM1] {
            irq.disable();
        }
        // SAFETY: HAL initialization transferred the peripheral singletons;
        // handlers are not enabled until the caller installs the board owner.
        unsafe {
            pac::modify(
                pac::SYSCTRL_BASE + pac::sysctrl::AHBEN,
                pac::SYSCTRL_KEY_MASK,
                pac::SYSCTRL_KEY | (1 << 4) | (1 << 5) | (1 << 6),
            );
            pac::modify(
                pac::SYSCTRL_BASE + pac::sysctrl::APBEN1,
                pac::SYSCTRL_KEY_MASK,
                pac::SYSCTRL_KEY | (1 << 0) | (1 << 5),
            );
            // Basic timers share this clock gate; only timer 1 is accessed.
            pac::modify(
                pac::SYSCTRL_BASE + pac::sysctrl::APBEN2,
                pac::SYSCTRL_KEY_MASK,
                pac::SYSCTRL_KEY | (1 << 2) | (1 << 9),
            );
            configure_pin(pac::GPIOC_BASE, 13, PinMode::OutputHigh);
            configure_pin(pac::GPIOA_BASE, 3, PinMode::InputPullUp);
            for (port, pin) in [
                (pac::GPIOA_BASE, 0),
                (pac::GPIOA_BASE, 1),
                (pac::GPIOA_BASE, 2),
                (pac::GPIOA_BASE, 6),
                (pac::GPIOA_BASE, 7),
                (pac::GPIOB_BASE, 0),
                (pac::GPIOB_BASE, 2),
                (pac::GPIOA_BASE, 8),
                (pac::GPIOA_BASE, 10),
                (pac::GPIOA_BASE, 11),
            ] {
                configure_pin(port, pin, PinMode::Analog);
            }
            initialize_sampling_trigger();
            // External-feedback OPA1: PA6 INP2, PA7 INN2, PB0 output/ADC1 CH8.
            pac::BGR
                .cr()
                .write(pac::bgr::fields::cr::BGREN.write(pac::BGR.cr().read(), true));
            pac::OPA1.cal().write(0);
            pac::OPA1.cr().write(0xe221);
            initialize_adc(pac::ADC1, 4, 0x2108, 0x4444);
            initialize_adc(pac::ADC2, 5, 0xf875b, 0xfffff);
            initialize_millisecond_timer();
        }
        // Nominal >= 1 ms startup settling; not used as a sample timebase.
        cortex_m::asm::delay(CPU_HZ / 1_000);
        INITIALIZED.store(true, Ordering::Release);
        Ok(Self {
            _atim: p.ATIM,
            _adc1: p.ADC1,
            _adc2: p.ADC2,
            _opa1: p.OPA1,
            _bandgap: p.BGR,
            _millisecond: p.BTIM1,
            _pins: [
                p.PA0.into(),
                p.PA1.into(),
                p.PA2.into(),
                p.PA6.into(),
                p.PA7.into(),
                p.PB0.into(),
                p.PB2.into(),
                p.PA8.into(),
                p.PA10.into(),
                p.PA11.into(),
                p.PC13.into(),
                p.PA3.into(),
            ],
            sampling_fault: false,
        })
    }

    /// Configure sampling sources and P1 priorities, leaving NVIC masked.
    ///
    /// # Safety
    /// ADC1, ADC2_DAC and BTIM1 must remain NVIC-masked throughout this
    /// call and until the caller has installed their exclusive IRQ state.
    pub unsafe fn prepare_interrupts(&mut self) {
        unsafe {
            pac::ADC1.icr().write(0);
            pac::ADC2.icr().write(0);
            pac::ADC1.ier().write(pac::adc::fields::ier::EOS.mask());
            pac::ADC2.ier().write(pac::adc::fields::ier::EOS.mask());
            pac::BTIM1.icr().write(BTIM_ICR_MASK & !1);
            pac::BTIM1.dier().write(1);
            for irq in [interrupt::ADC1, interrupt::ADC2_DAC, interrupt::BTIM1] {
                irq.unpend();
                irq.set_priority(interrupt::Priority::P1);
            }
            pac::ADC1
                .trigger()
                .write(pac::adc::fields::trigger::ATIMOC4REFC.mask());
            pac::BTIM1.cr1().write(5); // EN, URS: overflow events only.
            pac::ATIM
                .cr1()
                .write(pac::atim::fields::cr1::ARPE.mask() | 1);
        }
        self.start_adc2();
    }

    pub fn sampling_fault(&self) -> bool {
        self.sampling_fault
    }

    pub fn button_pressed(&self) -> bool {
        unsafe { pac::read(pac::GPIOA_BASE + pac::gpio::IDR) & (1 << 3) == 0 }
    }

    pub fn set_led(&mut self, on: bool) {
        unsafe {
            pac::write(
                pac::GPIOC_BASE + if on { pac::gpio::BRR } else { pac::gpio::BSRR },
                1 << 13,
            );
        }
    }

    pub fn adc1_irq(&mut self) -> Option<[u16; 4]> {
        let r = pac::ADC1;
        unsafe {
            if self.sampling_fault || !pac::adc::fields::isr::EOS.read(r.isr().read()) {
                return None;
            }
            // Freeze future triggers before reading the sequence; a currently
            // active next sequence means this EOS can no longer be used safely.
            r.trigger().write(0);
            if pac::adc::fields::start::START.read(r.start().read()) {
                r.start().write(0);
                r.ier().write(0);
                r.icr()
                    .write(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
                self.sampling_fault = true;
                emergency_stop();
                return None;
            }
            let result = [
                r.result0().read() as u16 & 4095,
                r.result1().read() as u16 & 4095,
                r.result2().read() as u16 & 4095,
                r.result3().read() as u16 & 4095,
            ];
            r.icr()
                .write(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
            r.trigger()
                .write(pac::adc::fields::trigger::ATIMOC4REFC.mask());
            Some(result)
        }
    }

    pub fn adc2_irq(&mut self) -> Option<[u16; 5]> {
        let r = pac::ADC2;
        unsafe {
            if !pac::adc::fields::isr::EOS.read(r.isr().read())
                || pac::adc::fields::start::START.read(r.start().read())
            {
                return None;
            }
            let result = [
                r.result0().read() as u16 & 4095,
                r.result1().read() as u16 & 4095,
                r.result2().read() as u16 & 4095,
                r.result3().read() as u16 & 4095,
                r.result4().read() as u16 & 4095,
            ];
            r.icr()
                .write(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
            Some(result)
        }
    }

    pub fn ms_irq(&mut self) -> bool {
        unsafe {
            if pac::BTIM1.isr().read() & pac::BTIM1.dier().read() & 1 == 0 {
                return false;
            }
            pac::BTIM1.icr().write(BTIM_ICR_MASK & !1);
        }
        true
    }

    pub fn start_adc2(&mut self) {
        unsafe {
            // Never restart an active conversion or overwrite an unread tuple.
            if !pac::adc::fields::start::START.read(pac::ADC2.start().read())
                && !pac::adc::fields::isr::EOS.read(pac::ADC2.isr().read())
            {
                pac::ADC2.start().write(1);
            }
        }
    }
}

/// # Safety
/// The caller must install SAMPLING first and end all references to its state.
/// Only the equal-priority sampling handlers may subsequently access that state.
pub unsafe fn enable_interrupts() {
    for irq in [interrupt::ADC1, interrupt::ADC2_DAC, interrupt::BTIM1] {
        unsafe { irq.enable() };
    }
}

impl Drop for SamplingBoard {
    fn drop(&mut self) {
        emergency_stop();
        unsafe {
            pac::ADC1.trigger().write(0);
            pac::ADC2.trigger().write(0);
            pac::ADC1.start().write(0);
            pac::ADC2.start().write(0);
            pac::ADC1.ier().write(0);
            pac::ADC2.ier().write(0);
            pac::BTIM1.dier().write(0);
            pac::BTIM1.cr1().write(0);
            pac::ATIM.cr1().write(0);
        }
    }
}

/// Borrow-free shutdown is safe even if an exception interrupted an IRQ owner.
pub fn emergency_stop() {
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe {
            pac::ATIM.bdtr().write(0);
            pac::ATIM.ccer().write(pac::atim::fields::ccer::CC4E.mask());
        }
    }
}

enum PinMode {
    OutputHigh,
    InputPullUp,
    Analog,
}

unsafe fn configure_pin(port: usize, pin: u32, mode: PinMode) {
    let mask = 1 << pin;
    let af_offset = if pin < 8 {
        pac::gpio::AFRL
    } else {
        pac::gpio::AFRH
    };
    let af_shift = (pin % 8) * 4;
    unsafe {
        pac::modify(port + pac::gpio::DIR, 0, mask);
        pac::modify(port + af_offset, 0xf << af_shift, 0);
        pac::modify(port + pac::gpio::RISEIE, mask, 0);
        pac::modify(port + pac::gpio::FALLIE, mask, 0);
        pac::modify(port + pac::gpio::OPENDRAIN, mask, 0);
        pac::modify(
            port + pac::gpio::PUR,
            mask,
            if matches!(mode, PinMode::InputPullUp) {
                mask
            } else {
                0
            },
        );
        pac::modify(
            port + pac::gpio::ANALOG,
            mask,
            if matches!(mode, PinMode::Analog) {
                mask
            } else {
                0
            },
        );
        if matches!(mode, PinMode::OutputHigh) {
            // Latch the inactive level before connecting the output driver.
            pac::write(port + pac::gpio::BSRR, mask);
            pac::modify(port + pac::gpio::DIR, mask, 0);
        }
    }
}

unsafe fn initialize_sampling_trigger() {
    use pac::atim::fields as f;
    unsafe {
        pac::ATIM.cr1().write(0);
        pac::ATIM.bdtr().write(0);
        pac::ATIM.dier().write(0);
        pac::ATIM.ccer().write(0);
        pac::ATIM.cr2().write(0);
        pac::ATIM.smcr().write(0);
        pac::ATIM.psc().write(0);
        pac::ATIM.arr().write(u32::from(SAMPLING_PERIOD - 1));
        pac::ATIM.rcr().write(0);
        pac::ATIM.cnt().write(0);
        // No phase PWM mode, channel enable or alternate-function gate mux.
        pac::ATIM.ccmr1cmp().write(0);
        // OC4REFC rising edge occurs at CCR4 with PWM2, as in RM25.12.6.
        pac::ATIM
            .ccmr2cmp()
            .write(f::ccmr2cmp::OC4PE.write(f::ccmr2cmp::OC4M.write(0, 7), true));
        pac::ATIM.ccr1().write(0);
        pac::ATIM.ccr2().write(0);
        pac::ATIM.ccr3().write(0);
        pac::ATIM.ccr4().write(2000); // Source stopped count 4000 at half PCLK.
        pac::ATIM.dtr2().write(0);
        pac::ATIM.af1().write(0);
        pac::ATIM.af2().write(0);
        pac::ATIM.ccer().write(f::ccer::CC4E.mask());
        pac::ATIM.egr().write(1);
        pac::ATIM.icr().write(0);
    }
}

unsafe fn initialize_adc(r: pac::adc::RegisterBlock, length: u32, channels: u32, sample: u32) {
    use pac::adc::fields as f;
    unsafe {
        r.trigger().write(0);
        r.start().write(0);
        r.ier().write(0);
        // Preserve reserved CR bits; the uploaded SDK's SAM[9:8] disagrees
        // with the pinned official header / RM and is deliberately not used.
        let reserved = r.cr().read() & !0xff;
        r.cr().write(reserved);
        r.awdcr().write(0);
        r.sqrcfr().write(channels);
        r.sample().write(sample);
        r.icr().write(0);
        r.cr().write(f::cr::EN.write(
            f::cr::ENS.write(f::cr::CLK.write(reserved, 3), length - 1),
            true,
        ));
    }
}

unsafe fn initialize_millisecond_timer() {
    unsafe {
        pac::BTIM1.cr1().write(4);
        pac::BTIM1.dier().write(0);
        pac::BTIM1.cr2().write(0);
        pac::BTIM1.smcr().write(0);
        pac::BTIM1.psc().write(47);
        pac::BTIM1.arr().write(999);
        pac::BTIM1.cnt().write(0);
        pac::BTIM1.egr().write(1);
        pac::BTIM1.icr().write(0);
    }
}
