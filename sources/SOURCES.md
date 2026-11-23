# YAML 数据与官方原始资料

本页对应当前整合源码、二十三个精确料号/封装的主动 LSE/RTC 资格及同为二十三款的 LSE SYSCLK 资格（2026-10-10），回答“这份 YAML 依据哪一本原厂手册、哪个 SDK、哪几页”。当前源码包附带 `approved-sdk-members/` 下的 11 份 Apache-2.0 芯片主头文件及其许可；完整 PDF、SDK ZIP、SVD 与 HTML 原件未附。下列文档链接直达官网，运行获取脚本后才会写入本地 `sources/vendor/`。

`evidence-sources.json` 是唯一 URL / SHA-256 / SDK 成员锁；本页是阅读入口，不再复制一份 JSON。原件版本是项目选定快照，不能据此声称已是厂商最新版。页码与章节从现有 YAML 及其明确引用的记录摘出，本次未重新逐页校读原件。

- RM = 该族中文用户手册；DS = 该族数据手册；SDK = 该族标准外设库 ZIP。每族下面给出完整文件名及版本。
- “PDF”从 1 起计；“书页”是页面上印刷页码，二者不一定只差 1。源记录的 `*_page_indices` 从 0 起计，本页显示时加 1；未注明基准的自由文本不换算。
- 页码集合仅涵盖已记录的引文，不能代替每条事实的适用条件。SDK 宏/行号留在对应 YAML 的 `source_macro`、`source_line`、`locator` 中；行号不是手册章节。
- SVD 是寄存器导入基线；审核后的 `registers/*.yaml` 还含人工修正。寄存器复用、构建策略、软件保守限值均不自动成为厂商硬件事实。

L010/L011 软件 ADC 中断单次与顺序扫描沿用既有 YAML。其 EOS/EOC、START 清零停止、R1W0 和结果槽依据分别是 L010 RM Rev 1.2 §20.5.1–2、§20.12（PDF 508–510、515、518、523–526）与 L011 RM Rev 1.1 §20.5.1–2、§20.11（PDF 510–512、516、519、525–528）；这些手册的书页为 PDF 页减一。配套 SDK 示例为两族 `Examples/ADC/adc_sqr_irq_sw/USER/src/` 的 `main.c` 与各自 `interrupts_cw32l010.c` / `interrupts_cw32l011.c`，准确压缩包内路径和 SHA 见唯一来源锁，事实说明见 [adc-low-async.md](../docs/adc-low-async.md)。这不补足原手册未提供的硬件排空握手或中止最长延迟。

## 当前文件级来源覆盖

只统计 `cw32-data/` 内的 297 份 authored YAML：137 份寄存器定义 + 160 份其他数据；不含 `cw32-data/data/` 等生成目录。此前新增 `af/cw32f030-atim-complementary.yaml` 与 `af/cw32a030-atim-complementary.yaml`，两份均保留自己的 DS 单元格、共享 x030 RM 单元格及精确 SDK 宏/行号。L010/L011 与 L052/L083 HSE、UART/SPI DMA 和当前异步 ADC 沿用已有硬件 YAML；新增 `mco-qualified.yaml` 直接引用既有 x030 RM、F030 DS 与三个 SDK 成员，仅投影精确 C8T7 的 MCO 事实和 PA8 AF4；主动 LSE 的 `lse-qualified.yaml` 直接记录二十三个精确料号/封装的原厂文档 ID、SHA、PDF 页码与书页；完整清单及各族来源见末节。以下四类按顺序互斥，每个文件只计一次。定位可为“官方文档 ID + 章节/页码”，也可为“锁定 SDK 成员 + 宏/行号/SVD 寄存器名”；“直接”表示文件中至少有一处这样的定位，或寄存器基线经既有 canonical/input 映射能定位原 SVD。人工覆盖不因基线可定位而自动通过审查。

| 归类 | 文件数 | 边界 |
| --- | ---: | --- |
| 可直接定位原件中的内容 | 286 | 137 份寄存器有既有 canonical/input → 官方 SVD 结构来源；149 份其他 YAML 至少有文档章节/页码、SDK 符号/行号等入口。 |
| 经明确引用的审阅记录可找到原件与章节/页码 | 11 | 下列十一个根级能力文件；正文已摘出其中部分精确引文。 |
| 只找到原文件，未找到上述内容定位 | 0 | 指整个文件没有任何内容定位；不表示每条事实都已有章节。 |
| 连原文件都无法确定 | 0 | 仅表示这次文件级来源链检查没有遇到此类文件。 |

第二类是 `adc-sequences.yaml`、`classic-adc-scans.yaml`、`classic-timer-input.yaml`、`crypto.yaml`、`lcd.yaml`、`lvd-ir.yaml`、`ram-parity.yaml`、`rtc-alarms.yaml`、`rtc-calendar.yaml`、`spi.yaml`、`rtc-live-lsi-qualified.yaml`。这组计数不是逐字段覆盖率，也不是重新完成硬件审查；一份文件里可以同时有已定位的内容和仍待补全的人工修正/电气条件。

优先检查的边界：`field-access.yaml` 的 UIFCPY 已直接写明 L010/L011 RM §11.9.8（书页 184 / PDF 185）、§13.9.13（276 / 277），以及 L012 RM §14.10.8（208 / 234）、§16.10.13（303 / 329）；RAM 项经明确的同族审阅记录定位。`register-writes.yaml` 的逐命令语义、RCC 的派生文本行号、`electrical.yaml` 的 ADC 限值仍须保留各自 claim 与原文的关系，不能由上述“文件至少一处有定位”推定全部覆盖。

## 按数据主题查找

| YAML | 原始依据与定位方式 |
| --- | --- |
| `inputs/*.yaml`、`registers/*.yaml` | 本页各族的 SDK → SVD / CMSIS；输入中的修正需再看同族 RM。137 份寄存器 YAML 的 SVD 基线可按寄存器/字段名定位；人工修正仍须自己的 RM 引文，复用哈希不代替该引文。 |
| `parts.yaml`、`additional-parts.yaml` | 各族 DS 产品/存储器表与订货表 9-1；DFP `.pdsc` 只作交叉核对。网页目录是产品快照，不裁定寄存器兼容性。 |
| `pinouts/*.yaml`、`af/*.yaml` | 各族 DS 封装/复用表、RM 路由表，以及 SDK GPIO 宏；保留候选、未解决和适用封装标记。新增 F030/A030 `*-atim-complementary.yaml` 的 CH1B–3B 路由及封装定位见下表。 |
| `clock/*.yaml`、`dma/*.yaml`、`triggers/*.yaml` | 各族 RM 时钟、复位、DMA 请求与目标触发表；CMSIS 和外设头文件作编码核对。 |
| `adc-sequences.yaml`、`classic-adc-scans.yaml` | 各族 RM ADC 顺序、启动/结束/结果寄存器；async 的独立 IRQ、有限转换与 R1W0 定位另列如下。L012 ADC2 另有 DAC 共享 IRQ 入场依据；电气条件仍见 `electrical.yaml`。 |
| `classic-timer-input.yaml` | 同族 RM GTIM 捕获/QEI/输入选择与 GPIO AF；SDK GTIM 仅佐证。 |
| `rtc-calendar.yaml`、`rtc-alarms.yaml` | 同族 RM RTC 访问窗口、日历与闹钟；DS 振荡器条件；SDK RTC 佐证。冲突未解决的 Alarm B 不因有引文而变成已验证功能。 |
| `ram-parity.yaml`、`field-access.yaml`、`register-writes.yaml` | 同族 RM 读写属性、状态/清零语义；后两项是字段/命令覆盖，若只写审计文件名，仍欠直接原件定位。 |
| `electrical.yaml`、`spi.yaml` | 同族 DS 电气表和 RM 分频/时序；`electrical.yaml` 大部分旧策略只间接引用审计文件，不能把这些 JSON 当成原厂来源。 |
| `hse-qualified.yaml` | F020/F030/A030/L010/L011/L031/L052/L083/R031/W031 HSE 的各自 RM/DS；L010/L011/L031/L052/L083/R031/W031 的实际版本、章节与页码见下表。保留 RM/DS 外部输入下限冲突的交集，不跨族继承电气限值或 CCS 策略。 |
| `lse-qualified.yaml` | 仅末节二十三个精确料号/封装；x030、F020、L031、R031、W031、L052、L083、L010、L011、L012 各自 RM/DS 与原件对应收据给出原件 ID/SHA、PDF/书页和 SDK 成员定位，其他料号不自动继承主动配置资格。 |
| `pll-qualified.yaml`、`electrical.yaml` 的 PLL、对应 SYSCTRL 模板与 `field-access.yaml` 的 PLL 字段 | CW32L083/F020/F030/A030 的一次性 factory-HSI- 或 HSE-fed 系统 PLL；晶振/旁路共用既有HSE板级契约，各族自己的 RM/DS、原件 SHA 与 PDF/书页见末节。F020 采用 current-datasheets 中 printed Rev1.3，输出交集12–48MHz；L083/F030/A030为12–64MHz。保留模拟档位与电气上限区别、reserved-debug默认0x5、STABLE只读及rate-only时序限制；不外推其他族。 |
| `lsi-sysclk-qualified.yaml`、`electrical.yaml` 的 `lsi_sysclk` | F020/F030/A030、精确 CW32F002F3P7/F3U7、CW32F003F4P7/F4U7/E4P7、CW32L031C8T6/C8U6/F8U6、CW32R031C8U6 与 CW32W031R8U6 的 init-only factory-LSI SYSCLK；逐族记录既锁 RM/DS 的 ID、SHA、PDF/书页，以及完整消费者、门控、复位与基址事实。F020 使用 current-datasheets 中真正 printed Rev1.3。F002/F003 generic alias 与其他未列料号不继承资格；两族均无 RTC/专用 LSI_OUT。classic 同源 RTC/LSI 别名仍为 rate-only，定位与兼容变化见下节。 |
| `hex-qualified.yaml` | F002/F003 各自 RM/DS 的直接 HEX 输入、PB0/PB1 与 AWT 来源；精确 PDF/书页见下表。保留 RM 4–32 MHz 与 DS 1–32 MHz 的交集及全部波形条件，不据此推定晶振、PLL 或失钟恢复能力。 |
| `gpio-interrupt.yaml` | 各族 RM GPIO ICR 和中断表；CMSIS IRQ 枚举。 |
| `reference-dividers.yaml` | L010/L011/L012 RM VC 分压器及 DS 电气范围，SDK VC 头/实现作佐证。 |
| `dac-opa.yaml` | L012 RM DAC/OPA 与 DS §§7.3.14、7.3.18；DAC 电源/参考与模拟建立时间保留原条件。 |
| `accelerators.yaml` | L012 RM Rev 1.4，书页 156–157；只有书页，无直接 PDF 页码。 |
| `crypto.yaml` | L083 RM Rev 2.0 §§26、28，TRNG 书页 536–541、AES 565–570；DS §§4.20–4.21，书页 21–22。 |
| `lcd.yaml` | L052/L083 各自 RM LCD 章节、DS 引脚与电气表；原件关键章节见下方。 |
| `lvd-ir.yaml` | 同族 RM LVD/IR 控制，DS IR 引脚表，SDK GPIO/LVD；不继承别族阈值。 |
| `register-source-aliases.yaml` | A030 DS Rev 1.1 表 6-1 书页 28，与 x030 RM Rev 2.5 表 2-1 书页 26–27、§6.5 书页 108，支持限定的 F030→A030 寄存器映射复用。 |
| `register-reuse.yaml` | 项目对完整标准化寄存器描述的相等性记录；不是原厂手册，不证明电气或行为等价。 |

## F020/F030/A030 factory-LSI SYSCLK 来源与兼容边界

新增的 `lsi-sysclk-qualified.yaml` 只引用已有锁定原件，不增加来源或修改其 SHA。F020 RM Rev1.4 的 CR1、LSI、ready/IRQ、门控分别见 PDF 69、72、76–79、80–85；GPIO、RTC、AWT、UART 的选择器与消费者定位分别在 PDF 140–155、174–186、163–170、267–295，IWDT 独立 RC10K 见 PDF 250–251。F030/A030 共同使用 x030 RM Rev2.5，对应页为 71、74、78–81、82–87，以及 143–158、177–189、166–173、329–357、312–313。这两本 RM 在上述定位的书页均为 PDF 页减一；完整离散页集合与逐字段定位保留在 YAML。

LSI 电气表使用 F020 current DS Rev1.3 PDF 44（书页 43）、F030 DS Rev1.9 PDF 46（45）及 A030 DS Rev1.1 PDF 43（42）。三族标称均为 32800 Hz，适用 1.65–5.5 V、−40–105 °C；F020 速率区间为 31160–34440 Hz，F030/A030 为 31816–33784 Hz。这些是速率界限，没有逐边沿周期或抖动保证。三族现有 `LsiClock`、`CalendarClock::Lsi` 和 RTC 分频后界限统一保留 rate-only 属性，即使 SYSCLK 选 HSI 也如此；严格周期与死区时序请求会被拒绝。另有下节精确 L031 三封装、R031C8U6 与 W031R8U6 的独立 rate-only 兼容变化；其他料号的 RTC/LSI 资格不变。完整 admission、whole-GPIO 功能移交与失败边界见 [factory-lsi-sysclk.md](../docs/factory-lsi-sysclk.md)。

## L031 精确三封装 factory-LSI SYSCLK

仅 CW32L031C8T6（LQFP48）、CW32L031C8U6（QFN48）和 CW32L031F8U6（QFN20）使用自身已锁 `CW32L031_UserManual_CN_V1.6.pdf`、`CW32L031_DataSheet_CN_V1.9.pdf` 与 SDK V1.4；不增加原件或修改来源锁。RM SHA-256 为 `4288cfd97b56385059c5a283f69972047af4773ef8bbc4d8b51d8155fb17a760`，DS 为 `90525f4085d00e9d586a991c24f2e4398d92a41e6cc1402c413e963423a35855`。SDK ZIP 身份为 `ef955869214dc80400c0b572f71f2ca54d8a76d89b5ed2756003477a11425e0d`，仅作自身字段佐证。

DS PDF47/书页46 表7-18/19 与 PDF38/书页37 表7-4给出 32800 Hz、31816–33784 Hz、1.65–5.5 V、−40–85°C。25°C 的 ±1% 行和有条件105°C低耗工作区不扩大该精度范围。DS PDF10/23–27/32/77–78（书页各减一）绑定封装、64 KiB Flash、8 KiB SRAM 和引脚；F8U6 无 HSE 焊盘，PB11 也未键合。仍检查其 AFR11 是保守软件政策，不代表存在物理输出。

RM PDF54/56/71（书页53/55/70）给出0x00100A02原厂半字、十位 TRIM 与 WAIT；PDF55/67/68/74–76/91–95（书页各减一）支持原生 selector3、可配置 CCS/LSELOCK、双稳定状态、独立 LSIRDY 与 SYSCTRL IRQ4（RCC 功能）。九门十一selector的自身依据为 RM PDF77–82/89/141–142/150/165/168–172/179–180/321–322/353，以及 DS PDF30；书页均减一。FILTER7 是文档化 AWT overflow，本次保守拒绝；SDK 的 LPTIMPWM 名称不替代 RM，MCO7 则另属未文档化值。

所有精确三款 RTC LSI 别名与分频后界限在任何 SYSCLK 下均为 rate-only；其他 L031 与板级 LSE 资格不变。R031C8U6 的独立变化见下节；原 L031 审阅不覆盖 R031。完整冷/活源、外部所有权、工作门、失败与有限验证边界见 [L031 contract](../docs/l031-factory-lsi-sysclk.md)。该说明及其有限验证计划不替代[运行/源码审阅](../docs/l031-factory-lsi-runtime-review.json)与[metadata 审阅](../docs/l031-factory-lsi-metadata-review.json)的明确范围和结果；最终 clean 复现需另有收据绑定最终源码包，不改写 main 审阅。既有 LSE 资格记录不被改写为新 LSI 验收。

## R031 精确单封装 factory-LSI SYSCLK

原 R031 资格仅为 CW32R031C8U6（QFN48）新增该系统源；generic R031 与全部 W031 均不继承该历史验收。CW32W031R8U6 的新独立资格见下节，generic W031 仍排除。自身已锁 RM CN V1.3 的 SHA-256 为 `fbee9b6942be9fa09f00c946705644d5356c4249f3e2280e3dfe5f0cb342eddb`，DS CN V1.2 为 `88759314fa4cf8b6caf7098df4829489179e27aa3752a13de85ce955b29c5805`，SDK V1.1 为 `cec9df232d64b53b638c8a372f50fde1fd677f72c04bb3ae467b8138d8433082`；原件与成员继续由 `evidence-sources.json` 管理，不增加下载或变更锁。

DS PDF11/书页10 与35/34绑定精确封装、64 KiB Flash（0x00000000）和8 KiB SRAM（0x20000000）；PDF54/53表7-23及42/41表7-4给出32800/31816/33784 Hz、2.2–3.6 V、−40–85°C。25°C的±1%、RM±10%调整范围及RF 250 kbps的−40–70°C行不扩大或替代该MCU资格。VDDA=VDD、VDDRF与所有接地须由实际板级资格保证。

RM PDF56/58/73支持0x00100A02原厂半字和原生十位TRIM；PDF69–78支持selector3、可配置CCS/LSELOCK、双稳定与独立ready状态，SYSCTRL为IRQ4。RM PDF79–84/91/139/143–144/152/167/170–174/182/356与DS33支持九门十一selector、PB11 AF1、RTC和AWT/UART/GPIO/MCO消费者。PDF306–309、477/485、490/495分别区分IWDT RC10K、VC PCLK/RC150K与LVD HSIOSC/RC150K；书页均减一。

RM PDF49/498/533与DS10/14/16明确专用16 MHz振荡器→RFCLK、可分频RF XTAL_OCLK以及PCLK→GPIOA→PA00..PA03的RF主机路径。无直接MCU LSI→RFCLK根，并不表示没有间接主机影响；恢复GPIOA门不能撤销采样/滤波/事件或保证RF连续性。既有功能移交继续适用，若RF输出供给HSE旁路须保持可用；不读写RF寄存器、不增加safe Rust隐藏内存安全义务。

该精确单款的RTC LSI别名及日历tick在所有SYSCLK下变为rate-only，标称32800/32768不变。既有LSE自身资格仍不授予LSI系统源。main验收已覆盖六个实际library构建、七个链接ELF与六项source/data检查，所有命令通过且编译零warning；自身53字段/access、九门、十基址、SYSCTRL4与精确封装/存储器另经审阅。完整边界及冻结设计身份见 [R031 contract](../docs/qualified-r031-lsi-sysclk.md)，main结论分别见[运行/源码审阅](../docs/r031-factory-lsi-runtime-review.json)与[metadata审阅](../docs/r031-factory-lsi-metadata-review.json)。这两份不可变main收据在签署时记录clean尚未执行；最终源码包仍要求独立的一library/两ELF clean复现，其结果由另行追踪的final addendum记录，不改写main或旧族审阅结论。

## W031 精确单封装 factory-LSI SYSCLK

仅 CW32W031R8U6（QFN64）取得独立资格，generic W031 与其他封装不继承。自身已锁 RM CN V1.4 的 SHA-256 为 `b6973677946a9332b0e5b3e954119768aa40469d44140a73e18648e9419bedc9`，DS CN V1.3 为 `45ec43e6370956d09f9b9aa4c0f6c83fb0661f576d2d2bf64e0a6d7c4e203d3c`，SDK V1.3 为 `1010176816766d025c84babf84ffdf2ee03c5687d2a29133267d0013a1970337`。原件、DFP1.0.2 的 SVD/PDSC 及 SDK 成员继续由 `evidence-sources.json` 锁定，不下载或重新锁定。

DS PDF9/书页8、33–34/32–33、71/70 与自身 PDSC 绑定 QFN64、64 KiB Flash（0x00000000）及8 KiB SRAM（0x20000000）。DS53/52 表7-23/7-24 在41/40 表7-4条件下给出32800/31816/33784 Hz、2.0–3.6 V、−40–85°C。供电采用 RF-LDO/DCDC 保守交集；VDDA=VDD，RF 使用时独立 VDDRF 须与 VDD 同源，全部供电及接地由板级资格保证。25°C ±1%、调整范围、占空比及启动时间不构成逐周期界限。

RM55/54、57/56、65/64、72/71及自身SDK支持0x00100A02单次原生半字读取、十位停止态TRIM、保留WAIT及reserved。53字段/access、九门、十基址、十一selector与精确引脚由自身证据绑定。LSIRDY观察SYSCTRL IRQ4；CLKFAULT IRQ31在自身RM/header/startup另证，不能替代该观察。已有原生P0–P9可执行代码不变，GPIOB FILTER/PB11共用一个严格恢复窗口。

RM48/47、49/48、55/54、499/498 与 DS29/28 明确专用32 MHz振荡器→RFCLK/内部RF PLL；RM363/362、366/365、509/508和DS14/13明确PCLK→SPI1及内部PB05 MOSI、PB04 MISO、PB03 CS、PB13 SCK、PB06 RF IRQ，主机速率须低于10 Mbps。whole-GPIOB窗口可推进主机/IRQ采样及事件，恢复门不能撤销；既有功能移交须完成或静默时钟敏感主机传输并允许完整窗口，不增加隐藏Rust内存安全前提。无RF寄存器、页、模式、电源、复位、命令或中断状态操作。

精确W031的RTC LSI别名及日历界限在任何SYSCLK下均为rate-only，标称32800/32768不变；generic W031、旧族边界及板级LSE保持原资格。固定1 MHz time driver在singleton与RCC MMIO之前纯预检拒绝。冻结设计及独立设计接受身份、主五library/六ELF与clean一library/一ELF计划见 [W031 contract](../docs/qualified-w031-lsi-sysclk.md)。main十一条构建命令已全部通过，保留五个rlib与六个ELF，1,294文件输入快照不变；生成及六项有限source/data命令通过，main Cargo/source-data无失败调用记录。独立运行/源码及metadata/投影审阅已接受冻结main实现，身份与SHA见契约；main接受时clean尚未执行；最终包完成须另有一library/一ELF clean收据及确定性打包验收，main收据的历史结论不改写。未来recipe计数与实际收据分开，旧R031/L031/LSE接受记录不改写。

## F002 精确两封装 factory-LSI SYSCLK

仅 CW32F002F3P7（TSSOP20）和 CW32F002F3U7（QFN20）使用自身已锁 RM Rev1.4 与 DS Rev1.2。RM SHA-256 为 `e453b3f82d9d180a86cd5f7cb18d858d7f228afc8101d87c700ca9d5c02d5add`，DS 为 `6d0c5c37d069e5b4e33d394b0be6c53938868e9375e7b4bbbbdd40d62bb9d506`；不变更来源锁。RM PDF 44–60（书页 43–59）给出系统/LSI、双稳定状态与 ready 事件；DS 表 7-14 PDF 37（书页 36）给出 32800 Hz、31160–34440 Hz、1.65–5.5 V、−40–105°C 的 full-range factory 行。RM 的 ±10% 调整范围不作 factory 精度使用。SDK `cw32f002_rcc.h:133`、`cw32f002_rcc.c:301–306` 佐证自身 0x001007BA halfword；不复制 SDK 的整寄存器写和 WAIT 赋值。

DS 表 3-1 PDF 8（书页 7）明确两个料号均为 16 KiB FLASH、2 KiB SRAM；表 6-1 PDF 26（书页 25）给出 0x00000000–0x00003FFF 与 0x20000000–0x200007FF。例程从既有 exact metadata 生成链接边界并核实真实容量，不扩大容量以通过链接。

新资格检查 AWT、UART1/2、GPIOA/B/C、MCO 与 ready/NVIC 观察者；没有 RTC、UART3、GPIOF、PB11 LSI_OUT 或 classic CCS 字段。whole-bank 检查可能推进采样、滤波和事件，软件不清旗标但旗标可自然改变。`lsi_output_pin` 从必有字符串变为 Option 是公开 metadata 类型变化；classic 仍为真实 Some，保持原 RTC/output/consumer 检查。ADC 与固定 1MHz time driver 拒绝 selected LSI 的 rate-only 系统时钟。完整初始化、功能移交和失败边界见 [F002 contract](../docs/f002-factory-lsi-sysclk.md)；编译/静态审阅不构成硬件验证。

## F003 精确三封装 factory-LSI SYSCLK

仅 CW32F003F4P7（TSSOP20）、CW32F003F4U7（QFN20）与 CW32F003E4P7（TSSOP24）使用自身已锁 RM Rev2.3、DS Rev1.9 和 SDK V1.7；不修改来源锁。DS PDF 5/8/27/62（书页 4/7/26/61）给出精确封装及 20 KiB Flash、3 KiB SRAM，基址分别为 0x00000000、0x20000000。E4 由 DS 支持，没有匹配 DFP device，不能标作 DFP 验证。

DS 表 7-15 PDF 38（书页 37）给出 32800 Hz、全温 ±3% 即 31816–33784 Hz，适用 1.65–5.5 V、−40–105°C；不采用 25°C ±1% 或 RM ±10% 调整范围代替该行。RM PDF 46/48/59（书页 45/47/58）与 SDK `cw32f003_rcc.h:133`、`cw32f003_rcc.c:301–307` 支持自身 0x001007BA halfword 和十位 TRIM；保留 WAIT 与其他位，不复制 SDK 整字写。RM SHA-256 为 `0fa58dac223add7f2ac1ee714a7df7db4e414e0f193601b80dda96a948bfc738`，DS 为 `5fe15321b3963472c2629030767cf61a0ce36063815b54add74025b5b13b95dd`。

自身 RM PDF 62–67、70–71、188–189、221–222、355–356、360 支持 gate/reset 邻位保护及 ATIM/IR 的传递路径；没有独立 LSI selector，不新增它们的工作窗口或 API 资格。AWT/UART1/2/GPIOA/B/C、MCO 和 ready/NVIC 构成七 selector、六 gate；保留 whole-bank 功能移交、rate-only ADC/time-driver 拒绝及失败部分状态。既有 optional output metadata 已能表达自身无 RTC/专用 LSI_OUT，不新增 schema；F002/classic 事实与消费者集合保持。完整边界见 [F003 contract](../docs/f003-factory-lsi-sysclk.md)。

## 13 个芯片族对应的原件版本

共 45 项固定来源：12 个 SDK ZIP、31 个 PDF、2 个 HTML。下表是各族主要来源，补充/历史原件另列；文件名、内部版本和日期分别记录，不能互相替代。

默认获取与硬件审计要求 43 个 PDF/SDK 原件及其锁定派生文件；两份 HTML 是历史发现记录，单独使用 `./d refresh-discovery` 检查当前页面。追加 `--include-discovery` 可严格复验全部 45 项所选原件，仍拒绝缺失或变更的 HTML。此范围划分不改变下列硬件引用，详见 [发现记录策略](../docs/a030-discovery-policy.json)。

| 芯片族 | 用户手册 RM | 数据手册 DS | SDK ZIP |
| --- | --- | --- | --- |
| CW32A030 | [CW32x030_UserManual_CN_V2.5.pdf](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf)；Rev 2.5，封面 2024-09，修订表 2024-07-24 | [CW32A030_DataSheet_CN_V1.1.pdf](https://www.whxy.com/uploads/files/20251230/CW32A030_DataSheet_CN_V1.1.pdf)；Rev 1.1，发布/修订 2025-12-30 | [CW32F030_StandardPeripheralLib_V2.2.zip](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip)；Rev 2.2，发布/修订 2024-11-11 |
| CW32F002 | [CW32F002_UserManual_CN_V1.4.pdf](https://www.whxy.com/uploads/files/20240920/CW32F002_UserManual_CN_V1.4.pdf)；Rev 1.4，封面 2024-09，修订表 2024-07-19 | [CW32F002_DataSheet_CN_V1.2.pdf](https://www.whxy.com/uploads/files/20251230/CW32F002_DataSheet_CN_V1.2.pdf)；Rev 1.2，发布/修订 2025-12-30 | [CW32F002_StandardPeripheralLib_V1.2.zip](https://www.whxy.com/uploads/files/20240115/CW32F002_StandardPeripheralLib_V1.2.zip)；Rev 1.2，发布/修订 2024-01-15 |
| CW32F003 | [CW32F003_UserManual_CN_V2.3.pdf](https://www.whxy.com/uploads/files/20240920/CW32F003_UserManual_CN_V2.3.pdf)；Rev 2.3，封面 2024-09，修订表 2024-07-24 | [CW32F003_DataSheet_CN_V1.9.pdf](https://www.whxy.com/uploads/files/20251226/CW32F003_DataSheet_CN_V1.9.pdf)；Rev 1.9，发布/修订 2025-12-25 | [CW32F003_StandardPeripheralLib_V1.7.zip](https://www.whxy.com/uploads/files/20250606/CW32F003_StandardPeripheralLib_V1.7.zip)；压缩包名版本 1.7；未核实内部发布版本，发布日期未记录 |
| CW32F020 | [CW32F020_UserManual_CN_V1.4.pdf](https://www.whxy.com/uploads/files/20240920/CW32F020_UserManual_CN_V1.4.pdf)；Rev 1.4，封面 2024-09，修订表 2024-07-19 | [current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf](https://www.whxy.com/uploads/files/20251230/CW32F020_DataSheet_CN_V1.3.pdf)；Rev 1.3，发布/修订 2025-12-30 | [CW32F020_StandardPeripheralLib_V1.2.zip](https://www.whxy.com/uploads/files/20240115/CW32F020_StandardPeripheralLib_V1.2.zip)；Rev 1.2，发布/修订 2024-01-15 |
| CW32F030 | [CW32x030_UserManual_CN_V2.5.pdf](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf)；Rev 2.5，封面 2024-09，修订表 2024-07-24 | [CW32F030_DataSheet_CN_V1.9.pdf](https://www.whxy.com/uploads/files/20251229/CW32F030_DataSheet_CN_V1.9.pdf)；Rev 1.9，发布/修订 2025-12-29 | [CW32F030_StandardPeripheralLib_V2.2.zip](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip)；Rev 2.2，发布/修订 2024-11-11 |
| CW32L010 | [CW32L010_UserManual_CN_V1.2.pdf](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf)；Rev 1.2，封面 2026-05，修订表 2025-09-19 | [CW32L010_DataSheet_CN_V1.3.pdf](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf)；Rev 1.3，发布/修订 2026-04-16 | [CW32L010_StandardPeripheralLib_V1.0.9.zip](https://www.whxy.com/uploads/files/20260806/CW32L010_StandardPeripheralLib_V1.0.9.zip)；Rev 1.0.9，发布日期未记录 |
| CW32L011 | [CW32L011_UserManual_CN_V1.1.pdf](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf)；Rev 1.1，封面 2026-06，修订表 2025-09-19 | [CW32L011_DataSheet_CN_V1.1.pdf](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf)；Rev 1.1，封面 2025-07，修订表 2025-05-19 | [CW32L011_StandardPeripheralLib_V1.0.3.zip](https://www.whxy.com/uploads/files/20251016/CW32L011_StandardPeripheralLib_V1.0.3.zip)；压缩包名版本 1.0.3；未核实内部发布版本，发布日期未记录 |
| CW32L012 | [CW32L012_UserManual_CN_V1.4.pdf](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)；Rev 1.4，封面 2026-06，修订表 2026-06-03 | [CW32L012_DataSheet_CN_V1.0.pdf](https://www.whxy.com/uploads/files/20250717/CW32L012_DataSheet_CN_V1.0.pdf)；Rev 1.0，封面 2025-07，修订表 2025-07-16 | [CW32L012_StandardPeripheralLib_V1.0.5.zip](https://www.whxy.com/uploads/files/20260701/CW32L012_StandardPeripheralLib_V1.0.5.zip)；压缩包名版本 1.0.5；未核实内部发布版本，发布日期未记录 |
| CW32L031 | [CW32L031_UserManual_CN_V1.6.pdf](https://www.whxy.com/uploads/files/20240920/CW32L031_UserManual_CN_V1.6.pdf)；Rev 1.6，封面 2024-09，修订表 2024-07-24 | [CW32L031_DataSheet_CN_V1.9.pdf](https://www.whxy.com/uploads/files/20251229/CW32L031_DataSheet_CN_V1.9.pdf)；Rev 1.9，发布/修订 2025-12-29 | [CW32L031_StandardPeripheralLib_V1.4.zip](https://www.whxy.com/uploads/files/20250721/CW32L031_StandardPeripheralLib_V1.4.zip)；Rev 1.4，发布/修订 2025-07-21 |
| CW32L052 | [CW32L052_UserManual_CN_V1.5.pdf](https://www.whxy.com/uploads/files/20240920/CW32L052_UserManual_CN_V1.5.pdf)；Rev 1.5，封面 2024-09，修订表 2024-07-24 | [CW32L052_DataSheet_CN_V1.3.pdf](https://www.whxy.com/uploads/files/20251229/CW32L052_DataSheet_CN_V1.3.pdf)；Rev 1.3，发布/修订 2025-12-29 | [CW32L052_StandardPeripheralLib_V1.4.zip](https://www.whxy.com/uploads/files/20260309/CW32L052_StandardPeripheralLib_V1.4.zip)；Rev 1.4，发布/修订 2026-03-09 |
| CW32L083 | [CW32L083_UserManual_CN_V2.0.pdf](https://www.whxy.com/uploads/files/20240920/CW32L083_UserManual_CN_V2.0.pdf)；Rev 2.0，封面 2024-09，修订表 2024-07-24 | [CW32L083_DataSheet_CN_V1.9.pdf](https://www.whxy.com/uploads/files/20251229/CW32L083_DataSheet_CN_V1.9.pdf)；Rev 1.9，发布/修订 2025-12-29 | [CW32L083_StandardPeripheralLib_V2.2.zip](https://www.whxy.com/uploads/files/20240821/CW32L083_StandardPeripheralLib_V2.2.zip)；Rev 2.2，发布/修订 2024-08-21 |
| CW32R031 | [CW32R031_UserManual_CN_V1.3.pdf](https://www.whxy.com/uploads/files/20240920/CW32R031_UserManual_CN_V1.3.pdf)；Rev 1.3，封面 2024-09，修订表 2024-07-25 | [CW32R031_DataSheet_CN_V1.2.pdf](https://www.whxy.com/uploads/files/20251230/CW32R031_DataSheet_CN_V1.2.pdf)；Rev 1.2，发布/修订 2025-12-30 | [CW32R031_StandardPeripheralLib_V1.1.zip](https://www.whxy.com/uploads/files/20240115/CW32R031_StandardPeripheralLib_V1.1.zip)；Rev 1.1，发布/修订 2023-11-06 |
| CW32W031 | [CW32W031_UserManual_CN_V1.4.pdf](https://www.whxy.com/uploads/files/20240920/CW32W031_UserManual_CN_V1.4.pdf)；Rev 1.4，封面 2024-09，修订表 2024-07-25 | [CW32W031_DataSheet_CN_V1.3.pdf](https://www.whxy.com/uploads/files/20251230/CW32W031_DataSheet_CN_V1.3.pdf)；Rev 1.3，发布/修订 2025-12-30 | [CW32W031_StandardPeripheralLib_V1.3.zip](https://www.whxy.com/uploads/files/20240119/CW32W031_StandardPeripheralLib_V1.3.zip)；Rev 1.3，发布/修订 2024-01-15 |

A030 没有独立选定 SDK/SVD。只在 `register-source-aliases.yaml` 明确允许的寄存器范围内借用 F030 SDK；A030 的封装、引脚和电气仍以自己的 DS 为准。

## SDK 内的 SVD 和芯片头到底来自哪里

下表路径均从对应 ZIP 根开始；`→` 表示先打开包内 `.pack`，再找其内部成员，不是拼接出来的目录。SVD 未附。本包 11 份可分发芯片主头位于 `approved-sdk-members/sdk-members/<ZIP 名去掉 .zip>/<原始成员路径>`；F002 主头未附。其他外设头文件/实现必须从原 ZIP 取得，YAML 的 `source_ref` 可在唯一来源锁内查到完整成员链。

| SDK 家族 | 芯片主头原始成员 | SVD 原始成员链 |
| --- | --- | --- |
| CW32F002 | `Libraries/inc/cw32f002.h` | `IdeSupport/EWARM/arm/config/debugger/WHXY/CW32F002.svd` |
| CW32F003 | `Libraries/inc/cw32f003.h` | `IdeSupport/MDK/WHXY.CW32F003_DFP.1.0.2.pack` → `SVD/CW32F003.svd` |
| CW32F020 | `Libraries/inc/cw32f020.h` | `IdeSupport/EWARM/arm/config/debugger/WHXY/CW32F020.svd` |
| CW32F030 | `Libraries/inc/cw32f030.h` | `IdeSupport/EWARM/arm/config/debugger/CW/CW32F030.svd` |
| CW32L010 | `CW32L010_StandardPeripheralLib_V1.0.9/Libraries/inc/cw32l010.h` | `CW32L010_StandardPeripheralLib_V1.0.9/IDEsupport/MDK/WHXY.CW32L010_DFP/SVD/CW32L010.svd` |
| CW32L011 | `Libraries/inc/cw32l011.h` | `IDEsupport/MDK/WHXY.CW32L011_DFP.1.0.1.pack` → `SVD/CW32L011.svd` |
| CW32L012 | `Libraries/inc/cw32l012.h` | `IDEsupport/MDK/WHXY.CW32L012_DFP.1.0.2.pack` → `SVD/CW32L012.svd` |
| CW32L031 | `Libraries/inc/cw32l031.h` | `IdeSupport/EWARM/arm/config/debugger/WHXY/CW32L031.svd` |
| CW32L052 | `CW32L052_StandardPeripheralLib_V1.4/Libraries/inc/cw32l052.h` | `CW32L052_StandardPeripheralLib_V1.4/IdeSupport/EWARM/arm/config/debugger/WHXY/CW32L052.svd` |
| CW32L083 | `Libraries/inc/cw32l083.h` | `IdeSupport/MDK/WHXY.CW32L083_DFP.1.0.9.pack` → `SVD/CW32L083.svd` |
| CW32R031 | `Libraries/inc/cw32r031.h` | `IdeSupport/MDK/WHXY.CW32R031_DFP.1.0.2.pack` → `SVD/CW32R031.svd` |
| CW32W031 | `Libraries/inc/cw32w031.h` | `IdeSupport/MDK/WHXY.CW32W031_DFP.1.0.2.pack` → `SVD/CW32W031.svd` |

例如 L010 的 ZIP 原来就含 `CW32L010_StandardPeripheralLib_V1.0.9/` 顶层目录，所以交付路径里 SDK 名会出现两次；这是保留原成员路径，并非多复制了一份。

## 已记录的型号、引脚和 DMA 页码

PDF 页从 1 起计；“书页”沿用印刷页码。以下各行使用上表同族原件，另标英文手册或历史 DS 的除外。pinouts 行只定位物理引脚表，不能替代 AF/模拟输入/比较器路由的独立事实。

| 芯片族 | `parts.yaml` / `additional-parts.yaml` | `pinouts/<family>.yaml` | `dma/<family>.yaml` |
| --- | --- | --- | --- |
| CW32A030 | DS：PDF 8, 29, 63；书页 62；表 Table 9-1 (minimum order quantities) | DS：PDF 22–25 | CW32x030_UserManual_EN_V1.0.pdf：PDF 98–99, 131, 148–149；书页 97–98, 130, 147–148；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32F002 | DS：PDF 8, 26, 58；书页 57；表 Table 9-1 (minimum order quantities) | DS：PDF 22–23 | DS：PDF 26；§6, Table 6-1: complete memory/peripheral map |
| CW32F003 | DS：PDF 8, 27, 62；书页 61；表 Table 9-1 (minimum order quantities) | DS：PDF 23–24 | DS：PDF 27；§6, Table 6-1: complete memory/peripheral map |
| CW32F020 | DS：PDF 8, 30, 69；书页 68；表 Table 9-1 (minimum order quantities) | DS：PDF 21, 23–26 | RM：PDF 94, 123, 138；书页 93, 122, 137；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32F030 | DS：PDF 8, 32, 75–77；书页 75；表 Table 9-1 (minimum order quantities) | DS：PDF 22, 24–28 | CW32x030_UserManual_EN_V1.0.pdf：PDF 98–99, 131, 148–149；书页 97–98, 130, 147–148；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32L010 | DS：PDF 8, 27, 66；书页 65；表 Table 9-1 (minimum order quantities) | DS：PDF 24–25 | DS：PDF 27；§6, Table 6-1: complete memory/peripheral map |
| CW32L011 | DS：PDF 8, 34, 71；书页 68；表 Table 9-1 (minimum order quantities) | DS：PDF 29–31 | DS：PDF 34；§6, Table 6-1: complete memory/peripheral map |
| CW32L012 | DS：PDF 8, 41, 81；书页 78；表 Table 9-1 (minimum order quantities) | DS：PDF 35–38 | RM：PDF 96, 133, 147；书页 70, 107, 121；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32L031 | DS：PDF 10, 32, 78；书页 77；表 Table 9-1 (minimum order quantities) | DS：PDF 26–29 | RM：PDF 91, 120, 134；书页 90, 119, 133；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32L052 | DS：PDF 10, 36, 79；书页 78；表 Table 9-1 (minimum order quantities) | DS：PDF 26–30 | RM：PDF 96, 125, 139；书页 95, 124, 138；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32L083 | DS：PDF 10, 40, 86；书页 85；表 Table 9-1 (minimum order quantities) | DS：PDF 27–33 | RM：PDF 105, 135, 148–150；书页 104, 134, 147–149；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32R031 | DS：PDF 11, 35, 73；书页 72；表 Table 9-1 (minimum order quantities) | DS：PDF 29–31 | RM：PDF 93, 122, 136；书页 92, 121, 135；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |
| CW32W031 | DS：PDF 9, 34, 71；书页 70；表 Table 9-1 (minimum order quantities) | DS：PDF 27–30 | RM：PDF 92, 121, 135；书页 91, 120, 134；§5.4, Table 5-1; 8.2; 8.8.4; 8.8.4, DMA_TRIGy.HARDSRC[7:2] |

## 当前异步 ADC 的补充依据

13 个芯片族的软件触发有限单次/扫描 async 包括 L010/L011 的 ADC、L012 的 ADC1/ADC2，以及 F002/F003/F020/F030/A030/L031/L052/L083/R031/W031。ADC2 单独检查有效时钟/复位域中的 DAC DMAUDRIE1/2 均为零，保留正常直接装载 DAC 输出；共享 ADC2_DAC 的整线禁用只用于无法确认本地完成中断已屏蔽的故障。ADC1 的独立向量依据保持不变。现行说明见 [adc-remaining-async.md](../docs/adc-remaining-async.md) 与 [ADC2 契约](../docs/adc-l012-adc2-async.md)，精确来源 ID 与逐项页码分别见 [原有记录](../docs/adc-remaining-async-evidence.json) 和 [ADC2 新记录](../docs/adc-l012-adc2-async-evidence.json)。这些记录只使用已经锁定的自身手册、SVD 和 SDK 成员，不新增原厂文件或 SDK 示例依赖。Stage91 main 通过5个独立ARM库和10个ELF链接，共16次尝试；保留首次E1超出64 KiB Flash 1,088字节的失败以及仅示例 release 优化 s→z 的已审修正，原5个库的实际输入不变。L4保留3条既有L010 RCC警告，没有新警告。两个有限数据/来源门通过，早先导航证据定位键失败及修正另有记录；910个生成文件未变，main未重新生成硬件。main验收时clean 2库/3ELF和final54最终报告绑定尚未执行；54选择语义核对已有自身记录，最终完成仍需另行绑定clean和final54收据，不声明实板结果。

下表从各族当前记录摘出专用 ADC IRQ12、MODE0 单次、MODE4 扫描及 START/IER/ISR/ICR 的页码；均为“PDF / 书页”。版本和文件名沿用上方各族原件表；F030/A030 明确共用 x030 RM，其余逐族使用自己的 RM。

| 芯片族 / RM 版本 | IRQ（§5.4） | MODE0 / MODE4 | START、IER / ISR / R1W0 ICR |
| --- | --- | --- | --- |
| F030、A030 / x030 CN Rev 2.5 | 96 / 95 | 440 / 439；447 / 446 | 467 / 466；468 / 467；469 / 468 |
| F002 / CN Rev 1.4 | 72 / 71 | 307 / 306；314 / 313 | 332 / 331；333 / 332；334 / 333 |
| F003 / CN Rev 2.3 | 74 / 73 | 367 / 366；374 / 373 | 394 / 393；395 / 394；396 / 395 |
| F020 / CN Rev 1.4 | 94 / 93 | 378 / 377；385 / 384 | 405 / 404；406 / 405；407 / 406 |
| L031 / CN Rev 1.6 | 91 / 90 | 437 / 436；444 / 443 | 466 / 465；467 / 466；468 / 467 |
| L052 / CN Rev 1.5 | 96 / 95 | 474 / 473；481 / 480 | 506 / 505；507 / 506；508 / 507 |
| L083 / CN Rev 2.0 | 105 / 104 | 479 / 478；486 / 485 | 508 / 507；509 / 508；510 / 509 |
| R031 / CN Rev 1.3 | 93 / 92 | 440 / 439；447 / 446 | 468 / 467；469 / 468；470 / 469 |
| W031 / CN Rev 1.4 | 92 / 91 | 440 / 439；447 / 446 | 469 / 468；470 / 469；471 / 470 |

L012 使用 `CW32L012_UserManual_CN_V1.4.pdf` Rev 1.4：§5.4 IRQ 表 PDF 96–97 / 书页 70–71；§25.5.2 有限扫描 607–608 / 581–582；§25.5.1 与 §25.12.2 停止/游标复位 605、616 / 579、590；§25.10、§25.12.8–10 中断访问 613、622–623 / 587、596–597；结果/BGR 624–625 / 598–599；DAC 共享中断源 §26.8 表 26-2 为 633 / 607。经典系列 START=0 只采用其手册所述停止语义，不移植 L010/L011 的显式游标复位结论；这批功能均未据此承诺独立排空握手、最长模拟中止延迟或硬件实测结果。

ADC2 共享向量的补充定位：CN RM1.4 的 IRQ30 为书页71/PDF97；DAC 两个 DMA 欠载中断源为607/633，CR0 的 DMAUDRIE1/2 位13/29及完整控制字段为609–611/635–637，DOR1/2 的 DMAUDR 位15是 R1W0、DATA 为只读（615/641），CR1 外部输出开关为616/642。DAC APBEN2/APBRST2 位10分别见59/85及63/89；ADC1/2 共用 APBEN1/APBRST1 位0见57–58/83–84及61–62/87–88。当前 EN RM1.0 的对应 IRQ、DAC 中断/控制/状态/输出、DAC 门/复位及 ADC 共用域分别为76/102、663/689、665–668/691–694、672/698、673/699、64/90、68/94、62–63/88–89及66–67/92–93。当前英文原件 SHA-256 为 `f56d5ed899dd090b469fac6a09084aab9f5068ae3b56cf7562b83997bd2f1088`，不替换成同名旧版本。

实际 IE 零值检查保留掩蔽的 DAC DMAUDR 标志，不要求 EN/OUT/TEN/WAVE/DMAEN 为零，也不写任何 DAC 控制、数据或状态。DAC 时钟门开启后可能推进保留活动属于保守功能边界；EN 字段下的时钟说明不能等同于明确的 APBEN2→DOR 转移保证。当前 safe DAC/OPA/RCC/DMA 调用保持零 IE 的结论属于软件写集合审阅，不能编码成硬件事实；未来 DAC 中断接口必须重新处理共享调度与故障策略。硬件证据记录与运行/构建验收分开保存。

## 已记录的外设引文入口

这部分摘取明确归属原件的页码。它是查阅入口，不是对每条字段/电气值的完整覆盖证明；详细 claim 仍应保留在相邻 YAML 或其明确引用记录中。这里只复用已记录的定位，本次没有重新逐页校读 PDF。

| YAML / 芯片族 | 原件定位 |
| --- | --- |
| `af/cw32f030-atim-complementary.yaml`、`af/cw32a030-atim-complementary.yaml` / 互补 PWM | 共享 RM `CW32x030_UserManual_CN_V2.5.pdf` Rev 2.5：控制/预装载/时钟与命令语义书页 253–255、268、271、277–278、295–307（PDF 254–256、269、272、278–279、296–308）；死区公式 PDF 279 / 书页 278，DTR 表 PDF 306 / 书页 305，ICR R1W0 为 PDF 301 / 书页 300。死区采用 TCLK=PCLK/PRS；最大 1010 ticks；第四段下界按公式和 DTR 表取 514 ticks，保留正文 63.25 µs 与正确 64.25 µs 的冲突。范围仅 F030/A030 三组完整 A+B 对、固定死区与全局 MOE，不推出 BK、逐对门控或制动恢复保证。 |
| 同上 / F030 与 A030 各自 AF 和封装 | `CW32F030_DataSheet_CN_V1.9.pdf` Rev 1.9：表 5-3/5-4 PDF 29–30 / 书页 28–29，已用封装单元格 PDF 25–27；`CW32A030_DataSheet_CN_V1.1.pdf` Rev 1.1：表 5-3/5-4 PDF 26–27 / 书页 25–26，封装单元格 PDF 23–24。共享 x030 CN Rev 2.5 RM 路由单元格 PDF 147–148 / 书页 146–147；F030 SDK V2.2 的 `Libraries/inc/cw32f030_gpio.h` 宏行 833、892、900、908、924、932、1003、1011、1019 作佐证。两族各 9 条 CH1B–3B 路由，精确封装投影和原件哈希保留在 YAML；不能把 F030 封装套到 A030。 |
| `register-writes.yaml` / classic ATIM ICR | 原件 R1W0 页码：x030 CN Rev 2.5 PDF 301 / 书页 300；F003 CN Rev 2.3 234 / 233；L031 CN Rev 1.6 292 / 291；L052 CN Rev 1.5 329 / 328；L083 CN Rev 2.0 341 / 340；R031 CN Rev 1.3、W031 CN Rev 1.4 均 295 / 294。各自完整 RM 文件名见上表；三种既有 ATIM 寄存器变体的命令旁表保留逐项直接引文，不扩展这些族的互补 PWM HAL 范围。 |
| `dma/cw32f030.yaml`、`dma/cw32a030.yaml` / staged UART、SPI DMA | 当前行为复核使用 `CW32x030_UserManual_CN_V2.5.pdf` Rev 2.5：§8.4.1、§8.4.4 表 8-2、§8.5–8.6、§8.8.2–3 为 PDF 128、131–133、135–136、139–140 / 书页 127、130–132、134–135、138–139；UART TX/RX §18.6、§18.7.1.5–6 为 PDF 345、350–351 / 书页 344、349–350，接收/ICR 为 PDF 339、359–361 / 书页 338、358–360；SPI 成对字节流程 §19.6.1.3 为 PDF 382–383 / 书页 381–382，状态/错误/请求为 PDF 376–379、389–394 / 书页 375–378、388–393。YAML 原有 EN V1.0 路由页码仍保留，不能改标成 CN V2.5。 |
| `dma/cw32l083.yaml` / staged UART1–6、SPI1–2 DMA | 独立依据 `CW32L083_UserManual_CN_V2.0.pdf` Rev 2.0：§8.4.4/8.5 为 PDF 141、143 / 书页 140、142；§8.6/8.8.3–4 为 PDF 144、148–150 / 书页 143、147–149；§5.4 IRQ 表为 PDF 105–106 / 书页 104–105；UART §19.6/19.7.1.5–6 为 PDF 385、390–391 / 书页 384、389–390，接收/ICR 为 PDF 379、384、401 / 书页 378、383、400；SPI §20.5/20.6.1.3 为 PDF 418、421–422 / 书页 417、420–421，错误/破坏性操作为 PDF 416–417、432–433 / 书页 415–416、431–432。SDK V2.2 `Libraries/inc/cw32l083_dma.h` 仅核对选择码，不从 x030 或软件搬运支持外推外设 DMA。 |
| `hex-qualified.yaml` / CW32F002 | RM：PDF 42, 47, 54, 55, 57, 129；书页 41, 46, 53, 54, 56, 128 / DS：PDF 22, 30, 31, 36, 37, 40；书页 21, 29, 30, 35, 36, 39 |
| `hex-qualified.yaml` / CW32F003 | RM：PDF 44, 49, 56, 57, 59, 131；书页 43, 48, 55, 56, 58, 130 / DS：PDF 23, 31, 32, 37, 38, 41；书页 22, 30, 31, 36, 37, 40 |
| `dma/cw32l083.yaml` / CW32L083，软件 BLOCK 补充 | RM：§§8.4.1–8.4.2、8.5 PDF 137–139, 143 / 书页 136–138, 142；§§8.6、8.8.3、8.8.4 PDF 144, 148, 150 / 书页 143, 147, 149；§§2.3、6.1–6.3 PDF 33, 115–116 / 书页 32, 114–115；§§4.7.12、4.7.15 PDF 86, 90 / 书页 85, 89。仅定位软件搬运、成功完成/SRAM/共享门控证据，不把 EN 清零或 TE 当作总线排空证明。 |
| `adc-sequences.yaml` / CW32L010 | RM：PDF 507–510, 517–519, 523–526；书页 506–509, 516–518, 522–525 |
| `adc-sequences.yaml` / CW32L011 | RM：PDF 509–512, 518–520, 525–528；书页 508–511, 517–519, 524–527 |
| `adc-sequences.yaml` / CW32L012 | DS：PDF 63–64, 66；书页 60–61, 63 / RM：PDF 601–608, 611–612, 615–617, 623–625；书页 575–582, 585–586, 589–591, 597–599 |
| `classic-adc-scans.yaml` / CW32A030, CW32F030 | RM：PDF 436, 438, 447–448, 460–464, 467–470, 473；书页 435, 437, 446–447, 459–463, 466–469, 472；§22.12; 22.13 ADC_ICR; 22.13 ADC_ISR; 22.13 ADC_RESULT0-ADC_RESULT3; 22.13 ADC_START/ADC_IER; 22.13.1; 22.13.2; 22.13.4; 22.4.1; 22.4.3-22.4.4; 22.5.5; 23.3.1 |
| `classic-adc-scans.yaml` / CW32F002 | RM：PDF 303, 305, 314–315, 326–329, 332–335；书页 302, 304, 313–314, 325–328, 331–334；§19.11; 19.12 ADC_ICR; 19.12 ADC_ISR; 19.12 ADC_RESULT0-ADC_RESULT3; 19.12 ADC_START/ADC_IER; 19.12.1; 19.12.2; 19.12.4; 19.4.1; 19.4.3-19.4.4; 19.5.5 |
| `classic-adc-scans.yaml` / CW32F003 | RM：PDF 363, 365, 374–375, 387–391, 394–397, 400；书页 362, 364, 373–374, 386–390, 393–396, 399；§20.12; 20.13 ADC_ICR; 20.13 ADC_ISR; 20.13 ADC_RESULT0-ADC_RESULT3; 20.13 ADC_START/ADC_IER; 20.13.1; 20.13.2; 20.13.4; 20.4.1; 20.4.3-20.4.4; 20.5.5; 21.3.1 |
| `classic-adc-scans.yaml` / CW32F020 | RM：PDF 374, 376, 385–386, 398–402, 405–408, 411；书页 373, 375, 384–385, 397–401, 404–407, 410；§21.12; 21.13 ADC_ICR; 21.13 ADC_ISR; 21.13 ADC_RESULT0-ADC_RESULT3; 21.13 ADC_START/ADC_IER; 21.13.1; 21.13.2; 21.13.4; 21.4.1; 21.4.3-21.4.4; 21.5.5; 22.3.1 |
| `classic-adc-scans.yaml` / CW32L031 | RM：PDF 433, 435, 444–445, 458–463, 466–470, 473；书页 432, 434, 443–444, 457–462, 465–469, 472；§22.12; 22.13 ADC_ICR; 22.13 ADC_ISR; 22.13 ADC_RESULT0-ADC_RESULT7; 22.13 ADC_START/ADC_IER; 22.13.1; 22.13.2; 22.13.4-22.13.5; 22.4.1; 22.4.3-22.4.4; 22.5.5; 23.3.1 |
| `classic-adc-scans.yaml` / CW32L052 | RM：PDF 470, 472, 481–482, 496–501, 506–510, 513；书页 469, 471, 480–481, 495–500, 505–509, 512；§23.12; 23.13 ADC_ICR; 23.13 ADC_ISR; 23.13 ADC_RESULT0-ADC_RESULT7; 23.13 ADC_START/ADC_IER; 23.13.1; 23.13.2; 23.13.4-23.13.5; 23.4.1; 23.4.3-23.4.4; 23.5.5; 24.3.1 |
| `classic-adc-scans.yaml` / CW32L083 | RM：PDF 475, 477, 486–487, 500–505, 508–512, 515；书页 474, 476, 485–486, 499–504, 507–511, 514；§23.12; 23.13 ADC_ICR; 23.13 ADC_ISR; 23.13 ADC_RESULT0-ADC_RESULT7; 23.13 ADC_START/ADC_IER; 23.13.1; 23.13.2; 23.13.4-23.13.5; 23.4.1; 23.4.3-23.4.4; 23.5.5; 24.3.1 |
| `classic-adc-scans.yaml` / CW32R031 | RM：PDF 436, 438, 447–448, 460–465, 468–472, 475；书页 435, 437, 446–447, 459–464, 467–471, 474；§22.12; 22.13 ADC_ICR; 22.13 ADC_ISR; 22.13 ADC_RESULT0-ADC_RESULT7; 22.13 ADC_START/ADC_IER; 22.13.1; 22.13.2; 22.13.4-22.13.5; 22.4.1; 22.4.3-22.4.4; 22.5.5; 23.3.1 |
| `classic-adc-scans.yaml` / CW32W031 | RM：PDF 436, 438, 447–448, 461–466, 469–473, 476；书页 435, 437, 446–447, 460–465, 468–472, 475；§22.12; 22.13 ADC_ICR; 22.13 ADC_ISR; 22.13 ADC_RESULT0-ADC_RESULT7; 22.13 ADC_START/ADC_IER; 22.13.1; 22.13.2; 22.13.4-22.13.5; 22.4.1; 22.4.3-22.4.4; 22.5.5; 23.3.1 |
| `rtc-alarms.yaml` / CW32A030, CW32F030 | RM：PDF 179, 182, 188–189, 191–192, 194–196；书页 178, 181, 187–188, 190–191, 193–195 |
| `rtc-alarms.yaml` / CW32F020 | RM：PDF 176, 179, 185–186, 188–189, 191–193；书页 175, 178, 184–185, 187–188, 190–192 |
| `rtc-alarms.yaml` / CW32L010, CW32L011 | RM：PDF 143, 146, 152–153, 156–160；书页 142, 145, 151–152, 155–159 |
| `rtc-alarms.yaml` / CW32L012 | RM：PDF 194, 197, 202–203, 206–210；书页 168, 171, 176–177, 180–184 |
| `rtc-alarms.yaml` / CW32L031 | RM：PDF 171, 174, 180–181, 183–184, 186–188；书页 170, 173, 179–180, 182–183, 185–187 |
| `rtc-alarms.yaml` / CW32L052 | RM：PDF 184, 187, 193–194, 196–197, 199–201；书页 183, 186, 192–193, 195–196, 198–200 |
| `rtc-alarms.yaml` / CW32L083 | RM：PDF 196, 199, 205–206, 208–209, 211–213；书页 195, 198, 204–205, 207–208, 210–212 |
| `rtc-alarms.yaml` / CW32R031 | RM：PDF 173, 176, 182–183, 185–186, 188–190；书页 172, 175, 181–182, 184–185, 187–189 |
| `rtc-alarms.yaml` / CW32W031 | RM：PDF 172, 175, 181–182, 184–185, 187–189；书页 171, 174, 180–181, 183–184, 186–188 |
| `ram-parity.yaml` / CW32A030, CW32F030 | DS：PDF 4, 9；书页 3, 8 / RM：PDF 106–111；书页 105–110 |
| `ram-parity.yaml` / CW32F002 | DS：PDF 4, 9；书页 3, 8 / RM：PDF 82–87；书页 81–86 |
| `ram-parity.yaml` / CW32F003 | DS：PDF 4, 9；书页 3, 8 / RM：PDF 84–89；书页 83–88 |
| `ram-parity.yaml` / CW32F020 | DS旧版：PDF 4, 9；书页 3, 8 / RM：PDF 104–109；书页 103–108 |
| `ram-parity.yaml` / CW32L010 | DS：PDF 4, 9；书页 3, 8 / RM：PDF 99–103；书页 98–102 |
| `ram-parity.yaml` / CW32L011 | DS：PDF 4, 10；书页 1, 7 / RM：PDF 97–101；书页 96–100 |
| `ram-parity.yaml` / CW32L012 | DS：PDF 4, 10；书页 1, 7 / RM：PDF 106–111；书页 80–85 |
| `ram-parity.yaml` / CW32L031 | DS：PDF 5, 11；书页 4, 10 / RM：PDF 101–106；书页 100–105 |
| `ram-parity.yaml` / CW32L052 | DS：PDF 5, 11；书页 4, 10 / RM：PDF 106–111；书页 105–110 |
| `ram-parity.yaml` / CW32L083 | DS：PDF 5, 11；书页 4, 10 / RM：PDF 115–120；书页 114–119 |
| `ram-parity.yaml` / CW32R031 | DS：PDF 5, 12；书页 4, 11 / RM：PDF 103–108；书页 102–107 |
| `ram-parity.yaml` / CW32W031 | DS：PDF 4, 10；书页 3, 9 / RM：PDF 102–107；书页 101–106 |
| `reference-dividers.yaml` / CW32L010 | DS：PDF 52；书页 51 / RM：PDF 80, 83, 529, 534–536；书页 79, 82, 528, 533–535 |
| `reference-dividers.yaml` / CW32L011 | DS：PDF 58；书页 55 / RM：PDF 78, 81, 531, 536–538；书页 77, 80, 530, 535–537 |
| `reference-dividers.yaml` / CW32L012 | DS：PDF 66；书页 63 / RM：PDF 83–84, 87, 645–646, 651–653；书页 57–58, 61, 619–620, 625–627 |
| `accelerators.yaml` / CW32L012 | RM 书页 156–157；Q1.31 数学加速器定义域。未记录直接 PDF 页码。 |
| `dac-opa.yaml` / CW32L012 | DS §7.3.14 表 7-30 书页 62、§7.3.18 表 7-34 书页 65；RM §26.2 书页 601、§25.12.19 书页 599。这里保留 YAML 自由文本页码，未换算 PDF 页码。 |
| `crypto.yaml` / CW32L083 | RM §§26.1–26.6 书页 536–541（TRNG）、§§28.1–28.5 书页 565–570（AES）；DS §§4.20–4.21 书页 21–22。 |
| `lcd.yaml` / CW32L052 | RM §§26.3.2–26.3.7; 26.4; 26.6–26.7; 4.3.6, 4.5.5, 4.7.2, 4.7.5 (LSI enable, stable and prohibition on live trim changes)；DS §§5.2 Table 5-2; 7.3.8 Table 7-18; 7.3.17 Table 7-33 |
| `lcd.yaml` / CW32L083 | RM §§27.3.2–27.3.7; 27.4; 27.6–27.7; 4.3.6, 4.5.5, 4.7.2, 4.7.5 (LSI enable, stable and prohibition on live trim changes)；DS §§5.2 Table 5-2; 7.3.8 Table 7-18; 7.3.18 Table 7-34 |
| `hse-qualified.yaml`、`registers/sysctrl_cw32l010_v1.yaml`、`field-access.yaml` / CW32L010 | RM `CW32L010_UserManual_CN_V1.2.pdf` Rev 1.2：HSE/HSI/CCS 为 PDF 49–59、67–73 / 书页 48–58、66–72；HSE 原生字段与 STABLE RO 为 PDF 73 / 书页 72；故障/就绪标志为 PDF 76–78 / 书页 75–77。Flash 为 PDF 114 / 书页 113；GPIO 为 PDF 126 / 书页 125；RTC/AWT 为 PDF 148–153 / 书页 147–152；ADC/BGR 为 PDF 504、513、517 / 书页 503、512、516；VC 输入与 PCLK blanking 为 PDF 537–539 / 书页 536–538；LVD 为 PDF 543、547–548 / 书页 542、546–547。DS `CW32L010_DataSheet_CN_V1.3.pdf` Rev 1.3：OSC_IN PA0 / OSC_OUT PA1、PA0 与 VC1_CH1 交叉为表5-2 PDF24 / 书页23；供电/HSE/HSI/LSI 电气表为 PDF32、39–43 / 书页31、38–42。SDK V1.0.9 的 `Libraries/inc/cw32l010_sysctrl.h`、`Libraries/src/cw32l010_sysctrl.c` 只佐证；驱动码8–15未资格。初期旁路4–32MHz是对原件1–32MHz的明确软件收窄，无 FREQRANGE/PDR，继承CCS强制4MHz回退。 |
| `hse-qualified.yaml`、`registers/sysctrl_cw32l011_v1.yaml`、`field-access.yaml` / CW32L011 | RM `CW32L011_UserManual_CN_V1.1.pdf` Rev 1.1（选定2026-06-02上传字节）：HSE/HSI/CCS 为 PDF47–57、65–71 / 书页46–56、64–70；HSE 原生字段与 STABLE RO 为 PDF71 / 书页70；故障/就绪标志为 PDF74–76 / 书页73–75。Flash 为 PDF113 / 书页112；GPIO 为 PDF126 / 书页125；RTC/AWT 为 PDF148–153 / 书页147–152；ADC/BGR 为 PDF506、518 / 书页505、517；VC 输入/blanking 为 PDF539–541 / 书页538–540；LVD 为 PDF545、549–550 / 书页544、548–549。DS `CW32L011_DataSheet_CN_V1.1.pdf` Rev 1.1：OSC_IN PC13 / OSC_OUT PB7、无VC输入交叉为表5-2 PDF31 / 书页28；供电/HSE/HSI/LSI 为 PDF39、47–51 / 书页36、44–48。SDK V1.0.3 的 `Libraries/inc/cw32l011_sysctrl.h`、`Libraries/src/cw32l011_sysctrl.c` 只佐证。旁路取RM4MHz与DS1MHz下限的4–32MHz交集；LSI RM±10%与DS−10/+25%冲突保留，检测器算术用41kHz上界；无 FREQRANGE/PDR，继承CCS强制4MHz回退。 |
| `hse-qualified.yaml` / CW32L031 | RM `CW32L031_UserManual_CN_V1.6.pdf` Rev 1.6：§4.3.3（HSE）、§4.4.3（稳定/启动失败/运行失效）、§4.7（寄存器）、§11.8.1（AWT 来源）；PDF 49–50、56–59、67–68、71–72、165，书页 48–49、55–58、66–67、70–71、164。DS `CW32L031_DataSheet_CN_V1.9.pdf` Rev 1.9：§7.3.1（运行条件）、§7.3.7（外部时钟）、§7.3.8（保留 HSI）；PDF 26、36、38、44、46–47、50，书页 25、35、37、43、45–46、49。QFN20 无 PF0/PF1。 |
| `hse-qualified.yaml` / CW32R031 | RM `CW32R031_UserManual_CN_V1.3.pdf` Rev 1.3：§4.3.3、§4.4.3、§4.7、§11.8.1；PDF 51–52、58–61、69–70、73–74、167，书页 50–51、57–60、68–69、72–73、166。DS `CW32R031_DataSheet_CN_V1.2.pdf` Rev 1.2：§7.3.1、§7.3.8（外部时钟）、§7.3.9（保留 HSI）；PDF 29、42、51、53–54，书页 28、41、50、52–53。HSE 供电范围 2.2–3.6 V，不据此配置 RF。 |
| `hse-qualified.yaml` / CW32W031 | RM `CW32W031_UserManual_CN_V1.4.pdf` Rev 1.4：§4.3.3、§4.4.3、§4.7、§11.8.1；PDF 50–51、57–60、68–69、72–73、166，书页 49–50、56–59、67–68、71–72、165。DS `CW32W031_DataSheet_CN_V1.3.pdf` Rev 1.3：§7.3.1、§7.3.8（外部时钟）、§7.3.9（保留 HSI）；PDF 30、41、50、52–53，书页 29、40、49、51–52。HSE 保守取 LDO/DCDC 交集 2.0–3.6 V，未扩大 RF 范围，也未缩窄既有 HSI-only 条件。 |
| `hse-qualified.yaml` / CW32L052 | RM `CW32L052_UserManual_CN_V1.5.pdf` Rev 1.5：HSE/预启动参数书页 51–52、74–75（PDF 52–53、75–76），CCS/选择器书页 61、69–70（PDF 62、70–71），Flash 书页 111（PDF 112），AUTOTRIM/RTC/LVD 保留来源书页 174、192、532（PDF 175、193、533）。DS `CW32L052_DataSheet_CN_V1.3.pdf` Rev 1.3：引脚表书页 25（PDF 26），运行条件书页 42（PDF 43），外部时钟/波形/HSI 书页 47、49–50、53（PDF 48、50–51、54）。三种已建模精确封装均有 PF0/PF1。保留旁路 RM 4–32 MHz 与 DS 1–32 MHz 的交集；CLKCCS 固定 HSI/6（标称 8 MHz），不继承 L083 的配置分频回退。自身 SOURCE 枚举、原始锁和完整限制见 [L052 HSE](../docs/qualified-l052-hse.md) 及其[来源记录](../docs/qualified-l052-hse-source-receipt.json)。 |
| `hse-qualified.yaml` / CW32L083 | RM `CW32L083_UserManual_CN_V2.0.pdf` Rev 2.0：HSE/启动 PDF 53–54、62–63、65、70、80 / 书页 52–53、61–62、64、69、79；选择器/CCS/切换 PDF 63、67–69、75–76、79–80、84–86 / 书页 62、66–68、74–75、78–79、83–85；Flash PDF 121、131 / 书页 120、130；GPIO PDF 162–169 / 书页 161–168；RTC/AUTOTRIM/LVD 依赖 PDF 87、89、178、187、205–206、530、534–535 / 书页 86、88、177、186、204–205、529、533–534。DS `CW32L083_DataSheet_CN_V1.9.pdf` Rev 1.9：PF0/PF1 封装表 PDF 27 / 书页 26，电气 PDF 47、52、54–55、58 / 书页 46、51、53–54、57。保留 4–32 MHz 的 RM/DS 交集与独立波形条件；不外推 L052 HSE 或独立 PLL_OUT；本族当前有界系统 PLL 见末节。 |
| `registers/rtc_cw32l031_v1.yaml` / SOURCE 枚举 | 既有完整布局复用组恰为 L031/L083/R031/W031。各自 RM §12.5.3 分别是 Rev 1.6 PDF 180 / 书页 179、Rev 2.0 PDF 205 / 书页 204、Rev 1.3 PDF 182 / 书页 181、Rev 1.4 PDF 181 / 书页 180；均给出 LSE=0、LSI=2、HSE/128,/256,/512,/1024=4,5,6,7。只补类型名，未改变布局、访问属性或复用成员。 |

其他直接引文：`af/*` 的 `datasheet_cell`、`manual_cell`、`manual_pin_cell` 等记录逐路由页码/表格；`triggers/*` 保留逐触发关系的原件记录；`gpio-interrupt.yaml` 有每族 ICR 的手册章节/页码；`hse-qualified.yaml` 有 F020/F030/A030/L031/L052/L083/R031/W031 自己的 RM/DS 页码。它们均对应上表同族原件，原 SDK GPIO/外设头文件由同族 ZIP 提供。

上述 staged DMA 的完整出处与适用条件见 [UART TX](../docs/uart-dma-tx.md)、[UART RX](../docs/uart-dma-rx.md)、[SPI](../docs/spi-dma.md) 和 [L083 独立资格](../docs/l083-peripheral-dma.md)。干净 TC 与 Complete 状态支持成功完成路径；EN 清零、TE 或请求门关闭本身不是总线排空证明。互补 PWM 的精确来源见 [classic ATIM 说明](../docs/classic-atim-complementary-pwm.md)，L083 HSE 逐项来源见 [来源记录](../docs/qualified-l083-hse-source-receipt.json)。这些项目记录是查阅索引，不是原厂资料。

## 补充、历史和非芯片原件

| 原件 | 版本及用途 |
| --- | --- |
| [CW32F020_DataSheet_CN_V1.3.pdf](https://www.whxy.com/uploads/files/20250821/CW32F020_DataSheet_CN_V1.3.pdf) | Rev 1.2，发布/修订 2023-02-14。历史原件：文件名 V1.3，但内页 Rev 1.2，不能写成 Rev 1.3；仍被旧 AF/ADC/RAM 审阅引文使用。 |
| [CW32x030_UserManual_EN_V1.0.pdf](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_EN_V1.0.pdf) | Rev 1.0，发布/修订 2022-08-23。补充语言版本，不能替代同族当前选定中文手册的行为修订。 |
| [a030-official-manuals.html](https://www.whxy.com/tongyonggaoxingnengMCU/CW32A030C8T7.html?act=doc&cid=21) | 官网 A030 文档/SDK 分类页的固定捕获；不是产品手册。 |
| [a030-official-sdk.html](https://www.whxy.com/tongyonggaoxingnengMCU/CW32A030C8T7.html?act=doc&cid=22) | 官网 A030 文档/SDK 分类页的固定捕获；不是产品手册。 |
| [NXP_UM10204_Rev7.pdf](https://cache.nxp.com/docs/en/user-guide/UM10204.pdf) | Rev 7.0，封面 2021-10-01，修订表 2021-10-01。I²C 总线协议外部标准，不是 CW32 寄存器手册。 |
| [CW32L012_UserManual_EN_V1.0.pdf](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_EN_V1.0.pdf) | Rev 1.0，当前封面 2026-06，修订表仍为 2026-01-16。同一官方 URL 已替换原 January 封面版本；旧身份与有限迁移见 docs/l012-english-source-update.json。补充语言版本，不能替代同族当前选定中文手册的行为修订。 |
| [CW32L052_UserManual_EN_V1.0.pdf](https://www.whxy.com/uploads/files/20240920/CW32L052_UserManual_EN_V1.0.pdf) | Rev 1.0，发布/修订 2023-06-20。补充语言版本，不能替代同族当前选定中文手册的行为修订。 |
| [CW32L083_UserManual_EN_V1.0.pdf](https://www.whxy.com/uploads/files/20240923/CW32L083_UserManual_EN_V1.0.pdf) | Rev 1.0，发布/修订 2022-10-10。补充语言版本，不能替代同族当前选定中文手册的行为修订。 |

CW32L011 中文 Rev 1.1 曾有封面 2025-09 的旧字节快照；本页选定的是封面 2026-06、SHA-256 `b245887e…` 的版本。旧快照未作为现行 YAML 输入，不纳入上述原件清单。文件名或相同 Rev 号不足以替代 SHA-256。

## 仍需补足的引用粒度

- `registers/*.yaml` 共 137 份可通过既有映射定位官方 SVD 基线及寄存器/字段名。需要补的是尚未附明确 RM 出处的人工修正，不要求正常 SVD 字段重复抄一份手册页码。`register-reuse.yaml` 的相等性不证明每个人工修正。
- `clock/*.yaml` 共 13 份已有原件/派生文本哈希，但大部分 claim 使用文本行号，没有对应原 PDF 页码；`source_ref` 只能定位文件，不能当章节。
- `af/cw32f002.yaml`、`af/cw32f003.yaml`、`af/cw32f020.yaml`、`af/cw32f020-serial.yaml`、`af/cw32l010.yaml`、`af/cw32l011.yaml`、`af/cw32l012.yaml`、`af/cw32l031.yaml`、`af/cw32l052.yaml`、`af/cw32l083.yaml`、`af/cw32r031.yaml`、`af/cw32w031.yaml` 在文件内只列 SDK GPIO 宏/行号，缺独立 RM/DS 页码。不能与已逐格核对的同族 `*-serial`、`*-pwm` 等路由混同。
- `register-writes.yaml` 的 classic ATIM ICR 已补同族 RM 直接页码，其余多数命令语义仍只指项目审阅记录；`field-access.yaml` 的 RAM 项只记 §6.6.2，计时器 UIFCPY 项已有明确页码。没有直接页码的条目应回填到相邻 YAML。
- `adc-sequences.yaml`、`classic-adc-scans.yaml`、`classic-timer-input.yaml`、`rtc-calendar.yaml`、`rtc-alarms.yaml`、`ram-parity.yaml`、`lcd.yaml`、`lvd-ir.yaml`、`spi.yaml` 目前主要通过项目审阅记录间接归属原件；上表只能帮助打开原页。精简这些旧记录前，应先把所需 claim 级原件定位并回 YAML。
- `electrical.yaml` 大部分 ADC、时钟、Flash、I²C、IWDT 限值仍经 `policies` 间接引用项目 JSON。少数 factory trim SDK 行号和 HSE 页码不等于所有值都有直接引文。
- `accelerators.yaml`、`dac-opa.yaml` 有原件版本与书页；前者缺直接 PDF 页码，后者部分定位仍是自由文本。
- SDK F003 V1.7、L011 V1.0.3、L012 V1.0.5 目前仅确认 ZIP 文件名版本；未建立内部发布级版本/日期。不能拿某个头文件或 SVD 的版本替代 SDK 发布版本。

## 维护与获取

保留既有 `evidence-sources.json` 一份 URL/hash/member 锁；不新增每族 JSON，也不把项目审计记录冒充原厂资料。下载使用该锁的精确 URL 和 SHA-256；`sources/vendor/` 是获取后的本地缓存目标，不表示本补充包附带完整原件。

`catalog.json` 是已记录的网页产品快照，`layout-history.json` 是旧路径解释记录，均不是寄存器原件。它们仍被维护工具读取，本次整合不改写或新增副本。后续移入维护档案时须同步现有读取路径。

## Inherited LSE pad ownership source additions

`cw32-data/lse-ownership.yaml` adds read-only inherited ownership facts, not LSE
initialization. The original paths below resolve through the existing source
lock, which retains exact URLs and hashes. Page numbers are absolute PDF pages.

| Own family / manual | CR1 / LSE / mode pages | Own datasheet pad page |
|---|---|---|
| F002 CN1.4 | CR1 55, clock tree 40: LSE absent | No OSC32 rows |
| F003 CN2.3 | CR1 57, clock tree 42: LSE absent | No OSC32 rows |
| F020 CN1.4 | 69 / 74 / 49 | CN1.3 current 2025-12-30 PDF 23 |
| F030/A030 x030 CN2.5 (both named) | 71 / 76 / 51 | F030 CN1.9 24; A030 CN1.1 22 |
| L010 CN1.2 | 68 / 74 / 52 | CN1.3 24; PB1/PB0 |
| L011 CN1.1 current revision | 66 / 72 / 50 | CN1.1 29 |
| L012 CN1.4 | 71 / 77 / 56 | CN1.0 35 |
| L031 CN1.6 | 68 / 73 / 52 | CN1.9 26 |
| L052 CN1.5 | 71 / 76 / 55 | CN1.3 26 |
| L083 CN2.0 | 76 / 81 / 56 | CN1.9 27 |
| R031 own CN1.3, MCU only | 70 / 75 / 54 | CN1.2 29 |
| W031 own CN1.4, MCU only | 69 / 74 / 53 | CN1.3 30 |

L010 PINLOCK is independently applicable; L011/L012 require PINLOCK and LSELOCK.
Every present LSE uses MODE=0 crystal and MODE=1 external digital input. A pad
lock reserves both pins even with EN=0; unstable/fault states never release the
software reservation. Other families have no pad-lock field. The exact source
paths and page locators are in the authored YAML and
`docs/lse-pad-source-evidence.json`; the latter is a project evidence receipt,
not an original manufacturer document. Existing own pinout YAML supplies each
package's OSC32 bond-out. `ci/verify-lse-ownership-data.py` checks these projections
against originals without HAL tests or hardware execution.

## 当前有界 PLL 候选的逐项来源

当前 PLL 初始化资格为 CW32L083/CW32F020/CW32F030/CW32A030 的一次性 factory-HSI- 或 HSE-fed 系统模式：`pll-qualified.yaml` 与 `electrical.yaml` 分族记录自己的原件 SHA/页码，并投影为 `ClockLimits.pll`。HSE使用既有Config.hse的晶振或旁路声明，来源码由模式派生为0或1。独立 PLL_OUT、运行时重调、DeepSleep 恢复和保证失钟恢复均不在此范围。L052没有系统PLL，不沿共用HAL文件开放API。

L083 原有资格与来源保持不变：

- 启停、来源、MUL、模拟档位、WAITCYCLE和保留位：L083 RM Rev2.0 §4.3.7、§4.7.8，PDF59–60、82/书页58–59、81。
- 过渡、HSI校准、CCS/状态：同一RM PDF63–65、67、69、71–73、75–78/书页62–64、66、68、70–72、74–77；原始HSI trim地址保持既有权威。
- retained AUTOTRIM/RTC/LVD与输出：同一RM PDF87、89、103、157、159、187、205–206、530、534–535；书页各减一。Flash WAIT/KEY：PDF121、131/书页120、130。
- 工作电压、factory HSI误差和PLL输入/输出/周期间抖动：L083 DS Rev1.9表7-4、7-17、7-21，PDF47、55、56/书页46、54、55。300ps周期间抖动不等于绝对周期误差，rate-only标记保持到严格时序调用点。

F020/F030/A030 的独立新增来源见 [原件对应收据](x030-f020-hsi-pll-source-receipt.json)：

- F020 RM `CW32F020_UserManual_CN_V1.4.pdf` Rev1.4：PLL §4.3.7 PDF52–53/书页51–52，寄存器 §4.7.8 PDF75/74；HSI factory trim PDF55、71/54、70；Flash PDF110、120/109、119。
- F030/A030 共用的 `CW32x030_UserManual_CN_V2.5.pdf` Rev2.5 明确覆盖两族：PLL PDF54–55/53–54，寄存器 PDF77/76；HSI factory trim PDF57、73/56、72；Flash PDF112、122/111、121。F030 SDK仅佐证共用布局，不冒充独立A030 SDK。
- F020 采用 `vendor:current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf`，printed Rev1.3，SHA-256 `1e330d800f10114c654b97cbd45b64d56a99c3cb39138d4ef20de3c12968dab0`：运行条件/HSI/PLL为PDF36、44、45/书页35、43、44；原始输出8–48MHz，factory HSI±5%。同名根目录历史文件实为printed Rev1.2，不用作本项选定来源。
- F030 DS Rev1.9对应PDF38、46、47/书页37、45、46；A030自身DS Rev1.1对应PDF35、43、44/书页34、42、43。两者原始PLL输出8–64MHz、factory HSI±2%。三族采用−40…105°C、1.65–5.5V；低于1.8V时HCLK/PCLK≤24MHz，最终总线仍独立检查。
- 既有HSI路径使用HSI分频后的时钟；新增HSE路径使用未分频的HSE。MUL字面值2–12，输入4–24MHz，WAITCYCLE=7。整段实际输入和倍频输出必须各自落在单一模拟档位，最高输出码取4。F020九组、F030/A030各十二组资格组合逐族列于YAML，未接纳的组合不因此被断言为硬件无效。
- 两个自身手册均给出PLL复位值`0x00053483`、debug[19:16]默认0x5和STABLE只读；SVD继承零复位，F020 SVD称debug字段为RFU，x030 SVD遗漏它。保留/检查默认值的类型化修订、访问旁表和复用账本原子对应；当前IR没有reset槽，手册复位权威记入收据并由来源核验检查，不添加PAC reset API。SDK整寄存器写入清掉debug默认值的做法不采用。

HSE新增功能依据见[七份原件对应收据](../docs/hse-pll-source-receipt.json)及[独立晶振组合审阅](../docs/hse-pll-crystal-contract.md)，旧HSI收据保留其历史范围：

- F020 RM Rev1.4：HSE电路/模式PDF46–47、PLL来源/档位52–53、启动握手55、§4.5.8晶振来源例程66；书页各减一。选定current-datasheets的F020 DS printed Rev1.3：供电36、旁路波形41、晶振说明/电气42–43、PLL表7-21为45；书页各减一。
- x030 RM Rev2.5明确覆盖F030/A030：HSE电路/模式PDF48–49、PLL来源/档位54–55、启动握手57、§4.5.8晶振来源例程68；书页各减一。F030 DS Rev1.9对应供电38、旁路43、晶振44–45、PLL表7-21为47；A030 DS Rev1.1对应35、40、41–42、PLL表7-20为44；书页各减一。
- L083 RM Rev2.0：HSE电路/模式PDF53–54、PLL来源/档位59–60、启动握手62、§4.5.8晶振来源例程73；书页各减一。DS Rev1.9对应供电47、旁路52、晶振53–54、PLL表7-21为56；书页各减一。
- 四族PLL输入条件均为4–24MHz、40–60%占空比。晶振功能接纳依赖厂商明确推荐的内部组合和既有谐振器/负载/驱动/布局契约，不独立证明隐藏节点的数值占空比。旁路须在OSC_IN同时满足40–60%占空比、电平、每高低脉冲≥15ns与边沿≤20ns；STABLE不测量这些条件。全部实际频率端点仍各落单一模拟档位并独立服从raw上限，输出始终rate-only；不保证参考丢失后的PLL频率、回退或CPU继续运行。

来源锁的目录总数仍为45项：硬件出处验证要求43个原件，另2项A030 HTML发现页面按既有 discovery-only 策略单列；11份已批准头文件与许可证边界不变。新增的来源说明和审阅记录均为项目撰写，不重新分发完整PDF、SDK、SVD或其全文抽取。

## Oscillator status access preservation

`cw32-data/field-access.yaml` and `docs/oscillator-status-access-evidence.json` preserve already-correct original SVD field access for L012 HSI/LSI/HSE and all 11 LSE-present families. Each evidence row gives its own RM section, printed/PDF page, URL/hash and selected SVD member/archive. The containing registers remain RW; no vendor SVD, normalized register IR or reuse hash is corrected. This closes a field-access lowering omission only.

## CW32L012 direct HSE qualification

The L012 direct-HSE candidate uses the existing locked own RM CN V1.4 and DS CN
V1.0. The source policy is `cw32-data/hse-qualified.yaml`; its exact projection
and digest are in `cw32-data/electrical.yaml`. The detailed receipt is
`docs/qualified-l012-hse-source-receipt.json`. HSE drive/wait and SYSCLK enums
are semantic additions, with canonical singleton history in `register-reuse.yaml`;
no source offset/width/access change is attributed to these additions. The original
SVD already marks oscillator STABLE fields read-only; its separately accepted
mixed-register access projection is recorded in `docs/oscillator-status-access.md`.

The optional `fixed_ccs_hsi_divisor` describes only an explicitly qualified fixed
HSIOSC fallback divisor: L010 /12, L011 /24, L052 /6, L012 /24. Absence provides
no default/configured fallback inference. `hsi_operating_range_hz` records L012's
own legal incoming 90–100 MHz HSIOSC requirement (RM §4.4.2 PDF59), not a bound
for arbitrary TRIM. Generated L012 startup protection requires that fact.

## 二十三个精确料号/封装的主动 LSE 与 RTC 来源

当前资格以 [lse-qualified.yaml](../cw32-data/lse-qualified.yaml) 的二十三个条目为准：CW32F030C8T7、CW32A030C8T7（均为 LQFP48），CW32F020C6U7（QFN48），CW32L031C8T6（LQFP48）、CW32L031C8U6（QFN48）、CW32L031F8U6（QFN20），CW32R031C8U6（QFN48）、CW32W031R8U6（QFN64），以及 CW32L052C8T6（LQFP48）、CW32L052R8S6（LQFP64 7×7 mm）、CW32L052R8T6（LQFP64 10×10 mm）；另有 CW32L083RBT6、CW32L083RCT6（LQFP64 10×10 mm）、CW32L083RCS6（LQFP64 7×7 mm）、CW32L083MCT6（LQFP80）、CW32L083VCT6（LQFP100）；另有 CW32L010F8P6（TSSOP20）、CW32L010F8U6（QFN20）、CW32L010Y8M6（SOP16）；新增 CW32L011K8T6/K8U6（LQFP32/QFN32）、CW32L012C8T6/C8U6（LQFP48/QFN48）。原有十六个条目的独立监测/入场资格保持不变；原生七款按下节各自契约资格化，L010 维持 inherited_legal，L011/L012 使用 factory_trim，其他料号、封装和族别不自动继承主动配置资格。板级契约及运行边界见 [qualified-lse.md](../docs/qualified-lse.md)。

### x030 两款 LQFP48 的专属引文

本小节只覆盖 CW32F030C8T7 与 CW32A030C8T7。共享 RM 为 `CW32x030_UserManual_CN_V2.5.pdf` Rev 2.5：LSE 控制、电气流程、保留源及 RTC 寄存器相关 PDF 页为 51–52、71、76、94、173、184、187–195、357，书页各减一；RTC 补偿独立消费 LSE 的补充依据见 [lse-active-rtc-admission.json](../docs/lse-active-rtc-admission.json)，不因 RTC.SOURCE 选择 LSI/HSE 而忽略。这些 x030 页码不作为其他族的依据。

各自 DS 为 F030 Rev 1.9（PDF 24、26、30、38、44–45）与 A030 Rev 1.1（PDF 22–23、27、35、41–42），书页各减一。PC14/PC15 引脚、供电/温度、晶体与旁路条件分别取自身表格。这两款 x030 的标称 32768 Hz、精确料号范围、板级每周期边界和保守的全 RTC 复位态入场属于软件资格策略；典型启动时间不作为最长等待保证。

共享 AWT 的 `Source::LSE=3` 还核对了 F020 自身 RM Rev 1.4 §11.8.1（PDF 170、书页169）；F020 的主动配置资格另由下节自身原件审核支持。

### F020 C6U7 QFN48 的自身原件

CW32F020C6U7 使用 `CW32F020_UserManual_CN_V1.4.pdf` 与选定的 `current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf`。原件 ID/SHA、PDF/书页和 SDK 成员定位见 [F020原件对应收据](../docs/lse-f020-source-receipt.json)，逐项事实及 RTC 入场依据见 [F020主动LSE记录](../docs/lse-active-f020.json) 和 [F020 RTC记录](../docs/lse-active-f020-rtc-admission.json)。它不借用 x030 的页码或封装依据。

该精确 QFN48 的 PC14/PC15 为引脚3/4；SDK 的 PC13/PC14 配方不采纳，以自身 DS 为准。F020 的 CLKCCS/HSECCS/LSECCS 必须写1，晶体及旁路均保留自身文档的1 MHz上限。F020F6U7、F020K6U7及族别别名仍无主动LSE资格。

### L031 / R031 / W031 的自身原件与差异

- L031 的 C8T6 LQFP48、C8U6 QFN48、F8U6 QFN20：`CW32L031_UserManual_CN_V1.6.pdf` 与 `CW32L031_DataSheet_CN_V1.9.pdf`；[原件对应收据](../docs/lse-l031-source-receipt.json)、[主动LSE记录](../docs/lse-active-l031.json)、[RTC入场依据](../docs/lse-active-l031-rtc-admission.json)。
- R031 的 C8U6 QFN48：`CW32R031_UserManual_CN_V1.3.pdf` 与 `CW32R031_DataSheet_CN_V1.2.pdf`；[原件对应收据](../docs/lse-r031-source-receipt.json)、[主动LSE记录](../docs/lse-active-r031.json)、[RTC入场依据](../docs/lse-active-r031-rtc-admission.json)。
- W031 的 R8U6 QFN64：`CW32W031_UserManual_CN_V1.4.pdf` 与 `CW32W031_DataSheet_CN_V1.3.pdf`；[原件对应收据](../docs/lse-w031-source-receipt.json)、[主动LSE记录](../docs/lse-active-w031.json)、[RTC入场依据](../docs/lse-active-w031-rtc-admission.json)。

这些收据分别给出自身原件 ID/SHA、PDF/书页与 SDK 成员定位；供电、温度、引脚和寄存器要求按各族记录，不能沿用 x030 的电气条件。三族 CCS 控制可配置，GPIO 均无 SPEED、LOCK、HIGHIE、LOWIE 寄存器；L031F8U6 QFN20 没有直接 LSE 输出 AF 路由，其空路由表是精确封装结论。

三族持有源能力的软件策略要求 LSE 监测。仅在重复确认 LSI 停止且无文档所列使用者时，才允许载入原厂 trim；正在使用的 LSI 不停机、不重调。精确已启用 LSE 的复用不重启或更改 CCS。前述八个条目仍以标称32768 Hz和板级每周期边界资格化；没有自动RTC回退、失钟后的日历连续性或低功耗恢复保证。上述原件、数据、源码及编译检查均不替代实板验证。

### L052 三个精确封装的自身原件

CW32L052C8T6、CW32L052R8S6 与 CW32L052R8T6 使用自己的 `CW32L052_UserManual_CN_V1.5.pdf` Rev 1.5 与 `CW32L052_DataSheet_CN_V1.3.pdf` Rev 1.3；英文 RM Rev 1.0 仅作佐证。原件 ID/SHA、PDF/书页与适用范围见 [L052 原件对应收据](../docs/lse-l052-source-receipt.json)、[专属资格说明](../docs/qualified-l052-lse.md)、[主动 LSE 事实](../docs/lse-active-l052.json) 和 [RTC 入场记录](../docs/lse-active-l052-rtc-admission.json)。SDK V1.4 的精确成员定位与哈希见 [SDK 成员收据](../docs/lse-l052-sdk-member-receipt.json)，SDK 初始化捷径不覆盖自身手册要求。

DS PDF 10、26–27、32–34、43、49–51、79（书页各减一）分别支持精确封装、PC14/PC15 引脚3/4、输出路由及电气边界；PC4/AF6 直接 LSI 输出仅在两款64引脚封装上存在。RM PDF 55–56、61–62、70–83、94、146–147、151–157、167–175、184–200、224–225、235–237、359、390、536、553–554 支持原生 LSE 双模拟参数组、可配置 CCS 与 LSI 检测依赖、GPIO、AUTOTRIM/UART/LPTIM/LCD 使用者及 RTC 复位状态；LSE 复位值0x0A2B、ALARMA 复位值0x04120000不沿用旧族值。逐项定位以相邻收据为准，这些页码不作为其他族的依据。

三款 L052 均只按标称32768 Hz及板级每周期边界资格化；供电1.65–5.5 V、环境−40…85 °C及各自波形条件必须同时满足。启动/运行 drive 与 amplitude 在使能前独立写入，使能后不重写。AUTOTRIM 校准或自动模式阻止入场；已关闭的 LPTIM/LCD 工作时钟门保持关闭，不能为检查而恢复工作。默认 None 路径不新增检查或源/引脚写入。失败保留使能、预留与诊断状态；普通复位未必清除 LSE 控制，可能需要 POR。未建立实板启动保证、自动 RTC 回退或失钟后的经过时间连续性。

### L083 五个精确封装的自身原件

CW32L083RBT6、CW32L083RCT6、CW32L083RCS6、CW32L083MCT6 与 CW32L083VCT6 使用自己的 `CW32L083_UserManual_CN_V2.0.pdf` Rev 2.0 与 `CW32L083_DataSheet_CN_V1.9.pdf` Rev 1.9；英文 RM Rev 1.0 仅作佐证。原件 ID/SHA、PDF/书页与适用范围见 [L083 原件对应收据](../docs/lse-l083-source-receipt.json)、[专属资格说明](../docs/qualified-l083-lse.md)、[主动 LSE 事实](../docs/lse-active-l083.json) 和 [RTC 入场记录](../docs/lse-active-l083-rtc-admission.json)。SDK V2.2 的五个精确成员及哈希见 [SDK 成员收据](../docs/lse-l083-sdk-member-receipt.json)；SDK 未先准备 LSI 的启用捷径不覆盖自身手册的检测依赖。

DS PDF 10、27–32、35–38、53–55、86（书页各减一）支持封装、引脚、AF 和振荡器条件。PC14/PC15 在64/80引脚封装为3/4，在100引脚封装为8/9；LSI 输出按完整封装表区分 PC4/AF6、PD5/AF6、PF2/AF4。RM PDF 56–58、61–65、75–89、163–172、178、180–181、187、197、204–212、238、247–249、370、396–397、544、562–563（书页各减一）支持单模拟参数组、GPIO LCKR、六个 UART 与 AUTOTRIM/LPTIM/LCD 使用者、RTC 补偿和复位态。LSE 复位值0x2B、ALARMA复位值0x00120000不借用L052双模拟参数组的值；逐项定位以相邻记录为准。

五款 L083 仅按标称32768 Hz、板级每周期边界、供电1.65–5.5 V和环境−40…85 °C资格化。DS表7-18（PDF55/书页54）的原厂LSI范围31816–33784 Hz用于检测器；软件在获取外设或执行RCC写入前要求 `256 × LSE_min_hz > 129 × 33784`，最小整数下界为17024 Hz，声明区间仍须包含标称32768 Hz。额外一沿是保守计数相位裕量，不是新的厂商指标。晶体负载/驱动或旁路电平、45–55%占空比、≥450 ns高低脉宽及≤50 ns边沿仍须独立满足；1.5 s启动值仅为典型值。

此前L083扩展时，原有十一个资格JSON、schema身份、HSI/HSE/PLL后端与电气资格保持不变。默认None不增加LSE检查或源/引脚写入。活跃源不停止或重调，关闭的LPTIM/LCD工作门不为检查而打开；失败保留使能、预留与诊断状态，普通复位未必清除LSE控制。源码审查与ARM编译/链接不构成实板启动、频率精度、自动RTC回退或失钟连续性保证。来源锁仅追加这些项目证据的关联，不更换原件、SDK成员或许可范围。

### L010 三个精确封装的自身原件与原生契约

CW32L010F8P6、CW32L010F8U6、CW32L010Y8M6 使用自身 `CW32L010_UserManual_CN_V1.2.pdf` Rev1.2 与 `CW32L010_DataSheet_CN_V1.3.pdf` Rev1.3。当前原件 ID/SHA、PDF/书页及逐项寄存器/观察者事实见 [L010资格记录](../docs/lse-l010-qualification.json)、[RTC入场记录](../docs/lse-l010-rtc-admission.json) 与 [运行契约](../docs/qualified-l010-lse.md)。RM PDF74/书页73给出四位独立 DRIVER/PDRIVER、WAITCYCLE、PINLOCK与STABLE，没有AMP；DS PDF24–25、27、32、40、42（书页各减一）支持封装、AF、供电及振荡器条件。PB1/PB0 在 TSSOP20 为12/11、QFN20为9/8、SOP16为10/9；没有直接LSE输出路由，PB4/PB6的AF2是独立RTC输出路由，按实际封装裁剪。

三款只按标称32768Hz、每周期板级边界、1.62–5.5V及−40…85°C资格化。StartupOnly 要求继承LSECCS为0且保持0，失钟后STABLE可能仍为1；MonitoredExistingRoutes 要求合法、已稳定且trim/wait不变的LSI，上界36080Hz，满足 `256 × LSE_min_hz > 129 × 36080`。原生CCS可自动请求LSI，因此LSIEN=0不等于没有运行监测；该路径不冷启动或校准LSI。既有保护、IRQ及定时器故障路由刻意保留，不承诺隔离或经过时间连续性。

RTC source0的新源启动另需公开的RTC_OUT/RTC_1Hz观察者移交与整个GPIOB工作窗口契约；配置门尝试有界恢复，恢复失败明确报错；休眠定时器工作门不为检查打开。三款原生LSE/HSI日历例程位于 `examples/shared/l010-lse-clock`。三份共享模型采用已接受的可选原生事实扩展；11份许可头文件、43个硬件原件加2个discovery-only条目与发布边界均不变。


### L011/L012 四个精确封装的自身原件与原生契约

L011 使用当前 CN RM Rev1.1（2026年6月封面）、DS Rev1.1；L012 使用 CN RM Rev1.4、当前 EN RM Rev1.0（2026年6月封面，锁中状态为 corroborating-language-edition）及 DS Rev1.0。完整自身原件 ID/SHA、PDF/书页见 [L011资格记录](../docs/lse-l011-qualification.json)、[L012资格记录](../docs/lse-l012-qualification.json) 与 [范围说明](../docs/qualified-l011-l012-lse.md)。没有新增原件、SDK示例依赖或许可结论。

L011 PC14/PC15 为封装脚2/3，L012为3/4。两族各自支持四位 DRIVER/PDRIVER、StartupOnly 与 CCS监测区分；新监测政策要求已稳定、原厂trim匹配且TRIM/WAITCYCLE不变的LSI，显式自身半字地址0x001007C2。L011 DS PDF51/书页48给出的原厂环境上界是41000Hz，不能被RM±10%偷换；L012 DS PDF58/书页55给出36080Hz。硬件128沿/256个LSI周期与工程额外一沿分开保存；稳定/原厂匹配不是频率或窗口抖动测量。旧L010的九项事实、36080Hz inherited_legal 行为以及此前十六款保持不变。三模型将native_l010显式重命名为native_low_power，不提供别名或掩盖所需事实的默认值。

L011 RM PDF152/书页151及L012 CN PDF202/书页176、EN PDF219/书页193确认 RTC无ACCESS字段。LSE SOURCE0、PSC1=0、PSC2=0x3fff保持实际RTCCLKD≤1MHz；HSIOSC仍96MHz。切源先把第一分频数设置为当前与目标的较大值，验证后切SOURCE，再写入目标分频，避免转换期间提高输入一级频率。保留式attach不通过复位、解锁、停源或重调制造兼容。

GPIOC检查允许整bank采样/滤波/事件推进；门恢复不撤销此前副作用。未开启输出bank、外部RTC/LSE接收者、保留定时器根与不可访问L012 UART3属于未完全运行验证的功能移交条件。L012 UART3中英文门语义以及BTIM/ATIM映射冲突仍明确保留。无额外safe-Rust内存安全义务、自动RTC回退、低功耗恢复或失钟后连续性保证。

### 三个 classic 精确封装的 init-only LSE SYSCLK

仅 CW32F020C6U7、CW32F030C8T7、CW32A030C8T7 新增 `Config.lse + Sysclk::LSE`；主动 LSE/RTC 原有23款与其他系统源路径不扩大。新增自身依据见 [独立系统源资格](../docs/classic-lse-sysclk-qualification.json) 与 [运行契约](../docs/classic-lse-sysclk.md)。硬件128沿/256个LSI周期和软件额外1沿分开记录在原 `lse-qualified.yaml` 的这三个精确配置内，经三模型和 metadata/build 正式投影；原厂LSI上界/条件/半字地址复用自身Stage56事实。

x030 RM CN2.5 的 PDF51–52、57、59–60、62–64、70–71、74、76、79–80、112，以及 F020 RM CN1.4 的 PDF49–50、55、57–58、60–62、68–69、72、74、77–78、110 支持LSE启动、强制CCS、LSI依赖、合法PLL经HSI切换、selector4和Flash等待。书页均减一。F030 DS CN1.9 PDF44–46、A030 DS CN1.1 PDF41–43、选定 `current-datasheets/` F020 DS CN1.3 PDF42–44 分别支持自身波形/晶振和原厂LSI条件；F020最大34440Hz，x030两款33784Hz，不混用原件。

新目标先完整准备/借用原厂LSI，保持请求，再在HSI上启动/借用LSE；最终分频与Flash完成后才选择LSE。相同LSE元组供给RTC，板级每周期界与LSI仅速率界分开。源码/链接不代替实板启动、检测窗口、源丢失恢复或RF验证；发布仍由独立审阅边界决定。

### 五个 L031/R031/W031 精确封装的 init-only LSE SYSCLK

CW32L031C8T6/C8U6/F8U6、CW32R031C8U6、CW32W031R8U6 单独新增同一 `Config.lse + Sysclk::LSE`。依据为 [自身六原件资格](../docs/l031-r031-w031-lse-sysclk-qualification.json) 与 [运行契约](../docs/l031-r031-w031-lse-sysclk.md)，不借用 classic 强制 CCS 或 L052/L083 回退规则。三个共享数据模型、来源锁身份及原23款辅助 LSE 资格不变；现有 `sysclk_detector` 仅增加这五个精确行。

L031 RM CN1.6 PDF54–59、62–63、67–68、71、75、107、170；R031 RM CN1.3 PDF56–61、64–65、69–70、73、77、109、172；W031 RM CN1.4 PDF55–60、63–64、68–69、72、76、108、171 分别支持自身LSI启动/参数冻结、selector4、可配置CCS、128/256检测、Flash和RTC。L031 DS CN1.9 PDF26/38/45–47、R031 DS CN1.2 PDF29/42/52–54、W031 DS CN1.3 PDF30/41/51–53 独立绑定封装与电气范围。原厂LSI半字地址0x00100A02、32800/31816/33784Hz及供电/温度复用既有RTC事实，不新增LSI SYSCLK资格。

新目标保留CLKCCS/HSECCS，仅新启动添加LSECCS。匹配冷LSI不写TRIM/WAIT、不套用重调所有权证明，但请求可恢复停驻AWT/UART/GPIO滤波/MCO/直出消费者；功能移交必须允许事件推进。失配冷源保留自身两遍证明。完整双稳定/原厂参数检查先于HSI校准桥或LSE启动；最终HSI分频、Flash完成后仅一次LSE选择。软件129/256裕量与LSI仅速率事实分开；RTC共享板级每周期LSE元组。无实板检测窗口、失钟恢复、低功耗或RF保证。

### 三个 L052 精确封装的 init-only LSE SYSCLK

仅 CW32L052C8T6、CW32L052R8S6、CW32L052R8T6 新增同一 `Config.lse + Sysclk::LSE`；自身依据及原件 ID/SHA 见 [双原件系统源资格](../docs/l052-lse-sysclk-qualification.json)，功能边界见 [运行契约](../docs/l052-lse-sysclk.md)。三个共享数据模型与既有辅助 LSE 资格不变；该 L052 路径不新增 LSI SYSCLK 或 L083 系统源资格。

`CW32L052_UserManual_CN_V1.5.pdf` Rev1.5 的 PDF54、57、59、61–62、65–66、70–74、112（书页53、56、58、60–61、64–65、69–73、111）绑定自身切换流程、selector4、可配置 CCS、128/256 检测、原厂 LSI、HSI 字段及 Flash；`CW32L052_DataSheet_CN_V1.3.pdf` Rev1.3 的 PDF43、51（书页42、50）绑定供电/温度、24/48 MHz 总线上限和原厂 HSI/LSI 误差。以上两组与资格 JSON 的精确页码集合一致；不借用其他族规则，原有引脚/双模拟组/消费者依据仍见前述 L052 专节。

保留 CLKCCS/HSECCS/LSELOCK；最终分频下的配置 HSI 独立校验，固定 HSI8MHz 回退按不分频的8.16MHz上界覆盖总线和Flash，不假定回退保留任何分频器。HSI无WAIT字段；LSI的TRIM/WAIT与板级LSE元组分别冻结，软件额外1沿不是厂商指标。源码/编译不建立实板启动、检测窗口、失钟恢复或日历连续性保证。

### 五个 L083 精确封装的 init-only LSE SYSCLK

Stage60 时 CW32L083RBT6、CW32L083RCT6、CW32L083RCS6、CW32L083MCT6 与 CW32L083VCT6 单独新增同一 `Config.lse + Sysclk::LSE`；当时系统目标由原十一款增至十六款，原二十三款辅助 LSE/RTC 的资格、schema 与其他系统源路径保持不变。自身原件 ID/SHA、页码与保守策略见 [系统源资格](../docs/l083-lse-sysclk-qualification.json)，入场与失败边界见 [运行契约](../docs/l083-lse-sysclk.md)。`lse-qualified.yaml` 仅为这五款增加现有 `sysclk_detector` 事实；不扩展族别别名、LSI SYSCLK 或独立 PLL 输出生命周期。

`CW32L083_UserManual_CN_V2.0.pdf` Rev2.0 的 PDF65/书页64、75–76/74–75 支持运行检测、selector4 与可配置 CLKCCS；PDF67/66 限定 PLL 切换经 HSI/HSE；PDF59–60/58–59、72/71、78/77、82/81 支持 PLL 来源、停稳握手、HSI DIV/TRIM 与保留参考；PDF62/61 给出 HSI 校准的32–48MHz安全范围，64MHz总线限值不扩大振荡器范围。PDF121/120、131/130 给出 Flash WAIT0/1/2 的实际 HCLK 上限24/48/72MHz。`CW32L083_DataSheet_CN_V1.9.pdf` Rev1.9 的 PDF47/书页46 与55/54 分别绑定供电/总线条件及原厂 HSI±2%、LSI±3%：HSI 上界48.96MHz，LSI 31816–33784Hz。原件身份沿用 `evidence-sources.json`；SDK V2.2 只佐证编码，不替代切换契约。原件和取回收据属于外部证据，不复制入源码包。

该新系统目标要求声明最低 VDD≥1.8V、最高≤5.5V、VDDA=VDD 与环境−40…85°C。保守回退按原始48.96MHz覆盖，不计 HSI/AHB/APB 分频收益，即使 CLKCCS 为0也一样；这不是断言硬件重置分频器，也不回改既有直接 HSE 策略。最终分频下的配置 HSI 单独校验，Flash WAIT2保留；辅助 LSE 原1.65–5.5V源范围不变。继承 PLL 先经独立合法、未改参数且已就绪的 HSI 切换，再观察 PLLEN=0 与 PLL.STABLE=0 后才允许改参考。PLL配置保留，但停止会有意停止输出；调用方必须移交静止的输出与消费者。最终 LSE 选择后不再写 CR0，失败不重放选择、不恢复失钟或时间连续性。

单模拟组、PC14/PC15、板级每周期界与保留消费者继续使用前述 L083 自身资料，不引入 L052 PDRIVER/PAMP。示例复用3.0–3.6V、−20…70°C与32766–32770Hz声明；必须由实际板级资格替换。Stage60 时的 `ci/check-lse-sysclk.sh` 只定义当时的本地后续检查范围，所列17个库与16组晶振/旁路固件并非已执行数量；每次实际执行的子集、结果与未跑项目须分别记录，不据此宣称 hosted CI 或实板验证。

### 三个 L010 精确封装的 init-only LSE SYSCLK

Stage61 时仅 CW32L010F8P6、CW32L010F8U6、CW32L010Y8M6 通过原生 `Config.lse + Sysclk::LSE` 新增系统目标，当时合计十九个精确封装；此前十六款系统目标和二十三款辅助 LSE 资格保持不变，该阶段未扩大通用族别、L011/L012。自身原件、页码及策略见 [系统源资格](../docs/l010-lse-sysclk-qualification.json) 与 [运行契约](../docs/l010-lse-sysclk.md)。原 `sysclk_detector` 记录增加这三款，不新增 schema 或另设 LSI 校准来源。

`CW32L010_UserManual_CN_V1.2.pdf` 的 PDF52–53/74（书页51–52/73）支持四位运行/启动 drive 与启动计数；PDF57–59/68（56–58/67）区分启动 STABLE、128沿/256个LSI周期检测与开启 CLKCCS 时的有效 HSI4MHz 回退；PDF63/67（62/66）支持 LSE selector4、原生分频与 KEY。PDF69/71/75/77–78（68/70/74/76–77）支持自动 LSI 请求、HSI校准/未指定复位 trim 与 ready IRQ。PDF70/114（69/113）给出 SYSCTRL FLASHWAIT 与 FLASH WAIT 的同功能关系，不能把其余路由变化当作 Flash 别名。

`CW32L010_DataSheet_CN_V1.3.pdf` 的 PDF32（书页31）限定1.62–5.5V及低于1.8V时24MHz、至少1.8V时48MHz总线上限；PDF40/42（39/41）支持100kHz旁路上限、45–55%占空、450ns最小脉宽、50ns最大边沿与仅典型的晶振启动时间；PDF43（42）支持原厂HSI48MHz±2%，固定4MHz回退独立按4.08MHz覆盖总线和Flash，不计分频收益。原厂LSI±3%不替代RM PDF56（55）的继承合法32.8kHz±10%契约。

StartupOnly 保持LSECCS=0，后来失钟可能不清STABLE且直接停住CPU；MonitoredExistingRoutes 要求进入时已稳定、合法且参数不变的LSI，最大36080Hz，`256 * LSE_min_hz > 129 * 36080`，不自动冷启动或准备原厂监测。HSI重新校准可能临时请求继承合法LSI，需首次非稳定请求的RTC/UART/LPTIM/MCO/ready IRQ/模拟消费者否决，并保留未全查的timer/GPIO/IWDT及下游观察者功能移交。最终LSE选择后不写CR0；旧保护、IRQ、brake路由和部分失败状态保留，不承诺回退发生或时间连续性。

示例使用PB1输入/PB0输出（F8P6脚12/11、F8U6脚9/8、Y8M6脚10/9），原生运行Level2/启动Level10及16384周期均须实板资格化。HSI默认/12；生成metadata提供三款各65536-byte Flash/4096-byte RAM，链接显式使用link.x。Stage61 时本地检查脚本的后续矩阵扩为20个库与19组晶振/旁路固件（38个二进制），只描述待执行范围，不是已跑数量，也不添加 hosted CI、HAL测试或实板验证。原件身份与许可边界不变；外部原件不复制入源码包。


### 两个 L011 精确封装的 init-only LSE SYSCLK

Stage62 仅加入 CW32L011K8T6（LQFP32）与 CW32L011K8U6（QFN32），当时系统目标共二十一款，辅助 LSE/RTC 仍为二十三款。旧十九款系统配置和全部辅助资格投影保持不变；该阶段未扩大 L012 与通用别名。独立自身原件、claim 页码与策略见 [系统源资格](../docs/l011-lse-sysclk-qualification.json) 与 [运行契约](../docs/l011-lse-sysclk.md)。继续使用既有 `sysclk_detector`，不引入新 schema、适配层或 SDK 初始化代码。

选定 `CW32L011_UserManual_CN_V1.1.pdf` 为2026年6月封面、SHA-256 `b245887e9037caf267f739e25bd3c354c2579d23315fc06389e055f371159f4f`；DS Rev1.1 为 `0b7414049824881920fc38f829029e3ba0af88feb4536fb27351df0d60f688a5`，SDK V1.0.3 ZIP 为 `76adfe39360eb1d05ef58c25f26a8c1f99f2bc2f8fef677214aaca85cffc679e`。同名旧RM不替换当前锁定字节；原件 URL、revision、hash、成员字节和许可范围不变。SDK `Libraries/src/system_cw32l011.c` 的43–66行仅佐证软件明确装载HSI/LSI原厂trim；Rust启动不是该流程，全1替代值不是原厂资格。

RM PDF61/书页60、65/64绑定selector4，54/53、69/68绑定96MHz HSI、0x001007C0、TRIM[10:0]/DIV[14:11]/STABLE15以及数值/24对应编码14；57/56、66/65给出选中LSE失效且CLKCCS=1时有效HSI4MHz回退。DS PDF39/书页36、51/48绑定1.7–5.5V、低于1.8V时24MHz/至少1.8V时96MHz总线限值及HSI±2%。软件独立覆盖完整4.08MHz回退上界而不计AHB/APB分频收益，另查最终分频下配置HSI；不推定回退重写DIV/使能/总线字段或CPU继续执行。RM PDF102/101、113/112给出Flash WAIT0/1/2/3的24/48/72/96MHz上限，68/67与113/112绑定FLASHWAIT/WAIT同功能视图。初始WAIT3只是保守值，最终WAIT由LSE、配置HSI和回退的最大合格HCLK决定；默认HSI/24为WAIT0。

监测继续要求首次配置门写入前已稳定、未擦除且匹配自身0x001007C2原厂半字的10位LSI TRIM，并保留WAIT[11:10]；自动硬件请求可在LSIEN=0下工作。DS PDF51/书页48表7-19的−10/+25%给出29520–41000Hz，监测检查为 `256 * LSE_min_hz > 129 * 41000`（整数最小20661Hz）；RM PDF57/56、66/65的硬件阈值是128沿/256周期，软件额外一沿不冒充硬件事实。RM PDF54/53的合法调整32.8kHz±10%、70/69的30–36kHz/约0.4%步进，与DS的0.16%步进及原厂范围保持独立冲突说明。稳定或原厂匹配不测量频率/每窗口抖动，也不代替板级合法性。

StartupOnly继续允许合法冷启动，不增加原厂LSI前置条件。既有HSI→不改参数的合法LSI→原厂HSI桥不写LSI TRIM/WAIT，也不自动准备监测。首次非稳定LSI请求在保留消费者检查后与紧靠LSIEN前重复检查；RTC SOURCE2及保留4…7、UART1/2/3 SOURCE3（不受RXEN/TXEN影响）、使能LPTIM ICLKSRC3、MCO4、LSIRDY IRQ和使能LSI滤波VC/LVD均拒绝，后来的STABLE不能跳过第二次检查。UART3自己的配置门/复位均为APBEN1/APBRST1 bit8（RM PDF78/77、81/80），SOURCE见422–425/421–424。各配置门独立恢复，复位保持态不释放，恢复失败保留专属错误。

功能移交仍包括未全查的GTIM/ATIM LSI_OUT selector9、GPIO LSI滤波、IWDT及timer/ADC/GPIO级联/外部观察者；不开休眠工作门来制造无活动证明。PB0 AF3 HSIOSC_OUT有自身RM PDF123/书页122、DS PDF30/书页27依据，即使MCO关闭也会看到HSI启动/重调中断。RTC SOURCE3的原始HSIOSC所有权与MCO3/HSIRDY检查独立；ADC/LVD/VC否决保持。GPIOC整bank检查可能推进事件，门恢复不撤销进度；RTC_OUT PA1/PA3 AF3和source-zero RTC功能移交继续成立，不扩大safe-Rust调用者的内存安全义务。

DS PDF28–29/书页25–26绑定两款PC14输入/PC15输出为脚2/3；旁路只用PC14。DS PDF48/45保留100kHz上限，与RM PDF51/50允许1MHz的差异不默默消解；波形仍需0.7VDDIO高/0.3VDDIO低阈值、至少450ns高低脉宽、至多50ns边沿及45–55%占空。DS PDF50/47的1.50s晶体启动仅为典型值。例程保持独立Level2/Level10、16384周期、StartupOnly，声明3.0–3.6V、−20…70°C及每周期32766–32770Hz，须实际板级资格化。两款使用自身生成memory与显式link.x；无crate/依赖升级。LSE最后选择后不写CR0，故障/brake/IRQ路由保留；失钟可停CPU，错误不发布时钟且需复位，不保证回滚、RTC迁移/连续性或故障回退时限。

Stage62本地脚本计划范围为22个普通库、21组晶振/旁路SYSCLK固件（42个二进制）及既有4条其他来源回归命令；这些是待执行范围，不是已通过数量。原有实际执行收据不覆盖或重写，逐次实际子集另记。`examples/shared/l010-lse-clock`的既有L011辅助晶体/旁路/HSIOSC日历例程是脚本之外的保留验证入口；本次无新增测试、编译探针、hosted CI或实板运行。


### 两个 L012 精确封装的 init-only LSE SYSCLK

仅 CW32L012C8T6（LQFP48）与 CW32L012C8U6（QFN48）通过既有原生 `Config.lse + Sysclk::LSE` 新增系统目标；当前系统及辅助 LSE/RTC 均为二十三款。旧二十一款系统记录与全部二十三款辅助资格投影保持不变；通用别名及其他封装不扩大。自身依据见 [系统源资格](../docs/l012-lse-sysclk-qualification.json) 与 [运行契约](../docs/l012-lse-sysclk.md)。继续使用既有 `sysclk_detector`，不新增 schema、适配层、LSI SYSCLK 或 PLL 分支。

使用锁定的 CN RM Rev1.4、当前2026年6月 EN RM Rev1.0、DS CN Rev1.0 与 SDK1.0.5；原件 SHA-256 依次为 `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`、`f56d5ed899dd090b469fac6a09084aab9f5068ae3b56cf7562b83997bd2f1088`、`08a605fe8843d1eefa2b1115474b5a74df2833068f0ad6f35225e4f39377da76`、`8b0a4c0ee865642d08f353aec98906c900a8b10f672120e649e6b1bf5e29c18f`。同名旧 EN 手册不能替换当前字节；URL、revision、来源哈希、SDK成员及许可范围不变，完整原件只作外部证据。

L012 自身 HSIOSC 为96MHz±2%；配置默认/12对应8MHz与7.84–8.16MHz，硅复位及文档有效回退/24对应4MHz是不同事实（EN PDF63/书页37、78/52；DS58/55；回退CN62/36、EN66/40）。自身VDD1.7–5.5V、VDDA=VDD、−40…85°C及低于1.8V时24MHz/否则96MHz总线限制保持不变（DS47/44）。最终配置HSI与完整4.08MHz回退上界分别检查；回退不计AHB/APB分频收益，也不保证寄存器保留或CPU继续运行。

自身CN PDF73/书页47、127/101及EN77/51、135/109确认 SYSCTRL.FLASHWAIT[6:4] 与 FLASH.WAIT[2:0] 同功能。只经FLASH写入，保留FETCH/CACHE/CACHEINVALID/RFU；初始WAIT3在受保护HSI前建立，最终WAIT在已校准HSI及最终总线分频下确认后才选LSE。默认HSI/12为WAIT0；旧/新混合别名读回立即失败，其他身份变化不可等待消除。最后选择LSE后任何成功或错误路径均不再写CR0或FLASH，不回滚、重选源或清标志。

StartupOnly 保持LSECCS=0，失钟可保留STABLE并停住CPU；合法冷启动的HSI校准桥仅临时请求参数不变的合法LSI，不自动准备监测。MonitoredExistingRoutes 必须在首次配置门写入前已有稳定、非擦除原厂半字0x001007C2匹配的9位TRIM及不变WAIT（CN59/33、75/49；EN63/37、79/53）。DS58/55给出32.8kHz±10%，即29520–36080Hz；`256 * LSE_min_hz > 129 * 36080` 的整数最小值18181Hz，硬件128沿与额外工程裕量1沿分开。稳定及原厂匹配不测量频率或每窗口抖动，RM/DS的trim步进描述冲突保留。

首次非稳定LSI请求的入场分类保持到紧靠LSIEN前的重复检查；RTC/AWT、UART1/2、已开门UART3、I2C1/2主从raw1/3、使能LPTIM ICLKSRC3、MCO/ready IRQ、无门LVD及四个VC按自身布局拒绝。需要HSI启动/重调/等待初始未就绪时也拒绝I2C主从raw1/3，不选择方便的手册或SDK定义制造无消费者证明。UART3门属性仍冲突，关闭时不为检查而开启；复位保持态拒绝且不释放，各门独立恢复，恢复失败保留专属错误。

ADC共享门自身CN84/58及EN89/63明确为配置兼工作门：开启后ADC1/ADC2转换、触发或下游可先于EN拒绝而前进，恢复门不能撤销。功能移交须覆盖该运行窗口、整GPIOC、PB0 AF3 HSIOSC_OUT、PB11 AF4/PF3 AF2 LSI_OUT、不可见UART3、IWDT、timer/ADC/GPIO级联及外部观察者，既有平台bus-master/内存所有权边界不扩大。BTIM原生raw3仅BTIM1为RTC_OUT，BTIM2为LPTIM_OV，BTIM3为VC4_OUT，raw0为无信号、raw6为UARTx_RXD（CN232/206、EN252/226），与SDK定义及ATIM冲突保持可见；不打开休眠工作门来推断无活动。

两款PC14/PC15分别为脚3/4，旁路只持有PC14。原生Level2/Level10、16384周期、StartupOnly与3.0–3.6V、−20…70°C、每周期32766–32770Hz均为须实板资格化的显式声明，不是测量结果。DS100kHz旁路上限不被RM1MHz替换；波形仍需45–55%占空、至少450ns高低脉宽与至多50ns边沿，1.50s晶体启动仅典型（DS55/52、57–58/54–55）。既有原生LSE与HSIOSC日历源/分频保留，SYSCLK切换不迁移RTC/AWT；固定1MHz time-driver在取所有权前拒绝。失钟使冻结时钟失效，错误不发布新时钟且须复位。

既有 `examples/shared/lse-sysclk` 仅新增两款feature，复用原生晶体/旁路与自身生成FLASH/RAM、显式link.x；此前21款配置、引脚和依赖不变。本地 `ci/check-lse-sysclk.sh` 计划范围为24个普通库、23组晶体/旁路SYSCLK固件（46个二进制）及原4条回归命令，数字不是已执行结果。原 `examples/shared/l010-lse-clock` 两款L012辅助晶体/旁路/HSIOSC日历仍为独立保留检查入口，不新建workspace、probe、HAL测试、harness、hosted CI或实板验证。


## Exact CW32L052 factory-LSI SYSCLK

Only CW32L052C8T6, CW32L052R8S6 and CW32L052R8T6 receive the independent
factory-LSI SYSCLK capability in `cw32-data/lsi-sysclk-qualified.yaml`.
Generic L052 and native L010/L011/L012 are excluded. That L052 addition did
not qualify L083; the later exact-five L083 contract is recorded below. The earlier
L052 LSE-specific qualification remains historical; its statement that it did
not qualify LSI is unchanged. See the [separate L052 factory-LSI contract](../docs/l052-factory-lsi-sysclk.md).

External manufacturer evidence is the selected own CN RM V1.5, CN DS V1.3
and SDK V1.4 in [`evidence-sources.json`](evidence-sources.json). The RM SHA-256
is `4bac53df4db69a3b76c833cb14dd45b0e5c7f9884506c83a5cda6ea00a859f41`,
the DS is `f03e2c3545b52942b576f669e3de69e1962631834ef6a01e51b45996fbeb3e7f`,
and the SDK is `d05a1ff5749ad8c06a51ad8a34d53bc85e30b1350478bcabc2858c2e58d8fa1f`.
Originals, extracted text and unapproved SDK members remain external evidence;
these page locators and project contracts do not redistribute them.

- RM PDF57/59/74 (printed56/58/73): factory halfword 0x00100A02, native
  ten-bit TRIM, WAIT[11:10], STABLE[15] and stopped-only trim programming.
- DS PDF43/51 (printed42/50): 32,800 Hz nominal, 31,816–33,784 Hz factory
  range at 1.65–5.5 V, VDDA=VDD and −40–85°C; the conditional +105°C row
  does not extend that tolerance. Buses have a 24 MHz ceiling below 1.8 V
  and 48 MHz at/above 1.8 V, independent of Flash latency.
- RM PDF64/66/67/70–74 and112 (printed pages one less): direct source
  switching, HSI /6 fixed CCS escape, native request/status/divider identity
  and Flash sequencing. HSI /6 has an 8.16 MHz factory upper rate before any
  bus-divider credit; undivided factory HSI has an upper rate of 48.96 MHz.
- RM PDF94/142/144/146/148/157/166–169/175/184–185/192–195/200,
  224–225/235–237/359/362/390/536/538/552–554: MCO/GPIO, AUTOTRIM,
  RTC, LPTIM, UART and LCD source/control identities. GPIO FLTCLK7 is LPTIM
  PWM; no standalone AWT exists. Work-off LCD/LPTIM gates are not opened.
- DS PDF10/27/33/79 and RM PDF146: exact packages and PC4/AF6 route.
  PC4 is unbonded on C8T6; both R8 packages expose it at physical pin24.
  Conservative inspection of its register grants no C8T6 output token.

The native source graph includes internal MCO-to-timer edges, PCLK-derived
ADC/timers/SPI/I2C, GPIO/filter cascades, independent IWDT RC10K and WWDT PCLK.
Source checks do not prove external pin feedback is harmless; that belongs to
the whole-bank functional handover. No RF or PLL capability is added.
Software build/source review is separate from hardware timing, fault recovery
or sleep/wake qualification. Final main verification passed eight actual library
builds and eight ELF links with zero warnings, plus generation and six finite
Python source/data checks. See the [metadata review](../docs/l052-factory-lsi-metadata-review.json)
and [runtime/source review](../docs/l052-factory-lsi-runtime-review.json).
At main acceptance, clean replay had not run; final-package completion requires
a separate final-source two-library/three-ELF clean receipt. Earlier failed
attempts remain historical records as described in the contract, not successful
checks or hardware tests.


## Exact CW32L083 factory-LSI SYSCLK

Only CW32L083RBT6/RCT6/RCS6/MCT6/VCT6 gain the independent init-only
factory-LSI capability; generic L083 and native L010/L011/L012 remain excluded.
See the [source contract and finite verification scope](../docs/l083-factory-lsi-sysclk.md).
Earlier LSE-only qualification records keep their historical meaning.

Authority remains the selected own RM CN V2.0, DS CN V1.9 and SDK V2.2 in
[`evidence-sources.json`](evidence-sources.json). RM SHA-256 is
`9930bf1755f3bbf8933163c2d0da57fd9a4f3250a358a4c0bfc75ed4eda3a0a3`, DS is
`852f772e9174cb76bf0f475f31f1e275254f8fe176bd3e7ad60d00b41db9509e`, and SDK is
`2d58765568d8dd8a218b52e4650aa6e5bd4f2e4b5f386196bae637dfc0b3fe73`. The selected
SDK member `IdeSupport/MDK/WHXY.CW32L083_DFP.1.0.9.pack` → `SVD/CW32L083.svd`
has SHA-256 `ddc840e2c52840ca3687fb78edeffff8ef7a41d6acd33cba32f44ec1b606a334`.
Originals and derived text remain external evidence; no vendor bytes or source
locks change for this addition. PDF pages below are one-based, with printed
page numbers one less.

- RM PDF58/62/79 and DS PDF47/55 establish factory halfword 0x00100A02,
  native ten-bit TRIM, preserved WAIT[11:10], nominal32800/min31816/max33784 Hz,
  1.65–5.5 V, VDDA=VDD and −40–85°C. Neither conditional +105°C nor adjustable
  trim range expands factory tolerance. HSI uses its separate 0x00100A00
  halfword, eleven-bit trim and 47.04–48.96 MHz factory envelope.
- RM PDF59–60/67/72/82 establishes actual PLL reference/bin/stop identity and
  requires departure through HSI/HSE. PDF63–65/75–85 separates startup status,
  sticky history, SYSCTRL IRQ4 and CLKFAULT IRQ31. Its direct HSE/LSE fallback
  text promises neither retained dividers nor HSE-fed PLL recovery. New LSI
  policy uses raw 48.96 MHz no-divider-credit coverage and retains Flash WAIT2;
  DS PDF47 and RM PDF67/77 keep voltage/bus limits independent of latency.
- RM PDF86–89/103/157–170/181/187/195–207/236/247–249/370/377/397/544/562–563
  binds sixteen gate/reset pairs, all six UART source fields, whole GPIOA…F,
  MCO, RTC, AUTOTRIM, LPTIM and LCD. Off work gates remain unread/off. The
  source graph does not import L052 UART timeout/timer or internal MCO-to-timer
  selectors; external clock feedback remains part of functional handover.
- DS PDF10/27–33/36–38/86 and RM PDF157–159 bind exact memory/bonds and all
  PC4/PF2/PD5 LSI and PC12/PC13/PF8 PLL alternatives. RBT6 is 128 KiB Flash,
  the other four 256 KiB, all 24 KiB SRAM. Complete bonded route counts are
  LSI1/1/1/2/3 and PLL2/2/2/3/3. Unbonded LSI PF2/PD5 and PLL PF8 AF0 guards
  are conservative register policy, not newly bonded output tokens.

Full cold ownership applies even with matching stopped trim; live matching LSI
is preserved. Six-bank inspection can advance unrelated activity before failure.
RTC LSI aliases on exactly these five parts become rate-only under every SYSCLK;
their numeric facts and exact nominal calendar fraction remain unchanged.
The all-54 comparison intentionally admits both new exact-five LSI metadata and
bonded PLL_OUT projections. Main generation, seven finite source/data commands,
ten actual libraries, ten ELFs and independent implementation reviews passed.
At main acceptance clean had not run; final-package completion requires a
separate two-library/three-ELF clean receipt. None of
these software obligations is a hardware timing, recovery, RF or sleep/wake claim.

## Exact CW32L010 live factory-LSI SYSCLK

CW32L010F8P6/F8U6/Y8M6 gain only init-time reuse of an already software-requested,
stable, factory-matching LSI. That L010 addition left generic L010 and all L011/L012 excluded;
ordinary POR, stopped startup and automatic-only requests are not admitted.
The [live-only contract](../docs/l010-live-factory-lsi-sysclk.md) and its
[source qualification record](../docs/l010-live-factory-lsi-qualification.json)
bind the own RM CN1.2, DS CN1.3 and SDK1.0.9 inputs already retained in the
existing source lock. No vendor bytes or historical source hashes are changed.

Own RM PDF 54/printed 53 forbids changes to running LSI parameters; PDF56/55
and 72/71 bind the factory halfword and ten-bit TRIM, while PDF69/68 distinguishes
automatic requests from software LSIEN. Own DS CN1.3 supplies the exact memory,
package and factory-rate envelope. RM PDF89/printed88 binds IRQ4, named RCC by
the RM; the separately cited device header and native SVD name it SYSCTRL.
Manual page numbers belong to the RM, not to the header member. F8P6/F8U6 keep
their DFP corroboration; Y8M6 remains DS-only without an invented DFP device.

The selected native SVD explicitly records SYSCTRL.AHBEN.KEY as read-write;
own RM PDF79/printed78 gives WO and the 0x5a5a write-key semantics. The policy
preserves that raw-SVD access fact separately from the operational RM rule.
APBEN1.KEY and APBEN2.KEY remain write-only in the selected SVD. The corrected
source checker uses exact per-register access expectations without weakening
field, gate, source-member or historical qualification equality.

Independent main runtime and source/metadata reviews accepted the frozen
implementation. Main verification passed eight actual libraries and seven
ELF links. Six finite source/data/layout gates passed in separate receipts,
including the corrected L010 source gate and all 43 locked hardware originals;
the aggregate future recipe was not executed. Three existing unused LSE-constant
warnings remain in L010/L011 main rows. At main acceptance clean had not run;
final-package completion requires a separate final-source two-library/two-ELF
clean receipt. No hardware, watchdog-survival or oscillator-continuity result
follows from these software receipts.


## Exact CW32L012 live factory-LSI SYSCLK

Only CW32L012C8T6 (LQFP48) and CW32L012C8U6 (QFN48), each 64 KiB Flash and
8 KiB SRAM, gain the separate [live-only contract](../docs/l012-live-factory-lsi-sysclk.md).
Generic L012 remains excluded. The Stage75 exclusion of all L011 is historical;
the separate L011 addition below does not rewrite that acceptance. Own CN RM1.4, EN RM1.0, CN DS1.0 and
SDK1.0.5 retain their vendor-original identities, versions and hashes. Raw and
curated register-access facts stay unchanged. The evidence registry only appends
reviewed qualification-evidence references.

CN RM PDF58/printed32 and EN PDF60/34 prohibit modifying running LSI parameters;
CN59/33 and EN63/37 bind the factory halfword at 0x001007C2. Own nine-bit TRIM,
RFU9, WAIT[11:10] and STABLE15 are distinct from L010's ten-bit layout. DS58/55
Table 7-19 qualifies nominal 32,800 Hz, 29,520–36,080 Hz at 1.7–5.5 V and
−40–85°C. CN72/46 and EN76/50 document automatic LSI request from IWDT
KR=0xCCCC; CN453/427 and EN495/469 separately source the counter from RC10K.
Neither fact permits watchdog-based admission or a cold-request inference.

DS39/36 and40/37 bind both PB11 AF4/physical22 and PF3 AF2/physical44 on each
package. Both complete Pin records are projected; scalar PB11 is only the old
nonexhaustive compatibility anchor. EN88/62's UART3 work-gate meaning conflicts
with CN; EN626/600 versus636/610 and SDK disagree on I2C master raw3. Timer
routing tables also differ. No inactive-looking state or borrowed closed work
gate resolves these conflicts. Raw-HSI RTC source3 stays 96 MHz with unchanged
first120/second400000; no native LSI calendar alias is introduced.

Stage75 main verification passed seven actual libraries, six linked ELFs and
six finite source/data gates, with independent runtime and metadata/source
acceptance. At main acceptance clean replay had not run; final-package completion
requires a separate final-source two-library/two-LSI-ELF clean receipt. Actual
54-object/904-file comparison found only two new LSI records and four appended
LSI_OUT Pin tuples in the two exact L012 chip JSONs and shared metadata_0021.rs:
three changed files, no additions/deletions, all other 901 files and 52 chip
objects unchanged. Board qualification remains unverified.
The accumulated local recipe, historical LSE qualifications and earlier family's
accepted receipts do not supply these results. No HAL tests, probes, register
models, hosted CI or hardware execution are added.

## Exact F030/A030 owned MCO qualification

The [exact MCO catalog](../cw32-data/mco-qualified.yaml) covers CW32F030C8T7,
CW32F030K8T7, CW32F030K8U7 and CW32A030C8T7 only. It retains the existing
[Stage76 C8T7 qualification](../docs/f030-mco-qualification.json). Additional
Stage77 source records are `docs/f030k8t7-mco-qualification.json`,
`docs/f030k8u7-mco-qualification.json` and `docs/a030c8t7-mco-qualification.json`;
the extension has separate Stage77 main implementation/build acceptance.
Record names and historical acceptance do not themselves certify the extension.

Source authority remains the selected shared x030 RM CN2.5, F030 DS CN1.9,
A030's own DS CN1.1 and F030 standard-peripheral SDK V2.2 identities in
[`evidence-sources.json`](evidence-sources.json). The shared RM explicitly covers
F030/A030. The F030 SDK retains its own provenance and is supplementary for
A030; it is not an A030-owned SDK. No new vendor-original identity, hash,
license inference or redistribution scope is introduced. Full PDFs, extracted
text and rendered source-review pages remain external evidence.

- Shared RM PDF61/printed60 describes MCO; PDF94/printed93 specifies SYSCTRL
  offset 0x70, SOURCE[3:0], DIV[6:4] and reserved[31:7]. Disabled/HCLK/PCLK
  codes are 0/1/2; codes 0..7 select /1, /2, /8, /64, /128, /256, /512 and /1024.
  RM PDF147/printed146 supplies PA8 AF4.
- F030 DS Table5-2 PDF24/printed23 records PC14/PC15 absent in both K8 package
  columns. PDF26/printed25 places PA8 at LQFP48 pin29, LQFP32 pin18 and QFN32
  pin18; Table5-3 PDF29/printed28 supplies AF4 MCO. The existing QFN32 exposed
  VSS position 0 convention remains unchanged. Table7-4 PDF38/printed37 gives
  ordinary voltage/ambient and junction limits, with power at TA=105°C of
  364 mW for LQFP48, 357 mW for LQFP32 and 541 mW for QFN32. These package
  limits are not interchangeable. Table7-27/Figure7-7 PDF51/printed50 supplies
  SPEED1 I/O values and their conditional fmax definition.
- A030's own DS PDF22/printed21 records PC14/PC15 on pins3/4; PDF23/printed22
  places PA8 at LQFP48 pin29 and PDF26/printed25 gives AF4 MCO. Own Table7-4
  PDF35/printed34 supplies ordinary −40..105°C, voltage/junction restrictions
  and 364 mW LQFP48 power at TA=105°C. Own Table7-26/Figure7-7 PDF48/printed47
  supplies SPEED1 I/O data; F030's table/page identities are not substituted.
- Locked `cw32f030_gpio.h` line837 corroborates PA8 AF4; `cw32f030_rcc.h`
  lines402–440 and `cw32f030_rcc.c` lines906–914 corroborate source/divider facts.
  Exact member paths and hashes remain in the source lock.

Both own I/O tables list SPEED1 50/30/20 MHz at respectively 30/50/50 pF under
their documented pad-supply ranges. The HAL retains its conservative 20 MHz,
2.4–5.5 V, normal ambient −40–105°C and declared total-load 0 < CL <= 50 pF
software policy. The vendor calls those tables design/simulation data, explicitly
unmeasured; fmax additionally requires loaded duty45–55% and tr+tf<=2T/3.
Numerical rate admission and cycle-qualified logical bounds do not prove those
conditions, receiver suitability or pin-level pulses. Init and MCO declarations
must truthfully describe the same board; frozen Clocks cannot verify equality
with the earlier declaration. Actual waveform/current/package-power/thermal
qualification remains required. See the [complete contract](../docs/f030-owned-mco.md).

Current permanent data policy admits exactly four of all 54 selections and
excludes the other 50, including generic family/density aliases and no-PA8
F030F8V7/QFN20 and F030F6P7/TSSOP20. Both K8 parts require PC14/PC15 identities
and LSE routes absent everywhere, without removing GPIOC or die-level LSE
register facts. Both C8T7 parts require those LSE reservations; all four require
PF0/PF1 HSE reservations. No generic family discovery grants an API.

Stage77's independent one-time generated comparison passed: only the three
additional chip JSONs gain the complete MCO fact and PA8/AF4 route; the other
51, including F030C8T7, remain byte-identical. The 905 generated files comprise
416 data and 489 PAC files. All 54 wrappers resolve to 39 metadata bodies;
six wrapper include numbers change, while 51 resolved selected bodies retain
their bytes. Exactly 15 generated paths change, with no additions/deletions.
Runtime code apart from its first scope comment, the three schema models and
the generated metadata template remain unchanged.

Main verification passed seven real library builds and five ELF links: all
twelve commands exited zero with no warnings. The 1,330 authored and 905 generated
build-input files stayed unchanged during the matrix. Eight finite source/data
gates passed, including 22 existing metadata contracts and all 43 required
hardware originals; two discovery HTML records remain excluded, so acquisition
of all 45 catalog entries is not claimed. Independent main runtime and metadata
reviews accept the frozen implementation. The initial generation failure on the
retained C8T7/C8T6 source roster and its exact-roster repair remain recorded;
the repair grants no C8T6 MCO capability.

At main acceptance final-source clean had not run; completion requires a
separate two-library/two-ELF and finite-gates receipt, using F030K8U7 and A030C8T7
from a fresh extraction and target. The current local recipe matches 7/5; no
aggregate recipe or hardware execution supplied the results, and no hardware
output has been measured. Main-source and later final-source identities remain
separate; these status updates are not retroactive build inputs.

### Historical Stage76 MCO evidence

The original qualification, [schema](../docs/f030-mco-schema-acceptance.json),
[runtime](../docs/f030-mco-runtime-acceptance.json) and
[metadata](../docs/f030-mco-metadata-acceptance.json) reviews keep their original
C8T7-only scope and identities. Only that exact part gained the initial MCO fact
and PA8 route; the optional schema shape then changed shared generated metadata
without granting other parts an API. Historical main verification produced five
actual ARM libraries and three firmware ELFs; all eight Cargo builds passed
with no warnings. Generation, the 904-to-905-file comparison across all 54
selections, 19 schema contracts and eight finite source/data gates passed, with
original failures retained in their historical receipts.

At Stage76 main acceptance clean had not run; its separate required final-source
replay was one exact F030C8T7 release library and one HCLK ELF. Those historical
5/3 main and 1/1 clean scopes remain separate from Stage77 7/5 main and 2/2 clean.
No aggregate recipe or hardware execution was run. No hosted CI, HAL tests,
probes or register models are added, and no historical receipt supplies new
implementation, physical-output or loaded-waveform acceptance.


## Exact CW32L011 live factory-LSI SYSCLK

Only CW32L011K8T6 (LQFP32) and CW32L011K8U6 (QFN32), each 64 KiB Flash and
6 KiB SRAM, gain the separate [live-only contract](../docs/l011-live-factory-lsi-sysclk.md).
Generic L011 remains excluded. This addition retains the selected own RM CN1.1
(June 2026 cover), DS CN1.1 (July 2025 cover) and SDK1.0.3 identities in
[the evidence authority](evidence-sources.json). The own
[qualification record](../docs/l011-live-factory-lsi-qualification.json) binds
the facts independently of previous LSE acceptance.

One-based physical PDF/printed locators use those own editions:

- DS 8/5, Table 3-1 and 34/31, Table 6-1 establish exact package and memory.
  DS 51/48, Table 7-19 gives nominal 32,800 Hz, factory −10/+25%, hence
  29,520–41,000 Hz at 1.7–5.5 V, VDDA=VDD and −40–85°C. The 25°C-only ±1%
  row does not narrow the complete envelope. DS 72/69 records the 2025-05-19
  Table 7-19 change. Its old printed-page locator 1 is corrected to 69 without
  changing physical page 72 or any source bytes; both values and the rendered
  basis remain in the new proof.
- RM 54/53 retains the warning about adjustment outside 32.8 kHz ±10%.
  The RM 0.4% trim-step/30–36 kHz range and DS 0.16% typical trim step remain
  unresolved. The DS factory envelope does not withdraw the RM warning or
  qualify every arbitrary trim, analog operating point or strict cycle time.
- RM 52/51 prohibits modifying running oscillator parameters; 62/61 describes
  LSI SYSCLK; 65/64 and 66–67/65–66 give SYSCLK3 and software LSIEN. RM 70/69
  gives ten-bit TRIM, WAIT[11:10], STABLE15 and factory halfword 0x001007C2.
  Raw 0xffff is refused before masking; zero is valid. RM 55/54 warns STABLE
  can remain set after running failure. ISR LSIRDY is a retained event, not
  current readiness. No LSI parameter/request or IWDT operation prepares entry.
- RM 140/139, 152/151 and 154/153 give the common 1 MHz RTC prescaler ceiling,
  SOURCE0/1/2/3 = LSE/HSE/LSI/raw HSIOSC and actual eight-bit PSC1+1.
  This HSE-off slice rejects SOURCE1 after the real gate window is restored;
  SOURCE3 requires original factory-ready raw HSI and raw PSC1 ≥97. Widen
  before adding one so raw 255 gives divisor 256. RTC WAIT2 is load status,
  not a source-readiness test. Native RTC stays raw 96 MHz HSIOSC.
- RM 68/67 binds SYSCTRL FLASHWAIT to the same-function FLASH WAIT. The current RM
  labels CR2[9:7] RFU while CMSIS/SVD names bit7 RTCLPM; preserve the bit without
  assigning new semantics. Full 4.08 MHz effective-HSI fallback budgeting
  grants no new fault recovery. SYSCTRL IRQ4 remains distinct from CLKFAULT31.
- DS 29–32/26–29 lists MCO PA4 AF4, PA8 AF4 and PC13 AF1, and raw HSIOSC_OUT
  PB0 AF3. No direct bonded LSI_OUT route is supplied. Own UART1/2/3, GPIOA/B/C,
  integrated RTC/AWT and timer observers retain their identities; no L012
  GPIOF/output/nine-bit/RC10K facts are imported. L011 IWDT is LSI-clocked
  (RM 384–386/383–385), without an admission scan or survival guarantee.

The narrow target requires Config.hse=None, Config.lse=None and original
HSEEN/HSECCS/LSECCS=0, with HSI or LSI entry and already requested/stable,
factory-matching LSI. Inherited unmonitored LSE remains protected. Selected
SDK/SVD/member facts corroborate own register shapes; vendor startup/example
code is not evidence that Rust initialization executed it. The two additional
startup/example members are identified within the locked archive in the new
proof, not promoted to new individually locked canonical members.

Historical LSE proof booleans, source IDs/hashes/sizes/URLs/licenses/member
chains, native RTC source/profile and all 54 Pin records retain their scope.
Apart from the one DS printed-page correction, the authority only appends
approved evidence references. Final main verification passed seven actual
libraries and seven linked ELFs, all with exit zero; generation and seven
finite data gates passed in separate invocations. The 905-file comparison
found exactly two L011 JSONs and their shared metadata body changed, with
902 files unchanged. Removing the two new LSI objects restored complete
54-chip equality. Three existing LSE-constant warning headers remain; earlier
checker failures and the first main diagnostic correction remain in external
history. At main acceptance clean had not run; completion requires a separate
final-source two-library/two-ELF receipt. The unchanged cumulative local recipe
is not execution evidence.


## Native exact7 live factory-LSI RTC

`rtc-live-lsi-qualified.yaml` 通过 [自身来源记录](../docs/native-live-factory-lsi-rtc-qualification.json) 定位三个原生族的现有锁定原件；复用已审厂校频率边界，不改默认 HSIOSC `rtc-calendar.yaml`。精确范围为 L010F8P6/F8U6/Y8M6、L011K8T6/K8U6、L012C8T6/C8U6。

- L010 CN RM1.2：书页139的分频、142的停止/运行访问条件、151的 SOURCE/WAIT/ACCESS、153的 PSC；CN DS1.3书页42为厂校 LSI 范围。
- L011 CN RM1.1：书页139的分频、142的直接访问、151的 SOURCE/WAIT、153的 PSC；CN DS1.1书页48为 −10/+25% 厂校范围，RM的调整警告仍保留。
- L012 CN RM1.4：书页165的分频、168的直接访问、176的 SOURCE/WAIT、178的 PSC；CN DS1.0书页55为厂校范围。各自 SDK 的 `rtc.h` 仅佐证源编码，不替代手册。

SOURCE2 与除数 `(1, 16400)` 对应名义日历 /32800；源和秒率仍仅有 rate-only 资格。新能力只复用已软件使能、稳定、厂校匹配的 LSI，不提供冷启动或低功耗唤醒；L010新源时间修改及全部新源异步闹钟等待明确拒绝。本批主验证已产生12个实际库和7个链接ELF，19条命令全部exit0。gen-all与8条有限source/data门分别通过，905个生成data/PAC文件路径和bytes完全不变。本批冻结实现已获运行与metadata/source独立主验收。L010/L011行保留三个既有未使用LSE常量warning头，不能称零warning。在main验收时clean尚未运行；最终包完成仍要求另行记录最终源码两库/两ELF clean收据，不得由main结果代替。

首轮main在第一库遇到build.rs中非Copy access取值的E0507，未产生库；仅改为引用比较保留完整相等性后，原19行主矩阵全部通过。初次失败与修正历史保留，不改写旧stage结果，也不构成实板、连续性或计时精度验证。

## Exact-eight owned LSE AWT run-mode sources

The [current contract](../docs/lse-awt.md) and
[own-source record](../docs/lse-awt-evidence.json) reuse eleven already locked
originals: five own manuals and six own datasheets. They add evidence locators,
not vendor revisions, selected artifacts, hardware YAML or source-sharing
metadata. All existing source IDs, URLs, hashes and historical receipts retain
their scope. In particular,
[`low-power-timers-evidence.json`](../docs/low-power-timers-evidence.json)
remains byte-for-byte historical, including its old F020/F030/A030 IP labels.

The exact cohort is CW32F020C6U7, CW32F030C8T7, CW32A030C8T7,
CW32L031C8T6, CW32L031C8U6, CW32L031F8U6, CW32R031C8U6 and CW32W031R8U6.
Current canonical inputs already use `awt_cw32f030_v1` for F020/F030/A030 and
`awt_cw32l031_v1` for L031/R031/W031, both with LSE=3. F002/F003 retain
`awt_v1` and its two distinct HEX pad selectors; general aliases, unqualified
packages, LPTIM and RTC's separate internal wake timer gain no admission.

The following physical PDF pages are one-based; these manuals' printed page
labels are PDF minus one. F030/A030 explicitly share x030 authority; each
other family uses its own manual.

| Own manual | PCLK configuration / AWT gate | Timer / CR / UD flags | GPIO FILTER / loss detector |
| --- | --- | --- | --- |
| `CW32x030_UserManual_CN_V2.5.pdf` | 56 / 84 | 168–169 / 173 / 174 | 158 / 60 |
| `CW32F020_UserManual_CN_V1.4.pdf` | 54 / 82 | 165–166 / 170 / 171 | 155 / 58 |
| `CW32L031_UserManual_CN_V1.6.pdf` | 55 / 79 | 160–161 / 165 / 166 | 142, 150 / 59 |
| `CW32R031_UserManual_CN_V1.3.pdf` | 57 / 81 | 162–163 / 167 / 168 | 144, 152 / 61 |
| `CW32W031_UserManual_CN_V1.4.pdf` | 56 / 80 | 161–162 / 166 / 167 | 143, 151 / 60 |

These pages distinguish the independently gated PCLK configuration domain
from the AWT counting source; bind SRC=3, MD=3, PRS1..15, EN/reload ordering,
ARR+1 cycles and R1W0 UD/no-op63; and identify native L031/R031/W031 FILTER7
as AWT underflow. F020/x030 FILTER tables qualify 0..6 only. The lack of a
source-edge update acknowledgment supports local control after counting-source
loss while CPU/PCLK/gate/reset remain operational; it is not a measured bus
latency or fault-transition result. x030/F020 mandatory CCS remains HAL
admission policy, not a hardware-fixed monitor claim.

Exact PC14/PC15 package positions and LSE electrical conditions retain these
own datasheet authorities. Full identities and page-text hashes are in the
new record and existing LSE qualification.

| Own datasheet | Physical PDF locators |
| --- | --- |
| `CW32A030_DataSheet_CN_V1.1.pdf` | 22, 27, 35, 41, 42 |
| `CW32F030_DataSheet_CN_V1.9.pdf` | 24, 30, 38, 44, 45 |
| `current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf` | 23, 28, 36, 42, 43 |
| `CW32L031_DataSheet_CN_V1.9.pdf` | 26, 30, 38, 45, 46 |
| `CW32R031_DataSheet_CN_V1.2.pdf` | 5, 9, 11, 29, 33, 42, 52, 53 |
| `CW32W031_DataSheet_CN_V1.3.pdf` | 4, 7, 9, 30, 32, 41, 51, 52 |

R031's −40–85°C row retains its own PDF5/9/11 locators; W031's retains
PDF4/7/9 and the existing 2.0–3.6 V LDO/DCDC intersection. Typical startup
and nominal 32768 Hz do not prove a maximum startup time or board cycle bounds.
The active board capability must continue to cover loading, drive, amplitude,
waveform, supply, temperature and aging.

Existing `LseClock` acquisition/reacquisition can temporarily enable whole
GPIOC for pad verification and advance retained filters/edges even before an
error. AWT health snapshots do not perform that inspection. Retained native
FILTER7 recipients can change or stall when AWT is replaced, reconfigured,
stopped, dropped or loses its source; forgetting can leave them driven. This
functional handover boundary adds no all-recipient-quiescence prerequisite or
new bank scan. An independently owned live RTC cannot share the source owner;
retained RTC hardware, LSE/LSI and persistent pad reservations remain untouched.

Stage80 main produced 11 actual ARM libraries and 8 linked ELFs with zero
warnings: a successful library prefix and resumed ELF tail with identical
1,347-file source snapshots, modes/bytes and toolchain. Main generation exited
zero; all 905 generated files, including all 54 chip records, retained identical
paths and bytes. The four separately invoked finite gates passed, covering
current LSE AWT source/data, full locked-source provenance, module layout and
canonical data. Independent runtime and metadata/source reviews accepted the
frozen main implementation.

The initial RCC-name assertion failure produced no artifact. After its
correction the 11-library prefix passed, then the first ELF attempt stopped
for missing locked offline `cordyceps 0.3.5`. Official locked fetching preserved
the lockfile, and only the 8-ELF tail resumed. These failures remain recorded;
there was no single uninterrupted successful 19-row invocation.

At main acceptance clean had not run; final completion requires a separate
final-source 2-library/2-ELF receipt, its independent ordinary generation,
whole54/905 comparison and the same four finite gates using the same absolute
source cache. Main acceptance does not replace clean replay or publication.
Compilation/source review cannot establish runtime loss handling, cancellation,
detector latency, electrical validity or silicon behavior; no cumulative HAL
recipe expansion follows.

## BTIM coalesced overflow IRQ sources

The [overflow-event contract](../docs/btim-overflow-irq.md) and
[current own-source record](../docs/btim-overflow-irq-evidence.json) cover
BTIM1/2/3 on all thirteen existing families and 54 normalized selections,
including aliases: 162 selection/instance associations across six register
variants. The record identifies twelve own manual PDF/text pairs, twelve
existing SDK archives and 36 selected members: each SDK's CMSIS header,
BTIM header and BTIM implementation. F030/A030 explicitly share x030 authority;
every other family retains its own manual and SDK evidence.

These are additional locators for already locked originals in
[`evidence-sources.json`](evidence-sources.json), not new vendor files or
revised identities. The record binds IRQ20/21/22, own-bank addresses, bit0
OV/UIF enable/status and direct R1W0 overflow-only masks `0x06`/`0x40`.
It independently records L052 `BTIM2_LPTIM`, L083 `BTIM2_LPTIM1` and L012
`BTIM3_HALLTIM`, including each partner's distinct bank/gate/reset. Native
MMS=0 still forwards UG onto TRGO; overflow IRQ qualification does not prove
isolation from retained recipients or lossless event history.

The earlier `docs/btim-counter-evidence.md` and `.json` retain their historical
polling scope and results. No canonical hardware schema, register identity,
source version, source hash or redistribution boundary changes. Final main
verification passed 16 actual ARM library builds and 23 ELF links, all exit zero;
the four finite source/data gates passed separately. The 1,353 source files and
905 generated files stayed unchanged during the final matrix; no main generation
was performed. Three existing native L010/L011 RCC warnings remain in applicable
rows. Independent main runtime and metadata/source reviews accepted the frozen
implementation. At main acceptance clean had not run; final-package completion
requires a separate final-source 2-library/6-ELF receipt.

The initial main attempt stopped intentionally after 25 successful commands
(16 libraries/nine ELFs) for a visibility-only correction of eight introduced
warnings; it is not a full-matrix receipt. The first provenance invocation
failed on a stale derived reference index, refreshed before the final gates.
These earlier results and the complete final rerun remain recorded in
[the verification history](../docs/btim-overflow-irq.md#verification-boundary).
Source provenance and software builds do not establish interrupt execution,
sleep/wake, fault recovery or silicon behavior.

## Exact-part F020 owned DMA and staged UART TX sources

The [F020 contract](../docs/f020-dma-owned.md) and
[own-source record](../docs/f020-dma-owned-evidence.json) qualify native
CH1/CH2 safe owned SRAM copies and staged byte UART TX only on
CW32F020C6U7/K6U7/F6U7. They reuse locked F020 originals and do not
replace earlier x030/L083 records or infer F020 behavior from those families.

- `CW32F020_UserManual_CN_V1.4.pdf`, SHA-256
  `279521eaf7d32b241e614a9553044a4d99c1e7ca2566efa3b0fdd2b66c106eed`:
  own printed pp93–94 bind independent DMACH1 IRQ9 and DMACH2 IRQ10;
  pp124–126, 128, 132–138 bind software/hardware BLOCK, request routing,
  clean TC, STATUS5, software SOFTSRC and native two-channel registers.
  ICR p135 supplies the four-field R1W0 reset/no-op seed `0xFFFFFFFF`
  added through the existing `register-writes.yaml` schema. UART pp282/287
  supply the byte TDR handshake, EN-before-DMATX sequence and separate
  wire completion. One-based PDF viewer pages equal these printed pages + 1.
- `current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf`, printed Rev1.3,
  SHA-256 `1e330d800f10114c654b97cbd45b64d56a99c3cb39138d4ef20de3c12968dab0`:
  printed p7 binds the exact three parts to 32 KiB flash and 8 KiB SRAM.
  Its own package pad tables and the pinned SDK AF macros bind the example
  routes; the older same-named root file with printed Rev1.2 is not substituted.
- `CW32F020_StandardPeripheralLib_V1.2.zip`, SHA-256
  `1d77fece47a0c615c8ae51374ea946b17ab489042222f33e38d93e6969945b5d`:
  its bound CMSIS, DMA/GPIO/UART headers, UART implementation and native
  SVD provide corroboration. Stale SDK extra-channel aliases or Channel2/3
  comments do not override the two-channel manual and independent IRQs.

The record retains exact source/member identities, official URLs and own
page locators. Generic `cw32f020` has no SRAM metadata and retains
`MissingMemoryMetadata` rejection. F6 UART1 still has no safe TX pad;
PA13/PA14/PF3 exclusions remain. This Stage84 record qualified no RX/combined
UART. The separate [Stage92 classic RX record](../docs/classic-uart-dma-rx.md)
now supplies its own receive qualification; SPI, other peripheral consumers
and borrowed hardware-request APIs remain excluded.
TE, EN-clear and request closure alone provide no bus-drain proof;
errors and inconsistent completion permanently retain the owners.

Stage84 main passed 15 actual ARM library builds and 16 ELF links, all 31
commands exiting zero on an unchanged 1,391-file source snapshot. Only the
L010F8P6 and L011K8T6 library rows emitted three existing RCC-constant
warnings each; all F020 rows and all ELF links were warning-free.
One ordinary generation produced 906 generated files: the native ICR JSON
was added, the native DMA PAC helper file changed, and the other 904 existing
files, including all 54 chip JSON files, remained byte-identical. The
capability renderer and five finite source gates passed in documented
segments. Initial missing-cache-manifest and mutable-backlink comparison
failures remain recorded; the approved checker-only correction after ARM
builds changed no HAL or example bytes. Independent actual-main runtime
and metadata/source reviews accepted the bounded work. At main acceptance
clean had not run; final-package completion requires a separate final-source
2-library/4-ELF clean receipt. Source qualification and linked-image
inspection do not establish hardware transfer, wire timing, throughput or
finite liveness. See [the verification boundary](../docs/f020-dma-owned.md#verification-boundary).

## Native four-channel owned DMA and staged UART TX sources

The [four-family contract](../docs/four-channel-dma-owned.md) and
[own-source record](../docs/four-channel-dma-owned-evidence.json) independently
bind CW32L031 CN V1.6, CW32L052 CN V1.5, CW32R031 CN V1.3 and
CW32W031 CN V1.4 manuals, their own datasheets and SDK members. These are
the existing locked originals; identity, URL, digest, byte size, revision,
package/AF inventories and canonical layout reuse remain unchanged. New
consumer evidence backlinks do not repin an original. Earlier DMA records
and their verification receipts retain their historical family/stage scope.

| Own manual | DMA IRQ | Software BLOCK / finite setup / terminal | ICR | HARDSRC / SOFTSRC | Byte SRAM-to-TDR setup |
| --- | --- | --- | --- | --- | --- |
| L031 CN V1.6 | 90 | 123 / 127 / 128 | 131 | 133 | 344 |
| L052 CN V1.5 | 95 | 128 / 132 / 133 | 136 | 138 / 139 | 381 |
| R031 CN V1.3 | 92 | 125 / 129 / 130 | 133 | 135 | 347 |
| W031 CN V1.4 | 91 | 124 / 128 / 129 | 132 | 134 | 347 |

These are printed pages, also zero-based PDF indices; one-based viewer pages
add one. The evidence binds every own UARTx contract, UART1/2/3 base table,
derived SVD endpoint and selector, not only the UART1 worked example.
All four native controllers have CH1/DMACH1 IRQ9, CH2/3/DMACH23 IRQ10,
and CH4/DMACH4 IRQ11. ISR and ICR have TC/TE pairs at bits0/1, 4/5,
8/9 and 12/13. Own ICR reset/no-op is `0xFFFFFFFF`; typed selected-field
writes preserve sibling flags and reserved bits. No fifth channel is inferred.

The complete admitted exact cohort is CW32L031C8T6/C8U6/F8P6/F8U6/K8U6/K8V6,
CW32L052C8T6/R8S6/R8T6, CW32R031C8U6 and CW32W031R8U6, each with
64 KiB flash and 8 KiB SRAM. The four generic aliases retain empty memory
and safe-admission/staging rejection. All eleven existing package route sets
and 31 selected TX endpoints are listed in [the TX scope](../docs/uart-dma-tx.md#native-four-channel-cohort).
L031F8P6/F8U6 have no safe UART1 TX route because their only bonded TX pad
is excluded PA14; PA13/PA14/PF3 policy is unchanged. L052 uses its own PA9
AF1 UART3 route. L031/R031/W031 retain the vendor spelling
`PA10_AFx_SUART3TXD` with AFR10=6, corroborated by the own datasheet's
UART3_TXD route, without rewriting source inventory.

The only hardware-data addition is the native eight-field ICR command seed
in `register-writes.yaml`. One ordinary generation added
`cw32-data/data/register-writes/dma_cw32l031_v1.json` and appended the native
ICR reset/no-op helpers to `cw32-metapac/src/peripherals/dma_cw32l031_v1.rs`,
taking the generated inventory from 906 to 907 files. Independent generated-byte
review accepted exactly these two paths: the other 905 existing files,
including all 54 chip JSONs, stayed byte-identical; the native PAC's previous
full content remained an unchanged prefix. No register layout, part, pin,
request, vector, enum, schema or generator changed. Administrative coverage
projections are separate from that hardware-data delta.

This Stage85 record qualified owned SRAM copying and private-staging UART TX.
[Stage92 classic RX](../docs/classic-uart-dma-rx.md) separately supplies finite
RX/combined qualification. Existing raw software-copy contracts remain; borrowed
hardware constructors remain x030-only. SPI, other requests, early abort/recovery
and buffered-plus-DMA ownership remain excluded.
TE, STATUS2, EN-clear, request closure and clock-off are not bus-drain proofs;
errors or unconfirmed completion permanently retain resources. R031 GPIOA
whole-bank effects and W031 internal RF boundaries preserve the existing
functional handover obligations and establish no RF-safe coexistence.

Stage85 main passed six standalone ARM library builds and 21 ELF links,
all 27 commands exiting zero without warnings. The separately retained 21
HAL dependency archives from ELF builds are not additional standalone library
commands. All 1,402 source files and 907 generated files stayed unchanged
during that ARM matrix, as did the toolchain and Cargo configuration.

The seven planned non-ARM commands passed in segments: generation and DMA
source projection in the initial segment; own DMA facts, provenance, capability
rendering, capability consistency and module layout in the successful final
tail. This is one generation, five finite source gates and one renderer,
not a claim that one uninterrupted seven-command run passed. Three failed
own-source-checker attempts remain recorded: text nodes required their parent
artifact's ID; clock fact comparison incorrectly included narrative notes and
consumer evidence; and L052's own SVD used standard `lsb`/`msb` fields rather
than `bitOffset`/`bitWidth`. These were checker preparation/schema errors,
not HAL execution or hardware failures. The corrections retained strict source
identity and normalized hardware-fact comparisons, changed no HAL/example
bytes or locked original facts, and supplied no runtime evidence. The final
SVD parser correction happened after the ARM matrix and changed only the
source checker; the successful final source tail used that corrected checker.

At main acceptance clean had not run; final-package completion requires a
separate final-source 2-library/4-ELF clean receipt. No cumulative HAL recipe
expansion or hardware/RF execution is part of this scope. Source gates and
linked-image inspection do not establish silicon transfer, wire timing,
error-time drain, RF coexistence or finite liveness. See
[the verification boundary](../docs/four-channel-dma-owned.md#verification-boundary).


## Stage92 classic finite UART DMA RX sources

The [classic RX contract](../docs/classic-uart-dma-rx.md) and
[own-source ledger](../docs/classic-uart-dma-rx-evidence.json) add positive
byte-RDR, software-only RC-clear, FE/PE-without-ORE/NE and package qualification
for fourteen exact F020/L031/L052/R031/W031 parts. Own manual receive examples
are F020 CN1.4 p288, L031 CN1.6 p345, L052 CN1.5 p382, R031 CN1.3 p348 and
W031 CN1.4 p348; printed pages equal zero-based PDF indices. The ledger retains
the selected original/member/text identities and complete safe RX/TX routes.
L031F8U6 has UART1 RX-only on PA9/AF2 or PC15/AF6 without a safe TX route.
Generic SRAM rejection, native channel/vector topology and PA13/PA14/PF3
exclusions remain. L012 UART DMA remains TX-only. The Stage84/85 results above
are historical copy/TX receipts, not Stage92 RX acceptance. Stage92 main passed
eight standalone ARM libraries and 21 ELF links, all 29 exiting zero without
warnings. Both finite source checks passed after the preserved initial D01
PIN/AFR comparison failure and checker-only correction. All 910 canonical
generated files stayed unchanged; main ran no canonical data generation. Independent
runtime and metadata/source reviews accepted the bounded main result. At main acceptance final capability reports
and the separate final-source two-library/three-ELF clean replay had not run.
No hardware result is claimed.

## L012 原生 DMA 复制和 UART TX 资格

本批仅 CW32L012C8T6/C8U6，沿用自身 CN RM V1.4、DS CN V1.0、SDK V1.0.5
及已锁 EN RM V1.0 作佐证。DMA 寄存器定位 CN PDF143–149 对应书页117–123；
UART 字节 SRAM 到 TDR 示例 CN PDF500 / 书页474，UART 基址/CR2/TDR/ISR
与各自 SVD/CMSIS 对应。两包 64 KiB Flash / 8 KiB SRAM，PA9/PA7/PA8 AF1
为 UART1/2/3 TX；通用别名无安全内存资格。锁定原件、许可和旧证据不改。

五个 CCNT 原 SVD/header RW 与手册 RO 的冲突按既有寄存器覆盖修正；
十个 CSR TC/TE 字段原 SVD/header 已 RO，补既有访问 sidecar 是恢复导入遗漏，
不是原厂勘误。STATUS 保持 RW。新增原生枚举及 ICR 全一 R1W0 seed 均来自本族。
SDK 清中断 helper 的掩码域及 flag stride 错误保留原文，不作为实现模板。
原生 PAC 类型和可写接口变化须由使用方迁移，不能用适配层重新暴露错误写权限。

主 4 个独立 ARM 库 / 11 ELF 已全部 exit0、零 warning，1413 源文件与工具链未漂移。
实际生成 907→910，仅十二条本族输出差量，54 chip JSON 不变；
原件引用的新增消费链接可逆，当前来源门不依赖历史全文件摘要。
详见[本族合同](../docs/l012-dma-owned.md)与[逐项证据](../docs/l012-dma-owned-evidence.json)。

主验收时 clean 尚未运行，最终包需独立 2 库 / 4 ELF 与来源检查收据。
七项非 ARM 命令分段成功：第二次 gen 与 DMA 投影先过，随后五项尾门通过。
首次 gen 因新证据把 EN 旧 pin_history 当成当前原件引用而被 provenance 拒绝，
事务回滚保留旧 907 产物；只显式标记两条历史 path/hash 并在 checker 精确逆还原。
随后 checker 把 CSRCADDR 误纳 CSR 前缀，改为严格 CSR/CSR1..4 匹配后通过。
原失败与修正记录保留，未改变锁定身份、旧历史、原件或运行算法。

## 经典 I2C 异步主机来源

Stage87 仅为既有十二个非 L012 家族增加 SI 中断等待的七位主机接口；
五种既有寄存器版本、十七个家族/控制器实例、独占 IRQ23/24 与各自
APBRST 位按本族手册、SVD 和设备头文件核对，不借用 L012 命令/FIFO 语义。
来源继续记录在 [i2c-typed-register-evidence.json](../docs/i2c-typed-register-evidence.json)；
原始锁、旧证据、寄存器 YAML 和引脚数据不因此改写。

各自 SI W0 推进、STO 硬件清零且写零无动作、F8 无中断及仲裁丢失状态
决定异步等待与有限同步 STOP 的边界。任意 EN 关闭不是隐藏状态机复位
证明；确认关闭且无待决 STOP 后，复用先走自身独占复位位的断言/解除
读回，再配置本地时钟。该事实不证明物理总线恢复、其他主机不受影响或
目标设备无副作用。主验收已覆盖十三个实际 ARM 库、十七个最终 z-profile
ELF 与六项有限来源/数据命令，910 个生成文件字节保持不变。L010/L011
保留三类既有 RCC warning；历史失败不计通过。F002/F003 例程使用自身
已公开安全路由 PB1 SCL AF3 / PA6 SDA AF3，保留 PA5 的 debug 排除策略。
F020 的旧 timing PDF 与当前 pinout PDF 分别绑定各自已锁身份，修正的是
本次实施遗漏而非原件或原冻结来源事实。主验收时 clean 尚未执行；最终
源码包必须另有六库/六 ELF clean 收据。详细限制及独审见
[classic-i2c-async.md](../docs/classic-i2c-async.md)，主结论分别见[运行审阅](../docs/classic-i2c-async-runtime-review.json)与[来源审阅](../docs/classic-i2c-async-metadata-review.json)。

## GTIM/ATIM counter events and native classic ATIM input sources

The [bounded contract](../docs/timer-event-input.md) and
[current own-source evidence](../docs/timer-event-input-source-evidence.json)
cover 34 GTIM and eleven ATIM family/instance overflow associations and native
classic ATIM input qualification on F030, A030, F003, L031, R031, W031, L052 and
L083. The source record keeps separate `counter_events` and
`classic_atim_input` sections. Each family's own manual, datasheet and locked
SDK provenance supplies its actual register semantics, GLOBAL vector, AF input
cell and package bonding. Existing output routing is not input permission.

The native input catalog `cw32-data/classic-atim-input.yaml` projects the direct
`ClassicPairedV1` qualification and CAP1A/B, CAP2A/B, CAP3A/B routes onto the
37 admitted current selections. The other 17 retain no native-classic fact;
F002/F020 lack ATIM, while L010/L011/L012 keep their distinct buffered owners.
All 54 selections retain their existing exact-package or common-package-alias
policy, protected/debug/oscillator/radio exclusions, and time-driver ownership.
Raw pad spellings such as F003 PA00 are resolved to existing canonical GPIO
pins, never treated as new Rust identifiers or permission to use debug PA5.

The own classic ATIM capture acceptance fields operate independently of EN;
latest CCR reads, capture flags and overwrite flags have separate semantics.
Native QEI uses SMS4/5/6, fixed PRS=/1 and distinct OVF/UNDF observations. The
manual's phase and input pulse requirements are expressed in TCLK pulse widths,
not silently replaced by full periods or a measured maximum edge rate.
L012 GTIM's manual DIER name and its SDK/PAC `ier()` accessor remain an explicit
naming difference; L012 ATIM uses `dier()`. Selective R1W0 event acknowledgments
preserve the own masks and reserved reset-one bits.

Main acceptance records 16 actual ARM libraries and 27 linked ELFs. The ten
finite source/data gates passed through seven retained successes plus three
successful continuation commands, with eleven actual attempts and the failed
D08 receipt preserved. Its existing independent AF expected-route helper had
omitted the newly approved native-input catalog. Only that helper was extended;
full ordered actual-versus-expected equality, own-source proof, canonical facts
and generated projections were retained. No HAL test or lifecycle probe was
added. The ARM matrix similarly preserves the pre-compilation M24 lockfile
failure: restoring one local time-driver forwarding feature retained the
original pinned lockfile bytes and left the earlier 23 commands' actual inputs
unchanged. The 43 successful rows therefore span 44 attempts and two documented
manifest snapshots, not an uninterrupted same-tree run.

One main generation produced 910 canonical files with 77 allowed metadata
changes and 833 byte-identical files. Exact inverse checks restored prior JSON
values/array order and Rust metadata bytes after removing only the admitted
additions; canonical PAC register bodies and write policies stayed unchanged.
The earlier static buffered ATIM accessor correction used its existing own
`0x00ff3fff` clear seed, retaining the classic `write_noop()` helper without
changing PACs or source facts. Three existing L010/L011 RCC `dead_code` warnings
remain in applicable ARM rows.

At main acceptance clean had not run; final-package completion requires a
separate final-source 2-library/6-ELF receipt. Documentary qualification and
ordinary compilation/linking do not establish silicon capture accuracy,
lossless delivery, wake behavior or external timing limits. Prior BTIM/PWM/input
histories keep their original scope. The detailed correction and acceptance
history is in [the contract](../docs/timer-event-input.md#verification-history).
