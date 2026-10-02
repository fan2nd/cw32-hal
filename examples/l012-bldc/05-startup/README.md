# 05：定位、强制启动、过零接管与原保护

独立MCU binary，外设初始化、前台循环和ISR直接位于main.rs；控制/保护为本地普通模块。无lib、UART、邮箱、Embassy执行器或未调用PI。

电机前台连续运行，BTIM1仅做按键与计数，ADC1/BTIM3按原顺序进行过零/换相。原400 ms延时、150 ms定位、最多200个10 ms强制步、15次过零接管和30 ms/1%斜坡保留。阻塞段以续行状态表达，仍暂停原本无法执行的前台保护；不会将电机控制量化为1 ms任务。

共享控制器由前台短临界区和不嵌套P1 IRQ访问，没有Mutex/RefCell；等待期间不持有引用。初始化后main仍持有实际外设令牌。

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-05-startup
```

默认功率关闭。只有审核硬件后显式添加 `--features motor-output-enable` 才允许桥臂输出；逻辑上两种构建都保留6个bootstrap tick。没有实板验证或烧录。

原板VDDA=5 V，时钟恢复96 MHz HCLK/PCLK、20 kHz PWM（4800刻度）、8 MHz BTIM2/3；ADC1为48 MHz/70采样周期，ADC2为12 MHz/518周期、双触发和逐槽DMA。ADC2请求选择的确定性修复、ADC CR bit8的新版官方库依据及故障边界见[一致性审查](../../../docs/bldc-source-parity.md)。

原始ADC、逻辑桥图、状态、错误和实际输出授权可在断电halt时查看DIAGNOSTICS。正常错误处理时机保持原C；不再添加ADC失联故障或同事件立即关桥。原源码存在的状态覆盖和保护暂停均明示，不能把源码一致性当作电气安全认证。完整引脚和上板约束见[板级说明](../README.md)。
