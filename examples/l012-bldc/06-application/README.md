# 06：连续电机前台与 Embassy UI

独立MCU binary，含与05一致的本地控制/保护算法。main.rs直接初始化外设并连续运行电机前台；ADC1/BTIM1/BTIM3保留原中断职责。电机不由Embassy任务或1 ms调度器驱动。

真正的Embassy `InterruptExecutor`使用专用UART2向量的低优先级软件中断，UART2外设保持未使用。UI任务操作PA3、PC13、UART1，通过有界值邮箱交换按键、LED和遥测，不持有电机引用。UART每次唤醒最多尝试发送一字节；UI处理方式允许不同，但不增加原工程没有的UI失联故障。

前台短临界区与不嵌套P1 IRQ保护控制器的可变借用；无跨await引用。没有Board/Ui硬件包装、lib target或跨例程源码依赖。

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-06-application
```

默认功率关闭，`motor-output-enable`为审核硬件后的显式选择。两种构建保留同样的6个逻辑bootstrap tick；只有授权构建执行实际低桥充电。未做实板测试。

原板VDDA=5 V，恢复96 MHz HCLK/PCLK、20 kHz PWM/4800刻度、8 MHz BTIM2/3、ADC1 48 MHz/70周期与ADC2 12 MHz/518周期。原前台保护暂停、计数器保留、故障派发顺序和启动状态边界均保留。ADC2 DMA的最小确定性修复与ADC CR bit8的新版官方库依据见[一致性审查](../../../docs/bldc-source-parity.md)，引脚和电气限制见[板级说明](../README.md)。
