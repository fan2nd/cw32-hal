# v0.14.0 ADC, analog and math comparison

All 18 existing files in these modules were compared with actual pinned Embassy
[ADC](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/adc),
[Comp](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/comp.rs),
[OpAmp](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/opamp.rs),
[DAC](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dac/mod.rs)
and [CORDIC](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/cordic/mod.rs),
and the relevant [L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)
and [F030 RM2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf)
sections. CORDIC formula tables were visually checked rather than trusting text
extraction of fractions and square roots.

## Corrected ADC lifecycle

Forgetting a Sequence can skip Drop and leave conversion/trigger/IRQ/watchdog
state behind. A new valid sequence now cancels that predecessor, disables its
watchdog selection and clears exposed stale watchdog faults before configuring
new channels. Argument validation happens first and invalid requests do not
write registers. A watchdog deliberately configured on the new sequence remains
active through its subsequent samples.

Both ADC backends retain their existing physical differences, including L012
per-slot sample time/paired sampling and F030 single EOC versus scan EOS and
single-channel-only watchdog. Other assigned drivers required no new semantic
change in this pass; their omissions are recorded below rather than hidden.

## Module matrix

| Existing file | Actual implementation comparison and result | CW evidence / exercised evidence | Remaining scope or justified difference |
|---|---|---|---|
| `adc/mod.rs` | Variant selection/reexports expose the two existing implementations and shared channel API without chip-name routing. No change needed. | Source inspection; both current chip builds. | Backend-specific SampleTime/config are real IP differences. |
| `adc/channel.rs` | Follows the pinned Embassy `AdcChannel`, `BorrowedAdcChannel`, `BorrowedChannel` ownership pattern. Sealed verified pins; no integer channel constructor; consumed and reborrowed channels retain lifetimes. No cosmetic rename required. | Fresh ARM positive and negative channel/owner probes on both chips. | Internal TS/reference channels are not exposed because resource/startup policies are unimplemented. No differential metadata exists here. |
| `adc/common.rs` | Real shared one-shot state machine, not STM32 register emulation. Register-before-latch avoids missed wake; ISR services only enabled pending source; disarm then clear then inspect busy handles a final retriggered scan without erasing its final completion. Cancellation masks, disarms, stops/reset position, clears, and resets latch/waker. | 32 fresh extracted-source cases across both backends; current EventState from `interrupt.rs`. | No DMA buffer ownership, streaming, or lossless first-edge guarantee. |
| `adc/l012.rs` | Corrected new-sequence recovery above. Existing owner/mode constructors and borrowed operation channels agree with useful upstream architecture; pin and peripheral ownership remain independent. Eight slots, per-slot samples, EOS completion, preserved reserved CR bit8, slave pair control and drop cleanup were retained. | RM ch25, especially 25.5.2 and 25.12.1–.3/.10/.19; PAC-backed current-source sequence probe; ARM API/borrow checks. | Hardware has two ADCs, shared reset, automatically enabled BGR and 12-bit outputs. Do not copy STM32 reset/unpend, internal-channel or DMA assumptions. Async paired sampling, general trigger routing and async watchdog notifications are omitted features. |
| `adc/f030.rs` | Corrected recovery above. Existing per-mode EOC/EOS selection, CHMUX for N=1, SQR slots for N>1, watchdog restriction, ADC READY budget and live BGREN/BIAS preservation checked. Both completion sources are masked on stop. | RM22.4.1/.2, 22.5.1/.5, 22.9, 22.13.1/.2/.8/.11; current-source PAC-backed probe and common lifecycle/ownership cases. | Single ADC, 1–4 slots, one shared sample time and single-channel-only watchdog are hardware differences. The driver retains a conservative 500 kHz policy for the full VDDA range, not a universal silicon maximum; the RM permits higher clocks at higher VDDA. Internal channels, accumulation/DMA and broader trigger APIs are omitted. |
| `analog/mod.rs` | Individual IP capability gating remains independent; OPA/L012 VC additionally require the BGR witness; DAC/reference optional APIs are gated locally. Private MMIO traits and generated identity associations are maintained. | Full source inspection; generated `VcInstance::Reference` uses the schema 7 instance association. Both chip builds. | `I::NUMBER` is diagnostic only. No runtime routing relies on an integer instance-number convention. |
| `analog/bgr/l012.rs` | Exclusive software-control token plus startup witness; constructor RMW preserves TSEN. Drop deliberately does not switch off an automatically/shared-enabled reference. No exact standalone STM32 BGR owner analogue. | RM25.12.19; source inspection; token lifetime rejection. | 32us is a conservative software delay around an approximately 30us specification, not measured board accuracy. |
| `analog/vcref/l012.rs` | Pair-owned divider and retained comparator borrow prevent live-reference destruction/reconfiguration. DIV accepts only 0–7; the modeled field is 3 bits, while the manual register table prints 3:0; source/enable programming validated before writes. | RM27.3.1 explicitly says valid DIV 0–7; 27.7.1/.2 pair association; domain/encoding probe; reference-pair and reference-drop ARM rejections. | VDDA/Vcore fraction is a raw electrical configuration, not calibrated volts. Pair topology is distinct from interrupt pairing. |
| `analog/dac/l012.rs` | Existing whole-DAC owner safely retains optional output pins and immutable analog-source borrows. Validated 12-bit single writes and atomic pair holding-register writes match CW semantics; all pair arguments are validated before MMIO; Drop disconnects routes before releasing pins. Upstream additionally offers split channel owners/modes/DMA, which are not falsely claimed here. | RM26.10, TEN=0 one-PCLK transfer and dual holding register; DAC domain probe; token/output borrow rejections. | Independent channel ownership, DMA/ringbuffer streaming, external triggers/waves and selectable widths remain omitted. Whole-DAC immutable borrowing is intentionally conservative. 10us startup is not a characterized maximum guarantee. |
| `analog/opa/l012.rs` | Existing follower/PGA/external/DAC constructors retain peripheral, input, output, BGR and optional DAC lifetimes. Checked route/gain/bias programming; dedicated CAL command writes avoid trigger RMW. Calibration must see busy before accepting idle and times out conservatively if an entire busy pulse is missed. | RM29 OPA routes/registers/calibration; fresh encoding/domain/busy-transition probe; positive API plus six OPA borrow rejections. | Unlike upstream `OpAmpOutput`, this owner does not expose its output as an ADC channel; dynamic DAC threshold changes are prevented by whole-source borrow. These are scoped capability omissions, not falsely reported full parity. No calibration accuracy claim. |
| `analog/vc/mod.rs` | Selects actual F030 or L012 electrical backend while sharing one-shot event/ownership semantics. No new support module or family-name dispatch. | Both current builds; module inspection. | F030 does not require an unrelated L012 BGR capability. |
| `analog/vc/l012.rs` | Real Comp<Instance,Mode> owner; external negative input restricted to CH0/1; divider/DAC inputs retain correct source borrows. Starts disabled, requires explicit settling-aware enable, and does not unpend/disable shared VC13/VC24 NVIC. DROP disables its source and disconnects pins. | RM27.3/.7; fresh comparator state probe; reference/DAC/pin/binding compile failures. | Explicit startup requirement is intentional CW behavior, safer than copying upstream automatic bare-enable waits. Window/blanking and comparator output-pin APIs remain omitted. |
| `analog/vc/f030.rs` | Eight external inputs and real response/hysteresis/filter encodings; starts disabled; READY polling failure shuts this comparator down. Existing brake guard borrows both source and PWM. Forgotten guard recovery disables MOE/AOE/VCE before removing the source route, preserving sibling state. | RM23.3–.4; fresh comparator state/shutdown/brake-route checks and borrow failures. | Internal ADC/BGR references, shared divider, window/blanking and comparator output pins remain unmodeled. F030 ATIM break route is implemented, not generalized to unproved hardware. |
| `analog/vc/async.rs` | Lazy first-poll arm, clear-old-then-arm, level recheck, enabled-source filter, mask-before-RW0 acknowledge, latch and wake outside cleanup; cancellation clears only owned source. Exclusive &mut owner borrow prevents simultaneous waits. More explicit cancellation/shared-vector behavior than pinned STM32 EXTI-based code. | Both fresh comparator executables exercise unpolled cancellation, stale state, arm-time edge, high/low, ISR wake, disabled-source sibling, reentrant registration; ARM blocking/async/binding checks. | Coalesced event notification only, no edge counter. L012/F030 use direct VC IRQ sources, not STM32 EXTI. |
| `cordic/mod.rs` | Only audited L012 capability selects the driver. No change. | Source and current L012 build. | F030 lacks this accelerator; no fabricated substitute. |
| `cordic/l012.rs` | Bounded exclusive Q31 owner plus IRQ-mode owner, dedicated-reset cancellation and owner-drop recovery. Range checks, signed conversion, even 2–32 iteration encoding, COMP=0, X/Y/Z final-operand triggers and result masks match CW manual. EOC/busy/IE gating and stale-result draining checked. | RM12.3–12.6, visually checked formula table; new numeric-domain/one-LSB endpoint, MMIO ordering/status/budget probes; fresh positive+five ownership/binding and two private-value construction rejections. | CW X/Y/Z and half-magnitude/scaled outputs differ materially from STM32 WDATA/RDATA/access-count interface. Q1.15, DMA, batching omitted. No silicon numerical accuracy, convergence or endpoint behavior claim. |
| `eau/mod.rs` | Only audited L012 capability exports EAU. No change. | Source/current L012 build. | No matching STM32 EAU implementation is claimed. |
| `eau/l012.rs` | Exclusive bounded accelerator API, validated zero-divisor and MIN/-1 overflow, signed truncation/remainder domain, sqrt mode, and correct mode-specific status flags. Writes DIVIDEND then DIVISOR for divide, only DIVIDEND for sqrt. Busy refusal and timeout do not silently cancel an operation. | RM11.3–11.7; fresh domain/status/write-order/budget probes and retained-token compile rejection. | EAU has no supported IRQ/DMA completion path; synchronous bounded polling is hardware-driven, not missing Embassy async work. Host mock probes do not calculate or prove silicon arithmetic. |

## Verification boundary

Current-source lifecycle, real-PAC register, comparator event/shutdown and
math/analog domain/order/budget probes passed, along with fresh ownership and
Binding compile checks. See [validation](validation-v0.14.0.md).

Mock register probes do not calculate real accelerator results or establish
analog accuracy, convergence, calibration quality, electrical behavior, ISR
latency or motor safety. Internal channels, split owners and other omitted
interfaces listed above remain explicit scope limits.
