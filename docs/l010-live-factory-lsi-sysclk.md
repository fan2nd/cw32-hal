# Exact CW32L010 live factory-LSI system clock

This initialization-only contract covers CW32L010F8P6, CW32L010F8U6 and
CW32L010Y8M6, each with 64 KiB Flash and 4 KiB SRAM. Generic CW32L010 and every
L011/L012 remain without LSI SYSCLK capability. Existing default HSI and all
HSI/HSE/LSE target contracts remain unchanged. This mode reuses an already
live source; ordinary POR is deliberately unsupported.

Main verification passed all fifteen commands with exit zero, producing eight
actual libraries and seven linked ELFs with all 1,305 source files unchanged.
Six finite source/data/layout gates passed in separate receipts, and independent
runtime and metadata reviews accept the frozen main implementation. The main
logs retain three existing unused LSE-constant warnings in L010/L011 rows:
`RCC_LSE_SYSCLK_LSE_EDGES`, `RCC_LSE_SYSCLK_LSI_CYCLES` and
`RCC_LSE_SYSCLK_MARGIN_LSE_EDGES`. At main acceptance clean had not run;
final-package completion requires a separate final-source two-library/two-ELF
clean receipt. The [qualification record](l010-live-factory-lsi-qualification.json)
binds source facts; independent main receipts record acceptance separately.
Source review, checking and linked ELFs do not demonstrate hardware execution.

## Original entry and conditions

`Config.sys=Sysclk::LSI` requires `Config.lse=None`. A new LSE declaration is
refused during pure validation before ownership or hardware changes; inherited
LSE stays protected. Configured HSE retains existing source/pad ownership and
electrical limits. Original entry must have software LSIEN=1, STABLE=1 and legal
factory-matching typed TRIM. Later readiness cannot upgrade an ineligible
original observation. Automatic-only requests with LSIEN=0, stopped startup,
ordinary POR and trim preparation are refused.

Own DS CN1.3 qualifies nominal 32,800 Hz at 31,816–33,784 Hz over 1.62–5.5 V and
−40–85°C. These are source-qualified rate bounds, not measured frequency or
strict cycle-duration bounds. Read the existing factory halfword at 0x001007C2
as 16 bits; only raw 0xffff is erased, and typed ten-bit zero is valid. Preserve
live TRIM and WAIT exactly. No SDK-preferred WAIT value is imposed. No LSI
parameter register, IWDT gate, reset, command, feed, counter or status is written
to establish admission.

The incoming clock tree, Flash wait and retained sources must already be
electrically legal. Supply, temperature, reset history and physical source
continuity are not measured. Own bus ceilings remain 24 MHz below 1.8 V and
48 MHz at/above 1.8 V. Configured factory HSI at final bus divisors and the full
4.08 MHz native fallback are independently checked; the slow final LSI rate
cannot legalize an unsafe intermediate HSI edge.

## Preserved consumers and functional handover

Running IWDT uses LSI and can progress toward reset throughout the transition
and after refusal. RUN/update flags do not establish cold request history.
RTC SOURCE2 and its prescalers/AWT/calendar/output tree, UART1/2 SOURCE3,
LPTIM source3, MCO4, GPIO filter5, enabled VC/LVD filters, fault detectors,
LSIRDY and SYSCTRL IRQ4 observers may retain the continuously live source.
Independent RTC SOURCE0/1/3, HSI/HSE and analog-owner vetoes still apply.
No consumer stop, reconfiguration, IRQ/flag clearing or watchdog action obtains
admission. Preserving source routes does not freeze active work or faults.

There is no dedicated GPIO AF named LSI_OUT on these packages. This does not
remove MCO4 exports through PA4/AF4, PA8/AF3 or PB3/AF4 where bonded, or native
GTIM/ATIM internal LSI_OUT routes. Downstream BTIM/ADC/GPIO cascades and external
recipients retain their functional consequences. No dormant work gate is opened
merely to prove idleness. Existing required GPIO inspections run an entire bank;
unrelated sampling, filtering and armed events may advance before an error.
Restoring a gate cannot undo progress. Functional handover permits these effects
and does not imply every retained clock-dependent client stays correct across
system/bus changes.

The existing platform entry boundary remains: before Rust uses application
memory, all bus masters must already be quiescent under the runtime handover.
Additional observer/whole-bank conditions concern functional support and add
no hidden memory-safety duty to safe `init` callers.

## Guarded transition and failure

An immutable original snapshot tracks live LSI parameters/request/readiness and
retained source/fault/policy images. Existing RTC, ADC and VC inspection gates
restore their individual incoming state before semantic results are interpreted.
Detailed gate-enable/restore failures retain priority and complete diagnostics;
failed restoration can leave a gate active. Target-only HSE matching borrows
one GPIOA interval and retains the same error distinction. Existing HSI/HSE/LSE
targets use their original helper behavior.

Checks surround every allowed source/policy/divider/Flash write and its bounded
acknowledgment. Only the specific owned field can move from its prior to requested
value during that acknowledgment. Retained source parameters and policy cannot
drift. Factory HSI is prepared under existing ownership rules: RTC SOURCE3,
raw-HSI MCO and HSIRDY protect their root; only stopped HSI is retrimmed, while
a factory-ready DIV-only change preserves raw HSIOSC. No LSI parameter write or
temporary request restoration occurs for this live target.

Final AHB/APB divisors and conservative Flash WAIT are established under verified
factory HSI. The authoritative Flash register must initially agree with the
original SYSCTRL mirror; both must acknowledge the final WAIT. The last source,
divider or Flash write selects LSI. A final HSE pad inspection can borrow GPIOA,
then a complete read-only source/fault/mux/divider/WAIT check precedes publication.
LSI request/readiness and unchanged parameters remain required afterward.
Snapshot consistency does not prove physical continuity.

Errors publish no new frozen clocks and do not roll hardware back. An attempt
may leave conservative guards, altered HSI preparation, an enabled gate or the
last selected source. Reset before retry; use power reset where retained
controls require it. Source loss may stop the CPU before an error can return.
No runtime switching, watchdog survival, low-power restoration, source-loss
recovery or electrical performance is qualified.

## RTC and strict consumers

RTC identity comes from its own validated semantic source. Native L010
`HsiOscClock` and RTC keep raw 48 MHz HSIOSC, 47.04–48.96 MHz, −40–85°C and
1.62–5.5 V with cycle-qualified timing under every SYSCLK and HSI divider.
L011/L012 retain raw 96 MHz, 94.08–97.92 MHz, −40–85°C and 1.7–5.5 V.
No native `LsiClock` or LSI calendar variant is introduced. Existing classic
fixed-LSI numbers and rate-only/strict qualification stay unchanged; held board
LSE keeps its own declared bounds.

Native sequence ADC returns `Error::UnqualifiedCycleTiming` after its existing
limits/uninitialized checks and before acquisition/rate selection or its first
gate, ADC/BGR, trigger/watchdog or IRQ mutation. L010's zero minimum ADC rate
cannot replace this guard. Blocking/async constructors, scans and BTIM1-triggered
construction share it. Earlier caller-created timers or analog pins are not
restored; constructors return errors on rejection. Public manually constructed
timing values retain the strict-helper preconditions. Rejected triggered construction
also drops its supplied BTIM1 owner and runs its existing stop/request-disable/
gate-release cleanup, as detailed in [ADC sequence timing](adc-scan-sequences.md).

Buffered complementary PWM returns `ConfigError::UnqualifiedCycleTiming` before
strict timing selection or timer/output-connection changes, even with zero dead
time. Earlier pin wrappers may already have disconnected GPIO pads and are
consumed on error. Ordinary rate-oriented timers and simple PWM may propagate
rate-only bounds without granting strict duration qualification. The unchanged
fixed 1 MHz time driver rejects every 32,800/D nominal source before singleton
acquisition and RCC MMIO; its debug library row checks compilation only.
Stage73's LPTIM exact-fraction work and `cycles_for` availability remain unchanged.

## Example and finite acceptance

The existing `examples/shared/lsi-clock` adds only the three exact L010 features,
retaining its F030 default, dependencies and older calendar body. It declares
the L010 supply/ambient range, requests LSI with no LSE declaration and calls
ordinary `init`. It never prepares LSI or commands IWDT. Ordinary POR is
expected to stop in its error handler. Success inspects frozen system, bus and
HSI bounds and acquires read-only `HsiOscClock`; RTC is not changed.

Each library row uses `cargo build --locked --manifest-path firmware/Cargo.toml
--package embassy-cw32 --lib --target thumbv6m-none-eabi --no-default-features`.
Release adds `--release`; debug omits it.

| Main library | Profile | Exact features |
| --- | --- | --- |
| 1 | release | cw32l010f8p6,rt,defmt |
| 2 | release | cw32l010f8u6,rt,defmt |
| 3 | release | cw32l010y8m6,rt,defmt |
| 4 | debug | cw32l010f8p6,rt,defmt,time-driver-gtim1 |
| 5 | release | cw32l010,rt,defmt |
| 6 | release | cw32l011k8t6,rt,defmt |
| 7 | release | cw32l012c8t6,rt,defmt |
| 8 | release | cw32f030c8t7,rt,defmt |

Each ELF uses `cargo build --release --locked --target thumbv6m-none-eabi
--no-default-features` plus the manifest, binary and features below.

| Main ELF | Manifest | Binary | Exact features |
| --- | --- | --- | --- |
| 1 | examples/shared/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l010f8p6,defmt |
| 2 | examples/shared/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l010f8u6,defmt |
| 3 | examples/shared/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l010y8m6,defmt |
| 4 | examples/shared/lse-sysclk/Cargo.toml | crystal | cw32l010f8p6 |
| 5 | examples/shared/rtc-calendar/Cargo.toml | preserve_calendar | cw32l010f8p6 |
| 6 | examples/shared/hse-clock/Cargo.toml | crystal | cw32l011k8t6 |
| 7 | examples/shared/rtc-calendar/Cargo.toml | preserve_calendar | cw32f030c8t7 |

The clean ceiling is exactly release library rows1/8 and ELF rows1/7 from the
final source-only package in a separate clean source/target. Prior generated
outputs or compiled targets cannot supply that result. Record statuses,
warnings, fresh output/source hashes and actual ELF memory limits. Main
acceptance is complete within its finite scope. Clean had not run at main
acceptance and requires its own final-package receipt.

The six finite source/data/layout gates passed separately: own L010 source
verification after the reviewed checker correction, own L011 and L012 source
verification, current data validation, locked-original provenance and module
layout. Their split receipts are not an aggregate recipe execution. Full-54
source/metadata review establishes exactly three new L010 LSI records, generic
exclusion, native RTC and old fixed-LSI invariants, unchanged historical LSE
proofs and inherited-legal 36,080 Hz monitor, native source/field/package/member
bindings and actual generated-output differences. All 43 locked hardware
originals passed provenance verification. Deterministic final source-only
packaging and its clean replay remain separate completion obligations. The
accumulated future `check-lsi-clock.sh` totals are separate; it must not replace
these finite rows. No HAL tests, probes, register models, hosted CI or hardware
execution were performed or are implied by this acceptance.

## Source authority

External evidence consists of own L010 RM CN1.2, DS CN1.3 and SDK1.0.9, plus
selected native SVD/PDSC/device-header/SYSCTRL/IWDT members already pinned in
[`sources/evidence-sources.json`](../sources/evidence-sources.json). The
[qualification record](l010-live-factory-lsi-qualification.json) carries exact
hashes and physical/printed page pairs, including the complete IWDT
source/admission audit. Raw originals, extracted text and rendered tables
remain external and are not redistributed. SDK setup corroborates documented
behavior; it is not evidence that it ran before this initializer.
