# 电机专用 ownership-bypass 操作 API

`embassy-cw32::motor` 单独位于 `embassy-cw32/src/motor/`。它是低层的电机硬件操作集合，
不包含开发板、控制算法、任务执行器、状态机、邮箱或通用运行时。
一般应用仍优先使用持有 `Peri` 的 GPIO/ADC/PWM/模拟驱动；只有已明确管理并发的
电机代码才使用这条显式 `unsafe` 入口。

## 边界与分层

模块与操作按元数据中的独立 IP 配置启用：ADC 为 `adc_l012`，PWM 为 `atim_l012`，
基本定时器为 `btim_l012`，引脚为 `gpio_l012`，OPA/BGR 组合为
`all(opa_l012, bgr_l012)`。没有按具体芯片型号分支，也不要求使用 ADC 时必须同时
拥有所有其他电机 IP。当前操作审查仅覆盖 L012 相应 IP，不能把它当作 F030 的
兼容寄存器接口。共享时钟门的读改写使用短临界区；不会复位整个共享外设域。

句柄不实现 `Copy`/`Clone`/`Send`/`Sync`，不接受伪造的 singleton，也不自动
接管中断。`unsafe acquire` 只创建一个短期操作句柄，**并不证明独占**：

- 调用者在整个句柄存活期间排除安全 HAL 驱动、其他 motor 句柄、PAC、可嵌套
  中断及 DMA 配置写入的干扰。只保留但不使用的 `Peri` token 可以存在。
- 原安全驱动的配置被改写后，不能仅等待 motor 句柄结束就恢复使用；必须恢复
  其全部不变量，或丢弃并重新初始化。句柄的 Drop 不恢复配置、不停外设。
- 活跃 DMA 可在已协调的协议下读取 ADC 结果，不能写入 ADC 配置；缓冲区存活、
  同步、DMA 总线完成等义务可能超出句柄生命周期。
- IRQ/executor 优先级、临界区、管脚封装/复用、VDDA/时钟、模拟建立、功率级
  极性/死区/硬件保护由板级代码负责。接口不会把 48 MHz ADC 自动视为所有供电都合法。
- 不可跨 `await` 保留会被 ISR 使用的操作句柄或软件状态借用。

## 可复用操作

### ADC 扫描与触发

`AdcScan::<peripherals::ADC1>::acquire()`、`ScanSlot`、`ScanConfig` 组合一至八个硬件序列槽。
ADC实例复用已有 `adc::Instance`，类型固定寄存器与元数据IRQ关联，不传入/伪造 `Peri`。
通道/采样时间和时钟分频由调用方指定。`configure` 的启动阶段为：
取消触发、停软件启动、禁 IRQ/DMA、读取一次 CR、仅更改文档字段并禁用 ADC、
写序列/采样、清事件、用同一 CR 保留位快照配置分频/槽数并启用 ADC。
保留实际 CR bit8 及其余保留位，不以全零或重读替代原快照。

```rust
unsafe {
    AdcScan::<peripherals::ADC1>::acquire().configure(ScanConfig {
        slots: &[
            ScanSlot::new(8, SampleTime::Cycles70),
            ScanSlot::new(0, SampleTime::Cycles70),
            ScanSlot::new(1, SampleTime::Cycles70),
            ScanSlot::new(2, SampleTime::Cycles70),
        ],
        divider: ClockDivider::Div2,
    });
}
```

`enable_sequence_interrupt::<AdcHandler>(Irqs)` 验证实际绑定后选择 EOS IRQ；
`enable_conversion_dma` 选择 EOC DMA
并明确关闭 EOS DMA。`trigger_from_pwm` 只选 ATIM OC4REFC。
`start_software` 只启动，不偷偷清 flags、取消 ATIM 触发或重排调用。
`take_sequence::<N>` 检查 EOS，先清事件再依序读取结果，不承诺结果是冻结快照。
单独的 `acknowledge_sequence` 以正确 W0C 掩码只确认 EOS，不误清其他事件。
`result_address` 供显式 unsafe DMA 传输使用，不产生内存引用或虚构所有权。

### PWM 换相、PWM-only 与致命关断

`PwmBridge` 拥有临时的 ATIM 操作域；`MotorPin` 是独立管脚操作句柄。
`PwmConfig` 明确指定周期、采样比较和是否配置相输出。采样教学程序使用
`phase_outputs: false`，始终只配置 CH4；不会触碰六个桥臂脚。

`apply(..., PhaseDrive, BridgeUpdate)` 是完整换相操作，非逐寄存器套壳：

1. 正常换相先关未选择的低侧。
2. 清零不活动的相比较值，再写入活动相的完整 `u32` 比较字。
3. 开选择的低侧，最后更新 CH4 采样比较值。
4. 全关闭桥图保留原先“先三个 CCR 清零、再三个低侧关闭、最后采样 CCR”的顺序。
5. `PwmOnly` 完全不写低侧 GPIO。不重设 mux，不每次开关 MOE，不插入更新脉冲。

```rust
unsafe {
    let mut lows = [
        MotorPin::acquire(PinId::new(Port::A, 15)),
        MotorPin::acquire(PinId::new(Port::B, 3)),
        MotorPin::acquire(PinId::new(Port::B, 4)),
    ];
    PwmBridge::acquire().apply(&mut lows, PhaseDrive {
        pwm_counts: [500, 0, 0], low_sides: [false, true, false],
        sample_compare: 300,
    }, BridgeUpdate::Commutate);
}
```

比较值不截断为经过比例变换的新控制量；完整比较字延续原 C 的边界。
调用者负责有效范围和电气安全。正常控制错误仍由板级原控制器在原派发位置处理。

`arm_outputs` 是另外的显式 unsafe 启动操作：检查硬件 break latch，保持 AOE=0，
写 MOE 后再次检查；有故障则拒绝/断开，不清故障。最初已存在故障时也先关闭
AOE/MOE和相通道再返回错误，不能因“拒绝启动”而留下接管前已经开启的功率。`disable_outputs` 在同一个BDTR写入中关闭AOE及MOE，再关闭
相通道，留下 CH4。其他操作不隐式重新开 MOE。没有自动恢复功率接口。

不可返回的致命异常使用 `emergency_disconnect`：MOE 关闭 → 相通道关闭 →
按板级列表分组清 GPIO latch → 高侧恢复 GPIO mux → 相 CCR 清零。
它在关闭MOE的同一写入中也清AOE，不借用被打断的软件状态、不清 break latch；调用者必须屏蔽可屏蔽中断，
永久放弃被打断的操作且永不返回。常规 `acquire` 的排他承诺不能当作
可返回异常里的通用别名许可。即使接管的ATIM此前AOE=1，两个关闭接口也会清AOE，
防止硬件自动重新开MOE；当前板级初始化/显式启动原本已使AOE=0，原有效轨迹不变。

这些例程原本没有接入有效的硬件瞬时过流/刹车路由，初始化也不擅自增加它。
API 的硬件故障观察/关闭能力不能替代外部保护。

### 基本定时器、管脚与电流前端

`BasicTimer::<peripherals::BTIM1>::acquire()` 提供重复计数配置、使能中断源的检查/确认、原始计数、计数预置，
以及“ARR → CNT=0 → 重复使能”的 `arm`。不会替换成单次模式、添加回卷补偿，
或在重新 arm 时偷偷清掉原待处理事件。BTIM2/3 的周期、8 MHz tick 仍由板级指定。

`MotorPin` 使用元数据 `Port` 与经过范围校验的 `PinId`，支持模拟/输入/输出配置、
原子 SET/CLR 与仅修改 mux。板级复用号和桥臂位置不写进 HAL。
`configure_current_sense` 明确选择 OPA1 外部反馈正负输入；模拟脚、增益电阻和
启动等待仍留在板级代码。OPA CR以经审查的复位值为起点，保留BIAS=7，
对应INP2/INN2配置字0xe220、启用后0xe221；不把未知寄存器配置全部写零。
它不创建一个虚假的安全模拟源借用。

## 真正的 Embassy 中断绑定

`AdcScan<T>` 与 `BasicTimer<T>` 保留具体外设的关联中断类型。
ADC复用已有 `adc::Instance::Interrupt`；基本定时器的 `BasicTimerInstance` 由
芯片元数据生成寄存器和 `GLOBAL` IRQ关联。ADC1对应ADC1、ADC2对应ADC2_DAC，
BTIM1/2分别对应BTIM1/2，BTIM3对应共享BTIM3_HALLTIM。

`enable_sequence_interrupt::<H>(binding)` 和 `enable_update_interrupt::<H>(binding)`
都要求真正的 `interrupt::typelevel::Binding<T::Interrupt, H>`，其中
`H: Handler<T::Interrupt>`。错误IRQ、错误Handler或省略proof无法通过编译。
这里使用官方 `embassy-hal-internal` 的类型和契约；没有另造一个同名标记、
运行时回调表、注册器或事件搬运层。`acquire` 的unsafe独占义务仍独立存在，
绑定proof并不证明别名安全、优先级、ISR业务逻辑正确或硬件独占。

板级应用声明handler并由真实 `bind_interrupts!` 生成向量和proof，例如ADC部分：

```rust
use embassy_cw32::{
    interrupt::typelevel::{self, Handler},
    motor::AdcScan,
    peripherals,
};

struct AdcHandler;
impl Handler<typelevel::ADC1> for AdcHandler {
    unsafe fn on_interrupt() {
        // SAFETY: the application has established the non-nesting motor domain.
        let Some(raw) = (unsafe { AdcScan::<peripherals::ADC1>::acquire() })
            .take_sequence::<4>() else { return };
        // Consume this sample and perform required control/commutation here.
        core::hint::black_box(raw);
    }
}
embassy_cw32::bind_interrupts!(struct Irqs { ADC1 => AdcHandler; });

// During serialized setup, after configuring the ADC and the shared state:
unsafe {
    AdcScan::<peripherals::ADC1>::acquire()
        .enable_sequence_interrupt::<AdcHandler>(Irqs);
}
```

使能操作只写外设中断源，不偷偷改NVIC优先级、mask或pending，也不清事件。
板级仍先完成源确认、状态初始化和P1设置，再在原时序位置使用typelevel IRQ使能。
共享向量中每个handler只服务自己的来源；BTIM3_HALLTIM不做NVIC `unpend`，
以免抹去HALLTIM待处理事件。当前05/06不启用HALLTIM；若以后启用，必须在同一个
`bind_interrupts!` 列表添加其handler，并一起审查向量mask/优先级的所有权。
BTIM3 handler先检查并确认自己的UIF/UIE，不清兄弟外设状态。

02–05各自在main声明 `Irqs` 并保留原同步ISR主体。06在main集中声明ADC1、BTIM1、
BTIM3_HALLTIM和UART2绑定；`motor::start(..., Irqs)` 要求
`Binding<typelevel::UART2, MotorExecutorHandler>`。UART2 handler只调用官方
`InterruptExecutor::on_interrupt`；设置P1后把 `typelevel::UART2::IRQ` 交给其
`start`，由官方执行器先初始化再unmask。UART2外设保持未使用。
ADC采样、滤波和即时换相、BTIM节拍及定时换相仍在各自handler同步执行，
只有已更新状态的检查通知会唤醒原P1任务。HardFault/NMI/Panic仍是异常路径，
不迁入外部IRQ绑定。

对照实际固定版本的[中断契约](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/interrupt.rs)、
[绑定宏](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs)
与[执行器启动顺序](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-executor/src/platform/cortex_m.rs)。
v0.16.0移除动态 `AdcUnit`/`TimerUnit` 选择并让IRQ使能必须提供proof，属于显式API变更。

## 与真正 DMA 驱动配合

DMA 不是 motor 文件夹中的“启动若干寄存器”代用品。`embassy_cw32::dma` 由
独立 channel token 驱动；例程将 `p.DMA_CH2` 交给
`Channel::new_blocking` 并保留 `start_repeating_raw` 返回的 guard。

ADC2 保持 `ADC2_SINGLE`、BLOCK、32 位、计数 5、REPEAT=1、双地址自增、RESTART。
外设 EOC 请求在 DMA 配置完后才打开，ATIM 路由及最初的软件启动仍在原顺序位置。
每次转换更新一个槽；CPU 使用原始 volatile 读，可能混合两个序列，不能称为
EOS 整体快照或无损采样队列。存储是静态数组，活跃 DMA 期间不创建 Rust 切片引用。
DMA guard 常驻整个程序；它不与 ADC/定时器 ISR 分享可变 Rust 句柄。

## 六个板级 crate 的迁移清单

- 01 保留普通安全 GPIO 驱动，不为了展示 bypass API 而增加不必要的 unsafe。
- 02–04 使用 motor 的 ADC/OPA/CH4/BTIM1 操作，DMA 使用独立通道驱动；
  六个桥臂脚仍完全不配置。观察/六步表/过零算法保留在各自 crate。
- 05 保留直接 main/ISR 与临界区前台访问；控制和保护模块内容未改。
- 06 main 只做时钟、通道/token 拆分与任务启动；motor.rs 保留板脚、调参、
  控制器、事件唤醒与 ISR。P1 电机 InterruptExecutor 和普通线程 UI 保持分离；
  不把计算或硬件事件改成固定 1 ms 轮询。
- 06 UI 使用普通安全 GPIO 和拥有 UART1/PB12/PB11 的 UART HAL；类型化引脚
  约束、计数门控和原每次唤醒最多一字节的轮询语义均由正常驱动承担。

按含 `pac::` 的源代码行统计：02/03/04/05 从 85/84/85/139 行降为 0；
06 在 v0.15 从149行降为10行，v0.19进一步移除 UI UART1 路径后为0行。
全部六个例程均不依赖 `unstable-pac` feature。六个 crate 仍独立，没有新增共享业务库，
也没有测试/Python/主机入口等附加骨架进入例程。

## 验证边界

六级默认与 05/06 `motor-output-enable` 均经 Thumb ARM release 构建。
控制、保护、协议、帧队列与过零算法文件和迁移前逐字节一致。
另在发布树外将真实 motor 源码与生成 PAC 接入记录式 MMIO，比较原桥臂写序，
覆盖 ADC 保留位、EOS、PWM 初始化/故障、定时器与致命关断；结果见
[原工程一致性审查](bldc-source-parity.md)。

以上不包含真实电气波形、DMA 总线行为、最坏 IRQ 延迟或上板验证。
