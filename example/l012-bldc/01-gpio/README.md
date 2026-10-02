# 01：仅 GPIO

本 crate 只使用 HAL 默认 4 MHz 时钟、PC13 低有效 LED 和 PA3 上拉低有效按键；不配置、不持有或写入六根电机门极控制线。没有电机 Board、ADC、OPA、PWM、定时器、UART、executor 或 PAC 裸寄存器操作，不依赖其他例程。

在本目录执行（继承板级 `.cargo/config.toml` 的 MCU target）：

```sh
cargo build --release
```

第一次使用整个源码仓库时，先在根目录执行 `cargo run -p xtask -- regenerate`。本例仅有 main.rs 二进制入口，无 lib.rs，直接构建为 MCU 程序。没有 `motor-output-enable` feature，没有桥臂解锁路径；实验先断开母线。更多引脚及上板限制见[板级说明](../README.md)。
