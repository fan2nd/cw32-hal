# MT6701QT 配置工具（Rust + MCP2221A）

独立的电脑端命令行程序，通过 MCP2221A 的 USB HID 接口访问 MT6701QT 的 I²C 配置寄存器。无需串口、不烧录电机主控，也不修改 MCP2221A 的 Flash/GPIO。Windows 使用系统 HID 驱动。

支持读取角度和配置，设置 ABZ 分辨率、方向、零位、Z 脉宽、迟滞、UVW 极对数和 QT 辅助输出。所有配置都先读取再按位修改；未指定字段和保留位保持原值。尚未提供模拟/PWM 参数修改、I²C 地址修改或任意寄存器写入。

## 接线

以下是 **MT6701QT QFN-16 芯片引脚**，不是转接板排针编号：

| MCP2221A / 电源 | MT6701QT |
| --- | --- |
| SDA | 6：A/SDA |
| SCL | 7：B/SCL |
| GND | 16：GND，共地 |
| 编码器电源 | 13：VDD |
| VDD | 14：MODE，配置期间拉高 |
| VDD | 8：Z/CSN，配置期间拉高 |

SDA、SCL 使用上拉（参考值 4.7 kΩ），电平与两端供电兼容。EEPROM 编程要求编码器实际 VDD **大于 4.5 V、小于 5.5 V**，一般使用 5 V；`--vdd-mv` 是操作者输入的实测值，工具不会测量或调节供电。不要把外部 5 V 与转接板的 3.3 V 电源直接并联；接到 3.3 V 主控时需确认电平兼容。

配置时断开主控对 A/B/Z 复用引脚的驱动。配置完成后，把 MODE 拉低，并释放原来接高的 Z/CSN，6/7/8 才可作为 A/B/Z 使用。工具不控制 MODE 或电源开关。

QT 的 11/12/9 是 U/V/W；`--secondary inverted-abz` 将其选择为 -A/-B/-Z，而非正相 A/B/Z。不要只因接口标了 UVW 就把它当成 I²C 接口。

## 编译和使用

在仓库根目录执行（不要在带嵌入式默认 target 的 examples 目录执行）：

```powershell
cargo build -p mt6701-tool --release
$tool = '.\target\release\mt6701-tool.exe'
& $tool --help
& $tool devices
& $tool read
```

默认 USB VID:PID 为 `04D8:00DD`，I²C 为 7 位地址 `0x06`，时钟 100 kHz。已改地址的芯片用 `--address 0x46`；不要传 8 位读写地址 `0x0C/0x0D`。多个桥同时连接时，使用 `devices` 打印出的 `--serial` 或完整 `--path`，程序不会随意选一个。

先预览 1024 PPR（A/B 四倍频后 4096 计数／圈）：

```powershell
& $tool configure --abz --ppr 1024 --direction ccw --z-width 1
```

只临时写入配置并读取校验，不烧 EEPROM：

```powershell
& $tool configure --abz --ppr 1024 --direction ccw --z-width 1 --apply --backup ram-test.json
```

永久保存同样的配置（实际使用 5 V 供电时）：

```powershell
& $tool configure --abz --ppr 1024 --direction ccw --z-width 1 --eeprom --vdd-mv 5000 --backup encoder-1024.json
```

程序先完成备份落盘、配置写入与 RAM 回读校验，再发送 EEPROM 指令并静默等待 700 ms。随后**将编码器断电重上电，仍保持 I²C 接线**，执行：

```powershell
& $tool verify encoder-1024.json
```

`verify` 只读。它比较全部 11 个配置字节，不会自动断电，也无法识别操作者是否真的断过电；未断电的回读一致只能证明 RAM 一致。最后再切换 MODE 接线使用 ABZ。

## 参数和行为

| 选项 | 含义 |
| --- | --- |
| `--abz` | 清除 ABZ_MUX，选择 ABZ |
| `--ppr 1..1024` | 每圈 A/B 周期数；四倍频计数是其 4 倍 |
| `--direction ccw/cw` | 计数方向配置 |
| `--secondary uvw/inverted-abz` | QT 辅助输出选择 |
| `--pole-pairs 1..16` | UVW 极对数，和 ABZ PPR 独立 |
| `--zero 0..4095` | 零位寄存器值，每单位 360/4096 度 |
| `--z-width 1/2/4/8/12/16/180deg` | Z 脉宽；数字单位为 LSB |
| `--hysteresis 0/0.25/0.5/1/2/4/8` | 迟滞，单位 LSB |

不带 `--apply` 或 `--eeprom` 时，只读取并预览，不写配置。未指定参数保持当前值；`--ppr` 不隐式修改方向、输出复用或零位。

每次写入前保存 JSON，包括原值、期望值、寄存器地址、桥标识和编程选项。默认使用当前目录中的唯一时间戳文件名；指定 `--backup` 时拒绝覆盖旧文件。快照是在执行前生成的，**文件存在不代表执行成功**。工具没有自动恢复命令；原值可用于人工重设已支持的字段。

写入只发送差异字节；即使 RAM 已相同，显式 `--eeprom` 仍会发起保存，便于先 `--apply` 测试再保存。总线错误立即退出，不自动重发配置或 EEPROM 指令。中途失败可能留下部分 RAM 修改；先读取确认状态。编程指令响应丢失时同样等待 700 ms，之后应断电重上电验证，避免盲目重复烧写。

若总线故障后 MCP2221A 一直 busy，先检查供电、MODE、CSN、共地与上拉，再执行 `recover` 取消桥中的事务。它不重新编程编码器。程序无法通过芯片 ID 验证对端型号，应保证所选地址上的器件确实是 MT6701。

## 验证与实现依据

```powershell
cargo test -p mt6701-tool
cargo clippy -p mt6701-tool --all-targets -- -D warnings
```

单元测试覆盖全部 PPR 编码、共享寄存器位保留、配置范围、读回不匹配、HID 命令序列、重复起始读、NACK、错误回包和 EEPROM 等待。模拟传输测试不能代替实物验证。

2026-10-07 本机验证：Windows 构建、9 项单元测试和严格 Clippy 检查通过；USB 枚举结果为 0 个 MCP2221(A)，尚未进行实物寄存器读写或 EEPROM 编程验证。

同日后续接入实物：成功枚举 1 个 MCP2221(A)，在地址 `0x06` 读取全部配置及角度。初始配置为 ABZ、1024 PPR（4096 四倍频计数／圈）、CCW，辅助 U/V/W 为 -A/-B/-Z。寄存器按上文程序快照顺序读取为 `80 04 0F FF 00 00 00 03 00 00 00`。1024 四倍频计数／圈对应 256 PPR，只需将 `0x30` 从 `0x0F` 改为 `0x0C`，保留其余位；1024 PPR 则无需修改。此记录仅证明读取成功，尚不证明配置已写入或 EEPROM 已保存。

用户随后确认采用 ABZ 最高分辨率（1024 PPR / 4096 计数），实际供电 5 V，并要求保存 EEPROM。USB 重新连接后的保存尝试在读取 `0x25` 时失败：MCP `0x40` 返回 `status=0x41, state=0x51`。执行恢复后报告 STOP 超时 `0x62`，再次读取报告 I²C busy。本次未生成写前备份，未写配置或发送 EEPROM 保存指令；需恢复 I²C 接线/供电状态后继续。

- [MagnTek MT6701 Rev.1.8](https://www.magntek.com.cn/upload/pdf/202407/MT6701_Rev.1.8.pdf)：QT 接线见第 4、11、20 页；配置及 EEPROM 流程见第 28–32 页。
- [Microchip MCP2221A DS20005565D](https://ww1.microchip.com/downloads/en/DeviceDoc/MCP2221A-Data-Sheet-DS20005565D.pdf)：HID 命令和 I²C 传输结构。
- [Linux MCP2221 驱动](https://code.googlesource.com/linux/torvalds/linux/+/111d0bda8eeb4b54e0c63897b071effbf9fd9251/drivers/hid/hid-mcp2221.c)：桥固件内部状态码参考，未复制其实现。
- [hidapi Rust API](https://docs.rs/hidapi/latest/hidapi/struct.HidDevice.html)：HID 报告读写约定。
