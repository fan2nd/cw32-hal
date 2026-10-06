# 02：OPA 与 ADC 采样

本 crate 仅在 GPIO 基础上增加原板 OPA1/BGR、ADC1/ADC2、ATIM 内部 CH4 采样触发和 BTIM1 毫秒节拍。没有电机控制器、桥臂使能接口、UART 或换相计时器。

- 用户确认VDDA=5 V，CPU/PCLK恢复96 MHz；ADC1为48 MHz，ADC2为12 MHz。
- ADC1：原PWM1 OC4REFC触发，每槽70+15周期，4槽约7.083 µs；顺序为电流、A/B/C相电压。
- ADC2：保留ATIM和每5 ms软件双触发；每槽518+15周期，5槽约222.083 µs，逐槽EOC DMA搬运。双触发遇忙的实际采样率须实测。
- 六个桥臂引脚不配置、不写入，故障路径也不触碰它们；ATIM CH1–3 未连接，MOE 为 0。
- 按下 PA3 按键时点亮 PC13 LED。调试器观察 `main.rs` 中的 `ADC1_RAW`、`ADC2_RAW`、`ADC1_SEQUENCES` 和 `MILLISECONDS`。

在本目录执行（继承板级 `.cargo/config.toml` 的 MCU target）：

```sh
cargo build --release
```

本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序。先物理断开母线或禁用 gate driver；此例未实板验证，IRQ 延迟、OPA 建立及 ADC 顺序采样偏斜仍需测量。不再添加原工程没有的重叠/失联关采样策略。DMA确定性修复和ADC保留位依据见[一致性审查](../../../docs/bldc-source-parity.md)。

外设初始化直接写在 `main`，ADC 和毫秒节拍直接写在对应 ISR；仅 IRQ 使用的数组、计数器放在静态变量中。全部相关 IRQ 固定为同一 P1 优先级，不能互相抢占；主循环在休眠唤醒后检查毫秒数，每秒仅在短临界区内复制一次诊断快照，在临界区外输出日志，不写控制状态。

本级使用HAL独立 `motor` 操作API与真正的DMA通道驱动，不再直接操作PAC或启用 `unstable-pac`。板级参数、算法和ISR仍留在本crate；接口排他义务与顺序保证见[电机API](../../../docs/motor-api.md)。

本板实际电流采样电阻为 **10 mΩ**，OPA 差分增益为 10，灵敏度为 100 mV/A。ADC1/PB0 读取瞬时母线电流，ADC2/PB2 读取同一信号经 RC 滤波后的母线电流；本例只记录原码，没有安培换算或软件过流门限，因此不更改 ADC 原码。零点和后续换算依据见[板级电流说明](../README.md#电流采样与10-mω实板校准)。

## defmt RTT 日志

已接入非阻塞 `defmt-rtt`，启动时输出例程和 HAL 记录的标称时钟。每秒输出实际 ADC1 四槽、ADC2 五槽原始值，以及 ADC1 序列数和 ADC2 已观察到的 EOS 次数。ADC2 是原有实时 DMA 视图，可能包含尚未整组刷新完的值；EOS 次数不等于实际触发次数。

RTT 通过 SWD 输出，不使用 UART；构建已自动链接 `defmt.x`。查看方式见[板级 RTT 说明](../README.md)。ADC/定时器 ISR 不输出日志；快照复制和 RTT 编码仍有短暂关中断开销，不能将带日志版本当作硬实时延迟保证。调试器未读取或缓冲满时允许丢日志，程序不会等待 RTT 主机。原有 panic/异常安全处理保持不变，不在故障路径调用日志。
