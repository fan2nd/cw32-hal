> Current startup-clock stage: [v0.22.0 validation](validation-v0.22.0.md), [completed scope and limits](clock-roadmap-v0.22.0.md).

> Prior timer/bus-DMA stage: [v0.21.0 validation](validation-v0.21.0.md), [scope and eight-gap progress](timer-dma-v0.21.0.md).

> Prior internal/RTC/Flash/WWDT stage: [v0.20.0 validation](validation-v0.20.0.md), [scope and eight-gap progress](analog-rtc-flash-v0.20.0.md).

> Prior bus/CRC/IWDT stage: [v0.19.0 validation](validation-v0.19.0.md), [scope and remaining work](buses-v0.19.0.md).

> Prior clock/PWM/PAC stage: [v0.18.0 validation](validation-v0.18.0.md), [scope](clock-pwm-pac-v0.18.0.md), [schema10](schema-v10.md).

> Prior resource-composition stage: [v0.17.0 validation](validation-v0.17.0.md), [scope](resource-composition-v0.17.0.md), [schema9](schema-v9.md).

> Prior interrupt migration: [v0.16.0 validation](validation-v0.16.0.md), [type-level wiring](typelevel-interrupts.md).

> v0.15.0 DMA/motor release: [v0.15.0 validation](validation-v0.15.0.md), [DMA](dma.md), [motor API](motor-api.md), [schema8](schema-v8.md).

> v0.14.0 whole-chain review: [v0.14.0](full-chain-audit-v0.14.0.md), [schema 7](schema-v7.md), [validation](validation-v0.14.0.md). Earlier version records remain historical.

> 当前结构整理检查见 [v0.13.2](validation-v0.13.2.md)。
> 当前 HAL IP/cfg 分层检查见 [v0.13.1](validation-v0.13.1.md)。

> 当前 v0.13.0 完整重构与干净源码验收见 [v0.13.0](validation-v0.13.0.md)。以下仅为历史记录。

> 历史版本验证记录。本文涉及的测试文件和旧例程已从当前源码移除；当前构建说明见 README。

> v0.11.0 六个最小独立 crate 与完整回归验收见[本版记录](validation-v0.11.0.md)。下文保留 v0.9.1 历史结果。

# v0.9.1 验证记录

日期：2026-10-02。Linux x86_64、Rust/Cargo 1.99.0、thumbv6m-none-eabi。
本记录是源码、软件模拟与真实 ARM 编译/链接证据；没有连接开发板，没有电机上电或波形测量。

## 维护树已执行结果

- `cargo fmt --all -- --check`：通过。
- `xtask regenerate` 后 `xtask regenerate --check`：schema 3 JSON、PAC、metadata、runtime 文件集合及内容逐字节一致。
- L012 配置的 workspace 测试：**204 项通过**，其中共用生成器 85、PAC API 4、L012 HAL 单测 80、共享 ISR host 分发 1、模块布局回归 5、doctest 29。3 项 ARM ELF 测试和 1 段示意代码默认 ignored，不算通过数。
- F030 配置 HAL：**76 单测 + 1 IRQ 分发测试 + 5 模块布局回归 + 16 doctest** 通过；另 F030 PAC API **4 项**通过。生成器共用测试不再累计一次。
- L012 **3 项 ARM ELF 检查**、F030 **1 项 ARM ELF 检查**均明确加 `--ignored` 执行并通过。
- 两芯片各 8 组 ARM feature 检查通过：chip-only、rt、rt+memory-x、metadata、unstable-pac、defmt、time-driver-gtim1、time-driver-any。
- 无芯片、双芯片、`cw32f030c8t7` 封装后缀 selector、`time-driver-systick` 均按预期拒绝。
- 8 个例程均编译并链接为 ARM ELF：L012 的 timer_blink / foc_peripherals / analog_foc / async_events；F030 的 f030_timer_blink / f030_pwm / f030_async_adc / f030_async_events。
- 当时的生成链路变更检查：在隔离源码副本更改 L012 chip.pins、GPIOA 基址、TOG 偏移和 IRQ 名称；原生成物漂移被拒绝，重新生成后真实 PAC/HAL 编译与 metadata 断言通过。多芯片寄存器产物集合保持正确。

## 生成器与数据

共用 85 项：access_width 10、extensions 16、f030_pins 5、full_sources 1、full_sources_f030 7、multi_chip 1、pin_routes 3、pipeline 15、runtime 3、semantics 7、sources 14、unified_cli 3。

- L012 数据仍为 50 外设视图、28 IP、306 寄存器视图、1713 字段、40 die GPIO、82 条 FOC routes；原有 L012 寄存器定义没有为了 F030 改写。
- F030 为 37 实例、20 canonical IP、32 IRQ、223 canonical 寄存器视图、1226 canonical 字段。官方 SVD 原始统计含 GPIOC/F 子集视图，共 268 寄存器、1404 字段；不是两个不同“覆盖率”。18 个独立 f030 模型，仅 IWDT/WWDT 的完整契约实证复用 l012。
- F030 39 个保守 die GPIO、61 条 ADC/ATIM/VC routes，与独立官方黄金表核对。PF3/BOOT 因原厂来源冲突不发普通 GPIO token；实际封装 bonding 属于板级责任。
- F030 70 条独立读写副作用记录，完整 header/SVD 双向集合核对，缺项/多项、偏移/宽度/access 冲突均须说明。
- schema 3 只保留 Chip.pins；维护数据目录、schema、JSON、生成 metadata 不再有 Package 模型/封装交集。旧 schema 2 或旧 package 字段被拒绝。
- 寄存器访问宽度 8/16/32 与 field mask/read/write 类型一致；LLVM 输出实证 volatile i8/i16/i32。真实 CRC DR8/16/32、GPIO ODR byte subviews 校验了底层访问宽度和相邻字节不受影响。alias 必须完整包含在 canonical 字节区间、无环/链、access 一致。
- 全链仍是单 `cw32-gen` crate，YAML → 写出 JSON → PAC 读回/验证 JSON。普通 PAC 构建无 generator build dependency，不读 YAML/JSON。

## F030 驱动与竞态

F030 76 单测包括 ADC 14、VC 14、ATIM 12、EventState 4、build associations 2、RCC 3、GTIM core 22、队列 5。

- ADC 四槽序列、统一 sample time、保守时钟、READY、实际 EOS IRQ、取消/重入与迟到外部触发边界；保留 ADC BGREN/TSEN/BIAS 与 reserved 字段，构造不 pulse ADC reset。
- VC 实际 0..7 外部输入、响应/迟滞/filter 编码、READY、INTF RW0、真实 VC1/VC2 IRQ；共享门控/复位不扰动兄弟实例。
- VC brake guard 经过独立审查并修复普通借用以外的边界：`mem::forget` guard、嵌套 guard、ATIM reset 后另一 VC 遗留 ATIMBK。安装在同 critical section 检查 MOE、VCE 和两个 VC 的 route；关闭仍在路由的源之前先清功率/自动输出/比较器刹车使能。两个 compile-fail 证明活跃 guard 阻止直接重借 PWM 或 drop comparator。
- ATIM 实际 CR/CHxCR/DTR/FLTR/TRIG、A/B 互补、死区额外 2 tick、UIE/BIE IRQ、故障旗保留、命令位不重放。硬件支持 shadow/UEV；当前严格三相 set_duty 在输出开启时返回 Busy，详见支持文档，不能作为“完整运行中 FOC”证据。
- F030 GTIM 无 UIFREMAP/PSC，使用 OV/CNT/OV 一致性读取、64 位 epoch 与真实 deadline compare；覆盖读取/清旗/写 CCR 边界、旧比较旗、跨回卷、长 deadline、1 tick 无固定延后、10,000 轮对抗性时序模拟及队列不提前唤醒。

## 真实 ARM 向量

L012 检查 32 个真实向量、共享 ADC2_DAC ISR 的两条 Thumb BL 分发；官方 executor 例程保留 ADC1/ADC2_DAC/VC13/VC24/ATIM/CORDIC/GTIM1 七个实际 handler。

F030 executor 例程实际初始化单 ADC、两个外部输入 VC，并在功率输出关闭时启动 ATIM 计数并等待 update。ELF 检查 ADC IRQ12、ATIM IRQ13、VC1 IRQ14、VC2 IRQ15、GTIM1 IRQ16 的向量均指向非 DefaultHandler 的强符号，其他 IRQ 默认；SysTick 保持默认。检查向量表地址 0、48 个槽、栈落在 8 KiB SRAM 内，不生成 CORDIC/EAU/OPA/ADC2_DAC 等不存在符号。

这些结果证明编译、链接和分发，不证明硬件延迟。每次最早未服务的 GTIM overflow 至清旗必须严格小于 65.536 ms，包括关中断、更高优先级 ISR、Flash stall、waker 与 debugger 暂停。没有 STOP 连续计时、动态时钟或 1 微秒调度精度保证。

## 干净源码验收

验证使用新目录中的源码副本，开始时无 Cargo.lock、generated-data、生成 PAC 或 target。独立执行 regenerate、fmt、drift、上述双芯片测试、mutation、4 项 ELF、16 组 feature、4 组负向 selector 和 8 个 ARM 例程。最终 source-only ZIP 再逐项核对归档内容与这份已验收的源码；只允许验证记录等文档更新，不复用维护树的生成 PAC。

ZIP 排除 Cargo.lock、所有生成树、target、SDK ZIP/cache、日志及临时脚本。原厂 header/SVD 与固定证据保留；vendor 的 system_cw32f030.c 只是时钟启动证据，不参与构建。本项目生成工具和运行实现均为 Rust，无 C 编译或 Python 生成链。

## 未验证与限制

没有实板、调试器、示波器、故障注入、DMA 流式采样或电机带载测试。F030 内部分压/BGR 参考仍仅 PAC，不伪造互不干扰的资源 token；三相无扰原子占空比更新尚未建立。HSI 频率为标称值。

`proc-macro-error2 2.0.1` 仍有工具链 future-incompatibility 提示，当前编译通过，不能保证未来 Rust 无提示。Cargo.lock 按要求不分发，固定直接依赖不等于冻结所有未来传递依赖解析。

## v0.9.1 组织与命名回归

- ADC、analog、ATIM、RCC 统一为目录内 `mod.rs` facade，按生成的 `*_l012` / `*_f030` cfg 选择 `l012.rs` / `f030.rs`；crate root 不再以 `#[path]` 重复声明公共模块。
- GPIO 的已有共同实现位于 `gpio/shared.rs`，芯片差异仍由 cfg 明确处理。CORDIC/EAU 只有已实现的 `l012.rs`，不为不存在的 F030 外设制造空后端。GTIM runtime、两套计数算法和共享 queue 全部归入 `time_driver/`。
- 28 个 L012 与 18 个 F030 寄存器模型的文件名、version、引用和生成 metadata 分别使用 `l012` / `f030`；所有 46 个 canonical register JSON 产物名称由新增测试核对。F030 的 IWDT/WWDT 继续复用 `l012`，不复制同一份契约。schema 3、原厂 SDK/manual 版本和合成 fixture 的版本标签不受该命名迁移影响。
- 5 项模块布局回归验证目录结构、根模块唯一声明、对称 cfg，以及编译实际 facade 配合最小后端桩时的重导出和 timer feature/test-only 选择；在两个芯片配置下均运行。
- 16 个搬迁 HAL 实现与 v0.9.0 对比，除 cfg/import 路径和 rustfmt 排版外一致。53 份数据源、48 份生成 JSON、8 份 PAC/metadata/runtime 文件在仅规范化本次版本名后逐字节一致；没有修改寄存器字段、地址、位宽、访问语义或 IRQ。
- README 与 docs 的本地 Markdown 链接检查无缺失。源码包保持单一 `cw32-gen` crate 的 YAML → 落盘 JSON → PAC 流水线，无 Python 前置条件。
