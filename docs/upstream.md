# Upstream references and compatibility contract

This workspace is an experimental, independent CW32 port. It is not an official
Embassy project and does not claim complete chip or peripheral coverage.

## Pinned references

The editable upstream pins are in the canonical
[sources/evidence-sources.json](../sources/evidence-sources.json) lock.
The generated [source catalog](../build/provenance/SOURCE-CATALOG.md) lists exact Embassy,
stm32-data, chiptool and svd-parser commits. The
[reference index](../build/provenance/reference-index.json) also records local generator
source-file hashes, exact Rust dependencies and verification-tool versions.
`./d provenance` checks Cargo.lock Git revisions against the canonical pins.

The chiptool revision matches stm32-data's pinned dependency, not a floating head.
The root license files are from Embassy. Embassy, chiptool and svd-parser have
verified MIT/Apache-2.0 notices for the inspected code/package scope. In the pinned
stm32-data checkout, stm32-metapac-gen declares MIT OR Apache-2.0, but no root
LICENSE or license key was found in the inspected root/data-gen/data-serde
manifests. Do not infer a repository-wide redistribution grant from one package.
The canonical lock records the inspected license locations and hashes. Vendor
source conditions are separately tracked and raw vendor material is excluded.

## Preserved architecture

1. Vendor documents/SVD/header inputs are acquired with provenance and hashes.
2. Explicit vendor import creates review candidates. Normal `cw32-data-gen` reads
   authoritative curated YAML plus source-backed chip metadata and emits normalized IR.
3. `cw32-data-serde` defines the intermediate chip metadata schema.
4. `cw32-metapac-gen` renders PACs through chiptool, with per-chip feature selection,
   reusable peripheral modules, interrupt vectors, linker data, and static metadata.
5. `embassy-cw32/build.rs` consumes selected-chip static metadata, generating
   peripheral ownership tokens and type-level interrupts.
6. HAL drivers use actual upstream `embassy-hal-internal::Peri`, singleton macros,
   sealed traits, critical sections, and embedded-hal implementations.

`bind_interrupts!` is adapted from the pinned embassy-stm32 macro, preserving
multiple handlers for shared interrupt lines, conditional handlers, and unsafe
`Binding<I, H>` implementation. There is no callback registry or dynamic dispatch.

## Deliberate divergences and incomplete areas

- CW32 hardware is not STM32 hardware. Addresses, register layouts, clock trees,
  DMA request wiring, alternate-function muxes, and IRQ names must come from CW32
  sources, never renamed STM32 values.
- The acquisition path imports CW32 vendor SVD data into separate candidates.
  Maintained versioned YAML is authoritative for normal generation, matching the
  upstream authoring policy. STM32Cube databases do not describe CW32. Missing
  metadata is explicit rather than fabricated.
- Chip-level metadata and a generated PAC do not mean a tested HAL driver exists.
  Consult the coverage manifest before selecting a peripheral.
- `init(Config)` validates configuration, acquires singleton ownership, and
  configures the verified family-specific HSI path with bounded readiness polls. It returns `Result<Peripherals, rcc::Error>`. Stopped-HSI factory calibration uses reviewed
  temporary-source sequencing, with retained-RTC constraints on affected families.
  Board supply, active incoming clock and oscillator assumptions still apply.
  Hardware failure can consume ownership and leave partial clock changes; reset
  before retrying. This is not arbitrary external-clock fault recovery, and it
  does not imply general PLL, external-oscillator or low-power-mode support.
- Generic family/chip features do not assert package pin availability. Pin tokens
  must be generated only from verified package mappings.
- The public `Async` marker is only an API building block. It does not make blocking
  code asynchronous. The optional GTIM/GTIM1 run-mode time driver uses the pinned
  current Driver/Queue APIs with qualified clocks and an explicit blackout bound;
  see [time-driver.md](time-driver.md). No low-power integration is claimed.
- No hardware-in-loop tests were performed; build/tests cannot establish electrical
  behavior, oscillator accuracy, silicon errata compliance, or DMA cancellation.

The optional trigger-register extension preserves legacy `Trigger {signal, source}`
JSON. Its owned register record is defined once in the independently authored
`cw32-data-serde` model and reused by the PAC generator; static PAC metadata is a
borrowed projection. This additive extension does not change the existing source
ancestry or upstream redistribution limitations. CW32L010's two qualified routes
produce destination-local selector/enable types, not a cross-timer ITR enum.
