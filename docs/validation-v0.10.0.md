# v0.10.0 验证记录

日期：2026-10-02。Linux x86_64，Rust/Cargo1.99.0，目标thumbv6m-none-eabi。
新增按板组织的 `example/l012-bldc/01-gpio` 至 `06-application`，迁移上传CW32L012六步无感BLDC程序。没有连接开发板，没有烧录、母线上电、带载或示波器测试。

## 新增实现

- 原六步表、150ms定位、400ms延时、强制启动最多200步、15次连续过零接管、退磁/延迟换相、占空比斜坡、停机与故障状态。
- ADC参考标定换算、原电压/电流/NTC门限与计数、按键六档、100ms速度/保护、500ms遥测、10s闲置关机、5s堵转；未使用的原PI独立保留，不宣称速度闭环。
- 默认禁止功率输出；01–04即使启用输出feature也不能打开桥臂。05/06需显式feature才授权。
- 完整06使用真实Embassy executor；电机外设/pin tokens转移到IRQ域，PA3/PC13/UART及引脚独立转移给task，有界值邮箱交接，无steal、无重复外设所有权。
- 确认的96MHzCPU/48MHzPCLK、20kHzPWM、8MHzBTIM、6MHzADC、保留寄存器位与OC4REFC PWM2触发适配详见[板说明](../example/l012-bldc/README.md)。默认HAL4MHz/F0308MHz保持不变。

## 安全修正

与C源有意不同：启动失败不能被后续Run状态覆盖；取消延时不再继续启动；占空比钳位；零/擦除标定拒绝；ADC1连续2ms、ADC2连续20ms缺失时故障停机；ADC数据不一致风险在板层立即锁定关闭；故障与停止立即移除桥臂并取消旧换相事件。

06的任务输入只有连续新样本才能完成60ms按键去抖；2ms未刷新按键即按释放处理，100ms未刷新且有运行/启动请求时故障停机。恢复任务本身不清除故障。UART采用当前完整帧+最新待发帧，最多14字节，不忙等，不拼接半帧。

板级桥臂事务经主机mock验证先关闭旧输出、断开通道、写比较值、在断开通道时打开主输出并核查break、连接通道并再次核查、最后打开低侧。此顺序仅证明寄存器操作，不证明MOSFET死区；GPIO低侧不经过ATIM硬件死区电路。Panic/HardFault/NMI/未处理IRQ均走输出关闭路径。

## 软件检查

- 新增BLDC包：**72项主机测试通过**，包括控制器33、保护15、PI7、协议3、有界邮箱5、板级时序/桥臂事务/UART队列9。
- L012配置workspace：**280项通过**，即新增72 + 生成器85 + PAC API4 + HAL84 + IRQ分发1 + 模块布局5 + doctest29。HAL84中RCC7项，新增高频切换先后次序、保留位/回读与默认时钟回归。
- F030 HAL：**76单测 + 1 IRQ + 5布局 + 16 doctest**通过，另PAC API4项通过；不重复累计共用生成器。
- 新增ARM ELF测试显式执行通过：6个编号程序×默认关闭/显式输出授权两种feature，共**12个最终ELF**，检查四个真实IRQ、向量/SRAM布局、HardFault与GTIM1/SysTick未被抢占。此为1项集成测试，不能冒充12项host算法测试。
- 原L0123项/F0301项最终ELF测试显式执行通过；原8个例程全部ARM编译/链接通过。
- 两芯片各8组feature ARM检查通过；无芯片、双芯片、封装后缀selector、旧SysTick驱动feature四组负向检查按预期拒绝。
- 生成漂移检查及隔离YAML pin/offset/base/IRQ mutation → JSON → PAC/HAL检查通过；xtask源码副本现在包含新的example workspace成员。
- cargo fmt检查及新增包rustdoc warnings-as-errors通过。

全部上述验收在新源码副本中重跑，起点没有Cargo.lock、generated-data、生成PAC或target；仅使用已有依赖缓存，不复用维护树生成PAC。默认ignored的ARM测试只有显式执行成功才计入ARM结果，未混入host通过数。最终ZIP从这份验收副本导出并逐文件核对。

## 必须保留的限制

- 没有电机硬件验证，也没有20kHz ISR最坏执行时间或ADC相位窗口测量。100ms保护换算在电机所有权域中完成，会消耗ISR时间。
- MCU VDD必须至少1.8V才允许96MHz；实际板子的模拟量、门极驱动、RC、保护链、外部OPA增益与电阻改装均未核实。
- 新ADC方案与上传C时序不同，顺序采样造成时间偏斜；不能仅据22µs预算认定可用BEMF闭环。
- 原工程没有实际启用HardFAULTAD/NUMtimes硬件瞬时限流，本版也没有伪造硬件刹车。需独立硬件过流关断/急停。Debugger暂停不等于安全关断。
- BTIM2沿用65530重装载、8MHz、约8.19ms回卷；10ms强制步可能看到回卷计数，这是原C语义而非32位精确长时间测量。
- `RealS` 是原工程的换相次数比例量，不是机械RPM。原PI未在运行路径调用。
- source-only ZIP排除Cargo.lock、生成JSON/PAC、target、SDK输入ZIP、日志和脚本。原上传C工程不再分发，应用来源及权利说明见NOTICE。

依赖提示：defmt feature检查仍出现 `proc-macro-error2 2.0.1` 的future-incompatibility警告，当前编译通过；本包rustdoc没有警告。Cargo.lock按要求不分发，固定直接依赖不等于固定全部未来传递依赖解析。
