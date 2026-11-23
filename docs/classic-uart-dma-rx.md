# Classic finite staged UART DMA RX

Stage92 extends the existing `UartRx::new_with_dma` and `Uart::new_with_dma`
API to fourteen exact F020/L031/L052/R031/W031 parts. This is finite staged
byte receive, including existing `split` and `split_ref` combined TX/RX owners.
The UART and DMA runtime algorithms are unchanged; the existing family policy
admits RX using each family's own source qualification. L012 remains TX-only
for UART DMA. Ordinary, idle and IRQ-buffered UART APIs retain their separate
contracts.

Main software verification passed eight standalone ARM libraries and 21 ELF
links, all 29 commands exiting zero without warnings. The two finite source
checks passed after the preserved initial checker failure. Independent
runtime and metadata/source reviews accepted the bounded main result. At main acceptance, final capability reports
and the separate final-source two-library/three-ELF clean replay had not run.
Earlier Stage84/85 copy/TX receipts remain historical and
do not verify this RX extension. No hardware transfer, error injection, wire
timing, throughput or bounded-liveness result is claimed.

## Exact parts, channels and package limits

| Family | Exact selections | Physical DMA channels and IRQs |
| --- | --- | --- |
| F020 | CW32F020C6U7, CW32F020K6U7, CW32F020F6U7 | CH1 / DMACH1 IRQ9; CH2 / DMACH2 IRQ10 |
| L031 | CW32L031C8T6, CW32L031C8U6, CW32L031F8P6, CW32L031F8U6, CW32L031K8U6, CW32L031K8V6 | CH1 / DMACH1 IRQ9; CH2 and CH3 / DMACH23 IRQ10; CH4 / DMACH4 IRQ11 |
| L052 | CW32L052C8T6, CW32L052R8S6, CW32L052R8T6 | Same native four-channel topology, independently sourced |
| R031 | CW32R031C8U6 | Same native four-channel topology, independently sourced |
| W031 | CW32W031R8U6 | Same native four-channel topology, independently sourced |

Each exact part has SRAM `[0x20000000,0x20002000)`. F020 has 32 KiB Flash;
the other eleven parts have 64 KiB. Generic `cw32f020`, `cw32l031`, `cw32l052`,
`cw32r031` and `cw32w031` keep empty memory metadata. Their direction surface
can compile, but safe admission/staging returns `MissingMemoryMetadata`;
no alias memory fallback or example linker map is supplied.

UART1/2/3 RX request selectors are 0/2/4 and TX selectors are 1/3/5, selectable
by every physical channel of the family's native controller. RX-only needs an
existing safe RX pin. Combined construction needs safe TX and RX on distinct
pins and two distinct channels. Every existing package/AF filter remains:

- F020F6U7 and L031F8P6 have no safe UART1 RX or TX route.
- L031F8U6 supports UART1 RX-only on PA9/AF2 or PC15/AF6, but its only UART1
  TX pad is excluded PA14. Combined UART1 remains unavailable on that package.
- PA13/PA14/PF3 stay excluded. A request trait never grants an unbonded pin.
- W031 UART3 PA2/AF5 has its own datasheet authority; its missing SDK cell
  is not fabricated or substituted from another family.

The [own-source ledger](classic-uart-dma-rx-evidence.json) binds all fourteen
exact memory records and complete safe RX/TX pin projections, including pad
numbers and unavailable routes. Existing [F020](f020-dma-owned.md) and
[four-channel](four-channel-dma-owned.md) records retain the native controller,
copy and TX evidence. R031 whole-GPIOA effects and W031 internal RF boundaries
remain functional handover limits; adding bus traffic proves no RF continuity
or coexistence.

## Positive own-source receive authority

The selected originals are already locked in `sources/evidence-sources.json`.
The new ledger binds their exact PDF/text/member bytes, archive chains, own
page hashes, complete UART ISR fields, selectors and package routes. Printed
pages below equal zero-based PDF indices; viewer page numbers are one greater.
Shared register layouts do not substitute for any family's own evidence.

| Family | Own manual | Byte RDR-to-SRAM example | Own SDK UART RC line | Request table | RDR / IER / ISR / ICR |
| --- | --- | --- | --- | --- | --- |
| F020 | CN Rev1.4 | §17.7.1.6 p288 | 330 (also 383) | p137, continuation p138 | pp295 /296 /297 /298 |
| L031 | CN Rev1.6 | §18.7.1.6 p345 | 357 (also 416) | p133 | pp354 /355 /356 /357 |
| L052 | CN Rev1.5 | §19.7.1.6 p382 | 339 (also 398) | p138, continuation p139 | pp391 /392 /393 /394 |
| R031 | CN Rev1.3 | §18.7.1.6 p348 | 357 (also 416) | p135 | pp357 /358 /359 /360 |
| W031 | CN Rev1.4 | §18.7.1.6 p348 | 354 (also 413) | p134 | pp357 /358 /359 /360 |

Each example explicitly receives into a `uint8_t` SRAM array with fixed UART
RDR source, incrementing destination, SIZE=byte, BLOCK, REPEAT=1 and hardware
UART RX request. It enables DMA before DMARX, observes TC and then closes
DMARX. Each family's UART instance map, own UART2/3 SVD derivation and request
table extend that endpoint qualification from the UART1 example to UART2/3.
This narrow DMA byte access does not change ordinary typed 32-bit CPU MMIO.

All five expose PE bit4, FE bit3 and RC bit2, with no native ORE/NE flag. Their
own SDK drivers explicitly say reading RDR does not clear RC. Unread RDR can
be silently overwritten, so a clean DMA result establishes neither lossless
reception nor a loss count. RC after DMA is software-acknowledged; the CPU
must not read RDR as an extra byte while DMA owns it.

UART clear commands begin with the selected typed `Icr::write_noop()`:
`0x000000ff` on F020 and `0x00000fff` on the other four families. Only observed
owned FE/PE/RC fields are changed to zero; TX/CTS, reserved seeds and L052 RFU
bit0 remain intact. The manual's blanket-zero example and SDK ICR
read-modify-write are not the safe driver's command contract. Native DMA ICR
starts at `0xffffffff` and clears only the selected channel's observed TC/TE.

## Startup, launch and successful retirement

Normal reset or trustworthy clean-runtime entry is required. One-time DMA
admission rejects missing SRAM, inherited gate/reset state, failed bounded
clock acknowledgment or nondefault descriptors/status after enabling the gate.
It never resets or clears dirty state into admission, reads gated registers as
proof, or authenticates arbitrary bootloader bus history. Raw acquisition or
downgrade permanently revokes safe admission.

Constructors consume static UART/pin/channel tokens and exclusive static SRAM
staging, 1..=65535 bytes per direction. The whole allocation must lie inside
exact SRAM; the count ceiling does not enlarge the 8 KiB memory. Combined
construction validates configuration and both full stage ranges before atomic
pair admission and before UART preparation. Rejected inputs are consumed.
Successful channel admission can enable DMA IRQs, but launches no descriptor.
UART preparation retires any old software ring before configuring this owner;
there is no same-instance buffered-plus-DMA constructor or conversion.

For a new chunk the owner closes DMARX, masks RX waiters and samples/acknowledges
only observed RX flags. Preexisting PE/FE is returned before launch. It publishes
InFlight before the first possible DMA start/panic boundary, programs the typed
RDR-to-private-SRAM descriptor, publishes Running with a fence, enables DMA,
and only then opens DMARX. No caller-buffer address becomes a DMA endpoint.
Unexpected launch failure permanently quarantines the lease.

Completion waits register UART and DMA wakers before final observations. Only
FE/PE interrupts are armed; RC remains masked. The no-ring UART ISR masks and
wakes without consuming RDR or clearing status. The selected DMA channel can
also service its own terminal flags through a nonblocking observation when
CPU interrupts are masked. A TC that predates the first poll remains recorded.

Success requires recorded TC, no TE and typed STATUS=Complete (5). TE wins
over simultaneous TC; inconsistent completion is permanently unconfirmed.
Hardware RX does not use SOFTSRC as completion evidence. On clean completion
the owner closes DMARX, samples FE/PE again, acknowledges observed FE/PE/RC,
fences and retires DMA, and only then permits CPU reads from staging. A clean
invalid chunk is discarded after retirement. UART flags, EN=0, count=0,
request closure, a stop request, timeout, reset or clock-off do not prove that
outstanding DMA bus accesses have drained.

## Reads, line errors, cancellation and retained resources

The persistent receive lease belongs to `UartRx`, independently of its current
future. Idle permits a new chunk; InFlight owns RDR/staging exclusively; Ready
permits copying clean bytes and retaining a partially consumed tail; Quarantined
permanently reserves the affected channel, staging and UART resources.

An observed PE/FE returns promptly while an incomplete DMA chunk remains
running with DMARX open. That chunk is invalid and reports its line error at
most once. Parity takes precedence over framing; a later higher error can
update retained invalidity without a second line-error report. A DMA fault
still wins and permanently quarantines. Recovery through a later inherent
read awaits the original count and discards the invalid chunk only after
clean retirement. It can wait indefinitely if the peer sends too few bytes.

Cancelling or forgetting a read does not stop DMA or free staging. Cancellation
masks that waiter's error interrupts; forgetting can leave them armed until an
ISR or later operation masks them. Already copied caller prefixes remain.
An incomplete chunk never exposes its staging or writes to caller memory.

| Entry point | Contract |
| --- | --- |
| Inherent async `read` | Reap an old chunk, deliver clean Ready bytes, then arm finite chunks of the smaller of staging capacity and remaining requested length until filled. |
| Empty inherent async `read` | May await old work or return its error; starts no new DMA and consumes no Ready bytes. |
| `blocking_read` | Observe old DMA once; pending yields `DmaBusy`. Copy Ready bytes first, then use CPU RDR only when the DMA lease is Idle. Starts no new DMA; ordinary remaining bytes may block. |
| Empty inherent blocking read | Make the same nonblocking observation and return success or its error; consume no Ready bytes. |
| Nonempty `embedded_io::Read` | Call blocking exact read for one byte and return 1. |
| Nonempty `embedded_io_async::Read` | Check old work without waiting; an abandoned pending chunk yields `DmaBusy`. Otherwise read one byte, using count 1 if a new DMA chunk is needed, and return 1. |
| Empty sync/async Read trait calls | At most one nonblocking completion observation, then zero or an ordinary error; no recovery await or staged-byte consumption. |

The [shared RX contract](uart-dma-rx.md) covers combined owners and their split
halves. Each direction keeps its lease through `split`/`split_ref`. Shared
CR2/IER updates use the existing critical section. A retained half cannot
disable its live peer or overwrite the peer's request gate. Drop of Idle/Ready
RX can discard the clean tail and release the RX pin; unresolved or quarantined
RX retains channel/staging/pin/configuration/clocks without waiting for the
peer. Once neither half is live, UART dispatch is inactive; clocks remain on
while either direction is retained. A late favorable completion cannot
resurrect an abandoned owner or restore consumed tokens.

F020's DMA vectors stay independent. CH2/CH3 on the other four families share
DMACH23, requiring both handlers when both channels are used; each handles only
its own flags. No channel lifecycle resets/gates the controller or disables or
unpends a sibling's vector. Existing L083 shared-UART behavior is unchanged.

## Application and verification boundary

The existing [UART DMA firmware](../examples/shared/uart-dma/README.md) covers combined
and standalone receive through ordinary local caller arrays. For all fourteen
new exact parts its main uses UART2 PA6/AF2 TX and PA7/AF2 RX. F020 uses TX CH2
and RX CH1 with separate IRQs; the four-channel cohort uses TX CH3 and RX CH2
with both handlers on DMACH23. It polls RX first, demonstrates borrowed and
owned splits, and explicitly flushes TX. Separate `receive_only` images use
CH1/DMACH1, including L031F8U6 UART1 RX without a safe TX route. These are
separate firmware images, not three simultaneous channel owners on F020.

Main linked all fourteen combined images, five family-representative receive-only
images, F030 main and L083 shared-vector firmware, plus eight standalone HAL
libraries including L012's unchanged TX-only profile. All 29 commands exited
zero without warnings. The 21 retained HAL dependency archives from ELF links
are not additional standalone library commands. Source, toolchain and Cargo
configuration stayed unchanged throughout the matrix. Main ran no canonical data
generation; all 910 canonical generated files remained byte-identical.
This later documentation/status update changes no HAL or example code.

The two finite checks, DMA own-source facts and source provenance, passed in
the continuation. The original D01 failure compared SDK fields using PINn
instead of their actual AFRn spelling; the approved one-token checker correction
changed no HAL or example bytes. That failed attempt and a separate pre-command
recorder stop remain recorded. The recorder correction retained source-input
bindings and the earlier successful navigation refresh. These are source and
receipt-preparation outcomes, not runtime or silicon failures.

Independent runtime and metadata/source reviews accepted the bounded main
result. At main acceptance F01/F02
capability rendering/checking and the fresh final-source clean replay had not
run. Clean requires F020C6U7/W031R8U6 libraries and F020C6U7 main, L031F8U6
receive-only and W031R8U6 main from a fresh source extraction and target,
with its own generation, source checks, 910-file correspondence and package
identity receipts. Main acceptance does not replace those separate results.

This remains explicitly armed finite reception. Executor, CPU-copy and rearm
gaps can lose frames without overrun indication. The peer must provide enough
bytes for the programmed count. No continuous, ring, circular, scatter, idle-
terminated or RTS/CTS DMA, borrowed zero-copy, safe early abort, post-error
reclamation, resource-return API, new raw hardware requests, other peripheral
consumer, low-power integration or all-DMA-complete claim is supplied.
