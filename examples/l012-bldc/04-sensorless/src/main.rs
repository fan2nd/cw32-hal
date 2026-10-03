//! Sampling is shown directly in main and the three equal-priority IRQs.
//! ATIM CH4 is an internal ADC trigger; bridge gate pins are untouched.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    interrupt::{self, InterruptExt},
    pac, rcc,
};

const CPU_HZ: u32 = 96_000_000;
const SAMPLING_PERIOD: u16 = 4800; // 96 MHz / 20 kHz, original scale.
const ADC_ICR_MASK: u32 = 0xf;
const BTIM_ICR_MASK: u32 = 0x41;
// DMA is the only writer after setup. CPU uses raw volatile reads, never Rust references.
static mut ADC2_DMA: [u32; 5] = [0; 5];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// Only ADC1 and BTIM1 access these observation variables after unmask.
// Both run at P1 and cannot preempt each other. Main never reads them.
// NMI/HardFault touch no software state and shut hardware down before diverging.
// No other handler may access these variables or change the IRQ priorities.
static mut ADC1_RAW: [u16; 4] = [0; 4];
static mut ADC2_RAW: [u16; 5] = [0; 5];
static mut ADC1_SEQUENCES: u32 = 0;
static mut ADC2_SEQUENCES: u32 = 0;
static mut MILLISECONDS: u32 = 0;
static mut ADC2_ELAPSED_MS: u8 = 0;

mod six_step;
use six_step::{Bridge, DEMONSTRATION_DUTY, KEY_DEBOUNCE_MS};
static mut SECTOR: u8 = 0;
static mut KEY_HELD_MS: u8 = 0;
static mut LOGICAL_BRIDGE: Bridge = Bridge::commutation(0, DEMONSTRATION_DUTY);

mod sensorless;
use sensorless::ZeroCrossingDetector;
static mut DETECTOR: ZeroCrossingDetector = ZeroCrossingDetector::new();
static mut OBSERVED_CROSSINGS: u32 = 0;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_frequency = rcc::HsiFrequency::Mhz96;
    config.rcc.pclk_divider = rcc::PclkDivider::Div1;
    // Retain the HAL singleton tokens for the lifetime of this program.
    let _peripherals = embassy_cw32::try_init(config).expect("clock initialization failed");
    for irq in [interrupt::ADC1, interrupt::BTIM1] {
        irq.disable();
    }
    // SAFETY: HAL initialization transferred the peripheral singletons;
    // main keeps the singleton tokens; it never accesses hardware after unmask.
    unsafe {
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::AHBEN,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | 1 | (1 << 4) | (1 << 5) | (1 << 6),
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
        // LED/key plus analog inputs only; bridge gate pins are untouched.
        for (port, pin, analog, pull_up, output) in [
            (pac::GPIOC_BASE, 13, false, false, true),
            (pac::GPIOA_BASE, 3, false, true, false),
            (pac::GPIOA_BASE, 0, true, false, false),
            (pac::GPIOA_BASE, 1, true, false, false),
            (pac::GPIOA_BASE, 2, true, false, false),
            (pac::GPIOA_BASE, 6, true, false, false),
            (pac::GPIOA_BASE, 7, true, false, false),
            (pac::GPIOB_BASE, 0, true, false, false),
            (pac::GPIOB_BASE, 2, true, false, false),
            (pac::GPIOA_BASE, 8, true, false, false),
            (pac::GPIOA_BASE, 10, true, false, false),
            (pac::GPIOA_BASE, 11, true, false, false),
        ] {
            let mask = 1 << pin;
            let af = if pin < 8 {
                pac::gpio::AFRL
            } else {
                pac::gpio::AFRH
            };
            pac::modify(port + pac::gpio::DIR, 0, mask);
            pac::modify(port + af, 0xf << ((pin % 8) * 4), 0);
            pac::modify(port + pac::gpio::RISEIE, mask, 0);
            pac::modify(port + pac::gpio::FALLIE, mask, 0);
            pac::modify(port + pac::gpio::OPENDRAIN, mask, 0);
            pac::modify(port + pac::gpio::PUR, mask, if pull_up { mask } else { 0 });
            pac::modify(
                port + pac::gpio::ANALOG,
                mask,
                if analog { mask } else { 0 },
            );
            if output {
                pac::write(port + pac::gpio::BSRR, mask);
                pac::modify(port + pac::gpio::DIR, mask, 0);
            }
        }

        pac::ATIM
            .cr1()
            .write_value(pac::atim::fields::cr1::ARPE.mask());
        pac::ATIM.bdtr().write_value(0);
        pac::ATIM.dier().write_value(0);
        pac::ATIM.ccer().write_value(0);
        pac::ATIM.cr2().write_value(0);
        pac::ATIM.smcr().write_value(0);
        pac::ATIM.psc().write_value(0);
        pac::ATIM.arr().write_value(u32::from(SAMPLING_PERIOD - 1));
        pac::ATIM.rcr().write_value(0);
        pac::ATIM.cnt().write_value(0);
        // No phase PWM mode, channel enable or alternate-function gate mux.
        pac::ATIM.ccmr1cmp().write_value(0);
        // Preserve original PWM1: OC4REFC rises at reload, not at CCR4.
        pac::ATIM.ccmr2cmp().write_value(
            pac::atim::fields::ccmr2cmp::OC4PE
                .write(pac::atim::fields::ccmr2cmp::OC4M.write(0, 6), true),
        );
        pac::ATIM.ccr1().write_value(0);
        pac::ATIM.ccr2().write_value(0);
        pac::ATIM.ccr3().write_value(0);
        pac::ATIM.ccr4().write_value(2400); // Original PWM_PERIOD / 2 at initialization.
        pac::ATIM.dtr2().write_value(0);
        pac::ATIM.af1().write_value(0);
        pac::ATIM.af2().write_value(0);
        pac::ATIM
            .ccer()
            .write_value(pac::atim::fields::ccer::CC4E.mask());
        pac::ATIM.icr().write_value(0);

        // External-feedback OPA1: PA6 INP2, PA7 INN2, PB0 output/ADC1 CH8.
        pac::BGR
            .cr()
            .write_value(pac::bgr::fields::cr::BGREN.write(pac::BGR.cr().read(), true));
        pac::OPA1.cr().write_value(0xe220);
        pac::OPA1.cal().write_value(0);
        pac::OPA1.cr().write_value(0xe221);
        for (r, length, channels, sample, divider) in [
            (pac::ADC1, 4, 0x2108, 0x9999, 1),
            (pac::ADC2, 5, 0xf875b, 0xfffff, 3),
        ] {
            r.trigger().write_value(0);
            r.start().write_value(0);
            r.ier().write_value(0);
            // Preserve reserved CR bits; the uploaded SDK's SAM[9:8] disagrees
            // with the pinned official header / RM and is deliberately not used.
            let reserved = r.cr().read() & !0xff;
            r.cr().write_value(reserved);
            r.awdcr().write_value(0);
            r.sqrcfr().write_value(channels);
            r.sample().write_value(sample);
            r.icr().write_value(0);
            r.cr().write_value(pac::adc::fields::cr::EN.write(
                pac::adc::fields::cr::ENS.write(
                    pac::adc::fields::cr::CLK.write(reserved, divider),
                    length - 1,
                ),
                true,
            ));
        }

        pac::BTIM1.cr1().write_value(0);
        pac::BTIM1.dier().write_value(0);
        pac::BTIM1.cr2().write_value(0);
        pac::BTIM1.smcr().write_value(0);
        pac::BTIM1.psc().write_value(95);
        pac::BTIM1.arr().write_value(999);
        pac::BTIM1.cnt().write_value(0);
        pac::BTIM1.icr().write_value(0);
    }
    // Nominal >= 1 ms startup settling; not used as a sample timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    INITIALIZED.store(true, Ordering::Release);

    unsafe {
        (*core::ptr::addr_of_mut!(DETECTOR)).begin_sector(0, 2);

        pac::ADC1.icr().write_value(0);
        pac::ADC2.icr().write_value(0);
        pac::ADC1
            .ier()
            .write_value(pac::adc::fields::ier::EOS.mask());
        // Original EOC + BLOCK intent: one 32-bit result per conversion.
        // Defined correction: ADC2_SINGLE (15), not source's mismatched SEQ (14).
        // EOS DMA is explicitly disabled; CNT=5, REPEAT=1, both addresses increment.
        pac::DMA.csr2().write_value(0);
        pac::DMA.cnt2().write_value((1 << 16) | 5);
        pac::DMA
            .srcaddr2()
            .write_value((pac::ADC2_BASE + pac::adc::RESULT0) as u32);
        pac::DMA
            .dstaddr2()
            .write_value(core::ptr::addr_of_mut!(ADC2_DMA).cast::<u32>() as u32);
        pac::DMA.trig2().write_value(1 | (15 << 2));
        pac::DMA
            .csr2()
            .write_value((1 << 11) | (2 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | 1);
        pac::ADC2
            .ier()
            .write_value(pac::adc::fields::ier::DMAEOC.mask());
        pac::ADC2
            .trigger()
            .write_value(pac::adc::fields::trigger::ATIMOC4REFC.mask());

        pac::BTIM1.icr().write_value(BTIM_ICR_MASK & !1);
        pac::BTIM1.dier().write_value(1);
        for irq in [interrupt::ADC1, interrupt::BTIM1] {
            irq.unpend();
            irq.set_priority(interrupt::Priority::P1);
        }
        pac::ADC1
            .trigger()
            .write_value(pac::adc::fields::trigger::ATIMOC4REFC.mask());
        pac::BTIM1.cr1().write_value(1); // Original timer enable.
        pac::ATIM
            .cr1()
            .write_value(pac::atim::fields::cr1::ARPE.mask() | 1);

        pac::ADC2.start().write_value(1);
        // Initialization and all borrows finish before any IRQ can run.
        core::sync::atomic::compiler_fence(Ordering::Release);
        for irq in [interrupt::ADC1, interrupt::BTIM1] {
            irq.enable();
        }
    }
    loop {
        cortex_m::asm::wfi();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC1() {
    // SAFETY: only the non-nesting P1 handlers access these static variables.
    unsafe {
        let r = pac::ADC1;
        if !pac::adc::fields::isr::EOS.read(r.isr().read()) {
            return;
        }
        r.icr().write_value(0);
        ADC1_RAW = [
            r.result0().read() as u16 & 4095,
            r.result1().read() as u16 & 4095,
            r.result2().read() as u16 & 4095,
            r.result3().read() as u16 & 4095,
        ];
        ADC1_SEQUENCES = ADC1_SEQUENCES.wrapping_add(1);
        let detector = &mut *core::ptr::addr_of_mut!(DETECTOR);
        // Read the live DMA bus slot, as the C detector does.
        let bus =
            core::ptr::read_volatile(core::ptr::addr_of!(ADC2_DMA).cast::<u32>().add(1)) as u16;
        if detector.observe(ADC1_RAW, bus) {
            OBSERVED_CROSSINGS = OBSERVED_CROSSINGS.wrapping_add(1);
            pac::write(pac::GPIOC_BASE + pac::gpio::BRR, 1 << 13);
        }
        core::hint::black_box(&*core::ptr::addr_of!(OBSERVED_CROSSINGS));
        core::hint::black_box((
            &*core::ptr::addr_of!(ADC1_RAW),
            &*core::ptr::addr_of!(ADC1_SEQUENCES),
        ));
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM1() {
    unsafe {
        if pac::BTIM1.isr().read() & pac::BTIM1.dier().read() & 1 == 0 {
            return;
        }
        pac::BTIM1.icr().write_value(BTIM_ICR_MASK & !1);
        MILLISECONDS = MILLISECONDS.wrapping_add(1);
        let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
        for i in 0..5 {
            ADC2_RAW[i] = core::ptr::read_volatile(dma.add(i)) as u16;
        }
        if pac::adc::fields::isr::EOS.read(pac::ADC2.isr().read()) {
            ADC2_SEQUENCES = ADC2_SEQUENCES.wrapping_add(1);
            pac::ADC2
                .icr()
                .write_value(ADC_ICR_MASK & !pac::adc::fields::icr::EOS.mask());
        }
        core::hint::black_box((
            &*core::ptr::addr_of!(ADC2_RAW),
            &*core::ptr::addr_of!(ADC2_SEQUENCES),
        ));

        ADC2_ELAPSED_MS += 1;
        if ADC2_ELAPSED_MS == 5 {
            ADC2_ELAPSED_MS = 0;
            pac::ADC2.start().write_value(1);
        }
        let pressed = pac::read(pac::GPIOA_BASE + pac::gpio::IDR) & (1 << 3) == 0;
        let detector = &mut *core::ptr::addr_of_mut!(DETECTOR);
        KEY_HELD_MS = if pressed {
            KEY_HELD_MS.saturating_add(1)
        } else {
            0
        };
        if KEY_HELD_MS == KEY_DEBOUNCE_MS {
            SECTOR = (SECTOR + 1) % 6;
            LOGICAL_BRIDGE = Bridge::commutation(SECTOR, DEMONSTRATION_DUTY);
            pac::write(pac::GPIOC_BASE + pac::gpio::BSRR, 1 << 13);
            detector.begin_sector(SECTOR, 2);
        }
        core::hint::black_box((
            &*core::ptr::addr_of!(SECTOR),
            &*core::ptr::addr_of!(LOGICAL_BRIDGE),
        ));
        core::hint::black_box(&*core::ptr::addr_of!(MILLISECONDS));
    }
}

fn fatal() -> ! {
    cortex_m::interrupt::disable();
    // No software-state borrow, even when an exception interrupted an ISR.
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe {
            pac::ATIM.bdtr().write_value(0);
            pac::ATIM
                .ccer()
                .write_value(pac::atim::fields::ccer::CC4E.mask());
        }
    }
    loop {
        cortex_m::asm::wfi();
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    fatal()
}
#[cortex_m_rt::exception]
unsafe fn HardFault(_: &cortex_m_rt::ExceptionFrame) -> ! {
    fatal()
}
#[cortex_m_rt::exception]
unsafe fn NonMaskableInt() -> ! {
    fatal()
}
#[cortex_m_rt::exception]
unsafe fn DefaultHandler(_: i16) {
    fatal()
}
