# v0.11.6 验证记录

2026-10-02，Rust/Cargo 1.99.0，目标 `thumbv6m-none-eabi`。

## 执行器与模块

06改为普通 `#[embassy_executor::main]` 运行UI，高优先级P1 `InterruptExecutor` 运行电机任务。UART2仅提供软件中断向量，其外设token由电机任务保留。ADC1/BTIM1/BTIM3仍立即执行采样、原计数与换相；ADC/换相ISR在产生可推进的状态时通知任务，BTIM1通知真实tick与时限变化。任务使用显式就绪条件推进有限延续，等待路径返回Pending，无忙轮询、yield自唤醒或固定1 ms异步轮询。

四个电机IRQ同为P1，ARMv6-M同级异常不抢占。所有控制器/诊断借用限于该域，不能跨await；通知与waker也只有该域访问。样本、定时器转换及桥臂动作先在ISR完整执行，pending只合并冗余的检查通知。共享时钟先于执行器初始化；UI GPIO初始化用一次短临界区避免寄存器读改写冲突。

`mailbox.rs`已删除。06的`motor.rs`承担真实电机PAC/任务/ISR职责，`ui.rs`承担按键/LED/UART及软件命令/状态同步，不将整个控制器包进Mutex/RefCell；纯控制、保护、帧格式与队列保留独立本地模块。05维持直接前台/ISR教学阶段，01–04未改成异步或增加外设。六例均为独立单binary，无lib、跨例程导入、例程内主机测试或平台条件入口。

## 已执行

- `cargo fmt --all --check` 与 `cargo run -p xtask -- regenerate --check`。
- 六个例程默认ARM release与05/06 `motor-output-enable` ARM release。
- L012/F030 HAL：各自最小no-default配置，以及`memory-x,metadata,unstable-pac,defmt,time-driver-any`完整配置ARM release。
- 当前06纯算法对原C外部差分重跑：56,762个电机事件、200,000个保护输入，结果一致。
- 外部事件就绪/延续验证：250,000个事件轮次，语义就绪drain与每轮12次无条件参考前台迭代的结果一致；这些随机可达路径中一次最多2个立即延续。另覆盖270,336组状态/计数/标志边界组合，最多3个立即延续；包括9/10、149/150、199/200、399/400、499/500 ms边界、强制启动过零/接管/故障、保护/斜坡/自动关机、Stop等待和全部8个状态。

ADC/PWM/BTIM/DMA寄存器配置、5 V板级参数、ADC CR bit8的新版官方SDK依据、原故障先后及未定义算术错误10边界不变。默认功率禁用，opt-in单独构建；逻辑bootstrap均为6 tick。

外部验证程序不加入发布树。编译与宿主模型不能验证真实总线/ADC建立/DMA竞争、同级中断最坏阻塞、正常线程调度裕量或实际门极波形。未上板、未烧录，不声称可直接上电运行。依赖`proc-macro-error2 2.0.1`仍有Cargo的未来不兼容提示，当前工具链构建成功。

源码归档独立解包重建结果记录于交付摘要；归档不含生成JSON/PAC、target、Cargo.lock、日志、缓存或私有验证工程。
