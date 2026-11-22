# Init-only factory LSI SYSCLK on three L031 packages

This document records the public contract and finite verification scope.
Main-source decisions and recorded outcomes are in the separate
[runtime/source review](l031-factory-lsi-runtime-review.json) and
[metadata review](l031-factory-lsi-metadata-review.json). The plan below does
not replace those receipts. Final clean replay requires a separate receipt
bound to the final packaged source without changing the main review receipts.
None of these source/build records is hardware validation. Older
F002/F003/classic and LSE records retain
their original scope and do not establish this new L031 implementation.

## Exact scope and own sources

Only CW32L031C8T6 (LQFP48), CW32L031C8U6 (QFN48) and CW32L031F8U6 (QFN20)
gain `rcc::Sysclk::LSI`. All three have 64 KiB Flash at 0x00000000 and 8 KiB
SRAM at 0x20000000. Generic L031, F8P6/K8U6/K8V6 and every R031/W031 remain
excluded. An excluded part is outside this software qualification; that is
not a claim that its silicon lacks LSI. HSI remains the default, with /6
retained unless configured otherwise. No configuration field, schema, PAC
layout, peripheral mode, runtime switch or low-power recovery API is added.

Set `config.rcc.sys = rcc::Sysclk::LSI` before one-time `init(config)`.
The existing operating-condition declaration and AHB/APB dividers apply.
Optional HSE/LSE retain their independently qualified configuration and
ownership rules; F8U6 still lacks a qualified HSE pad route. L031 has no PLL.

Authority is the already selected own L031 RM CN V1.6, DS CN V1.9 and SDK V1.4
in [the source lock](../sources/evidence-sources.json), with these identities:

- RM: `4288cfd97b56385059c5a283f69972047af4773ef8bbc4d8b51d8155fb17a760`
- DS: `90525f4085d00e9d586a991c24f2e4398d92a41e6cc1402c413e963423a35855`
- SDK ZIP: `ef955869214dc80400c0b572f71f2ca54d8a76d89b5ed2756003477a11425e0d`

The policy is [lsi-sysclk-qualified.yaml](../cw32-data/lsi-sysclk-qualified.yaml).
The source index has [own L031 page references](../sources/SOURCES.md#l031-精确三封装-factory-lsi-sysclk).
No new vendor download or sibling numerical substitution is required.
Page references below use one-based PDF/printed pages.

## Rate bounds and public compatibility

Own DS 47/46 tables 7-18/19 and 38/37 table 7-4 qualify factory LSI at
32,800 Hz nominal, 31,816–33,784 Hz (±3%), VDD 1.65–5.5 V and ambient
−40–85°C. The 25°C-only ±1% row and conditional low-dissipation 105°C ambient
extension do not widen this accuracy envelope. These are rate bounds;
startup, duty-cycle and factory accuracy statements do not independently
establish an absolute every-cycle or jitter bound. Neither actual supply,
temperature nor frequency is measured by the HAL.

Exact source numerators and integer AHB/APB divisors remain intact, with
whole-Hz endpoints rounded outward. The exact three parts' `LsiClock::bounds`,
`CalendarClock::Lsi`, RTC source bounds and divided calendar-tick bounds are
also rate-only under every SYSCLK, including HSI, HSE and LSE. This is an
intentional compatibility change: `has_cycle_timing_bounds()` returns false,
and strict cycle/duration helpers retain their existing refusal or assertion
behavior. Rate arithmetic and nominal calendar use remain available. The
calendar ratio is still exactly 32800/32768 nominal, not a certified 1 Hz tick.
No automatic RTC migration, compensation or elapsed-time continuity is added.
Excluded L031 packages, R031/W031 and board-qualified LSE retain their prior
qualification.

The configured retained factory HSI tree is checked independently at the final
bus dividers. Its own oscillator envelope is 48 MHz ±2% over −40–85°C; AHB/APB
limits are 24 MHz below 1.8 V and 48 MHz at or above 1.8 V. HSI /1 with AHB /1
can exceed 48 MHz at positive tolerance and is refused even though LSI is
slow. Default HSI /6 fits the lower-voltage ceiling. Final Flash wait covers
the greater of the selected-LSI upper HCLK and configured retained-HSI upper
HCLK, using the existing wait-step formula.

Classic ADC rejects the selected rate-only LSI tree before its initialization
writes. The fixed 1 MHz Embassy time driver rejects selected LSI during pure
preflight, before singleton acquisition or RCC writes. A library built with
that feature can still initialize its supported non-LSI tree; compilation does
not execute the LSI refusal. UART keeps existing PCLK/baud feasibility and
bounds, with no new local LSI mode. AWT retains independent HSIOSC behavior.
No complementary-PWM capability is added.

## Entry, ownership and observable working gates

The established platform handover applies: hardware reset, or a low-level
handover that made bus masters quiescent before Rust uses application memory.
The currently executing source must remain available in an electrically legal
clock/voltage state. Normal interrupts are masked during initialization;
application, DMA, NMI and other owners must not change clock, pad or consumer
configuration during the transaction. A critical section does not stop hardware
state machines. Asynchronous clock loss, CPU stall or reset mid-transaction
is outside the bounded software guarantee.

Safe `init` can briefly run each entire GPIOA/B/C/F working
clock during inspection. Sampling, filters and armed events may advance and
be externally observable, including before an error. GPIO configuration and
locks are preserved, and software does not clear flags, but hardware flags
may change naturally. Restoring gates cannot undo work performed in that
interval. This is a visible functional handover limitation; it adds no hidden
Rust memory-safety precondition or rollback promise.

The new strict source-root inspections save each original gate, reject an
asserted reset and check enabled/not-reset before and after selector reads
inside the central `RccInfo::inspect_for_init` window. Restoration and its
readback complete before buffered selector/reset refusal is released. Failure
to restore wins over a simultaneous semantic refusal. After successful gate
restoration, relevant HSE/LSE FAIL/FAULT checks precede buffered refusal.
GPIOB FILTER and AFR11 are read in the same window.

Existing HSE/LSE pad helpers keep their established internal inspection and
error contracts. Before/after target proofs do not imply stronger checks
inside every pad-read closure. Exclusive configuration ownership and continued
source availability bound those helper intervals.

## Immutable entry classification and complete cold roots

Selected-LSI capture precedes existing LSE preflight and every source/oscillator
write. It freezes normalized CR0/CR1/IER, HSI/HSE/LSE/LSI parameters, both LSI
stable observations, LSIRDY and RCC NVIC pending. Only read-only STABLE and
write-only keys are removed from relevant readable identities; reserved and
parameter bits remain. One aligned factory halfword is read at 0x00100A02
(RM 54/53, 56/55 and 71/70). Raw 0xffff is conservatively rejected before
ten-bit masking; zero remains valid. All-ones refusal does not diagnose silicon.

Incoming SYSCLK must use native legal HSI0/HSE1/LSI3/LSE4, with a legal HSI
divider and direct/mirrored readiness for the executing source. Selected
HSE/LSE must also have its software enable; an ambiguous disabled selected
external source is not repaired.

- Cold means LSIEN=0, SYSCLK is not LSI, both LSI stable observations are zero,
  HSECCS=LSECCS=0, IER.LSIRDY=0, ISR.LSIRDY=0 and RCC pending=0. Detector bits
  refuse cold entry even when their external source is disabled. CLKCCS and
  LSELOCK may retain either value.
- Live/requested means LSIEN=1 or selected LSI, factory TRIM matches, and
  stable views agree. An enabled, unselected source may finish startup within
  the poll budget. Selected LSI must already be ready. An enabled detector
  requires LSIEN=1 and both ready views immediately.
- Live mismatching trim, contradictory status or ambiguous ownership rejects.
  A ready selected LSI with LSIEN=0 is live, not cold; its permanent software
  request is asserted before escaping to HSI. No live source is stopped or
  retrimmed to gain admission. Later readiness never changes original cold
  classification into live reuse, and readiness is sticky once observed.

Cold preparation uses nine gated roots and eleven selectors, in this order:

| Root | Register | Accepted cold selector values |
| --- | --- | --- |
| RTC | CR1.SOURCE | 0, 4, 5, 6, 7 |
| AWT | CR.SRC | 0, 2, 3, 4 |
| UART1, UART2, UART3 | CR2.SOURCE | 0, 1, 2 |
| GPIOA, GPIOB, GPIOC, GPIOF | FILTER.FLTCLK | 0, 1, 2, 3, 4, 6 |
| MCO, ungated | SOURCE | 0, 1, 2, 3, 5, 6, 8, 9 |
| PB11, same GPIOB window | AFRH.AFR11 | 0, 3, 5, 6, 7 |

Own selectors are documented at RM 89/88, 141–142/140–141, 150/149,
165/164, 168–172/167–171, 179–180/178–179, 321–322/320–321 and 353/352,
plus DS 30/29. FILTER7 is documented AWT overflow and conservatively rejected,
even when AWT selects a safe source. The SDK's LPTIMPWM label does not override
the own RM. MCO7 is separately undocumented. PB11 is bonded on C8T6/C8U6 but
unbonded on F8U6; its conservative register check does not claim an output pad.
Blank AF2/4 and values 8–15 are not accepted as safe merely because they differ
from LSI_OUT AF1.

Own gate/reset pairs are RTC APBEN1/APBRST1 bit3; UART2/3 bits7/8; AWT
APBEN2/APBRST2 bit13; UART1 bit9; and GPIOA/B/C/F AHBEN/AHBRST bits4/5/6/9
(RM 77–82/76–81). They are enable-high, reset-low and unkeyed. No peripheral
reset is pulsed. RTC START/output/wakeup, UART TX/RX enables, per-pin filter
enables, closed gates and unbonded pads do not override source-root exclusion.
IWDT uses independent RC10K; LVD/VC have HSIOSC/RC150K/PCLK routes. L031 has
no AUTOTRIM, LCD or LPTIM root to import from other families.

Factory-matching live reuse does not apply cold selector exclusions: active
RTC, UART and output clients can keep consuming the unchanged source. Their
events are not cleared and no idle state is claimed.

## Bounded sequence and exact expected state

Pure source, board-envelope, retained-HSI and time-driver checks precede
ownership/hardware admission. Only the new selected-LSI target skips the old
`prepare_lse_monitor`; non-LSI targets retain its historical ordering and
matching-cold shortcut. Existing RTC/AWT HSE ownership, enabled AWT HSIOSC and
enabled LVD HSIOSC-filter blockers remain. The full retained AWT control word
is captured and verified through a restored inspection at final handover.

1. Establish conservative Flash wait and monotonic bus guards of at least
   AHB /4 and APB /8, preserving a stronger incoming divider and explicitly
   preserving the incoming source. Guard/gate work is the only mutation before
   target preparation; no oscillator enable, trim or source-select write occurs.
2. Cold entry gets two complete global-before/consumer/global-after passes
   with identical original-gate/selector arrays, even when trim matches.
   Immediately before a needed trim write, recheck full cold identity. Change
   only native TRIM[9:0], preserving WAIT[11:10] and reserved bits, then poll the
   expected word while both stable observations remain zero. Matching trim
   skips only that write. A third complete consumer comparison and global
   proof occur at the first-request use edge. Live reuse instead retains
   matching parameters and boundedly awaits permitted pending startup.
3. Jointly request HSIEN and permanent LSIEN, preserving all other CR1 bits,
   before the HSI escape. Check full readback, permanent request, factory
   parameters, frozen IER and both-view readiness. LSIRDY/RCC pending may rise
   naturally after this request; neither is cleared. Recheck cold identity
   after request, wait for unchanged HSI and explicitly select guarded HSI.
4. Retain the existing HSI calibration bridge when needed. Verify the target
   and HSI owners before temporary LSI selection and before HSI stop. Calibrate
   HSI, restart/read back, and explicitly return to guarded HSI. Every source
   write keeps permanent LSIEN. The old temporary restore-off tail is excluded
   for this target. Apply the requested HSI divider while still guarded.
5. Apply only permitted HSE setup/reuse, with target proofs before and after.
   A retained owner permits exact already-enabled, ready HSE with correct pads
   and parameters, with no stop or retuning. Fresh setup may advance only the
   actual qualified HSE bank gates after success. L031 HSE uses GPIOF; F8U6's
   absent route grants no gate advance. Inherited undeclared HSE retains its
   enable, parameters and actual pad reservation.
6. Start optional auxiliary LSE through the existing helper while HSI and
   guarded buses remain selected. LSI is already permanently factory-ready.
   Complete before/after target proofs bracket that bounded helper; no extra
   callback adjacent to its internal LSEEN write is claimed. Fresh LSE uses its
   existing typed-default whole-register parameter write and may add LSEEN and
   LSECCS. This is not a new preserve-reserved contract. Reused/unrequested LSE
   retains original parameters. LSE bypass inspection restores its GPIOC gate,
   so it does not authorize a permanent gate advance. Enabled inherited LSE
   needs readiness, no fault and the actual mode's required pads; an undeclared
   retained source gains no board-frequency certificate.
7. Recheck expected source/parameter/IER state, faults, both LSI/HSI ready
   views, cold consumers/gates, the AWT owner and HSE/LSE pads. Recheck source
   identity after bounded owner/pad work. While explicitly retaining HSI,
   install and acknowledge the independently legal final AHB/APB dividers.
   Keep conservative Flash latency through this transition.
8. Set/read back final Flash wait from the greater HSI/LSI upper HCLK. Verify
   the complete configured HSI tree and LSI identity/readiness again. The last
   owned CR0 source write changes only SYSCLK to LSI, retaining the installed
   dividers. Acknowledge source/dividers and barrier, then repeat final source,
   pad, owner, consumer and Flash verification. Read final selector/dividers
   last. No oscillator/source/divider write follows this final selection; only
   then publish frozen clocks.

Expected state is the original snapshot advanced by intended, verified owned
fields. It is never replaced by a fresh observed word to absorb unrelated
changes. Existing configurable CLKCCS/HSECCS/LSECCS/LSELOCK are preserved except
the established fresh auxiliary-LSE addition. There is no invented LSILOCK,
classic mandatory-CCS policy, forced HSI /6 fallback or LSI-loss recovery.

## Errors and limits

Capability-gated errors are `InvalidLsiCalibration`, `LsiCalibrationInUse`,
`LsiClockInUse`, `LsiGateEnableTimeout`, `LsiGateRestoreTimeout`,
`LsiConfigurationTimeout` and `InvalidEntryClock`. They distinguish invalid
factory data, live calibration conflict, ambiguous ownership, strict gate
enable/restore failure, target configuration timeout and invalid incoming clock.
Existing source, HSI/HSE/LSE
owner, Flash, mux and time-driver errors retain their established meanings.
`LsiTimeout` also covers bounded permanent-target readiness failure.

Precedence follows the actual stages: pure validation first; central gate
enable/restore resolution before buffered semantic results; failed restoration
dominates semantic refusal; relevant external faults after successful restoration
are checked before accepting buffered results. Later readback failures keep
their specific error. Existing helpers retain their old error mappings and
ordering outside the new target.

Failure publishes no RCC clocks or peripheral tokens. It may leave a trim
attempt, source enable, temporary execution source, conservative Flash/bus
guards, pad configuration or a gate changed. HSI calibration can fail while
executing on LSI with HSI stopped or incompletely restarted. Reset before retry;
no successful rollback or recovery is promised. STABLE/mirrored status describes
startup, not assured continued source availability. Poll budgets count register
observations, not microseconds; loss of the execution clock may prevent return.

## Finite verification plan, not execution receipts

The [ordinary LSI firmware](../examples/lsi-clock/README.md) reuses its RTC
branch, selects the own −40–85°C envelope and asserts real 64 KiB/8 KiB linker
bounds. No time driver is enabled in that example. Cargo.lock, dependency
versions and hosted CI workflows are unchanged.

The focused main library matrix is exactly ten actual builds:

1. C8T6, C8U6 and F8U6 release with defmt and `time-driver-gtim1` (three).
2. C8T6 debug with those same features (one).
3. Generic `cw32l031` and excluded `cw32l031f8p6` release with defmt (two).
4. `cw32r031c8u6` and `cw32w031r8u6` release with defmt and
   `time-driver-gtim1` (two).
5. Prior-LSI `cw32f003e4p7` release with defmt and `time-driver-gtim`, and
   `cw32f030c8t7` release with defmt and `time-driver-gtim1` (two).

Use `cargo build --locked --manifest-path firmware/Cargo.toml -p embassy-cw32
--target thumbv6m-none-eabi --no-default-features`, the named chip/features,
and `--release` except the one debug configuration. A cargo check is not an
actual library build.

The focused main firmware matrix is exactly ten release/locked/thumbv6m ELFs,
all with `--no-default-features`:

1. `examples/lsi-clock`: each exact L031 part with defmt (three).
2. `examples/lse-sysclk`: C8T6 `crystal`, F8U6 `bypass`, R031C8U6 `crystal`
   and W031R8U6 `crystal` (four).
3. `examples/rtc-calendar`: C8T6 `preserve_calendar` on default HSI (one).
4. `examples/lse-clock`: C8T6 `crystal_calendar` with auxiliary LSE (one).
5. `examples/hse-clock`: C8T6 `crystal` (one).

One clean replay from the final frozen source covers C8T6/F8U6 representative
release libraries (two) and the three new LSI ELFs (three), with the same
options. Do not repeat unrelated main regression ELFs or relabel a cached
library-only receipt as a newly linked firmware image. If a command only
duplicates an existing exact-source receipt, omit it and report the reduced
actual count.

The wider [local reusable recipe](../ci/README.md) has twenty library commands
(fifteen builds plus five historical checks: two generic and three classic) and twenty-five ELFs. Its future
scope is deliberately separate from this focused matrix, and it is not an
acceptance record. No count above asserts execution.

Later authorized verification must also compare all 54 generated chip objects:
only the three exact parts gain the reviewed LSI clock metadata; every excluded
object and every other peripheral/route/memory/source fact remains unchanged.
Generated PAC register/peripheral bytes have no expected change. Inspect real
entry/vector/PT_LOAD regions, Flash/static RAM and stack headroom; compare
corresponding clean/main loadable sections and addresses, not incidental whole
ELF debug bytes. Source packaging must preserve files, modes and exclusions
with a deterministic archive and retained hashes. Generation/data-source checks
and final independent source review need their own explicit outcomes.

This is focused software verification, not the repository-wide production
validation pass. No HAL test, runtime model, probe, harness or hardware
execution is part of this plan. Build success cannot execute admission/failure
branches or prove fault precedence, rate, silicon timing, source loss,
sleep/wake or hardware recovery.
