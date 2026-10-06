#![no_std]
#![no_main]

mod logging;
use defmt_rtt as _;

pub mod control;
pub mod protection;
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, TimerCommand};
use crate::protection::Adc2Sample;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_cw32::{
    dma,
    gpio::Port,
    interrupt::{
        self,
        typelevel::{self, Handler, Interrupt as _},
    },
    motor::{
        self, AdcScan, BasicTimer, BridgeUpdate, ClockDivider, MotorPin, NegativeInput, PhaseDrive,
        PinId, PinMode, PositiveInput, PwmBridge, PwmConfig, SampleTime, ScanConfig, ScanSlot,
        TimerConfig,
    },
    peripherals, rcc,
};

embassy_cw32::bind_interrupts!(
    struct Irqs {
        ADC1 => AdcHandler;
        BTIM1 => TickHandler;
        BTIM3_HALLTIM => CommutationHandler;
    }
);

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
    pub sector: u8,
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
    sector: 0,
    logical_bridge: Bridge::off(),
    outputs_armed: false,
    state: 0,
    fault: 0,
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_divider = rcc::HsiDivider::Div1;
    config.rcc.pclk_divider = rcc::PclkDivider::Div1;
    let p = embassy_cw32::init(config);
    let clocks = embassy_cw32::rcc::clocks();
    defmt::info!(
        "05-startup: boot HCLK={}Hz PCLK={}Hz RTT=nonblocking",
        clocks.hclk_hz(),
        clocks.pclk_hz()
    );
    defmt::info!(
        "PWM=20000Hz ADC1=48000000Hz ADC2=12000000Hz; bootstrap=6ms, delay=400ms, align=150ms"
    );
    defmt::info!("power outputs: 6ms low-side bootstrap, then off until key start");

    let mut adc2_channel = dma::Channel::new_blocking(p.DMA_CH2);
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
        MotorPin::acquire(PinId::new(Port::C, 13)).configure(PinMode::OutputHigh, 0);
        MotorPin::acquire(PinId::new(Port::A, 3)).configure(PinMode::InputPullUp, 0);
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

    // Keep the real HAL singleton tokens for the lifetime of this program.
    // Foreground motor MMIO is serialized with P1 handlers using critical sections.
    let motor_peripherals = (
        p.ATIM, p.ADC1, p.ADC2, p.OPA1, p.BGR, p.BTIM1, p.BTIM2, p.BTIM3, p.PA15, p.PB3, p.PB4,
        p.PB5, p.PB6, p.PB7, p.PA0, p.PA1, p.PA2, p.PA6, p.PA7, p.PB0, p.PB2, p.PA8, p.PA10,
        p.PA11, p.PC13, p.PA3,
    );
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
        // Later foreground access is serialized with IRQs by critical_section.
        core::sync::atomic::compiler_fence(Ordering::Release);
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
        typelevel::BTIM3_HALLTIM::enable();
    }

    let mut last_status = None;
    let mut last_log_ms = 0u32;
    let mut last_fault = None;
    loop {
        let snapshot = critical_section::with(|_| unsafe {
            if BOOTSTRAP_MS != 0 {
                return None;
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
            let actions =
                controller.foreground_step(BasicTimer::<peripherals::BTIM2>::acquire().counter());
            apply_actions(controller, diagnostics, armed, actions);
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
                Some(logging::Snapshot::capture(
                    controller,
                    diagnostics.milliseconds,
                    diagnostics.adc1,
                    diagnostics.adc2,
                    diagnostics.adc1_sequences,
                    *armed,
                    calibration_mv,
                ))
            } else {
                None
            }
        });
        // All controller borrows and motor critical sections ended before RTT.
        if let Some(snapshot) = snapshot {
            snapshot.report(last_fault);
            last_fault = snapshot.fault;
        }
        core::hint::black_box((&motor_peripherals, &adc2_stream));
    }
}

// Motor IRQs call directly; foreground calls inside its finite critical section.
unsafe fn apply_actions(
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    armed: &mut bool,
    actions: Actions,
) {
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
    if let Some(on) = actions.led_on {
        if on {
            MotorPin::acquire(PinId::new(Port::C, 13)).set_high(false);
        } else {
            MotorPin::acquire(PinId::new(Port::C, 13)).set_high(true);
        }
    }
    diagnostics.outputs_armed = *armed;
    diagnostics.logical_bridge = controller.bridge();
    diagnostics.sector = controller.sector();
    diagnostics.state = controller.state() as u8;
    diagnostics.fault = controller.fault().map_or(0, |fault| fault as u8);
}

struct AdcHandler;
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
            apply_actions(controller, diagnostics, armed, actions);
            core::hint::black_box(&*diagnostics);
        }
    }
}

struct TickHandler;
impl Handler<typelevel::BTIM1> for TickHandler {
    unsafe fn on_interrupt() {
        // SAFETY: P1 IRQs cannot nest; foreground borrows only with interrupts masked.
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
            let key_pressed = !MotorPin::acquire(PinId::new(Port::A, 3)).is_high();
            let actions = controller.tick_1ms(
                key_pressed,
                BasicTimer::<peripherals::BTIM2>::acquire().counter(),
            );
            apply_actions(controller, diagnostics, armed, actions);
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
        }
    }
}

struct CommutationHandler;
impl Handler<typelevel::BTIM3_HALLTIM> for CommutationHandler {
    unsafe fn on_interrupt() {
        // SAFETY: P1 IRQs cannot nest; foreground borrows only with interrupts masked.
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
            apply_actions(controller, diagnostics, armed, actions);
            core::hint::black_box(&*diagnostics);
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
