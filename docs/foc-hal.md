# CW32L012C8 FOC peripheral HAL

This document describes the CW32L012C8 backend using RM v1.4 and audited chip-level signal routes. For the different F030 ADC, VC, timer and clock contracts, see [cw32f030-support.md](cw32f030-support.md). Schema v3 contains no package model; the application checks physical bonding and board wiring. It is not a port of the STM32G431 register model, a finished FOC algorithm, or board-level safety verification. No hardware, motor, debugger or programmer was connected.

## Actual hardware mapping

- STM32 TIM1 role → CW32 **ATIM**: three selected complementary pairs, common rising/falling dead time, external break input and update TRGO.
- **ADC1 and ADC2** are two real converters. Each has an eight-entry sequence. ADC2 CR.SLAVE can follow ADC1 START; this is implemented for bounded software-triggered paired sampling.
- **OPA1 and OPA2**, not three OPAMPs.
- **VC1–VC4**, with shared **VC12REF/VC34REF** divider resources; IRQ grouping is independently VC13/VC24.
- Dual-channel **DAC**, hardware **CORDIC**, and integer hardware **EAU**.

## Ownership and pin routing

Drivers hold the official `Peri<'d, T>` ownership type from `embassy-hal-internal =0.5.0`. `Peripherals` fields are `Peri<'static, peripherals::T>`; callers can move them into a driver or use `reborrow()` for a shorter exclusive lifetime. A motor owner can hold ATIM, ADC1/2, CORDIC and EAU. Their parent tokens cannot simultaneously be used by another driver or task. Copyable peripheral identities are not copyable ownership. Raw PAC access and unsafe `steal`/`clone_unchecked` remain explicit escape hatches. In v0.8, `into_async(binding)` transfers the same resources and lifetimes to an IRQ-driven wrapper; it does not clone them.

HAL build.rs generates sealed route traits from chip capability and pin-route metadata. ATIM consumes six output pins plus an external BK pin. ADC inputs consume distinct external-channel pins. GPIO `Pin` is now a sealed trait; an erased `Peri<'d, AnyPin>` cannot satisfy a peripheral signal trait requiring an audited concrete pin. SWD PA13/PA14 are deliberately not emitted as ATIM routes. Pin choices in examples demonstrate legal chip signal routes only; no unknown board wiring is assumed.

`init(Config) -> Peripherals` acquires resources; `try_init` is the fallible alternative. The configuration includes `config.rcc`, and `rcc::clocks()` returns the checked nominal clocks after successful initialization. There is no `Peripherals.clocks` field. The supported clock remains reset HSI/24 with undivided buses, nominally 4 MHz. The RCC stabilization poll limit does not add arbitrary clock-tree support.

All shared SYSCTRL read-modify-writes use critical sections, compatible with Cortex-M0+ without compare-and-swap. Clock gates are never disabled on driver drop. A reset marked shared in metadata is never asserted by a single-instance constructor: ADC1/2, OPA1/2 and VC/reference sibling drivers therefore cannot reset each other. Drivers initialize their own registers instead.

## PWM behavior and limitations

ATIM construction keeps MOE and CEN clear. Starting the counter is separate from explicitly enabling phase outputs. Auto-output-enable is disabled; a break does not automatically re-arm. Fault acknowledgement first disables output and clears only the relevant R1W0 flag bits. Drop disables MOE/CEN and disconnects owned pins. The external break input is required; this API does not yet offer internal VC-to-break routing or a second break input.

Dead-time requests are PCLK ticks with CKD=/1, not prescaled counter ticks. The 8-bit segmented encoding is decoded according to RM table 17-12; requests round upward and values above 1008 ticks are rejected. Zero dead time is an explicit accepted configuration, not a safe recommendation for a power stage. The configured idle states are low and active polarities are non-inverted, but construction holds CCER=0: by RM table 17-13 the outputs are high-impedance, not actively driven low. The board must provide gate-driver disable or pull resistors to establish a safe level. Gate-driver electrical behavior, pull resistors, fault polarity/filter choice and dead time must be reviewed for the actual board.

Three duty values are preloaded together. The driver temporarily sets UDIS to avoid a partly updated phase tuple being latched. A coincident update/TRGO can be suppressed, so an ADC trigger may be skipped. This API does not promise an uninterrupted, jitter-free FOC sampling schedule. Duty values cannot exceed ARR. No dynamic dead-time or lock-level programming is exposed.

## ADC behavior and limitations

ADC constructors select independent one-shot sequences, keep peripheral IRQs disabled and wait 32 μs after EN to cover analog startup plus automatic BGR startup. Only audited external pin channels are exposed; TS, internal BGR and internal OPA source ownership are not fabricated. The clock policy conservatively limits ADCCLK to 6 MHz across supported supply conditions. Sample times are explicit hardware encodings, and results are raw uncalibrated 12-bit values, not amperes or calibrated volts.

Software sampling and ADC1-master/ADC2-slave sampling have bounded iteration budgets. Timeout stops conversions and clears trigger routes; it is not a calibrated real-time deadline. Both real ADCs must report complete sequences for paired sampling.

Blocking ATIM-triggered sampling is a bounded one-shot capture: after seeing EOS it disables further trigger acceptance, finishes any already-running sequence and returns the latest coherent completed sequence. It may return a later sequence than the first observed EOS. The IRQ-driven equivalent has the same coherent/latest-sequence boundary, with cancellation instead of an internal poll budget. Neither is a continuous DMA stream or a proof of zero sample loss. Polling watchdog thresholds are supported; watchdog IRQ waits and linking watchdog/VC outputs to ATIM are not exposed yet.

## Interrupts, concurrency, and math

v0.8 保留原阻塞构造器，同时实现 ADC、VC、ATIM 事件和 CORDIC 的真实 IRQ future。`into_async(binding)` 要求 `bind_interrupts!` 生成的对应 Binding 并转移整个 driver 的所有权；等待没有事件时返回 Pending，由实际 handler 唤醒。ADC2_DAC 与 VC13/VC24 共享向量上的 handler 各自只处理本实例的 enabled/pending 状态；取消不关闭或清 pending 整条共享 NVIC。若 DAC 也启用 IRQ，应用必须自行实现并加入它的 handler。完整签名和共享向量示例见 [异步 API](async-api.md)。

这不是连续采样流水线。ADC `sample().await` 等待软件触发的完整序列，`sample_atim_update(&timer).await` 在首次 poll 后等待 ATIM update trigger；两个独立 async ADC 等待不等于已证明同步的主从采样，`sample_pair` 仍为阻塞 API。VC 提供 rising/falling/any-edge/high/low 等待，但单个 INTF 合并多个事件，不保留完整边沿历史。CORDIC 等待 EOC 中断；EAU 没有经审查的完成 IRQ，仍为有界阻塞接口。

ATIM 的 `wait_update().await` 不启动 counter；`wait_break().await` 可立即报告已有故障锁存，但不确认故障或开启输出。取消 ATIM future 只撤销 UIE/BIE 等待，**不会停止已有 counter、关闭已有功率输出、清除 break 锁存或禁用硬件 break 保护**。显式 `disable_outputs`、`acknowledge_fault` 和 `enable_outputs` 的责任不变；取消等待不是急停。drop 整个 owner 才沿用关闭输出/counter 的清理。VC 边沿 future 也不会建立 VC→ATIM 的硬件保护路径。

所有 async 等待借用 `&mut self`，同一 owner 只能有一个活跃等待；ATIM 的 update 与 break 不能同时作为两个独立 future 持有。ADC/CORDIC 提供 `into_blocking`，VC/ATIM 当前没有逆转换。各 future 都没有内置实时时限，外部 timeout/select 必须实际 drop 它们才取消。ADC 取消停止转换并清 trigger/EOS；VC 取消清本源选择/INTF，保留模拟 comparator；CORDIC 取消复位自己的专用 accelerator，丢弃该次结果。未配置外部 trigger 或 IRQ 无法服务时可以无限 Pending。

CORDIC uses the real CSR/X/Y/Z hardware interface and Q1.31 input/output with explicit operation domains, scales and bounded busy waits. Its polar magnitude is **half magnitude**, following the manual, not a mislabeled full hypot. Q1.15 mode and hardware accuracy validation are not claimed. EAU exposes checked signed/unsigned division and integer square root; division by zero and signed MIN/-1 are rejected before triggering.

## Validation

Board programs live only under the root example directory. Build verification does not establish board-level safety.

## Analog drivers

`analog::Bandgap::new` consumes BGR, enables BGREN while preserving TSEN and waits 32 μs. It intentionally never disables BGR, including on drop, because hardware may have enabled it for another live analog block.

`Dac::new` owns the unique dual-channel DAC and enables internal channel outputs, initially zero, with external output pads disconnected. `with_output1`/`with_output2` explicitly consume the corresponding chip pin token before enabling that external output. `set`/`set_pair` validate 12-bit codes. This is a raw voltage-code DAC, not calibrated voltage/current control.

`RefDivider` owns VC12REF or VC34REF. `Comparator` can use external inputs, borrow the correct reference divider, or borrow the DAC. VC1/VC2 share VC12REF; VC3/VC4 share VC34REF. Separately, VC1/VC3 use DAC1 and VC2/VC4 use DAC2. The lifetime borrow prevents dropping/reconfiguring a live reference or DAC during comparator use; changing such a DAC threshold requires ending the borrow. `AsyncComparator` retains these borrows and adds shared-IRQ edge/level waits; no digital output pin, window/blanking scheme or ATIM break route is implemented.

`Opa` supports follower, PGA, external-feedback and DAC-follower configurations, consumes the actual output pin, and borrows Bandgap (and DAC when used). OPA and DAC therefore cannot both safely drive an already-consumed shared pin. OPA constructors do not claim calibration. `calibrate` explicitly requests hardware calibration and must observe AZRUN become high then low before reporting success; missing the pulse, zero budget or never-ending busy yields a conservative Timeout. The calibration period follows the RM rather than the conflicting SDK comment. Calibration completion and subsequent settling are not proofs of analog accuracy.

OPA 的两寄存器块没有经审查的校准完成 IRQ；`calibrate` 保持明确的阻塞接口，不用 async 包装忙轮询。OPA/DAC/Bandgap/RefDivider 初始化和模拟稳定等待也仍同步。资料边界见 RM §§29.3–29.6 及 [逐外设语义](full-peripheral-semantics.md)。

Internal OPA-to-ADC source channels are not yet exposed through a lifetime-safe ownership API. Consequently the external ADC APIs and OPA APIs do not constitute a complete internal OPA→ADC current-sensing pipeline. No closed-loop motor-control algorithm is included.


## Independent review fixes and reproducible checks

Manual review found and fixed three concrete ordering issues: ADC trigger routes are now disabled before the busy check or configuration writes; paired-sampling early errors use the same cleanup path; ATIM sets MOE only while CCER is disconnected, checks the fault latch again, and never writes MOE=1 after channels connect. ADC CR reserved bits are preserved, including the RM's reserved reset bit 8. OPA calibration additionally waits for explicit busy acceptance. Software ordering review does not establish silicon timing.

Only BKF=0 provides the documented asynchronous break path without a running filter clock. Nonzero break filters require clock operation and the board/system's fault-safe clock strategy; the limited reset-clock HAL does not configure that strategy.





Chip selection and the critical-section implementation belong to the application. The GTIM1 time driver reserves its timer, provides nominal 1 MHz timestamps, and requires overflow servicing within 65.536 ms. No FOC timing or hardware safety is implied.

