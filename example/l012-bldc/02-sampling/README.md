# 02：OPA 与 ADC 采样

本 crate 仅在 GPIO 基础上增加原板 OPA1/BGR、ADC1/ADC2、ATIM 内部 CH4 采样触发和 BTIM1 毫秒节拍。没有电机控制器、桥臂使能接口、UART 或换相计时器。

- CPU 96 MHz，PCLK 48 MHz，ADC 6 MHz。
- ADC1：20 kHz 触发，4 槽共 22 µs；顺序为电流、A/B/C 相电压。
- ADC2：启动时采一次，之后每 5 ms 软件启动；5 槽约 444.167 µs，不重启进行中的转换或覆盖尚未读取的序列。
- 六个桥臂引脚不配置、不持有、不写入，故障路径也不触碰它们；ATIM CH1–3 未连接，MOE 为 0。
- 按下 PA3 按键时点亮 PC13 LED。调试器观察 `main.rs` 中的 IRQ 状态中的 `diagnostics` 字段 的原始数组、序列计数、毫秒数和采样错误标志。

在本目录执行（继承板级 `.cargo/config.toml` 的 MCU target）：

```sh
cargo build --release
```

本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序。没有 `motor-output-enable` feature。先物理断开母线或禁用 gate driver；此例未实板验证，IRQ 延迟、OPA 建立及 ADC 顺序采样偏斜仍需测量。检测到 ADC1 序列重叠时锁存错误并停掉该采样流，不自动恢复。
