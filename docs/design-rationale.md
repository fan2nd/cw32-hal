> v0.14.0 whole-chain review: [v0.14.0](full-chain-audit-v0.14.0.md), [schema 7](schema-v7.md), [validation](validation-v0.14.0.md). Earlier version records remain historical.

> v0.13.2 consolidates build helpers into build.rs and the shared IRQ event latch into interrupt.rs without changing behavior; see [validation](validation-v0.13.2.md).
> v0.13.1 adds independently selected IP/capability layers and shared workflows; see [HAL cfg layering](hal-cfg-layering.md). Earlier version-specific discussion below remains historical where labeled.

# 设计审查：理解 Embassy 的链路，而不只模仿目录

当前v0.9.1采用封装无关的芯片模型：chip/feature为cw32l012c8和cw32f030c8，schema v3直接定义pins，不存在Package/package_pin或封装交集。以下上游历史设计描述不构成本项目保存封装层的要求。见[schema v3](schema-v3.md)。

> 本文保留早期分层设计的推导，当前芯片模型同步至 v0.9.1。逐项官方源码与本地实现对照见 [embassy-api-alignment.md](embassy-api-alignment.md)。完整寄存器覆盖见 [full-register-coverage.md](full-register-coverage.md)，schema v3 见 [schema-v3.md](schema-v3.md)。源码对照不替代实际验证记录，更不表示已上板。

本项目保留单个 `cw32-gen` crate 和 Rust-only 双阶段链路，并在 v0.7 接入官方 Peri 所有权、sealed GPIO/路由约束及真实中断分发基础设施。它仍不是 embassy-stm32 的完整驱动集合或可发布多芯片 metapac；不能用“已生成关联表”代替“已实现硬件约束”。

## 1. 固定审查基线

2026-10-02 实际获取并阅读以下官方仓库源码；这些是本次固定快照，不声称永远是最新版本：

- Embassy：`b12a6d9efcd2711037abca1b63a661a9ef726444`。
- stm32-data：`e6a417fa643efaf031abc79590ea3e76bc4edbf2`。
- chiptool：`be1bff3e9e1b27b090e69bd9ac753c66fdcce678`，取自上述 stm32-data 的 [Cargo.lock](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/Cargo.lock)，并实际检出阅读。

关键源码链接：

- [data-gen/registers.rs](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-gen/src/registers.rs)、[perimap.rs](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-gen/src/perimap.rs)、[generator.rs](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-gen/src/generator.rs)。
- [数据 schema](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-data-serde/src/lib.rs)、[metapac-gen](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-metapac-gen/src/lib.rs)、[发布 PAC 的 build.rs 模板](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-metapac-gen/res/build.rs)。
- [chiptool IR](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/ir.rs)、[render](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/generate/mod.rs)。
- [HAL build.rs](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs)、[GPIO](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/gpio.rs)、[USART](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/usart/mod.rs)。
- [Peri 所有权封装](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/peripheral.rs)、[bind_interrupts!](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs)。

## 2. 核心抽象：IP 契约、实例、连接关系各自独立

相同外设 IP 跨芯片复用，意味着寄存器名、布局、字段意义和访问接口保持一致。芯片 family 是产品分类，不能充当所有外设的统一版本。一个芯片中 GPIO、USART、RCC 可分别属于不同 IP 版本。

上游引用三元组 `kind/version/block`：kind 是 IP 类别，version 是兼容接口版本，block 允许一个 IP 模型有多个寄存器块。实例地址、RCC 门控、IRQ、DMA 和引脚连接属于器件实例，不属于公共寄存器布局。chiptool IR 则进一步分开 devices、blocks、fieldsets 和 enums；寄存器引用 fieldset，fieldset 可复用，不必在每芯片复制整份代码。

本项目 schema5 显式支持字段数组、寄存器数组（等距或逐项 offset）及可复用嵌套 block 引用，DMA channel 使用真实子块。字段仍内联，支持8/16/32-bit访问；共享 register JSON 与芯片实例分离。尚不具备 chiptool 通用继承和完整变换体系，不能把已实现数组等同于完整生成器兼容性。

## 3. 数据生成阶段是在建立可信契约

厂商 SVD、header、Cube 数据和手册均是证据，不是无需校验的真理。上游 data-gen 汇合多个来源；register YAML 是清理后维护的模型；perimap 将器件实例和厂商 IP 信息映射到 canonical kind/version/block。generator 校验映射到的 block 存在，并应用有范围的 extras、封装 pin 过滤等。其 register validator 甚至显式允许部分 overlap，因此“同上游”不等于把每个检查开到最严，更不等于证明所有硬件事实正确。

本项目使用审核后的 YAML 作为维护源，vendor header/SVD 作为离线对照证据。v0.6.0 将 schema 和两个生成阶段的实现实际归入单个 `cw32-gen` crate：`cw32_gen::schema` 在 `src/schema.rs` 中维护纯模型和共享验证，不单独作为 package；第一阶段 `cw32_gen::data` 做寄存器版本选型、加载、可追踪 correction/alias、排序、引用校验与芯片直接定义的 pins，再将版本化 JSON 写入磁盘；第二阶段 `cw32_gen::pac` 重新读取并校验落盘 JSON，不重新推断硬件事实或重新读取 YAML。统一入口 `cw32_gen::generate` 必须经过这个文件接口，不能将第一阶段内存模型直接传给渲染器。同一 crate 的 schema/data/pac 模块划分表达职责边界，不再以独立 crate 的依赖隔离声称 PAC 生成工具没有 YAML 依赖。

本地 `perimap.yaml` 现支持两种明确模式：默认 `mode: alias` 仅重命名已加载模型；`mode: select` 则在加载前按 chip、instance 与可选 vendor_ip/vendor_version 提示选择 canonical kind/version 对应的寄存器 YAML，原 vendor 身份的文件可以不存在。load_blocks 先验证所选文件身份，再临时保留原身份供 fixes 定位，最后归一化。两 synthetic-chip 测试使用同名 GPIOA 和不同厂商版本提示，真实选出 offset 不同的 gpio/l012、gpio/f030，并输出两份版本化 JSON。缺文件、错身份、歧义规则和冲突原身份会报错，不能靠遍历顺序覆盖。

这是实际版本选型，不再只是 alias；但不等同上游的完整数据集或正则 perimap。当前 selector 精确匹配，normalize.block 仅接受 RegisterBlock；同一芯片内一个 kind 的多版本仍不支持，原 block 键也不能承载相冲突的所选模型。fix 仍以 chip+原 block/version 定位，会影响该芯片所有引用此原模型的实例，不是 GPIOA 专属 layout override。跨芯片不同版本可以独立选择；测试不意味着第二个真实器件已获支持。

不变式：同一发布集合里相同 kind/version 必须指向同一 canonical register 内容；有真实不兼容差异就分版本。多芯片汇总使用这个受校验入口；单芯片 `data::write_json` 也拒绝覆盖同 kind/version 的不同现存内容。

## 4. PAC 生成和 HAL 编译期专化不是同一步

上游 metapac-gen 构造芯片 device IR，整理名称和排序；每种 kind/version 的公共寄存器 IR 展开继承、组织 regs/vals 模块，再由 chiptool render。芯片 pac.rs 引用共享 peripherals/<kind>_<version>.rs；metadata 和 device.x 同样生成。发布 crate 已携带生成源码，发布 PAC 的 build.rs 主要验证唯一芯片 feature，选择 PAC/metadata 路径及运行时链接搜索目录。

HAL build.rs 随用户选择的芯片编译，消费选定 PAC 的 METADATA，生成 cfg、singletons 和绑定实现。数据可在发布阶段冻结，HAL 的器件专化仍在用户编译阶段进行；两个时间点各有职责。CW32 v0.9.1 生成 kind/version、family/chip cfg；各外设 mod.rs 使用已知 kind_l012 或 kind_f030，未实现的驱动版本应拒绝构建，不能因为生成了某个 cfg 就宣称兼容。

CW32 当前已将两个生成阶段移出消费者构建：`cargo run -p xtask -- regenerate` 是统一生成库的薄封装，在临时目录调用 `cw32_gen::generate`，先落盘 JSON，再由 PAC 模块读回，最后替换 generated-data/ 与 cw32-metapac/src/chips/。直接入口是 `cargo run -p cw32-gen -- generate cw32-data all generated-data cw32-metapac/src/chips`；可选 data/pac 子命令用于独立检查各阶段。`xtask regenerate --check` 在临时目录重建并逐字节检查漂移。PAC 没有 `cw32-gen` 或其他 generator build-dependencies；其 build.rs 仅选取预生成 PAC/metadata 并复制到 OUT_DIR，不解析 YAML/JSON。这实现了发布前生成与消费者编译的分离。

仍有差异：公共寄存器 JSON 已复用，但 Rust PAC 当前按芯片整体生成，尚未像上游把跨芯片公共 Rust 模块单独打包。当前支持L012C8与F030C8；真实兼容IP复用同一JSON，不兼容布局分版本，仍需在后续优化跨芯片Rust模块去重，不能为节省代码而混同硬件契约。publish=false 的 workspace 不代表已执行 crates.io 打包发布验收。

## 5. HAL 类型系统如何兑现 metadata

上游的价值不止是 Rust 常量：

- `cfg(kind)` 与 `cfg(kind_version)` 选择能理解该寄存器契约的驱动实现，避免以芯片型号为轴复制驱动。
- `Instance`/私有 `SealedInstance` 绑定有效外设、状态和 IRQ。下游不能为虚构设备任意实现内部契约。
- `SealedPin`、`TxPin<T>`/`RxPin<T>` 等把 pin 与具体外设 T 的可用信号关系编码到构造函数参数；AFIO remap 也需与选择一致。只有 route 表但构造器接收任意 pin，就仍未约束错误接线。
- 当前上游使用 `Peri<'a,T>`；历史资料常称 PeripheralRef。Peri 携带独占 lifetime，reborrow 在子借用期间阻止父资源同时使用。内部 marker 可以 Copy，不代表可复制独占所有权。复制所有权需要 unsafe clone_unchecked。
- DMA trait 将方向、外设、channel/request 与 remap 关联；构造器要求匹配的 DMA 能力，不能只检查 channel 数字存在。
- RCC 生成 enable/reset/kernel-clock 关系，并对共享 clock-enable 字段安排引用计数。一个实例释放时不能关闭另一使用者仍需要的时钟。寄存器合法不代表资源生命周期正确。
- `bind_interrupts!` 同时生成实际调用 handler 的入口和 unsafe Binding 证明。共享 vector 调用多个 handler；单独声明 marker 或名字相同并不能证明 ISR 已安装。

CW32 v0.7 直接使用官方 `embassy-hal-internal =0.5.0`：生成身份实现 `PeripheralType`，`Peripherals` 字段为 `Peri<'static, T>`，安全入口仍全局 take-once。现有 GPIO、ATIM、ADC、模拟和数学驱动持有 `Peri<'d, T>`，允许 move 或受 lifetime 约束的 reborrow。sealed `gpio::Pin` 与 `AnyPin` 分别表示具体能力和擦除身份；可复制身份不等于可复制独占许可。GPIO port/SYSCTRL 及 `ownership_parent` 子视图不获得会与已有资源冲突的独占 token。

现有 ADC、ATIM、模拟构造器使用 metadata 生成的 sealed instance/signal traits 限制已审核路线；并未扩展为全芯片 Pin/DMA 类型系统。公开关联表受 `metadata` feature 控制，仍只是数据；AFIO、quirk 文字和 IRQ 关联不会自行执行算法。原始 PAC 仅在 `unstable-pac` 下从 HAL 公开，unsafe steal/raw MMIO/clone_unchecked 仍可绕过安全所有权。

IRQ 基础设施也已从仅有 marker 推进到实际分发。PAC 生成 `rt.rs`、`device.x`，按物理号布置向量；稀疏编号保留零槽而不紧缩。HAL 使用官方 `interrupt_mod!` 和类型级 Handler/Binding，`bind_interrupts!` 在 `rt` 下同时生成真实 ISR 调用与证明，共享向量可顺序调用多个 handler。当前 ADC/VC/ATIM/CORDIC 和 GPIO interrupt input 已实现真实 pending/clear/wake、NVIC 使能及取消；DMA 生命周期仍未实现，不能仅凭基础设施声称 DMA 驱动完成。

`init(Config) -> Peripherals` 与 `Config.rcc` 遵循官方入口形状，另保留可报告错误的 `try_init`。本地仅允许经过校验的复位 HSI/24，`rcc::clocks()` 返回保存的标称频率且在初始化前 panic；官方 getter 要求 RCC Peri，本地没有发出 SYSCTRL token，不能宣称 getter 签名完全相同。共享门控采用不在 drop 关闭的保守策略，共享 reset 不由单个实例断言；仍没有上游完整 clock 引用计数和低功耗管理。

HAL 默认只有 `rt`，芯片、`memory-x`、公开 `metadata` 和 `unstable-pac` 均显式选择；`defmt` 接入实际的上游依赖和格式化实现。没有用空的 EXTI/DMA/time feature 冒充已实现能力。非时间驱动配置下，应用仍须提供适合单核 Cortex-M0+ 的 critical-section backend。

时间驱动按专用 GTIM1 设计：`time-driver-any` 是 `time-driver-gtim1` 的真实 alias，HAL 在 RCC 后自动初始化，配置中提供默认 P0 的 `time_interrupt_priority`。build.rs 从交给应用的 Peripherals 字段中移除 GTIM1，保留给时间驱动，因此不与应用或 FOC ATIM 重复取得硬件；不开启驱动时 GTIM1 仍归应用。标称 1 MHz 的 16-bit counter 每 65.536 ms wrap，从 overflow 到 ISR 清除 UIF 必须严格短于该周期，包含中断/critical-section 阻塞。compare 写入实际 deadline，写后重读并在必要时 pend IRQ；短 sleep 仍受 4 MHz CPU、MMIO、IRQ 和 executor 延迟影响。1 μs timestamp 分辨率不构成 HSI 精度、1 μs 唤醒准确度或低功耗连续性证明。实际算法及链接验证以 [API 对照](embassy-api-alignment.md) 和最终验证记录为准，SysTick 已不作为此版本的时间驱动方案。

现有 blocking GPIO/RCC 的关键要求是：手写算法真正引用生成数据；只有chip.pins中审核过的引脚能力生成singleton，具体封装及PCB引出由板级负责；共享 GPIO 配置 RMW 受 critical section 保护；键值、动作寄存器和保留位语义不能由普通 RW 标签代替。RCC/GPIO 使用生成的 typed/indexed PAC setter 及 GPIO 能力 mask，不保留地址算术或旧字段描述器 facade。数据表不能替代这些算法审查。

## 6. 能编译却违背设计的反例

1. 为某新芯片 GPIOA 的特殊寄存器位修改公共 gpio/l012，却保持其余芯片、GPIOB 仍引用 l012：所有 Rust 都可能编译，其他实例的硬件语义却已错误。必须建立新兼容版本或限定且被驱动实际处理的实例差异；quirk 描述字符串本身没有修复效果。
2. 为每个 IRQ 生成空的 `Binding<I,H>` impl，不输出对应 ISR 调用：驱动构造器可通过，future 永远等不到唤醒，甚至错误地相信中断初始化已经完成。当前 CW32 不提供这种伪证明。
3. 把GPIO implemented_mask的所有位或任意复用关系自动认定为可用接口：寄存器位存在不代表该引脚身份及信号路由已核实。应由chip.pins与pin_routes明确声明；HAL不选择封装，具体封装引出、调试占用和板级接线由调用方另行验证。
4. 将两个外设共用的 RCC 位分别视为独立资源：两个驱动都能构造，drop 一个却停止另一个。需要共享门控策略而不是更多常量。

## 7. 本轮验收及不可外推的结论

可接受本轮链路的理由：纯 schema 模块/落盘 JSON 两阶段接口、真实从字段定义生成 API、单向 PAC metadata→HAL 依赖、共享 IP 冲突检查、临时 YAML→JSON→PAC→HAL 的实际变异编译、固定厂商证据校验，以及有边界的所有权/IRQ API。

变异测试在隔离 workspace 更改 pin、offset、实例地址和 IRQ 身份，显式重新生成预生成产物，再 cargo check HAL 并检查生成物；它证明传播关系，不等于变异固件已链接或硬件行为正确。目标例程链接、ELF 物理向量与共享 IRQ 分发检查、compile-fail 所有权测试是另外的证据；实际执行结果见 [validation.md](validation.md)。尚未上板，也未证明全外设支持、中断硬件时序、DMA 生命周期或完整类型级路由。

后续每增加一个安全驱动，应先明确它消耗哪些 token、pin route、clock/reset、DMA 和 IRQ 证明，再补数据及失败编译测试。不要先生成一个宽松 API，再依赖用户记住硬件限制。
