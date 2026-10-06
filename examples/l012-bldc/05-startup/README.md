# 05：定位、强制启动、过零接管与原保护

独立MCU binary，外设初始化、前台循环和ISR直接位于main.rs；控制/保护为本地普通模块。无lib、UART、邮箱、Embassy执行器或未调用PI。

电机前台连续运行，BTIM1仅做按键与计数，ADC1/BTIM3按原顺序进行过零/换相。原400 ms延时、150 ms定位、最多200个10 ms强制步、15次过零接管和30 ms/1%斜坡保留。阻塞段以续行状态表达，仍暂停原本无法执行的前台保护；不会将电机控制量化为1 ms任务。

共享控制器由前台短临界区和不嵌套P1 IRQ访问，没有Mutex/RefCell；等待期间不持有引用。初始化后main仍持有实际外设令牌。

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-05-startup
```

不再提供例程 feature 开关，普通构建包含实际功率输出。上电先执行原有 6 ms 三个低桥 bootstrap 充电，随后关闭桥臂等待按键启动；原故障处理与 panic/异常紧急关断保留。烧录前须核对硬件并物理禁用驱动；没有实板验证或烧录。

原板VDDA=5 V，时钟恢复96 MHz HCLK/PCLK、20 kHz PWM（4800刻度）、8 MHz BTIM2/3；ADC1为48 MHz/70采样周期，ADC2为12 MHz/518周期、双触发和逐槽DMA。ADC2请求选择的确定性修复、ADC CR bit8的新版官方库依据及故障边界见[一致性审查](../../../docs/bldc-source-parity.md)。

原始ADC、逻辑桥图、状态、错误和实际输出授权可在断电halt时查看DIAGNOSTICS。正常错误处理时机保持原C；不再添加ADC失联故障或同事件立即关桥。原源码存在的状态覆盖和保护暂停均明示，不能把源码一致性当作电气安全认证。完整引脚和上板约束见[板级说明](../README.md)。

本级使用HAL独立 `motor` 操作API与真正的DMA通道驱动，不再直接操作PAC或启用 `unstable-pac`。板级参数、算法和ISR仍留在本crate；接口排他义务与顺序保证见[电机API](../../../docs/motor-api.md)。

实际电流采样电阻为 **10 mΩ**，OPA 差分增益为 10，灵敏度为 100 mV/A。`protection.rs` 将 ADC2 去零点后的电压（mV）乘 10 得到 mA，原系数已匹配实板，无须再缩小五倍。3 A 持续 30 次和 10 A 当次检查过流门限保持原值；它们使用 PB2 的滤波母线电流，不能当作 ADC1 瞬时硬件过流或相电流 RMS。零点采集、RC 滤波和日志单位见[板级电流说明](../README.md#电流采样与10-mω实板校准)。

## RTT 调试

本例默认包含 defmt RTT 日志。在本目录运行 `cargo run --release`，由 probe-rs 显示启动时钟/输出状态、启动阶段和故障；周期诊断为 500 ms，状态变化另行报告。检查 `armed`、`steps`、`crossings`、ADC 原码及 `last_protection_bus` 可定位三闪 `StartupFailed`。普通构建可驱动电机：上电先执行 6 ms 低桥充电，随后关闭桥臂等待按键启动；烧录前须物理禁用驱动。

日志只在普通线程中发出，保留原 panic/异常关闭路径；主机断开不等待，但可能丢日志。ADC2 不是原子 EOS 快照，保护电压在启动等待期间可能保持上次结果，日志仍有短暂关中断的时序代价。命令、单位与全部限制见[统一 RTT 说明](../README.md#defmt-rtt-日志01–06)。
