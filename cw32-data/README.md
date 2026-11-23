# CW32 authored hardware data

This directory contains the reviewed hardware descriptions used to generate the
CW32 chip metadata and PAC. The authoring workflow follows pinned `stm32-data`:
maintain YAML definitions and routing rules, then generate chip/register JSON and
Rust. Vendor downloads and the source identity catalog live in [`../sources/`](../sources/README.md).

## Authored inputs

- `registers/*.yaml`: versioned register blocks, fields, access modes and enums.
- `inputs/*.yaml`: family configuration, source IDs/locations, peripheral versions,
  IRQ corrections and explicit vendor-import corrections.
- `parts.yaml`, `additional-parts.yaml`: qualified part/package/memory selections.
- `pinouts/*.yaml`, `af/*.yaml`: package bonds and qualified peripheral pin routes.
- `clock/*.yaml`, `dma/*.yaml`, `triggers/*.yaml`: clock/reset controls, DMA mappings
  and destination-specific trigger routes.
- Root YAML catalogs: electrical limits, peripheral capabilities, field/write
  policies and explicitly reviewed register reuse.

The generated `data/` tree and `../cw32-metapac/` are ignored build products and
are excluded from source distributions. Do not edit them. The canonical source
lock is `../sources/evidence-sources.json`; JSON is retained for machine-readable
source identities, vendor snapshots and generated output, not as a second copy
of the authored hardware descriptions.

## Generate and review

```sh
./d fetch-sources
./d gen-all
./d test
./d check
```

The root generator workspace builds before the generated PAC exists; the
separate `firmware/Cargo.toml` workspace builds the HAL and generated PAC.
`./d gen-all` produces both data and PAC. Full original-source checks additionally
need the hash-verified official manuals and SDK members acquired by
`./d fetch-evidence`. See the root README for toolchain requirements.

`./d import-registers` writes fresh candidates under `build/register-candidates/`.
It never overwrites authored YAML or treats a raw SVD as reviewed HAL metadata.
Compare candidates with the authored definitions, inspect the original manual,
and deliberately update the source evidence and correction policy. Changed
source hashes and old-value guards prevent silently applying patches to a new
vendor revision.

Register reuse requires equality of the complete normalized IR, including names,
descriptions, layout, access and enums. It establishes a shared PAC representation,
not equal electrical limits or HAL behavior.

## Coverage

The catalog covers 13 families and 37 current part models. Its 54 selectable
profiles also include generic families and legacy aliases; they are not 54
independently verified physical parts. Register definitions, physical package
pins, qualified AF routes, central RCC controls, DMA metadata and supported
peripheral electrical limits have source-backed data. Detailed capabilities
vary by family, instance and operating mode.

Coverage reports are regenerated from the authored catalogs. An existing chip
feature or peripheral record does not imply every HAL operation is implemented.
RF work is explicitly deferred. Advanced routing, low-power operation and some
peripheral modes remain incomplete; unknown or withheld mappings must not be
interpreted as absent hardware. No silicon or board execution is claimed.

## Source provenance and redistribution

Use `../sources/README.md` and its canonical lock for official download URLs,
printed revisions, hashes, upstream commits and license status. Generated
compatibility indexes belong in `../build/provenance/`; `./d provenance` checks
them against the source authority. Refreshes require deliberate source review.

Vendor archives, PDFs and SVDs are downloaded into ignored `sources/vendor/` and
are not bundled where redistribution permission has not been established.
The independently rewritten schema and macros retain factual ancestry records
in `../docs/upstream-file-provenance.json`. Old unresolved predecessor snapshots
are excluded. Neither that replacement nor the project license files constitute
legal clearance for external material.

## Package-neutral firmware targets

Family input manifests list interchangeable order codes under `firmware_targets`.
Generation compares the completed chip records, excluding the order code and
physical package tables. Memory, bonded GPIOs, peripheral routes and capabilities
must match. The generated `data/package-groups/` index maps each shared feature
to its exact package records. For example, select `cw32l012c8` in HAL or PAC
for either CW32L012C8T6 or CW32L012C8U6. Use the exact order code in
`data/chips/` to look up physical pin numbers.
