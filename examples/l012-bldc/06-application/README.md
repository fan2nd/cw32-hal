# 06：中断执行器电机任务与普通执行器 UI

独立 MCU binary。普通 `#[embassy_executor::main]` 初始化时钟并拆分所有权：电机外设和引脚 tokens 转移到高优先级电机任务；PA3/PC13/UART1 与串口引脚留给普通线程执行器中的 UI。UART2 硬件不启用，仅占用其向量作为 P1 `InterruptExecutor` 的软件中断。

- `main.rs`：入口、时钟配置、DMA通道/所有权拆分、不可返回的异常出口。
- `motor.rs`：板级管脚与采样参数、短期unsafe HAL操作、所有权 tokens、独立DMA guard、事件驱动任务及ADC1/BTIM1/BTIM3 ISR。换相寄存器写序由HAL `motor::PwmBridge`实现，不搬入控制算法。
- `control.rs` / `protection.rs`：本地纯控制与保护算法。
- `ui.rs`：按键采样、LED、非阻塞串口发送及少量软件命令/状态同步；不是硬件 mailbox。
- `protocol.rs` / `frame_queue.rs`：遥测帧格式与完整帧发送队列。

ADC1 的每次采样/滤波和 BTIM3 的立即换相仍在对应硬件 ISR 完成，BTIM1 保留每个真实 1 ms tick 的原计数/按键职责。ADC/换相 ISR 仅在产生可推进的控制工作时唤醒电机任务；BTIM1 更新时限与 UI 后通知任务。任务连续推进已就绪的有限状态延续，然后等待真实中断事件；没有忙轮询、`yield_now` 或 `Timer::after(1 ms)` 轮询。

四个电机向量（ADC1、BTIM1、BTIM3_HALLTIM、UART2）均为 P1。ARMv6-M 同优先级异常不能互相抢占，控制器/诊断/桥臂借用只存在于该域的一次有限执行中，不能跨 await。硬件事件在 ISR 内完整执行，只有“重新检查状态”的通知允许合并；置 pending 与登记 waker 也在同一不可嵌套域，避免丢失唤醒。线程 UI 通过短临界区交换复制值，不持有电机引用。改变此优先级布局需要重新审查；同级阻塞时间、ADC 最坏延迟和线程可调度性仍需实板测量，不能由编译通过证明。

UI 每次真实 tick 唤醒最多尝试发送一字节；迟到 UI 的按键观察会过期，不引入原工程没有的 UI 失联故障。HAL操作按需打开时钟，共享门和GPIO配置读改写在短临界区内完成。UI持有普通安全GPIO句柄；UART1驱动尚未抽象，相关十行PAC访问是最终程序仅剩的直接寄存器操作。

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-06-application
```

默认功率关闭，`motor-output-enable` 为审核硬件后的显式选择。两种构建保留相同的 6 个逻辑 bootstrap tick；只有 opt-in 构建执行实际低桥充电。没有 Board/Runtime/State 硬件套壳、lib target、跨例程源码依赖、主机入口或例程内测试。

原板 VDDA=5 V，96 MHz HCLK/PCLK、20 kHz PWM/4800 刻度、8 MHz BTIM2/3、ADC1 48 MHz/70 周期与 ADC2 12 MHz/518 周期不变。保留原保护暂停、故障派发先后及启动边界。ADC2 DMA 最小确定性修复与 ADC CR bit8 的新版官方库依据见[一致性审查](../../../docs/bldc-source-parity.md)，引脚和电气限制见[板级说明](../README.md)。未上板验证。

专用ownership-bypass接口的安全契约、操作清单和DMA配合见[电机API](../../../docs/motor-api.md)。unsafe acquire不证明独占；这里靠P1不嵌套域、未被其他驱动使用的tokens和不跨await的借用兑现义务。
