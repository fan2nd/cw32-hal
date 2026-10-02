#![no_std]
#![no_main]

pub mod control;
pub mod frame_queue;
pub mod mailbox;
pub mod protection;
pub mod protocol;
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, TimerCommand, KEY_DEBOUNCE_MS};
use crate::frame_queue::FrameQueue;
use crate::mailbox::{UiCommand, UiFeedback};
use crate::protection::Adc2Sample;
use crate::protocol::telemetry_frame;
use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, Ordering};
use critical_section::Mutex;
use embassy_cw32::{
    interrupt::{self, InterruptExt},
    pac, rcc,
};

// Original clock profile, confirmed VDDA=5 V: HCLK/PCLK=96 MHz.
// ADC1=48 MHz, 70-cycle sampling; ADC2=12 MHz, 518-cycle sampling.
const CPU_HZ: u32 = 96_000_000;
const PWM_PERIOD: u16 = 4800;
const BTIM_ICR_MASK: u32 = 0x41;
const HIGH_MASK: u32 = (1 << 5) | (1 << 6) | (1 << 7);
// DMA is the only writer after setup. CPU uses raw volatile reads, never Rust references.
static mut ADC2_DMA: [u32; 5] = [0; 5];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Motor-domain observations. Logical bridge requests are not physical outputs.
/// Read only while halted with power disconnected; a live debugger can tear.
#[derive(Clone, Copy, Debug)]
pub struct Diagnostics {
    pub adc1: [u16; 4],
    pub adc2: [u16; 5],
    pub milliseconds: u32,
    pub adc1_sequences: u32,
    pub logical_bridge: Bridge,
    pub outputs_armed: bool,
    pub state: u8,
    pub fault: u8,
}

// Main initializes before unmask. Motor IRQs are all P1 and cannot nest.
// The tight foreground loop accesses these only inside a finite critical section;
// no reference escapes. UI uses only its mailbox. Fatal exceptions never return.
static mut CONTROLLER: Option<MotorController> = None;
static mut OUTPUTS_ARMED: bool = false;
static mut BOOTSTRAP_MS: u8 = 0;
static mut DIAGNOSTICS: Diagnostics = Diagnostics {
    adc1: [0; 4],
    adc2: [0; 5],
    milliseconds: 0,
    adc1_sequences: 0,
    logical_bridge: Bridge::off(),
    outputs_armed: false,
    state: 0,
    fault: 0,
};
static mut KEY_HOLD_MS: u16 = 0;
static mut TELEMETRY_MS: u16 = 0;
const TELEMETRY_INTERVAL_MS: u16 = 500;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_frequency = rcc::HsiFrequency::Mhz96;
    config.rcc.pclk_divider = rcc::PclkDivider::Div1;
    let p = embassy_cw32::init(config);
    for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
        irq.disable();
    }
    // SAFETY: global HAL initialization transferred all singletons here;
    // interrupts remain masked until the controller has been installed below.
    unsafe {
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::AHBEN,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | 1 | (1 << 4) | (1 << 5) | (1 << 6),
        );
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::APBEN1,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | (1 << 0) | (1 << 3) | (1 << 5),
        );
        pac::modify(
            pac::SYSCTRL_BASE + pac::sysctrl::APBEN2,
            pac::SYSCTRL_KEY_MASK,
            pac::SYSCTRL_KEY | (1 << 2) | (1 << 9),
        );
        // All six gates get low latches before their output directions.
        for (port, pin) in [
            (pac::GPIOA_BASE, 15),
            (pac::GPIOB_BASE, 3),
            (pac::GPIOB_BASE, 4),
            (pac::GPIOB_BASE, 5),
            (pac::GPIOB_BASE, 6),
            (pac::GPIOB_BASE, 7),
        ] {
            configure_pin(port, pin, PinMode::OutputLow, 0);
        }
        configure_pin(pac::GPIOC_BASE, 13, PinMode::OutputHigh, 0);
        configure_pin(pac::GPIOA_BASE, 3, PinMode::InputPullUp, 0);
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
            configure_pin(port, pin, PinMode::Analog, 0);
        }
        configure_pin(pac::GPIOB_BASE, 12, PinMode::OutputHigh, 1);
        configure_pin(pac::GPIOB_BASE, 11, PinMode::InputPullUp, 1);
        use pac::atim::fields as f;
        pac::ATIM.cr1().write(pac::atim::fields::cr1::ARPE.mask());
        pac::ATIM.bdtr().write(0);
        pac::ATIM.dier().write(0);
        pac::ATIM.ccer().write(0);
        pac::ATIM.cr2().write(0);
        pac::ATIM.smcr().write(0);
        pac::ATIM.psc().write(0);
        pac::ATIM.arr().write(u32::from(PWM_PERIOD - 1));
        pac::ATIM.rcr().write(0);
        pac::ATIM.cnt().write(0);
        // Original PWM1 and preload on all four channels.
        pac::ATIM
            .ccmr1cmp()
            .write((6 << 4) | (1 << 3) | (6 << 12) | (1 << 11));
        pac::ATIM
            .ccmr2cmp()
            .write((6 << 4) | (1 << 3) | (6 << 12) | (1 << 11));
        pac::ATIM.ccr1().write(0);
        pac::ATIM.ccr2().write(0);
        pac::ATIM.ccr3().write(0);
        pac::ATIM.ccr4().write(2400); // Original PWM_PERIOD / 2.
        pac::ATIM.dtr2().write(0);
        pac::ATIM.af1().write(0);
        pac::ATIM.af2().write(0);
        pac::ATIM.bdtr().write(0);
        pac::ATIM.ccer().write(
            f::ccer::CC1E.mask()
                | f::ccer::CC2E.mask()
                | f::ccer::CC3E.mask()
                | f::ccer::CC4E.mask(),
        );
        pac::ATIM.icr().write(0);
        // OPA1: external feedback, PA6 INP2, PA7 INN2, PB0 output.
        // PB0 is read by ADC1 CH8 without creating a second pin owner.
        pac::BGR
            .cr()
            .write(pac::bgr::fields::cr::BGREN.write(pac::BGR.cr().read(), true));
        pac::OPA1.cr().write(0xe220);
        pac::OPA1.cal().write(0);
        pac::OPA1.cr().write(0xe221);
        for (r, length, channels, sample, divider) in [
            (pac::ADC1, 4, 0x2108, 0x9999, 1),
            (pac::ADC2, 5, 0xf875b, 0xfffff, 3),
        ] {
            r.trigger().write(0);
            r.start().write(0);
            r.ier().write(0);
            let reserved = r.cr().read() & !0xff;
            r.cr().write(reserved);
            r.awdcr().write(0);
            r.sqrcfr().write(channels);
            r.sample().write(sample);
            r.icr().write(0);
            r.cr().write(pac::adc::fields::cr::EN.write(
                pac::adc::fields::cr::ENS.write(
                    pac::adc::fields::cr::CLK.write(reserved, divider),
                    length - 1,
                ),
                true,
            ));
        }
        for (r, prescaler, reload, oneshot) in [
            (pac::BTIM1, 95, 999, false),
            (pac::BTIM2, 11, 65530, false),
            (pac::BTIM3, 11, 65530, false),
        ] {
            r.cr1().write(if oneshot { 8 } else { 0 });
            r.dier().write(0);
            r.cr2().write(0);
            r.smcr().write(0);
            r.psc().write(prescaler);
            r.arr().write(reload);
            r.cnt().write(0);
            r.icr().write(0);
        }
        pac::UART1.ier().write(0);
        pac::UART1.cr1().write(0x1003); // PCLK, oversample16, 8N1, RX+TX.
        pac::UART1.cr2().write(0);
        pac::UART1.cr3().write(0);
        // 96MHz/(16*115200)=52.08333; integer52, fractional1/16.
        pac::UART1.brri().write(52);
        pac::UART1.brrf().write(1);
    }
    // >=1 ms nominal instruction delay: exceeds BGR (~30 us), OPA and ADC
    // startup requirements. It is not used as the motor timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    // SAFETY: documented factory calibration halfword, RM25.10/SDK
    // ADC_BGR_VOL_ADDRESS. Keep the source calibration value; protection
    // handles undefined arithmetic explicitly rather than fabricating a voltage.
    let calibration_mv = unsafe { core::ptr::read_volatile(0x0010_07d2 as *const u16) };
    INITIALIZED.store(true, Ordering::Release);

    // Keep the real HAL singleton tokens for the lifetime of this program.
    // Foreground motor MMIO is serialized with P1 handlers using critical sections.
    let motor_peripherals = (
        p.DMA, p.ATIM, p.ADC1, p.ADC2, p.OPA1, p.BGR, p.BTIM1, p.BTIM2, p.BTIM3, p.PA15, p.PB3,
        p.PB4, p.PB5, p.PB6, p.PB7, p.PA0, p.PA1, p.PA2, p.PA6, p.PA7, p.PB0, p.PB2, p.PA8, p.PA10,
        p.PA11,
    );
    // Only this Embassy task reads/writes the UI pins and UART after setup.

    let mut controller = MotorController::new(calibration_mv);
    controller.begin_bootstrap();
    let output_opt_in = cfg!(feature = "motor-output-enable");
    let armed = output_opt_in;
    // Always preserve the original six-tick startup bookkeeping. Physical
    // low-side bootstrap charge is enabled only by the output opt-in feature.
    unsafe {
        if armed {
            // Configure the physical PWM connection once. Commutation below
            // changes CCR/low sides only, as in the C source.
            for pin in [5, 6, 7] {
                set_af(pac::GPIOB_BASE, pin, 7);
            }
            pac::ATIM
                .bdtr()
                .write(pac::ATIM.bdtr().read() | pac::atim::fields::bdtr::MOE.mask());
            // Source bootstrap toggles only the three low-side GPIOs.
            pac::write(pac::GPIOA_BASE + pac::gpio::BSRR, 1 << 15);
            pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 3);
            pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 4);
        }
        core::ptr::addr_of_mut!(CONTROLLER).write(Some(controller));
        core::ptr::addr_of_mut!(OUTPUTS_ARMED).write(armed);
        core::ptr::addr_of_mut!(BOOTSTRAP_MS).write(6);
        (*core::ptr::addr_of_mut!(DIAGNOSTICS)).outputs_armed = output_opt_in;
        pac::ADC1.icr().write(0);
        pac::ADC2.icr().write(0);
        pac::ADC1.ier().write(pac::adc::fields::ier::EOS.mask());
        // Original EOC + BLOCK intent: one 32-bit result per conversion.
        // Defined correction: ADC2_SINGLE (15), not source's mismatched SEQ (14).
        // EOS DMA is explicitly disabled; CNT=5, REPEAT=1, both addresses increment.
        pac::DMA.csr2().write(0);
        pac::DMA.cnt2().write((1 << 16) | 5);
        pac::DMA
            .srcaddr2()
            .write((pac::ADC2_BASE + pac::adc::RESULT0) as u32);
        pac::DMA
            .dstaddr2()
            .write(core::ptr::addr_of_mut!(ADC2_DMA).cast::<u32>() as u32);
        pac::DMA.trig2().write(1 | (15 << 2));
        pac::DMA
            .csr2()
            .write((1 << 11) | (2 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | 1);
        pac::ADC2.ier().write(pac::adc::fields::ier::DMAEOC.mask());
        pac::ADC2
            .trigger()
            .write(pac::adc::fields::trigger::ATIMOC4REFC.mask());

        pac::BTIM1.icr().write(BTIM_ICR_MASK & !1);
        pac::BTIM3.icr().write(BTIM_ICR_MASK & !1);
        pac::BTIM1.dier().write(1);
        pac::BTIM3.dier().write(1);
        for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
            irq.unpend();
            irq.set_priority(interrupt::Priority::P1);
        }
        pac::ADC1
            .trigger()
            .write(pac::adc::fields::trigger::ATIMOC4REFC.mask());
        pac::BTIM1.cr1().write(1); // Original timer enable.
        pac::BTIM2.cr1().write(1);
        pac::ATIM
            .cr1()
            .write(pac::atim::fields::cr1::ARPE.mask() | 1);
        pac::ADC2.start().write(1);
        // All shared values are ready and no reference survives unmask.
        // Later foreground access is serialized with IRQs by critical_section.
        core::sync::atomic::compiler_fence(Ordering::Release);
        for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
            irq.enable();
        }
    }

    interrupt::UART2.disable();
    interrupt::UART2.unpend();
    interrupt::UART2.set_priority(interrupt::Priority::P3);
    // UART2 hardware remains disabled; reserve its IRQ for the official UI executor.
    let _ui_irq_peripheral = p.UART2;
    UI_EXECUTOR
        .start(interrupt::UART2)
        .spawn(ui_task(p.PC13, p.PA3, p.UART1, p.PB12, p.PB11).unwrap());
    loop {
        critical_section::with(|_| unsafe {
            if BOOTSTRAP_MS != 0 {
                return;
            }
            let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
                .as_mut()
                .unwrap();
            let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
            let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
            let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
            let raw = [
                core::ptr::read_volatile(dma) as u16,
                core::ptr::read_volatile(dma.add(1)) as u16,
                core::ptr::read_volatile(dma.add(2)) as u16,
                core::ptr::read_volatile(dma.add(3)) as u16,
                core::ptr::read_volatile(dma.add(4)) as u16,
            ];
            diagnostics.adc2 = raw;
            controller.update_adc2(Adc2Sample {
                current: raw[0],
                bus_voltage: raw[1],
                potentiometer: raw[2],
                temperature: raw[3],
                reference: raw[4],
            });
            let actions = controller.foreground_step(pac::BTIM2.cnt().read() as u16);
            apply_actions(controller, diagnostics, armed, actions, None);
            core::hint::black_box(&*diagnostics);
        });
        core::hint::black_box(&motor_peripherals);
    }
}

static UI_EXECUTOR: embassy_executor::InterruptExecutor =
    embassy_executor::InterruptExecutor::new();
#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn UART2() {
    unsafe {
        UI_EXECUTOR.on_interrupt();
    }
}

#[embassy_executor::task]
async fn ui_task(
    led: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PC13>,
    key: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA3>,
    uart: embassy_cw32::Peri<'static, embassy_cw32::peripherals::UART1>,
    tx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB12>,
    rx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB11>,
) {
    let mut tx = FrameQueue::new();
    loop {
        let feedback = next_ui_tick().await;
        let pressed = unsafe { pac::read(pac::GPIOA_BASE + pac::gpio::IDR) & (1 << 3) == 0 };
        critical_section::with(|cs| UI_LINK.borrow(cs).borrow_mut().command.update(pressed));
        if let Some(on) = feedback.led_on {
            // PC13 LED is active-low; SET/CLR does not race motor GPIO mux RMW.
            unsafe {
                pac::write(
                    pac::GPIOC_BASE + if on { pac::gpio::BRR } else { pac::gpio::BSRR },
                    1 << 13,
                )
            };
        }
        if let Some(frame) = feedback.telemetry {
            tx.push(frame);
        }
        // Nonblocking, at most one UART byte per task wake.
        if unsafe { pac::uart::fields::isr::TXE.read(pac::UART1.isr().read()) } {
            if let Some(byte) = tx.pop_byte() {
                unsafe { pac::UART1.tdr().write(u32::from(byte)) };
            }
        }
        core::hint::black_box((&led, &key, &uart, &tx_pin, &rx_pin));
    }
}

// Called from motor IRQs or a finite foreground critical section, with their live borrows.
unsafe fn apply_actions(
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    armed: &mut bool,
    mut actions: Actions,
    telemetry: Option<[u8; 7]>,
) {
    critical_section::with(|cs| {
        UI_LINK.borrow(cs).borrow_mut().feedback.merge(UiFeedback {
            led_on: actions.led_on.take(),
            telemetry,
        });
    });
    if let Some((sector, duty)) = actions.pre_alignment_pwm {
        unsafe { apply_bridge(Bridge::commutation(sector, duty), armed, true) };
    }
    let alignment = actions
        .bridge
        .is_some_and(|b| b.low_sides == [false, true, true]);
    if let Some(mut bridge) = actions.bridge {
        if alignment {
            bridge.low_sides[2] = false;
        }
        unsafe { apply_bridge(bridge, armed, actions.pwm_only) };
    }
    unsafe {
        if let Some(ticks) = actions.step_timer_preset {
            pac::BTIM2.cnt().write(ticks.into());
        }
        match actions.sensorless_timer {
            TimerCommand::Unchanged => {}
            TimerCommand::Stop => pac::BTIM3.cr1().write(0),
            TimerCommand::Arm(reload) => {
                pac::BTIM3.arr().write(reload.into());
                pac::BTIM3.cnt().write(0);
                pac::BTIM3.cr1().write(1); // Original repetitive timer enable.
            }
        }
    }
    // Source alignment turns C- on after Commutation(0) and its timer writes.
    if alignment && *armed {
        unsafe {
            pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 4);
        }
    }
    if actions.start_adc2 {
        unsafe {
            pac::ADC2.start().write(1);
        }
    }
    diagnostics.outputs_armed = *armed;
    diagnostics.logical_bridge = controller.bridge();
    diagnostics.state = controller.state() as u8;
    diagnostics.fault = controller.fault().map_or(0, |fault| fault as u8);
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn ADC1() {
    unsafe {
        let r = pac::ADC1;
        if !pac::adc::fields::isr::EOS.read(r.isr().read()) {
            return;
        }
        r.icr().write(0);
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        let raw = [
            r.result0().read() as u16 & 4095,
            r.result1().read() as u16 & 4095,
            r.result2().read() as u16 & 4095,
            r.result3().read() as u16 & 4095,
        ];
        diagnostics.adc1 = raw;
        diagnostics.adc1_sequences = diagnostics.adc1_sequences.wrapping_add(1);
        let dma = core::ptr::addr_of!(ADC2_DMA).cast::<u32>();
        controller.update_adc2(Adc2Sample {
            current: core::ptr::read_volatile(dma) as u16,
            bus_voltage: core::ptr::read_volatile(dma.add(1)) as u16,
            potentiometer: core::ptr::read_volatile(dma.add(2)) as u16,
            temperature: core::ptr::read_volatile(dma.add(3)) as u16,
            reference: core::ptr::read_volatile(dma.add(4)) as u16,
        });
        let actions = controller.on_adc1(Adc1Sample::from(raw), pac::BTIM2.cnt().read() as u16);
        apply_actions(controller, diagnostics, armed, actions, None);
        core::hint::black_box(&*diagnostics);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM1() {
    // SAFETY: P1 IRQs cannot nest; foreground borrows only with interrupts masked.
    unsafe {
        if pac::BTIM1.isr().read() & pac::BTIM1.dier().read() & 1 == 0 {
            return;
        }
        pac::BTIM1.icr().write(BTIM_ICR_MASK & !1);
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
        publish_ui_tick();
        let command = critical_section::with(|cs| {
            let mut link = UI_LINK.borrow(cs).borrow_mut();
            link.command.tick_1ms();
            link.command
        });
        let key_pressed = command.key_pressed();
        let step_ticks = pac::BTIM2.cnt().read() as u16;

        let (motor, telemetry) = {
            KEY_HOLD_MS = if key_pressed {
                KEY_HOLD_MS.saturating_add(1)
            } else {
                0
            };
            let key_event = KEY_HOLD_MS == KEY_DEBOUNCE_MS;
            let was_off = controller.powered_off();
            // In the source, immediate key telemetry precedes this tick's 100 ms
            // measurement update. A periodic frame on the same tick supersedes it.
            let key_voltage = controller.measurements().bus_decivolts;
            let motor = controller.tick_1ms(key_pressed, step_ticks);
            let mut telemetry = key_event.then(|| {
                telemetry_frame(
                    controller.speed_level(),
                    key_voltage,
                    controller.powered_off(),
                )
            });
            if was_off && !controller.powered_off() {
                TELEMETRY_MS = 0;
            }
            if controller.powered_off() {
                if !was_off {
                    // Final off frame wins even when auto-off and cadence coincide.
                    telemetry = Some(telemetry_frame(
                        controller.speed_level(),
                        controller.measurements().bus_decivolts,
                        controller.powered_off(),
                    ));
                }
            } else {
                TELEMETRY_MS += 1;
                if TELEMETRY_MS >= TELEMETRY_INTERVAL_MS {
                    TELEMETRY_MS = 0;
                    telemetry = Some(telemetry_frame(
                        controller.speed_level(),
                        controller.measurements().bus_decivolts,
                        controller.powered_off(),
                    ));
                }
            }
            (motor, telemetry)
        };
        apply_actions(controller, diagnostics, armed, motor, telemetry);
        if BOOTSTRAP_MS > 0 {
            BOOTSTRAP_MS -= 1;
            if BOOTSTRAP_MS == 0 {
                if *armed {
                    pac::write(pac::GPIOA_BASE + pac::gpio::BRR, 1 << 15);
                    pac::write(pac::GPIOB_BASE + pac::gpio::BRR, 1 << 3);
                    pac::write(pac::GPIOB_BASE + pac::gpio::BRR, 1 << 4);
                }
                controller.finish_bootstrap();
            }
        }
        core::hint::black_box(&*diagnostics);
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM3_HALLTIM() {
    // SAFETY: P1 IRQs cannot nest; foreground borrows only with interrupts masked.
    unsafe {
        if pac::BTIM3.isr().read() & pac::BTIM3.dier().read() & 1 == 0 {
            return;
        }
        pac::BTIM3.icr().write(BTIM_ICR_MASK & !1);
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        let actions = controller.on_sensorless_timer(pac::BTIM2.cnt().read() as u16);
        apply_actions(controller, diagnostics, armed, actions, None);
        core::hint::black_box(&*diagnostics);
    }
}

// Preserve source Commutation/UPPWM write order without remuxing or masking MOE.
// Fatal exceptions use a separate shutdown path and never return.
unsafe fn apply_bridge(bridge: Bridge, armed: &mut bool, pwm_only: bool) {
    unsafe {
        if !*armed {
            return;
        }
        let lows = bridge.low_sides;
        if !pwm_only && lows == [false; 3] {
            pac::ATIM.ccr1().write(0);
            pac::ATIM.ccr2().write(0);
            pac::ATIM.ccr3().write(0);
            pac::write(pac::GPIOA_BASE + pac::gpio::BRR, 1 << 15);
            pac::write(pac::GPIOB_BASE + pac::gpio::BRR, 1 << 3);
            pac::write(pac::GPIOB_BASE + pac::gpio::BRR, 1 << 4);
            pac::ATIM.ccr4().write(bridge.sample_compare.into());
            return;
        }

        if !pwm_only {
            // Source Commutation: first switch off only the unselected low sides.
            if !lows[0] {
                pac::write(pac::GPIOA_BASE + pac::gpio::BRR, 1 << 15);
            }
            if !lows[1] {
                pac::write(pac::GPIOB_BASE + pac::gpio::BRR, 1 << 3);
            }
            if !lows[2] {
                pac::write(pac::GPIOB_BASE + pac::gpio::BRR, 1 << 4);
            }
        }
        // Preserve the source order: zero inactive CCRs before writing active CCR.
        for i in 0..3 {
            if bridge.pwm_counts[i] == 0 {
                pac::write(pac::ATIM_BASE + pac::atim::CCR1 + i * 4, 0);
            }
        }
        for i in 0..3 {
            if bridge.pwm_counts[i] != 0 {
                pac::write(
                    pac::ATIM_BASE + pac::atim::CCR1 + i * 4,
                    bridge.pwm_counts[i],
                );
            }
        }
        if !pwm_only {
            if lows[0] {
                pac::write(pac::GPIOA_BASE + pac::gpio::BSRR, 1 << 15);
            }
            if lows[1] {
                pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 3);
            }
            if lows[2] {
                pac::write(pac::GPIOB_BASE + pac::gpio::BSRR, 1 << 4);
            }
        }
        pac::ATIM.ccr4().write(bridge.sample_compare.into());
    }
}

unsafe fn drive_off() {
    unsafe {
        pac::ATIM
            .bdtr()
            .write(pac::atim::fields::bdtr::MOE.write(pac::ATIM.bdtr().read(), false));
        pac::ATIM.ccer().write(pac::atim::fields::ccer::CC4E.mask());
        pac::write(pac::GPIOA_BASE + pac::gpio::BRR, 1 << 15);
        pac::write(
            pac::GPIOB_BASE + pac::gpio::BRR,
            HIGH_MASK | (1 << 3) | (1 << 4),
        );
        for pin in [5, 6, 7] {
            set_af(pac::GPIOB_BASE, pin, 0);
        }
        pac::ATIM.ccr1().write(0);
        pac::ATIM.ccr2().write(0);
        pac::ATIM.ccr3().write(0);
    }
}

enum PinMode {
    OutputLow,
    OutputHigh,
    InputPullUp,
    Analog,
}
unsafe fn set_af(port: usize, pin: u32, af: u32) {
    let offset = if pin < 8 {
        pac::gpio::AFRL
    } else {
        pac::gpio::AFRH
    };
    let shift = (pin % 8) * 4;
    unsafe { pac::modify(port + offset, 0xf << shift, af << shift) };
}
unsafe fn configure_pin(port: usize, pin: u32, mode: PinMode, af: u32) {
    let mask = 1 << pin;
    unsafe {
        pac::modify(port + pac::gpio::DIR, 0, mask);
        set_af(port, pin, af);
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
        if matches!(mode, PinMode::OutputLow | PinMode::OutputHigh) {
            pac::write(
                port + if matches!(mode, PinMode::OutputHigh) {
                    pac::gpio::BSRR
                } else {
                    pac::gpio::BRR
                },
                mask,
            );
            pac::modify(port + pac::gpio::DIR, mask, 0);
        }
    }
}

struct UiLink {
    command: UiCommand,
    feedback: UiFeedback,
    pending_tick: bool,
    waker: Option<core::task::Waker>,
}
static UI_LINK: Mutex<RefCell<UiLink>> = Mutex::new(RefCell::new(UiLink {
    command: UiCommand::default_const(),
    feedback: UiFeedback {
        led_on: None,
        telemetry: None,
    },
    pending_tick: false,
    waker: None,
}));

fn publish_ui_tick() {
    let waker = critical_section::with(|cs| {
        let mut link = UI_LINK.borrow(cs).borrow_mut();
        link.pending_tick = true;
        link.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}

async fn next_ui_tick() -> UiFeedback {
    core::future::poll_fn(|cx| {
        critical_section::with(|cs| {
            let mut link = UI_LINK.borrow(cs).borrow_mut();
            if link.pending_tick {
                link.pending_tick = false;
                core::task::Poll::Ready(link.feedback.take())
            } else {
                if !link
                    .waker
                    .as_ref()
                    .is_some_and(|old| old.will_wake(cx.waker()))
                {
                    link.waker = Some(cx.waker().clone());
                }
                core::task::Poll::Pending
            }
        })
    })
    .await
}

fn fatal() -> ! {
    cortex_m::interrupt::disable();
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe { drive_off() };
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
