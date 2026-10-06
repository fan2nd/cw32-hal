//! 通过显式的电机 HAL 临时访问租约，完成板级配置、电机任务及 ISR 中的即时处理。
use crate::control::{Actions, Adc1Sample, Bridge, MotorController, TimerCommand, KEY_DEBOUNCE_MS};
use crate::io::IoFeedback;
use crate::protection::Adc2Sample;
use crate::protocol::telemetry_frame;
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

// 原时钟配置，已确认 VDDA=5 V：HCLK/PCLK=96 MHz。
// ADC1=48 MHz，采样 70 周期；ADC2=12 MHz，采样 518 周期。
const CPU_HZ: u32 = 96_000_000;
const PWM_PERIOD: u16 = 4800;
// 初始化后只有 DMA 写入。CPU 通过原始指针做 volatile 读取，绝不使用 Rust 引用。
// 保持静态存储：移动 MotorResources 时，绝不能移动 DMA 目标缓冲。
static mut ADC2_DMA: [u32; 5] = [0; 5];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// 电机域的观测值。逻辑桥臂请求不等同于物理输出。
/// 仅在断开功率电源并暂停程序时读取；运行中通过调试器读取可能得到不一致的数据。
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

// 只有 P1 电机任务和 P1 电机中断处理函数访问这些对象。ARMv6-M
// 不会用同优先级异常抢占正在执行的异常，包括
// 由软件挂起的 UART2。借用不得跨越 await，也不得延续到中断处理函数返回之后。
// 普通线程模式的 I/O 只交换复制值。致命异常不再返回。
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

/// 将各单例的所有权一起移交到电机执行器域。
pub struct MotorResources {
    pub atim: embassy_cw32::Peri<'static, peripherals::ATIM>,
    pub adc1: embassy_cw32::Peri<'static, peripherals::ADC1>,
    pub adc2: embassy_cw32::Peri<'static, peripherals::ADC2>,
    pub adc2_dma: embassy_cw32::Peri<'static, peripherals::DMA_CH2>,
    pub opa1: embassy_cw32::Peri<'static, peripherals::OPA1>,
    pub bgr: embassy_cw32::Peri<'static, peripherals::BGR>,
    pub btim1: embassy_cw32::Peri<'static, peripherals::BTIM1>,
    pub btim2: embassy_cw32::Peri<'static, peripherals::BTIM2>,
    pub btim3: embassy_cw32::Peri<'static, peripherals::BTIM3>,
    pub pa15: embassy_cw32::Peri<'static, peripherals::PA15>,
    pub pb3: embassy_cw32::Peri<'static, peripherals::PB3>,
    pub pb4: embassy_cw32::Peri<'static, peripherals::PB4>,
    pub pb5: embassy_cw32::Peri<'static, peripherals::PB5>,
    pub pb6: embassy_cw32::Peri<'static, peripherals::PB6>,
    pub pb7: embassy_cw32::Peri<'static, peripherals::PB7>,
    pub pa0: embassy_cw32::Peri<'static, peripherals::PA0>,
    pub pa1: embassy_cw32::Peri<'static, peripherals::PA1>,
    pub pa2: embassy_cw32::Peri<'static, peripherals::PA2>,
    pub pa6: embassy_cw32::Peri<'static, peripherals::PA6>,
    pub pa7: embassy_cw32::Peri<'static, peripherals::PA7>,
    pub pb0: embassy_cw32::Peri<'static, peripherals::PB0>,
    pub pb2: embassy_cw32::Peri<'static, peripherals::PB2>,
    pub pa8: embassy_cw32::Peri<'static, peripherals::PA8>,
    pub pa10: embassy_cw32::Peri<'static, peripherals::PA10>,
    pub pa11: embassy_cw32::Peri<'static, peripherals::PA11>,
    pub uart2: embassy_cw32::Peri<'static, peripherals::UART2>,
}

static MOTOR_EXECUTOR: embassy_executor::InterruptExecutor =
    embassy_executor::InterruptExecutor::new();

pub fn start(
    resources: MotorResources,
    _irq: impl Binding<typelevel::UART2, MotorExecutorHandler>,
) {
    typelevel::UART2::disable();
    typelevel::UART2::unpend();
    typelevel::UART2::set_priority(interrupt::Priority::P1);
    // 此绑定证明 main 模块的中断向量会调用 MotorExecutorHandler。
    // 上游 start 实现在解除其中断屏蔽前先初始化执行器。
    // UART2 外设保持未使用，其单例由电机任务持有。
    MOTOR_EXECUTOR
        .start(typelevel::UART2::IRQ)
        .spawn(motor_task(resources).unwrap());
}

pub(crate) struct MotorExecutorHandler;
impl Handler<typelevel::UART2> for MotorExecutorHandler {
    unsafe fn on_interrupt() {
        unsafe { MOTOR_EXECUTOR.on_interrupt() };
    }
}

// 这里传递唤醒通知，不构成硬件事件队列。每次采样、
// tick 和换相均在通知前于对应 ISR 中完成。只有重复的
// “检查已更新状态”请求可以合并。统一的 P1 优先级域
// 确保“检查 pending + 登记 waker”相对于通知生产者是原子的。
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
async fn motor_task(resources: MotorResources) {
    let MotorResources {
        atim,
        adc1,
        adc2,
        adc2_dma,
        opa1,
        bgr,
        btim1,
        btim2,
        btim3,
        pa15,
        pb3,
        pb4,
        pb5,
        pb6,
        pb7,
        pa0,
        pa1,
        pa2,
        pa6,
        pa7,
        pb0,
        pb2,
        pa8,
        pa10,
        pa11,
        uart2,
    } = resources;
    let mut adc2_channel = dma::Channel::new_blocking(adc2_dma);
    // 重复传输 guard 借用通道，并在每次 await 期间持续存活。
    let adc2_stream;
    typelevel::ADC1::disable();
    typelevel::BTIM1::disable();
    typelevel::BTIM3_HALLTIM::disable();
    // 安全性：保留全部电机资源，且不供其他驱动使用。每次
    // 临时电机访问租约均在下一次访问或任何 await 前结束；初始化时
    // 屏蔽电机中断，之后由 P1 优先级域将所有电机访问串行化。
    unsafe {
        // 六个栅极均先将输出锁存器设为低电平，再切换为输出方向。
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
        // OPA1：外部反馈，PA6 为 INP2，PA7 为 INN2，PB0 为输出。
        // ADC1 CH8 读取 PB0，不创建第二个引脚所有者。
        motor::configure_current_sense(PositiveInput::Inp2, NegativeInput::Inn2);
        // 此处显式列出板级通道与采样时间。
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
    // 标称至少 1 ms 的指令延迟：超过 BGR（约 30 us）、OPA 和 ADC
    // 的启动等待要求。不将其用作电机时基。
    cortex_m::asm::delay(CPU_HZ / 1_000);
    // 安全性：此处读取文档指定的出厂校准半字，见 RM25.10/SDK 的
    // ADC_BGR_VOL_ADDRESS。保留原源码的校准值；保护逻辑
    // 显式处理未定义算术边界，不编造电压值。
    let calibration_mv = motor::factory_reference_mv();
    INITIALIZED.store(true, Ordering::Release);

    let mut controller = MotorController::new(calibration_mv);
    controller.begin_bootstrap();
    // 保留原源码持续六个 tick 的低侧自举充电。随后 tick
    // 处理函数关闭所有低侧，等待按键启动。
    unsafe {
        // 仅配置一次物理 PWM 连接。下方换相过程
        // 仅改变 CCR/低侧状态，与 C 源码一致。
        for pin in [5, 6, 7] {
            MotorPin::acquire(PinId::new(Port::B, pin)).alternate_function(7);
        }
        PwmBridge::acquire()
            .arm_outputs()
            .expect("hardware break latched");
        // 原源码自举充电过程只切换三个低侧 GPIO。
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
        // 保留原 EOC + BLOCK 意图：每次转换传输一个 32 位结果。
        // 明确修正为 ADC2_SINGLE（15），不使用原源码中不匹配的 SEQ（14）。
        // 显式禁用 EOS DMA；CNT=5、REPEAT=1，源地址和目标地址均递增。
        // 仅供 DMA 写入的静态存储比传输流活得更久，即使传输被取消也如此。
        // CPU 仍按字执行 volatile 读取：扫描结果可能仅有部分已刷新。
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
        // 此向量与 HALLTIM 共享：仅确认并清除 BTIM3 的外设中断源。
        // 不要清除同向量其他外设的 NVIC 挂起事件。HALLTIM 保持未使用。
        typelevel::BTIM3_HALLTIM::set_priority(interrupt::Priority::P1);
        AdcScan::<peripherals::ADC1>::acquire().trigger_from_pwm();
        BasicTimer::<peripherals::BTIM1>::acquire().start(); // 保留原定时器使能操作。
        BasicTimer::<peripherals::BTIM2>::acquire().start();
        PwmBridge::acquire().start();
        AdcScan::<peripherals::ADC2>::acquire().start_software();
        // 所有共享值均已就绪，解除中断屏蔽时没有遗留引用。
        // 电机任务和所有电机中断处理函数均以 P1 优先级执行，不能相互嵌套。
        core::sync::atomic::compiler_fence(Ordering::Release);
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
        typelevel::BTIM3_HALLTIM::enable();
    }

    let mut last_status = None;
    let mut last_log_ms = 0u32;
    loop {
        next_motor_event().await;
        // UART2/P1 正在执行时，其他中断不能修改此控制器。
        // 在此执行完原源码中已就绪且有限的状态延续；等待路径返回 Pending。
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
                    // 这里只复制数据。离开电机域后，由普通线程模式的 I/O 输出 RTT。
                    crate::io::publish_diagnostics(crate::logging::Snapshot::capture(
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
        // 任务挂起期间，仍保持每个单例和 DMA guard 存活。
        core::hint::black_box((
            &atim,
            &adc1,
            &adc2,
            &opa1,
            &bgr,
            &btim1,
            &btim2,
            &btim3,
            &pa15,
            &pb3,
            &pb4,
            &pb5,
            &pb6,
            &pb7,
            &pa0,
            &pa1,
            &pa2,
            &pa6,
            &pa7,
            &pb0,
            &pb2,
            &pa8,
            &pa10,
            &pa11,
            &uart2,
            &adc2_stream,
        ));
    }
}

// 仅在串行化的 P1 电机域内调用，借用有效且范围有界。
unsafe fn apply_actions(
    controller: &MotorController,
    diagnostics: &mut Diagnostics,
    armed: &mut bool,
    mut actions: Actions,
    telemetry: Option<[u8; 7]>,
) {
    crate::io::publish_status(IoFeedback {
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

    // 原源码对齐时，在 Commutation(0) 及其定时器写入完成后导通 C-。
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
            // 采样处理留在此 ISR 中。仅当采样产生可立即处理的
            // 前台工作时唤醒（过零/启动成功/故障/初始数据）。
            if controller.foreground_ready() {
                notify_motor();
            }
        }
    }
}

pub(crate) struct TickHandler;
impl Handler<typelevel::BTIM1> for TickHandler {
    unsafe fn on_interrupt() {
        // 安全性：所有电机中断处理函数和电机执行器均为 P1，不能相互嵌套。
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
            crate::io::publish_io_tick();
            let key_pressed = crate::io::key_sample();
            let step_ticks = BasicTimer::<peripherals::BTIM2>::acquire().counter();

            let (motor, telemetry) = {
                KEY_HOLD_MS = if key_pressed {
                    KEY_HOLD_MS.saturating_add(1)
                } else {
                    0
                };
                let key_event = KEY_HOLD_MS == KEY_DEBOUNCE_MS;
                let was_off = controller.powered_off();
                // 原源码中，按键即时遥测先于本次 tick 的 100 ms
                // 测量值更新。同一 tick 的周期帧会覆盖该按键帧。
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
                        // 即使自动关闭与周期发送同时发生，也优先保留最终关闭帧。
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
        // 安全性：所有电机中断处理函数和电机执行器均为 P1，不能相互嵌套。
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

// 保留原 Commutation/UPPWM 写入顺序，不重新配置引脚复用，也不屏蔽 MOE。
// 致命异常使用独立的关断路径，且不再返回。
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
    // 此致命异常路径不再返回：被中断的所有临时访问租约均不再恢复。
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
