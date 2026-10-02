# v0.11.3 验证记录

2026-10-02，Rust/Cargo 1.99.0，目标 `thumbv6m-none-eabi`。

本次仅将六个板级 binary crate 的模块加载改为自包含形式：移除 03/04/06 共 8 处跨 crate `#[path]` 声明，将必要的 8 个源文件复制到各自 `src/`，使用普通 `mod` 声明。复制后的生产算法与原模块逐字节一致；没有加入共享 crate、lib.rs、例程之间的依赖、测试文件或额外运行时封装。

已执行并通过：

- 源码检查：六例的 Rust 文件没有 `#[path]` 或 `include!` / `include_str!` / `include_bytes!`；没有主机入口、目标平台条件分支或测试脚手架。所有新增模块都在消费它的 crate 内。
- 8 个复制模块逐文件 `cmp` 一致；原有控制、保护、中断与所有权代码未改动。
- `cargo run -p xtask -- regenerate`，随后 `regenerate --check`：YAML → 落盘 JSON → PAC/metadata 逐字节一致。
- `cargo fmt --all -- --check`。
- 六个板级 binary 的 ARM release 默认构建。
- 05/06 显式启用 `motor-output-enable` 的 ARM release 构建。
- L012C8、F030C8 HAL 的 ARM release 构建：无默认 feature 的最小配置，以及 `memory-x,metadata,unstable-pac,defmt,time-driver-any` 配置。
- 最终 source-only ZIP 解包后，从无生成 JSON/PAC、Cargo.lock 和 target 的源码重新生成、检查格式与生成漂移，再完成全部六例默认和 05/06 输出授权构建；解包源码与维护树逐文件一致。

构建使用已有依赖缓存离线运行；第三方 `proc-macro-error2 2.0.1` 的未来兼容性警告不影响结果。源码包不含生成文件、构建产物、Cargo.lock、缓存、日志、Python 或原 SDK ZIP。

01–04 仍不配置、不持有、不写入桥臂引脚；05/06 默认禁止功率输出。06 仍使用真实 Embassy executor、独占电机 IRQ 域、独立 UI owner 与安全标志。未执行实板测试；编译结果不能证明电气安全或实时性，上板要求见[板级说明](../example/l012-bldc/README.md)。
