//! Sampling is shown directly in main and the two equal-priority IRQs.
//! ATIM CH4 is an internal ADC trigger; bridge gate pins are untouched.
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    dma,
    gpio::Port,
    interrupt::{
        self,
        typelevel::{self, Handler, Interrupt as _},
    },
    motor::{
        self, AdcScan, BasicTimer, ClockDivider, MotorPin, NegativeInput, PinId, PinMode,
        PositiveInput, PwmBridge, PwmConfig, SampleTime, ScanConfig, ScanSlot, TimerConfig,
    },
    peripherals, rcc,
};

embassy_cw32::bind_interrupts!(
    struct Irqs {
        ADC1 => AdcHandler;
        BTIM1 => TickHandler;
    }
);

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
    config.rcc.hsi_divider = rcc::HsiDivider::Div1;
    config.rcc.pclk_divider = rcc::PclkDivider::Div1;
    // Retain the HAL singleton tokens for the lifetime of this program.
    let _peripherals = embassy_cw32::try_init(config).expect("clock initialization failed");
    let mut adc2_channel = dma::Channel::new_blocking(_peripherals.DMA_CH2);
    let adc2_stream;
    typelevel::ADC1::disable();
    typelevel::BTIM1::disable();
    // SAFETY: HAL initialization transferred the peripheral singletons;
    // main keeps the singleton tokens; it never accesses hardware after unmask.
    unsafe {
        // LED/key plus analog inputs only; bridge gate pins are untouched.
        for (port, pin, analog, pull_up, output) in [
            (Port::C, 13, false, false, true),
            (Port::A, 3, false, true, false),
            (Port::A, 0, true, false, false),
            (Port::A, 1, true, false, false),
            (Port::A, 2, true, false, false),
            (Port::A, 6, true, false, false),
            (Port::A, 7, true, false, false),
            (Port::B, 0, true, false, false),
            (Port::B, 2, true, false, false),
            (Port::A, 8, true, false, false),
            (Port::A, 10, true, false, false),
            (Port::A, 11, true, false, false),
        ] {
            let mode = if analog {
                PinMode::Analog
            } else if output {
                PinMode::OutputHigh
            } else if pull_up {
                PinMode::InputPullUp
            } else {
                unreachable!()
            };
            MotorPin::acquire(PinId::new(port, pin)).configure(mode, 0);
        }

        PwmBridge::acquire().configure(PwmConfig {
            period: SAMPLING_PERIOD,
            sample_compare: 2400,
            phase_outputs: false,
        });
        // External-feedback OPA1: PA6 INP2, PA7 INN2, PB0 output/ADC1 CH8.
        motor::configure_current_sense(PositiveInput::Inp2, NegativeInput::Inn2);
        // Board channels and acquisition times remain explicit here.
        AdcScan::<peripherals::ADC1>::acquire().configure(ScanConfig {
            slots: &[
                ScanSlot::new(8, SampleTime::Cycles70),
                ScanSlot::new(0, SampleTime::Cycles70),
                ScanSlot::new(1, SampleTime::Cycles70),
                ScanSlot::new(2, SampleTime::Cycles70),
            ],
            divider: ClockDivider::Div2,
        });
        AdcScan::<peripherals::ADC2>::acquire().configure(ScanConfig {
            slots: &[
                ScanSlot::new(11, SampleTime::Cycles518),
                ScanSlot::new(5, SampleTime::Cycles518),
                ScanSlot::new(7, SampleTime::Cycles518),
                ScanSlot::new(8, SampleTime::Cycles518),
                ScanSlot::new(15, SampleTime::Cycles518),
            ],
            divider: ClockDivider::Div8,
        });
        BasicTimer::<peripherals::BTIM1>::acquire().configure(TimerConfig {
            prescaler: 95,
            reload: 999,
        });
    }
    // Nominal >= 1 ms startup settling; not used as a sample timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    INITIALIZED.store(true, Ordering::Release);

    unsafe {
        AdcScan::<peripherals::ADC1>::acquire().clear_events();
        AdcScan::<peripherals::ADC2>::acquire().clear_events();
        AdcScan::<peripherals::ADC1>::acquire().enable_sequence_interrupt::<AdcHandler>(Irqs);
        // Original EOC + BLOCK intent: one 32-bit result per conversion.
        // Defined correction: ADC2_SINGLE (15), not source's mismatched SEQ (14).
        // EOS DMA is explicitly disabled; CNT=5, REPEAT=1, both addresses increment.
        // Static DMA-only storage outlives the stream even on cancellation.
        // CPU reads remain volatile words: a scan can be partially refreshed.
        adc2_stream = adc2_channel
            .start_repeating_raw::<u32>(
                AdcScan::<peripherals::ADC2>::acquire().result_address(),
                core::ptr::addr_of_mut!(ADC2_DMA).cast::<u32>(),
                5,
                dma::RawConfig {
                    trigger: dma::Trigger::Hardware(dma::Request::ADC2_SINGLE),
                    mode: dma::TransferMode::Block,
                    source_increment: true,
                    destination_increment: true,
                },
            )
            .expect("ADC2 DMA configuration");
        AdcScan::<peripherals::ADC2>::acquire().enable_conversion_dma();
        AdcScan::<peripherals::ADC2>::acquire().trigger_from_pwm();

        BasicTimer::<peripherals::BTIM1>::acquire().clear_update();
        BasicTimer::<peripherals::BTIM1>::acquire().enable_update_interrupt::<TickHandler>(Irqs);
        typelevel::ADC1::unpend();
        typelevel::ADC1::set_priority(interrupt::Priority::P1);
        typelevel::BTIM1::unpend();
        typelevel::BTIM1::set_priority(interrupt::Priority::P1);
        AdcScan::<peripherals::ADC1>::acquire().trigger_from_pwm();
        BasicTimer::<peripherals::BTIM1>::acquire().start(); // Original timer enable.
        PwmBridge::acquire().start();

        AdcScan::<peripherals::ADC2>::acquire().start_software();
        // Initialization and all borrows finish before any IRQ can run.
        core::sync::atomic::compiler_fence(Ordering::Release);
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
    }
    loop {
        core::hint::black_box(&adc2_stream);
        cortex_m::asm::wfi();
    }
}

struct AdcHandler;
impl Handler<typelevel::ADC1> for AdcHandler {
    unsafe fn on_interrupt() {
        // SAFETY: only the non-nesting P1 handlers access these static variables.
        unsafe {
            let Some(raw) = AdcScan::<peripherals::ADC1>::acquire().take_sequence::<4>() else {
                return;
            };
            ADC1_RAW = raw;
            ADC1_SEQUENCES = ADC1_SEQUENCES.wrapping_add(1);
            core::hint::black_box((
                &*core::ptr::addr_of!(ADC1_RAW),
                &*core::ptr::addr_of!(ADC1_SEQUENCES),
            ));
        }
    }
}

struct TickHandler;
impl Handler<typelevel::BTIM1> for TickHandler {
    unsafe fn on_interrupt() {
        unsafe {
            if !BasicTimer::<peripherals::BTIM1>::acquire().take_update() {
                return;
            }
            MILLISECONDS = MILLISECONDS.wrapping_add(1);
            let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
            for i in 0..5 {
                ADC2_RAW[i] = core::ptr::read_volatile(dma.add(i)) as u16;
            }
            if AdcScan::<peripherals::ADC2>::acquire().sequence_pending() {
                ADC2_SEQUENCES = ADC2_SEQUENCES.wrapping_add(1);
                AdcScan::<peripherals::ADC2>::acquire().clear_events();
            }
            core::hint::black_box((
                &*core::ptr::addr_of!(ADC2_RAW),
                &*core::ptr::addr_of!(ADC2_SEQUENCES),
            ));

            ADC2_ELAPSED_MS += 1;
            if ADC2_ELAPSED_MS == 5 {
                ADC2_ELAPSED_MS = 0;
                AdcScan::<peripherals::ADC2>::acquire().start_software();
            }
            let pressed = !MotorPin::acquire(PinId::new(Port::A, 3)).is_high();
            if pressed {
                MotorPin::acquire(PinId::new(Port::C, 13)).set_high(false);
            } else {
                MotorPin::acquire(PinId::new(Port::C, 13)).set_high(true);
            }
            core::hint::black_box(&*core::ptr::addr_of!(MILLISECONDS));
        }
    }
}

fn fatal() -> ! {
    cortex_m::interrupt::disable();
    // No software-state borrow, even when an exception interrupted an ISR.
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe { motor::emergency_disable_outputs() };
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
