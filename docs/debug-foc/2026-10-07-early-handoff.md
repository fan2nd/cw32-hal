# 2026-10-07 提前接管与速度环实板调试

本轮从“有可用反电势后尽早接管”出发，完成了实际烧录和16轮自动运行。基线Git为569ed2c，沿用上一轮本地3S配置和12 V台架。最终控制参数连续两轮8 s试验及两轮20 s试验均正常停机，结束前处于ClosedLoop。后两轮分别在ClosedLoop维持17.70 s、17.47 s；最终一轮最后1.6 s估计速度平均24.991 Hz，原始估计范围24.064–25.919 Hz，Iq命令71–77 mA，32个诊断点无低反电势。

这验证当前台架、当前负载条件下的软件无感接管及有界运行；没有独立编码器测速、真实相电流探头或不同负载／母线范围验证。状态ClosedLoop和PLL速度均属于软件证据，不能据此给出机械RPM或绝对角精度。期间询问过实际转动表现，截至本记录未收到该问题的用户回复。

## 最终实现

1. 校准后上电3 s请求自动启动，5 ms自举、200 ms对齐；I/f仍以250 mA、2→18电气Hz加速。达到强制／估计速度12 Hz、反电势500 mV以及原角差／PLL／速度一致性判据后，连续80帧（20 ms）即可接管，不等待2.5 s平台。开环5 s超时保留。
2. 接管帧保持αβ电压，重表达PI／电流坐标。0.5 s Blend期间速度PI不运行，Id淡出，Iq保持接管值并至少以100 mA为目标；原2 mA/帧斜坡、250 mA矢量圆仍约束实际命令。没有在接管帧直接跳变电流参考。
3. Blend结束后以滤后当前速度和已有Iq初始化速度PI，再将速度参考向25 Hz爬升。原先Blend期间速度PI曾将Iq降为零，电势随后衰减而失锁；分开两个过程后首次完成ClosedLoop。
4. 反电势滤波alpha从18/64调为8/64，相位提前使用现有按alpha计算的公式；PLL10240/100恢复原值。速度反馈在4 kHz下以Q8、alpha=1/64低通，再送100 Hz速度PI。速度PI最终Q15 Kp/Ki=123/1；原始速度仍用于超速／失速保护。
5. 台架默认上电23 s停止，即自动请求后的20 s有界试验。专用BENCH_STOP_REQUEST在reload关桥并停高频源，冻结运行快照，fault=0表示正常到时停止。PA3普通停止在截止前保留原启停路径；故障及台架冻结结束后下一轮需复位。
6. 当前REPLAY v5捕获Blend起始age1..80；额外512 B环形缓冲只在既有50 ms诊断节拍保存最近32个闭环快照，停机后输出RUN_RESULT／RUN_HISTORY／RUN_ROW。运行中不输出RTT。接管后的强制角已冻结，因此最终固件不再计算这些状态的接管拒绝位。

供电阈值9.9–16 V、750 mA软件跳闸、采样时序和deadline、R=3.35 Ω/L=875 µH、相电流校准及全部故障锁存路径保留；未改共享HAL或01–06。

## 对照过程

- 仅提前评估、20 ms资格：约11.1 Hz成功进入Blend，但约62.75 ms后ObserverLost；说明不再只是等待资格。
- PLL Kp减半/Ki至四分之一：仍在约59.75 ms失锁，撤回该改动。
- 入口电势750 mV：最长69/80帧，StartupTimeout，恢复500 mV。
- EMF alpha8/64：Blend延长到376 ms，但末态Iq命令为0；因此暂缓速度PI到Blend结束。
- 保持过渡Iq后首次连续ClosedLoop约6 s，不过末段估计速度14–40 Hz、Iq在0–250 mA摆动。
- 降速度PI增益以及对4 kHz速度反馈低通，最终123/1配合滤波显著减小末段速度摆动。低通与过大的原增益组合曾在ClosedLoop失锁，未保留该组合。
- 8–10 Hz低速接管仍有失败，最低接管速度设为12 Hz。随后又发现12.2 Hz、入口投影Iq约48 mA仍会Blend失锁；成功保持过渡Iq的试验入口约95–123 mA，最终稳定运行命令约70–76 mA。因此只在Blend设100 mA下限，经原斜坡和电流圆加入。最终第一轮20 s试验入口投影Iq约60 mA，也完成了过渡。

这些是台架对照支持的整定结果，不是对所有电机通用的参数识别或唯一物理根因证明。不同启动的转子初始位置、实际机械轨迹不同，不能将多次试验当作完全相同的输入。

## 原始结果

state=4/5/6分别是OpenLoop/Blend/ClosedLoop；持续时间为停止前该状态的age/4000，不是总通电时间。表中均速来自最后最多1.6 s的50 ms快照，不能当独立机械测速。

| 日志 | fault | 停止前状态 | 状态持续s | 末段估计均速Hz |
|---|---:|---:|---:|---:|
| [2026-10-07-early20-01.log](2026-10-07-early20-01.log) | 8 | 5 | 0.063 | — |
| [2026-10-07-early20-pllhalf-01.log](2026-10-07-early20-pllhalf-01.log) | 8 | 5 | 0.060 | — |
| [2026-10-07-early20-emf750-01.log](2026-10-07-early20-emf750-01.log) | 9 | 4 | 5.000 | — |
| [2026-10-07-early20-filter8-01.log](2026-10-07-early20-filter8-01.log) | 8 | 5 | 0.376 | — |
| [2026-10-07-early20-iqhold-01.log](2026-10-07-early20-iqhold-01.log) | 0 | 6 | 6.034 | — |
| [2026-10-07-early20-iqhold-02.log](2026-10-07-early20-iqhold-02.log) | 0 | 6 | 5.787 | 28.404 |
| [2026-10-07-early20-speedhalf-01.log](2026-10-07-early20-speedhalf-01.log) | 0 | 6 | 5.621 | 28.507 |
| [2026-10-07-early20-speedki6-01.log](2026-10-07-early20-speedki6-01.log) | 0 | 6 | 5.702 | 27.627 |
| [2026-10-07-early20-speedfilter-01.log](2026-10-07-early20-speedfilter-01.log) | 8 | 6 | 3.470 | 27.446 |
| [2026-10-07-early20-speedslow-01.log](2026-10-07-early20-speedslow-01.log) | 8 | 5 | 0.062 | — |
| [2026-10-07-early20-min12-01.log](2026-10-07-early20-min12-01.log) | 0 | 6 | 5.545 | 25.421 |
| [2026-10-07-early20-final-01.log](2026-10-07-early20-final-01.log) | 8 | 5 | 0.061 | — |
| [2026-10-07-early20-torque100-01.log](2026-10-07-early20-torque100-01.log) | 0 | 6 | 5.396 | 25.437 |
| [2026-10-07-early20-torque100-02.log](2026-10-07-early20-torque100-02.log) | 0 | 6 | 5.396 | 25.494 |
| [2026-10-07-early20-long20s-01.log](2026-10-07-early20-long20s-01.log) | 0 | 6 | 17.702 | 24.888 |
| [2026-10-07-early20-long20s-final.log](2026-10-07-early20-long20s-final.log) | 0 | 6 | 17.468 | 24.991 |

`early-handoff-results.json`保留解析结果与每个日志SHA256。所有存在REPLAY的数据都核验为80行、n/age连续和完整尾标；有RUN_ROW的日志均为32行连续序号。未接管时REPLAY_UNAVAILABLE是预期结果，不补造数据。

注意：`early20-speedfilter-01.log`中RUN_HISTORY/RUN_ROW第四列标签曾错误写成filtered_speed，实际仍为PLL角残差；该列不能用于滤后速度分析。下一轮speedslow起修正了字段。更早RUN_HISTORY第四列本来就是PLL残差。最终固件第四列确实为滤后速度。

## 验证与交付状态

- 最终ARM release构建、07全部Rust格式检查及git diff --check通过；Cargo保留既有proc-macro-error2未来兼容性提示。
- 11项主机源码测试通过：加速中资格、低幅值／反向／速度差／角差拒绝、故障和正常停止快照、Blend不运行速度PI、Blend结束PI初始化、原始超速绕过速度低通、接管帧αβ电压逐值一致，以及低投影Iq入口的斜坡／电流圆约束。测试直接使用最终control/config/sampling/arithmetic源码，临时工程位于target/foc-early-tests，不作为硬件模型或机械仿真证据。
- 最终控制源码MODEL_TAG：`2ee9afd91c605dd442528146e4c228677e29dedc3e43b4791eced5424ea6cb5b`。它只覆盖四个指定源文件，不覆盖IO与诊断。
- 最终ELF SHA256：`a3bd1ebf2d87b4f78303162204f863cc67bd21be24edc3d4f4e9c2e8fdcbfacd`，路径`target/thumbv6m-none-eabi/release/bldc_07_sensorless_foc`；已通过probe-rs --verify烧录并完成最终20 s试验。
- 最终日志报告fault=0、armed=false、PWM/ADC triggers stopped。退出采集前已确认完整REPLAY_END，当前保持停止。源码和本目录记录尚未Git提交。
