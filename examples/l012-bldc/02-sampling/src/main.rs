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
        let mut ahben = pac::SYSCTRL.ahben().read();
        ahben.set_key(0x5a5a);
        ahben.set_dma(true);
        ahben.set_gpioa(true);
        ahben.set_gpiob(true);
        ahben.set_gpioc(true);
        pac::SYSCTRL.ahben().write_value(ahben);
        let mut apben1 = pac::SYSCTRL.apben1().read();
        apben1.set_key(0x5a5a);
        apben1.set_adc(true);
        apben1.set_atim(true);
        pac::SYSCTRL.apben1().write_value(apben1);
        // Basic timers share this clock gate; only timer 1 is accessed.
        let mut apben2 = pac::SYSCTRL.apben2().read();
        apben2.set_key(0x5a5a);
        apben2.set_btim123(true);
        apben2.set_opa(true);
        pac::SYSCTRL.apben2().write_value(apben2);
        // LED/key plus analog inputs only; bridge gate pins are untouched.
        for (port, pin, analog, pull_up, output) in [
            (pac::GPIOC.as_ptr(), 13, false, false, true),
            (pac::GPIOA.as_ptr(), 3, false, true, false),
            (pac::GPIOA.as_ptr(), 0, true, false, false),
            (pac::GPIOA.as_ptr(), 1, true, false, false),
            (pac::GPIOA.as_ptr(), 2, true, false, false),
            (pac::GPIOA.as_ptr(), 6, true, false, false),
            (pac::GPIOA.as_ptr(), 7, true, false, false),
            (pac::GPIOB.as_ptr(), 0, true, false, false),
            (pac::GPIOB.as_ptr(), 2, true, false, false),
            (pac::GPIOA.as_ptr(), 8, true, false, false),
            (pac::GPIOA.as_ptr(), 10, true, false, false),
            (pac::GPIOA.as_ptr(), 11, true, false, false),
        ] {
            let port: pac::gpio::Gpio = pac::gpio::Gpio::from_ptr(port);
            port.dir().modify(|w| w.set_pin(pin, true));
            port.afr(pin / 8).modify(|w| w.set_afr(pin % 8, 0));
            port.riseie().modify(|w| w.set_pin(pin, false));
            port.fallie().modify(|w| w.set_pin(pin, false));
            port.opendrain().modify(|w| w.set_pin(pin, false));
            port.pur().modify(|w| w.set_pin(pin, pull_up));
            port.analog().modify(|w| w.set_pin(pin, analog));
            if output {
                port.bsrr().write(|w| w.set_bss(pin, true));
                port.dir().modify(|w| w.set_pin(pin, false));
            }
        }

        pac::ATIM.cr1().write(|w| w.set_arpe(true));
        pac::ATIM.bdtr().write(|_| {});
        pac::ATIM.dier().write(|_| {});
        pac::ATIM.ccer().write(|_| {});
        pac::ATIM.cr2().write(|_| {});
        pac::ATIM.smcr().write(|_| {});
        pac::ATIM.psc().write(|w| w.set_psc(0));
        pac::ATIM.arr().write(|w| w.set_arr(SAMPLING_PERIOD - 1));
        pac::ATIM.rcr().write(|w| w.set_rep(0));
        pac::ATIM.cnt().write(|w| w.set_cnt(0));
        // No phase PWM mode, channel enable or alternate-function gate mux.
        pac::ATIM.ccmr_cmp(0).write(|_| {});
        // Preserve original PWM1: OC4REFC rises at reload, not at CCR4.
        pac::ATIM.ccmr_cmp(1).write(|w| {
            w.set_ocm(1, 6);
            w.set_ocpe(1, true);
        });
        pac::ATIM.ccr(0).write(|w| w.set_ccr(0));
        pac::ATIM.ccr(1).write(|w| w.set_ccr(0));
        pac::ATIM.ccr(2).write(|w| w.set_ccr(0));
        pac::ATIM.ccr(3).write(|w| w.set_ccr(2400)); // Original PWM_PERIOD / 2 at initialization.
        pac::ATIM.dtr2().write(|_| {});
        pac::ATIM.af1().write(|w| w.set_bkine(false));
        pac::ATIM.af2().write(|w| w.set_bk2ine(false));
        pac::ATIM.ccer().write(|w| w.set_cc4e(true));
        pac::ATIM.icr().write_value(pac::atim::regs::Icr(0));

        // External-feedback OPA1: PA6 INP2, PA7 INN2, PB0 output/ADC1 CH8.
        pac::BGR.cr().modify(|w| w.set_bgren(true));
        pac::OPA1.cr().write(|w| {
            w.set_inn2en(true);
            w.set_inp2en(true);
        });
        pac::OPA1.cal().write(|_| {});
        pac::OPA1.cr().write(|w| {
            w.set_inn2en(true);
            w.set_inp2en(true);
            w.set_en(true);
        });
        for (r, length, channels, sample, divider) in [
            (pac::ADC1, 4, [8, 0, 1, 2, 0], [9, 9, 9, 9, 0], 1),
            (pac::ADC2, 5, [11, 5, 7, 8, 15], [15; 5], 3),
        ] {
            r.trigger().write(|_| {});
            r.start().write(|_| {});
            r.ier().write(|_| {});
            // Preserve reserved CR bits; the uploaded SDK's SAM[9:8] disagrees
            // with the pinned official header / RM and is deliberately not used.
            let mut cr = r.cr().read();
            cr.set_slave(false);
            cr.set_ens(0);
            cr.set_clk(0);
            cr.set_cont(false);
            cr.set_en(false);
            r.cr().write_value(cr);
            r.awdcr().write(|_| {});
            r.sqrcfr().write(|w| {
                w.set_sqrch(0, channels[0]);
                w.set_sqrch(1, channels[1]);
                w.set_sqrch(2, channels[2]);
                w.set_sqrch(3, channels[3]);
                w.set_sqrch(4, channels[4]);
            });
            r.sample().write(|w| {
                w.set_sqrch(0, sample[0]);
                w.set_sqrch(1, sample[1]);
                w.set_sqrch(2, sample[2]);
                w.set_sqrch(3, sample[3]);
                w.set_sqrch(4, sample[4]);
            });
            r.icr().write_value(pac::adc::regs::Icr(0));
            cr.set_clk(divider);
            cr.set_ens(length - 1);
            cr.set_en(true);
            r.cr().write_value(cr);
        }

        pac::BTIM1.cr1().write(|_| {});
        pac::BTIM1.dier().write(|_| {});
        pac::BTIM1.cr2().write(|_| {});
        pac::BTIM1.smcr().write(|_| {});
        pac::BTIM1.psc().write(|w| w.set_psc(95));
        pac::BTIM1.arr().write(|w| w.set_arr(999));
        pac::BTIM1.cnt().write(|w| w.set_cnt(0));
        pac::BTIM1.icr().write_value(pac::btim::regs::Icr(0));
    }
    // Nominal >= 1 ms startup settling; not used as a sample timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    INITIALIZED.store(true, Ordering::Release);

    unsafe {
        pac::ADC1.icr().write_value(pac::adc::regs::Icr(0));
        pac::ADC2.icr().write_value(pac::adc::regs::Icr(0));
        pac::ADC1.ier().write(|w| w.set_eos(true));
        // Original EOC + BLOCK intent: one 32-bit result per conversion.
        // Defined correction: ADC2_SINGLE (15), not source's mismatched SEQ (14).
        // EOS DMA is explicitly disabled; CNT=5, REPEAT=1, both addresses increment.
        pac::DMA.ch(1).csr().write(|_| {});
        pac::DMA.ch(1).cnt().write(|w| {
            w.set_repeat(1);
            w.set_cnt(5);
        });
        pac::DMA
            .ch(1)
            .srcaddr()
            .write(|w| w.set_srcaddr(pac::ADC2.result(0).as_ptr() as u32));
        pac::DMA.ch(1).dstaddr().write(|w| {
            w.set_dstaddr(core::ptr::addr_of_mut!(ADC2_DMA).cast::<u32>() as u32);
        });
        pac::DMA.ch(1).trig().write(|w| {
            w.set_type(true);
            w.set_hardsrc(15);
        });
        pac::DMA.ch(1).csr().write(|w| {
            w.set_restart(true);
            w.set_size(2);
            w.set_dstinc(true);
            w.set_srcinc(true);
            w.set_trans(true);
            w.set_en(true);
        });
        pac::ADC2.ier().write(|w| w.set_dmaeoc(true));
        pac::ADC2.trigger().write(|w| w.set_atimoc4refc(true));

        pac::BTIM1.icr().write(|w| w.set_uif(false));
        pac::BTIM1.dier().write(|w| w.set_uie(true));
        for irq in [interrupt::ADC1, interrupt::BTIM1] {
            irq.unpend();
            irq.set_priority(interrupt::Priority::P1);
        }
        pac::ADC1.trigger().write(|w| w.set_atimoc4refc(true));
        pac::BTIM1.cr1().write(|w| w.set_en(true)); // Original timer enable.
        pac::ATIM.cr1().write(|w| {
            w.set_arpe(true);
            w.set_cen(true);
        });

        pac::ADC2.start().write(|w| w.set_start(true));
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
        if !r.isr().read().eos() {
            return;
        }
        r.icr().write_value(pac::adc::regs::Icr(0));
        ADC1_RAW = [
            r.result(0).read().result(),
            r.result(1).read().result(),
            r.result(2).read().result(),
            r.result(3).read().result(),
        ];
        ADC1_SEQUENCES = ADC1_SEQUENCES.wrapping_add(1);
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
        if !(pac::BTIM1.isr().read().uif() & pac::BTIM1.dier().read().uie()) {
            return;
        }
        pac::BTIM1.icr().write(|w| w.set_uif(false));
        MILLISECONDS = MILLISECONDS.wrapping_add(1);
        let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
        for i in 0..5 {
            ADC2_RAW[i] = core::ptr::read_volatile(dma.add(i)) as u16;
        }
        if pac::ADC2.isr().read().eos() {
            ADC2_SEQUENCES = ADC2_SEQUENCES.wrapping_add(1);
            pac::ADC2.icr().write(|w| w.set_eos(false));
        }
        core::hint::black_box((
            &*core::ptr::addr_of!(ADC2_RAW),
            &*core::ptr::addr_of!(ADC2_SEQUENCES),
        ));

        ADC2_ELAPSED_MS += 1;
        if ADC2_ELAPSED_MS == 5 {
            ADC2_ELAPSED_MS = 0;
            pac::ADC2.start().write(|w| w.set_start(true));
        }
        let pressed = !pac::GPIOA.idr().read().pin(3);
        if pressed {
            pac::GPIOC.brr().write(|w| w.set_brr(13, true));
        } else {
            pac::GPIOC.bsrr().write(|w| w.set_bss(13, true));
        }
        core::hint::black_box(&*core::ptr::addr_of!(MILLISECONDS));
    }
}

fn fatal() -> ! {
    cortex_m::interrupt::disable();
    // No software-state borrow, even when an exception interrupted an ISR.
    if INITIALIZED.load(Ordering::Acquire) {
        pac::ATIM.bdtr().write(|_| {});
        pac::ATIM.ccer().write(|w| w.set_cc4e(true));
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
