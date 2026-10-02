# L012C8：真实 IRQ 驱动的异步 HAL

本文逐项说明L012C8接口。v0.9新增F030C8的独立ADC/VC/ATIM后端，其通道数、事件名和硬件约束不可套用本文的L012参数，见[F030支持说明](cw32f030-support.md)。两个chip都不选择封装；board检查实际引出。

本版为 CW32L012C8 的 ADC1/2、VC1～4、ATIM 事件和 CORDIC 提供硬件中断驱动的 future。等待尚未完成时，future 注册 waker 并返回 `Pending`；外设的真实 IRQ handler 检查本实例的使能与状态，处理本源标志，发布事件并唤醒任务。没有把有界 busy loop 外包一层 `async fn` 作为这些接口的实现。

这仍是实验性 HAL：没有上板验证、连续 DMA 采样或完整 FOC 闭环。初始化及模拟稳定延时仍是同步过程；异步运算也有有限的寄存器配置与结果复制工作，并非零 CPU 开销。源代码和公开签名是接口依据；实际执行过的测试、链接及未执行项目见 [validation.md](validation.md)，本文不预先宣告验证结果。

## 1. 绑定、所有权与启动

`init(Config) -> Peripherals` 和官方 `Peri<'d, T>` 所有权模型保持不变。先按既有构造器创建阻塞 owner，再用 `into_async(Irqs)` 消耗该 owner。转换保留其外设、引脚及模拟参考源的 lifetime；不是复制资源，也不允许在另一个任务中同时使用原阻塞对象。

使用默认 `rt` 时，`bind_interrupts!` 安装与 PAC 物理向量相连的分发函数，并给出 `Binding<IRQ, Handler>` 证明。以下清单展示各异步实例的真实向量；应用只需列出自己使用的实例，并把同一物理 IRQ 的所有 handler 放在同一绑定中。

```rust
use embassy_cw32::{adc, analog, atim, bind_interrupts, cordic, peripherals};

bind_interrupts!(struct Irqs {
    ADC1 => adc::InterruptHandler<peripherals::ADC1>;
    ADC2_DAC => adc::InterruptHandler<peripherals::ADC2>;
    VC13 => analog::InterruptHandler<peripherals::VC1>,
            analog::InterruptHandler<peripherals::VC3>;
    VC24 => analog::InterruptHandler<peripherals::VC2>,
            analog::InterruptHandler<peripherals::VC4>;
    ATIM => atim::InterruptHandler;
    CORDIC => cordic::InterruptHandler;
});
```

每个 `into_async` 都要求匹配的 Binding；`()` 不能替代。转换后驱动可开启对应 NVIC IRQ，单次 future 在首次 poll 时开启自己需要的外设事件。转换本身不替应用完成所有外设启动：例如 ATIM counter 仍须显式启动，等待 ADC 的 ATIM trigger 也不启动 ATIM。

ADC2 与 DAC 共用 `ADC2_DAC`；VC1/VC3 共用 `VC13`，VC2/VC4 共用 `VC24`。handler 只服务自己启用的源，取消不得清除兄弟实例的状态或禁用/清 pending 整条共享 NVIC 向量。若应用另外启用 DAC 中断，必须自行提供并在同一 `ADC2_DAC` 分发中加入相应 handler；本版 DAC 没有异步中断驱动。绑定清单不是对未启用外设中断的自动支持。

每个等待借用 `&mut self`，同一 owner 不能并行发起两个等待；不同外设 owner 可以作为不同任务或 `join` 的输入。不能通过同时调用 ATIM 的 `wait_update` 和 `wait_break` 来获得两个独立订阅者。事件锁存为单次通知，并非无界事件队列。

## 2. ADC：单次完整序列

实际接口见 [adc/l012.rs](../embassy-cw32/src/adc/l012.rs)：

- `Adc<'d, I, N>::into_async(binding) -> AsyncAdc<'d, I, N>`。
- `AsyncAdc::sample(&mut self).await -> Result<[u16; N], adc::Error>`：启动一次软件触发序列。
- `AsyncAdc::sample_atim_update(&mut self, &ThreePhasePwm<'_>).await -> Result<[u16; N], adc::Error>`：等待 ATIM update TRGO 触发的序列。
- `AsyncAdc::into_blocking(self) -> Adc<'d, I, N>`：归还同一 ADC 与输入 pin 给阻塞接口。
- `watchdog`、`watchdog_fault`、`acknowledge_watchdog` 仍为同步配置/查询；不提供 watchdog IRQ future。

`sample` 使用 EOS（整个序列结束）而不是把单通道 EOC 当作全部完成。handler 关闭后续 trigger 接收；如果此时另一个序列已经开始，则等它完成后再发布结果，以保证复制时数据不再变化。因此 ATIM 模式可能返回比首个 EOS 更晚的完整序列，**不保证第一个触发、每个触发、样本无丢失或固定控制延迟**。触发路由在首次 poll 才 arm；此前的触发不会被追溯捕获。

构造器的外部 pin 路由、最多八项序列、显式 sample time、保守 ADCCLK 限制和启动等待不变。返回的是未校准 12-bit 原始码，不是电流/电压。`sample_pair` 的 ADC1-master/ADC2-slave 接口仍为有界阻塞版本；不要把两个独立 `sample().await` 当作已证明同步的双 ADC 硬件采样。

取消正在等待的 ADC future 会关闭本 ADC 的 EOS interrupt、清除 trigger 路由、停止转换并复位序列索引、清本次 EOS 和注册的 waker。它不 pulse ADC1/ADC2 的共享 reset，不关闭共享 gate/NVIC。成功完成后的 guard 也执行本次操作清理。没有内置 async 超时；外部 timeout/select 的落败 future 被实际 drop 时才进行取消。

## 3. Comparator：边沿与电平通知

`Comparator<'d, I>::into_async(binding) -> AsyncComparator<'d, I>` 保留 Bandgap、DAC/RefDivider 及输入 pin 的全部借用。见 [analog/l012.rs](../embassy-cw32/src/analog/l012.rs)。

以下方法都返回 `()`，并通过 `.await` 等待：

- `wait_for_rising_edge(&mut self)`、`wait_for_falling_edge(&mut self)`：首次 poll arm 之后的新边沿。
- `wait_for_any_edge(&mut self)`：任一边沿；返回值不含方向。
- `wait_for_high(&mut self)`、`wait_for_low(&mut self)`：当前电平已满足时可立即完成，否则等待相应事件。
- `is_high()`、`number()` 保留同步查询。

边沿和电平均对应已配置 filter/polarity 后的输出。启动新的边沿等待会丢弃先前的陈旧 INTF；硬件单个 INTF 会合并多个边沿，不能用于无损计数或反推出完整波形。电平等待表示条件曾被观察/通知，恢复执行时电平可能已经改变；有需要应再次查询。

取消只撤销本 VC 的事件选择/IE、清本 VC 的 INTF 和 waker，不关闭模拟 comparator，也不触碰共享向量的兄弟 VC。drop 整个 `AsyncComparator` 才继续执行内部 Comparator 的关闭和 pin 断开。当前没有 `AsyncComparator::into_blocking`；不要假定所有 wrapper 都提供逆转换。整个对象的 drop 不等于释放借入 DAC 的某一个通道：DAC 依赖仍按已有整体借用模型处理。

等待 VC 边沿不是硬件电机跳闸：本版没有配置 VC→ATIM break 内部保护路线，不能用 executor 被唤醒后的软件动作替代经过板级审查的硬件保护。

## 4. ATIM：update 与 break 等待

`ThreePhasePwm<'d>::into_async(binding) -> AsyncThreePhasePwm<'d>` 保留整个 ATIM 和七个 pin。wrapper 通过 `Deref`/`DerefMut` 保留原有同步 PWM 配置与显式控制。见 [atim/l012.rs](../embassy-cw32/src/atim/l012.rs)。

- `wait_update(&mut self).await` 等待首次 poll 之后的 update；返回 `()`。它不启动 counter；应用须先显式 `start_counter()`。多个 update 可以合并，不是周期计数器。
- `wait_break(&mut self).await -> BreakFlags` 返回新 break 或已有锁存 break。`BreakFlags` 提供 `external_break()`、`second_break()`、`system_break()` 查询；能识别相应状态不表示安全 API 已能配置第二路 break。

等待不会写 MOE 来开启功率输出，不会调用 `acknowledge_fault`，也不自动 re-arm。break handler 保留硬件故障锁存；应用须另行显式确认故障和决定是否重新启用输出。取消 update/break future 只关闭本次 UIE/BIE 订阅及 waker，**不会停止正在运行的 counter、关闭已开启的功率输出、清故障或关闭 BKE/BK2E 硬件保护**。取消等待不是急停接口。

drop 整个 async owner 时会撤销订阅，并由原 `ThreePhasePwm` 的 Drop 关闭输出/counter、断开 pin。当前没有 `AsyncThreePhasePwm::into_blocking`。`sample_atim_update` 接受对底层 PWM 的共享借用；它并不允许同时对同一 timer 发起需要 `&mut self` 的事件等待。

构造时 MOE/CCER/CEN 的无功率默认状态、外部 BK 必需、AOE 关闭以及显式故障恢复边界保持不变。CCER=0 对应高阻，不保证 pin 被主动拉低。详见 [FOC 安全边界](foc-hal.md)。

## 5. CORDIC 与仍为阻塞的硬件

`Cordic<'d>::into_async(binding) -> AsyncCordic<'d>` 使用真实 CORDIC IRQ；`into_blocking(self) -> Cordic<'d>` 可转回原接口。异步方法保持原运算的输入/返回类型，去掉阻塞版末尾的 `poll_budget`：

- `sin_cos(angle_pi: Q31) -> Result<SinCos, Error>`
- `polar(x: Q31, y: Q31) -> Result<Polar, Error>`
- `half_magnitude(x: Q31, y: Q31) -> Result<Q31, Error>`
- `atan(y: Q31, scale: u8) -> Result<Q31, Error>`
- `hyperbolic(z: Q31) -> Result<Hyperbolic, Error>`
- `atanh(y: Q31) -> Result<Q31, Error>`
- `ln_scaled(x: Q31, scale: u8) -> Result<Q31, Error>`
- `sqrt_scaled(x: Q31, scale: u8) -> Result<Q31, Error>`

以上在调用时产生 future，结果须 `.await`。最后一个必要操作数写入启动硬件；IRQ 检查 IE/EOC/BUSY，读取结果并唤醒等待任务。结果端口的读取会清 EOC，因此不是无副作用的任意读取。Q1.31、收敛域、scale 和 **half magnitude** 定义保持原约束；异步不会扩展数值范围，也没有证明硅上计算精度。

取消会 mask CORDIC IE、复位专用 CORDIC accelerator 并丢弃该结果/waker，随后可开始新操作；不会 pulse 其他模拟外设的共享 reset。异步方法没有内置超时。具体操作定义见 [cordic/l012.rs](../embassy-cw32/src/cordic/l012.rs)；RM §12.6.1 对多结果读清顺序的证据边界仍见 [逐外设语义审查](full-peripheral-semantics.md)。

EAU 的除法/平方根只有 BUSY/结果接口，没有经审查的完成 IRQ；OPA 的校准只有 AZRUN 等状态，没有完成 IRQ。这两者保留明确的有界阻塞接口，不将忙轮询伪装成 interrupt-driven async。OPA/DAC/Bandgap/RefDivider 的配置和稳定等待也仍为同步调用。依据为 RM §§11.4–11.7、§§29.3–29.6 及 [对应源审查](full-peripheral-semantics.md)；ADC/VC/ATIM 分别依据 RM 第25章、第27章、第17章的状态/使能/清除语义。

## 6. executor、超时与时间驱动

外设 async 方法依赖自己的 IRQ 与 executor 唤醒，不要求一定启用 HAL 时间驱动。若使用 `embassy_time::Timer` 或基于它的超时，则必须另有一个实际时间后端。

`time-driver-any` 是 `time-driver-gtim1` 的真实别名：HAL `init` 在 RCC 后启动 GTIM1，并从交给应用的 `Peripherals` 中保留它。GTIM1 不占用 ATIM 或 SysTick；`Config.time_interrupt_priority` 默认 P0。4 MHz PCLK/PSC=3 提供标称 1 MHz 计数，16-bit wrap 为 65.536 ms。从 overflow 到 ISR 清 UIF 必须严格小于一个 wrap，包括临界区和 IRQ 屏蔽时间；无法从一个 pending 位恢复多个丢失的 wrap。1 μs timestamp 分辨率不保证 HSI 的实测精度或 1 μs 任务唤醒。没有 STOP/动态调频支持。

executor 由应用选择；HAL 提供 IRQ 驱动的 future。根目录板级示例包含真实 Embassy executor 的应用。

future 是惰性的：仅创建并保存不会启动等待；必须 poll/await。外部 timeout/select 若要取消操作，必须实际 drop 对应 future；`mem::forget` 或泄漏不执行取消清理。四类 future 都没有事件发生的时间保证，错误绑定、IRQ 被长期屏蔽或外部触发不出现可以让它们一直 Pending。时间后端正常运行也不代表该外设 IRQ 已正确绑定。

`cw32l012c8` + `rt` 本身不提供 critical-section backend；应用必须提供本芯片正确实现。GTIM1 feature 带入 Cortex-M `critical-section-single-core`，不使用该 feature 时可由应用自己的 cortex-m 依赖启用；不要混入主机 `std` backend 或重复的冲突实现。

## 7. 未扩展的范围与数据链路

本版没有 async UART/SPI/I2C/EXTI/DMA、ADC continuous ring buffer、同步双 ADC async 调度、VC 内部硬件跳闸路线或完整电机控制算法。有事件通知不等于硬实时控制闭环；driver/executor 的延迟、ADC 触发丢失、fault 极性和外部电气保护仍须上板测量与系统审查。

异步支持不改变单个 `cw32-gen` crate 内的 schema/data/pac 分层；仍显式执行 YAML → 落盘 normalized JSON → 重新读取 JSON → PAC/metadata/runtime，再由 HAL build.rs 消费 metadata。`all` 全芯片生成、Rust/Cargo-only 工作流及生成物 ignore 保持不变；没有以运行时解析 YAML 或手改生成 PAC 代替源数据维护。
