# v0.13.0 PAC and HAL architecture validation

2026-10-03, Rust/Cargo1.99.0, ARM target `thumbv6m-none-eabi`.

This record covers the typed PAC and all indexed register/subblock groups, GPIO shared-driver and interrupt Wait implementation, ADC/comparator owner/mode/channel redesign, validated clock lookup, generic timer/PWM, and migrated callers. These are implemented and reviewed; it does not claim complete embassy-stm32 feature parity.

## Source and coverage review

- Audited all46 IP definitions and520 original register views, classifying84 candidate structures. Implemented24 register arrays and2 DMA subblock arrays; semantic/layout exceptions are documented in register-array-coverage.md.
- Independent expansion reproduces all520 original views,2,919 field instances and7,667 implemented bits. All69 register-array element records preserve original names, descriptions and reset values/sources/notes.
- GPIO descending AFR bank offsets, narrow ODR aliases, irregular ATIM CCMR offsets, DMA stride/child layout and special CCR/CH4 layouts are checked explicitly.
- Hardware reset evidence survives schema5/YAML/JSON/metadata/PAC. Per-element initialization is independently checked for F030 GTIM CCR and L012 RTC alarm; common Default is absent when no single value exists.
- GPIO Flex/wrappers, ownership erasure, Drop/release, mode ordering, pull capability checks and F030 SPEED/DRIVER polarity were independently compared with fixed Embassy source and official CW32 RM2.5. No source-level blocking issue found.

## Executed checks

- Host generator/xtask checks, all-chip regenerate, byte-for-byte regenerate --check, cargo fmt --all --check.
- Both chip PACs with metadata/rt on ARM.
- Both HAL chips: minimum no-default release and full memory-x/metadata/unstable-pac/defmt/time-driver-any ARM release.
- All six board examples ARM release;05/06 motor-output-enable release. No local code warnings.
- L012 and F030 HAL documentation checks, minimum and full features, with RUSTDOCFLAGS="-D warnings".
- Independent47 JSON mutation cases: counts/overflow/strict fields, reset width/provenance/equality, aliases, nested overlap/cycle/missing/wrong-version references and ownership relationships.
- Six YAML→persistedJSON→PAC cases: nested-only and deep dependencies, perimap kind/version normalization and invalid references.
- Host pointer/default/bounds probes:529 expanded register handles across the two chip views; all26 indexed accessors and both unequal-reset groups. An independent probe additionally exercised78 valid element offsets and52 rejected out-of-range accesses.
- Typed read/write/modify memory probes retain ADC bit8, allow deliberate reset-one clearing, do not merge raw writes with reset, and perform exact8/16-bit transactions.
- Compile-negative probes reject unknown Default/write, BSRR command modify, W0C modify, read-clear CCR modify, WO read, RO write, numeric write_value, integer-to-bool setters and a false shared RTC alarm Default.
- Analog/ATIM external word/event comparisons and board-call-site trace comparisons preserve configuration words, IRQ handling, ADC reserved reads and PWM order. ATIM comparisons include161,760 F030 and4,194,304 L012 configurations,100,000 random event traces/family and200,000 F030 duty cases. Board comparisons check exact21/22-write ATIM setup traces,1,005 ADC CR snapshots and343 PWM combinations including full-u32 boundaries.

## ADC, comparator and GPIO interrupt validation

- Both chip implementations have actual mode-bearing Adc/Comp owners. Independent source review checked pin/reference borrows, settling/readiness, shared analog ownership, disabled comparator construction and cancellation ordering.
- 24 ADC compiler-negative ownership/type cases and 18 comparator cases reject fabricated/mismatched inputs, resource reuse and invalid borrowing.
- ADC source-derived state/register probes: 46 checks, including 18 new F030 single-channel EOC versus scan EOS cases. The previous F030 MODE4 watchdog mismatch was corrected using RM2.5 §22.9: N=1 MODE0 is supported, multi-slot watchdog requests are rejected before writes.
- Comparator runtime probes: 26 scenarios and 58 assertions, including disabled/ready behavior and reference/brake lifetime cleanup.
- GPIO async: 9 compiler-negative Binding/ownership cases per chip and 29 source-derived register/race/cancellation traces per chip. Shared-port peers survive cancellation/Drop; stale state, arm/recheck, level wait, waker and forgotten-future paths were checked.
- Independent review separately checked real port IRQ topology, per-pin state and mask/clear/latch/wake ordering. A clean source review is not presented as a separate rerun of the implementation probes.

## Generic timer/PWM validation

- Audited 90 additional main-output routes against primary manuals/datasheets and exact SDK definitions: 44 L012 (41 GTIM plus 3 ATIM.CH4) and 46 F030 GTIM. Chip route inventories now contain 126 and 107 entries respectively. F030 ATIM.CH4 remains internal-only; package/board bonding is not inferred.
- Three positive external ARM clients and 27 compiler-negative cases cover ATIM/GTIM ownership, optional pins, channel/AF restrictions, L012 CH4, reserved GTIM1, SWD pins, exclusive channel borrowing and invalid caller clocks.
- Extracted production divider/period solver checked 4 MHz, 8 MHz, 96 MHz and u32::MAX clock boundaries, unsupported divisors, full 65536-count periods and exact frequency ratios. 327,680 normalized u16 duty mappings cover native periods 2, 3, 100, 65535 and 65536.
- Real production Timer/SimplePwm/backends plus exact generated PAC accessors ran with instrumented MMIO and host clock/pin stubs for both chips' ATIM and GTIM. Checked stopped/disconnected construction, explicit start/enable, invalid request zero writes, ordinary running duty exactly one CCR write, forced-full endpoints, trigger-gated software updates, restoration and Drop.
- Independent manual/source review checked all three prescaler families, F030 mode polarity, L012 MMS/MMS2 and F030 MSCR/TRIG gates. An unnecessary L012 URS=1 setting was removed to avoid conflicting manual prose about preload transfer; changed-source build/API/state probes passed again.
- Running enable explicitly permits the prior active preload until the next update. Frequency changes and maximum-period forced-mode transitions reset phase; ordinary duty does not stop peer channels. Disabled pins float. These limitations are documented, not hidden by passing builds.

Validation probes and scripts stay outside the source tree. No test crate, Python project, host example target or target-platform shim was added. Existing shared board target configuration is retained.

## Clean source archive verification

A fresh extraction without generated JSON/PAC, Cargo.lock or target artifacts
was regenerated using the same Rust toolchain and a new Cargo target directory.
The complete minimum/full release and strict-documentation matrix passed again
for both chips; all six board executables linked, and 05/06 opt-in builds passed.
All 48 regenerated JSON files and 8 PAC/metadata/runtime files are byte-identical
to the maintained tree. Formatting and read-only drift checks passed. Chip-less,
dual-chip and package-suffixed selections were rejected for their intended reasons.

The archive contains only maintained source/evidence/documentation. It excludes
generated trees, locks, build output, validation scripts/tests, logs and download
caches. Development/probe scripts are not required for source regeneration.

## Limits

No hardware was flashed or powered; source/compile/probe results do not establish actual pin timing, ADC analog settling, ISR latency or motor safety. Direct PAC access requires correct ownership, clock/power and register-side-effect discipline. Indexed Default is an in-memory initializer, never proof of present hardware state.

The dependency proc-macro-error2 2.0.1 still emits Cargo's pre-existing future-incompatibility notice; current builds succeed. See embassy-api-alignment.md for the explicit remaining feature scope and small code-generation debt.
