# 按开发板组织的递进例程

- [`l012-bldc/`](l012-bldc/README.md)：上传工程对应的 CW32L012 无感六步 BLDC 板，`01-gpio/` 至 `06-application/` 是六个独立 Cargo crate。每一级仅引入本级必要模块、依赖和硬件初始化，使用本级 `src/main.rs` 入口；阶段说明集中在板级 README。另有独立的 [`07-sensorless-foc`](l012-bldc/07-sensorless-foc/README.md)，实现单电阻无感 FOC；普通构建按键启动，仅在启动前及故障停机后输出 RTT，须先整定和台架验证。

每块板独立维护引脚、电气/时钟假设和运行说明；不要把一种板的功率输出例程直接烧录到另一种板。
