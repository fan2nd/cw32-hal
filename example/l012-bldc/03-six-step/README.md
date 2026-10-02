# 03：六步换相的纯逻辑桥臂图

本 crate 自带采样专用板级模块，算法与 02 一致，只增加 `Bridge` 与 `SectorSelector`。所有模块都由本地普通 `mod` 声明加载。没有完整控制器、功率输出接口或换相计时器。

换相顺序：A+B−、A+C−、B+C−、B+A−、C+A−、C+B−。上电调试视图显示扇区 0 的 5% 逻辑占空比；按键连续按住 60 ms 后前进一个扇区，长按不重复触发，松开后才能再次切换。`Bridge::is_valid()` 检查范围和同相上下管互斥；非法扇区返回全关闭图。

`main.rs` 中的 IRQ 状态中的 `diagnostics` 字段 包含 02 的 `sampling`、`selected_sector`、`logical_bridge`，以及恒为 false 的 `outputs_armed`。逻辑图只供观察，不配置或写入六个物理桥臂引脚，不能据此断言其电平为低。

在本目录执行（继承板级 `.cargo/config.toml` 的 MCU target）：

```sh
cargo build --release
```

本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序，读取实际 ADC。没有 `motor-output-enable` feature。实验仍须物理断开母线或禁用 gate driver；不能用按键切换静态电流来代替真正的电机换相调度。
