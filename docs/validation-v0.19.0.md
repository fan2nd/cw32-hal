# v0.19.0 validation

Validation used Rust/Cargo/rustfmt 1.99.0 and the real `thumbv6m-none-eabi`
target. New HAL coverage is [UART/SPI/I2C, CRC and non-window IWDT](buses-v0.19.0.md).
No board was flashed or run. Host register/protocol models check software
ordering and ownership against documented behavior; they do not measure silicon
timing, oscillator tolerance, electrical compliance or undocumented side effects.

## Build and source checks

- Formatting and actual YAML → persisted JSON → re-read JSON → PAC regeneration
  and drift checks pass.
- Both chips' PAC release builds include runtime and metadata.
- Both chips' HAL minimum/no-default and complete supported-feature release
  builds pass. Minimum and complete rustdocs pass with warnings denied.
- Fourteen separate optional-feature checks pass: `rt`, `memory-x`, `metadata`,
  `unstable-pac`, `defmt`, `time-driver-gtim1` and `time-driver-any`, independently
  for each chip. Selecting both chips together is intentionally not a valid
  `--all-features` build.
- All six board binaries link in release mode, including the separate 05/06
  `motor-output-enable` builds. The existing release profile retains debug
  information while optimizing for the 64 KiB Flash target. Unoptimized full
  application linking is not a supported firmware-size claim.
- Both dedicated bus integration ELFs link actual UART split/async, 16-bit SPI,
  I2C write/read, CRC and IWDT code with the GTIM1 time driver. Every one of the
  32 external vector entries matches chip metadata. UART1/2/3, SPI1 and SPI23
  (L012) or SPI2 (F030), I2C1/2 and GTIM1 have real handler symbols; initial stack
  and all allocated sections fit the chip's Flash/RAM map.

The only build notice is the existing upstream `proc-macro-error2` future Rust
compatibility notice. New HAL source emits no compiler or rustdoc warnings.

The 259-file source-only ZIP was extracted into an empty directory without
generated data/PAC, Cargo.lock or a build target. The complete release/docs/six
examples plus motor-opt-in matrix passed there using a fresh target. All 48
normalized JSON and 102 generated PAC/metadata/runtime files match the maintained
tree. Final documentation was synchronized after recording these results; every
archive source file matches both trees byte-for-byte. No generated output,
test scaffolding, Python project, build cache or log is included in the ZIP.

## Independent hardware and protocol review

The audit pins Embassy to `b12a6d9efcd2711037abca1b63a661a9ef726444`, L012 RM1.4
and F030 RM2.5. Each driver document links its actual source sections.

- All 133 added L012 routes match both manual Table9-2 and SDK1.0.5 macros.
  All 116 F030 routes match visually inspected manual tables; SPI and I2C
  identities were additionally checked against their peripheral route tables.
- UART register probes cover 24 valid frame/source configurations per IP and
  reject 12 invalid nine-bit-plus-parity combinations. They check source field
  placement, selective RC/error acknowledgement, independent TX/RX masks and
  the existing application's exact 96 MHz / 833 divisor.
- SPI backend probes cover four CPOL/CPHA modes, two frame widths and both bit
  orders, plus documented divisor boundaries. IRQ/common-source probes cover
  pending masks, one-shot wake/latch behavior, receive errors, BUSY drain,
  abort commands and stale interrupts after clock release.
- I2C has 450 timing combinations per IP: 273 accepted / 177 rejected on L012,
  330 accepted / 120 rejected on F030. L012 checks the documented setup/hold,
  prescaler/filter and valid-data constraints. F030's documented total-period
  divider and the driver's conservative symmetric-phase model are checked;
  the reference manual does not establish measured high/low duty or board
  electrical margins.
- I2C protocol probes exercise reads of 1, 2, 255, 256, 257, 511, 512, 513 and
  1025 bytes, with service delays of 1, 7 and 97 model steps. Adjacent and mixed
  read/write buffers, empty writes, repeated START, final NACK and arbitration
  abort without STOP pass. The one-entry L012 FIFO model checks prequeued
  receive-command chaining across 256-byte boundaries.
- CRC sentinel probes verify L012's word-wide access with an eight-bit datum,
  F030's exact 8/16/32-bit feed aliases and 16/32-bit result reads, and every
  algorithm encoding. L012 compile probes reject CRC32 and wide feed methods.
- IWDT protocol probes cover constructor no-start/no-reset, inherited running
  state, key/configuration order, update waits, reset-only mode, bounded timeout
  relocking, partial-start feed rejection, retained running clock and explicit
  stop before normal clock release.

## Ownership, cancellation and application compatibility

External API probes compile real constructors, operation modes and standard
SPI/I2C blocking/async traits for both chips. Intended compile failures cover
wrong pin routes, wrong or missing interrupt bindings, asynchronous APIs on
blocking owners, unsupported SPI words and reuse of live peripheral/pin/buffer
borrows: 12 UART, 14 SPI and 10 I2C cases, 36 in total. SPI/I2C owner and
live-future `Send` checks also pass. UART split halves retain the borrowed peripheral and independent
clock/pin ownership.

Actual-HAL mapped-register probes exercise UART polling, IRQ wake/cancel,
TXBUSY flush, errors, forgotten waits and sibling clock/mask isolation. Bus
handlers retain no pointers into application buffers. Additional whole-driver
SPI/I2C probes exercise cancelled and forgotten operations before rearm or
owner destruction; resource cleanup precedes pin/clock release. SPI covers
15 whole-driver scenarios per chip, 30 in total.

Compared with the frozen v0.18 source, family/clock/DMA metadata is identical.
The only register-model changes are the two CRC MODE semantic enums; addresses,
widths, access, reset and side-effect contracts remain unchanged. All motor HAL
files and 22 of 23 example Rust files are byte-identical. Only example06's UI
file changes to consume the typed UART owner. It retains PCLK_ALT, BRRI52,
BRRF1, 8N1, PB12/PB11, both directions enabled, no UART1 IRQ, the original frame
queue and at most one transmitted byte per UI wake. UART2's P1 motor executor,
ADC/commutation/fault handlers and original control algorithms are unchanged.
All six examples now have zero direct PAC accesses and no `unstable-pac`
dependency.

Safe bus DMA, buffered serial, target modes, WWDT, Flash/RTC, internal ADC,
capture/encoder and external/low-power clocks remain explicitly tracked in the
[subsequent stages](buses-v0.19.0.md), rather than being inferred from a passing
compile or complete register data.
