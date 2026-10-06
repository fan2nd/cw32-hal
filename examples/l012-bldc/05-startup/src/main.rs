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

// 原时钟配置，已确认 VDDA=5 V：HCLK/PCLK=96 MHz。
// ADC1=48 MHz，采样 70 周期；ADC2=12 MHz，采样 518 周期。
const CPU_HZ: u32 = 96_000_000;
const PWM_PERIOD: u16 = 4800;
// 初始化后只有 DMA 写入。CPU 通过原始指针做 volatile 读取，绝不使用 Rust 引用。
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
    pub sector: u8,
    pub logical_bridge: Bridge,
    pub outputs_armed: bool,
    pub state: u8,
    pub fault: u8,
}

// main 在解除中断屏蔽前完成初始化。电机中断均为 P1，不能相互嵌套。
// 紧凑的前台循环仅在有限临界区内访问这些对象；
// 不允许引用逃逸。界面仅使用自己的消息交换区。致命异常不再返回。
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

    // 在程序整个生命周期内保留真正的 HAL 单例 token。
    // 通过临界区，将前台电机 MMIO 访问与 P1 中断处理串行化。
    let motor_peripherals = (
        p.ATIM, p.ADC1, p.ADC2, p.OPA1, p.BGR, p.BTIM1, p.BTIM2, p.BTIM3, p.PA15, p.PB3, p.PB4,
        p.PB5, p.PB6, p.PB7, p.PA0, p.PA1, p.PA2, p.PA6, p.PA7, p.PB0, p.PB2, p.PA8, p.PA10,
        p.PA11, p.PC13, p.PA3,
    );
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
        // 后续前台访问通过 critical_section 与中断处理串行化。
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
        // 开始 RTT 输出前，所有控制器借用和电机临界区均已结束。
        if let Some(snapshot) = snapshot {
            snapshot.report(last_fault);
            last_fault = snapshot.fault;
        }
        core::hint::black_box((&motor_peripherals, &adc2_stream));
    }
}

// 电机中断直接调用；前台在其有限临界区内调用。
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

    // 原源码对齐时，在 Commutation(0) 及其定时器写入完成后导通 C-。
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
        // 安全性：P1 中断不能相互嵌套；前台只在屏蔽中断时借用。
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
        // 安全性：P1 中断不能相互嵌套；前台只在屏蔽中断时借用。
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
