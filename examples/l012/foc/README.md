# CW32L012 FOC 分级例程

十个独立工程使用同一固件目标 `cw32l012c8`。在各自目录内运行 `cargo build --release --locked` 构建，或运行 `cargo run --release --locked` 上板并查看 RTT。

| 例程 | 操作与观察 |
| --- | --- |
| [01 开环标定](01-open-loop/) | 空载归零解锁。六点定位、正反扫场，辨识极对数、方向和零偏，看到 `01 FLASH save OK` 表示写入并回读成功。 |
| [02 电流采样与 Iq 闭环](02-current-loop/) | 启动时关桥，每 100 ms 打印两相 ADC 原始值、零偏与电流。标定和零偏有效后，电位器先归零再抬高，给定 0–200 ADC 计数的 Iq。空载达到 240 rpm 时撤去给定。 |
| [03 速度闭环](03-speed-loop/) | 电位器给定 0–600 rpm，比较目标与反馈，观察 Iq 对负载变化的响应。 |
| [04 位置跟踪与定点保持](04-position-loop/) | 电位器有效行程线性对应相对标定零位的 0–360° 机械角，中点约 180°，满量程与零位重合。电位器固定时，到位后关桥，受扰超出回差后自动回位。位置环按最短方向跟随，上限为 ±120 rpm。剩余误差小于约 21° 时，速度给定随误差线性降低。 |
| [05 五段式 SVPWM](05-five-segment/) | 与 03 相同的速度给定，示波器观察三相中最低占空比相的上管钳位。 |
| [06 高频注入无感速度闭环](06-sensorless/) | 静止辨识磁轴与极性，运行中用高频电流响应估算角度和速度，RTT 仅读取并记录 MT6701 原始机械角。 |
| [07 混合无感速度闭环](07-hybrid/) | 低速用 HFI 换相，高速转由反电动势观测器接管并关闭注入。 |
| [08 无感弱磁](08-field-weakening/) | 高速 EMF 模式按电压利用率调节负 Id，观察 Id/Iq 电流预算。 |
| [09 顺逆风启动](09-windmill/) | 零电流探测带速转子，顺风同步接管，逆风限流制动后从低速重新启动。 |
| [10 编码器快速往返](10-reversal/) | 上电自动起转，在实测 +600 与 −600 rpm 各恒速运行 1 秒，再直接切换转矩，逐次打印过零和换向耗时。 |

02–05 使用 01 写入的标定页 `0xFC00..0xFE00`。按扇区下载可保留记录；整片擦除、更换电机、调整编码器或相线后须重新标定。02 缺少有效标定时仍可观察电流采样。

03 和 05 的速度环以 2 kHz 运行，04 的位置和速度环以 1 kHz 运行，均在 SysTick 中断中执行。04 在同一次中断内先算位置误差，再算速度与转矩给定。16 kHz ADC1 电流中断优先级更高。主循环可独立处理应用任务。

[板级资源分配](common/board_resources.rs) 集中列出引脚与外设。各例程维护自己的硬件初始化、中断和控制流程。[纯软件算法](foc-algorithms/) 提供三角查表、Clarke/Park、PI、SVPWM 与标定换算。[测时程序](algorithm-timing/) 单独运行。

## 参考板接线

The six PWM routes match the supplied board:

| Function | Pin |
| --- | --- |
| ATIM CH1/CH1N | PB5 / PA15 |
| ATIM CH2/CH2N | PB6 / PB3 |
| ATIM CH3/CH3N | PB7 / PB4 |
| ADC1 current A/B | PB0 / PB1 |
| ADC2 bus voltage / speed pot | PA8 / PA10 |
| MT6701 SSI SCK/MISO/CS | PB8 / PC14 / PF0 |
| Status LED (active-low) | PC13 |

The motor constants are taken from `JD-4621.jpg`: 22 poles (11 pole pairs),
6.7 Ω phase resistance, 1.75 mH phase inductance, 12 V nominal bus and 1.1 A
rated current. The C board uses 5 V ADC reference, a 1:10 bus divider, and
OPA1/OPA2 external current amplifiers from PA6/PA7 and PA4/PA5 into PB0/PB1.
The image names a 5048A encoder; the supplied C project uses a 14-bit MT6701
SSI frame, so the example keeps the encoder electrical protocol in the board
layer and uses the 14-bit frame currently present in the project.
