# 01 → 06：六个独立、递增的 BLDC crate

输入为 `10 XUNLIANYING 260726 LAST.zip` 中的 `BLDC CONTROL`，是反电动势过零检测六步换相，不是 FOC。每个编号目录都有自己的 Cargo.toml、唯一的 src/main.rs 入口和 build.rs；不设 lib.rs 或 library target。初始化、主循环和中断处理直接写在各自的 main.rs 中；其余模块按采样、换相、保护和板级硬件职责拆分。

**尚未实板验证。01–04 不配置、不持有也不写入六个桥臂引脚，不能靠软件保证它们为低；05/06 默认功率输出关闭。** 先断开母线或物理禁用 gate driver。编译通过不等于可安全接电机。只有 05、06 定义 `motor-output-enable`，01–04 根本不提供该 feature。

## 目录与最小依赖

- `01-gpio/`：默认 4 MHz 时钟、仅 LED/按键 GPIO；不依赖其他例程，不配置 ADC/OPA/PWM/定时器/UART。
- `02-sampling/`：独立采样 Board，ADC1/2、OPA/BGR、ATIM CH4 触发和 BTIM1 毫秒节拍；没有换相定时器、串口、电机状态机或输出解锁 API。
- `03-six-step/`：在本 crate 内保留聚焦采样模块，加六步桥状态与按键选扇区；独立入口与中断处理，不访问桥臂引脚。
- `04-sensorless/`：在本 crate 内保留采样与桥模型，加被动过零检测；独立入口与中断处理，不引入启动控制器或电机定时器。
- `05-startup/`：启动/换相/必要保护，独立电机 Board 与 IRQ；没有 UART、遥测队列、邮箱或 Embassy executor。
- `06-application/`：本地包含纯控制/保护算法，拥有完整 Board、UART/遥测和真正的 Embassy UI 任务。05 的 IRQ 向量只由其二进制导出，06 只编译自己的电机中断处理。原工程未调用的 PI 算法不保留。

## 自包含的普通模块

每个例程只有一个 binary target，业务源码都位于自己的 `src/` 内，通过普通 `mod` 声明加载；不使用跨 crate 的 `#[path]`、`include!` 或例程间依赖，也没有额外共享 crate。03/04 各自包含必要的 `sampling.rs`/`board.rs`，04 自带 `six_step.rs`，06 自带 `control.rs`、`protection.rs` 和 `board_contract.rs`。这些教学模块允许内容重复，算法保持一致，各级初始化与中断处理独立。01 的 LED/按键处理直接位于 main.rs。各例仍通过正常 Cargo 依赖使用 workspace 的 HAL/PAC，并非脱离 workspace 的独立发行包。

## 所有权与直接 PAC

01 只有普通 GPIO 主循环，无共享控制状态。02–05 没有 Mutex/RefCell，也不引入 async；02–04 的 `SAMPLING`、05/06 的 `MOTOR` 是本例具体业务状态，不是通用运行时。初始化先保持各 NVIC 向量屏蔽，配置硬件与统一 P1 优先级，将外设 owner 和控制状态一次移入静态存储，结束所有借用并设置编译器发布屏障后，最后才 unmask。之后前台不再借用该状态，只有同优先级 ISR 可访问；NMI/HardFault 用 PAC 直接关闭硬件后永久停机，不借用也不返回被打断的状态。

05/06 的桥操作直接写在 `board.rs::apply_bridge`，无 BridgeIo trait、模拟硬件或泛型转发层。该模块持有从 Embassy 初始化结果移出的真实单例；Embassy 任务不能再次取得电机外设。关桥、校验、采样点、故障检查、PWM/引脚连接及 MOE 顺序保留，默认仍禁止功率输出。

06 只有按键/存活信号、LED、完整遥测帧与唤醒通知通过小型邮箱共享，因此仅该邮箱保留 `Mutex<RefCell<UiLink>>` 和短临界区，不跨 await 持锁。没有对控制器、ADC、PWM 或整个 ISR 加锁。更改 IRQ 优先级、添加状态访问者或让异常返回，都必须重新审查这一独占约束。

## MCU 构建

干净源码先在 workspace 根目录执行 `cargo run -p xtask -- regenerate`。板级 `.cargo/config.toml` 已指定 `thumbv6m-none-eabi`；进入任意阶段目录即可直接构建，例如：

```sh
cd example/l012-bldc/01-gpio
cargo build --offline --release
```

六个 crate 直接面向 MCU，硬件依赖无条件启用，不保留主机入口或目标平台条件分支。ARM build 产生可供适配后的 probe 工具加载的 ELF；本仓库未设置通用烧录 runner，也未实际烧录。

从 workspace 根目录也可按包独立构建：

```sh
cargo build -p cw32-bldc-01-gpio --release --target thumbv6m-none-eabi
cargo build -p cw32-bldc-02-sampling --release --target thumbv6m-none-eabi
cargo build -p cw32-bldc-03-six-step --release --target thumbv6m-none-eabi
cargo build -p cw32-bldc-04-sensorless --release --target thumbv6m-none-eabi
cargo build -p cw32-bldc-05-startup --release --target thumbv6m-none-eabi
cargo build -p cw32-bldc-06-application --release --target thumbv6m-none-eabi
```

仅在审核硬件、供电、保护和测量时序后，05/06 可显式 `--features motor-output-enable` 允许相应输出路径：

```sh
cargo build -p cw32-bldc-05-startup -p cw32-bldc-06-application --release --target thumbv6m-none-eabi --features motor-output-enable
```

默认 ELF 用于断电检查。项目没有 Python 或外部项目生成脚本；生成仍为单 `cw32-gen` 的 YAML → 实际落盘 JSON → PAC。

## 各步骤运行行为与调试

### 01 GPIO 与原板引脚

先保持电机母线断开。使用已验证的默认4MHz复位时钟；只初始化PC13/PA3，不初始化电机引脚。按下PA3按键时PC13低有效LED亮，释放时灭。这一步不去抖、不启用ADC/定时器/UART，也没有 `motor-output-enable` feature。

构建：`cargo build -p cw32-bldc-01-gpio --release --target thumbv6m-none-eabi`。

### 02 OPA、ADC 与安全采样

加入原板OPA1输入与ADC1/ADC2序列，ATIM只作为20kHz内部采样触发源，不配置或写入六个桥臂控制脚。此步不存在 `motor-output-enable` feature。

使用96MHz CPU/48MHz PCLK，ADC均为6MHz。ADC1四槽18cycle采样，总22µs；ADC2五槽518cycle采样，总约444.167µs，每5ms软件触发。与原C的ADC时钟/同时触发有明确差异，见本文“迁移时确认的 SDK / 时序差异”。

通过调试器查看本阶段 `main.rs` 中的 IRQ 状态中的 `diagnostics` 字段 的 `adc1`、`adc2` 与序列计数；按键直接控制LED。不要在带功率时halt调试器。

构建：`cargo build -p cw32-bldc-02-sampling --release --target thumbv6m-none-eabi`。

### 03 六步换相表

保留02的实际模拟采样。上电调试图即为0号扇区的逻辑桥图，不触碰实际桥臂引脚。按键连续按下60ms后，将 `Diagnostics.selected_sector` 依次切换0..5，`logical_bridge` 显示原C的真实换相表与5%逻辑占空比；长按只切换一次。此处只检查逻辑桥臂图，不向电机施加静态电流，功率引脚不由本例配置或控制，没有输出解锁接口。

对应 `MOTOR.C::Commutation`，高侧PWM与低侧GPIO是两个不同控制量。桥图限制每一扇区同相上下管不能同时开启，非法扇区关闭，超范围占空比钳位。真正功率输出集成在05/06，不能用本例的慢按键当作电机换相调度器。

构建：`cargo build -p cw32-bldc-03-six-step --release --target thumbv6m-none-eabi`。

### 04 悬空相过零观察

在03基础上加入原C的悬空相选择 `[C,B,A,C,B,A]`、下降/上升交替、母线ADC值的一半门限、连续2次有效样本判断。选定扇区发生一次有效 crossing 后 `observed_crossings` 加1、LED亮，直到按键选择下一个扇区并重新布置检测器。

零母线、超过12位范围的母线值及超过20ms的ADC2旧值都拒绝用于检测。此步使用实际ADC值，但不配置或写入桥臂引脚，必须通过物理措施禁用驱动；可以在经电气确认的断电实验装置上观察外部拖动时的BEMF。没有自动施加电流，也不把静态比较结果宣称为已经证明转子位置。退磁消隐和延迟换相需要实际换相事件，完整路径在05/06中集成。

构建：`cargo build -p cw32-bldc-04-sensorless --release --target thumbv6m-none-eabi`。

### 05 定位、强制启动、过零接管

集成04的检测算法与真实ADC1/BTIM1/BTIM3中断，控制器独占状态与硬件。启动逻辑沿用原C：400ms延时、按电压计算初始占空比、150ms定位、首强制扇区2、最多200次强制换相、每次最多10ms、每次PWM+5、连续15次有效过零后接管，随后每30ms增加1%占空比。

保留电压/电流/温度保护、ADC失联检查、按键与停机逻辑，避免教学例程为了减少功能而去掉必要保护；不发送UART。06进一步把按钮与串口移到实际Embassy任务，展示所有权域分离。

默认不输出功率；未观察到模拟BEMF时，软件启动将按预期失败并记录错误，不是“编译后直接运行电机”。仅 `motor-output-enable` 允许05/06的板级驱动授权输出；上电初始6ms低桥bootstrap后关闭，第一次有效按键开机且0速，之后每次按键依次20/40/60/80/100/0%。

构建：`cargo build -p cw32-bldc-05-startup --release --target thumbv6m-none-eabi`。

### 06 Embassy 前台 + 独占电机实时域

这是完整应用入口，使用真实 `embassy_executor::main`。`Board` 持有ADC1/ADC2/OPA1/BGR/ATIM/BTIM1/2/3与电机引脚，初始化后移入仅由电机中断访问的 `MOTOR.board`。应用任务只保留由 `Board::new()` 一次性返回的独立 UI owner 内的PA3/PC13/UART1/PB11/PB12；不能再次取得任何电机外设句柄，没有 `Peripherals::steal`。

- ADC1 EOS：采样、过零检测、换相请求。
- ADC2 EOS：获取一致的五槽慢采样，送进电机控制域。
- BTIM3：退磁结束/延迟换相，8MHz tick。
- BTIM1：1ms核心状态与计时、100ms软件保护；通知Embassy任务有一次毫秒更新。
- Embassy任务：采样按键、显示LED、发送完整7字节遥测。任务通过仅含值的有界邮箱发送按键状态/存活信号，接收LED与帧；不跨域发送寄存器指针或控制器引用。

邮箱只保留最新完整UI状态；如果任务延迟，通知合并而不会无界堆积。活动电机期间UI任务100ms未更新会故障停机。UART非阻塞，每次唤醒最多尝试1字节，因此完整帧可能跨7次任务唤醒；不会在ISR忙等TXE。

UI任务的waker来自专用BTIM1事件，没有借用GTIM1或第二套时钟驱动。电机四个 IRQ 显式设为 P1，同优先级不互相抢占；初始化后只有这些 ISR 借用电机状态，前台不再读取它。IRQ 整体不加锁，只有实际跨线程/IRQ 的 UI 邮箱保留短临界区。100ms保护换算仍在该域执行，实际最坏执行时间、ADC相位偏斜与IRQ响应必须实板测量；编译通过不能作为20kHz实时性证明。

完整保留档位、500ms遥测、10s闲置关机、5s堵转与故障灯。默认功率输出关闭。构建命令同其他步骤，二进制名为 `bldc_06_application`；完整安全前提与可选输出feature见本文构建与安全章节。

## 原程序与 Rust 对应

- `global.h/global.c`：状态、周期、阈值与单位，转为具名常量和结构体字段。
- `MOTOR.C`：六步换相和 PWM 更新，转为桥臂状态与受限的板级写入。
- `sensorless.c`：退磁、连续过零、延迟换相、强制启动，转为事件驱动控制器，不使用忙等共享全局变量。
- `control.c`：启动/运行/停止/错误状态及占空比斜坡，转为状态机。
- `compu.c`：按内部参考标定换算母线电压/电流、计数保护、NTC 门限。
- `pid.c`：原工程没有调用的遗留 PI 计算不保留；本应用没有 PI 速度闭环。
- `User/main.c` 与 BTIM1 ISR：1ms 按键与计时、5ms ADC2 启动、100ms 保护/速度、500ms UART、10s 闲置关机。
- `init.c`：时钟、GPIO、ATIM、ADC、OPA、BTIM、UART，按阶段分布在各自的 `src/board.rs`。该适配层持有实际 singleton，不向通用 HAL 假装提供未实现的完整 BLDC 驱动。

## 原板约束，必须再次核对

原 `User/main.c` 明确要求断开 R40/R41、焊接 R38/R39，以取消 A 相独立采样并改为单电阻合成电流。我们只有工程源码，没有电路图或实际板子；这些是**原工程的说明，不是对你手中 PCB 的验证或改板指令**。

- PWM 上桥 A/B/C：PB5/PB6/PB7，ATIM CH1/2/3，高有效。
- 下桥 A/B/C：PA15/PB3/PB4，GPIO，高有效；不是 ATIM 互补输出。
- LED：PC13，低有效；按键：PA3，上拉、低有效。
- OPA1：PA6 正输入、PA7 负输入、PB0 输出，外部电阻决定增益。
- ADC1：PB0 电流，PA0/PA1/PA2 三相 BEMF；序列通道 `[8,0,1,2]`。
- ADC2：PB2 平均电流、PA8 母线、PA10 外部调速（原应用未用）、PA11 NTC、内部1.2V参考；序列 `[11,5,7,8,Vref]`。
- UART1：PB12 TX、PB11 RX，115200、8N1；发送7字节 `43 57 04 档位 电压(0.1V) 关机标志 校验和`。
- 原 PWM：96MHz / 4800 = 20kHz，边沿向上计数，ARR=4799。运行采样点 CCR4=300，停止 CCR4=4000。
- BTIM2/3：8MHz tick，退磁/延迟为上次换相间隔右移3位；高速阈值2000ticks、过快故障100ticks。不是默认1MHz Embassy time tick。
- 电流换算沿用原工程 mV×10 系数、母线11:1分压；3A持续30次与10A即时阈值、14V持续30次、欠压 `<6.6V` 或 `9.5V<V<10.0V`、NTC有效热样本 `50..=342` 累计25次（低于50忽略，大于342清零）。标定/分压/NTC/门限都不能直接泛用到其他硬件。

## 安全与差异

原程序存在故障后状态覆盖、零参考除法、阻塞启动/故障灯以及宏常量未接入保护等边界；Rust 版以显式错误、有限状态和立即关闭桥臂处理，详细差异和验证范围见验证记录。原 `HardFAULTAD=403`、`NUMtimes=2` 并没有在 ADC1 ISR 中执行，不应被误称为已经实现的硬件瞬时过流保护。

该板适配没有凭空添加不存在的比较器刹车链；即使有软件限流，也不能替代 gate-driver 硬件关断。必须自行验证短路、失步、ADC断线、欠压、MOSFET死区、相位、噪声、锁轴、异常复位和异常处理关断波形。暂停调试器时也可能留住输出，禁止在带功率时随意 halt。

查看 [验证记录](../../docs/validation-v0.11.4.md) 获取实际验证范围和剩余限制。

## 迁移时确认的 SDK / 时序差异

上传SDK的ADC控制寄存器含 `SAM[9:8]`，本仓库固定的官方header及RM1.4把bit8列为保留位（复位0x100）。Rust适配保留保留位读值，不向这一差异写入猜测值。

为遵守RM25.4.2在最低VDDA1.7–1.8V范围的ADC6MHz且200ksps上限，板例采用CPU/HCLK96MHz、PCLK48MHz，而不是原C的全96MHz：

- ATIM PSC=0，ARR=2399，仍为20kHz。算法继续使用原4800刻度，板驱动将CCR除2，奇数值向下取整，最大误差半个硬件tick。绝不把4800计数直接套在4MHz默认时钟上。
- BTIM1 PSC=47、ARR=999，仍1ms；BTIM2/3 PSC=5，仍8MHz，不改变控制器里的100/2000/右移3等时间常量。
- 两路ADC的时钟为48MHz/8=6MHz。ADC1每槽18采样+15转换=33cycle，四槽22µs，小于50µs PWM周期；采样速率181.8ksps。原C每槽70/48MHz的采样窗口约1.458µs，迁移后为3µs。三相结果依序取得，有真实时间偏斜，不能宣称等效同步采样。
- ADC2每槽518+15cycle，五槽约444.167µs。只保留5ms软件触发，不再同时接入50µs一次的ATIM触发；EOS中读取完整五槽，无DMA写数组与主循环读数组竞争。
- RM25.12.6的trigger bit5是ATIM OC4REFC上升沿。上传SDK命名为ATIMCC4且CH4配置PWM1，在向上计数时上升沿在重装载，不能直接声称采样在CCR4。Rust让无外引脚的CH4使用PWM2，使上升沿发生在按2:1映射后的CCR4位置；驱动相的CH1–3仍PWM1。

上述是从原厂寄存器/时序资料得到的保守软件适配。输入阻抗、RC滤波、OPA建立、反电动势采样窗口是否避开开关噪声，以及低占空比时是否仍可检测，必须用你的板测量。96MHz CPU本身也需符合实际供电/温度的芯片规格。

另保留两项原程序的重要语义：BTIM2的ARR=65530，在8MHz下约8.19ms就回卷，而强制换相可等待10ms；`StepTime` 仍是原始16位计数，不伪称为无回卷长时间戳。显示速度 `RealS` 是100ms内换相次数乘100，不是机械RPM；原工程没有更新 `RealS1`，因此停机采用其实际进入的500ms等待路径。NTC原始码小于50时本次忽略且保留已累计热样本，正常冷样本大于342才清零热计数。

寄存器依据：固定官方RM的4.7.3规定72–96MHz使用FLASHWAIT=3；4.7.1规定PCLKPRS=1为HCLK/2；25.4.1/25.4.2给出上述ADC转换周期和电压相关速度上限；25.12.6说明OC4REFC上升沿；25.10给出出厂参考校准halfword地址0x001007D2。原厂文档链接、版本与hash见[寄存器证据](../../docs/register-evidence.md)。

输入ZIP的SHA-256：`39c5840adb344f1acb06f7ad8768406d2bab2a7456bf731cfe4a0954d8877af4`，仅用于追溯，不包含在发布源码包中。
