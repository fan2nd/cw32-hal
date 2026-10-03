# embassy-cw32：YAML → normalized JSON → PAC → HAL

CW32L012C8 与 CW32F030C8，实验性 **v0.20.0**，尚未上板验证。数据、统一生成器 `cw32-gen`、PAC 与 HAL 位于同一个 Cargo workspace。schema 与两个生成阶段的代码保留在一个 crate 中，仍强制执行 YAML → 落盘的 normalized JSON → PAC，不能用内存模型跳过 JSON 接口。当前提供 typed PAC read/write/modify、显式寄存器/子块数组、有来源的 reset defaults、GPIO Flex/Input/Output/OpenDrain 与真实 IRQ Wait、Adc/Comp 模式 owner 和按调用借入 ADC 通道、拥有真实路由的通用 Timer/SimplePwm、两芯片 DMA、UART/SPI/I2C、CRC/IWDT/WWDT、内部 ADC 源、RTC 和保留数据分区 Flash，以及独立的 L012 电机操作 API；完整寄存器数据覆盖不代表全部外设驱动或 FOC 闭环均已实现。

芯片名与 feature 不带 T7、U6 等封装及温度后缀。`cw32-data` 不维护 packages 层；芯片直接定义 GPIO 能力和信号路由，实际封装是否引出、物理脚号及板级接线由板级设计负责。Flash/RAM 等芯片差异仍由 chip 数据描述。

v0.14.0 对 data、生成器/PAC 与全部现有 HAL 模块重新对照实际固定版本上游，修复共享 PAC 输出、连接元数据、ADC 序列遗留状态、软件更新触发及时间驱动回调/初始化契约。逐模块结论、真实硬件差异及未实现范围见 [完整审查](docs/full-chain-audit-v0.14.0.md)。

v0.15.0 增加 [DMA](docs/dma.md) 和单独的 [motor/ 电机 API](docs/motor-api.md)。DMA 通道、真实共享 IRQ 与硬件请求由 schema8 metadata 生成。安全 DMA 传输持有静态缓冲区，仅在正常完成后交还；CW 手册没有证明中止后总线写入已排空，取消、错误和超时会隔离缓冲区并使该通道失效。普通借入缓冲区与任意外设地址使用明确 unsafe 契约。motor/ 的 unsafe acquire 由调用者负责资源排他与中断串行化，板级控制算法与参数仍在例程中。检查范围见 [v0.15.0 验证](docs/validation-v0.15.0.md)。

v0.16.0 将 motor 和板级 ISR 也接入实际 Embassy typelevel 中断绑定：ADC/基本定时器采用类型身份，开中断须给出正确 Handler 的 Binding；06 main 集中声明真实向量，原同步采样/换相仍在 ISR。见 [中断绑定设计与迁移](docs/typelevel-interrupts.md) 和 [v0.16.0 验证](docs/validation-v0.16.0.md)。

v0.17.0 补齐首批资源组合：OPA 输出可受借用保护地交给 ADC；DAC 可拆分为两个独立通道，并在 VC/OPA 保留依赖时更新单通道电压码；ADC 可通过类型化 DMA 请求完成一次有限采样，保留静态输入/缓冲区与取消隔离契约；`copy_mut` 在正常完成后归还可写源。L012 支持 1–8 槽 DMA 序列，F030 首先开放已验证的单次单通道，未证明的多槽 DMA 明确拒绝。见 [资源组合与后续阶段](docs/resource-composition-v0.17.0.md)、[schema9](docs/schema-v9.md) 和 [本版验证](docs/validation-v0.17.0.md)。

v0.18.0 增加 [RCC 时钟资源](docs/rcc-resources.md) 与全部已记载 HSI/AHB/APB 分频组合，区分外设总线和内核输入频率；共享门控由实际 owner 引用计数，隔离中的 DMA/ADC 保留时钟。`SimplePwm::split()` 提供可独立持有的通道。PAC 数据新增语义枚举及重复字段索引，支持不规则 bit offsets；[46 IP 审查](docs/pac-fields-v0.18.0.md) 记录 50 个枚举字段与 156 组字段数组，保留 reserved 值和原寄存器契约。见 [本阶段边界](docs/clock-pwm-pac-v0.18.0.md)、[schema10](docs/schema-v10.md) 和 [验证](docs/validation-v0.18.0.md)。

v0.19.0 增加两芯片 [UART](docs/uart.md)、[SPI](docs/spi.md)、[I2C](docs/i2c.md) 阻塞与真实 IRQ 异步控制器驱动，以及 [CRC 和独立看门狗](docs/crc-watchdog.md)。新增249条已审核总线引脚路由，类型约束和实际共享中断来自 metadata。06 UI 改用拥有 UART1 和引脚的 HAL，保留实际96MHz PCLK 下原 BRRI52/BRRF1、每次唤醒最多一个字节及原电机中断结构。该版安全总线 DMA 等边界和后续阶段见 [本阶段范围](docs/buses-v0.19.0.md)。

v0.20.0 增加 [内部 ADC 源](docs/adc-internal.md)、[窗口看门狗](docs/window-watchdog.md)、[RTC](docs/rtc.md) 和 [Flash 数据分区](docs/flash.md)。RTC 使用真实 LSI/LSE 启动与时钟凭据，LSE 永久占用 PC14/PC15；RCC 保留已运行 LSI 的 trim。Flash 构造是明确的 unsafe 分区/并发边界，普通中断和 Flash 取指可能在写擦期间停顿。EAU/CORDIC、独立/窗口看门狗和 DAC 也改为由 metadata 生成的 sealed Instance 关联，寄存器、时钟、中断/源通道随实际实例传递；构造仍由传入的 Peri 自动推导类型。八项差距的完成情况和后续范围见 [第四阶段](docs/analog-rtc-flash-v0.20.0.md)。

## 先看设计与边界

[设计理由与固定版本上游审查](docs/design-rationale.md) 解释每层为何存在；[Embassy API 逐项对照](docs/embassy-api-alignment.md) 列出官方源码位置、已对齐接口、保留差异与未实现能力；[异步 API 与取消语义](docs/async-api.md) 说明真实 IRQ、共享向量、executor 及等待的边界。原则是：**共享 IP 的寄存器契约、芯片实例的连接事实、HAL 的安全所有权和驱动算法分层处理。** 生成只读 metadata 不等于生成了安全驱动。

### 接口与保留的所有权契约

- 直接使用官方 `embassy-hal-internal =0.5.0` 的 `Peri<'d, T>` 和 `PeripheralType`。`Peripherals` 字段是 `Peri<'static, peripherals::T>`；GPIO、ATIM、ADC、模拟与数学驱动消耗带 lifetime 的 `Peri`，支持受借用检查器约束的 `reborrow()`。
- GPIO 的 `Pin` 是 sealed trait，`AnyPin` 是擦除具体引脚身份后的类型；身份可复制不代表独占 `Peri` 可复制。外设信号约束仍要求具体的已审核 pin 类型。
- `init(Config) -> Peripherals` 采用 Embassy 入口形状；需要处理错误时用 `try_init(Config) -> Result<Peripherals, InitError>`。配置包含 `config.rcc`；通过 `rcc::clocks()` 读取已初始化的标称时钟，不再使用 `p.clocks`。
- 默认 feature 只有 `rt`；芯片、`memory-x`、`metadata` 和 `unstable-pac` 均显式选择。每个独立示例 crate 声明其需要的芯片、内存特性和依赖。`defmt` 启用实际依赖和上游 singleton/`Peri` 格式化实现。
- `rt` 提供生成的物理 IRQ 向量及 `device.x`；`bind_interrupts!` 生成真实分发入口和对应 Binding。`Adc<I, M>` 与 `Comp<I, M>` 通过 `new_blocking` 或带 Binding 的 `new` 选择模式；GPIO `InterruptInput` 持有逐 pin 的 Binding。现有 `ThreePhasePwm` 与 `Cordic` 保留 `into_async(binding)`；等待均由真实外设 IRQ 唤醒。共享向量必须列出每个实际使用的 handler。
- ADC 等待完整序列、VC 等待边沿/电平、ATIM 等待 update/break、CORDIC 等待运算完成。取消 future 撤销本次等待；ATIM 取消等待不自动关闭已有功率输出或确认故障。EAU 和 OPA 校准没有经审查的完成 IRQ，保留有界阻塞接口。
- **时间驱动使用独立 GTIM1，HAL init 在 RCC 之后自动启动。** `time-driver-any` 实际启用 `time-driver-gtim1`，从交给应用的 Peripherals 字段中移除 GTIM1，保留给时间驱动，不占 ATIM 或 SysTick；标称 1 μs 分辨率不表示实测准确度或任务调度精度。没有空的 `time`、EXTI 或 DMA feature。

## 目录

```text
cw32-data/                      唯一可维护硬件数据源：YAML
  registers/gpio/l012.yaml         共享寄存器 kind/version
  registers/sysctrl/l012.yaml      register/field/access/kind(bool/enum/raw)
  families/cw32l012.yaml         芯片 family 的实例地址、门控位、物理 IRQ
  chips/cw32l012c8.yaml           memory、pins、routing/remap/quirks
  chips/cw32f030c8.yaml           F030C8 的内存与芯片能力，不含封装模型
  perimap.yaml                 带来源的芯片/实例/vendor IP→规范 kind/version 选型与别名
  fixes/*.yaml                 带来源的数据纠错；不是 silicon errata workaround
  sources/provenance.yaml      原厂证据、冲突及未知范围
cw32-gen/                       统一 host-only 生成器 crate
  src/schema.rs                 纯版本化 IR + 共享校验；不加载 YAML、不生成 PAC
  src/data/mod.rs               YAML → normalize/validate → 写入 JSON
  src/data/extensions.rs        fixes、perimap 选型与 aliases
  src/pac.rs                    从文件读取/校验 JSON → PAC + Rust metadata
  src/lib.rs                    generate(source, selector, json, pac) 编排两阶段
  src/main.rs                   generate / data / pac CLI
generated-data/                本地生成中间产物，ignore，不入源码包
  chips/*.json                  芯片组成及 register references
  registers/*_*.json            共享 kind/version 寄存器定义
cw32-metapac/generated/         预生成的整个 PAC 输出树，ignore；普通构建只消费
  common.rs, metadata_types.rs 公共访问核心与 metadata 类型，各一份
  peripherals/, registers/    每个 kind/version 的 PAC 与 metadata，各一份
  chips/<chip>/                只选择共享模块、实例、IRQ 和芯片关联
embassy-cw32/                   HAL 算法 + metadata 驱动的构建期特化
  src/adc/mod.rs                统一公开入口，按 adc_l012/adc_f030 选择实现
  src/adc/{l012,f030}.rs        ADC IP 后端；ATIM/RCC 同样按版本，analog 再按具体 IP 分层
  src/gpio/{mod,shared}.rs      经确认可复用的 GPIO 驱动保留一份
  src/dma/                     实际通道 owner、Transfer、IRQ 与保守取消契约
  src/motor/                   显式 unsafe 获取的电机采样、换相、定时与保护操作
  src/time_driver/mod.rs        GTIM 时间驱动入口；实现、core 与 queue 收入此目录
xtask/                         统一生成库的薄封装：regenerate、漂移与流水线检查
vendor/                        固定原厂 header/SVD，离线证据；不是生成输入
```

HAL 入口按外设 kind 是否存在开启；驱动后端按该实例的 IP version 选择，GPIO 可选能力直接从寄存器 metadata 推导。芯片 feature 只选择 metadata，不生成全局芯片/系列 cfg。ATIM/GTIM 与各模拟 IP 独立选型，共同事件和等待流程只维护一份。详见 [分层说明](docs/hal-cfg-layering.md)。

芯片 family 不等于 IP version；同一芯片内多个兼容 GPIO 实例复用同一 register block，F030 的不兼容布局采用专用 IP version。单一芯片当前不支持同一个 kind 同时选多个 version。schema10 通过显式 block 引用支持嵌套子块及数组，并记录 DMA 通道和请求连接；尚未实现上游通用继承和完整变换体系。共享 register JSON、Rust PAC 与 register metadata 均按 kind/version 去重；芯片只选择这些模块并定义实例。相同复位值的不同来源也确定性合并，不再丢失先前来源。

外设 IP 版本统一使用芯片系列标识 `l012`、`f030`，不使用 `v1`/`v2`。它是寄存器兼容模型的名称，不是芯片白名单：F030 的 IWDT/WWDT 经审查与 L012 相同，仍引用 `l012`，不复制一份。YAML 文件名、version 字段、normalized JSON、PAC metadata、HAL cfg 与后端文件名沿用同一标识。JSON schema 的版本号与原厂文档修订号是独立概念。

## 干净源码自举与两阶段生成

### 第一步：显式生成（干净 checkout 必须执行）

```sh
cargo run -p xtask -- regenerate
cargo run --offline -p xtask -- regenerate --check
```

“离线/发布前”指生成时机；默认Cargo可下载缺失依赖，--offline要求已有完整缓存。第一条同时使 Cargo 在本地生成被 ignore 的 Cargo.lock，随后在新临时目录执行：

1. xtask 的薄封装调用 `cw32_gen::generate`；其中 `cw32_gen::data` 读取所有芯片 YAML，应用显式 fixes 和 perimap，排序归一化、校验，将 chips/*.json 与 registers/kind_version.json 实际写入磁盘。
2. `cw32_gen::pac` 从这些文件重新读取并校验 JSON，按 register references 加载共享定义，再生成每芯片 PAC 和 Rust METADATA。schema、data 与 pac 是同一个 crate 内的模块边界；统一 crate 包含 YAML 依赖，但 schema 保持纯模型/校验职责，PAC 渲染阶段不加载 YAML，也不直接接收上一阶段的内存模型。
3. 替换由工具拥有的 generated-data/ 与 cw32-metapac/generated/ 两个生成树。

--check 重新生成并逐字节检查文件集合与内容；发现缺失、漂移、多余文件即失败，不修改产物。干净源码先 regenerate，再 --check；没有生成物时 --check 明确失败，绝不默默写入。CI包含此检查。JSON 是可发布中间接口，不是维护源；PAC 同样禁止手改。

也可直接调用统一 CLI（在新的输出目录中生成，或用于干净源码自举）：

```sh
cargo run -p cw32-gen -- generate cw32-data all generated-data cw32-metapac/generated
```

需要单独检查某一阶段时，可以分别运行：

```sh
cargo run --locked -p cw32-gen -- data cw32-data all generated/data
cargo run --locked -p cw32-gen -- pac generated/data/chips/cw32l012c8.json generated/pac/cw32l012c8
```

`generate` 与 `data` 的单芯片模式可将 all 换成 cw32l012c8。多芯片发布使用 all，底层 `cw32_gen::data::generate_many` 先检查相同 kind/version 是否确为同一份寄存器契约，冲突时拒绝生成并要求拆 IP version。`data::write_json` 也拒绝覆盖已有但内容不同的同名寄存器产物。修改源码后应在新目录生成，或使用上述统一 Rust xtask，由它替换工具拥有的生成树。

### 第二步：消费本地预生成产物

```sh
rustup target add thumbv6m-none-eabi
cargo check -p embassy-cw32 --target thumbv6m-none-eabi --features cw32l012c8
```

测试工具链 Rust/Cargo 1.99.0，Cargo.lock 在本地生成并被 ignore；手工 manifest 固定直接依赖版本，但不承诺跨时点的完整传递依赖图一致。PAC 普通 build.rs **仅选择预生成 Rust 路径**，没有 `cw32-gen` 或其他 generator build dependency，不读取 YAML/JSON。HAL runtime 和 build.rs 依赖同一 cw32-metapac；build.rs 读取 METADATA 生成 kind/version cfg、pin singleton、GPIO 端口与门控掩码、IRQ 标记和只读关联表，并在 `memory-x` 开启时写出 memory.x。模块 gate 实际使用 kind/version cfg；L012 与 F030 的不兼容寄存器版本选择独立驱动，不支持的版本不会被当作兼容布局。驱动算法实际使用生成的 PAC 字段。正常 PAC 构建不复制整份输出到 OUT_DIR；metadata 和 runtime 文件只在对应 feature 开启时要求存在。

HAL 默认 feature 为 `rt`，不会默认选择芯片；HAL 检查需加 `--features cw32l012c8`，仅构建生成器/xtask 不需要选择芯片。PAC 也没有默认芯片，直接使用 PAC 时同样需要明确选择。`--no-default-features --features cw32l012c8` 可检查不含 runtime 的 HAL。未选芯片报错；不自动将名称相近的 CW32 型号视为兼容。PAC/HAL 为 no_std；生成器在 host 运行。HAL `metadata` feature 控制公开关联表，并转发 PAC metadata feature；build dependency 始终启用 metadata 供生成使用。`unstable-pac` 才公开 `embassy_cw32::pac`。未开启 `memory-x` 时，应用负责自己的内存布局和链接配置。

`cw32l012c8` + `rt` 本身不提供 critical-section backend。应用若未通过其他依赖获得正确 backend，需在自己的 Cargo.toml 中加入 `cortex-m = { version = "=0.7.7", features = ["critical-section-single-core"] }`，或提供另一个适合本芯片的实现；不要同时链接互相冲突的 backend。主机测试的 `std` backend 不能代表裸机配置已具备该实现。

## 已实现的数据能力及约束

- register：名称、offset、access、字段范围与 overlap；生成 typed raw/bool/enum getter 与 setter。bool 限制为一位，enum 校验值域及重复值，保留值读取返回 None。完整数据按原厂字段审计，不为缺少证据的位值编造枚举。当前真实数据中的多位字段仍为 raw，不能把 enum 生成能力当作已完成全部硬件枚举。
- JSON schema v8 显式维护字段、寄存器及子块数组，保留完整寄存器 reset_value 与实例级 register_resets；未知值不当作零，见[默认值与write API](docs/register-reset-defaults.md)。继续不包含 Package：`Chip.pins` 直接给出芯片引脚能力。新增 `Register.bit_size` 支持真实 8/16/32-bit volatile 总线访问，省略时为32位；字段范围、对齐和有声明的别名按访问宽度校验，不能用32位读取后截断冒充8位硬件操作。旧版本 JSON 必须重新生成。模型也显式描述同址 alias、寄存器访问副作用、通用门控/复位及 ownership_parent。W0C/W1C 按其实际写入语义显式清除；只有普通 RW 类型允许 typed modify。I2C 合法同址视图必须明确声明，不能以关闭重叠校验来放行；DMA 总块与 channel 视图不能获得互相冲突的安全所有权。
- perimap：精确 chip+instance+vendor_ip/vendor_version（或原block/version）匹配，保留datasheet实例名。mode: select在加载前选择规范kind/version/register block文件，即使原vendor名没有本地文件也可；mode: alias仅relabel已加载模型。外设根 block 的 perimap 仍精确匹配；嵌套 block 依赖显式解析，不支持上游通用数据库/正则映射。同chip相同原block身份不能选互相矛盾的模型，必须先区分源身份。
- fixes：原block名下的寄存器/字段纠错，先于alias；要求source/reason，未知目标、错键、冲突均报错，失败回滚。当前selector作用于该chip中共享这个block的全部实例，不支持仅GPIOA特例。局部布局差异应拆版本/数据模型，不能改坏公共 l012 模型。
- shared IRQ：物理 IRQ 表唯一；peripheral signal→IRQ 可多对多。重复引用不复制物理向量。PAC runtime 按物理 IRQ 号生成向量，稀疏编号保留空槽；HAL `bind_interrupts!` 只为实际声明的 handler 生成入口和 Binding，不从关联表批量生成空证明。
- 芯片 `pins`、pin-signal routes 与 remap 分开；route 验证目标存在。引脚 token 表示芯片能力，不保证某封装或开发板实际接出。现有 FOC 与新增 ATIM/GTIM main-output 路由有逐项证据，尚未审核的其他信号不会推断生成。quirks 与数据 fixes 分开；未列 quirk 不代表不存在 errata。
- 确定性排序，不含时间戳/绝对源路径；JSON schema_version校验；两chip fixture证明共享寄存器复用、芯片pin集合隔离、同名异内容拒绝、拆新version后共存。

当前输入是人工审计的分层YAML，不是自动融合所有SVD/厂商数据库的通用导入器。原始 vendor/cw32l012.h、CW32L012.svd 及人工审计 manifest.json 是必要的只读证据，不是可再生成产物。可重新下载的 SDK 压缩包放 vendor/cache/ 或 vendor/downloads/，这两处与 vendor 下 zip/pack 都被 ignore，不进入源码包。原始vendor证据只读，源差异记录在provenance；禁止把教学fixture作为真实芯片资料。

## L012C8 数据覆盖

- 50 个实例/视图、28 类 IP、306 个逻辑寄存器、1713 个字段；统计包含有声明的 I2C 同址视图和 DMA 重叠视图，不把它们误算成独立可占有硬件。
- 32 个物理 IRQ、46 个外设信号绑定、45 组门控/复位关联；共享 IRQ 引用不复制向量。runtime 提供中断入口连接机制，具体外设驱动仍须实现 pending/clear/wake 算法。
- L012C8 直接维护40个 GPIO 能力和259条路由（原126条模拟/定时器路由，加133条 UART/SPI/I2C 路由），不在数据模型中保存封装脚号。
- 106 项已审查副作用信息。头文件、SVD 与手册的差异逐项记录；VCREF DIV、I2C RXWATER 等尚有原厂资料冲突，采用有证据的保守范围，不能称为已获硅验证。
- 64 KiB Flash、8 KiB RAM，未把有资料冲突的 Boot ROM 区域作为可用链接内存。

详见 [全寄存器覆盖及剩余冲突](docs/full-register-coverage.md)、[逐外设语义审查](docs/full-peripheral-semantics.md)、[实例与中断](docs/peripheral-instances.md) 和 [引脚/路由证据](docs/pin-route-evidence.md)。完整覆盖测试核对集合，不仅核对已有项的值，因此新增缺项也会使检查失败。

## F030C8 新增支持

选择 `cw32f030c8`；不要附加 T6/T7/U7。它与 `cw32l012c8` 必须互斥，HAL 和 PAC 必须选择同一芯片。F030C8 为64 KiB Flash、8 KiB SRAM，39个 GPIO 能力、225条已审核路由（原107条模拟/定时器路由，加116条 UART/SPI/I2C 和2条 LSE 路由）；完整原厂寄存器覆盖与驱动差异见 [F030支持说明](docs/cw32f030-support.md)。

F030只有一路ADC、两路VC，没有L012的OPA、DAC、CORDIC、EAU；不存在的模块和外设不会出现在该芯片的安全API中。ADC/ATIM/VC以及RCC/GTIM使用独立寄存器版本与驱动。F030 ATIM硬件有比较影子寄存器，但本版严格三相批量 `set_duty` 尚不承诺运行中无扰原子提交；功率输出开启时返回 Busy。这是本版API的限制，不是硬件不支持运行时PWM更新。不能据此声称完整实时FOC控制已可用。


普通库应用显式选择一个芯片，不能用 `--all-features` 同时选择多颗芯片。

## HAL 范围

保留 embedded-hal 1.0 阻塞 GPIO、经过校验的 HSI 启动配置和可选专用 GTIM1 时间驱动。L012 默认 HSI/24、总线不分频，标称4 MHz；F030 默认 HSI/6，标称8 MHz。`config.rcc.hsi_divider`、`hclk_divider`、`pclk_divider` 选择已记载的分频组合，按要求先配置 Flash 等待周期；96MHz L012 仍要求 VDD 至少1.8V。`Config::clocks()` 是配置预测，`rcc::clocks()` 是成功初始化后的频率。`hsi_stabilization_limit` 是 trim 后的有界轮询次数。RTC 的低速 LSI/LSE 可独立启动；HSE/PLL 系统时钟、动态切换及低功耗恢复仍未实现。

`rcc::clocks()` 在成功初始化前会 panic。与它不同，启用的 Embassy 时间驱动在初始化前返回时间 0 且不访问硬件；提前登记的唤醒会保留到初始化。16 个槽是存储容量，队列满时提前唤醒被替换任务以重试，不再 panic；Timer future 会重新核对 deadline。

启用 `time-driver-any` 或 `time-driver-gtim1` 后，HAL 初始化自动启动专用 16-bit GTIM1；`Config.time_interrupt_priority` 默认 P0。GTIM1 从交给应用的 `Peripherals` 字段中移除，保留给时间驱动，不占用 FOC 的 ATIM；关闭时间驱动时仍可安全取得 GTIM1。旧 `time-driver-systick` 和手动转交 `core.SYST` 的接口已撤销。

L012 的4 MHz PCLK 经 PSC=3 得到标称1 MHz计数；F030用8 MHz PCLK及CR0.PRS=3的2ⁿ分频也得到标称1 MHz计数，其OV/CNT/OV一致快照算法不依赖不存在的UIFREMAP，ARR=65535，每 65.536 ms 溢出。从 overflow 到 ISR 清除 UIF 必须严格小于一个完整周期，包含 IRQ/critical-section/优先级阻塞时间。compare 写入实际 deadline，写后重读计数并在必要时 pend IRQ，避免错过已到期限。短 sleep 仍受所选芯片 CPU、MMIO、IRQ 和 executor 实际延迟影响。**1 μs timestamp 分辨率不等于 HSI 实测精度，也不保证 1 μs 唤醒准确度。** 未支持 STOP 或动态调频。实际算法、链接与测试结果见 [验证记录](docs/validation-v0.11.6.md)，不能从 API 文档推断已经完成硅验证。

L012 的 FOC 对应硬件为：ATIM、ADC1/2、OPA1/2、VC1～4、DAC、CORDIC，另有 EAU 数学运算接口。保留 `atim`、`analog`、`eau` 硬件专用模块；`timer::low_level::Timer` 与 `timer::simple_pwm::SimplePwm` 提供真实通用计数器、可选逐路引脚与借用 channel。CW32 只有两路 OPA，不把 STM32G431 的 OPAMP3 虚构为对应外设。ADC/Comp 使用共享的 `Blocking`/`Async` 模式 owner，ADC 按每次调用借入通道，固定序列是借用 owner 的 `Sequence` 扩展。ATIM/CORDIC 保留现有专用异步 owner；CORDIC 可 `into_blocking()`。并不宣称连续 DMA 采样或硬实时 FOC 闭环。

异步方法本身没有内置超时；外部 timeout/select 必须实际 drop future 才触发清理。详见 [异步 API](docs/async-api.md)。

接口、引脚类型约束、触发行为、故障处理及例程限制见 [FOC HAL 说明](docs/foc-hal.md)。示例仅用于编译和审查；在确认原理图、驱动器极性、保护链路和电源条件前，不应直接用于连接功率级的板子。

## 安全与未完成内容

GPIO/SYSCTRL 为 HAL 共享资源，不发会与 pin token 冲突的独立寄存器所有权 token。正常 init 只交付一次资源；驱动持有相关外设与引脚的 `Peri`，可拥有 `'static` 资源或持有受约束的短借用。原始 PAC 的寄存器方法并非都要求 unsafe；应用须自行承担时钟、资源别名与副作用责任。`peripherals::T::steal()`、`AnyPin::steal()` 与 `Peri::clone_unchecked()` 是显式 unsafe 边界。类型化方向和副作用不能证明时钟、供电、保护极性、外部接线或所有保留位均正确。

UART/SPI/I2C 已提供阻塞与真实 IRQ 异步驱动；当前开放的控制器模式、错误和取消行为见 [总线阶段](docs/buses-v0.19.0.md)。DMA 已提供通道和传输层，尚无这些总线驱动的安全端点集成，也不承诺 lossless ADC 环形采样。GPIO 中断使用真实 CW32 port IRQ，不虚构 STM32 EXTI token。全芯片其他路由的类型约束及全部 silicon workaround 也未实现。已有 FOC 驱动的约束不能外推到这些未实现驱动。`bind_interrupts!` 本身只负责分发与 Binding；相应 async 构造器或专用转换负责启用 NVIC，handler/future 负责本源状态、唤醒及取消。共享 DMA、ADC2_DAC、VC13/VC24 的兄弟源必须各自正确绑定，取消不禁用共享 NVIC；未绑定向量进入默认 handler。自定义启动/向量表必须保持 runtime 的分发契约。

共享复位位不会由某个实例的构造器无条件触发；物理门控按 owner 引用计数，最后一个可安全关闭的 owner 才释放时钟，不能关闭仍存活的兄弟实例。GPIO、时间驱动、unsafe motor 接管及 DMA/ADC 隔离按各自契约保留门控。ATIM 构造保持功率输出禁能，启用由调用者显式执行。驱动策略不等于板级安全认证。

未上板、未烧录，也未验证实际时钟精度/低功耗/中断时序。算法不能从寄存器表自动推导；ICR/BRR/BSRR/TOG即使SVD写rw也有特殊副作用，不能随意RMW。

## 验证

```sh
cargo fmt --all -- --check
cargo run -p xtask -- regenerate --check
```




[验证记录](docs/validation.md) · [寄存器证据](docs/register-evidence.md) · [数据来源](cw32-data/sources/provenance.yaml)

serde_yaml0.9上游已标记deprecated，目前锁定版本使用；crate 内的 schema/data/pac 模块边界允许后续替换 YAML loader。自有代码MIT OR Apache-2.0；厂商资料遵守各自许可，见NOTICE.md。未宣称Embassy官方支持CW32。

## 源码交付与 ignore

源码 ZIP 和干净 checkout 不含 generated-data/、cw32-metapac/generated/、Cargo.lock、target/、测试临时文件、日志或下载缓存。保留审核后的 YAML、修正规则、Rust 生成器、文档以及审计所需 vendor 原厂证据和许可。源码自举、生成及文档列出的日常校验只需 Rust/Cargo，无 Python 或 PyYAML 前置条件；外部审查探针的编排脚本不属于源码项目。


`--check` 是只读漂移检查，可捕获缺失文件、多余文件和内容变化。生成工具仅替换两个明确拥有的生成目录，不能用于存放手写代码。临时校验和变异测试使用独立临时目录并自动清理。离线命令需要提前缓存所有 Rust 依赖与目标工具链；缺缓存时先联网执行正常 Cargo 命令。

本版完成 PAC/数组、GPIO共享与异步实现、ADC/Comp模式所有权及通用timer/PWM的逐项重构；不宣称覆盖全部 embassy-stm32 驱动或全部芯片功能。见[逐模块现状](docs/embassy-api-alignment.md)。

v0.13.2 将构建辅助函数合回 `build.rs`，IRQ 事件状态合入现有 `interrupt` 模块，删除两个独立 support 文件；生成结果和 IRQ 行为保持不变，见 [v0.13.2 验证](docs/validation-v0.13.2.md)。

v0.13.1 按外设 IP/能力分层与复用的变化见 [v0.13.1 验证](docs/validation-v0.13.1.md) 和 [HAL cfg 分层](docs/hal-cfg-layering.md)。此前 PAC/所有权 API 迁移记录见 [v0.13.0](docs/validation-v0.13.0.md)。

## 上传 C 工程的递进 Rust 例程（v0.18.0）

新增根目录 [`examples/`](examples/README.md)，按 01–06 逐步迁移上传工程的无感六步 BLDC 功能。该源程序不是 FOC；默认构建保持功率输出禁用。六个例程直接构建为 MCU 程序，面向原工程 CW32L012 引脚与时钟契约，尚无实板或带载验证。所有例程仅放在根目录 `examples/`。

例程为六个独立、仅有 main.rs 入口的 binary crate（无 lib.rs），位于 [`examples/l012-bldc/`](examples/l012-bldc/)，每级的业务源码都在自己的 src 内，使用普通 mod 声明，仅引入必要代码、依赖与外设，不保留主机入口、例程测试或目标平台条件分支。只有 05/06 保留 `motor-output-enable` 输出授权开关。01–04 不初始化、不写入六个桥臂引脚，实验须物理断开母线或禁用驱动；GPIO 例程仅初始化 LED/按键；采样例程不初始化 UART 或换相定时器。构建与所有权边界见[板级 README](examples/l012-bldc/README.md)，验证与 v0.11.6 执行器/ISR 分工说明见 [验证记录](docs/validation-v0.11.6.md)。
