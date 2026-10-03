# 01 → 06：六个独立、递增的 BLDC crate

输入是 `10 XUNLIANYING 260726 LAST.zip` 的 `BLDC CONTROL`：反电动势过零检测的六步换相，不是FOC。ADC、电机和故障处理按原工程恢复；UART、LED、按键的组织方式可以不同。逐项依据、原有边界和明确例外见[原工程一致性审查](../../docs/bldc-source-parity.md)。

**尚未实板验证。01–04不配置或写入六个桥臂脚；05/06默认禁止功率输出。** 这不能代替物理禁用驱动器。编译和宿主轨迹通过均不代表可以直接给电机上电。

## 六个阶段

- `01-gpio`：默认4 MHz，只配置PC13 LED和PA3按键。
- `02-sampling`：原时钟、OPA/BGR、ADC1/2、ADC2 DMA、ATIM内部触发及BTIM1节拍。
- `03-six-step`：增加六步桥图和按键选择扇区，只观察，不输出功率。
- `04-sensorless`：增加原浮相/边沿/阈值的被动过零观察，不自动换相。
- `05-startup`：完整启动、换相、保护；电机前台连续运行，ADC1和BTIM中断独立，无UART或Embassy执行器。
- `06-application`：同样的控制/保护算法；高优先级InterruptExecutor运行事件驱动电机任务，普通线程executor运行按键/LED/UART任务。

每级都有自己的 `Cargo.toml`、`src/main.rs` 和 `build.rs`，只有一个binary，无lib target、跨例程源文件导入或共享业务crate。01–05的初始化、主循环和ISR直接位于各自main；06按实际职责分为main时钟与所有权拆分、motor板级配置/电机任务/ISR、ui按键/LED/UART与软件命令/状态交换，控制/保护/协议/帧队列是本地独立逻辑模块。没有Board、State、Ui硬件包装或未调用的PI。寄存器操作已收敛为独立HAL `motor` 文件夹中的ADC扫描、PWM换相、定时器与模拟前端操作；DMA使用独立通道驱动。02–05不再依赖PAC，06只剩UI串口寄存器。unsafe排他义务与逐项迁移见[电机API](../../docs/motor-api.md)。

## 构建

从干净源码先在workspace根执行 `cargo run -p xtask -- regenerate`。例程不额外添加目标配置文件，构建时显式指定MCU目标：

```sh
cd examples/l012-bldc/01-gpio
cargo build --offline --release --target thumbv6m-none-eabi
```

也可从workspace根选择包：

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-01-gpio
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-02-sampling
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-03-six-step
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-04-sensorless
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-05-startup
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-06-application
```

只有05/06提供 `motor-output-enable`。它改变实际功率引脚授权，不改变控制器的逻辑状态或六个bootstrap计时tick。确认原理图、供电、门极极性、死区及保护后才可选择：

```sh
cargo build --release --target thumbv6m-none-eabi -p cw32-bldc-05-startup -p cw32-bldc-06-application --features motor-output-enable
```

这些是MCU程序，无宿主入口/平台条件分支；源码包不包含例程测试或外部生成脚本。没有配置通用烧录runner，也没有实际烧录。

## 时钟与ADC

用户确认原板VDDA=5 V。本例恢复原HSI/HCLK/PCLK=96 MHz，ATIM PSC=0、ARR=4799，20 kHz；所有PWM比较值直接用原4800刻度，不除2。BTIM1为1 ms，BTIM2/3为8 MHz。

ADC1通道 `[8,0,1,2]`，48 MHz，每槽70+15周期，四槽约7.083 µs。ADC2通道 `[11,5,7,8,Vref]`，12 MHz，每槽518+15周期，五槽约222.083 µs。保留ADC2的ATIM及每5 ms软件双触发；忙转换期间触发的实际行为和采样有效率须实测。

CH4恢复原PWM1。官方RM §25.12.7指定触发为OC4REFC上升沿，不能把SDK的ATIMCC4名称直接解释成在CCR4比较位置采样。ADC2使用逐槽EOC的DMA更新；这不是EOS一致快照。原DMA请求选择与未初始化字段的最小确定性修复，以及ADC CR bit8采用新版官方库的依据，见[详细审查](../../docs/bldc-source-parity.md)。

## 电机执行与共享状态

05电机前台连续运行；06由真实ADC/定时器事件唤醒P1 InterruptExecutor中的电机任务，连续推进已就绪工作，不以1 ms轮询量化电机控制。BTIM1仅做原按键/ADC2启动/计数工作；100 ms电压电流温度检查在前台。原阻塞启动、停机和故障等待用显式前台续行状态表示：ISR仍运行，但原先被阻塞的前台保护不会额外执行。

05的连续前台在短临界区内借用控制器；06将电机任务放入UART2软件中断上的P1 InterruptExecutor，与ADC1/BTIM1/BTIM3的P1硬件ISR同级，ARMv6-M同级异常不能相互抢占。控制器不跨await借用，等待路径真正返回Pending，只有有限的立即可执行延续会连续推进。ADC逐样本处理与定时换相仍在ISR立即执行，唤醒只合并重复检查请求。普通线程executor仅有UI，少量命令/状态/waker通过短临界区同步，不持有电机引用。05无执行器，作为直接ISR/前台教学阶梯保留；06展示完整任务分层。改变IRQ优先级、新增访问者或允许异常返回都需要重新审查共享安全。

06使用官方 `InterruptExecutor`，专用未启用UART2外设的UART2向量作P1电机软件唤醒；普通线程UI任务处理PA3/PC13/UART1。UI不能持有控制器引用或电机寄存器指针。UART非阻塞，每次UI唤醒最多发送一字节。UI失联不再增加原工程没有的电机故障码。

## 原控制行为

- 六步顺序：A+B-、A+C-、B+C-、B+A-、C+A-、C+B-。
- 400 ms启动延时；`QDPwm=15*105/CanshuV`；150 ms定位；首强制扇区2。
- 最多200个强制步，每步最多10 ms、PWM增加5；连续15个有效过零后接管。
- 浮相 `[C,B,A,C,B,A]`、下降/上升交替、母线ADC右移1门限、严格大于或小于。
- 退磁/延迟为 `StepTime >> 3`；2000 tick分界；低于100 tick写过快故障。
- BTIM2 ARR=65530会在约8.191375 ms回卷，仍保留原始计数。
- 稳态每30 ms增加1%占空比，减速直接降到指定百分比；显示速度是100 ms换相数乘100，不是机械RPM。
- 原 `RealS1`未被更新，停机实际进入500 ms等待。

电流/电压换算保留C的f32语句顺序。3 A持续30次、10 A即时、14 V持续30次、欠压 `<6.6 V` 或 `9.5 V<V<10.0 V`持续30次；NTC原码50..342累积25次，小于50保留计数，大于342清零。原5000 ms堵转检查和故障先后顺序保留。原 `HardFAULTAD=403`、`NUMtimes=2` 未被代码使用，不宣称已有硬件瞬时过流。

原延时取消后仍可能进入启动、启动失败后状态暂时被覆盖、故障需到原前台位置才关桥等边界均保留并列明。没有继续保留移植时擅自添加的ADC/UI失联故障或故障即时关桥策略。零除法/非法浮点转整数的C无定义域以独立错误10明确标记，不能称为与原C有定义行为一致。

## 原板引脚与上板约束

- 高侧：PB5/PB6/PB7，ATIM CH1/2/3，高有效。
- 低侧：PA15/PB3/PB4，GPIO高有效，不是ATIM互补输出。
- LED PC13低有效；按键PA3上拉低有效。
- OPA1 PA6正输入、PA7负输入、PB0输出；外部电阻决定增益。
- ADC1：PB0电流、PA0/PA1/PA2反电动势。
- ADC2：PB2电流、PA8母线、PA10调速输入（原应用未用）、PA11温度、内部参考。
- UART1：PB12 TX/PB11 RX，115200 8N1；7字节 `43 57 04 档位 电压 关机标志 校验和`。

原main提到断开R40/R41并焊接R38/R39以改成单电阻合成电流采样。这是原工程说明，不是对实际PCB的验证或改板指令。没有原理图和实板测量，不能确认供电、分压、NTC、OPA、门极或保护参数适用。

必须实测模拟建立、PWM相位、GPIO换相间隔、DMA总线竞争、最坏IRQ延迟和外部硬件关断。Panic/HardFault紧急断开属于Rust运行时边界，并不替代原错误2..9或外部硬件刹车。禁止带功率随意halt调试器。

输入ZIP SHA-256：`39c5840adb344f1acb06f7ad8768406d2bab2a7456bf731cfe4a0954d8877af4`。原厂链接和资料版本见[寄存器证据](../../docs/register-evidence.md)；本次逐项验证见[原工程一致性审查](../../docs/bldc-source-parity.md)。
