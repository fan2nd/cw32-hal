# v0.12.0：寄存器 reset defaults 与 typed write

## 数据及证据

`cw32-data/registers/*/{l012,f030}.yaml` 每个寄存器维护完整 `reset_value`、`reset_source` 和必要的 `reset_note`。来源为已固定 SHA-256 的 L012 RM1.4、F030 RM2.5；每条来源写明章节、纸面页码、PDF 页码与原厂 URL。不能把 SVD device-level reset0、字段缺省零或 SDK 初始化值当作所有寄存器的硬件复位值。

46 个共享 IP 模型共 520 个寄存器视图，499 个具有确定的共享完整复位字，21 个共享模型明确为 `null`。两个芯片分别引用306和223个视图（看门狗共享9个视图）。另在 family 的 `Peripheral.register_resets` 保存28条已核实的GPIO实例覆盖：L01212条、F03016条。这些实例值不复制或拆分相同的 GPIO 寄存器布局。

例如 L012 ADC.CR 的完整默认值为 `0x00000100`（RM25.12.1，纸面589/PDF615），包括未建字段的保留 bit8；不能仅合并已建模字段推导默认值。F030 ATIM.CR=`0x00600008`、SPI.CR1=`0x00001c04`、CRC.RESULT16=`0xffff` 同样保留完整字。

`null` 或省略 `reset_value` 的含义始终为未知/不适用，绝不是零。主要限制：

- 通用 GPIO 类型没有跨端口统一的 DIR/ANALOG/ICR/IDR 默认值；已核实实例通过 family 覆盖得到各自类型。例如 GPIOA.DIR=`0xffff`，GPIOC.DIR=`0xe000`，GPIOF.DIR=`0xcb`。
- L012 GPIOA/B ANALOG 与 ICR 的表格、复位状态描述及位访问说明互相冲突，保留表格原值和冲突证据，不为这四个实例制造 Default。
- HSI 的复位标题与 TRIM 描述冲突；LSI、RESETFLAG、GPIO ODR（含窄视图）缺少通用确定完整字。
- FLASH.CR1 的 SECURITY 来自持久化保护状态；L012 FLASH.SDKCFR 来自已烧写安全库索引，不能假定所有器件均处于未保护的出厂状态。
- IDR 表格值只是名义复位值，实际读取采样外部引脚。RTC/LSE 等默认值带有 POR/保留域限制；F030 RTC 不能套用 L012 的整块 POR-only 规则。

以上限制完整保存在 YAML、JSON、metadata 及 PAC 的 RESET_NOTE 中。Default 是初始化一个内存中的寄存器值，不是承诺当前芯片处于该状态，也不会复位外设。

## PAC API

每个寄存器生成自己的 `regs::Cr` 等值类型，即便字段布局相同也不共享寄存器默认值。可写字段有 `set_en(bool)` 等方法；枚举字段接受已建模枚举，读取保留枚举编码返回 `None`。`.bits()` / `.0` 为明确的原始字访问。

```rust
// L012：只构造内存值，不访问硬件。
let mut value = pac::adc::regs::Cr::default();
assert_eq!(value.bits(), 0x100);
value.set_en(true);
assert_eq!(value.bits(), 0x101);

// 必须先满足时钟、电源、所有权及具体寄存器约束。
unsafe {
    pac::ADC1.cr().write(|w| {
        w.set_clk(3);
        w.set_en(true);
    }); // 从0x100开始；无硬件读取，最后一次u32 volatile write

    pac::ADC1.cr().write_value(0x100); // 按原样写入，不补零、不OR reset
    pac::ADC1.cr().modify(|w| w.set_en(false)); // 普通RW：一次读取+一次写入
}
```

- `write(|w| ...)` 从该寄存器/实例的已审定 Default 开始，执行 typed setters，再写一次。仅完整默认字已知的 RW/WO 寄存器具备此方法。
- `write_value(word)` 明确写入原始 `u8/u16/u32`，不会 OR reset value。reset 为1的普通可写字段仍可被显式清零。
- `modify(|w| ...)` 只对 ordinary RW / ordinary read 开放；从一次真实读取开始，不从 Default 开始。不对WO、W1C/W0C、keyed、command或read-side-effect寄存器提供。
- 原有 mask式修改保留为 `modify_value(clear, set)`，同样仅普通RW可用。
- `.read()` 保留原始字返回类型；`.read_value()` 返回带getter的寄存器专属值。RO没有写方法，WO没有读方法。
- `RESET_VALUE` 是 `Option<word>`。未知值没有 `Default`，没有 closure write，但允许显式 `from_bits`、适用方向的 `write_value` 和 ordinary modify。
- family GPIO实例值通过 const参数专化寄存器类型，Default只针对审核过的具体值生成；不为任意 const参数或未知 sentinel 提供Default。正常使用 `pac::GPIOA.dir()` 无须书写这些参数。手动 `from_address` 必须匹配地址与实例参数。

所有硬件操作继续是 unsafe。复位字不是W0C/W1C、key、command的通用安全写值；调用者仍须显式选择清除掩码、写key并满足具体语义。closure write 不会偷读WO、盲目RMW或自动触发一个额外的复位操作。旧W0C `clear(mask)` 是全字 `!mask` 快捷写，仅可用于所有未选中位（含保留位）允许写1的寄存器；否则必须显式构造写值，不能从reset字机械推导通用清除语义。

HAL与六个例程的原始寄存器写入已逐处迁移到 `write_value`，保持原字值、顺序和ADC保留位读取策略。ADC结果句柄现在有不同类型，因此结果采集改为按索引match读取；仍仅按原顺序读取N个结果，没有先读取所有槽。

## Schema v4 与构建

`cw32-gen` 仍是唯一生成crate，内部 schema/data/pac 分工不变：YAML → **实际落盘的schema4 JSON** → 重新读取JSON → PAC/metadata/runtime。寄存器默认值属于register JSON；实例覆盖属于chip JSON中的family外设。metadata消费者按实例覆盖优先、共享register值次之解析。

校验拒绝超过8/16/32位访问字的reset、无来源的已知值、空来源/说明、重复或不存在的实例寄存器覆盖，以及普通或实例级alias复位字矛盾。旧schema JSON必须重新生成。没有新增生成crate、Python工程、例程测试或目标平台条件入口；生成JSON/PAC和构建产物不进入源码归档。

```sh
cargo run -p xtask -- regenerate
cargo run -p xtask -- regenerate --check
cargo fmt --all --check
```

v0.12.0修改了原始 `.write(word)`/`.modify(clear,set)` 名称；下游原始写应改为 `.write_value(word)`，原始mask修改改为 `.modify_value(clear,set)`。既有字段descriptor的 `.write(word,value)` 不变。
