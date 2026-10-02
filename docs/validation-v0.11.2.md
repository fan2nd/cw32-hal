# v0.11.2 验证记录

> 历史记录。当前例程的自包含模块结构与验证见 [v0.11.3](validation-v0.11.3.md)。

2026-10-02，Rust/Cargo 1.99.0，目标 `thumbv6m-none-eabi`。

本版保留根目录 `example/l012-bldc/01–06` 六个单 binary crate。初始化、主循环与中断流程直接位于各自 `main.rs`；移除自定义 Runtime、转发式应用封装、库内 examples 和测试脚手架。硬件数据校验仍由生成器执行。

已执行并通过：

- `cargo run -p xtask -- regenerate`，随后 `regenerate --check`：YAML → 落盘 JSON → PAC/metadata 生成结果逐字节一致。
- `cargo fmt --all -- --check`。
- L012C8、F030C8 HAL 的 ARM release 构建：各自无默认 feature 的最小配置，以及 `memory-x,metadata,unstable-pac,defmt,time-driver-any` 配置。
- 六个板级 binary 的 ARM release 构建。
- 05/06 显式启用 `motor-output-enable` 的 ARM release 构建。

所有构建使用现有依赖缓存离线运行；包含 defmt 的构建提示第三方 `proc-macro-error2 2.0.1` 未来兼容性警告，不影响本次构建。

没有运行硬件，也没有宣称中断延迟、电气安全或电机行为经过实板验证。01–04 不配置、不持有、不写入桥臂引脚；05/06 默认禁止功率输出。输出授权、中断临界区和 06 的真实 Embassy executor 保留；上板前仍须完成[板级说明](../example/l012-bldc/README.md)中的供电、保护与时序检查。
