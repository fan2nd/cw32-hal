> 历史版本验证记录。本文涉及的测试文件和旧例程已从当前源码移除；当前构建说明见 README。

# v0.11.0 验证记录与 v0.11.1 简化说明

> 本文为 v0.11.0/0.11.1 历史记录；其中 lib.rs 与 01–04 门极拉低行为在 v0.11.2 已移除。当前结构与验证见 [v0.11.2](validation-v0.11.2.md)。

日期：2026-10-02。Linux x86_64，Rust/Cargo 1.99.0，目标 thumbv6m-none-eabi。没有实板、烧录、母线上电、带载或示波器测试。

## v0.11.1 例程简化

六个 BLDC crate 直接面向 MCU；移除例程单元/集成测试、主机 smoke 入口、目标平台条件分支及 `firmware` feature，硬件依赖无条件启用。只有 05/06 保留 `motor-output-enable`。未调用的 PI 算法及其历史测试一并移除。CI 直接构建六个默认 ARM 程序和 05/06 输出 opt-in；基础生成器、PAC、HAL 与 xtask 测试保留，主机测试显式按包选择。

v0.11.1 实际执行：

- 六个默认 ARM release 程序联合构建通过；分别进入六个编号目录，使用板级 `.cargo/config.toml` 的默认 target 构建也通过。
- 05/06 联合启用 `motor-output-enable` 的 ARM release 构建通过。
- `cargo fmt --all -- --check` 与 `cargo run -p xtask -- regenerate --check` 通过，生成结果逐字节一致。
- 当时的基础 crate 主机检查 通过；显式 ignored 测试未在本轮执行。

本轮没有实板测试；以下保留 v0.11.0 的历史验证范围，不表示其余项目已在 v0.11.1 重跑。

## 六个真实独立 crate

`example/l012-bldc/01-gpio` 至 `06-application` 各有自己的 Cargo.toml、build.rs、src/main.rs、src/lib.rs。旧公共完整 Board/Stage 运行器与旧包已移除。各级代码按必要范围分开，05/06 不再决定早期例程的硬件初始化。

- 01：默认 4 MHz GPIO，PC13/PA3 加六个安全低电平门极控制 GPIO；不使用裸 PAC、ADC、OPA、ATIM、BTIM、UART、executor 或电机包。
- 02：独立 ADC/OPA/BGR 采样板；ATIM 仅 CH4 做采样触发，CH1–3 断开，MOE 为零；BTIM1 提供采样节拍。没有 UART、BTIM2/3 资源/寄存器初始化、换相状态机或 arm API。
- 03/04：复用聚焦采样模块，分别增加桥图和被动检测，各有独立小型 runtime。01–04 不定义 motor-output-enable，Cargo 请求它会失败。
- 05：必要启动、六步、保护和四个真实中断；删除 UART 时钟/引脚/协议、遥测、邮箱、executor、未调用 PID。
- 06：只依赖 05 纯算法，自己的 Board::new 返回独立 (Board, Ui)，UI 自创建起就归真实 Embassy 任务，不在电机 Board 内保留 Option<Ui>。有界命令、反馈、完整帧队列和 100 ms UI watchdog 保留。
- 05 IRQ 向量导出归其 binary；修复 Cargo feature 统一时同时构建 05/06 会重复 ADC1 符号的问题。分别及同时构建两包均通过。

BTIM1–3 共用 APBEN2.BTIM123 门控位。02–04 为 BTIM1 必须打开共享门控；“不使用 BTIM2/3”指不持有、配置、启动或安装它们的 ISR，不声称共享时钟位保持关闭。

## v0.11.0 基础组件实际执行

- `cargo fmt --all -- --check` 通过。
- L012 HAL 84 单测、IRQ 分发 1、布局 5、29 doctest；F030 HAL 76 单测、IRQ 分发 1、布局 5、16 doctest，均通过。两芯片 PAC 各 4 项 API 测试通过。
- L012 三项、F030 一项原有 ARM 运行时 ELF 测试显式执行通过。
- 两芯片各 8 组 ARM feature check（chip、rt、memory-x、metadata、unstable-pac、defmt、time-driver-gtim1、time-driver-any）通过；缺芯片、双芯片、封装后缀和 SysTick 时间驱动 feature 按预期拒绝。
- 原有 L012/F030 共 8 个 HAL 例程 ARM release 构建通过。
- 统一 cw32-gen 85 项测试通过；xtask regenerate --check 通过；隔离 YAML pin/offset/base/IRQ 变异流水线通过。依旧严格 YAML → 落盘 JSON → PAC，没有改成隐式生成或多个生成器。

依赖 proc-macro-error2 2.0.1 的 future-incompatibility 提示仍由当前 Rust 报告；此次构建均成功，未自行升级固定依赖。

## 保留与明确的行为差异

ADC 6 MHz、ADC1 四槽 22 µs、ADC2 五槽约 444.167 µs/5 ms、20 kHz ATIM、05/06 8 MHz 步进计时与六毫秒 bootstrap 保留。05/06 默认关闭；opt-in 才有 bootstrap 和后续按键启动授权。

03/04 上电调试桥图显示扇区 0 的逻辑图，而不是旧统一 runtime 的初始 off 图；物理桥臂仍全低。04 保守拒绝零/超范围母线样本与超过 20 ms 的 ADC2 旧样本。原工程未调用的 PI 不属于运行功能。

以上仅证明源码隔离、模型回归、编译/链接和已审核的寄存器契约。仍需实板验证中断最坏延迟、ADC 相位偏斜/噪声、OPA 建立、换相波形、短路/失步/欠压、锁轴、外部硬件关断和实际供电限制。见[板级说明](../examples/l012-bldc/README.md)和[原始功能验证](validation-v0.10.0.md)。

## v0.11.0 干净源码包验收

从没有 Cargo.lock、JSON/PAC 生成树或 target 的隔离源码副本，离线重新解析缓存依赖、运行 regenerate/--check。该副本六个默认 ARM 固件联合构建，以及 05/06 opt-in 联合构建均通过。

source-only ZIP 仅包含维护源码、文档、YAML 与固定原厂证据；不含生成 JSON/PAC、Cargo.lock、target、缓存、日志、Python、原 SDK ZIP 或旧完整板级包。ZIP 解包后与最终维护树逐文件、逐字节检查一致；所有本地文档链接检查通过。
