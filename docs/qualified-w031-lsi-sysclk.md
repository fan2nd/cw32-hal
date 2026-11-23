# Init-only factory LSI SYSCLK on CW32W031R8U6

Status: independent runtime/source and metadata/projection reviews accept the
frozen main implementation. Main generation, five actual library builds, six
ELF links and six focused source/data commands passed. At main acceptance,
clean replay had not run; final-package completion requires a separate clean
receipt and deterministic package acceptance.
The own-source Stage69 design is independently accepted as a frozen design only.
Existing R031/L031/classic/F002/F003 and LSE acceptance records retain their
original scope and outcomes. These software results do not validate hardware.

## Exact scope and evidence

Only **CW32W031R8U6, QFN64, 64 KiB Flash and 8 KiB SRAM** gains the existing
`rcc::Sysclk::LSI` selection. HSI remains the default. Generic W031 and every
other W package remain unqualified. Shared `cw32l031_v1` identity does not grant
qualification; R031's 2.2 V lower limit and 16 MHz RF source are not W031 facts.
Flash is `[0x00000000, 0x00010000)` and SRAM is `[0x20000000, 0x20002000)`;
link regions must not be enlarged to hide overflow.

Authority remains [evidence-sources.json](../sources/evidence-sources.json),
with no download or re-pinning. PDF/printed page numbers below are one-based.

| Own source | SHA-256 |
| --- | --- |
| [W031 RM CN V1.4](https://www.whxy.com/uploads/files/20240920/CW32W031_UserManual_CN_V1.4.pdf) | `b6973677946a9332b0e5b3e954119768aa40469d44140a73e18648e9419bedc9` |
| [W031 DS CN V1.3](https://www.whxy.com/uploads/files/20251230/CW32W031_DataSheet_CN_V1.3.pdf) | `45ec43e6370956d09f9b9aa4c0f6c83fb0661f576d2d2bf64e0a6d7c4e203d3c` |
| [W031 SDK V1.3](https://www.whxy.com/uploads/files/20240119/CW32W031_StandardPeripheralLib_V1.3.zip) | `1010176816766d025c84babf84ffdf2ee03c5687d2a29133267d0013a1970337` |
| SDK member `cw32w031/IdeSupport/MDK/WHXY.CW32W031_DFP.1.0.2/SVD/CW32W031.svd` | `7b3f7cd1e1e9b311f8bf6da711b9af0a144040883a231dfcc93b9cd2bafe84d0` |
| SDK member `cw32w031/IdeSupport/MDK/WHXY.CW32W031_DFP.1.0.2/WHXY.CW32W031_DFP.pdsc` | `932cf4107a4a543cb92de4a80bf3e286f49dec9f5ea36659b682eff26f7d5b36` |

DS9/8 table3-1, DS33–34/32–33 and DS71/70, with the own PDSC, bind the exact
package and memory. The [LSI policy](../cw32-data/lsi-sysclk-qualified.yaml)
binds the own 53 native fields/access facts, ten bases, nine gate/reset pairs,
eleven selectors and source graph before exact-package projection. PB11 is
bonded at pin16; HSE PF0/PF1 are pins63/64 and LSE PC14/PC15 are pins61/62.

Frozen external evidence identities are retained separately from source:

- `external-evidence/stage69-design/w031-lsi-sysclk-design.md`:
  `4988ff35cfa5bbddc30844917049b55824c8d48688d98d92290a6944bfe229ab`
- `external-evidence/stage69-design/w031-lsi-sysclk-design-evidence.json`:
  `e530dc0ef47cc7846add6e423b51cac3435b35430cb010f8a5296fd202bd66eb`
- `external-evidence/stage69-design/independent-review/w031-lsi-sysclk-independent-review.json`:
  `8cd904114c25440b4a2bfce05a7247cfc8c3c4ba20b4046362b2c105c291fa12`

These accept the design and its source/sequence basis, not the implementation,
compiler outputs or hardware. Vendor originals and page renders remain external.

## Electrical and RTC compatibility boundary

DS53/52 tables7-23/7-24 under DS41/40 table7-4 qualify nominal **32,800 Hz**,
**31,816–33,784 Hz**, **2.0–3.6 V** and **−40–85°C**. Use the conservative
RF-LDO/DCDC supply intersection. DS16/15 requires independent VDDRF to use the
same supply as VDD when RF is used; the stricter DS41/40 table requires VDDA=VDD.
All board supplies and grounds must satisfy the own DS. The existing HSI-only
1.8–3.6 V profile is unchanged. No firmware RF-power probe widens the LSI range.
The 25°C-only ±1% row, adjustment range, duty cycle and startup time do not
establish individual-cycle duration or jitter bounds. API declarations are not
measurements or proof that a particular board satisfies them.

SYSCLK/HCLK/PCLK retain exact integer rate numerators and divisors and are
rate-only. On this exact part, `LsiClock::bounds`, `CalendarClock::Lsi`, RTC
source bounds and divided calendar-tick bounds become rate-only under **every
SYSCLK**, including HSI, HSE and LSE. `has_cycle_timing_bounds()` is false;
strict duration helpers retain their existing refusal/assertion behavior. The
numerical RTC bounds and nominal **32800/32768 Hz** calendar rate are unchanged.
This compatibility change does not grant a certified one-second period. Generic
W031 and other excluded selections keep their previous alias qualification;
board LSE remains independently qualified.

Pure preflight checks configured retained HSI under the final AHB/APB dividers
as well as selected LSI. HSI48 MHz ±2% with HSI/1 and AHB/1 exceeds the own
48 MHz bus ceiling and is refused. Default HSI/6 is 7.84–8.16 MHz. Final Flash
latency covers the greater retained-HSI/final-LSI upper HCLK; RM108/107 and
118/117 WAIT0/1/2 limits of 24/48/72 MHz do not grant 72 MHz operation.

Classic ADC refuses rate-only PCLK before gate acquisition/init writes. Existing
strict PWM duration defenses remain; no new complementary-PWM mode is added.
The fixed 1 MHz time driver returns `UnsupportedTimeDriverClock` during pure
preflight before `Peripherals::take()` and before RCC MMIO for selected LSI.
A library build with that feature only checks compilation. UART/SPI/I2C/timer/
WWDT feasibility checks, AWT HSIOSC and IWDT's independent RC10K remain intact.

## RF and whole-bank functional handover

RM48/47 figure4-1, 49/48, 55/54 and 499/498 figure25-1 establish the external
RFXC1/RFXC2 crystal → dedicated **32 MHz** oscillator → RFCLK → RF subsystem
and internal PLL path. DS29/28 binds crystal pins38/39. It is separate from the
four MCU SYSCLK sources and adds no tenth direct cold-LSI root.

The host path is nevertheless MCU SYSCLK → HCLK → PCLK → APBEN2.SPI bit8 →
internal SPI1 → RF registers/FIFO (RM363/362, 366/365 and 509/508). DS14/13
binds **PB05 MOSI, PB04 MISO, PB03 CS, PB13 SCK and PB06 RF IRQ**, with host
communication below 10 Mbps. Those internal pads are not extra QFN64 external
GPIO tokens. RM363/362 states external SPI pin functionality is unavailable
while using RF; this change adds no SPI/RF arbitration or peripheral entitlement.

The existing whole-GPIOB inspection can resume internal host/IRQ sampling,
filters or events on PB03/04/05/06/13. Its combined FILTER/PB11 window does not
isolate the other bank functions. Finish/quiet clock-sensitive host transfers
and permit the entire bounded bank interval, including an error path. Restoring
a gate cannot undo work. Unchanged RFCLK and restored MCU gates do not guarantee
unchanged SPI waveforms, packet continuity, RF throughput, radio idleness or
clock continuity.

These are visible functional limits of safe `init`, **not new hidden
Rust memory-safety preconditions**. The incoming clock and Flash latency must
be legal, configuration ownership exclusive and retained external sources
available. Normal interrupt masking does not stop NMI, DMA or autonomous work.
Concurrent raw clock/pad/consumer changes, reset, CPU stall and asynchronous
source loss are outside this bounded contract. Existing pad reservations remain.
No RF register, page, mode, power, reset, SPI command, event clear or interrupt-
state read/write is added. RF driver/protocol/runtime/low-power work is deferred.

## Frozen native admission and transition

The shared native executable `LsiSysclkState` and P0–P9 sequence remain unchanged.
The four functional library files are the policy, its electrical hash, its
source-bound generator validation and the two exact admission predicates in
`embassy-cw32/build.rs`. No schema, adapter, PAC layout/access or runtime algorithm
change is included. The existing LSI example adds only this exact feature and
its own declarations and memory guard.

RM55/54, 57/56, 65/64 and 72/71 plus the own SDK establish the single aligned
factory u16 read at 0x00100A02. Raw 0xffff is conservatively refused before
ten-bit masking; zero is permitted. Cold mutation modifies TRIM only while
preserving WAIT and reserved bits. Ready observers are IER.LSIRDY, ISR.LSIRDY
and NVIC SYSCTRL pending; readiness requires agreement of LSI.STABLE and
ISR.LSISTABLE. SYSCTRL IRQ4 is distinct from CLKFAULT IRQ31 (RM75/74,92–93/91–92
and own header/startup); the selected SVD supplies only the former as a distinct
interrupt. No invented RCC alias, IRQ substitution or pending clear is used.

Only requested LSI creates the target state. Capture precedes optional LSE
preparation and source writes. Cold requires unrequested/unselected LSI, both
stable views clear, no HSECCS/LSECCS and no ready observer. Factory-matching cold
trim does not bypass admission. Requested or selected matching LSI is live,
including selected LSI with LSIEN=0; a live mismatch is never retrimmed. Matching
live clients retain TRIM/WAIT without cold allowlists. This does not establish
client idleness or elapsed-time continuity. Readiness becomes sticky once seen.

Nine gate windows cover RTC, AWT, UART1/2/3 and GPIOA/B/C/F. Eleven retained
selectors include those source/filter values, MCO and PB11 AF; PB11 shares
GPIOB's FILTER window. RTC START, UART enables, closed gates and output inactivity
do not replace root proof. GPIO FILTER7 is documented AWT overflow and is
conservatively refused; MCO7 and other undocumented values remain refused.

Two full cold snapshots and a third request-use-edge proof preserve original
selectors and gates. The central inspector owns restoration. Within strict
windows, attempted enable/restore errors precede restored-gate equality,
relevant external faults, post-window reset and buffered semantics. No reset
pulse, IER write or event clear obtains admission. Whole-bank work is observable.
Older retained-HSE/pad helpers keep their own bounded contracts; this does not
claim strict reset checks inside every old closure.

The first source request jointly sets HSIEN and permanent LSIEN before escaping
incoming selected LSI. Full expected-state readbacks advance only intended
fields, with configurable CCS/LSELOCK retained. The existing HSI trim bridge,
HSE owners and actual pads, auxiliary LSE checks and source proofs remain.
W031's HSE GPIOF route has gate index8; LSE GPIOC inspection restores its gate
and advances none. Fresh auxiliary LSE retains its existing default-based
whole-word write rather than an invented preserve-reserved guarantee.

Configured HSI runs on final buses before final Flash latency is installed.
Final LSI selection is the last oscillator/source/divider write; bounded gate
inspections, source/pad/owner checks and final selector/divider readbacks finish
before clocks publication. RCC failure publishes no new clocks or peripheral
tokens, but can leave gates, conservative Flash/buses, attempted trim, partial
HSI bridge or the permanent LSI request. HSI failure can leave execution on LSI
with HSI stopped or incompletely restarted. Reset before retry; no rollback,
source-loss recovery, runtime switching or sleep/wake restoration is promised.

## Main verification and separate clean acceptance

The Stage69 main slice completed **five actual library builds and six linked
ELFs**: all eleven commands exited 0, with five retained rlibs, six retained
ELFs and no failed main Cargo attempt. The 1,294-file build-input snapshot
remained unchanged. Ordinary generation and all six focused source/data commands
also passed, with no failed source/data invocation. Their provenance check
verified the hardware scope of 43 required originals and 551 required source
files; the two omitted discovery HTML pages are not claimed as a complete
catalogue-manifest check.

The independent runtime/source and metadata/projection reviews accept this
frozen main implementation without blocking findings. Their main acceptance
records that clean replay had not run; a later final-source addendum does not rewrite
those immutable dispositions. Review identities are:

- `external-evidence/stage69/runtime-review/w031-factory-lsi-runtime-review.json`:
  `abbeea13f082e8f0fb1a028182dabdb9a7badf7401914724a0f8fbddef63c0d2`
- `external-evidence/stage69/metadata-review/w031-factory-lsi-metadata-review.json`:
  `3ba8c7474d22289c1082656f0b2dc0da80ee72c2982ef2d519851723387accbd`

Retained evidence labels and SHA-256 identities are:

- `external-evidence/stage69/verification/main/summary.json`:
  `c5532984f682004d7bcf5a3041fc038acdd6e1333ad296b3245871c59cc85c73`
- `external-evidence/stage69/verification/main/commands.json`:
  `00baa4d8b7bb52fac87ed67a3e27e4dd42683ebbd84492a4f45f87a1d94391d6`
- `external-evidence/stage69/verification/data/summary.json`:
  `248991c581f0438a61479d8cbeccdf01c39743ee24275ae501451a7b0db3c046`
- `external-evidence/stage69/verification/data/commands.json`:
  `c87ca95aca7bd4cbae725137981c5aca61bad77868815861828d2dfffab6945f`
- `external-evidence/stage69/verification/generation-v1.log` (exit 0):
  `7a66d86a91fb40be3741df1d2d96fb1be8993c757f7da4bb364bb98c9cbdd640`

The command receipts bind logs, actual artifacts and generated HAL outputs.
Cargo also used `--message-format=json-render-diagnostics` for artifact recording.
All library rows use `cargo build --locked --manifest-path
firmware/Cargo.toml -p embassy-cw32 --target thumbv6m-none-eabi
--no-default-features`; add `--release` for release rows and the exact features.

| Row | Profile | Features |
| --- | --- | --- |
| L1 | release | `cw32w031r8u6,defmt,time-driver-gtim1` |
| L2 | debug | `cw32w031r8u6,defmt,time-driver-gtim1` |
| L3 | release | `cw32w031,defmt` |
| L4 | release | `cw32r031c8u6,defmt,time-driver-gtim1` |
| L5 | release | `cw32l031c8t6,defmt,time-driver-gtim1` |

Each ELF uses `cargo build --release --locked --target thumbv6m-none-eabi
--no-default-features`, with the row's `--manifest-path`, `--features` and `--bin`.

| Row | Manifest | Features | Binary |
| --- | --- | --- | --- |
| E1 | `examples/shared/lsi-clock/Cargo.toml` | `cw32w031r8u6,defmt` | `cw32-lsi-clock-example` |
| E2 | `examples/shared/lse-sysclk/Cargo.toml` | `cw32w031r8u6` | `crystal` |
| E3 | `examples/shared/rtc-calendar/Cargo.toml` | `cw32w031r8u6` | `preserve_calendar` |
| E4 | `examples/shared/hse-clock/Cargo.toml` | `cw32w031r8u6` | `crystal` |
| E5 | `examples/shared/lsi-clock/Cargo.toml` | `cw32l031c8t6,defmt` | `cw32-lsi-clock-example` |
| E6 | `examples/shared/lse-sysclk/Cargo.toml` | `cw32r031c8u6` | `crystal` |

At main acceptance, clean replay had not run. Final-package completion requires
a separate clean receipt for regenerated frozen source and only **L1
and E1**. E3 covers the non-LSI RTC link; source tracing establishes the all-
SYSCLK alias propagation. These counts are separate from the accumulated future
local recipe, whose command-derived scope is 24 library commands (19 builds and
five historical checks) and 32 ELFs. Do not run that whole recipe for Stage69.
Selected-LSI firmware adds no fixed 1 MHz time-driver feature.

The ordinary transactional `./d gen-all` retains the explicit project root and
locked evidence. Compare all 54 generated objects: only exact W031 gains
`lsi_sysclk`; all unrelated facts and other objects stay unchanged. PAC register/
peripheral bytes, old-family emitted helpers and frozen shared executable spans
must match. Recheck own sources/access/gates/IRQs/pins and the RF/supply graph.
The approved existing source/data commands are `cw32-data/tools/validate.py`,
`tests/validate_pac_inventory.py`, `tests/verify_rcc_operating_envelope.py`,
`tests/verify_rtc_remaining_evidence.py`, `ci/verify-l031-lse-data.py` and
`cw32-data/tools/source_provenance.py`; the source packager retains its existing
static module-layout validation. No HAL tests, probes, new helper or executable
runtime model is part of this slice.

Retain actual rlibs/ELFs and exact command, profile, feature and hash receipts.
Inspect entry/Thumb reset vector, initial SP, vectors, every PT_LOAD physical/
virtual range, section-to-segment mapping and allocated sections, including
initialized-data Flash LMA and runtime RAM. Main/clean E1 loadable contents,
entry and load addresses must agree; debug bytes need not. Freeze the complete
final source manifest, package with `ci/package-source.py`, and deterministically
repack to compare archive bytes/hash. Preserve executable modes and exclusions.
Generated data/PAC, build outputs and raw evidence remain outside the source ZIP.

Independent main implementation reviews are accepted at their stated scopes.
At main acceptance, clean replay had not run. Final-package completion requires
a separate clean receipt and deterministic package acceptance in a final addendum;
the main results do not replace them. Compilation, linking and static correspondence do not execute
cold/live/error paths, prove time-driver refusal at runtime, measure frequency,
validate silicon timing or establish RF continuity. This finite slice is not a
full repository test/check pass.
