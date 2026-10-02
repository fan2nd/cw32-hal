# 05：保护、定位、强制启动与过零接管

本例只有 main.rs 二进制入口，无 lib.rs。它是独立 crate `cw32-bldc-05-startup`。新增完整电机状态机：400 ms 延时、150 ms 定位、强制换相、连续 15 次过零接管、占空比斜坡，以及实际 ADC1/ADC2/BTIM1/BTIM3 中断。没有 Embassy executor、UART、协议、邮箱或未使用的 PID，也没有通用 `Stage` 分支。

保留按键停机、故障灯、电压/电流/温度和堵转保护、ADC 新鲜度检查；有功率输出的例程不能为了精简去掉这些保护。所有寄存器操作与资源都在本 crate 的 `board.rs` 中，初始化与中断流程直接位于 `main.rs`。06 自带相同的纯算法源码，不引用本 crate 的源文件或硬件运行时。

从 workspace 根目录运行：

```sh
cargo build -p cw32-bldc-05-startup --release --target thumbv6m-none-eabi
```

默认功率输出关闭。本例直接构建为 MCU 程序。在审核原理图、电源、MOSFET、保护和时序后，显式 `--features motor-output-enable` 才允许输出。此时先执行原例程 6 ms 低桥 bootstrap，再关闭桥臂，按键才可开机和选择速度；这不是实板验证或硬件安全认证。

本步骤只启用所需 GPIO、ATIM、OPA1/BGR、ADC1/2、BTIM1/2/3；PB11/PB12 和 UART1 不初始化、不持有，UART1 时钟不打开。PC13 LED 与 PA3 按键直接由控制中断域持有，无异步执行器。

时序保持保守适配：96 MHz HCLK、48 MHz PCLK、20 kHz ATIM、6 MHz ADC、8 MHz 步进计时。ADC1 四槽 22 µs，ADC2 五槽约 444.167 µs、每 5 ms 启动。原 4800 刻度 PWM 在板边界除以 2；退磁和延迟仍用原来的步长右移 3。控制器状态、原始 ADC、逻辑桥图和授权状态可通过 `main.rs` 中的 IRQ 状态中的 `diagnostics` 字段 查看。

软件会立即关闭故障桥图，采样重叠使硬件永久撤销本次运行的输出授权，异常处理直接紧急关断。仍须实测最坏中断延迟、低桥 GPIO 的开关间隔、模拟建立时间、BEMF 窗口和外部硬件关断；不要在功率接通时随意 halt 调试器。

完整引脚、原程序差异、限制与上板要求见[板级说明](../README.md)。
