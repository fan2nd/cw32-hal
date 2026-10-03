# v0.15.0 DMA and motor API validation

2026-10-03. Rust/Cargo 1.99.0; `thumbv6m-none-eabi`.
This release adds the [DMA driver](dma.md), [sourced DMA topology](dma-hardware-evidence.md)
and [separate motor operations](motor-api.md). The earlier [whole-chain audit](full-chain-audit-v0.14.0.md)
remains the baseline; this record describes the new work actually checked.

## Data and generated identities

- Both original manuals were re-read for channel banks, nine channel/IRQ mappings,
  all 64 L012 and 43 F030 hardware selectors, widths/count/arbitration/restart,
  W0C flags and the undocumented abort-drain boundary.
- Schema8 YAML → persisted JSON → standalone PAC metadata round-trip compared
  every channel/IRQ and all 107 request records exactly.
- 18 malformed inputs per chip, 36 total, were rejected: duplicated channel
  number/index/alias or selector/name, absent evidence/channels/requests, invalid
  IRQ/index/address/owner, unknown peripheral/key, selector64, invalid signal and
  obsolete schema7.
- Generated channel tokens exist only through a controller split; they are not
  independently exposed alongside the parent in `Peripherals`. Shared-controller
  reset validates that no separately owned peripheral is affected. Every channel
  state is quarantined as needed before a repeated controller split/reset.
- Request enum values, channel indices and IRQ types consume metadata, not chip
  or family names. Normalized generation and byte-for-byte drift checks pass.

## DMA behavior and ownership

The engine was exercised externally using its production source, including all
three widths, increment combinations, BLOCK/BULK, software/hardware trigger,
count/alignment/address-end rejection, startup order, memory fences, selected
W0C clearing, all error values, simultaneous TC+TE, and L012 restart configuration.

An independent complete-module harness uses production DMA module/engine/hardware,
actual generated PAC/identity bindings, `Peri` and `EventState`. It passed 23
lifecycle scenarios per chip, 46 total:

- Shared-IRQ peer flags and enables remain untouched.
- All eight status encodings with TE and simultaneous TC+TE retain error priority.
- Static buffers return only after clean TC; invalid start returns untouched
  buffers; timeout/cancel/error quarantine them.
- Cancellation versus an already completed transfer is resolved by observed TC.
- Forget → owner drop → re-split preserves poison; stale later TC cannot recover
  quarantined buffers or make a poisoned channel reusable.
- Reentrant waker clone/drop paths preserve state/borrow correctness.

Twelve expected ARM compiler failures verify local source/destination rejection
(including forgetting a transfer), destination reuse, second channel borrow,
missing unsafe raw-call boundary, unsupported u64 words, awaiting Blocking mode,
wrong IRQ binding, absent F030 restart/request, independent channel acquisition
and duplicate token use. Positive async/static-copy/raw/repeating clients compile.
These probes do not establish physical bus-drain behavior.

## Motor migration and original behavior

The independent production-source MMIO harness passes 152 cases: 128 exact
old/new bridge write traces and 24 ADC reserved-bit/EOS, PWM setup/fault, analog,
timer and fatal-disconnect checks.
Two migration mistakes were found and corrected before release: EOS-only
acknowledgment now preserves both watchdog flags, and OPA configuration retains
the audited BIAS=7 reset seed (0xe220 → 0xe221). Shutdown also clears AOE with MOE
to prevent auto-rearm from an inherited enable setting; the established board
configuration already held AOE=0. All three preexisting break flags, fault arrivals
around the MOE write/check, and hardware MOE loss without a visible flag were
checked; refusals disable output while preserving fault latches.

Nine pure board algorithm files are byte-identical to v0.14.0: six-step, overzero,
control, protection, protocol and frame queue. Wiring, timing constants and
control/IRQ ordering remain in the examples. This is source/order parity, not a
claim of measured identical machine-cycle latency.

02–05 have zero `pac::` source lines and no `unstable-pac` dependency. 06 changes
from 149 such lines to ten, all in UI UART1 access; main and motor paths have none.
01 remains ordinary safe GPIO. The P1 interrupt motor executor, normal thread UI,
static raw-volatile live DMA scan and explicit power-output opt-in are preserved.

## Builds and linked executables

- Formatting and regeneration/drift checks pass.
- Both PACs build ARM release with runtime and metadata.
- Both HALs build minimum and full valid ARM release feature sets; each of seven
  additional features also checks individually with each chip, 14 combinations.
- Both HALs pass minimum/full rustdoc with warnings denied.
- All six independent board binaries link for ARM release, including individual
  package builds without accidental feature unification; 05/06 also link with
  `motor-output-enable`.
- Additional DMA executables link for both chips with their actual shared DMA
  handlers and GTIM1 time handler. ELF32 ARM, all 32 external vector slots against
  generated metadata, non-default DMA/GTIM1 symbols, stack and Flash/RAM bounds
  are checked.

## Clean source package

A fresh source-only archive extraction starts without generated data/PAC,
Cargo.lock or target output. Regeneration with a new Cargo target directory and
the complete release, strict-documentation and example matrix passed. Final
source and generated output match the maintained tree byte-for-byte. After the
final fault-refusal hardening, the complete matrix was repeated on the exact
updated extracted source before packaging.

## Limits

No board was flashed, no motor was powered, and no physical DMA transfers,
waveforms, arbitration latency, ADC accuracy or worst-case IRQ timing were tested.
The safe DMA cancellation quarantine deliberately reflects missing vendor drain
evidence. Peripheral DMA endpoints and L012 repeating scans remain explicit
unsafe contracts, not safe ring buffers. F030 motor register compatibility,
UART/SPI/I2C drivers and full STM32 feature parity are not claimed.

All focused probes live outside the source repository/package. No tests, Python
project, host example target or extra target cfg is a build prerequisite. The
existing proc-macro-error2 future-compatibility notice remains; it is not a project
compile failure. No Clippy result is claimed.
