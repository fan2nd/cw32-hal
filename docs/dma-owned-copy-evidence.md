# x030/L083 typed DMA and owned software copies

This records the underlying unsafe-entry transfer implementation. The later
init-admitted safe `CopyChannel::copy` layer and its current compatibility notes
are specified in [safe owned copies](dma-safe-owned-copy.md). The current real
examples use that safe entry; the hardware evidence and recovery conditions
below still apply. References below to `OwnedCopy::new` describe the retained
unsafe raw constructor, not the additional safe capability.

This batch removes the one-implementation `Registers`/`Hardware`/`Engine`
adapter from `dma.rs`. Channel accesses use the selected typed PAC directly;
`Transfer` itself holds its channel index and lifetime. RCC uses central
`RCC_INFO`; channel identities and request routes continue to come from generated
selected-chip metadata. The L083 extension shares the qualified native IP for
software SRAM copies only. No register backend, harness, mock or borrowed
peripheral-DMA wrapper is introduced.

## Documentary scope

Exact official URLs, printed revisions, revision dates, document and text SHA-256s,
page indices, enum values and SRAM facts are in `dma-owned-copy-evidence.json`.
Original vendor documents/SDKs remain outside the distributable tree.

The common CW32F030/CW32A030 manual is EN V1.0 and CN V2.5. CN §§8.4.1–2
pp127–128 describe software BLOCK setup and completion; §§8.5.1–3 p134 require
matching 8/16/32-bit widths, CNT=1..65535 and reloading REPEAT=1 before each
transfer. EN corresponding pages are 134–135 and 141. The implementation enables
both address increments for owned copies, programs addresses/count/trigger while
idle, publishes Running, orders memory, enables the channel, then writes SOFTSRC.

CN §8.6 p135 and EN §8.6 p142 define TC as all data transferred correctly. TE
lists address errors, stop requests, source access errors and destination access
errors. Neither text documents draining outstanding bus accesses on TE or EN=0.
CSR.STATUS in CN §8.8.3 p139 / EN p146 is 0 initial, 1 addressing error, 2 stopped,
3 source error, 4 destination error, 5 complete. It is not 0 on completion.
SOFTSRC read 0 means completed and read 1 means running; write 0 has no effect
(CN §8.8.4 p141 / EN p148). These are the recovery conditions, not EN readback.

The shared `dmachannel_v1` PAC schema gains enums for TRANS, SIZE, STATUS and
TYPE. Each other canonical user was independently checked against its OWN
manual: F020 CN V1.4 pp136/138, L031 CN V1.6 pp132/133, L052 CN V1.5 pp137/139,
L083 CN V2.0 pp147/149, R031 CN V1.3 pp134/135 and W031 CN V1.4 pp133/134.
All have matching values. TRANS=0 is BULK, not SINGLE. Reserved encodings remain
reserved. This source qualification is not a new HAL port or hardware claim.

`DMA.ICR` is R1W0, reset 0xffffffff. The generated `write_noop()` seed comes from
CN x030 §8.8.2 p138 / EN p145 and independently from L083 §8.8.2 p146, the other
user of `dma_v1`. A direct typed write sets only selected TC/TE fields to false;
all other channel flags and reserved bits remain 1. Shared vectors never undergo
blanket NVIC unpending or shared-clock/controller reset.

## Hardware-exclusivity entry contract

`OwnedCopy::new` is explicitly unsafe. The public PAC exposes global register
handles with safe write methods, so the type-level channel token cannot enforce
hardware exclusivity against arbitrary code using that PAC. This batch does not
change that repository-wide interface and makes no unconditional safe-API claim.

The channel must already be quiescent on entry, without outstanding accesses
to either buffer from an earlier transfer or another bus master.
The caller must prevent other code, interrupts, debuggers and bus masters from
reconfiguring the selected channel, altering its status/clear/trigger fields or
resetting/disabling its controller or clock. No other agent may access the
destination or mutate the source while DMA might access them. Other channels are
allowed only with disjoint buffers and isolated register/flag operations, leaving
the shared controller and clock configuration intact.

Hardware exclusivity must hold before and throughout construction and, when
construction succeeds, until clean completion returns all owners. It also
persists on forget, Drop without owner recovery and
error quarantine unless hardware quiescence is independently established. TE and
EN clearing do not establish it. No quarantined-owner recovery API is provided.
Constructor validation errors start no transfer and return all owners.

The original firmware example justified each unsafe construction block. The
current example uses the later safe capability, with separate singleton buffers
and real bound HAL DMA handlers; an error still halts without reclaiming the
resources. The retained raw constructor continues to require the contract above.

## Ownership and recovery under that contract

`OwnedCopy::new` moves `Channel<'static>` and two `&'static mut [W]` into the
future. W is sealed to u8/u16/u32. Equal, nonempty lengths are checked, followed
by aligned/nonwrapping addresses and entire-byte-range inclusion in selected
SRAM. The unique references already imply disjointness in sound safe Rust; an
explicit overlap check also rejects address overlap. Validation and Busy errors
return `CopyConfigError` containing all owners before hardware starts.

Common manual §2.3 p28 / CN §§6.1–6.3 pp105–106 / EN pp108–109 establish SRAM
DMA accessibility and 8/16/32-bit aligned accesses. The exact part metadata is
used instead of assuming the common manual's generic 8 KiB: F030F6P7 has 6 KiB,
while F030 F8/K8/C8 and A030C8T7 have 8 KiB. These capacities come from F030 DS
CN V1.9 table 3-1 p7 and A030 DS CN V1.1 tables 3-1/6-1 pp7/28. Family profiles
without a memory map fail with `MissingMemoryMetadata`; no guessed extent is used.

The resources are exposed again only when the recorded IRQ outcome is clean TC
(no TE), CSR.STATUS=5 and SOFTSRC=0. The extra readbacks may conservatively reject
an inconsistent indication, sacrificing liveness/resources rather than releasing
memory. TC+TE always takes the error path. Repeated poll after completion panics
without owning or exposing any prior resources.

On every error or inconsistent completion, the future forgets both the resources
and the low-level `Transfer`. It therefore never resets the software reservation
to Idle. Hardware might still access those static buffers; the owned API provides
no path to recover either reference or that channel token. This quarantine
preserves the resources subject to the unsafe entry contract above.

Forgetting the whole running future also leaks all owners and its reservation.
Its addresses continue to designate valid exclusive static storage. A reborrow
cannot defeat this: to satisfy `Channel<'static>` the reborrow itself must last
forever, so safe code cannot regain the original owner. Completion after forget
may leave Finished state, which still fails Busy; it never restores Idle.

Drop waits for TC/TE through direct flag service even if CPU interrupts are
masked. It never first disables EN as a cancellation mechanism. After clean TC
it discards the recovered static owners; their allocations are not reclaimed.
On error it permanently reserves everything. Hardware starvation/failure can
prevent any TC/TE and make Drop hang forever. There is no finite timeout, early
abort, borrowed safe DMA or peripheral async API in this batch.

The existing unsafe `Transfer` API retains its TC/TE return/Drop protocol. Its
safety docs now explicitly say that TE does not by itself establish quiescence;
callers must independently ensure it before reusing borrowed memory/channel.
It is not the owned API's stricter clean-completion recovery condition.

## Ordering and interrupts

A SeqCst fence precedes DMA enable/start so compiler and CPU ordering publish
source data before device access. After observed flags, another SeqCst fence
precedes Finished publication. Async Ready uses an Acquire fence; successful
owned recovery and transfer Drop fence again before exposing resources/reusing
state. These fences order accesses; none claims to force peripheral completion.
They follow the role of fences in pinned Embassy
`f16efeffe37581092ec184718e6fdb1620393214`,
`embassy-stm32/src/dma/dma_bdma/mod.rs`; STM32's reset/cancel behavior is not copied.

Result observation and waker registration occur under the same critical section
as the per-channel ISR state transition, covering completion before the first
poll and during registration. Wakers are taken inside and invoked outside that
section. Each handler reads/clears only its channel's typed flags; unselected
channels on DMACH23/DMACH45 keep their flags. State remains reserved from start
through successful owned handback, or forever on a leaked/error copy.

## Validation limits

The evidence validator checks own source bytes, actual register-table values,
generated field enums, no-op seed, whole-IR reuse fingerprints and selected-part
SRAM maps. Normal x030 ARM builds and the `dma_owned_copy` firmware link exercise
the public API with u8/u16/u32, blocking completion, concurrent channels 2/3,
shared real IRQ bindings, returned resources and channel reuse. No HAL tests,
harness, fake register implementation, silicon run, flashing or throughput claim.
Exact commands and exit statuses are retained with the handoff receipts.

## Bounded L083 extension

L083 is independently qualified by its own CN V2.0 manual, not by the x030
layout. Sections 8.4.1–2 pp136–138 establish software BLOCK setup, EN then
SOFTSRC, and completion after the final transfer; §8.5 p142 covers equal widths,
CNT=1..65535 and REPEAT=1. Section 8.6 p143 defines TC as all data transferred
correctly and enumerates address, stop, source and destination errors. STATUS=5
in §8.8.3 p147 and SOFTSRC=0 in §8.8.4 p149 corroborate successful completion.
The p138 diagram does not show an AHB response/flush acknowledgment. TE and EN=0
still provide no outstanding-access drain guarantee. The existing unsafe entry,
clean-completion handback and permanent error/forget reservation are unchanged.

L083 §2.3 p32 and §§6.1–3 pp114–115 establish SRAM access and alignment. Its own
DS CN V1.9 pp9/39 establishes 24 KiB SRAM for all five exact ordering codes:
MCT6, RBT6, RCS6, RCT6 and VCT6. Those facts already flow from `parts.yaml` into
selected metadata; the HAL does not infer capacity from family or part spelling.
`CW32L083` itself has no memory extent and returns `MissingMemoryMetadata` before
starting an owned copy. The common SRAM base is 0x20000000; every L083 exact-part
limit is 0x20006000. The build owner validates `dma_v1`, all five native
`dmachannel_v1` instances, ordered channel metadata and real IRQ associations.
State count is generated from that metadata. Native typed register dispatch,
shared-vector flag isolation and the typed `write_noop()` seed are preserved.

L083 §§4.7.12/4.7.15 pp85/89 independently qualify shared AHBEN.DMA enable and
active-low AHBRST.DMA reset. Existing central RCC controls remain the only gate
owner. No completion, error or Drop resets or disables the shared controller.

The original software-copy qualification did not expose L083 request routes.
The later [own-family staged qualification](l083-peripheral-dma.md) adds only
UART1–6/SPI1–2 RX/TX constants and routes; `Transfer::new_read/new_write`
remain x030-only. `Transfer::new_copy` remains unsafe; its terminal/error protocol
requires callers to establish quiescence independently before memory reuse on
error. No additional L083 DMA integration is inferred merely because
51 request routes exist in the authored hardware metadata.

`examples/l083/dma` is actual firmware for all five exact parts. It uses u16
blocking copy and concurrent disjoint u8/u32 copies on channels 2/3, binds the
real shared interrupt, compares returned buffers and reuses the owners. PB0 is
bonded on each selected package and only serves as a completion indicator;
SRAM copies require no peripheral pin. The normal build matrix links each
firmware and all generic/exact L083 libraries, plus affected x030 library and
existing firmware regressions. These are compile/link results only, without
HAL tests, fake registers, silicon execution or a throughput/liveness claim.
