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

本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序。没有 `motor-output-enable` feature。先物理断开母线或禁用 gate driver；此例未实板验证，IRQ 延迟、OPA 建立及 ADC 顺序采样偏斜仍需测量。不再添加原工程没有的重叠/失联关采样策略。DMA确定性修复和ADC保留位依据见[一致性审查](../../../docs/bldc-source-parity.md)。

外设初始化直接写在 `main`，ADC 和毫秒节拍直接写在对应 ISR；仅 IRQ 使用的数组、计数器放在静态变量中。全部相关 IRQ 固定为同一 P1 优先级，不能互相抢占；主循环解屏蔽后只休眠，不访问这些变量。
