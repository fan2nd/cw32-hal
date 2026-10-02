# v0.11.5 验证记录

2026-10-02，Rust/Cargo 1.99.0，目标 `thumbv6m-none-eabi`。
本记录覆盖结构简化后的ADC、电机、故障行为回归；此前仅简化结构的未交付归档不能代表本次结果。

## 当前改动

- 六级独立binary，初始化/ISR直接位于main.rs，无Board/State/Ui硬件套壳、lib、例程间源码导入、例程测试或平台条件入口。
- 原板VDDA=5 V，恢复96 MHz PCLK、ADC1 48 MHz/70采样周期、ADC2 12 MHz/518周期、双触发、4800 PWM刻度和PWM1、8 MHz重复BTIM。
- ADC2 DMA采用原EOC/逐字BLOCK意图，明确初始化EOS=false并修正请求源为ADC2_SINGLE；保留五槽逐次更新，非一致快照。
- ADC CR bit8依据用户补充的官方V1.0.5库保留默认值，新库与固定vendor来源一致。
- 电机前台连续运行，ISR仅承担原职责；保护暂停、状态先后、静态计数、浮点换算及故障覆盖边界恢复。05无执行器；06仅UI使用低优先级官方InterruptExecutor，电机不由Embassy调度。
- 前台在有限短临界区内完成借用；P1电机IRQ同优先级不嵌套。06仅UI值邮箱使用必要Mutex/RefCell，无跨await电机引用。
- 01–04不访问桥臂引脚；05/06默认禁止实际功率。默认与opt-in均保留6个逻辑bootstrap tick。

## 已执行

- 当前工作源码 `cargo fmt --all --check`。
- `cargo run -p xtask -- regenerate --check` 生成漂移检查。
- 六个例程 `thumbv6m-none-eabi` 默认release构建。
- 05/06 `motor-output-enable` release构建。
- 树外原 `compu.c` 对Rust保护：200,000个顺序输入，电流、电压、错误码轨迹全部一致；新增仅SampleVI的入口后复核仍一致。
- 树外原 `MOTOR.C`/`sensorless.c` 对Rust：56,762个事件转换，状态、扇区、滤波、错误、计数、CCR/低侧桥图、定时器状态一致。
- 树外Rust启动时钟模型：400 ms延时、150 ms定位、首扇区2、200×10 ms超时、每步PWM+5、原失败后RUNOPEN覆盖/下一轮派错、定位采样、15次过零接管均通过。
- 独立源码审查：ADC/时钟/触发/DMA源与模式、PWM预装载、BTIM重复模式、前台/ISR共享、bootstrap、换相/UPPWM/停止写序和新旧SDK的保留位依据。

树外验证程序未加入仓库；使用原输入源码作对照但不随发布包分发。上述宿主模型不是真实MMIO/总线/IRQ时序模拟，不证明与C逐指令相同。

本次源码归档已从冻结后的当前代码重新生成并独立解包，使用独立空target目录完成
YAML→JSON→PAC生成、漂移/格式检查、六级默认及05/06 opt-in ARM release构建，全部通过。
旧结构版归档未复用。未上板、未烧录，未验证门极波形、模拟建立、DMA竞争、最坏IRQ延迟或电气安全。源码包不应包含生成JSON/PAC、target、Cargo.lock、缓存、日志或私有验证工程。

逐项源码依据、原边界和确定性修正见[一致性审查](bldc-source-parity.md)，使用与硬件限制见[板级说明](../examples/l012-bldc/README.md)。
