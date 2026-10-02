当前v0.9.1模型更新：chip feature使用cw32l012c8或cw32f030c8（恰好一个），无封装/温度后缀。schema v3的chip.pins直接定义引脚能力，不再保存Package或封装交集；板级负责实际引出。F030采用独立IP版本与HAL后端，参见[cw32f030-support.md](cw32f030-support.md)。以下L012专属外设/API描述不能外推到F030。

# v0.8 Embassy HAL API 逐项对照

本文对照的是官方 `embassy-rs/embassy` 固定提交 **`b12a6d9efcd2711037abca1b63a661a9ef726444`**，不是随时间变化的 `main`。本地实现是实验性 CW32L012C8 v0.8.0。对齐表示采用已经落地的接口约定，不表示芯片寄存器兼容、全部驱动完成或获得 Embassy 官方支持。没有进行上板、烧录或电机测试。

## 1. 官方基线与本地实现矩阵

“已对齐”仅针对该行列出的契约；“部分对齐”保留明确差异；“未实现”不能由寄存器或 metadata 覆盖率代替。

| 项目 | 固定版本官方实现 | CW32 v0.8 实现及位置 | 结论与剩余差异 |
| --- | --- | --- | --- |
| 公共所有权类型 | [`lib.rs:391–397`][up-reexports] 重导出 `Peri`/`PeripheralType`；[`peripheral.rs:17–75`][up-peri] 实现独占 lifetime、`reborrow`、类型擦除 | [`Cargo.toml`](../embassy-cw32/Cargo.toml) 精确依赖官方 `embassy-hal-internal =0.5.0`；[`lib.rs`](../embassy-cw32/src/lib.rs) 直接重导出 | **已对齐**。使用上游 crate，不维护同名自制 Peri。registry 版本固定不等于把整个 Embassy 仓库锁为该 git 提交 |
| singleton 与资源集合 | [`macros.rs:1–84`][up-singletons] 生成身份和 `Peripherals` 中的 `Peri<'static, T>`，安全获取仅一次 | [`build.rs`](../embassy-cw32/build.rs) 使用上游 `peripherals_definition!` 生成身份；资源字段为 `Peri<'static, peripherals::T>`；[`lib.rs`](../embassy-cw32/src/lib.rs) 用 critical section 管理 take-once | **已对齐核心契约**。为保留可报告的时钟失败，本地自行生成资源集合并在成功时交付；未照搬全部 `Peripherals::take/steal` API |
| 共享/重叠硬件所有权 | [`build.rs:195–245`][up-singleton-filter] 按外设类别筛选 singleton，GPIO 等特殊处理 | 本地 [`build.rs`](../embassy-cw32/build.rs) 不发布 GPIO port、SYSCTRL 或 `ownership_parent` 子视图的独占 token | **按 CW32 特化**。DMA parent 与 alias channel 不会同时获得安全所有权；尚无安全 DMA 分区 API |
| 顶层初始化 | [`lib.rs:403–408`][up-config] 的非穷尽 `Config { rcc, … }`，[`lib.rs:594–602`][up-init] 的 `init(Config) -> Peripherals` | [`lib.rs`](../embassy-cw32/src/lib.rs) 同形状的 `init`、`Config.rcc`，并提供 `try_init -> Result` | **入口已对齐，能力较窄**。重复初始化或不支持的启动时钟使 `init` panic；需报告错误时用 `try_init` |
| RCC 配置 | 官方 RCC 实现按芯片选择时钟树，初始化在 [`lib.rs:1046–1054`][up-time-init] 调用 | [`rcc.rs`](../embassy-cw32/src/rcc/l012.rs) 默认复位 HSI/24；v0.10.0可显式选择96MHz HSI及APB/2，先配置Flash等待且VDD>=1.8V；可设 `hsi_stabilization_limit` | **部分对齐**。默认标称4MHz，可选固定96MHz配置，不支持 PLL、HSE 切换、任意分频或运行时重配；poll limit 不是校准后的时间超时 |
| 时钟查询 | [`rcc/mod.rs:153–159`][up-clocks] 的 `clocks(&Peri<RCC>) -> &Clocks` 通过 RCC token 约束查询 | [`rcc.rs`](../embassy-cw32/src/rcc/l012.rs) 的 `clocks() -> Clocks` 返回初始化后保存的副本；`Peripherals` 无 clocks 字段 | **有意不同**。SYSCTRL 不作为独立 token 发出；getter 初始化前 panic。频率字段为 Hz 的 `u32`，不是完整 STM32 clock/Hertz API |
| GPIO pin 身份 | [`gpio.rs:763–909`][up-pin] 的 sealed `Pin`、`AnyPin` 和 singleton 转换 | [`gpio.rs`](../embassy-cw32/src/gpio/shared.rs) 的 sealed `Pin: PeripheralType + Into<AnyPin>`；具体 pin impl 由 metadata 生成 | **核心所有权已对齐**。可复制的身份值不等于可复制的独占 `Peri`；未实现上游所有 pin 方法或完整 GPIO 能力 |
| GPIO 构造器 | [`gpio.rs:319–329`][up-input] 的 `Input<'d>`/`Peri`；[`gpio.rs:395–406`][up-output] 的 `Output<'d>` 另有 `Speed` 参数 | [`gpio.rs`](../embassy-cw32/src/gpio/shared.rs) 的 `Input::new(Peri<'d, impl Pin>, Pull)`、`Output::new(Peri<'d, impl Pin>, Level)`，内部持有 `Peri<'d, AnyPin>` | **lifetime/Peri 已对齐，签名非逐字兼容**。仅阻塞输入、推挽输出；没有虚构 Speed、Flex、通用开漏或 async EXTI 接口 |
| 外设与引脚构造约束 | 例如 [`usart/mod.rs:1623–1645`][up-usart] 同时约束 `Instance`、TX/RX pin、DMA 与 Binding | [`adc.rs`](../embassy-cw32/src/adc/l012.rs)、[`atim.rs`](../embassy-cw32/src/atim/l012.rs)、[`analog.rs`](../embassy-cw32/src/analog/l012.rs) 消耗 `Peri<'d, T>`，使用 sealed instance/signal traits；CORDIC、EAU 同样持有 Peri | **已有驱动已接入所有权**。ADC/ATIM/模拟路线限已审查子集；`AnyPin` 不能替代需要具体信号能力的 pin。没有 UART/SPI/I2C/DMA 驱动可宣称对应约束 |
| metadata 编译期专化 | [`build.rs:55–90`][up-cfg] 根据芯片和寄存器 kind/version 生成 cfg，包含多段 version 前缀 | [`build.rs`](../embassy-cw32/build.rs) 根据 PAC METADATA 生成 kind、完整 kind_version、family/chip cfg；[`lib.rs`](../embassy-cw32/src/lib.rs) 声明外设入口，各外设 mod.rs 实际使用 kind_l012/kind_f030 cfg，未支持的驱动版本拒绝构建 | **部分对齐**。目前两个真实芯片，差异 IP 以 l012/f030 命名；实证兼容的 IWDT/WWDT 共用 l012 模型。尚未支持上游全部 version 前缀/跨芯片特例 |
| 外设模块组织 | [`lib.rs:71–96`][up-modules-a]、[`lib.rs:188–286`][up-modules-b] 按实际硬件 cfg 导出模块 | 本地保留 `gpio/rcc/adc/atim/analog/cordic/eau/interrupt`，内部时间驱动受 feature 控制 | **按 CW32 保留差异**。不把 ATIM 强行改名成 STM32 timer，不虚构 OPAMP3；模拟组合资源集中在 `analog`，EAU 保留独立模块 |
| 默认与芯片 feature | [`Cargo.toml:237–240`][up-features] 默认仅 `rt`；[`build.rs:55–63`][up-cfg] 验证唯一芯片 | HAL [`Cargo.toml`](../embassy-cw32/Cargo.toml) 默认仅 `rt`，明确选择 `cw32l012c8`；PAC 默认为空；无芯片时拒绝构建 | **HAL 约定已对齐**。只有一个真实芯片；直接使用 PAC 也要选择芯片，不能依赖旧默认值 |
| `memory-x` | [`Cargo.toml:346–347`][up-public-features] 单独 feature；[`build.rs:3217–3220`][up-memory] 按条件生成 | 本地 [`build.rs`](../embassy-cw32/build.rs) 仅开启 feature 时写出 memory.x | **已对齐 opt-in 行为**。64 KiB Flash/8 KiB RAM 为直接复位布局；bootloader/自定义布局须由应用负责 |
| PAC 重导出 | [`lib.rs:394–397`][up-reexports] 以 `unstable-pac` 控制公开可见性 | 本地 [`lib.rs`](../embassy-cw32/src/lib.rs) 使用同一可见性模式 | **已对齐**。未启用时 PAC 仍供 HAL 内部使用；开启不使原始 MMIO 自动安全 |
| metadata 依赖与公开表 | [`Cargo.toml:219–224`][up-metapac-deps] 将 runtime 与 metadata build dependency 分开 | 本地 HAL/PAC 的 metadata 模块由 feature 控制，HAL build dependency 始终启用 PAC metadata；公开关联表是 CW32 附加接口 | **职责已分开，API 为本地扩展**。构建期需要 metadata 不要求固件公开同一模块；公开表不能安装 ISR、DMA 或 AFIO |
| `defmt` | [`Cargo.toml:244–256`][up-defmt] 接入实际依赖；[`peripheral.rs:99–103`][up-peri-defmt] 为满足 `Format` 的 T 格式化 Peri | 本地 feature 启用 `defmt =1.0.1` 与 `embassy-hal-internal/defmt`，上游宏为生成身份派生 Format | **功能真实存在，覆盖较窄**。不承诺所有 CW32 配置、错误或 `AnyPin` 均实现 Format；没有复制上游全部日志生态 |
| 类型级中断与 NVIC | [`build.rs:481–495`][up-irqs] 调用官方 `interrupt_mod!`；[`interrupt.rs:11–140`][up-interrupt-types] 定义 IRQ/Handler/Binding | 本地 [`build.rs`](../embassy-cw32/build.rs) 以已审核 IRQ 清单调用同一宏，[`interrupt.rs`](../embassy-cw32/src/interrupt.rs) 导出类型级中断；优先级使用 Cortex-M0+ 的 2 bits | **采用上游契约**。IRQ 身份来自数据，不把每条 peripheral-signal 关联当成新中断 |
| `bind_interrupts!` 与共享向量 | [`lib.rs:311–389`][up-bind] 生成 ISR、按顺序调用多个 handler，并生成对应 unsafe Binding | 本地 [`interrupt.rs`](../embassy-cw32/src/interrupt.rs) 实现同一分发模式，拒绝未知 IRQ；宏要求 `rt` | **分发与证明已实现**。不同于官方宏的导出条件，本地显式要求 runtime。各 handler 必须自行判断并处理 pending；宏不自动开启 IRQ，也不自动实现异步唤醒 |
| 物理向量连接 | 官方 HAL 的 `rt` 转发至 PAC（[`Cargo.toml:237–240`][up-features]），HAL 宏提供实际 IRQ 符号（[`lib.rs:362–383`][up-bind-body]） | [`cw32-gen/src/pac.rs`](../cw32-gen/src/pac.rs) 生成 `rt.rs`/`device.x`，[`cw32-metapac/build.rs`](../cw32-metapac/build.rs) 选择产物；向量按 IRQ 数字放置，空号为保留零槽 | **本地 runtime 已接通该契约**。默认别名指向 DefaultHandler；关联表不自动安装外设驱动。自定义启动/向量表需保持符号和分发关系 |
| Embassy 时间驱动 | [`Cargo.toml:362–381`][up-time-features] 的 `time`/`time-driver-any` 有真实依赖与 timer 选择；[`time_driver/tim.rs:11–32`][up-time-tim] 使用专用 timer，顶层 init 在 RCC 后启动（[`lib.rs:1052–1054`][up-time-init]） | 本地 `time-driver-any` 实际启用 `time-driver-gtim1`；[`lib.rs`](../embassy-cw32/src/lib.rs) 在 RCC 后自动初始化内部 [`time_driver.rs`](../embassy-cw32/src/time_driver/mod.rs)；build.rs 从交给应用的 Peripherals 字段中移除 GTIM1，保留给时间驱动 | **启动与资源保留结构对齐，timer 实现为 CW32 专用**。16-bit GTIM1 提供标称 1 MHz tick，不用 SysTick，也不占 ATIM；无 STM32 timer 选型集合、STOP 支持或空的 `time` feature。算法与链接验证以最终记录为准 |
| ADC 阻塞/异步 owner | 官方 [`adc/mod.rs:779–810`][up-adc-mode] 使用 `Adc<'d, T, M>`，`new_blocking` 与带 Binding 的 `new` 区分模式 | 本地 [`adc.rs`](../embassy-cw32/src/adc/l012.rs) 保留 `Adc<'d, I, N>`，以 `into_async(binding)` 转成 `AsyncAdc<'d, I, N>`；EOS IRQ 服务完整序列，提供取消 guard 和 `into_blocking` | **IRQ/Binding/借用契约对齐，公开类型不逐字兼容**。采用显式 wrapper，而非复制模式参数名称；ATIM 触发返回完整单次结果，不是持续 DMA，主从 `sample_pair` 仍阻塞 |
| 其他真实 IRQ future | 官方 HAL 按实际外设提供异步操作，不以函数名字代替中断实现 | 本地 [`analog.rs`](../embassy-cw32/src/analog/l012.rs) 的 `AsyncComparator` 等待边沿/电平；[`atim.rs`](../embassy-cw32/src/atim/l012.rs) 的 `AsyncThreePhasePwm` 等待 update/break；[`cordic.rs`](../embassy-cw32/src/cordic/l012.rs) 的 `AsyncCordic` 等待 EOC | **按 CW32 硬件实现**。无事件返回 Pending，由真实 handler 发布/wake；取消仅撤销本次操作。EAU 和 OPA 无经审查的完成 IRQ，明确保留阻塞，不伪造 async 能力 |
| executor 整合 | Embassy executor 与 HAL 分层 | 根目录板级 06-application 使用真实 Embassy executor | 电机 IRQ 所有权与 UI 任务分离，尚未上板验证 |
| DMA、EXTI、持续采样 | 官方 [`lib.rs:71–76`][up-modules-a]、[`lib.rs:189–190`][up-modules-b] 与 USART 构造器有相应实现 | CW32 存在寄存器与部分关联数据，没有 DMA/EXTI feature 或空模块，也没有 async UART/SPI/I2C | **仍未实现**。已有四类 IRQ future 不等于全 HAL 异步化、无损事件队列、连续 DMA 采样或实时 FOC 闭环 |
| 共享 RCC 生命周期 | [`rcc/mod.rs:302–336`][up-rcc-enable]、[`rcc/mod.rs:381–405`][up-rcc-disable] 对共享资源计数 | 本地 [`build.rs`](../embassy-cw32/build.rs)/[`rcc.rs`](../embassy-cw32/src/rcc/l012.rs) 不由单实例构造器重置共享 reset 位，也不在 drop 关闭共享 gate | **保守替代，未达到完整能力**。避免干扰兄弟实例，但没有上游完整引用计数、kernel clock 和低功耗管理 |

## 2. v0.6 使用者迁移要点

1. HAL 编译须显式选择一个芯片；生成器不需要芯片 feature。
2. 正常启动改为 `let mut p = embassy_cw32::init(Default::default());`，不要再对 `init` 返回值调用 `unwrap()`。需要处理初始化错误时改用 `try_init`。clock 错误可能出现在 trim 已写入之后，错误返回不承诺回滚硬件。
3. 原 `p.clocks` 改为 `embassy_cw32::rcc::clocks()`。若需要修改稳定轮询次数，先构造默认 Config，再设置 `config.rcc.hsi_stabilization_limit`；不要把它当作任意时钟源配置。
4. 所有权长期移交仍可写 `Output::new(p.PA0, Level::Low)`。短期借用使用 `Output::new(p.PA0.reborrow(), Level::Low)`，持有者在 driver drop 前不能再次使用父 token。数据类型 `peripherals::PA0` 的身份与 `Peri<'d, peripherals::PA0>` 的独占许可不同。
5. `gpio::Pin` 现在是 trait。需要保存不同 GPIO 时使用 `Peri<'d, gpio::AnyPin>`，可通过 `p.PA0.into()` 擦除身份；外设专用信号构造器仍需满足 sealed route trait。
6. 应用确需访问原始 PAC 时开启 `unstable-pac`；确需公开关联表时开启 `metadata`；使用本地自动内存布局时开启 `memory-x`。这些 feature 不互相代替。

### v0.7 → v0.8 的异步迁移

原构造器和阻塞操作保留；需要异步时显式绑定真实 handler，再调用 `into_async(binding)`。本地没有把现有类型改写成上游 `Adc<Async>` 形式，也没有让原 `Adc::new` 自动开启中断。转换保留外设、pin、DAC/RefDivider 等全部资源 lifetime，future 独占借用 `&mut self`。ADC/CORDIC 可 `into_blocking`；VC/ATIM 当前没有该逆转换。

ADC2_DAC 和 VC13/VC24 的每个活跃中断源都需包含在同一物理向量分发中；handler 只服务本实例，取消不能禁用共享 NVIC。取消 ADC 停止转换/trigger，取消 VC 撤销本源订阅，取消 CORDIC 复位其专用 accelerator；取消 ATIM 等待不关闭既有输出、counter 或硬件 break，不能当作急停。ATIM break 仍要求显式确认与重新启用。

新方法等待的都是实际 IRQ 事件，不使用忙轮询等待结果；但创建/模拟稳定延时仍同步。方法没有内置超时，外部 timeout/select 必须实际 drop future 才取消。详见 [异步 API、签名与限制](async-api.md)，尤其是单次 ADC 与边沿合并的语义；这些不能被描述成持续采样或控制闭环。

### 专用 GTIM1 时间驱动

启用 `time-driver-any` 或 `time-driver-gtim1` 后，HAL 初始化自动配置时间驱动；`Config.time_interrupt_priority` 默认 `Priority::P0`。不再由应用转交 `core.SYST` 或单独调用时间驱动初始化。以下片段说明入口，不是可直接部署的完整固件。

```rust,ignore
let mut config = embassy_cw32::Config::default();
config.time_interrupt_priority = embassy_cw32::interrupt::Priority::P0;
let p = embassy_cw32::init(config);
// GTIM1 已由时间驱动保留；p 不再含可安全交付给应用的 GTIM1 字段。
// 在应用 executor/async 上下文中使用 embassy_time::Timer。
```

当前复位 PCLK 为标称 4 MHz，GTIM1 的 PSC=3、ARR=65535，使硬件计数为标称 1 MHz，每 65.536 ms 溢出。驱动必须正确扩展计数和安排 compare alarm；从 overflow 到 ISR 清除 UIF 的时间必须严格小于一个完整周期，包含 IRQ 屏蔽、critical section 和优先级阻塞，不能靠一个 pending flag 恢复多个丢失的 wrap。compare 写入实际 deadline，写后重读 CNT 并在必要时 pend IRQ，避免编程期间越过期限后错过唤醒；不使用未经硬件测量支持的固定提前量。短 sleep 仍受 4 MHz CPU、MMIO、IRQ 和 executor 实际延迟影响。**1 μs timestamp 分辨率不等于 HSI 的实测准确度，也不保证 1 μs 唤醒准确度。** 未支持 STOP、动态调频或低功耗时间连续性。

为时间驱动移除 GTIM1 应用字段不影响用于 FOC 的 ATIM；不开启时间驱动时 GTIM1 仍作为普通 Peri 字段提供。时间驱动 feature 带入本芯片的 Cortex-M single-core critical-section backend；单独的 `cw32l012c8` + `rt` 不提供 backend，应用应通过 `cortex-m = { version = "=0.7.7", features = ["critical-section-single-core"] }` 或另一个正确实现提供它。

## 3. 数据链路保持不变

接口整理不拆回多个 generator crate。[`cw32-gen`](../cw32-gen/src/lib.rs) 仍是单个 host-only crate，内部 schema/data/pac 分工；Rust CLI/xtask 强制执行 **YAML → 写入 normalized JSON → 重新读取 JSON → PAC/metadata/runtime**。PAC 普通构建只选择本地预生成源码；HAL build.rs 只消费 PAC metadata，不直接解析 YAML。

`generated-data/`、`cw32-metapac/src/chips/`、`Cargo.lock` 和 `target/` 仍被 ignore。干净源码必须先 `cargo run -p xtask -- regenerate`，再运行 `--check` 或消费者构建。没有引入 Python/PyYAML 前置条件。共享寄存器 JSON 已复用，但跨芯片公共 Rust PAC 模块尚未独立打包；只有 CW32L012C8 一个真实芯片受到支持。

## 4. 审查与验证边界

本矩阵是源码/API 对照，不是单凭文档宣告测试通过。实际运行的命令、结果与未跑项目见 [`validation.md`](validation.md)。建议检查以下不同层级，不能互相替代：

- 主机单元测试及 compile-fail 测试：Peri 重借用、sealed 路由、类型级中断和 mock 软件时序。
- feature 正向/负向编译：显式芯片、无 `rt`、`memory-x`、`metadata`、`unstable-pac`、`defmt` 的真实作用。
- Cortex-M0+ 例程链接和 ELF 向量检查：证明 IRQ 符号、vector slot、默认 handler 及共享分发连接。
- 隔离 YAML→JSON→PAC→HAL 变异：证明地址、引脚和 IRQ 数据实际传播，稀疏 IRQ 不能被紧缩。
- 上板测量：时钟精度、IRQ 延迟、fault 极性、模拟精度和功率级行为。本版**没有完成**这一层。

保持 ATIM 构造时功率输出禁能、显式启用和故障恢复边界。原理图、外部上下拉、gate-driver 极性与保护路径仍必须由板级审查确认；Peri、Binding 和寄存器类型无法证明这些事实。更多限制见 [`foc-hal.md`](foc-hal.md) 和 [`design-rationale.md`](design-rationale.md)。

[up-peri]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/peripheral.rs#L17-L75
[up-peri-defmt]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/peripheral.rs#L99-L103
[up-singletons]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/macros.rs#L1-L84
[up-singleton-filter]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs#L195-L245
[up-reexports]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L391-L397
[up-config]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L403-L408
[up-init]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L594-L602
[up-clocks]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rcc/mod.rs#L153-L159
[up-pin]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/gpio.rs#L763-L909
[up-input]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/gpio.rs#L319-L329
[up-output]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/gpio.rs#L395-L406
[up-usart]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/usart/mod.rs#L1623-L1645
[up-cfg]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs#L55-L90
[up-modules-a]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L71-L96
[up-modules-b]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L188-L286
[up-features]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/Cargo.toml#L237-L240
[up-public-features]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/Cargo.toml#L346-L356
[up-memory]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs#L3217-L3220
[up-metapac-deps]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/Cargo.toml#L219-L224
[up-defmt]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/Cargo.toml#L244-L256
[up-irqs]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs#L481-L495
[up-interrupt-types]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/interrupt.rs#L11-L140
[up-bind]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L311-L389
[up-bind-body]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L362-L383
[up-time-features]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/Cargo.toml#L362-L381
[up-time-tim]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/time_driver/tim.rs#L11-L32
[up-time-init]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs#L1046-L1054
[up-rcc-enable]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rcc/mod.rs#L302-L336
[up-rcc-disable]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rcc/mod.rs#L381-L405
[up-adc-mode]: https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/adc/mod.rs#L779-L810
