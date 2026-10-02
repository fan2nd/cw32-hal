# v0.11.4 验证记录

2026-10-02，Rust/Cargo 1.99.0，目标 `thumbv6m-none-eabi`。

## 实现范围

- 01 保持只有 LED/按键的普通主循环。
- 02–06 删除逐资源 `Mutex<RefCell<Option<_>>>`、slot 解包与整个 ISR 的临界区。各级只保留自己的具体采样/控制结构，由同为 P1 的中断独占；没有新运行时、泛型状态容器或访问转发层。
- 主函数先构造完整状态，屏蔽 NVIC 时配置硬件事件与全部 P1 优先级，结束板 owner 借用，把状态一次移入静态存储，再经编译器屏障 unmask。NVIC unmask 函数没有 owner 参数，也不重新清除已产生的 pending 事件。
- 前台不再复制控制诊断状态。各 ISR 内用 `black_box` 保留诊断可观察性；只能在断开功率并暂停后检查一致数据，运行中调试器读并不是原子快照。
- 独立源码审查检查了初始化前后借用、P1 不互相抢占、异常仅直接关闭硬件且不返回、06 UI 与电机无运行期 GPIO RMW 冲突。新增更高优先级访问者或改变优先级必须重新审查。
- 05/06 删除 `BridgeIo`、`HardwareBridge` 和泛型 `apply_image`。`board.rs::apply_bridge` 直接使用 PAC，保留关桥、逻辑图校验、采样点换算、授权/刹车检查、占空比/引脚连接、MOE、两次再次检查、下桥输出的顺序。
- 只有 06 实际跨 Embassy 线程任务/IRQ 的小型 UI 邮箱保留 `Mutex<RefCell<UiLink>>` 和短临界区，承载命令/存活、LED/遥测和 waker；不持有电机外设或控制器，不跨 await 持锁。
- 02–05 删除直接 critical-section 依赖；cortex-m 的单核实现 feature 仍用于 HAL 初始化所需的内部临界区。
- 六级仍为各自本地普通模块、仅 binary、无 lib.rs/跨 crate path 模块/主机入口/例程测试脚手架。01–04 不接触桥臂引脚，05/06 默认关闭功率输出。

## 已通过

- YAML → 落盘 JSON → PAC/metadata 重新生成与 `regenerate --check` 逐字节漂移检查。
- `cargo fmt --all -- --check`。
- 六个板级 binary 的 ARM release 默认构建。
- 05/06 的 ARM release `motor-output-enable` 构建。
- L012C8、F030C8 HAL 的 ARM release 最小无默认 feature 配置，以及 `memory-x,metadata,unstable-pac,defmt,time-driver-any` 配置。
- source-only ZIP 解包，在无生成 JSON/PAC、Cargo.lock 和 target 的源码上重复上述全部生成、格式及 ARM 构建流程；归档文件与维护源码逐字节一致。

以上构建使用已有依赖缓存离线执行。第三方 proc-macro-error2 2.0.1 的未来兼容性警告仍存在。源码包不含生成文件、构建产物、Cargo.lock、缓存、日志、Python、原 SDK ZIP 或额外测试工程。

未进行实板、电气或实时性测试。静态审查与构建结果不证明门极时序、模拟建立、ADC 偏斜或中断最坏响应满足实际硬件；保留此前六步控制、采样时序与保护边界，详见[板级说明](../examples/l012-bldc/README.md)。
