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

## Current PAC API

v0.13 uses typed `read() -> regs::T` and `write_value(regs::T)`, true indexed register/subblock access, and accurate element-specific initialization. The legacy raw API description is superseded by [current PAC/GPIO usage](pac-gpio-v0.13.md). Array grouping preserves the520 original physical register views and all audited element reset evidence; it does not manufacture defaults.

## Schema v7 与构建

`cw32-gen` 仍是唯一生成crate，内部 schema/data/pac 分工不变：YAML → **实际落盘的schema9 JSON** → 重新读取JSON → PAC/metadata/runtime。寄存器默认值属于register JSON；实例覆盖属于chip JSON中的family外设。metadata消费者按实例覆盖优先、共享register值次之解析。

校验拒绝超过8/16/32位访问字的reset、无来源的已知值、空来源/说明、重复或不存在的实例寄存器覆盖，以及普通或实例级alias复位字矛盾。旧schema JSON必须重新生成。没有新增生成crate、Python工程、例程测试或目标平台条件入口；生成JSON/PAC和构建产物不进入源码归档。

```sh
cargo run -p xtask -- regenerate
cargo run -p xtask -- regenerate --check
cargo fmt --all --check
```

当前API迁移以[pac-gpio-v0.13.md](pac-gpio-v0.13.md)为准。
