# 06：中断执行器电机任务与普通执行器板级 I/O

独立 MCU binary。普通 `#[embassy_executor::main]` 初始化时钟并拆分所有权：`MotorResources` 将 ADC2 DMA 通道、其余电机外设和引脚 tokens 一起转移到高优先级 `motor_task`；PA3/PC13/UART1 与串口引脚留给普通线程执行器中的 `io_task`。UART2 硬件不启用，仅占用其向量作为 P1 `InterruptExecutor` 的软件中断。

- `main.rs`：入口、时钟配置、DMA通道/所有权拆分、不可返回的异常出口。
- `motor.rs`：板级管脚与采样参数、短期unsafe HAL操作、所有权 tokens、独立DMA guard、事件驱动任务及ADC1/BTIM1/BTIM3 ISR。换相寄存器写序由HAL `motor::PwmBridge`实现，不搬入控制算法。
- `control.rs` / `protection.rs`：本地纯控制与保护算法。
- `io.rs`：按键采样、LED、非阻塞串口发送、线程 RTT 日志及少量软件命令/状态同步；不是硬件 mailbox。
- `protocol.rs` / `frame_queue.rs`：遥测帧格式与完整帧发送队列。

`MotorResources` 直接以具名字段持有全部电机 tokens：`adc1`、`adc2`、`adc2_dma` 等处于同一层级，不再嵌套外设元组。电机任务内部创建通道驱动和 repeating transfer guard，二者覆盖整个任务生命周期；DMA 的五字缓冲仍是模块内静态存储，不随资源结构移动。CPU 继续使用原始指针做 volatile 读取，不将正在 DMA 写入的缓冲改成可移动的任务局部数组或普通 Rust 借用。

ADC1 的每次采样/滤波和 BTIM3 的立即换相仍在对应硬件 ISR 完成，BTIM1 保留每个真实 1 ms tick 的原计数/按键职责。ADC/换相 ISR 仅在产生可推进的控制工作时唤醒电机任务；BTIM1 更新时限与 I/O 后通知任务。任务连续推进已就绪的有限状态延续，然后等待真实中断事件；没有忙轮询、`yield_now` 或 `Timer::after(1 ms)` 轮询。

四个电机向量（ADC1、BTIM1、BTIM3_HALLTIM、UART2）均为 P1。ARMv6-M 同优先级异常不能互相抢占，控制器/诊断/桥臂借用只存在于该域的一次有限执行中，不能跨 await。硬件事件在 ISR 内完整执行，只有“重新检查状态”的通知允许合并；置 pending 与登记 waker 也在同一不可嵌套域，避免丢失唤醒。线程 I/O 通过短临界区交换复制值，不持有电机引用。改变此优先级布局需要重新审查；同级阻塞时间、ADC 最坏延迟和线程可调度性仍需实板测量，不能由编译通过证明。

I/O 每次真实 tick 唤醒最多尝试发送一字节；迟到 I/O 的按键观察会过期，不引入原工程没有的 I/O 失联故障。HAL操作按需打开时钟，共享门和GPIO配置读改写在短临界区内完成。I/O持有普通安全GPIO与UART1/PB12/PB11驱动句柄；最终程序没有直接PAC访问，也不启用 `unstable-pac`。UART保留实际96MHz PCLK下BRRI52/BRRF1的原分频和8N1格式，名义配置115200，实际约115246baud。

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-06-application
```

不再提供例程 feature 开关，普通构建包含实际功率输出。上电先执行原有 6 ms 三个低桥 bootstrap 充电，随后关闭桥臂等待按键启动；原故障处理与 panic/异常紧急关断保留。烧录前须核对硬件并物理禁用驱动。没有 Board/Runtime/State 硬件套壳、lib target、跨例程源码依赖、主机入口或例程内测试。

原板 VDDA=5 V，96 MHz HCLK/PCLK、20 kHz PWM/4800 刻度、8 MHz BTIM2/3、ADC1 48 MHz/70 周期与 ADC2 12 MHz/518 周期不变。保留原保护暂停、故障派发先后及启动边界。ADC2 DMA 最小确定性修复与 ADC CR bit8 的新版官方库依据见[一致性审查](../../../docs/bldc-source-parity.md)，引脚和电气限制见[板级说明](../README.md)。未上板验证。

专用ownership-bypass接口的安全契约、操作清单和DMA配合见[电机API](../../../docs/motor-api.md)。unsafe acquire不证明独占；这里靠P1不嵌套域、未被其他驱动使用的tokens和不跨await的借用兑现义务。

实际电流采样电阻为 **10 mΩ**，OPA 差分增益为 10，灵敏度为 100 mV/A。`protection.rs` 将 ADC2 去零点后的电压（mV）乘 10 得到 mA，原系数已匹配实板，无须再缩小五倍。3 A 持续 30 次和 10 A 当次检查过流门限保持原值；它们使用 PB2 的滤波母线电流，不能当作 ADC1 瞬时硬件过流或相电流 RMS。零点采集、RC 滤波和日志单位见[板级电流说明](../README.md#电流采样与10-mω实板校准)。

## RTT 调试

本例默认包含 defmt RTT 日志。在本目录运行 `cargo run --release`，由 probe-rs 显示启动时钟/输出状态、启动阶段和故障；周期诊断为 500 ms，状态变化另行报告。检查 `armed`、`steps`、`crossings`、ADC 原码及 `last_protection_bus` 可定位三闪 `StartupFailed`。普通构建可驱动电机：上电先执行 6 ms 低桥充电，随后关闭桥臂等待按键启动；烧录前须物理禁用驱动。

日志只在普通线程中发出，保留原 panic/异常关闭路径；主机断开不等待，但可能丢日志。ADC2 不是原子 EOS 快照，保护电压在启动等待期间可能保持上次结果，日志仍有短暂关中断的时序代价。命令、单位与全部限制见[统一 RTT 说明](../README.md#defmt-rtt-日志01–06)。
