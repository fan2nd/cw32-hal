//! Board wiring, motor task and immediate ISR work using explicit motor HAL leases.
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, TimerCommand, KEY_DEBOUNCE_MS};
use crate::protection::Adc2Sample;
use crate::protocol::telemetry_frame;
use crate::ui::UiFeedback;
use crate::Irqs;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    dma,
    gpio::Port,
    interrupt::{
        self,
        typelevel::{self, Binding, Handler, Interrupt as _},
    },
    motor::{
        self, AdcScan, BasicTimer, BridgeUpdate, ClockDivider, MotorPin, NegativeInput, PhaseDrive,
        PinId, PinMode, PositiveInput, PwmBridge, PwmConfig, SampleTime, ScanConfig, ScanSlot,
        TimerConfig,
    },
    peripherals,
};

// Original clock profile, confirmed VDDA=5 V: HCLK/PCLK=96 MHz.
// ADC1=48 MHz, 70-cycle sampling; ADC2=12 MHz, 518-cycle sampling.
const CPU_HZ: u32 = 96_000_000;
const PWM_PERIOD: u16 = 4800;
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

pub fn start(
    dma_channel: embassy_cw32::Peri<'static, embassy_cw32::peripherals::DMA_CH2>,
    peripherals: MotorPeripherals,
    _irq: impl Binding<typelevel::UART2, MotorExecutorHandler>,
) {
    typelevel::UART2::disable();
    typelevel::UART2::unpend();
    typelevel::UART2::set_priority(interrupt::Priority::P1);
    // Binding proves the main-module vector calls MotorExecutorHandler.
    // Upstream start initializes the executor before unmasking its IRQ.
    // UART2 peripheral stays unused; its singleton is retained by the motor task.
    MOTOR_EXECUTOR
        .start(typelevel::UART2::IRQ)
        .spawn(run(dma_channel, peripherals).unwrap());
}

pub(crate) struct MotorExecutorHandler;
impl Handler<typelevel::UART2> for MotorExecutorHandler {
    unsafe fn on_interrupt() {
        unsafe { MOTOR_EXECUTOR.on_interrupt() };
    }
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
async fn run(
    dma_channel: embassy_cw32::Peri<'static, embassy_cw32::peripherals::DMA_CH2>,
    peripherals: MotorPeripherals,
) {
    let mut adc2_channel = dma::Channel::new_blocking(dma_channel);
    let adc2_stream;
    typelevel::ADC1::disable();
    typelevel::BTIM1::disable();
    typelevel::BTIM3_HALLTIM::disable();
    // SAFETY: all motor resources are retained unused by other drivers. Each
    // temporary motor lease ends before the next access or any await; setup
    // runs with motor IRQs masked. Thereafter P1 serializes all motor access.
    unsafe {
        // All six gates get low latches before their output directions.
        for (port, pin) in [
            (Port::A, 15),
            (Port::B, 3),
            (Port::B, 4),
            (Port::B, 5),
            (Port::B, 6),
            (Port::B, 7),
        ] {
            MotorPin::acquire(PinId::new(port, pin)).configure(PinMode::OutputLow, 0);
        }
        for (port, pin) in [
            (Port::A, 0),
            (Port::A, 1),
            (Port::A, 2),
            (Port::A, 6),
            (Port::A, 7),
            (Port::B, 0),
            (Port::B, 2),
            (Port::A, 8),
            (Port::A, 10),
            (Port::A, 11),
        ] {
            MotorPin::acquire(PinId::new(port, pin)).configure(PinMode::Analog, 0);
        }
        PwmBridge::acquire().configure(PwmConfig {
            period: PWM_PERIOD,
            sample_compare: 2400,
            phase_outputs: true,
        });
        // OPA1: external feedback, PA6 INP2, PA7 INN2, PB0 output.
        // PB0 is read by ADC1 CH8 without creating a second pin owner.
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
        BasicTimer::<peripherals::BTIM2>::acquire().configure(TimerConfig {
            prescaler: 11,
            reload: 65530,
        });
        BasicTimer::<peripherals::BTIM3>::acquire().configure(TimerConfig {
            prescaler: 11,
            reload: 65530,
        });
    }
    // >=1 ms nominal instruction delay: exceeds BGR (~30 us), OPA and ADC
    // startup requirements. It is not used as the motor timebase.
    cortex_m::asm::delay(CPU_HZ / 1_000);
    // SAFETY: documented factory calibration halfword, RM25.10/SDK
    // ADC_BGR_VOL_ADDRESS. Keep the source calibration value; protection
    // handles undefined arithmetic explicitly rather than fabricating a voltage.
    let calibration_mv = motor::factory_reference_mv();
    INITIALIZED.store(true, Ordering::Release);

    let mut controller = MotorController::new(calibration_mv);
    controller.begin_bootstrap();
    // Preserve the source's six-tick low-side bootstrap charge. The tick
    // handler then turns all low sides off and waits for a key start.
    unsafe {
        // Configure the physical PWM connection once. Commutation below
        // changes CCR/low sides only, as in the C source.
        for pin in [5, 6, 7] {
            MotorPin::acquire(PinId::new(Port::B, pin)).alternate_function(7);
        }
        PwmBridge::acquire()
            .arm_outputs()
            .expect("hardware break latched");
        // Source bootstrap toggles only the three low-side GPIOs.
        MotorPin::acquire(PinId::new(Port::A, 15)).set_high(true);
        MotorPin::acquire(PinId::new(Port::B, 3)).set_high(true);
        MotorPin::acquire(PinId::new(Port::B, 4)).set_high(true);
        core::ptr::addr_of_mut!(CONTROLLER).write(Some(controller));
        core::ptr::addr_of_mut!(OUTPUTS_ARMED).write(true);
        core::ptr::addr_of_mut!(BOOTSTRAP_MS).write(6);
        (*core::ptr::addr_of_mut!(DIAGNOSTICS)).outputs_armed = true;
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
        BasicTimer::<peripherals::BTIM3>::acquire().clear_update();
        BasicTimer::<peripherals::BTIM1>::acquire().enable_update_interrupt::<TickHandler>(Irqs);
        BasicTimer::<peripherals::BTIM3>::acquire()
            .enable_update_interrupt::<CommutationHandler>(Irqs);
        typelevel::ADC1::unpend();
        typelevel::ADC1::set_priority(interrupt::Priority::P1);
        typelevel::BTIM1::unpend();
        typelevel::BTIM1::set_priority(interrupt::Priority::P1);
        // Shared with HALLTIM: acknowledge only BTIM3's peripheral source.
        // Do not erase a sibling's NVIC pending event. HALLTIM remains unused.
        typelevel::BTIM3_HALLTIM::set_priority(interrupt::Priority::P1);
        AdcScan::<peripherals::ADC1>::acquire().trigger_from_pwm();
        BasicTimer::<peripherals::BTIM1>::acquire().start(); // Original timer enable.
        BasicTimer::<peripherals::BTIM2>::acquire().start();
        PwmBridge::acquire().start();
        AdcScan::<peripherals::ADC2>::acquire().start_software();
        // All shared values are ready and no reference survives unmask.
        // Motor task and all motor handlers execute at P1, never nested.
        core::sync::atomic::compiler_fence(Ordering::Release);
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
        typelevel::BTIM3_HALLTIM::enable();
    }

    let mut last_status = None;
    let mut last_log_ms = 0u32;
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
                let actions = controller
                    .foreground_step(BasicTimer::<peripherals::BTIM2>::acquire().counter());
                apply_actions(controller, diagnostics, armed, actions, None);
                core::hint::black_box(&*diagnostics);
                let status = (
                    controller.state(),
                    controller.startup_state(),
                    controller.fault(),
                    controller.powered_off(),
                    controller.speed_percent(),
                );
                if last_status != Some(status)
                    || diagnostics.milliseconds.wrapping_sub(last_log_ms) >= 500
                {
                    last_status = Some(status);
                    last_log_ms = diagnostics.milliseconds;
                    // Copy only. The thread-mode UI emits RTT after leaving the motor domain.
                    crate::ui::publish_diagnostics(crate::logging::Snapshot::capture(
                        controller,
                        diagnostics.milliseconds,
                        diagnostics.adc1,
                        diagnostics.adc2,
                        diagnostics.adc1_sequences,
                        *armed,
                        calibration_mv,
                    ));
                }
                if !controller.foreground_ready() {
                    break;
                }
            }
        }
        core::hint::black_box((&peripherals, &adc2_stream));
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
        BasicTimer::<peripherals::BTIM2>::acquire().preset(ticks);
    }
    match actions.sensorless_timer {
        TimerCommand::Unchanged => {}
        TimerCommand::Stop => BasicTimer::<peripherals::BTIM3>::acquire().stop(),
        TimerCommand::Arm(reload) => {
            BasicTimer::<peripherals::BTIM3>::acquire().arm(reload);
        }
    }

    // Source alignment turns C- on after Commutation(0) and its timer writes.
    if alignment && *armed {
        MotorPin::acquire(PinId::new(Port::B, 4)).set_high(true);
    }
    if actions.start_adc2 {
        AdcScan::<peripherals::ADC2>::acquire().start_software();
    }
    diagnostics.outputs_armed = *armed;
    diagnostics.logical_bridge = controller.bridge();
    diagnostics.state = controller.state() as u8;
    diagnostics.fault = controller.fault().map_or(0, |fault| fault as u8);
}

pub(crate) struct AdcHandler;
impl Handler<typelevel::ADC1> for AdcHandler {
    unsafe fn on_interrupt() {
        unsafe {
            let Some(raw) = AdcScan::<peripherals::ADC1>::acquire().take_sequence::<4>() else {
                return;
            };
            let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
                .as_mut()
                .unwrap();
            let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
            let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
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
            let actions = controller.on_adc1(
                Adc1Sample::from(raw),
                BasicTimer::<peripherals::BTIM2>::acquire().counter(),
            );
            apply_actions(controller, diagnostics, armed, actions, None);
            core::hint::black_box(&*diagnostics);
            // Samples stay in this ISR. Wake only when the sample created ready
            // foreground work (crossing/startup success/fault/initial data).
            if controller.foreground_ready() {
                notify_motor();
            }
        }
    }
}

pub(crate) struct TickHandler;
impl Handler<typelevel::BTIM1> for TickHandler {
    unsafe fn on_interrupt() {
        // SAFETY: all motor handlers and the motor executor are P1 and cannot nest.
        unsafe {
            if !BasicTimer::<peripherals::BTIM1>::acquire().take_update() {
                return;
            }
            let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
                .as_mut()
                .unwrap();
            let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
            let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
            diagnostics.milliseconds = diagnostics.milliseconds.wrapping_add(1);
            crate::ui::publish_ui_tick();
            let key_pressed = crate::ui::key_sample();
            let step_ticks = BasicTimer::<peripherals::BTIM2>::acquire().counter();

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
                        MotorPin::acquire(PinId::new(Port::A, 15)).set_high(false);
                        MotorPin::acquire(PinId::new(Port::B, 3)).set_high(false);
                        MotorPin::acquire(PinId::new(Port::B, 4)).set_high(false);
                    }
                    controller.finish_bootstrap();
                }
            }
            core::hint::black_box(&*diagnostics);
            notify_motor();
        }
    }
}

pub(crate) struct CommutationHandler;
impl Handler<typelevel::BTIM3_HALLTIM> for CommutationHandler {
    unsafe fn on_interrupt() {
        // SAFETY: all motor handlers and the motor executor are P1 and cannot nest.
        unsafe {
            if !BasicTimer::<peripherals::BTIM3>::acquire().take_update() {
                return;
            }
            let controller = (&mut *core::ptr::addr_of_mut!(CONTROLLER))
                .as_mut()
                .unwrap();
            let diagnostics = &mut *core::ptr::addr_of_mut!(DIAGNOSTICS);
            let armed = &mut *core::ptr::addr_of_mut!(OUTPUTS_ARMED);
            let actions = controller
                .on_sensorless_timer(BasicTimer::<peripherals::BTIM2>::acquire().counter());
            apply_actions(controller, diagnostics, armed, actions, None);
            core::hint::black_box(&*diagnostics);
            if controller.foreground_ready() {
                notify_motor();
            }
        }
    }
}

// Preserve source Commutation/UPPWM write order without remuxing or masking MOE.
// Fatal exceptions use a separate shutdown path and never return.
unsafe fn apply_bridge(bridge: Bridge, armed: &mut bool, pwm_only: bool) {
    if !*armed {
        return;
    }
    unsafe {
        let mut lows = [
            MotorPin::acquire(PinId::new(Port::A, 15)),
            MotorPin::acquire(PinId::new(Port::B, 3)),
            MotorPin::acquire(PinId::new(Port::B, 4)),
        ];
        PwmBridge::acquire().apply(
            &mut lows,
            PhaseDrive {
                pwm_counts: bridge.pwm_counts,
                low_sides: bridge.low_sides,
                sample_compare: bridge.sample_compare,
            },
            if pwm_only {
                BridgeUpdate::PwmOnly
            } else {
                BridgeUpdate::Commutate
            },
        );
    }
}

unsafe fn drive_off() {
    // This fatal path never returns: any interrupted leases are abandoned.
    unsafe {
        motor::emergency_disconnect(
            [
                PinId::new(Port::A, 15),
                PinId::new(Port::B, 3),
                PinId::new(Port::B, 4),
            ],
            [
                PinId::new(Port::B, 5),
                PinId::new(Port::B, 6),
                PinId::new(Port::B, 7),
            ],
        )
    };
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
