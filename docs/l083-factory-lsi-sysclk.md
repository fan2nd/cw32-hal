# Exact CW32L083 factory-LSI system clock

This contract covers init-only `Sysclk::LSI` on CW32L083RBT6, CW32L083RCT6,
CW32L083RCS6, CW32L083MCT6 and CW32L083VCT6. Generic CW32L083 and native
L011/L012 and generic L010 gain no LSI SYSCLK capability. The later
[exact L010 live-only mode](l010-live-factory-lsi-sysclk.md) has its own contract. Leave `Config.pll=None` for this
system source; inherited hardware PLL state is admitted independently.
Existing HSI/HSE/PLL/LSE targets keep their separate contracts.

Main generation, seven finite source/data commands, ten actual library builds,
ten linked ELFs and independent implementation reviews passed on the frozen
main source. Eleven bounded invalid-data cases were rejected; the metadata
review records their actual rejection layers without equating an earlier
source-digest rejection with execution of a later semantic check. Both retained
L052 generated-helper regressions are byte-identical to their Stage70 artifacts.
The exact L083 rows have no new warnings; the L010 regression retains three
unused constants from unchanged earlier definitions. Initial unused-emission
warnings, a source-navigation correction and post-build evidence-capture
failures are retained separately and are not counted as final passes.

At main acceptance clean had not run. Final-package completion requires a
separate two-library/three-ELF clean receipt bound to the final source ZIP.
See the [runtime review](l083-factory-lsi-runtime-review.json) and
[metadata review](l083-factory-lsi-metadata-review.json). No full aggregate
campaign or hardware execution is claimed; recipe totals and design review
are not substitutes for actual implementation receipts.

## Parts, sources and rate bounds

| Part | Package | Flash / SRAM | Bonded LSI output routes |
|---|---|---|---|
| CW32L083RBT6 | LQFP64, 10×10 mm | 128 KiB / 24 KiB | PC4 AF6, lead24 |
| CW32L083RCT6 | LQFP64, 10×10 mm | 256 KiB / 24 KiB | PC4 AF6, lead24 |
| CW32L083RCS6 | LQFP64, 7×7 mm | 256 KiB / 24 KiB | PC4 AF6, lead24 |
| CW32L083MCT6 | LQFP80 | 256 KiB / 24 KiB | PC4 AF6 lead28; PF2 AF4 lead14 |
| CW32L083VCT6 | LQFP100 | 256 KiB / 24 KiB | PC4 AF6 lead33; PD5 AF6 lead86; PF2 AF4 lead19 |

Flash starts at 0x00000000 and SRAM at 0x20000000. The existing LSI example
asserts the exact memory sizes and RTC presence before writing `memory.x`.
Its default feature and older package branches remain unchanged. Its L083
branch declares 1.65–5.5 V, −40–85°C, HSI /6 and AHB/APB /1, then uses the
existing polling calendar branch without an Embassy time driver. These are
board declarations, not measurements. Qualify the actual board, including
VDDA=VDD and its entire supply/temperature range, before hardware use.

Own authority is `CW32L083_UserManual_CN_V2.0.pdf`,
`CW32L083_DataSheet_CN_V1.9.pdf`, SDK V2.2 and its selected native SVD.
The [source mapping](../sources/SOURCES.md#exact-cw32l083-factory-lsi-sysclk)
records their locked hashes and page locators. Vendor originals and extracted
text remain external evidence, outside the source package. An SDK startup
routine is not proof that it ran before Rust initialization.

LSI is nominal 32,800 Hz with factory bounds 31,816–33,784 Hz over 1.65–5.5 V,
VDDA=VDD and −40–85°C. The conditional +105°C operating row and ±10% adjustment
range do not broaden the ±3% factory envelope. Read one aligned 16-bit factory
halfword at 0x00100A02 for LSI and one at 0x00100A00 for HSI. Reject raw 0xffff
before masking; zero is not automatically erased. LSI TRIM is ten bits and HSI
TRIM eleven. Preserve LSI WAIT[11:10], whose four encodings mean 6/18/66/258
cycles, and every other readable non-TRIM bit. Readiness is startup status,
not a continuous frequency or source-health measurement.

Factory LSI has rate-only provenance. `ClockBounds::lsi()`, selected SYSCLK,
HCLK/PCLK and every exact-L083 RTC LSI alias are rate-only under HSI, HSE, PLL,
LSE and LSI SYSCLK. This includes `LsiClock::bounds`, `CalendarClock::Lsi`,
`Rtc::source_clock_bounds` and divided calendar bounds. Their numerical RTC
facts and nominal fraction 32800/32768 are unchanged; no precision one-second
claim follows. HSI and board-qualified LSE retain their own provenance. Generic
L083 and other unqualified parts keep their prior RTC qualification.

Strict duration helpers preserve their existing assertions/refusals. ADC
refuses rate-only PCLK before its gate/reset or ADC writes. Buffered
complementary PWM uses the strict dead-time helper before timer configuration,
including zero dead time; this retains assertion/panic semantics, and does not
exclude earlier pin-wrapper effects. The fixed 1 MHz time-driver check rejects
the selected 32,800-Hz-derived source before singleton acquisition or RCC MMIO.
A time-driver library build does not execute this refusal or prove a general
ban on every possible rate-only source.

## Supply, entry and functional handover

HCLK/PCLK limits are 24 MHz below 1.8 V and 64 MHz at/above 1.8 V. Full factory
HSIOSC bounds are 47.04–48.96 MHz. Preserve exact integer source division for
safety comparisons; rounded nominal Hertz is insufficient. Pure validation
checks final LSI buses and configured factory HSI at final AHB/APB divisors,
because that HSI edge precedes the final LSI selection.

For this new LSI target, requesting either HSE or LSE additionally requires
undivided factory HSI to fit with AHB/APB /1. Dynamic admission requires that
same no-divider-credit 48.96 MHz envelope wherever selected HSE/LSE with
CLKCCS=1 could fall back or a proposed HSE bridge has that policy. This requires
VDD≥1.8 V. It is a conservative refusal rule, not a hardware divider-reset
claim. Internal-only low-voltage operation remains possible when all actual
intermediate HSI/divider edges fit 24 MHz. Neither L052's fixed HSI /6 escape
nor its 48 MHz ceiling is imported. Flash WAIT2 stays in place throughout the
transition and on success; it cannot legalize an excessive bus rate.

The caller hands over clock-dependent application activity and the functional
effects of temporary whole-GPIOA/B/C/D/E/F inspection windows. Each bank can
resume unrelated pins, sampling, filters, edge/level interrupts/events,
alternate-function outputs and external pin feedback, including before a
later refusal. Closing a gate cannot undo this progress. Clock-dependent
clients and externally exported or looped clock signals must be quiescent.
This functional contract adds no hidden Rust memory-safety precondition.
Existing exclusive clock/memory ownership and legal execution at entry remain
required; a critical section does not freeze hardware.

A selected factory-qualified source or declared external/PLL source is checked
against actual entry divisors and incoming Flash latency. A selected inherited
non-factory HSI is preserved only under the existing legal-entry handover,
with legal divider encoding, coherent readiness and unchanged parameters.
It receives no factory rate claim before calibration, is not newly selected
or accelerated, and cannot justify an HSI PLL reference, new bridge or qualified
fallback. Selected HSE/LSE and an enabled HSE-fed PLL require their corresponding
board declaration. Unrequested, unselected external sources that are not PLL
references can be preserved without inventing rate bounds.

Incoming Flash latency comes from always-readable SYSCTRL.CR2.FLASHWAIT.
Reserved values 3…7 refuse. Do not read local FLASH.CR2 while its gate is off.
Once the central Flash gate is enabled, local WAIT must agree with the captured
mirror before setting WAIT2; both must then acknowledge 2. Other controls,
including SYSCTRL.CR2, remain preserved. If fallback is possible at entry, the
entry state must already satisfy the no-divider-credit HSI bound and WAIT2.

## Complete consumers and immutable entry

Admission captures full normalized sources, observers, gates/resets,
RTC/AUTOTRIM/LVD controls, six UARTs, six GPIO banks, MCO, clock output routes,
external oscillator pads, and tagged LCD/LPTIM controls. Source→consumers→source
passes run twice and must agree. A fresh complete collection also precedes each
actual trim/request/selection/reference use edge. Gate-off LCD/LPTIM work stays
off with no local access; cold gate-on LCD requires EN=0 and BUMP=0 independently,
and cold gate-on LPTIM requires EN=0 plus completed ARST/SRST commands.

Cold admission rejects RTC SOURCE2, AUTOTRIM direct/implicit LSI calibration,
UART1…6 SOURCE3, GPIO FLTCLK5, MCO4, every bonded direct LSI route and ready
observers even if cold TRIM already matches. UART RX/TX disable does not prove
its serial/synchronous UCLK unused. Retained AUTOTRIM automatic or active
calibration refuses. GPIO FLTCLK7 has its documented LPTIM dependency; it is
not treated as an undefined selector. No reviewed internal MCO-to-timer selector
or UART timeout/general-timer claim is imported from L052. Independent IWDT
RC10K, PCLK-derived WWDT/timers/SPI/I2C/ADC, GPIO cascades and external feedback
retain their actual roots and handover limits.

The complete LSI roster is independently validated from own AF and bond maps;
the legacy scalar PC4 field is not the roster. Unbonded native PF2 and PD5 fields
must remain AF0 on RBT6/RCT6/RCS6; unbonded PD5 must remain AF0 on MCT6. These
conservative register guards apply to every entry class and create no physical
pin tokens. VCT6 has no unbonded LSI guard. Full AFR words and gate/reset state
are retained. A parked bonded output with an LSI AF is still a source owner.
Reserved AF alternatives refuse; software never rewrites one to gain admission.

Entry classification never changes:

- Cold: request0, source not selected LSI, both ready0, detectors0 and no cold
  LSI ready observer/event or SYSCTRL pending state. Matching trim omits only
  the write, never this stopped-source proof.
- LiveReady: request1, coherent ready1 and factory TRIM match. Preserve source
  parameters and legal consumers without stop/retrim.
- LiveStarting: request1, coherent ready0, not selected LSI, detectors0 and
  factory match. Readiness/event/pending progress before the owned permanent
  request refuses; a later ready bit cannot reclassify entry.

Selected-but-unready, request0-but-ready, incoherent mirrors, live trim mismatch,
reserved source/divider/mode encodings, observed relevant external faults,
snapshot drift or unprovable bounds/owners refuse. SYSCTRL IRQ4/vector20 observes
startup/ready events; CLKFAULT IRQ31/vector47 observes external runtime faults.
Preserve IER, ICR and NVIC state. No event clear, timer/watchdog stop, LCD pump
write or peripheral reset manufactures admission.

## Transition, PLL and final publication

An inherited enabled PLL is validated from its actual reference, multiplier,
input/output bins, parameters and ready state. Bonded PLL_OUT owners are PC12
AF5 and PC13 AF1 on all five parts, plus PF8 AF3 on MCT6/VCT6; MCO7 is also an
owner. On the three R packages, native unbonded PF8 must remain AF0. Complete
AFR/gate/reset snapshots apply. Projected bonded PLL_OUT metadata fills an
existing source-backed omission; it is a separate intentional metadata delta
and does not grant a new PLL-output API or pin token.

If PLL is selected, first use a proven unchanged HSI or HSE bridge; direct
PLL→LSI/LSE is forbidden. Prefer factory-matching HSI, requesting it at unchanged
matching parameters only if it is not the active PLL reference. Otherwise an
HSE-fed PLL may use its already-running declared HSE reference when all bounds,
Flash and fallback checks hold. Never start a new HSE or change reference
parameters to manufacture this escape. Clear only PLLEN after owner proof;
require PLLEN0, PLL.STABLE0 and ISR.PLLSTABLE0 before changing any actual
reference/HSI parameters. Once stopped, any re-enable or readiness rise refuses.
PLL parameters, old events and IRQ ownership remain unchanged. Returned clocks
contain no active PLL rate/bounds.

The ordered transition is:

1. Pure validation and immutable entry, followed by two equal complete buffered
   snapshots before Flash/source/pad changes.
2. Central Flash enable and coherent WAIT2; monotonic AHB at least /4 and APB
   at least /8 guards, preserving slower inherited divisors. Never replay an
   external selector that hardware fallback could have changed.
3. Proven departure from selected PLL and full PLL stop acknowledgment, if needed.
4. Fresh cold proof, one differing LSI TRIM-only write if required, then a second
   fresh proof immediately before permanent LSIEN1. Live classes never retrim.
5. Coherent LSI readiness, permanently latched through every later phase.
6. Guarded LSI execution for owner-checked HSI stop/trim/restart when needed;
   preserve HSI DIV until PLL is stopped and trim is admitted. Matching HSI skips
   stop/trim but still requires enabled coherent readiness.
7. Set requested HSI divider while on guarded LSI, verify bounds and then select
   guarded factory HSI. Keep permanent LSI request/readiness throughout.
8. Reuse matching requested HSE/LSE or perform their own cold-owner/pad admission.
   Permit only exact native pin/parameter/request/detector deltas. HSE may retain
   the admitted GPIOF gate; LSE bypass changes only PC14 ANALOG and restores its
   temporary GPIOC gate. Do not bulk recapture arbitrary state as an expected
   snapshot. Preserve CLKCCS and unrelated pins, events and controls.
9. Complete configured HSI at final bus divisors and verify the whole tree,
   including PLL stopped and Flash WAIT2.
10. Source-only final LSI selection. No later CR0/CR1, oscillator, Flash, pad or
    policy write is permitted. Temporary read-only inspection gates still
    restore exactly. The last hardware observation is the exact normalized CR0
    read; successful caller publication follows the complete final seal.

Optional cold LSE also requires its own reset-like RTC/pads and all LSE owners
closed, immediately before each distinct pad/parameter/enable use edge. Ordinary
LSI admission does not perform unconditional calendar reads. Its factory-LSI
monitor follows the existing source-specific detector margin, not strict
cycle timing or recovery guarantees.

Each phase permits only its explicit old/target field changes. Local and ISR
ready observations must agree. Ready may rise only during its own request;
ready may fall only in an admitted HSI/PLL stop. Once observed ready, permanent
LSI cannot lose request/readiness in any later wait. Events may only rise after
their owned request; unrelated history and NVIC enables stay fixed. New
CLKFAULT pending or an observed relevant fault refuses. No pending bit alone
identifies the source of a ready event.

Restoration debt is handled before semantic results: gate-restore failure
outranks gate-enable failure; after successful access/restoration, observed
relevant external failure outranks semantic/readiness/drift errors, which
outrank poll exhaustion. Poll limits are attempts, not elapsed deadlines.
No error promises rollback. A failed RCC attempt publishes no frozen clocks or
peripheral tokens and may leave partial gates, pads, trim, PLL stop, conservative
bus/Flash state or permanent LSI request. Reset before another attempt; retained
LSE controls can require power reset. Loss of an execution clock may prevent
return, and STABLE cannot prove continuing LSI health. Runtime switching,
source-loss recovery, sleep/wake restoration and silicon behavior are unqualified.

## Finite software acceptance and separate future recipe

The finite main scope is ten real `embassy-cw32` rlib builds and ten linked ELFs,
all for `thumbv6m-none-eabi`, with the existing locks and `--locked --offline`:

| Main libraries | Profile / features |
|---|---|
| Exact five L083 parts | release; exact chip, `defmt,time-driver-gtim1` |
| CW32L083RCT6 | dev; `cw32l083rct6,defmt,time-driver-gtim1` |
| Generic CW32L083 | release; `cw32l083,defmt` |
| CW32L052C8T6, CW32L010F8P6, CW32F030C8T7 | release; each exact chip, `defmt,time-driver-gtim1` |

| Main ELF | Exact features / binary |
|---|---|
| `examples/shared/lsi-clock`, five L083 parts | each exact chip plus `defmt`; `cw32-lsi-clock-example` |
| `examples/shared/lsi-clock`, L052R8S6 | `cw32l052r8s6,defmt`; `cw32-lsi-clock-example` |
| `examples/shared/pll-clock`, RCT6 | `cw32l083rct6`; `pll-uart` |
| `examples/shared/pll-clock`, RCS6 | `cw32l083rcs6,hse-crystal`; `pll-uart` |
| `examples/shared/lse-sysclk`, MCT6 | `cw32l083mct6,defmt`; `crystal` |
| `examples/shared/rtc-calendar`, VCT6 | `cw32l083vct6`; `preserve_calendar` |

All main ELFs use release. Clean replay uses RCT6/VCT6 release libraries with
`defmt,time-driver-gtim1`, and RCT6/MCT6/VCT6 release LSI ELFs with `defmt`:
two libraries plus three ELFs. Use an independently reconstructed final source
package and target, preserving actual rlibs/ELFs, logs, cfg, emitted OUT_DIR
files, linked memory maps and source hashes. Main products cannot stand in for
clean builds. Inspect actual entry/vectors, load segments, Flash use, static RAM
and stack headroom; never enlarge `memory.x` to hide an overflow. RBT6's 128 KiB
limit is exercised by its main exact ELF and independent package validation.

All 54 chip objects are compared separately: positive LSI capability increases
from 29 to 34 exact/profile objects; exact-five bonded PLL_OUT metadata is the
other declared change. Preserve old profiles, RTC numeric tuples, generic/native
exclusions and historical LSE-only qualification flags. The existing L083 LSE
validator compares current exact-five LSI metadata against the full independently
validated policy while retaining the historical statement that the earlier LSE
addition itself did not qualify LSI. Preserve old L052 source semantics and
its no-PLL/fixed-/6/final-latency policy in the shared runtime.

The accumulated `ci/check-lsi-clock.sh` is a reusable future local recipe,
not this acceptance matrix or its execution record. Its six added library
commands and eight added ELFs bring its declared totals to 36 library commands
(31 builds and five historical checks) and 47 linked ELFs, reusing its existing
RCT6 release library and HSI-fed PLL ELF. Native L010 and actual F030 release
library builds required above remain explicit finite-matrix rows; the older
F030 recipe check is not relabeled as a build. Do not run the aggregate recipe
merely to collect this slice's receipts. No hosted CI, HAL tests, probes,
executable transition models, RF work or hardware results are implied.
