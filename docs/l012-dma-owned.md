# Native L012 owned DMA

The original Stage86 implementation qualifies only CW32L012C8T6/C8U6 for
init-admitted static-SRAM software copying and private-staging byte UART TX.
Its main implementation and independent runtime/source reviews are accepted;
at main acceptance clean had not run and requires a separate final-source
receipt. Stage94 additionally declares finite paired SPI1/SPI2/SPI3 DMA on the
same two exact parts, with separate main software acceptance. At Stage94 main
acceptance final-source clean had not run; its separate receipt remains required.
The original copy/UART acceptance does not validate the native SPI extension.
No hardware execution is claimed.

## Own hardware and source conflicts

Both parts have Flash [0,0x10000) and SRAM [0x20000000,0x20002000). Generic
`cw32l012` has no exact memory metadata and cannot acquire safe copy/staging.
Native DMA base is 0x40020000, with four channels at +0x20/+0x40/+0x60/+0x80.
CH1/2 share DMACH12 IRQ9; CH3/4 share DMACH34 IRQ10. IRQ11 remains CORDIC.
Each inactive handler returns before MMIO; a running channel only observes and
clears its own flags and never masks, resets or gates a sibling's controller.

The descriptor contains CSR, programmed CNT, SRCADDR, DSTADDR, TRIG, current
CCNT, CSRCADDR and CDSTADDR. Initial admission checks all eight reset values.
CSR is reset zero. RESTART, SRCLOAD and DSTLOAD are explicitly false at launch;
REPEAT=1, equal widths and BLOCK semantics preserve the existing copy protocol.
Remaining-count observation reads CCNT, never the unchanged programmed CNT.

Own CN RM V1.4 and corroborating EN V1.0 govern these fields. Original SVD/header
mark CCNT/CREPEAT RW while the manual marks them RO: five exposed register views
are corrected through existing overrides. CSR TC/TE are already RO in SVD/header:
ten mixed-register setter restrictions restore lost access information. STATUS
remains RW. Native SIZE/STATUS/TRANS/TYPE fields become typed enums. This is a
public PAC source compatibility change; CCNT register handles lose MMIO writes,
though detached value structs may retain setters. No compatibility shim is added.

ICR is R1W0, reset/no-op 0xffffffff, with TC/TE pairs at bits0/1,4/5,8/9,12/13.
Typed write-noop clears only selected flags. The SDK clear helper mixes interrupt-
enable masks with status flags and shifts incorrectly; neither error is copied.
The original vendor bytes remain untouched and external.

## Admission and lifetime

Normal reset or trustworthy clean-runtime entry is required. Existing keyed RCC
enable helpers are used; reset is active-low and unkeyed. An inherited enabled,
reset or dirty controller rejects admission rather than being reset into safety.
A gate enabled before descriptor rejection remains enabled. Generic memory
metadata is never inferred from the family name. Raw acquisition permanently
revokes safe admission; safe ownership cannot be recreated from a later idle read.

The sealed u8/u16/u32 copy API requires aligned, disjoint, complete static SRAM
ranges and count1..65535. Ownership is installed before launch. Only recorded
own TC without TE, STATUS5, SOFTSRC0 and a fence return the original allocations
and channel. TE wins. EN clear, zero count, stopped status, clock-off or a timeout
are not bus-drain proofs. Errors or ambiguous completion retain owners forever;
forget leaks and Drop may wait indefinitely. The existing unsafe software-copy
contract remains separate; borrowed hardware constructors remain x030-only.

## Byte UART TX

UART1/2/3 use native bases 0x40000c00/0x40001000/0x40002000 and TDR offset0x28,
TX request selectors1/3/5 on any of the four channels. Both package routes are
UART1 PA9 AF1 (pad30), UART2 PA7 AF1 (pad17), UART3 PA8 AF1 (pad29).
The example binds channels1/2/4 and both real shared vectors. Existing safe pin
and GPIO whole-bank functional handover constraints remain applicable.

Own CN RM p474 demonstrates SRAM bytes into TDR with SIZE0. This qualifies the
internal byte address carrier for this endpoint, not arbitrary byte MMIO. CPU
MMIO remains typed. Static UART, pin, channel and staging are consumed; ordinary
caller slices are CPU-copied into bounded private chunks and never exposed to DMA.
The lease is in-flight before EN; EN precedes DMATX. Recorded clean TC/noTE plus
STATUS5 precedes request closure, fencing and staging reuse. Hardware-trigger TX
does not require software SOFTSRC completion. Flush separately waits for TXBUSY0.

Cancellation/forget retains the current chunk; a later write or flush settles it
before reuse. Error or unresolved Drop retains staging, channel, UART/pin/config
and clocks. Clean Drop need not flush the final frame. Gaps, CPU copy overhead,
unbounded waiting and no throughput guarantee remain. The original TX qualification did not include RX/combined DMA UART,
circular/restart API, early abort, other unqualified requests or buffered-plus-DMA
owner is added. Existing ordinary/buffered/idle RX paths remain independent.
The L012 UART driver now also exposes finite RX and combined RX/TX for the servo example.
The native CN V1.4 manual §21.7.1.6 (p475) configures byte RDR-to-SRAM block DMA,
channel EN before DMARX, and closes the request on TC. RX selectors are 0/2/4.
The staged receiver checks ORE/NE as well as PE/FE and discards errored chunks.
RC/ORE behavior at full wire speed has not been verified on the servo board.

## Stage94 finite paired SPI

Exactly CW32L012C8T6/C8U6 gain SPI1/SPI2/SPI3 staged full-duplex eight-bit
master DMA: one family, three controller pairs and six selected part/controller
routes. RX/TX selectors are 6/7, 8/9 and 10/11. Native CR2 request gates preserve
sampled EN and unrelated fields. TYPE1/BLOCK/SIZE0 and per-chunk REPEAT1 use
fixed typed DR addresses and incrementing private byte SRAM; SIZE0 is not a
CPU byte-MMIO or byte-peripheral-access license. See the
[SPI lifecycle and native source contract](spi-dma.md#native-l012-scope-and-shared-interrupts).

Whole static SPI/pins, both channels and two exclusive static stages are retained
before launch. Admission remains sequential: second-channel failure can consume
the first admission, and later SPI configuration failure does not restore it.
Generic `cw32l012` has the compile surface but retains `MissingMemoryMetadata`
for safe staging/admission. Four channels permit two reserved full-duplex pairs,
with PIO available on a third controller; clean Drop does not permit re-admission.

PIO and DMA constructors both require the complete associated SPI23 handler
for SPI2/SPI3; all four peer/backend visits guard activity before MMIO and leave
inactive banks untouched. The DMA constructor's associated-handler bound may
require a downstream generic wrapper update even though concrete dedicated
bindings still resolve to the same handler. Old PIO construction stays borrowed.

Both clean DMA terminals, error-free TXE1/BUSY0 and both fenced retirements are
required before CPU access to received bytes. Cancellation retains the active
chunk: keep the original GPIO CS asserted through successful flush. Errors
permanently quarantine the pair. Unresolved Drop retains both stages/channels,
SPI, pins and clocks without disabling or gating them; it does not establish
safe CS release. No early abort, recovery, channel release, generic
cancellation-safe device adapter or finite-liveness guarantee is added.

Stage94 main passed six standalone ARM libraries and fifteen ELF links in
21 accepted Cargo rows, all exiting zero, plus five finite source/declaration
commands. Independent runtime and metadata/source reviews accepted the bounded
implementation. At main acceptance final-source clean had not run; completion
requires its separate two-library/six-ELF receipt after one existing generation
bootstrap. Main ran no canonical generation, no cumulative recipe is expanded
and no hardware result is claimed. The
[native SPI ledger](l012-spi-dma-evidence.json) is separate from the original
Stage86 source and review record below.

## Verification boundary

The [source ledger](l012-dma-owned-evidence.json) binds exact originals, archive
members, own page coordinates, native geometry/access/enums, clocks, IRQs and
package routes. The existing DMA source checker verifies current facts. Historical
full generated comparison is external one-time evidence, not a permanent gate.
Main generation added three sidecar files and changed nine native products,
leaving all54 chip JSONs and earlier-family projections unchanged.

Stage86 main passed four standalone ARM libraries and eleven ELF links,
all fifteen commands exiting zero with no warnings. The 1,413 authored files,
910 generated files, toolchain and Cargo configuration stayed unchanged.
Generation and the DMA projection check passed in one successful segment;
the own-source checker, provenance, renderer, consistency and layout checks
passed in a separate five-command tail. Two earlier preparation failures are
preserved: current provenance misread retained historical source identities,
and a CSR prefix also matched CSRCADDR. The precise corrections preserve full
source identity and all five CSR access assertions; no original or hardware
fact was changed. At main acceptance clean had not run; final-package completion
requires a separate final-source two-library/four-ELF clean receipt.
Each phase has one successful ordinary generation, five finite gates and one
renderer. The [runtime review](l012-dma-runtime-review.json) and
[metadata review](l012-dma-metadata-review.json) bind actual main code,
source facts and generated bytes. Two historical source identities are explicitly
labeled historical_path/historical_sha256 in the ledger; the checker reverses
only these keys before comparing every original identity field. No HAL tests, models, probes,
mutation matrix or hosted CI is included. These checks do not establish silicon
transfer, wire behavior, error-time drain or finite liveness.
