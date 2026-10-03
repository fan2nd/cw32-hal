# v0.14.0 complete-chain validation

2026-10-03. Rust/Cargo 1.99.0, ARM target `thumbv6m-none-eabi`.
The [module comparison](full-chain-audit-v0.14.0.md) records the findings,
corrections and remaining scope. This record states what was actually checked.

## Data and schema

- All 46 IP models and 87 instance bases checked against pinned vendor sources.
- 520 expanded register views; 442 unique known reset records, 21 unknown records,
  28 instance overrides and the nine shared-watchdog reset values independently
  reconciled with the cited manual headings. Reused DMA records account for the
  57-view difference between unique and expanded counts.
- All 158 gate/reset references, both 32-entry IRQ tables, 79 pin identities and
  233 maintained routes checked; documented source discrepancies remain explicit.
- 18 malformed topology/namespace JSON cases rejected; seven additional
  path/core/target/memory-bound cases passed, including no partial output on a
  flattened filename collision. Valid final-address and baseline inputs accepted.
- Renaming both L012 reference-divider instances in copied normalized JSON,
  regenerating PAC metadata and running the actual HAL build script produced the
  correct associated type for all four comparators. No VC12REF/VC34REF string
  inference remained in those generated bindings.

## Generator and PAC

- Two chips' 48 selected IP models share 46 PAC and 46 metadata modules. Final
  generated tree has 102 files; normalized JSON has 48 files.
- 993 accessor instances tested: 824 audited-reset writes, 852 explicit typed
  writes, 976 reads, 592 ordinary modifies, 49 register/subblock bounds and 270
  indexed field checks. These include repeated instance/subblock use, not 993
  distinct register definitions.
- F030 narrow accesses include nine byte and two halfword views, preserving
  adjacent bytes. Known nonzero, varying indexed and unknown resets are covered.
- 24 intended access-capability compile failures, including unknown defaults,
  forbidden command/flag/read-effect modify and directional access.
- Synthetic irregular arrays, partial reset knowledge, sparse enums/to_bits,
  IRQ holes, cross-chip reset/provenance aggregation, input-order stability,
  duplicate/colliding filenames and conflicting IP definitions checked.
- Copied JSON-only input generated a standalone compilable bundle without YAML.
  A separate consumer-only crate copy built both chip PAC+metadata+rt ARM
  configurations without data/generator/parser dependencies. Removed optional
  metadata/runtime files still permit plain PAC but correctly reject those
  features when requested.

## HAL behavior and ownership

- ADC: 32 fresh lifecycle cases plus both real-PAC sequence/register probes,
  including forgotten predecessor cleanup and validation-before-MMIO.
- Both comparator event/cancellation/shutdown probes; CORDIC/EAU/analog domain,
  command order, flags and bounded-polling probes. Six positive ARM API clients
  and 59 expected analog/math ownership/type/Binding rejections.
- Both actual time-driver/queue/counter implementations: pre-init calls and
  retained alarms, missed compares, wrap/peer flags, saturation, reentrant wake
  and RawWaker clone/drop callbacks, plus 40 near/distant/wrap cases per IP.
- Current ATIM owners/events and time-startup code with instrumented actual PAC:
  constructor/commit trigger gating, invalid requests, F030 Busy, injected break
  races, explicit acknowledgement/rearm, wait/cancel/Drop and valid dividers.
- GPIO: 26 current event/MMIO cases per chip. Generic timers: eight mixed or
  standalone IP configurations and 327,680 normalized duty endpoint cases.
- RCC extracted validators/trim/transition routines cover invalid boot state,
  clock-ordering and failure paths. Factory calibration reads and physical clock
  changes were not emulated or performed.
- Current digital compile-negative checks: GPIO 18, ATIM 10, generic timer 27,
  with their corresponding positive clients.
- Independent review covers all 45 HAL `src/` files and its build script and final data/PAC fixes.
  Additional independent compiler probes accept correct comparator pairings and
  reject wrong pairings, reserved GTIM1 access, unknown IRQs and wrong bindings.

## Production build and link matrix

- Host formatting, all-chip generation and byte-for-byte drift check passed.
- Both chip PACs with metadata/runtime built for ARM release.
- Both HAL chips built minimum and full valid feature configurations for ARM
  release; minimum/full documentation passed with `RUSTDOCFLAGS="-D warnings"`.
- All six independent board programs linked for ARM release; 05/06 additionally
  linked with `motor-output-enable`. All 23 board Rust source files are byte-identical
  to v0.13.2; sampling/control/fault logic and default power-disable policy remain.
- Additional minimal full-feature ARM executables linked for each chip. ELF32 ARM
  headers, all 32 physical vector slots against current metadata, the strong
  GTIM1 time handler and allocated Flash/RAM bounds were checked.

## Clean source archive

A fresh archive extraction, without generated output, Cargo.lock or target files,
regenerated and repeated the complete release/strict-doc/example matrix using a
new Cargo target directory. All checks passed. The 48 JSON and 102 generated PAC/
metadata/runtime files match the maintained tree byte-for-byte. Final archive
entries are compared with maintained source and the clean compiled tree; any
post-build changes are documentation only.

## Limits and reproducibility

Focused probes are external verification assets, not repository test scaffolding
or required build dependencies. They use current code and generated accessors;
where registers/events are modeled, the matrix identifies that boundary.

No board was flashed, no motor powered, and no analog accuracy, accelerator
numerical accuracy, ISR latency, waveform timing or electrical safety measured.
No full STM32 driver parity is claimed. The toolchain lacks cargo-clippy; no
Clippy result is claimed. Cargo's existing proc-macro-error2 future-compatibility
notice is not a project compile failure.
