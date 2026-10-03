# v0.18.0 clock, PWM and PAC validation

2026-10-03. Rust/Cargo 1.99.0, `thumbv6m-none-eabi`.
Scope and remaining features are in the [stage record](clock-pwm-pac-v0.18.0.md).
No physical device was programmed or measured.

## Independent contract and behavior review

- All 512 L012 and 288 F030 documented HSI/AHB/APB combinations passed nominal
  arithmetic, manual encoding, Flash/bus/HSI ordering and unrelated-bit checks.
  Readback failures stop dependent writes. Clock-guard probes cover retain/drop,
  forgotten references, permanent pins, suppressed/exclusive reset and overflow
  rejection before MMIO.
- Both-chip actual-HAL probes with synthetic mapped registers passed independent
  PWM owner behavior, shared MOE, final counter/gate release, retained forgotten
  owners, full-scale whole-owner transitions, unchanged rejected split, empty
  split cleanup, DAC split references and DMA poison. Each chip additionally
  passed 8,000 concurrent sibling enable/disable/duty iterations.
- 30 ARM API outcomes passed: six valid programs and 24 intended rejections.
  They check independent `Send + 'static` PWM drivers, non-cloneable ownership,
  static split requirements, inaccessible shared counter/frequency changes,
  fixed-kernel versus bus queries, and sealed clock traits. UART kernel queries
  reject on both chips; L012 I2C kernel queries reject; F030 I2C queries compile.
- Unequal HCLK/PCLK runtime setups verified HCLK sources separately from PCLK
  sources without an STM32-style timer multiplier.
- The review found and fixed forgotten F030 comparator-brake clock retention,
  GTIM1 profile rejection before RCC writes, and incorrect fixed-PCLK metadata
  for L012 I2C. F030 Flash's configuration-clock prerequisite was also verified.

## PAC and data

- All 50 semantic enum types passed all 256 `u8` input patterns, including
  reserved values/masking. Separate generated 12/32-bit sparse fixtures and
  descending field offsets compile and run.
- Every emitted accessor across 156 arrays / 1,075 indexes passed exact bit
  position, neighbor preservation and bounds checks. Nine production arrays
  use explicit irregular offsets.
- 29 schema outcomes passed: four valid fixtures and 25 intended rejections for
  evidence, enum values/names, array identities/semantics, collisions, overflow,
  bounds and malformed representations.
- Against frozen v0.17, the 1,931 original scalar definitions and all 61 existing
  arrays / 658 indexed elements preserve physical positions, widths and access.
  Eight existing groups intentionally gain enum kinds. The new total adds 95
  arrays and indexes 417 previously scalar fields.
- A separate expanded contract comparison preserves all 463 reusable register
  views / 2,748 field slots in address, width, access, reset and side-effect
  behavior. These counts distinguish source definitions from expanded views.

## Integration and motor preservation

- Actual ADC/DMA probes pass 21 lifecycle cases per chip, including clock
  retention on cancellation/poison and re-split after forgotten ownership.
- Normal and forgotten F030 comparator-brake cleanup orders pass. Late
  ADC/VC/CORDIC/ATIM handlers return after relevant peripheral address space is
  unmapped in the host probe, checking that gated-off peripherals are not read.
- Motor verification passes 144 PWM/pin/analog traces, including 128 commutation
  permutations, plus ADC configuration/reserved-bit/trigger and all three BTIM
  setup/arm/acknowledgment traces. Startup takeover adds explicit clock retention;
  ISR handle acquisition does not add a clock operation.
- All 23 board Rust source files equal v0.17 after exactly normalizing five HSI
  API spelling changes and UART SOURCE enum encoding1. Sampling, commutation,
  fault logic and executor placement retain their original source. Examples
  02–06 preserve HSI/AHB/APB at 96/96/96 MHz.

## Aggregate verification

- Formatting, actual persisted-JSON regeneration and byte-for-byte drift checks.
- Both PACs ARM release with runtime/metadata; both HALs minimum/full release.
- Both HALs minimum/full warnings-denied rustdoc.
- Fourteen individual optional-feature checks across both chips.
- All six board programs and 05/06 motor-output opt-in, ARM release.
- Additional linked executables include independent PWM splitting, distinct
  frequency queries, finite ADC DMA and mutable copy; L012 also includes
  DAC→OPA→ADC. All 32 external vector slots per chip match metadata, active
  DMA/GTIM1 entries are strong handlers, and Flash/RAM/stack bounds pass.

The source-only candidate was extracted into a fresh directory with no generated
trees or Cargo lock. A new Cargo target passed regeneration/drift, both-chip
PAC/HAL minimum/full release and strict docs, all six programs and both motor
opt-in builds. The final archive's 237 source files match the maintained and
compiled clean trees byte-for-byte; the only post-build change was this validation
record. All 48 JSON and 102 PAC/generated metadata files also match exactly.

All focused probe assets remain outside the maintained repository. No checked-in
tests, Python build dependency, extra platform cfg or generated artifacts were
added. Host register models do not establish hardware preload/bus-drain timing,
oscillator accuracy, electrical behavior, motor safety or full STM32 parity.
The existing `proc-macro-error2` future-incompatibility notice remains.
