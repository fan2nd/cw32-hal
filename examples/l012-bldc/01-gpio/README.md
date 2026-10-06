# 01：仅 GPIO

本 crate 只使用 HAL 默认 4 MHz 时钟、PC13 低有效 LED 和 PA3 上拉低有效按键；不配置、不持有或写入六根电机门极控制线。没有电机 Board、ADC、OPA、PWM、定时器、UART、executor 或 PAC 裸寄存器操作，不依赖其他例程。

在本目录执行（继承板级 `.cargo/config.toml` 的 MCU target）：

```sh
cargo build --release
```

第一次使用整个源码仓库时，先在根目录执行 `cargo run -p xtask -- regenerate`。本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序。没有桥臂解锁路径；实验先断开母线。更多引脚及上板限制见[板级说明](../README.md)。

## defmt RTT 日志

已接入非阻塞 `defmt-rtt`：启动时输出例程、HAL 记录的标称时钟和 PA3 初始状态；随后每 100000 次主循环最多检查一次按键变化并输出按键/LED 状态。该分频仅限制日志，不是定时器或去抖，短按可能不出现在日志中，LED 仍立即跟随按键。

RTT 通过 SWD 输出，不使用 UART；构建已自动链接 `defmt.x`。查看方式见[板级 RTT 说明](../README.md)。调试器未读取或缓冲满时允许丢日志，程序不会等待 RTT 主机。原有 panic 处理保持不变，不在故障路径调用日志。
