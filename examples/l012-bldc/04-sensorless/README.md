# 04：被动过零观察

本 crate 的直接采样和六步逻辑图与 03 一致，只增加 `ZeroCrossingDetector`；所有模块都由本地普通 `mod` 声明加载。悬空相依次为 C/B/A/C/B/A，下降/上升方向交替；以母线 ADC 值一半为门限，连续两次严格越过门限后报告一次，随后等待选择新扇区。

`main.rs` 在 03 的变量基础上增加 `DETECTOR` 和 `OBSERVED_CROSSINGS`，没有 Board 或 State 包装。每次报告时 LED 亮，下一次 60 ms 有效按键选择扇区时重新布置检测器并关灯。阈值比较保留原C行为，不添加零母线或采样新鲜度门控；不满足当前边沿条件的样本会清连续计数。

这仍是被动检测，不配置或写入六个门极引脚，不能据此断言其电平为低。它不启动电机、不调用完整控制器、不调度退磁/延迟换相。静态阈值资格满足也不能证明已测得真实转子过零；退磁消隐与延迟换相在后续主动启动例程中处理。

在本目录执行（继承板级 `.cargo/config.toml` 的 MCU target）：

```sh
cargo build --release
```

本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序，观察实际 ADC。没有 `motor-output-enable` feature。实验先物理断开母线或禁用 gate driver；外部拖动测 BEMF 也需要先确认电气条件。ADC/OPA 模拟特性和 IRQ 最坏延迟尚未实板验证。

外设初始化直接写在 `main`，ADC 和毫秒节拍直接写在对应 ISR；仅 IRQ 使用的数组、计数器放在静态变量中。全部相关 IRQ 固定为同一 P1 优先级，不能互相抢占；主循环解屏蔽后只休眠，不访问这些变量。

本级使用HAL独立 `motor` 操作API与真正的DMA通道驱动，不再直接操作PAC或启用 `unstable-pac`。板级参数、算法和ISR仍留在本crate；接口排他义务与顺序保证见[电机API](../../../docs/motor-api.md)。
