//! Direct PAC initialization, motor task, and immediate hardware interrupt work.
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, TimerCommand, KEY_DEBOUNCE_MS};
use crate::protection::Adc2Sample;
use crate::protocol::telemetry_frame;
use crate::ui::UiFeedback;
use crate::{configure_pin, set_af, PinMode};
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    interrupt::{self, InterruptExt},
    pac,
};

// Original clock profile, confirmed VDDA=5 V: HCLK/PCLK=96 MHz.
// ADC1=48 MHz, 70-cycle sampling; ADC2=12 MHz, 518-cycle sampling.
const CPU_HZ: u32 = 96_000_000;
const PWM_PERIOD: u16 = 4800;
const BTIM_ICR_MASK: u32 = 0x41;
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

// Only the P1 motor task and P1 motor handlers access these objects. ARMv6-M
// does not preempt an active exception with one of equal priority, including
// software-pended UART2. No borrow crosses await or returns from a handler.
// Thread-mode UI exchanges copied values only. Fatal exceptions never return.
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

pub type MotorPeripherals = (
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::DMA>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::ATIM>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::ADC1>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::ADC2>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::OPA1>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::BGR>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::BTIM1>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::BTIM2>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::BTIM3>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA15>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB3>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB4>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB5>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB6>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB7>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA0>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA1>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA2>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA6>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA7>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB0>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB2>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA8>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA10>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA11>,
    embassy_cw32::Peri<'static, embassy_cw32::peripherals::UART2>,
);
static MOTOR_EXECUTOR: embassy_executor::InterruptExecutor =
    embassy_executor::InterruptExecutor::new();

pub fn start(peripherals: MotorPeripherals) {
    interrupt::UART2.disable();
    interrupt::UART2.unpend();
    interrupt::UART2.set_priority(interrupt::Priority::P1);
    // UART2 peripheral stays unused; its singleton is retained by the motor task.
    MOTOR_EXECUTOR
        .start(interrupt::UART2)
        .spawn(run(peripherals).unwrap());
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn UART2() {
    unsafe { MOTOR_EXECUTOR.on_interrupt() };
}

// These are wake notifications, not a queue of hardware events. Every sample,
// tick and commutation is applied in its ISR before notification. Only redundant
// requests to inspect the already-updated state may coalesce. Single P1 domain
// makes checking pending + registering the waker atomic with respect to producers.
static mut MOTOR_PENDING: bool = false;
static mut MOTOR_WAKER: Option<core::task::Waker> = None;

unsafe fn notify_motor() {
    unsafe {
        MOTOR_PENDING = true;
        if let Some(waker) = (&*core::ptr::addr_of!(MOTOR_WAKER)).as_ref() {
            waker.wake_by_ref();
        }
    }
}

async fn next_motor_event() {
    core::future::poll_fn(|cx| unsafe {
        if MOTOR_PENDING {
            MOTOR_PENDING = false;
            core::task::Poll::Ready(())
        } else {
            let slot = &mut *core::ptr::addr_of_mut!(MOTOR_WAKER);
            if !slot.as_ref().is_some_and(|old| old.will_wake(cx.waker())) {
                *slot = Some(cx.waker().clone());
            }
            core::task::Poll::Pending
        }
    })
    .await
}

#[embassy_executor::task]
async fn run(peripherals: MotorPeripherals) {
    for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
        irq.disable();
    }
    // SAFETY: global HAL initialization transferred all singletons here;
    // interrupts remain masked until the controller has been installed below.
    unsafe {
        // All six gates get low latches before their output directions.
        for (port, pin) in [
            (pac::GPIOA.as_ptr(), 15),
            (pac::GPIOB.as_ptr(), 3),
            (pac::GPIOB.as_ptr(), 4),
            (pac::GPIOB.as_ptr(), 5),
            (pac::GPIOB.as_ptr(), 6),
            (pac::GPIOB.as_ptr(), 7),
        ] {
            configure_pin(port, pin, PinMode::OutputLow, 0);
        }
        for (port, pin) in [
            (pac::GPIOA.as_ptr(), 0),
            (pac::GPIOA.as_ptr(), 1),
            (pac::GPIOA.as_ptr(), 2),
            (pac::GPIOA.as_ptr(), 6),
            (pac::GPIOA.as_ptr(), 7),
            (pac::GPIOB.as_ptr(), 0),
            (pac::GPIOB.as_ptr(), 2),
            (pac::GPIOA.as_ptr(), 8),
            (pac::GPIOA.as_ptr(), 10),
            (pac::GPIOA.as_ptr(), 11),
        ] {
            configure_pin(port, pin, PinMode::Analog, 0);
        }
        pac::ATIM.cr1().write(|r| r.set_arpe(true));
        pac::ATIM.bdtr().write(|_| {});
        pac::ATIM.dier().write(|_| {});
        pac::ATIM.ccer().write(|_| {});
        pac::ATIM.cr2().write(|_| {});
        pac::ATIM.smcr().write(|_| {});
        pac::ATIM.psc().write(|_| {});
        pac::ATIM.arr().write(|r| r.set_arr(PWM_PERIOD - 1));
        pac::ATIM.rcr().write(|_| {});
        pac::ATIM.cnt().write(|_| {});
        // Original PWM1 and preload on all four channels.
        pac::ATIM.ccmr_cmp(0).write(|r| {
            r.set_ocm(0, 6);
            r.set_ocpe(0, true);
            r.set_ocm(1, 6);
            r.set_ocpe(1, true);
        });
        pac::ATIM.ccmr_cmp(1).write(|r| {
            r.set_ocm(0, 6);
            r.set_ocpe(0, true);
            r.set_ocm(1, 6);
            r.set_ocpe(1, true);
        });
        pac::ATIM.ccr(0).write(|_| {});
        pac::ATIM.ccr(1).write(|_| {});
        pac::ATIM.ccr(2).write(|_| {});
        pac::ATIM.ccr(3).write(|r| r.set_ccr(2400)); // Original PWM_PERIOD / 2.
        pac::ATIM.dtr2().write(|_| {});
        pac::ATIM.af1().write(|r| r.set_bkine(false));
        pac::ATIM.af2().write(|r| r.set_bk2ine(false));
        pac::ATIM.bdtr().write(|_| {});
        pac::ATIM.ccer().write(|r| {
            r.set_cc1e(true);
            r.set_cc2e(true);
            r.set_cc3e(true);
            r.set_cc4e(true);
        });
        pac::ATIM.icr().write_value(pac::atim::regs::Icr(0));
        // OPA1: external feedback, PA6 INP2, PA7 INN2, PB0 output.
        // PB0 is read by ADC1 CH8 without creating a second pin owner.
        pac::BGR.cr().modify(|r| r.set_bgren(true));
        pac::OPA1.cr().write(|r| {
            r.set_inn2en(true);
            r.set_inp2en(true);
            r.set_en(false);
        });
        pac::OPA1.cal().write(|_| {});
        pac::OPA1.cr().write(|r| {
            r.set_inn2en(true);
            r.set_inp2en(true);
            r.set_en(true);
        });
        for (r, length, channels, sample, divider) in [
            (pac::ADC1, 4, [8, 0, 1, 2, 0], [9, 9, 9, 9, 0], 1),
            (pac::ADC2, 5, [11, 5, 7, 8, 15], [15; 5], 3),
        ] {
            r.trigger().write(|_| {});
            r.start().write(|r| r.set_start(false));
            r.ier().write(|_| {});
            // Read once, clear only the documented low byte, and retain all
            // reserved bits from that same snapshot through both CR writes.
            let mut control = r.cr().read();
            control.set_slave(false);
            control.set_ens(0);
            control.set_clk(0);
            control.set_cont(false);
            control.set_en(false);
            r.cr().write_value(control);
            r.awdcr().write(|_| {});
            r.sqrcfr().write(|r| {
                r.set_sqrch(0, channels[0]);
                r.set_sqrch(1, channels[1]);
                r.set_sqrch(2, channels[2]);
                r.set_sqrch(3, channels[3]);
                r.set_sqrch(4, channels[4]);
            });
            r.sample().write(|r| {
                r.set_sqrch(0, sample[0]);
                r.set_sqrch(1, sample[1]);
                r.set_sqrch(2, sample[2]);
                r.set_sqrch(3, sample[3]);
                r.set_sqrch(4, sample[4]);
            });
            r.icr().write_value(pac::adc::regs::Icr(0));
            control.set_clk(divider);
            control.set_ens(length - 1);
            control.set_en(true);
            r.cr().write_value(control);
        }
        for (r, prescaler, reload, oneshot) in [
            (pac::BTIM1, 95, 999, false),
            (pac::BTIM2, 11, 65530, false),
            (pac::BTIM3, 11, 65530, false),
        ] {
            r.cr1().write(|r| r.set_oneshot(oneshot));
            r.dier().write(|_| {});
            r.cr2().write(|_| {});
            r.smcr().write(|_| {});
            r.psc().write(|r| r.set_psc(prescaler));
            r.arr().write(|r| r.set_arr(reload));
            r.cnt().write(|r| r.set_cnt(0));
            r.icr().write_value(pac::btim::regs::Icr(0));
        }
    }
    // >=1 ms nominal instruction delay: exceeds BGR (~30 us), OPA and ADC
    // startup requirements. It is not used as the motor timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    // SAFETY: documented factory calibration halfword, RM25.10/SDK
    // ADC_BGR_VOL_ADDRESS. Keep the source calibration value; protection
    // handles undefined arithmetic explicitly rather than fabricating a voltage.
    let calibration_mv = unsafe { core::ptr::read_volatile(0x0010_07d2 as *const u16) };
    INITIALIZED.store(true, Ordering::Release);

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
                set_af(pac::GPIOB.as_ptr(), pin, 7);
            }
            let mut bdtr = pac::ATIM.bdtr().read();
            bdtr.set_moe(true);
            pac::ATIM.bdtr().write_value(bdtr);
            // Source bootstrap toggles only the three low-side GPIOs.
            pac::GPIOA.bsrr().write(|r| r.set_bss(15, true));
            pac::GPIOB.bsrr().write(|r| r.set_bss(3, true));
            pac::GPIOB.bsrr().write(|r| r.set_bss(4, true));
        }
        core::ptr::addr_of_mut!(CONTROLLER).write(Some(controller));
        core::ptr::addr_of_mut!(OUTPUTS_ARMED).write(armed);
        core::ptr::addr_of_mut!(BOOTSTRAP_MS).write(6);
        (*core::ptr::addr_of_mut!(DIAGNOSTICS)).outputs_armed = output_opt_in;
        pac::ADC1.icr().write_value(pac::adc::regs::Icr(0));
        pac::ADC2.icr().write_value(pac::adc::regs::Icr(0));
        pac::ADC1.ier().write(|r| r.set_eos(true));
        // Original EOC + BLOCK intent: one 32-bit result per conversion.
        // Defined correction: ADC2_SINGLE (15), not source's mismatched SEQ (14).
        // EOS DMA is explicitly disabled; CNT=5, REPEAT=1, both addresses increment.
        pac::DMA.ch(1).csr().write(|_| {});
        pac::DMA.ch(1).cnt().write(|r| {
            r.set_repeat(1);
            r.set_cnt(5);
        });
        pac::DMA
            .ch(1)
            .srcaddr()
            .write(|r| r.set_srcaddr(pac::ADC2.result(0).as_ptr() as u32));
        pac::DMA
            .ch(1)
            .dstaddr()
            .write(|r| r.set_dstaddr(core::ptr::addr_of_mut!(ADC2_DMA).cast::<u32>() as u32));
        pac::DMA.ch(1).trig().write(|r| {
            r.set_type(true);
            r.set_hardsrc(15);
        });
        pac::DMA.ch(1).csr().write(|r| {
            r.set_restart(true);
            r.set_size(2);
            r.set_dstinc(true);
            r.set_srcinc(true);
            r.set_trans(true);
            r.set_en(true);
        });
        pac::ADC2.ier().write(|r| r.set_dmaeoc(true));
        pac::ADC2.trigger().write(|r| r.set_atimoc4refc(true));

        pac::BTIM1
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        pac::BTIM3
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        pac::BTIM1.dier().write(|r| r.set_uie(true));
        pac::BTIM3.dier().write(|r| r.set_uie(true));
        for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
            irq.unpend();
            irq.set_priority(interrupt::Priority::P1);
        }
        pac::ADC1.trigger().write(|r| r.set_atimoc4refc(true));
        pac::BTIM1.cr1().write(|r| r.set_en(true)); // Original timer enable.
        pac::BTIM2.cr1().write(|r| r.set_en(true));
        pac::ATIM.cr1().write(|r| {
            r.set_arpe(true);
            r.set_cen(true);
        });
        pac::ADC2.start().write(|r| r.set_start(true));
        // All shared values are ready and no reference survives unmask.
        // Motor task and all motor handlers execute at P1, never nested.
        core::sync::atomic::compiler_fence(Ordering::Release);
        for irq in [interrupt::ADC1, interrupt::BTIM1, interrupt::BTIM3_HALLTIM] {
            irq.enable();
        }
    }

    loop {
        next_motor_event().await;
        // No interrupt can mutate this controller while UART2/P1 is active.
        // Drain finite source continuations now; waiting paths return Pending.
        unsafe {
            if BOOTSTRAP_MS != 0 {
                continue;
            }
            loop {
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
                let actions = controller.foreground_step(pac::BTIM2.cnt().read().cnt());
                apply_actions(controller, diagnostics, armed, actions, None);
                core::hint::black_box(&*diagnostics);
                if !controller.foreground_ready() {
                    break;
                }
            }
        }
        core::hint::black_box(&peripherals);
    }
}

// Called only within the serialized P1 motor domain, with live bounded borrows.
unsafe fn apply_actions(
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    armed: &mut bool,
    mut actions: Actions,
    telemetry: Option<[u8; 7]>,
) {
    crate::ui::publish_status(UiFeedback {
        led_on: actions.led_on.take(),
        telemetry,
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

    if let Some(ticks) = actions.step_timer_preset {
        pac::BTIM2.cnt().write(|r| r.set_cnt(ticks));
    }
    match actions.sensorless_timer {
        TimerCommand::Unchanged => {}
        TimerCommand::Stop => pac::BTIM3.cr1().write(|_| {}),
        TimerCommand::Arm(reload) => {
            pac::BTIM3.arr().write(|r| r.set_arr(reload));
            pac::BTIM3.cnt().write(|_| {});
            pac::BTIM3.cr1().write(|r| r.set_en(true)); // Original repetitive timer enable.
        }
    }

    // Source alignment turns C- on after Commutation(0) and its timer writes.
    if alignment && *armed {
        pac::GPIOB.bsrr().write(|r| r.set_bss(4, true));
    }
    if actions.start_adc2 {
        pac::ADC2.start().write(|r| r.set_start(true));
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
        if !r.isr().read().eos() {
            return;
        }
        r.icr().write_value(pac::adc::regs::Icr(0));
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        let raw = [
            r.result(0).read().result(),
            r.result(1).read().result(),
            r.result(2).read().result(),
            r.result(3).read().result(),
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
        let actions = controller.on_adc1(Adc1Sample::from(raw), pac::BTIM2.cnt().read().cnt());
        apply_actions(controller, diagnostics, armed, actions, None);
        core::hint::black_box(&*diagnostics);
        // Samples stay in this ISR. Wake only when the sample created ready
        // foreground work (crossing/startup success/fault/initial data).
        if controller.foreground_ready() {
            notify_motor();
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM1() {
    // SAFETY: all motor handlers and the motor executor are P1 and cannot nest.
    unsafe {
        if !(pac::BTIM1.isr().read().uif() & pac::BTIM1.dier().read().uie()) {
            return;
        }
        pac::BTIM1
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
        crate::ui::publish_ui_tick();
        let key_pressed = crate::ui::key_sample();
        let step_ticks = pac::BTIM2.cnt().read().cnt();

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
                    pac::GPIOA.brr().write(|r| r.set_brr(15, true));
                    pac::GPIOB.brr().write(|r| r.set_brr(3, true));
                    pac::GPIOB.brr().write(|r| r.set_brr(4, true));
                }
                controller.finish_bootstrap();
            }
        }
        core::hint::black_box(&*diagnostics);
        notify_motor();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "C" fn BTIM3_HALLTIM() {
    // SAFETY: all motor handlers and the motor executor are P1 and cannot nest.
    unsafe {
        if !(pac::BTIM3.isr().read().uif() & pac::BTIM3.dier().read().uie()) {
            return;
        }
        pac::BTIM3
            .icr()
            .write_value(pac::btim::regs::Icr(BTIM_ICR_MASK & !1));
        let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
            .as_mut()
            .unwrap();
        let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
        let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
        let actions = controller.on_sensorless_timer(pac::BTIM2.cnt().read().cnt());
        apply_actions(controller, diagnostics, armed, actions, None);
        core::hint::black_box(&*diagnostics);
        if controller.foreground_ready() {
            notify_motor();
        }
    }
}

// Preserve source Commutation/UPPWM write order without remuxing or masking MOE.
// Fatal exceptions use a separate shutdown path and never return.
unsafe fn apply_bridge(bridge: Bridge, armed: &mut bool, pwm_only: bool) {
    if !*armed {
        return;
    }
    let lows = bridge.low_sides;
    if !pwm_only && lows == [false; 3] {
        pac::ATIM.ccr(0).write(|_| {});
        pac::ATIM.ccr(1).write(|_| {});
        pac::ATIM.ccr(2).write(|_| {});
        pac::GPIOA.brr().write(|r| r.set_brr(15, true));
        pac::GPIOB.brr().write(|r| r.set_brr(3, true));
        pac::GPIOB.brr().write(|r| r.set_brr(4, true));
        pac::ATIM.ccr(3).write(|r| r.set_ccr(bridge.sample_compare));
        return;
    }

    if !pwm_only {
        // Source Commutation: first switch off only the unselected low sides.
        if !lows[0] {
            pac::GPIOA.brr().write(|r| r.set_brr(15, true));
        }
        if !lows[1] {
            pac::GPIOB.brr().write(|r| r.set_brr(3, true));
        }
        if !lows[2] {
            pac::GPIOB.brr().write(|r| r.set_brr(4, true));
        }
    }
    // Preserve the source order: zero inactive CCRs before writing active CCR.
    for i in 0..3 {
        if bridge.pwm_counts[i] == 0 {
            pac::ATIM.ccr(i).write(|r| r.set_ccr(0));
        }
    }
    for i in 0..3 {
        if bridge.pwm_counts[i] != 0 {
            // Preserve the complete source compare word, including its u32
            // representation; this is an intentional whole-register write.
            pac::ATIM
                .ccr(i)
                .write_value(pac::atim::regs::Ccr(bridge.pwm_counts[i]));
        }
    }
    if !pwm_only {
        if lows[0] {
            pac::GPIOA.bsrr().write(|r| r.set_bss(15, true));
        }
        if lows[1] {
            pac::GPIOB.bsrr().write(|r| r.set_bss(3, true));
        }
        if lows[2] {
            pac::GPIOB.bsrr().write(|r| r.set_bss(4, true));
        }
    }
    pac::ATIM.ccr(3).write(|r| r.set_ccr(bridge.sample_compare));
}

unsafe fn drive_off() {
    unsafe {
        let mut bdtr = pac::ATIM.bdtr().read();
        bdtr.set_moe(false);
        pac::ATIM.bdtr().write_value(bdtr);
        pac::ATIM.ccer().write(|r| r.set_cc4e(true));
        pac::GPIOA.brr().write(|r| r.set_brr(15, true));
        pac::GPIOB.brr().write(|r| {
            for pin in [3, 4, 5, 6, 7] {
                r.set_brr(pin, true);
            }
        });
        for pin in [5, 6, 7] {
            set_af(pac::GPIOB.as_ptr(), pin, 0);
        }
        pac::ATIM.ccr(0).write(|_| {});
        pac::ATIM.ccr(1).write(|_| {});
        pac::ATIM.ccr(2).write(|_| {});
    }
}

pub fn fatal() -> ! {
    cortex_m::interrupt::disable();
    if INITIALIZED.load(Ordering::Acquire) {
        unsafe { drive_off() };
    }
    loop {
        cortex_m::asm::wfi();
    }
}
