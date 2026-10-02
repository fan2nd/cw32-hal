# embassy-cw32：YAML → normalized JSON → PAC → HAL

CW32L012C8 与 CW32F030C8，实验性 **v0.11.4**，尚未上板验证。数据、统一生成器 `cw32-gen`、PAC 与 HAL 位于同一个 Cargo workspace。schema 与两个生成阶段的代码保留在一个 crate 中，仍强制执行 YAML → 落盘的 normalized JSON → PAC，不能用内存模型跳过 JSON 接口。本版在 Embassy 风格的所有权、初始化、metadata 专化与中断运行时之上，增加 ADC、VC、ATIM 事件及 CORDIC 的真实 IRQ 异步接口；完整寄存器数据覆盖不代表全部外设驱动或 FOC 闭环均已实现。

芯片名与 feature 不带 T7、U6 等封装及温度后缀。`cw32-data` 不维护 packages 层；芯片直接定义 GPIO 能力和信号路由，实际封装是否引出、物理脚号及板级接线由板级设计负责。Flash/RAM 等芯片差异仍由 chip 数据描述。

## 先看设计与边界

[设计理由与固定版本上游审查](docs/design-rationale.md) 解释每层为何存在；[Embassy API 逐项对照](docs/embassy-api-alignment.md) 列出官方源码位置、已对齐接口、保留差异与未实现能力；[异步 API 与取消语义](docs/async-api.md) 说明真实 IRQ、共享向量、executor 及等待的边界。原则是：**共享 IP 的寄存器契约、芯片实例的连接事实、HAL 的安全所有权和驱动算法分层处理。** 生成只读 metadata 不等于生成了安全驱动。

### 接口与保留的所有权契约

- 直接使用官方 `embassy-hal-internal =0.5.0` 的 `Peri<'d, T>` 和 `PeripheralType`。`Peripherals` 字段是 `Peri<'static, peripherals::T>`；GPIO、ATIM、ADC、模拟与数学驱动消耗带 lifetime 的 `Peri`，支持受借用检查器约束的 `reborrow()`。
- GPIO 的 `Pin` 是 sealed trait，`AnyPin` 是擦除具体引脚身份后的类型；身份可复制不代表独占 `Peri` 可复制。外设信号约束仍要求具体的已审核 pin 类型。
- `init(Config) -> Peripherals` 采用 Embassy 入口形状；需要处理错误时用 `try_init(Config) -> Result<Peripherals, InitError>`。配置包含 `config.rcc`；通过 `rcc::clocks()` 读取已初始化的标称时钟，不再使用 `p.clocks`。
- 默认 feature 只有 `rt`；芯片、`memory-x`、`metadata` 和 `unstable-pac` 均显式选择。`example` 汇总例程需要的芯片、内存和示例依赖。`defmt` 启用实际依赖和上游 singleton/`Peri` 格式化实现。
- `rt` 提供生成的物理 IRQ 向量及 `device.x`；`bind_interrupts!` 生成真实分发入口和对应 Binding。`Adc`、`Comparator`、`ThreePhasePwm`、`Cordic` 通过 `into_async(binding)` 转移到异步 owner；它们的等待由外设 IRQ 唤醒，不是包在 `async fn` 中的 busy loop。共享向量必须列出每个实际使用的 handler。
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
cw32-metapac/src/chips/         本地预生成 PAC/metadata，ignore；普通构建只消费
embassy-cw32/                   HAL 算法 + metadata 驱动的构建期特化
  src/adc/mod.rs                统一公开入口，按 adc_l012/adc_f030 选择实现
  src/adc/{l012,f030}.rs        对称的芯片系列 IP 实现；analog/atim/rcc 同样组织
  src/gpio/{mod,shared}.rs      经确认可复用的 GPIO 驱动保留一份
  src/time_driver/mod.rs        GTIM 时间驱动入口；实现、core 与 queue 收入此目录
xtask/                         统一生成库的薄封装：regenerate、漂移与流水线检查
vendor/                        固定原厂 header/SVD，离线证据；不是生成输入
```

芯片 family 不等于 IP version；同一芯片内多个兼容 GPIO 实例复用同一 register block，F030 的不兼容布局采用专用 IP version。单一芯片当前不支持同一个 kind 同时选多个 version，这与受查上游生成器的限制相同。当前 block 字段表示简化的 kind，一个 kind/version 一个 RegisterBlock；尚未扩成上游多个 block/继承模型。共享register JSON已去重，Rust寄存器类型当前在每个chip PAC内复用，尚未把跨chip公共Rust模块独立打包。

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
3. 替换由工具拥有的 generated-data/ 与 cw32-metapac/src/chips/ 两个生成树。

--check 重新生成并逐字节检查文件集合与内容；发现缺失、漂移、多余文件即失败，不修改产物。干净源码先 regenerate，再 --check；没有生成物时 --check 明确失败，绝不默默写入。CI包含此检查。JSON 是可发布中间接口，不是维护源；PAC 同样禁止手改。

也可直接调用统一 CLI（在新的输出目录中生成，或用于干净源码自举）：

```sh
cargo run -p cw32-gen -- generate cw32-data all generated-data cw32-metapac/src/chips
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

测试工具链 Rust/Cargo 1.99.0，Cargo.lock 在本地生成并被 ignore；手工 manifest 固定直接依赖版本，但不承诺跨时点的完整传递依赖图一致。PAC 普通 build.rs **仅选择预生成文件**，没有 `cw32-gen` 或其他 generator build dependency，不读取 YAML/JSON。HAL runtime 和 build.rs 依赖同一 cw32-metapac；build.rs 读取 METADATA 生成 kind/version cfg、pin singleton、GPIO 端口与门控掩码、IRQ 标记和只读关联表，并在 `memory-x` 开启时写出 memory.x。模块 gate 实际使用 kind/version cfg；L012 与 F030 的不兼容寄存器版本选择独立驱动，不支持的版本不会被当作兼容布局。驱动算法实际使用生成的 PAC 字段。

HAL 默认 feature 为 `rt`，不会默认选择芯片；HAL 检查需加 `--features cw32l012c8`，仅构建生成器/xtask 不需要选择芯片。PAC 也没有默认芯片，直接使用 PAC 时同样需要明确选择。`--no-default-features --features cw32l012c8` 可检查不含 runtime 的 HAL。未选芯片报错；不自动将名称相近的 CW32 型号视为兼容。PAC/HAL 为 no_std；生成器在 host 运行。HAL `metadata` feature 控制公开关联表，并转发 PAC metadata feature；build dependency 始终启用 metadata 供生成使用。`unstable-pac` 才公开 `embassy_cw32::pac`。未开启 `memory-x` 时，应用负责自己的内存布局和链接配置。

`cw32l012c8` + `rt` 本身不提供 critical-section backend。应用若未通过其他依赖获得正确 backend，需在自己的 Cargo.toml 中加入 `cortex-m = { version = "=0.7.7", features = ["critical-section-single-core"] }`，或提供另一个适合本芯片的实现；不要同时链接互相冲突的 backend。主机测试的 `std` backend 不能代表裸机配置已具备该实现。

## 已实现的数据能力及约束

- register：名称、offset、access、字段范围与 overlap；Field、BoolField、EnumField。bool 限制为一位，enum 校验值域及重复值，保留值读取返回 None。完整数据按原厂字段审计，不为缺少证据的位值编造枚举。
- JSON schema v3 不再包含 Package：`Chip.pins` 直接给出芯片引脚能力。新增 `Register.bit_size` 支持真实 8/16/32-bit volatile 总线访问，省略时为32位；字段范围、对齐和有声明的别名按访问宽度校验，不能用32位读取后截断冒充8位硬件操作。旧版本 JSON 必须重新生成。模型也显式描述同址 alias、寄存器访问副作用、通用门控/复位及 ownership_parent。W0C/W1C 提供对应清除操作；只有普通 RW 类型允许 typed modify。I2C 合法同址视图必须明确声明，不能以关闭重叠校验来放行；DMA 总块与 channel 视图不能获得互相冲突的安全所有权。
- perimap：精确 chip+instance+vendor_ip/vendor_version（或原block/version）匹配，保留datasheet实例名。mode: select在加载前选择规范kind/version/register block文件，即使原vendor名没有本地文件也可；mode: alias仅relabel已加载模型。当前只支持RegisterBlock单block、精确匹配，不支持上游通用数据库/正则映射。同chip相同原block身份不能选互相矛盾的模型，必须先区分源身份。
- fixes：原block名下的寄存器/字段纠错，先于alias；要求source/reason，未知目标、错键、冲突均报错，失败回滚。当前selector作用于该chip中共享这个block的全部实例，不支持仅GPIOA特例。局部布局差异应拆版本/数据模型，不能改坏公共 l012 模型。
- shared IRQ：物理 IRQ 表唯一；peripheral signal→IRQ 可多对多。重复引用不复制物理向量。PAC runtime 按物理 IRQ 号生成向量，稀疏编号保留空槽；HAL `bind_interrupts!` 只为实际声明的 handler 生成入口和 Binding，不从关联表批量生成空证明。
- 芯片 `pins`、pin-signal routes 与 remap 分开；route 验证目标存在。引脚 token 表示芯片能力，不保证某封装或开发板实际接出。尚未审核的非 FOC pinmux 不会推断生成。quirks 与数据 fixes 分开；未列 quirk 不代表不存在 errata。
- 确定性排序，不含时间戳/绝对源路径；JSON schema_version校验；两chip fixture证明共享寄存器复用、芯片pin集合隔离、同名异内容拒绝、拆新version后共存。

当前输入是人工审计的分层YAML，不是自动融合所有SVD/厂商数据库的通用导入器。原始 vendor/cw32l012.h、CW32L012.svd 及人工审计 manifest.json 是必要的只读证据，不是可再生成产物。可重新下载的 SDK 压缩包放 vendor/cache/ 或 vendor/downloads/，这两处与 vendor 下 zip/pack 都被 ignore，不进入源码包。原始vendor证据只读，源差异记录在provenance；禁止把教学fixture作为真实芯片资料。

## L012C8 数据覆盖

- 50 个实例/视图、28 类 IP、306 个逻辑寄存器、1713 个字段；统计包含有声明的 I2C 同址视图和 DMA 重叠视图，不把它们误算成独立可占有硬件。
- 32 个物理 IRQ、46 个外设信号绑定、45 组门控/复位关联；共享 IRQ 引用不复制向量。runtime 提供中断入口连接机制，具体外设驱动仍须实现 pending/clear/wake 算法。
- L012C8 直接维护40个 GPIO 能力和82条FOC路由（ATIM28、ADC24、OPA12、VC16、DAC2），不在数据模型中保存封装脚号。
- 106 项已审查副作用信息。头文件、SVD 与手册的差异逐项记录；VCREF DIV、I2C RXWATER 等尚有原厂资料冲突，采用有证据的保守范围，不能称为已获硅验证。
- 64 KiB Flash、8 KiB RAM，未把有资料冲突的 Boot ROM 区域作为可用链接内存。

详见 [全寄存器覆盖及剩余冲突](docs/full-register-coverage.md)、[逐外设语义审查](docs/full-peripheral-semantics.md)、[实例与中断](docs/peripheral-instances.md) 和 [引脚/路由证据](docs/pin-route-evidence.md)。完整覆盖测试核对集合，不仅核对已有项的值，因此新增缺项也会使检查失败。

## F030C8 新增支持

选择 `cw32f030c8`；不要附加 T6/T7/U7。它与 `cw32l012c8` 必须互斥，HAL 和 PAC 必须选择同一芯片。F030C8 为64 KiB Flash、8 KiB SRAM，39个 GPIO 能力、61条已审核 ADC/ATIM/VC 路由；完整原厂寄存器覆盖与驱动差异见 [F030支持说明](docs/cw32f030-support.md)。

F030只有一路ADC、两路VC，没有L012的OPA、DAC、CORDIC、EAU；不存在的模块和外设不会出现在该芯片的安全API中。ADC/ATIM/VC以及RCC/GTIM使用独立寄存器版本与驱动。F030 ATIM硬件有比较影子寄存器，但本版严格三相批量 `set_duty` 尚不承诺运行中无扰原子提交；功率输出开启时返回 Busy。这是本版API的限制，不是硬件不支持运行时PWM更新。不能据此声称完整实时FOC控制已可用。


普通库应用显式选择一个芯片，不能用 `--all-features` 同时选择多颗芯片。

## HAL 范围

保留 embedded-hal 1.0 阻塞 GPIO、复位时钟初始化和可选专用 GTIM1 时间驱动。L012 RCC 默认保持 HSI/24、总线不分频的标称4 MHz配置；v0.10.0增加从已验证复位状态显式选择96MHz HSI及APB/2（48MHz）的配置，先设置Flash等待周期，VDD必须至少1.8V；F030 RCC 按其独立复位配置使用 HSI 48 MHz/6 的标称8 MHz；`config.rcc.hsi_stabilization_limit` 是 trim 后的有界轮询次数，不是任意时钟树或校准后的时间超时。没有把更高频率支持混同于完整 SYSCTRL 寄存器数据。

`rcc::clocks()` 在成功初始化前会 panic。

启用 `time-driver-any` 或 `time-driver-gtim1` 后，HAL 初始化自动启动专用 16-bit GTIM1；`Config.time_interrupt_priority` 默认 P0。GTIM1 从交给应用的 `Peripherals` 字段中移除，保留给时间驱动，不占用 FOC 的 ATIM；关闭时间驱动时仍可安全取得 GTIM1。旧 `time-driver-systick` 和手动转交 `core.SYST` 的接口已撤销，。

L012 的4 MHz PCLK 经 PSC=3 得到标称1 MHz计数；F030用8 MHz PCLK及CR0.PRS=3的2ⁿ分频也得到标称1 MHz计数，其OV/CNT/OV一致快照算法不依赖不存在的UIFREMAP，ARR=65535，每 65.536 ms 溢出。从 overflow 到 ISR 清除 UIF 必须严格小于一个完整周期，包含 IRQ/critical-section/优先级阻塞时间。compare 写入实际 deadline，写后重读计数并在必要时 pend IRQ，避免错过已到期限。短 sleep 仍受所选芯片 CPU、MMIO、IRQ 和 executor 实际延迟影响。**1 μs timestamp 分辨率不等于 HSI 实测精度，也不保证 1 μs 唤醒准确度。** 未支持 STOP 或动态调频。实际算法、链接与测试结果见 [验证记录](docs/validation-v0.11.4.md)，不能从 API 文档推断已经完成硅验证。

L012 的 FOC 对应硬件为：ATIM、ADC1/2、OPA1/2、VC1～4、DAC、CORDIC，另有 EAU 数学运算接口。保留 `atim`、`analog`、`eau` 模块，不为外形相似而虚构 STM32 `timer`/`opamp` API 兼容性。CW32 只有两路 OPA，不把 STM32G431 的 OPAMP3 虚构为对应外设。原阻塞接口继续保留；ADC、VC、ATIM 事件和 CORDIC 可显式转换为 IRQ-driven async owner。ADC/CORDIC 还可 `into_blocking()`，VC/ATIM 当前没有该逆转换。并不宣称连续 DMA 采样或硬实时 FOC 闭环。

异步方法本身没有内置超时；外部 timeout/select 必须实际 drop future 才触发清理。详见 [异步 API](docs/async-api.md)。

接口、引脚类型约束、触发行为、故障处理及例程限制见 [FOC HAL 说明](docs/foc-hal.md)。示例仅用于编译和审查；在确认原理图、驱动器极性、保护链路和电源条件前，不应直接用于连接功率级的板子。

## 安全与未完成内容

GPIO/SYSCTRL 为 HAL 共享资源，不发会与 pin token 冲突的独立寄存器所有权 token。正常 init 只交付一次资源；驱动持有相关外设与引脚的 `Peri`，可拥有 `'static` 资源或持有受约束的短借用。原始 PAC 访问、`peripherals::T::steal()`、`AnyPin::steal()` 与 `Peri::clone_unchecked()` 仍是显式 unsafe 边界。类型化方向和副作用不能证明时钟、供电、保护极性、外部接线或所有保留位均正确。

仍未实现 UART/SPI/I2C/DMA/EXTI 等通用 HAL、全芯片所有信号的类型约束、完整 reset-value 模型或全部 silicon workaround。已有 FOC 驱动的约束不能外推到这些未实现驱动。`bind_interrupts!` 本身只负责分发与 Binding；已有异步 driver 的 `into_async` 负责启用 NVIC，handler/future 负责本源状态、唤醒及取消。ADC2_DAC、VC13/VC24 的兄弟源必须各自正确绑定，取消不禁用共享 NVIC；未绑定向量进入默认 handler。自定义启动/向量表必须保持 runtime 的分发契约。

共享复位位不会由某个实例的构造器无条件触发；时钟门控保守保持开启，不因一个驱动释放而关闭兄弟实例。ATIM 构造保持功率输出禁能，启用由调用者显式执行。驱动的保守策略不等于板级安全认证。

未上板、未烧录，也未验证实际时钟精度/低功耗/中断时序。算法不能从寄存器表自动推导；ICR/BRR/BSRR/TOG即使SVD写rw也有特殊副作用，不能随意RMW。

## 验证

```sh
cargo fmt --all -- --check
cargo run -p xtask -- regenerate --check
```




[验证记录](docs/validation.md) · [寄存器证据](docs/register-evidence.md) · [数据来源](cw32-data/sources/provenance.yaml)

serde_yaml0.9上游已标记deprecated，目前锁定版本使用；crate 内的 schema/data/pac 模块边界允许后续替换 YAML loader。自有代码MIT OR Apache-2.0；厂商资料遵守各自许可，见NOTICE.md。未推送远端，未宣称Embassy官方支持CW32。

## 源码交付与 ignore

源码 ZIP 和干净 checkout 不含 generated-data/、cw32-metapac/src/chips/、Cargo.lock、target/、测试临时文件、日志或下载缓存。保留审核后的 YAML、修正规则、Rust 生成器、文档以及审计所需 vendor 原厂证据和许可。所有开发与校验命令均为 Rust/Cargo，无 Python 或 PyYAML 前置条件。


`--check` 是只读漂移检查，可捕获缺失文件、多余文件和内容变化。生成工具仅替换两个明确拥有的生成目录，不能用于存放手写代码。临时校验和变异测试使用独立临时目录并自动清理。离线命令需要提前缓存所有 Rust 依赖与目标工具链；缺缓存时先联网执行正常 Cargo 命令。

## 上传 C 工程的递进 Rust 例程（v0.11.4）

新增根目录 [`example/`](example/README.md)，按 01–06 逐步迁移上传工程的无感六步 BLDC 功能。该源程序不是 FOC；默认构建保持功率输出禁用。六个例程直接构建为 MCU 程序，面向原工程 CW32L012 引脚与时钟契约，尚无实板或带载验证。所有例程仅放在根目录 `example/`。

例程为六个独立、仅有 main.rs 入口的 binary crate（无 lib.rs），位于 [`example/l012-bldc/`](example/l012-bldc/)，每级的业务源码都在自己的 src 内，使用普通 mod 声明，仅引入必要代码、依赖与外设，不保留主机入口、例程测试或目标平台条件分支。只有 05/06 保留 `motor-output-enable` 输出授权开关。01–04 不初始化、不持有、不写入六个桥臂引脚，实验须物理断开母线或禁用驱动；GPIO 例程仅初始化 LED/按键；采样例程不初始化 UART 或换相定时器。构建与所有权边界见[板级 README](example/l012-bldc/README.md)，验证与 v0.11.4 直接 PAC/IRQ 所有权说明见 [验证记录](docs/validation-v0.11.4.md)。
