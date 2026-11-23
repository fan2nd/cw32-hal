# 速度闭环

电位器控制 0–800 rpm 的速度给定。SysTick 每 500 µs 直接读取 SSI、执行速度 PI 并发布 Iq。更高优先级的 ADC1 中断以 16 kHz 执行电流环。主循环负责 LED、RTT 和其他应用任务。固定 HSI 时钟配置通过 HAL 的 `init_hsi` 初始化。运行前先完成[前两个工程的调试](../README.md)。

```sh
cargo run --release --locked --offline
```

板测现象、修正和复测结果见[速度环踩坑记录](踩坑.md)。
