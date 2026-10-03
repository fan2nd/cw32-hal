# v0.13.1 HAL cfg and shared-workflow validation

2026-10-03; Rust/Cargo 1.99.0; `thumbv6m-none-eabi`. This release changes HAL
selection and sharing, not device support or public driver functionality.
No chip was flashed and no electrical/timing measurements were made.

## Selection and data checks

- HAL no longer emits chip/family cfgs or branches on `Metadata.family`.
  Chip Cargo features remain solely the selector and HAL/PAC consistency check.
- Real metadata probes for both chips verify that renaming chip/family does not
  change driver cfgs; removing an IP removes only its presence/version/instance
  predicates; an unaudited IP version is rejected.
- GPIO capability probes remove SPEED, DRIVER or both HIGHIE/LOWIE independently.
  Only the matching capability disappears. Incomplete level-IRQ pairs and invalid
  register access/layout are rejected. Scalar and indexed pull-down layouts have
  symmetric ordinary RW/read-behavior checks.
- Schema 6 reset effects survive YAML → persisted JSON → PAC metadata. Eight
  negative cases reject self/unknown/duplicate targets, missing reset control,
  empty description/source, extra keys and old schema input. An independent
  reviewer reran the six relationship/provenance cases and both metadata probes.
- F030 ADC reset suppression consumes explicit VC1/VC2 cross-effects, preserving
  the existing shared-BGR protection without a family exception.
- All 46 maintained register YAML files and both generated typed `pac.rs` files
  remain byte-identical to v0.13.0. Register arrays, defaults and offsets did not
  change or get relabeled to manufacture reuse.

## Shared algorithms and independent backends

- GPIO source is equivalent to v0.13.0 after mapping old version predicates to
  the independently checked capabilities. Fresh production-source GPIO hardware
  and wait-state probes pass 29 traces per chip.
- ADC: normalized source comparison matches 12 extracted lifecycle/IRQ items;
  32 lifecycle/race/cancellation and 18 F030 register/mode/watchdog checks pass.
  Two positive API builds and 24 expected ownership/IRQ/channel failures pass.
  ATIM helpers require the compatible IP; paired ADC methods require both real
  ADC instances.
- Time driver: common state/schedule/ISR and initialization synchronization
  preserve both originals. Nine source-derived timer/queue/IRQ cases per IP
  pass, including reentrant wakers and missed compare recovery. The separate
  UIFCPY and OV/CNT/OV counter algorithms are byte-identical to the prior version.
- Analog: BGR/DAC/VCREF/OPA and comparator setup/Drop/hardware hooks preserve
  normalized source equivalence. Shared comparator state probes pass 26 scenarios
  and 58 assertions. Three positive API probe files and 28 expected ownership,
  reference, mode, binding and brake-borrow failures pass. Twelve independently
  filtered metadata fixtures compile standalone and reduced analog-IP combinations.
- Timer: eight configurations compile and run exact production code with real
  generated PAC types: both shipped pairs, both mixed-version pairs and each of
  the four standalone ATIM/GTIM adapters. Missing IPs are genuinely undefined in
  the fixture. Shipped-pair MMIO traces remain byte-identical: 364 L012 writes and
  302 F030 writes. Three positive and 27 expected-negative public API cases and
  the frequency/full-scale probes pass again.
- ATIM events: 10,000 old/new owner/wait/IRQ/cancel/rearm/Drop schedules per IP
  have identical traces. Configuration, power, duty, brake and backend event
  masks/service bodies remain token-equivalent; common event ownership has one
  implementation. Both-chip positive API checks and ten intended IRQ-proof/owner
  rejection cases pass. Version-specific protection and update behavior is retained.

Synthetic combinations prove software-layer independence only. They are not
new chip support or evidence that any imaginary mixed device exists.

## Production matrix

- `cargo fmt --all --check`, regenerate and read-only byte drift check pass.
- Both chip PACs with metadata/runtime build for ARM release.
- Both HALs build minimum and full valid feature configurations in ARM release.
- Both HALs build minimum/full documentation with `RUSTDOCFLAGS="-D warnings"`.
- All six board executables link for ARM release; 05/06 also link with explicit
  `motor-output-enable`. Default power-output policy is unchanged.

No repository tests, external validation scripts, Python dependencies, host
example target or target-platform shims were added. Temporary probes are not
part of the archive. The toolchain lacks cargo-clippy, so no Clippy result is
claimed. Cargo's pre-existing proc-macro-error2 future-compatibility notice is
not a local source warning.

## Clean archive rebuild

A fresh source-only extraction with a new Cargo target directory repeated the
entire production matrix successfully: generation, drift, formatting, both-chip
PAC and minimum/full HAL release builds, strict docs, six executables and 05/06
opt-in. All 48 normalized JSON and 8 PAC/metadata/runtime files match the working
tree byte-for-byte. Every packaged file was compared with maintained source;
no generated artifacts, locks, tests, logs or download caches are included.

## Boundaries

The [layering document](hal-cfg-layering.md) lists shared and version-specific
code. Genuine startup/source dependencies remain explicit. Some IP-specific
public configuration/owner declarations and the audited analog reference-pair
mapping remain separate; this release does not claim zero duplication or the
entire upstream code-generation framework. General bus/DMA/capture/encoder,
low-power, calibration and other previously unsupported features remain outside
scope. See [overall alignment](embassy-api-alignment.md).
