# 01 → 07：独立的 BLDC / FOC 例程

板级参考：[BLDC 驱动板原理图](sch.pdf)（用户提供的 `sch.pdf`，原文件未修改）。原理图标注与实板差异仍以实测记录为准，例如采样电阻 R0 实物为 10 mΩ。

新增 [`07-sensorless-foc`](07-sensorless-foc/README.md)：独立的单电阻无感 FOC，实际 10 mΩ 分流电阻，双硬件触发/电流重建/Id-Iq PI/观测器。普通构建按键启动，无额外确认开关，仅在启动前及故障安全停机后输出非阻塞 RTT 日志，尚未实板验证。以下原工程一致性、20 kHz 采样和六步流程说明仅适用于 01–06。

输入是 `10 XUNLIANYING 260726 LAST.zip` 的 `BLDC CONTROL`：反电动势过零检测的六步换相，不是FOC。ADC、电机和故障处理按原工程恢复；UART、LED、按键的组织方式可以不同。逐项依据、原有边界和明确例外见[原工程一致性审查](../../docs/bldc-source-parity.md)。

**尚未实板验证。01–04不配置或写入六个桥臂脚；05/06普通构建包含功率输出，上电执行6 ms低桥充电，随后关闭桥臂等待按键启动。** 这不能代替物理禁用驱动器。编译和宿主轨迹通过均不代表可以直接给电机上电。

## 六个阶段

- `01-gpio`：默认4 MHz，只配置PC13 LED和PA3按键。
- `02-sampling`：原时钟、OPA/BGR、ADC1/2、ADC2 DMA、ATIM内部触发及BTIM1节拍。
- `03-six-step`：增加六步桥图和按键选择扇区，只观察，不输出功率。
- `04-sensorless`：增加原浮相/边沿/阈值的被动过零观察，不自动换相。
- `05-startup`：完整启动、换相、保护；电机前台连续运行，ADC1和BTIM中断独立，无UART或Embassy执行器。
- `06-application`：同样的控制/保护算法；高优先级InterruptExecutor运行事件驱动电机任务，普通线程executor运行按键/LED/UART任务。

每级都有自己的 `Cargo.toml`、`src/main.rs` 和 `build.rs`，只有一个binary，无lib target、跨例程源文件导入或共享业务crate。01–05的初始化、主循环和ISR直接位于各自main；06按实际职责分为main时钟与所有权拆分、motor板级配置/电机任务/ISR、io按键/LED/UART与软件命令/状态交换，控制/保护/协议/帧队列是本地独立逻辑模块。没有Board、State、I/O硬件包装或未调用的PI。寄存器操作已收敛为独立HAL `motor` 文件夹中的ADC扫描、PWM换相、定时器与模拟前端操作；DMA使用独立通道驱动。02–06不再直接依赖PAC。unsafe排他义务与逐项迁移见[电机API](../../docs/motor-api.md)。

## 构建

从干净源码先在workspace根执行 `cargo run -p xtask -- regenerate`。板级 `.cargo/config.toml` 为六个例程统一设置 `thumbv6m-none-eabi`；在任一例程目录中构建时会自动继承：

```sh
cd examples/l012-bldc/01-gpio
cargo build --release
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

六个例程均不定义本地 feature，不需要额外的 `--features` 参数。依赖项中的芯片、运行时、临界区、执行器和 RTT 非阻塞 feature 仍是实际构建所需。

05/06 普通构建即包含实际功率输出：初始化先将六个桥臂输出置低，随后按原流程执行 6 ms 三个低桥 bootstrap 充电，再关闭桥臂等待按键启动。原故障处理与 panic/异常紧急关断保留；不能把“等待按键”理解为上电期间从未输出。烧录或运行前须确认原理图、供电、门极极性、死区及保护，并物理禁用驱动。

这些是MCU程序，无宿主入口/平台条件分支；源码包不包含例程测试或外部生成脚本。

## 烧录与运行

安装 [probe-rs](https://probe.rs/docs/getting-started/installation/) **0.31.0或更新版本**，并确保 `probe-rs` 在 `PATH` 中。CW32L012C8使用probe-rs内置的 [`CW32L012x8`](https://github.com/probe-rs/probe-rs/blob/v0.31.0/probe-rs/targets/CW32L0_Series.yaml) target及其 `flashcw32l012` 烧录算法；可先用 `probe-rs chip info CW32L012x8` 检查本机支持，无需连接开发板。

板级默认runner为 `probe-rs run --chip CW32L012x8`，不指定调试器型号或序列号。在任一例程目录执行：

```sh
cargo run --release
```

它会构建、烧录并启动该例程。只连接一个可用调试器时由probe-rs自动选择；连接多个时按probe-rs提示处理。

从workspace根执行时，Cargo不会自动读取子目录的配置，须显式加载同一份板级配置，例如：

```sh
cargo run --release --config examples/l012-bldc/.cargo/config.toml -p cw32-bldc-01-gpio
```

其他阶段替换包名即可。以上命令适用于Windows PowerShell和常见Unix shell。尚未实际烧录或验证调试器连接；烧录和调试前必须物理禁用功率驱动，遵守本页的上板约束。

## defmt RTT 日志（01–06）

六个例程均已接入 `defmt` / `defmt-rtt`，各自 `build.rs` 链接 `defmt.x`，未设置 `DEFMT_LOG` 时默认 `info`（从 workspace 根构建也有效），显式环境变量优先。在对应例程目录执行 `cargo run --release`，现有 probe-rs runner 会读取 ELF 中的 defmt 描述并显示 RTT 日志；不需要占用 UART 引脚。Windows PowerShell 可在构建前设置日志级别：

```powershell
$env:DEFMT_LOG = "info"
cargo run --release
```

- 01：启动、标称时钟、按键/LED 状态变化（限频观察，可能略过短按）。
- 02：每秒 ADC1/ADC2 原码和已观察采样计数。
- 03：每秒原码及逻辑扇区/桥图；仍不驱动桥臂。
- 04：每秒原码、扇区及被动阈值资格观察次数；不是机械转速。
- 05/06：启动时说明低桥充电和按键启动流程；状态、启动阶段、档位或故障变化时，以及每 500 ms，输出启动步数、连续过零数、ADC 原码、校准值和上次保护计算的电压/电流。`bus` 单位 dV（0.1 V），`duty` 满量程 4800；`step_ticks` 单位为 8 MHz 定时器计数。保护在启动/故障等待期间按原逻辑暂停，`last_protection_bus` 不保证为本次 ADC 对应的即时电压。

`StartupFailed / code=3` 表示强制启动未满足连续 15 次有效过零。`armed` 表示初始化已成功完成软件输出解锁，不代表电机正在转动，也不证明功率管实际导通或硬件无故障。结合 `off`、启动阶段、ADC 原码及过零计数定位问题，不据此自动断定硬件原因。

运行日志均在普通线程中输出；05 在释放控制器临界区后输出，06 通过现有 I/O 状态交换复制少量观测值后输出，电机 ISR/中断执行器不调用 RTT。06 使用最新值覆盖，极短的中间状态可能合并。ADC2 仍是逐槽 DMA 观测，可能混合相邻扫描数据，日志不把它冒充完整 EOS 快照。

明确启用 `disable-blocking-mode`：缓冲区满或主机断开时允许丢失日志，不能因 probe-rs 将通道设成阻塞而一直等待。标准 RTT 编码器仍会短暂屏蔽中断，因此日志有时序开销，最坏延迟仍需实测；保留原 panic/异常安全关闭路径，不在这些路径追加可能重入的日志。诊断版本未做实板验证，不要带功率随意 halt 调试器。

## 时钟与ADC

用户确认原板VDDA=5 V。本例恢复原HSI/HCLK/PCLK=96 MHz，ATIM PSC=0、ARR=4799，20 kHz；所有PWM比较值直接用原4800刻度，不除2。BTIM1为1 ms，BTIM2/3为8 MHz。

ADC1通道 `[8,0,1,2]`，48 MHz，每槽70+15周期，四槽约7.083 µs。ADC2通道 `[11,5,7,8,Vref]`，12 MHz，每槽518+15周期，五槽约222.083 µs。保留ADC2的ATIM及每5 ms软件双触发；忙转换期间触发的实际行为和采样有效率须实测。

CH4恢复原PWM1。官方RM §25.12.7指定触发为OC4REFC上升沿，不能把SDK的ATIMCC4名称直接解释成在CCR4比较位置采样。ADC2使用逐槽EOC的DMA更新；这不是EOS一致快照。原DMA请求选择与未初始化字段的最小确定性修复，以及ADC CR bit8采用新版官方库的依据，见[详细审查](../../docs/bldc-source-parity.md)。

## 电流采样与10 mΩ实板校准

用户确认全部例程使用的实板分流电阻均为 **10 mΩ**；原理图 `BLDC_SCH.pdf` 中 R0 的 2 mΩ 标注不代表当前实装值。01 只操作 GPIO，无电流采样；02–04 仅显示 ADC 原码；05/06 的毫安换算和原过流门限均按下列实际链路解释。

- OPA 输入电阻 R17/R18 为 1 kΩ，反馈 R19 为 10 kΩ，R20/R21 各为 20 kΩ。按该网络推导，理想输出为 `Vout = VDDA/2 + 10 × Vshunt`；VDDA=5 V 时零点约 2.5 V。10 mΩ 分流电阻对应 100 mV/A。
- ADC1/PB0 接 OPA 输出，反映瞬时母线电流；ADC2/PB2 接 R22=10 kΩ、C10=470 nF 后的同一信号，理想 RC 时间常数为 4.7 ms、直流增益为 1。两路原码不能当成同一采样时刻，ADC2 也不是相电流 RMS。
- 05/06 先用 ADC2 实测零点作差，再按 `校准mV × 原码差 / Vref原码` 得到毫伏，随后乘 **10 mA/mV**。现有 `t *= 10.0` 已匹配 10 mΩ 实板；再次除以 5 会把电流低估五倍并抬高实际过流点。首次测量及每次启动前都沿用原流程更新零点；理想 2048 码不是强制写入的校准值。小于等于零点的输入仍按原 C 记为 0 mA。
- 3 A 连续 30 次和 10 A 当次检查门限保持不变，检查周期及启动/故障等待期间暂停保护的行为也保持不变。这里“即时”仅表示当前软件检查立即判故障，不是 ADC1 中断或硬件逐周期过流。理想 5 V/12 位 ADC 下，3 A 和 10 A 分别对应约 246 和 819 个正向差值码；实际比较仍使用参考校准和浮点截断后的毫安值。
- 05/06 日志的 `current=...mA` 是上次保护计算的 ADC2 滤波母线电流，可能与当前打印的 ADC 原码不属于同一轮；02–04 只打印原码，没有需按分流阻值更改的数值系数。

这是对实板参数及现有公式的核对；05/06 的数值公式、保护门限和故障时序未改，仍保留原 C 的 f32 求值顺序。供电、模拟偏置和增益误差、滤波响应及保护效果须实板测量，不能由这些理论值或编译结果证明。

## 电机执行与共享状态

05电机前台连续运行；06由真实ADC/定时器事件唤醒P1 InterruptExecutor中的电机任务，连续推进已就绪工作，不以1 ms轮询量化电机控制。BTIM1仅做原按键/ADC2启动/计数工作；100 ms电压电流温度检查在前台。原阻塞启动、停机和故障等待用显式前台续行状态表示：ISR仍运行，但原先被阻塞的前台保护不会额外执行。

05的连续前台在短临界区内借用控制器；06将电机任务放入UART2软件中断上的P1 InterruptExecutor，与ADC1/BTIM1/BTIM3的P1硬件ISR同级，ARMv6-M同级异常不能相互抢占。控制器不跨await借用，等待路径真正返回Pending，只有有限的立即可执行延续会连续推进。ADC逐样本处理与定时换相仍在ISR立即执行，唤醒只合并重复检查请求。普通线程executor仅有I/O，少量命令/状态/waker通过短临界区同步，不持有电机引用。05无执行器，作为直接ISR/前台教学阶梯保留；06展示完整任务分层。改变IRQ优先级、新增访问者或允许异常返回都需要重新审查共享安全。

06使用官方 `InterruptExecutor`，专用未启用UART2外设的UART2向量作P1电机软件唤醒；普通线程I/O任务处理PA3/PC13/UART1。I/O不能持有控制器引用或电机寄存器指针。UART非阻塞，每次I/O唤醒最多发送一字节。I/O失联不再增加原工程没有的电机故障码。

02–05在各自main、06在main集中使用 `bind_interrupts!` 声明实际向量与handler。
ADC1/BTIM1/BTIM3的原ISR主体分别位于 `Handler<typelevel::具体IRQ>` 实现内，
同步处理顺序不变；06的UART2 handler同步调用官方执行器。
`AdcScan::<peripherals::ADC1>::acquire()` 与 `BasicTimer::<peripherals::BTIM1>::acquire()`
保留元数据IRQ类型，使能中断源必须传入准确 `Binding` proof，不能只手写向量名后忽略绑定检查。
向量优先级和mask使用官方typelevel操作；BTIM3_HALLTIM不做NVIC unpend，handler仅确认
BTIM3自己的更新源。HALLTIM保持未启用，未来使用它须增加同向量handler并重新审查共享所有权。
HardFault/NMI/Panic仍由异常/运行时入口处理。具体迁移与unsafe义务见[电机API](../../docs/motor-api.md)。

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

原延时取消后仍可能进入启动、启动失败后状态暂时被覆盖、故障需到原前台位置才关桥等边界均保留并列明。没有继续保留移植时擅自添加的ADC/I/O失联故障或故障即时关桥策略。零除法/非法浮点转整数的C无定义域以独立错误10明确标记，不能称为与原C有定义行为一致。

## 原板引脚与上板约束

- 高侧：PB5/PB6/PB7，ATIM CH1/2/3，高有效。
- 低侧：PA15/PB3/PB4，GPIO高有效，不是ATIM互补输出。
- LED PC13低有效；按键PA3上拉低有效。
- OPA1 PA6正输入、PA7负输入、PB0输出；外部电阻决定增益。
- ADC1：PB0电流、PA0/PA1/PA2反电动势。
- ADC2：PB2电流、PA8母线、PA10调速输入（原应用未用）、PA11温度、内部参考。
- UART1：PB12 TX/PB11 RX，115200 8N1；7字节 `43 57 04 档位 电压 关机标志 校验和`。

原main提到断开R40/R41并焊接R38/R39以改成单电阻合成电流采样。这是原工程说明，不是对实际PCB的验证或改板指令。现有原理图和用户确认的 10 mΩ 实装值提供了名义参数依据，但尚无实板测量，不能据此确认供电、分压、NTC、OPA、门极或保护效果。

必须实测模拟建立、PWM相位、GPIO换相间隔、DMA总线竞争、最坏IRQ延迟和外部硬件关断。Panic/HardFault紧急断开属于Rust运行时边界，并不替代原错误2..9或外部硬件刹车。禁止带功率随意halt调试器。

输入ZIP SHA-256：`39c5840adb344f1acb06f7ad8768406d2bab2a7456bf731cfe4a0954d8877af4`。原厂链接和资料版本见[寄存器证据](../../docs/register-evidence.md)；本次逐项验证见[原工程一致性审查](../../docs/bldc-source-parity.md)。
