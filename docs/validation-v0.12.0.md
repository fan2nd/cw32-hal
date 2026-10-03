# v0.12.0 验证记录

2026-10-02，Rust/Cargo1.99.0，MCU目标 `thumbv6m-none-eabi`。

## 变更范围

- 46个共享寄存器模型、520个视图新增完整reset字/未知状态及逐条原厂来源；499个共享值已知、21个共享值为null，另有28条GPIO实例值覆盖。
- schema4经真实落盘JSON传递reset与实例覆盖。寄存器专属typed value、审定Default、closure write、write_value和ordinary-only closure modify/modify_value由生成器产生。
- 554处寄存器原始写改名为write_value；与v0.11.6源树去空白及机械逆替换比较，除ADC结果句柄数组改为按N个索引match读取外，HAL/例程无其他逻辑差异。原始指针 `.write` 与字段descriptor `.write(word,value)` 未改。
- 包版本统一0.12.0；单cw32-gen crate、IP版本目录、真实JSON阶段、HAL variant目录及六个独立binary结构不变。

## 已执行

- `cargo fmt --all --check`。
- `cargo run -p xtask -- regenerate` 后 `regenerate --check`，全芯片JSON/PAC/metadata/runtime文件集合及字节一致。
- L012/F030各自最小no-default配置，以及 `memory-x,metadata,unstable-pac,defmt,time-driver-any` 完整配置ARM release。
- 六个例程默认ARM release；05/06 `motor-output-enable` ARM release。
- 外部宿主MMIO替身：ADC.CR从0x100初始化，closure修改正确；write_value不自动OR默认字，closure能够有意清除reset-one字段；ordinary modify从实际旧字读取。F0308/16位closure/raw写只改变对应字节，不用u32读写伪装窄访问。
- 对实际生成的全部已知实例Default逐项编译及运行断言：L012565条、F030399条（同一共享寄存器在各实际外设实例分别核对）。
- 编译拒绝：未知Default/unknown closure write、W0C modify、WO read、RO write_value、bool setter误传整数。独立审查另外核对L012 GPIOA.ANALOG的冲突未知默认拒绝closure write。
- 外部JSON变异检查：拒绝8位reset越界、已知值缺来源、普通alias不一致、alias实例覆盖/其canonical实例覆盖不一致、实例宽度越界、不存在的覆盖目标、旧schema3。
- 只复制normalized JSON供stage2：改变ADC默认字，生成Default随JSON改变；删除reset_value后不再生成Default。证明PAC不偷读YAML，也不将省略值补零。
- 独立数据复审检查ADC bit8、GPIO实例/矛盾、Flash安全持久状态、POR域、DMA、CRC别名及timer；矛盾或缺少确定字的项目保留null和来源说明。

上述宿主探针及变异程序位于发布树外，没有新增测试工程、Python工程或额外target配置文件。源码归档排除生成JSON/PAC、target、Cargo.lock、日志、下载缓存和私有探针。

## 限制

未上板或烧录；编译、内存替身与资料核对不能证明真实MMIO、副作用、复位域、电机时序或门极波形。Default不是万能安全写字：W0C/W1C/keyed/command仍由调用者满足硬件语义。原有W0C `clear(mask)` 为原始全字 `!mask` 快捷操作，仅适用于所有未选中位可写1的寄存器；本版没有把reset字错误地当作所有W0C寄存器的中性写值。

已知外部依赖 `proc-macro-error2 2.0.1` 仍显示Cargo未来不兼容提示，当前工具链构建通过。
