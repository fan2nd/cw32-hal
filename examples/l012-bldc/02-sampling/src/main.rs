//! 采样流程直接展示在 main 和两个同优先级 IRQ 中。
//! ATIM CH4 仅用作内部 ADC 触发源，不操作桥臂栅极引脚。
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use defmt_rtt as _;
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
const SAMPLING_PERIOD: u16 = 4800; // 96 MHz / 20 kHz，沿用原始计数尺度。
                                   // 配置完成后仅由 DMA 写入；CPU 通过裸指针进行易失读取，绝不创建 Rust 引用。
static mut ADC2_DMA: [u32; 5] = [0; 5];
static INITIALIZED: AtomicBool = AtomicBool::new(false);

// 解除中断屏蔽后，由 ADC1 和 BTIM1 写入这些观测变量。
// 两者均运行在 P1 优先级，不能相互抢占。主循环仅在屏蔽中断时复制诊断
// 快照，然后在该作用域外记录其独立持有的副本。
// NMI/HardFault 不访问任何软件状态，并在进入不返回的路径前关闭硬件。
// 其他处理程序不得访问这些变量或更改 IRQ 优先级。
static mut ADC1_RAW: [u16; 4] = [0; 4];
static mut ADC2_RAW: [u16; 5] = [0; 5];
static mut ADC1_SEQUENCES: u32 = 0;
static mut ADC2_SEQUENCES: u32 = 0;
static mut MILLISECONDS: u32 = 0;
static mut ADC2_ELAPSED_MS: u8 = 0;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("02 sampling boot; motor gate pins untouched");
    let mut config = embassy_cw32::Config::default();
    config.rcc.hsi_divider = rcc::HsiDivider::Div1;
    config.rcc.pclk_divider = rcc::PclkDivider::Div1;
    // 在程序的整个生命周期内保留 HAL 单例令牌。
    let _peripherals = embassy_cw32::try_init(config).expect("clock initialization failed");
    let clocks = rcc::clocks();
    defmt::info!(
        "nominal clocks: sysclk={} Hz hclk={} Hz pclk={} Hz",
        clocks.sysclk_hz(),
        clocks.hclk_hz(),
        clocks.pclk_hz()
    );
    let mut adc2_channel = dma::Channel::new_blocking(_peripherals.DMA_CH2);
    let adc2_stream;
    typelevel::ADC1::disable();
    typelevel::BTIM1::disable();
    // 安全性：HAL 初始化已移交外设单例；
    // main 保留这些单例令牌；解除中断屏蔽后，main 不再访问硬件。
    unsafe {
        // 仅配置 LED/按键和模拟输入，不操作桥臂栅极引脚。
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
        // 采用外部反馈的 OPA1：PA6 接 INP2，PA7 接 INN2，PB0 为输出/ADC1 CH8。
        motor::configure_current_sense(PositiveInput::Inp2, NegativeInput::Inn2);
        // 在此显式列出板级通道和采样时间。
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
    // 标称启动稳定等待时间 >= 1 ms，不用作采样时基。
    cortex_m::asm::delay(CPU_HZ / 1_000);
    INITIALIZED.store(true, Ordering::Release);

    unsafe {
        AdcScan::<peripherals::ADC1>::acquire().clear_events();
        AdcScan::<peripherals::ADC2>::acquire().clear_events();
        AdcScan::<peripherals::ADC1>::acquire().enable_sequence_interrupt::<AdcHandler>(Irqs);
        // 保留原始 EOC + BLOCK 设计意图：每次转换传输一个 32 位结果。
        // 明确修正为 ADC2_SINGLE (15)，而非原始代码中不匹配的 SEQ (14)。
        // 显式禁用 EOS DMA；CNT=5、REPEAT=1，源地址和目标地址均递增。
        // 此静态存储仅由 DMA 写入；即使传输被取消，其生命周期仍长于传输流。
        // CPU 仍按字进行易失读取：一次扫描的数据可能仅有部分已刷新。
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
        BasicTimer::<peripherals::BTIM1>::acquire().start(); // 沿用原始定时器使能操作。
        PwmBridge::acquire().start();

        AdcScan::<peripherals::ADC2>::acquire().start_software();
        // 在任何 IRQ 能够运行前，完成初始化并结束所有借用。
        core::sync::atomic::compiler_fence(Ordering::Release);
        typelevel::ADC1::enable();
        typelevel::BTIM1::enable();
    }
    defmt::info!(
        "sampling started: ADC1 20 kHz trigger; ADC2 live DMA snapshot may contain a partial scan"
    );
    let mut last_log_ms = 0u32;
    loop {
        core::hint::black_box(&adc2_stream);
        cortex_m::asm::wfi();
        // 每秒仅复制一次。IRQ 中从不记录日志，ADC2 原始值
        // 仍表示实时 DMA 数据（可能仅部分刷新），其原有语义不变。
        let snapshot = cortex_m::interrupt::free(|_| unsafe {
            let ms = MILLISECONDS;
            if ms.wrapping_sub(last_log_ms) < 1_000 {
                None
            } else {
                Some((ms, ADC1_RAW, ADC2_RAW, ADC1_SEQUENCES, ADC2_SEQUENCES))
            }
        });
        if let Some((ms, adc1, adc2, adc1_sequences, adc2_sequences)) = snapshot {
            last_log_ms = ms;
            defmt::info!(
                "ms={} ADC1 raw={} seq={} ADC2 live raw={} observed_eos={}",
                ms,
                adc1,
                adc1_sequences,
                adc2,
                adc2_sequences
            );
        }
    }
}

struct AdcHandler;
impl Handler<typelevel::ADC1> for AdcHandler {
    unsafe fn on_interrupt() {
        // 安全性：仅有不会相互嵌套的 P1 处理程序访问这些静态变量。
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
    // 即使异常打断了 ISR，也不借用任何软件状态。
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
