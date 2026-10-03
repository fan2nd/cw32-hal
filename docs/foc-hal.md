# CW32L012C8 FOC peripheral HAL

This document describes the CW32L012C8 backend using RM v1.4 and audited chip-level signal routes. For the different F030 ADC, VC, timer and clock contracts, see [cw32f030-support.md](cw32f030-support.md). Schema v5 contains no package model; the application checks physical bonding and board wiring. It is not a port of the STM32G431 register model, a finished FOC algorithm, or board-level safety verification. No hardware, motor, debugger or programmer was connected.

## Actual hardware mapping

- STM32 TIM1 role → CW32 **ATIM**: three selected complementary pairs, common rising/falling dead time, external break input and update TRGO.
- **ADC1 and ADC2** are two real converters. Each has an eight-entry sequence. ADC2 CR.SLAVE can follow ADC1 START; this is implemented for bounded software-triggered paired sampling.
- **OPA1 and OPA2**, not three OPAMPs.
- **VC1–VC4**, with shared **VC12REF/VC34REF** divider resources; IRQ grouping is independently VC13/VC24.
- Dual-channel **DAC**, hardware **CORDIC**, and integer hardware **EAU**.

## Ownership and pin routing

Drivers hold the official `Peri<'d, T>` ownership type from `embassy-hal-internal =0.5.0`. `Peripherals` fields are `Peri<'static, peripherals::T>`; callers can move them into a driver or use `reborrow()` for a shorter exclusive lifetime. A motor owner can hold ATIM, ADC1/2, CORDIC and EAU. Their parent tokens cannot simultaneously be used by another driver or task. Copyable peripheral identities are not copyable ownership. Raw PAC access and unsafe `steal`/`clone_unchecked` remain explicit escape hatches. `Adc<I,M>` and `Comp<I,M>` use actual mode-bearing owners; async construction requires an exact Binding. Specialized ATIM/CORDIC owners retain explicit async conversion without cloning resources.

HAL build.rs generates sealed route traits from chip capability and pin-route metadata. The specialized ThreePhasePwm consumes six output pins plus an external BK pin; generic SimplePwm owns independently optional channels. ADC reads borrow distinct external-channel pins for the call or explicit Sequence lifetime. GPIO `Pin` is now a sealed trait; an erased `Peri<'d, AnyPin>` cannot satisfy a peripheral signal trait requiring an audited concrete pin. SWD PA13/PA14 are deliberately not emitted as ATIM routes. Pin choices in examples demonstrate legal chip signal routes only; no unknown board wiring is assumed.

`init(Config) -> Peripherals` acquires resources; `try_init` is the fallible alternative. The configuration includes `config.rcc`, and `rcc::clocks()` returns the checked nominal clocks after successful initialization. There is no `Peripherals.clocks` field. The default clock is reset HSI/24 with undivided buses, nominally 4 MHz; the checked 96 MHz HSI/APB-div2 configuration is also supported. The RCC stabilization poll limit does not add arbitrary clock-tree support.

All shared SYSCTRL read-modify-writes use critical sections, compatible with Cortex-M0+ without compare-and-swap. Clock gates are never disabled on driver drop. A reset marked shared in metadata is never asserted by a single-instance constructor: ADC1/2, OPA1/2 and VC/reference sibling drivers therefore cannot reset each other. Drivers initialize their own registers instead.

## Specialized three-phase PWM behavior and limitations

The separate [Timer/SimplePwm layer](timer-pwm.md) provides ordinary ATIM/GTIM
counter and optional-channel PWM. The following stronger motor-specific
contracts describe `atim::ThreePhasePwm`, not every timer API.

ATIM construction keeps MOE and CEN clear. Its preload UG is issued with master trigger outputs gated, so construction does not spuriously trigger an already-armed ADC or downstream timer. Starting the counter is separate from explicitly enabling phase outputs. Auto-output-enable is disabled; a break does not automatically re-arm. Fault acknowledgement first disables output and clears only the relevant R1W0 flag bits. Drop disables MOE/CEN and disconnects owned pins. The external break input is required; this API does not yet offer internal VC-to-break routing or a second break input.

Dead-time requests are PCLK ticks with CKD=/1, not prescaled counter ticks. The 8-bit segmented encoding is decoded according to RM table 17-12; requests round upward and values above 1008 ticks are rejected. Zero dead time is an explicit accepted configuration, not a safe recommendation for a power stage. The configured idle states are low and active polarities are non-inverted, but construction holds CCER=0: by RM table 17-13 the outputs are high-impedance, not actively driven low. The board must provide gate-driver disable or pull resistors to establish a safe level. Gate-driver electrical behavior, pull resistors, fault polarity/filter choice and dead time must be reviewed for the actual board.

Three duty values are preloaded together. The driver temporarily sets UDIS to avoid a partly updated phase tuple being latched. A coincident update/TRGO can be suppressed, so an ADC trigger may be skipped. This API does not promise an uninterrupted, jitter-free FOC sampling schedule. Duty values cannot exceed ARR. No dynamic dead-time or lock-level programming is exposed.

## ADC behavior and limitations

`Adc::new_blocking` and Binding-based `Adc::new` construct a mode-bearing peripheral owner, keep completion subscriptions disabled, and wait 32 μs after EN to cover analog startup plus automatic BGR startup. `blocking_read`/async `read` borrow a channel per call. `configure_sequence` returns a fixed-N Sequence borrowing the owner and its channel tokens; it does not permanently bake pins into the ADC owner. Only audited external pin channels are exposed; TS, internal BGR and internal OPA source ownership are not fabricated. The clock policy conservatively limits ADCCLK to 6 MHz across supported supply conditions. Sample times are explicit hardware encodings, and results are raw uncalibrated 12-bit values, not amperes or calibrated volts.

Software sampling and ADC1-master/ADC2-slave sampling have bounded iteration budgets. Timeout stops conversions and clears trigger routes; it is not a calibrated real-time deadline. Both real ADCs must report complete sequences for paired sampling.

Blocking ATIM-triggered sampling is a bounded one-shot capture: after seeing EOS it disables further trigger acceptance, finishes any already-running sequence and returns the latest coherent completed sequence. It may return a later sequence than the first observed EOS. The IRQ-driven equivalent has the same coherent/latest-sequence boundary, with cancellation instead of an internal poll budget. Neither is a continuous DMA stream or a proof of zero sample loss. Polling watchdog thresholds are supported; watchdog IRQ waits and linking watchdog/VC outputs to ATIM are not exposed yet.

## Interrupts, concurrency, and math

ADC and comparator use actual `Blocking`/`Async` modes, with `new_blocking`
and Binding-based `new` constructors. ATIM event and CORDIC wrappers keep their
existing resource-transfer API. Futures return Pending and are woken by actual
handlers. ADC2_DAC and VC13/VC24 handlers touch only their own enabled/pending
source; cancellation never disables or unpends the shared NVIC vector. A DAC
IRQ user must also supply its handler. See [async contracts](async-api.md).

`Sequence::sample().await` is software-triggered; `sample_atim_update(&pwm)`
waits for a real ATIM trigger after first poll. Independent async ADC reads
are not a proven synchronous pair; L012 `sample_pair` remains bounded blocking.
VC waits notify edges/levels, but one hardware flag can coalesce transitions.
CORDIC waits for EOC; EAU has no audited completion IRQ and remains bounded
blocking.

ATIM `wait_update` does not start the counter. `wait_break` can report an already
latched fault, but never acknowledges it or enables outputs. Cancelling these
waits removes only the subscription: it does not stop an active power stage,
clear break protection or perform emergency shutdown. Owner Drop shuts down
outputs/counter. L012 VC waits do not create a hardware VC-to-ATIM protection
route; F030 has a separate checked brake guard.

Each wait borrows its owner mutably. No implicit async deadline exists;
external timeout/select must drop the future to cancel. ADC cleanup stops and
disarms conversion, clears only its completion subscription, and preserves
shared analog/IRQ resources. VC cleanup retains the enabled comparator and
removes only its event selection. CORDIC cancellation resets its own dedicated
accelerator and discards that result. CORDIC retains `into_blocking`; ADC and
Comp no longer use conversion-wrapper APIs.

CORDIC uses the real CSR/X/Y/Z hardware interface and Q1.31 input/output with explicit operation domains, scales and bounded busy waits. Its polar magnitude is **half magnitude**, following the manual, not a mislabeled full hypot. Q1.15 mode and hardware accuracy validation are not claimed. EAU exposes checked signed/unsigned division and integer square root; division by zero and signed MIN/-1 are rejected before triggering.

## Validation

Board programs live only under the root example directory. Build verification does not establish board-level safety.

## Analog drivers

`analog::Bandgap::new` consumes BGR, enables BGREN while preserving TSEN and waits 32 μs. It intentionally never disables BGR, including on drop, because hardware may have enabled it for another live analog block.

`Dac::new` owns the unique dual-channel DAC and enables internal channel outputs, initially zero, with external output pads disconnected. `with_output1`/`with_output2` explicitly consume the corresponding chip pin token before enabling that external output. `set`/`set_pair` validate 12-bit codes. This is a raw voltage-code DAC, not calibrated voltage/current control.

`RefDivider` owns VC12REF or VC34REF. `Comp<I,M>` can use external inputs, borrow the correct reference divider, or borrow the DAC. VC1/VC2 share VC12REF; VC3/VC4 share VC34REF. Separately, VC1/VC3 use DAC1 and VC2/VC4 use DAC2. The lifetime borrow prevents dropping/reconfiguring a live reference or DAC during comparator use; changing such a DAC threshold requires ending the borrow. Async mode retains these borrows and adds shared-IRQ edge/level waits. Constructors configure the unit disabled; explicit `enable(&mut delay)` settles it. Reads/waits reject a disabled unit. No L012 digital output pin, window/blanking scheme or ATIM break route is implemented.

`Opa` supports follower, PGA, external-feedback and DAC-follower configurations, consumes the actual output pin, and borrows Bandgap (and DAC when used). OPA and DAC therefore cannot both safely drive an already-consumed shared pin. OPA constructors do not claim calibration. `calibrate` explicitly requests hardware calibration and must observe AZRUN become high then low before reporting success; missing the pulse, zero budget or never-ending busy yields a conservative Timeout. The calibration period follows the RM rather than the conflicting SDK comment. Calibration completion and subsequent settling are not proofs of analog accuracy.

OPA 的两寄存器块没有经审查的校准完成 IRQ；`calibrate` 保持明确的阻塞接口，不用 async 包装忙轮询。OPA/DAC/Bandgap/RefDivider 初始化和模拟稳定等待也仍同步。资料边界见 RM §§29.3–29.6 及 [逐外设语义](full-peripheral-semantics.md)。

Internal OPA-to-ADC source channels are not yet exposed through a lifetime-safe ownership API. Consequently the external ADC APIs and OPA APIs do not constitute a complete internal OPA→ADC current-sensing pipeline. No closed-loop motor-control algorithm is included.


## Independent review fixes and reproducible checks

Manual review found and fixed three concrete ordering issues: ADC trigger routes are now disabled before the busy check or configuration writes; paired-sampling early errors use the same cleanup path; ATIM sets MOE only while CCER is disconnected, checks the fault latch again, and never writes MOE=1 after channels connect. ADC CR reserved bits are preserved, including the RM's reserved reset bit 8. OPA calibration additionally waits for explicit busy acceptance. Software ordering review does not establish silicon timing.

Only BKF=0 provides the documented asynchronous break path without a running filter clock. Nonzero break filters require clock operation and the board/system's fault-safe clock strategy; the supported RCC configurations do not establish that board-level strategy.





Chip selection and the critical-section implementation belong to the application. The GTIM1 time driver reserves its timer, provides nominal 1 MHz timestamps, and requires overflow servicing within 65.536 ms. No FOC timing or hardware safety is implied.

