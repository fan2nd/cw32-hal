# Safe staged UART TX DMA on qualified CW32 parts

`UartTx::new_with_dma` adds a TX-only, byte-oriented DMA constructor to the
existing async UART owner. It consumes static UART and compatible DMA singleton
tokens, a matching TX pin, both real interrupt bindings and one exclusive
`&'static mut [u8]` staging buffer. Its `ConfigError` reports UART configuration,
DMA staging or DMA admission rejection before any transfer starts; rejected
owners are consumed just as with the existing UART constructors.

The existing safe `write(&mut self, &[u8]).await` and `flush().await` signatures
remain the application interface, including the ordinary async I/O trait path.
Inputs can be flash, local/stack, heap or static slices: the CPU copies them into
private staging and their addresses never reach DMA. This implementation does
not expose an arbitrary peripheral register, DMA request number or raw pointer
through a new safe API. Matching pins and `dma::RequestRoute<T, signal::TX>` come
from the existing authored/generated routing data.

F020 admission is limited to CW32F020C6U7/K6U7/F6U7, with 8 KiB SRAM at
`0x20000000..0x20002000`; generic `cw32f020` rejects safe DMA and staging with
`MissingMemoryMetadata`. Native CH1/CH2 use separate DMACH1/DMACH2 IRQs.
Generated UART1_TX/2_TX/3_TX selectors 1/3/5 work on either physical channel,
subject to the existing safe TX pin routes. F6 UART1 has only excluded PA14
for TX, so it has no safe TX constructor. Stage92 separately extends
[finite RX and combined/split DMA](classic-uart-dma-rx.md) on the three exact
parts, subject to safe receive pins; F6 UART1 has no safe RX either. SPI DMA
and borrowed hardware-request constructors remain excluded. See
[the own-F020 contract and exact example pads](f020-dma-owned.md).

## Native four-channel cohort

The native L031/L052/R031/W031 staged TX scope covers exactly the eleven parts
below. Each has 64 KiB flash and 8 KiB SRAM at
`0x20000000..0x20002000`. Generic `cw32l031`, `cw32l052`, `cw32r031`
and `cw32w031` compile the controller/UART DMA surface but reject safe DMA admission
and staging with `MissingMemoryMetadata`; they have no example linker map.

The example selects the following existing safe pin/AF and channel combinations.
Request selectors are UART1_TX=1, UART2_TX=3 and UART3_TX=5, generated from
the qualified own-family records and usable on any physical CH1..4. The chosen
channels exercise shared DMACH23 on CH2/3 and independent DMACH4 on CH4;
CH1 has DMACH1. There is no CH5 or DMACH45 on this controller.

| Exact feature | UART1 / CH4 | UART2 / CH2 | UART3 / CH3 |
| --- | --- | --- | --- |
| `cw32l031c8t6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l031c8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l031f8p6` | No safe TX pin | PA6 AF2 | PA10 AF6 |
| `cw32l031f8u6` | No safe TX pin | PA6 AF2 | PA10 AF6 |
| `cw32l031k8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l031k8v6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l052c8t6` | PA8 AF2 | PA6 AF2 | PA9 AF1 |
| `cw32l052r8s6` | PA8 AF2 | PA6 AF2 | PA9 AF1 |
| `cw32l052r8t6` | PA8 AF2 | PA6 AF2 | PA9 AF1 |
| `cw32r031c8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32w031r8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |

L031F8P6/F8U6 UART1 has only excluded PA14 for TX; a request trait does not
provide a safe bonded pin. PA13/PA14/PF3 exclusions stay unchanged. The L052
UART retains its own PAC version. Each family's own UARTx contract, UART2/3
SVD derivation and request table support its own byte SRAM-to-TDR example;
matching register layout alone grants no peripheral-DMA qualification.

The original Stage85 qualification covered staged TX and owned SRAM copying.
Stage92 separately adds [finite RX and combined/split UART DMA](classic-uart-dma-rx.md)
on these eleven exact parts. L031F8U6 UART1 permits RX-only on PA9/AF2 or
PC15/AF6 despite its absent safe TX route; L031F8P6 UART1 has neither direction.
No SPI DMA, other request, borrowed hardware constructor, buffered-plus-DMA
owner, flow-controlled DMA, idle DMA or recovery API is added.
Ordinary and buffered UART owners keep their separate state and scope. R031
GPIOA whole-bank effects remain relevant even on PA6/8/10, which avoid internal
RF pins PA0..3; W031 GPIOA routes avoid the internal RF SPI1/GPIOB routes.
These choices establish no RF-safe functional handover or coexistence.
See [the independent four-family contract](four-channel-dma-owned.md) and
[all own-source bindings](four-channel-dma-owned-evidence.json).

## Storage, startup and completion

The staging buffer must be nonempty, no larger than the hardware's 65535-byte
count and wholly within the selected part's qualified SRAM range. Family
selections without qualified memory metadata are rejected rather than assigned
a guessed capacity. The driver owns staging permanently; there is no safe
staging accessor, split/reborrow, resource-return or recovery method. Each write
is divided into staging-sized chunks. Static RAM cost and one CPU copy per byte
are explicit; task scheduling and rearming can introduce gaps between chunks.

Normal reset or trustworthy clean-runtime entry remains the HAL platform
premise. The existing one-time DMA admission records a clean startup before
issuing a safe channel capability. This constructor uses that admission, not a
fresh register snapshot or a raw-to-safe channel conversion. Raw channel
acquisition permanently revokes safe eligibility. Startup observations can
reject dirty state but cannot establish that an arbitrary bootloader's prior
bus accesses drained. The raw PAC remains a technically unsafe ownership escape
within the project's existing chiptool model. See
[the startup boundary](dma-safe-owned-copy.md#supported-platform-boundary).

A chunk is copied while idle, then its persistent reservation is established
before DMA starts. The HAL programs the typed UART TDR endpoint, incrementing
SRAM source, fixed destination, equal byte widths, bounded count, REPEAT=1 and
hardware BLOCK request. It enables the channel before UART DMATX, following the
own manual's sequence. The selected-channel ISR records its result using the
existing critical-section/waker protocol and R1W0 flag-clear seed. TE takes
precedence over TC. Successful reuse requires recorded TC without TE and
STATUS=Complete with EN/TCIE/TEIE read back clear; the driver closes DMATX,
reads it back clear, and orders memory before copying the next chunk. Failed
channel readback reports unconfirmed completion; failed DMATX closure permanently
quarantines the lease. Neither path releases staging, pins or clocks.
SOFTSRC=0 is not positive completion evidence for a hardware request.

`write` completes after clean memory-to-TDR transfer. `flush` first settles any
retained DMA chunk, then waits for UART wire completion. UART TXBUSY=0 is the
serial idle condition, not a general DMA cancellation/drain acknowledgment.
Neither interrupt delivery nor successful completion has a finite-time promise.

## Cancellation, errors and teardown

The state belongs to the UART owner, not only to an async future's destructor:

- Dropping an unpolled write launches nothing.
- Cancelling or forgetting a polled write retains the current private chunk
  in flight. A prefix may already have been transmitted; the unscheduled
  remainder is not sent. The caller's input never became a DMA address.
- A later write or flush waits for the retained chunk's existing outcome.
  Clean completion allows reuse; a replacement cannot overwrite staging while
  that chunk remains active. An empty inherent async write also awaits the
  old chunk; empty async trait Write makes the existing nonblocking
  observation and returns zero or the current busy/error. This is
  normal-completion reaping, not abort.
- TE, TE+TC or inconsistent completion permanently quarantines the DMA lease.
  A later TC does not rehabilitate it. Transfer/flush calls return an error;
  no staging reuse, channel restart or safe resource return is available.
- Dropping an idle owner performs normal UART shutdown. Drop may first reap a
  currently observable clean TC with Complete status. Otherwise an owner with
  unconfirmed in-flight work or a quarantine retains the static staging,
  channel, UART/pin ownership and required clocks. UART waiter interrupts are
  masked; a combined owner's live RX peer keeps UART dispatch active. The UART
  DMA request is not disabled on this abandoned/error path. The retained DMA state can still
  record a terminal outcome. Resource and power retention is intentional.
  It does not promise early bus drain.

Blocking methods on the same owner make one nonblocking terminal observation,
servicing the selected channel's flags even if CPU interrupts are masked.
Pending work returns `Error::DmaBusy`; clean completion permits the operation.
Once idle, `blocking_write` performs ordinary CPU writes. The first observed
DMA failure returns `Error::Dma(error)`; subsequent transfer/flush calls return
`Error::DmaQuarantined`. No blocking entry bypasses the retained lease.
A cancelled flush after clean DMA completion has the
ordinary UART wire-wait semantics; it does not make already-reclaimed staging
unsafe. No destructor is required to keep separately borrowed caller memory
alive. Rust permits [forgotten futures](https://doc.rust-lang.org/core/mem/fn.forget.html),
so static private storage is the lifetime boundary even when cleanup is skipped.

This TX API introduces no CTS constructor, circular mode, borrowed zero-copy or
safe early-abort guarantee. On F030/A030/L083 and the fourteen exact classic
parts above, [staged RX and combined TX/RX](uart-dma-rx.md) extend
the same ownership model with per-direction live/retained state; an abandoned
half preserves its live peer and both retained halves preserve UART clocks. L083 UART1–6 use the same staged contract with independently reviewed own-family
request and terminal evidence; see [L083 qualification](l083-peripheral-dma.md).
The existing non-DMA UART constructors and low-level unsafe DMA APIs retain
their separate scope. DMA constructors prepare the UART with no RX ring;
there is no buffered-plus-DMA constructor or owner conversion for the same
UART. Stage83 buffered ownership and Stage82 native idle support retain
their independent scope; F020 gains no `read_until_idle`.

## Own-source evidence

L083 uses its own CN V2.0 manual and exact-part 24 KiB SRAM metadata, documented
in [the L083 source qualification](l083-peripheral-dma.md). The x030 sources and
page references below retain their original family scope. F020 uses its own
CN V1.4 manual, current CN V1.3 datasheet and SDK V1.2, bound separately in
[its evidence record](f020-dma-owned-evidence.json). In particular, own
printed p287/viewer p288 provides the byte SRAM-to-TDR setup, EN-before-DMATX
ordering and separate TC/TXBUSY completion sequence.

L031 CN V1.6, L052 CN V1.5, R031 CN V1.3 and W031 CN V1.4 are each
bound in [the four-channel evidence](four-channel-dma-owned-evidence.json).
Their own byte SRAM-to-TDR setup pages are printed 344/381/347/347;
their R1W0 ICR pages are 131/136/133/132. For these four manuals printed
pages equal zero-based PDF indices, and one-based viewer pages add one.
Each own UARTx chapter and UART1/2/3 base/request table is required; the
existing F020/x030/L083 records are not substituted.

The selected authoritative behavior source is the shared F030/A030
[CW32x030 User Manual CN V2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
cover date 2024-09 and revision-history date 2024-07-24. Its locked identities
are recorded in `sources/evidence-sources.json`:

- PDF SHA-256: `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`
- Extracted text SHA-256: `23935a54e976c25acfacd689818f272db88b579fc8f15363c2732c12da8773fe`

The retained selected revision was reviewed during the design pass. This change
does not claim a fresh vendor download or a latest-revision comparison. Raw
vendor documents and extracted text are not redistributed in the source tree.
For this PDF, the printed page equals the zero-based PDF index; a conventional
one-based viewer page number is one greater.

| Source location | Supported fact |
|---|---|
| §8.4.1, printed p127 / viewer p128 | Configure descriptors, width, count, increments and request before EN. |
| §8.4.4, printed pp130–132 / viewer pp131–133 | Each hardware BLOCK request transfers one block until the count completes; Table 8-2 supplies the UART request mapping. |
| §8.5, printed p134 / viewer p135 | Matching source/destination widths, count 1–65535 and REPEAT reloaded with 1. |
| §8.6, printed p135 / viewer p136 | TC denotes all data correctly transferred; TE includes address, stop-request and access errors. |
| §8.8.2, printed p138 / viewer p139 | DMA ICR uses R1W0 with reset/no-op seed 0xffffffff; only selected flags are cleared. |
| §8.8.3, printed p139 / viewer p140 | STATUS=101 is complete; STATUS=010 is stop-request abort. EN is enable/disable without a documented drain acknowledgment. |
| §18.6, printed p344 / viewer p345 | UART DMA handshakes until the count completes; DMA TC follows; TX shutdown additionally waits TXBUSY=0. |
| §18.7.1.5, printed p349 / viewer p350, steps 9, 12–25 | Official byte TX example enables the DMA channel, enables DMATX, waits TC and clears DMATX. |

The selected [F030 SDK V2.2](https://www.whxy.com/uploads/files/20241111/CW32F030_StandardPeripheralLib_V2.2.zip)
corroborates this setup: `Libraries/src/cw32f030_uart.c:450–475` controls CR2
DMATX/DMARX; `Libraries/src/cw32f030_dma.c:63–81` configures the channel and
`:119–133` sets/clears EN. Its DeInit at `:44–54` rewrites descriptors after
clearing EN without a drain wait, so it supplies no early-memory-release proof.
The STOPREQ status name at `Libraries/inc/cw32f030_dma.h:234` does not establish
that all outstanding memory accesses have drained. Neither EN-clear nor every
TE cause is treated as clean reclamation evidence.

Existing x030 source-qualified route and memory metadata remains unchanged:

- `cw32-data/dma/cw32f030.yaml` and `cw32a030.yaml` give UART2 TX request 3 on
  any of the five physical channels, with DMA_CH1 on DMACH1. The HAL consumes
  generated compatibility instead of copying selector numbers into the example.
- `cw32-data/af/cw32f030.yaml` and `cw32a030.yaml` give PA2 AF2 for UART2 TX;
  their package pinouts bond PA2 on the two C8T7 selections used by the example.
- `cw32-data/parts.yaml` provides exact-part memory capacities. SRAM accessibility
  and byte transfer evidence are retained in `dma-owned-copy-evidence.json`.
  Both example parts have 8 KiB SRAM; no new memory schema is needed.

Pinned Embassy revision `f16efeffe37581092ec184718e6fdb1620393214` supplies the
safe peripheral API/ownership comparison:
[UART constructor and write](https://github.com/embassy-rs/embassy/blob/f16efeffe37581092ec184718e6fdb1620393214/embassy-stm32/src/usart/mod.rs#L541)
and [its low-level DMA teardown](https://github.com/embassy-rs/embassy/blob/f16efeffe37581092ec184718e6fdb1620393214/embassy-stm32/src/dma/dma_bdma/mod.rs#L915).
STM32's reset/suspend behavior is not evidence for equivalent CW32 cancellation.

## Production verification boundary

[The real UART TX example](../examples/shared/uart-dma-tx/README.md) consumes a singleton
staging allocation, sends flash and ordinary local slices longer than staging,
and calls write/flush repeatedly. Its local metadata-derived linker script and
the exact x030/L083 package selections are included in `ci/check-hal.sh`. The linked ELF
must retain actual vectors, an entry/stack and nonempty firmware text; a compile
or empty image is not equivalent to a useful firmware link.

Affected x030/L083 builds, other no-DMA UART profiles and
the existing safe CopyChannel firmware remain regression scope. Source review
must also cover persistent state, error precedence, cancelled/forgotten writes,
all blocking/async/trait entry points, and teardown. Build receipts belong to
the exact source snapshot that was checked; the Stage84 outcomes below do
not establish a silicon run. No HAL test/harness, synthetic
register model, firmware execution, throughput or liveness result is supplied.

The [separate F020 firmware](../examples/f020/dma/README.md) selects UART1/PA8
and UART2/PA2 on C6/K6, UART2/PA6 and UART3/PA2 on F6, and both physical
DMA vectors. Stage84 main passed 15 actual ARM libraries and 16 ELF links;
all F020 rows and all ELF links were warning-free. Generation and five finite
source gates passed in documented segments, and independent runtime and
metadata/source reviews accepted the main work. At main acceptance clean
had not run; final-package completion requires a separate final-source
2-library/4-ELF clean receipt. See [the verification boundary](f020-dma-owned.md#verification-boundary)
for the full warning/failure history and limits. This scope does not add
F020 rows to the existing cumulative HAL recipe.

The [four-channel firmware](../examples/shared/four-channel-dma/README.md) selects the
31 TX endpoints above using private 32-byte stages, repeated flash/local input
longer than staging, and a separate wire flush. Stage85 main passed six
standalone ARM library builds and 21 ELF links, all 27 commands exiting zero
without warnings. This includes focused legacy UART RX/TX, SPI, F020 and
buffered-UART regressions. Generation, five finite source gates and the
capability renderer passed in documented segments; three source checker
preparation failures remain recorded in [the verification boundary](four-channel-dma-owned.md#verification-boundary).
At main acceptance clean had not run; final-package completion requires a
separate final-source 2-library/4-ELF clean receipt. The cumulative HAL recipe
is unchanged. No hardware, RF, throughput, error-drain or bounded-cancellation
result is claimed.

Stage92 RX/combined verification is separate from the Stage84/85 TX results
above. Its eight-library/21-ELF main passed without warnings, and both finite
source checks passed after the preserved initial D01 checker failure. Independent
runtime and metadata/source reviews accepted the bounded main result. At main acceptance final capability reports and
the separate final-source two-library/three-ELF clean replay had not run; see
[the Stage92 verification boundary](classic-uart-dma-rx.md#application-and-verification-boundary).
No hardware result is claimed.

## Native L012 owned DMA

Exactly CW32L012C8T6/C8U6 add owned static-SRAM copies and staged byte UART TX.
Each has 64 KiB Flash and 8 KiB SRAM; generic `cw32l012` retains safe admission
and staging rejection for missing memory metadata. Its native descriptor has
separate programmed CNT and current CCNT, and RESTART/SRCLOAD/DSTLOAD stay
zero. CH1/2 share DMACH12 IRQ9 and CH3/4 share DMACH34 IRQ10; IRQ11 is CORDIC.
UART1/2/3 use PA9/PA7/PA8 AF1 respectively on both packages. Only clean own
TC without TE plus STATUS5 permits request closure and staging reuse; software
copy additionally requires SOFTSRC0. Errors permanently retain resources,
forget leaks and Drop may wait indefinitely. Flush separately waits for TXBUSY0.

Native PAC SIZE/STATUS/TRANS/TYPE fields become typed enums; CCNT register
handles become read-only and CSR TC/TE setters are removed. STATUS stays RW.
Existing callers of these PAC fields must use the corrected types/access.
No DMA RX/combined UART, SPI, other hardware request or early-abort API is added.
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
See [the own-L012 contract](l012-dma-owned.md) and [ordinary firmware](../examples/l012/dma/README.md).
