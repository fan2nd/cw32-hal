# Reserved-region FLASH storage: thirteen qualified controller families

## Status and scope

This batch implements a blocking byte-program/page-erase engine for reviewed
F002/F003/F020/F030/A030/L010/L011/L012/L031/L052/L083/R031/W031 parts. It is source-qualified and
software-built, not silicon-qualified. No device was flashed, emulated or
accessed. The independent original-source review is in
[flash-breadth-source-review.md](flash-breadth-source-review.md); the current
flattened-path integration is reviewed in
[flash-breadth-flat-review.md](flash-breadth-flat-review.md).

There are 37 qualified exact ordering codes: the earlier 25 plus twelve
L010/L011/L012/L083 parts. This extension is described and source-pinned in
[flash-remaining-evidence.json](flash-remaining-evidence.json); independent identity review is pending. Their main arrays
begin at address zero and use 512-byte erase pages and byte programming.
Capacity is emitted from reviewed selected-part metadata and checked against an
explicit exact-part allowlist, never inferred from a suffix or SDK bound.

| Family | Exact main-array size | Temporary protection group | Legal lock mask | Cache |
|---|---:|---:|---:|---|
| F002 | 16 KiB | 2 KiB | 0x00ff | none |
| F003 | 20 KiB | 2 KiB | 0x03ff | none |
| F020 | 32 KiB | 4 KiB | 0x00ff | FETCH/CACHE |
| F030 | 32 or 64 KiB | 4 KiB | 0xffff | FETCH/CACHE |
| A030 | 64 KiB | 4 KiB | 0xffff | FETCH/CACHE |
| L031/L052/R031/W031 | 64 KiB | 4 KiB | 0xffff | none |
| L010/L011 | 64 KiB | 4 KiB | 0xffff | none |
| L012 | 64 KiB | 4 KiB | 0xffff | FETCH/CACHE plus explicit invalidate |
| L083 | 128 or 256 KiB | 4 KiB | 0xffffffffffffffff across four registers | none |

Generic profiles lack memory metadata. The four existing F030 package-neutral
aliases have verified memory sizes, but are not qualified for this storage API.
Both report `FLASH_SIZE = None` and reject reservation construction with
`UnknownCapacity`; for aliases this is an API-qualification limitation rather
than an assertion that their physical capacity is unknown. Existing compatibility
memory metadata is unchanged. L010/L011/L012 poll ISR.BUSY and always exclude
0xFE00..0x10000, the complete page containing the secure-library descriptor.
Construction and each operation reject overlap with any active SDKCFR interval.
No descriptor, password, security-level or option-byte API is provided. L083 uses
u32 addresses and a u64 lock snapshot so page 255/256/511 and all four protection
registers are represented; these registers do not imply independently operable banks.

## Deliberate `NorFlash` acceptance barrier

`embedded-storage` 0.3.2 `NorFlash::write` requires that power loss leave the rest
of the page unchanged. The audited own CW32 FLASH chapters and electrical tables
do not establish that guarantee. Therefore this batch implements `ReadNorFlash`
and explicit `blocking_write` / `blocking_erase`, but **does not implement
`NorFlash` or `MultiwriteNorFlash`**. The trait implementations are deliberately absent.
A valid-voltage unsafe constructor is not treated as a waiver of a trait promise.

Programs, erased prefixes, and neighboring data after supply loss/reset remain
unqualified. Generic crash-consistent storage integrations requiring `NorFlash`
are blocked pending authoritative vendor evidence or a separately justified
implementation. This updates the earlier audit's proposed trait surface without
weakening the actual engine or claiming a property absent from the sources.

## Ownership and use

`Flash<'d, Blocking>` owns an actual `Peri<'d, peripherals::FLASH>`, a non-copyable
`ReservedRegion<'d>`, and a non-default `OperatingConditions`. Public operations
use partition-relative offsets, and capacity is the reserved partition length.

An unsafe `ReservedRegion::new(start..end)` asserts exclusive linker/application
ownership of every byte for the driver's lifetime: no code, vectors, literals,
live immutable data/reference, DMA reader/writer, overlapping storage owner, or
other FLASH programming agent may use it. Owning the peripheral or choosing the
last page alone proves none of this. Region and erase endpoints must be page
aligned; byte writes and reads can be unaligned.

An unsafe `OperatingConditions::new(supply_mv, max_hclk_hz)`
asserts measured/validated board bounds throughout use, including oscillator
tolerance and supply transients, with own-datasheet temperature/power conditions.
It checks these numeric limits:

- L010: 1.62–5.5 V; below 1.8 V at most 24 MHz, otherwise at most 48 MHz.
- L011/L012: 1.7–5.5 V; below 1.8 V at most 24 MHz, otherwise at most 96 MHz.
- F030/A030/L083: 1.65–5.5 V; at most 24 MHz below 1.8 V, otherwise at most 64 MHz.
- F002/F003/F020/L031/L052: 1.65–5.5 V; at most 24 MHz when the supply
  voltage is below 1.8 V, otherwise at most 48 MHz.
- R031: 2.2–3.6 V, at most 48 MHz.
- W031: conservatively 2.0–3.6 V, at most 48 MHz. This supports both RF LDO and
  DCDC supply modes; this initial API does not qualify the LDO-only 1.8-V case.

`OperatingConditions::from_rcc(board, clocks)` accepts the RCC board declaration
and obtains the outward-rounded `clocks.hclk_bounds().maximum()`. It checks the
qualified temperature interval and the same supply/HCLK limits. It remains
unsafe: declarations and calculated bounds do not establish actual conditions.
W031 retains the conservative 2.0-V lower bound even if RCC permits 1.8-V LDO.

Existing WAIT must support the actual maximum HCLK bound, including tolerance:
WAIT0/1/2 permit 24/48/72 MHz, subject to the lower device/supply limits above.
L010 permits WAIT0–1; L011/L012 permit WAIT0–3 (24/48/72/96 MHz).
Other families reject WAIT3–7. F030/A030 above 48 MHz therefore require WAIT2. WAIT,
reset, security level, option words and STANDBY are never reconfigured. On
x030/F020/L012, keyed CR2 writes temporarily disable FETCH/CACHE, then restore
both saved enable bits after verified Read-mode restoration. Every such write
retains the originally captured WAIT; its readback is checked as well.
FLASH error IRQ enables must be clear before use; the FLASHRAM shared IRQ and
its RAM flags are never acquired or changed. Construction enables and retains
the configuration clock, preserving other AHB gates. Drop has no hardware effect.

A linker reservation may, for example, shorten the executable FLASH region to
60 KiB and define a separate 4-KiB range:

```ld
MEMORY {
  FLASH : ORIGIN = 0x00000000, LENGTH = 60K
  STORAGE : ORIGIN = 0x0000F000, LENGTH = 4K
  RAM : ORIGIN = 0x20000000, LENGTH = 8K
}
__storage_start = ORIGIN(STORAGE);
__storage_end = ORIGIN(STORAGE) + LENGTH(STORAGE);
```

The application still must inspect its actual linker map, load image, references,
DMA usage, bootloader policy, and board conditions before making either unsafe
assertion. Linker symbols and RAM capacity do not qualify a real board.
The real [storage example](../examples/shared/flash-storage/README.md) derives its
reservation from each exact part's memory metadata and demonstrates this linker
boundary. It is destructive firmware and must not be run without the board and
partition review. The earlier 25 selected examples were built and their ELF boundaries
inspected; the extension receipt separately records new builds and links; compilation/linking is not hardware validation.

## Operation and failure behavior

1. Validate range, alignment, and checked arithmetic before any unlock.
2. Check active-low reset, gate, BUSY, error IRQ enables, and WAIT.
3. Replace MODE with Read and preflight **every** program byte for 0xff. A
   non-erased byte rejects the whole request before the first program strobe.
4. Per byte/page, capture only documented lock bits; temporarily unlock only the
   affected 2-KiB or 4-KiB group while preserving unrelated legal groups. For
   x030/F020/L012, disable FETCH/CACHE with the original WAIT and verify readback
   before triggering. On L012 only, software additionally invalidates cache using a verified 1-then-0 cycle after returning to Read and before restoring the cache bits. This is a conservative implementation choice, not a mandatory step claimed from the operation sequence.
5. Clear every implemented error with typed ICR fields, using the documented reset seed. The resulting write is **0x0C** normally, **0x08** on L011, and **0x00** on L012. Those fields are
   W0C, while remaining reserved low bits retain their documented default ones.
   Neither ICR read-modify-write nor W1C semantics is used.
6. Write KEY=0x5A5A and exact MODE 1 or 2, keeping STANDBY. Never OR stale MODE
   into the requested value: 3 means chip erase, or is invalid on x030/F020.
7. Trigger an 8-bit store, wait until BUSY actually clears, synchronize, capture
   every implemented error bit, explicitly return to Read, and restore the saved
   lock state. Then restore the saved FETCH/CACHE bits with the original WAIT,
   but only after Read mode is confirmed. Report cleanup failures, retaining
   hardware error flags. Read back completed writes/erases.

Zero-based FLASH is accessed with ARM `ldrb` / `strb` using integer addresses.
No Rust reference/slice, including a null reference at zero, is created. The
production hardware constructor rejects a non-ARM target before constructing
usable hardware handles.

A pre-existing busy operation returns `Busy` without starting another operation.
Once a byte/page strobe occurs, polling is intentionally unbounded. CPU fetch
from FLASH stalls while busy, there is no completion IRQ, all available timings
are typical, and no safe abort/reset sequence is documented. The per-operation
critical section includes completion polling; validate IRQ/watchdog latency.
There is no poll-count timeout, async future, or cancellation/drop recovery.

Hardware error snapshots preserve simultaneous PC/PAGELOCK/PROG, L011/L012 SDKERR and L012 CACHEON flags. An error
can leave a completed prefix. If returning MODE to Read, restoring page locks or
restoring FETCH/CACHE and original WAIT fails, `Error::Cleanup` takes precedence
and reports all three restoration results
plus the captured hardware flags; it never presents failed cleanup as a normal
completed error. Inspect controller state before reuse. Nothing promises recovery
from power loss, stuck BUSY, controller reset, or an external writer.

## Own-source evidence and contradictions

The pinned filenames, hashes, exact PDF pages, and official URLs are in
[`flash-next-batch-audit.json`](flash-next-batch-audit.json). This implementation
uses each family's own FLASH chapter (§7), clock gate/reset chapter (§4), and
current data sheet:

- Extension families: see the independent source review for own F002/F003/F020,
  shared x030 and own L052 manual/data-sheet pages and hashes.
- L031 manual CN V1.6, PDF 107–119; data sheet CN V1.9, FLASH table PDF 48.
- R031 manual CN V1.3, PDF 109–121; data sheet CN V1.2, FLASH table PDF 55.
- W031 manual CN V1.4, PDF 108–120; data sheet CN V1.3, FLASH table PDF 54.

Own SDKs corroborate byte strobes but are not copied as geometry or safety proof.
Their headers accept pages through 511 although the array contains only 128;
operations OR a saved MODE into Program/PageErase, potentially selecting
ChipErase; and their status helpers/sequencing cannot establish safe timeouts.
The implementation explicitly avoids these patterns.

An additional review detail is material: clearing only error fields means writing
0x0C, not all-zero ICR, because own §7.9.6 requires reserved bits to remain at
reset values. The typed driver retains that semantic in its seed and field writes. On L011 only bit 3 remains reserved; L012 implements all five low error bits.

Upstream API comparison uses Embassy commit
`f16efeffe37581092ec184718e6fdb1620393214`, `embassy-stm32/src/flash/mod.rs` and
`common.rs`: the module, `Flash`, explicit `Blocking`, retained `Peri`, blocking
method names, and error-kind mapping are retained; STM32 whole-array ownership,
null-incompatible slice reads, u8 region indices, and multiwrite assumptions are
not inherited.

## Validation boundary

HAL test assets and test-only MMIO adapters have been removed at the user's
request. This batch uses original-source/provenance checks, normal ARM target
builds, independent production review, and the real example's linked ELF
inspection. No new HAL unit/integration/compile-negative harness is shipped.
Existing data/PAC/source tooling remains separate. Historic results in earlier
stage archives are not claimed as a current retained test suite.

Before deployment, board-authorized checks must establish supply/temperature,
HCLK including tolerance, exact linker/ownership exclusion, watchdog and IRQ
latency, cache behavior and preservation of neighboring pages. Reset/power-loss
recovery, guaranteed completion time and safe abort are not established.

## L01x and L083 electrical qualification

L01x data sheets have no separate Vprog row. Their own general working conditions
and FLASH characterization tables are used together, rather than borrowing the
legacy 1.65-V minimum. The qualified temperature interval remains -40..85 C.
Own manuals document HCLK, WAIT and no separate program-clock divider in the
program/erase sequences. The driver enforces the actual HCLK upper bound against
both own-device supply limits and the existing WAIT encoding for every operation;
it makes no claim that a nominal maximum clock leaves oscillator-tolerance margin.
No additional undocumented clock divider is programmed.

The formerly raw single-implementation FLASH backend is removed. One flat
flash.rs leaf owns Peri<FLASH>, accesses typed PAC fields and source-backed MODE
enums directly, and uses central RCC_INFO without a controller reset. A u64
software lock snapshot is the only metadata width extension; the independent
schema implementation and its historical ancestry are retained. Schema and exact
output identity review remain pending; compilation is not an acceptance review.
