# v0.24.0 validation: executable chiptool backend

Date: 2026-10-05. Toolchain: Rust/Cargo/rustfmt 1.99.0, Linux x86_64,
real `thumbv6m-none-eabi` target. The backend is pinned upstream chiptool
`be1bff3e9e1b27b090e69bd9ac753c66fdcce678`; see
[implementation and provenance](chiptool-backend.md).

## Generated-code semantics

An external Rust-only harness loads the maintained normalized JSON, compiles
the actual generated PAC separately for each chip, and exercises volatile
accesses against aligned RAM. Both the pre-migration and chiptool-generated
outputs pass the same checks:

- 427 distinct register-value types and 472 expanded register elements
- 2,609 expanded field elements, 25,959 masked setter/getter samples
- 13,312 enum conversion round trips, including reserved encodings
- 993 instance leaf addresses, including nested/indexed DMA register banks
- 28 GPIO instance-specific reset defaults and 380 reset writes, including
  six varying-array reset writes
- 439 individually diagnosed compile-fail cases for forbidden operations

The checks cover 8/16/32-bit bus footprints and adjacent-byte preservation,
register/field array bounds, read/write_value/reset-seeded write, ordinary
modify, RO/WO restrictions, unknown-reset Default/write rejection and
side-effect modify rejection. Each negative case must produce its intended
E0277/E0599 at its own source line; an unrelated compilation failure does not
count as success.

Additional synthetic data covers irregular/nonzero-first register offsets,
nonzero-first field arrays, irregular nested blocks, narrow aliases at nonzero
byte offsets, narrow varying resets, mixed known/unknown reset arrays, sparse
and reserved enum encodings, raw one-bit keyword fields, and write-once,
one-to-set and read-latch method restrictions. The fixture negative batch
contains 176 diagnosed cases, including ten additional boundary cases; this
count overlaps the reused chip cases and is not added to the 439 above.

An independent read-only review reproduced and verified fixes for three adapter
edges: unsuffixed narrow reset literals, upstream base-only field-overlap false
positives, and reserved identifier-prefix collisions. Its separate consumers
also verify instance/unknown reset rejection, per-element reset writes and
nested DMA addressing. Real expanded field overlaps and oversized byte reset
values still fail CW32 validation. No remaining must-fix finding was identified.

## Build and generation checks

- Rust formatting and `git diff --check`
- Explicit YAML → persisted normalized JSON → reread/validate → chiptool PAC
- Byte-for-byte regeneration drift check
- Both PAC configurations: minimal and rt+metadata ARM release builds
- Both HAL configurations: minimal and full-feature ARM release builds
- All fourteen individually selected optional-feature configurations across
  both chips: rt, memory-x, metadata, unstable-pac, defmt, time-driver-gtim1,
  time-driver-any
- Both-chip minimal/full PAC and HAL docs with `RUSTDOCFLAGS=-D warnings`
- All six board firmware binaries linked in ARM release mode
- Startup/application (05 and 06) also linked with motor-output-enable
- JSON-only standalone generation and ARM compilation for both chips using
  copied inputs in a directory containing no YAML
- PAC normal/build dependency tree contains no cw32-gen, chiptool, YAML or JSON
  parser; ordinary consumer builds still select pre-generated Rust only
- Isolated PAC-only copies, without generator/data/HAL sources, build both
  chips with runtime/metadata; no-chip and two-chip configurations reject with
  the expected exactly-one-chip diagnostic

A clean source-only ZIP extraction, initially without generated files or
Cargo.lock, passed regeneration and the entire build/documentation/feature/
example matrix again using a fresh target directory. Its 48 normalized JSON
and 102 generated files match the maintained checkout byte for byte. Only this
validation record was completed afterward; no implementation source changed.

Generated Rust documentation escapes hardware bit ranges and autolinks source
URLs. This affects rendered docs, not stored metadata or reset-source strings.
The existing third-party proc-macro-error2 future-compatibility notice remains;
it is not a new CW32 warning.

## Preserved scope

The 57 hardware-data files, 12 vendor-evidence files, HAL runtime/build source,
and all example Rust code are unchanged from v0.23.1. Only workspace package
versions change outside the generator and documentation. Public DMA tokens
remain DMA_CH1..4 on L012 and DMA_CH1..5 on F030. The six examples remain in
examples/l012-bldc/01-gpio through 06-application.

All 48 normalized JSON files and all 56 generated files outside the 46
peripheral implementations preserve their content. The generated layout still
has 102 files. Shared common access, metadata, chip composition, vectors and
linker aliases retain their previous implementations; chiptool now supplies the
register structures, field/enum APIs and address/array logic.

Source delivery excludes generated-data, cw32-metapac/generated, Cargo.lock,
target, caches, downloaded dependencies and external validation harnesses.

These checks establish generated-code behavior against the maintained data.
They do not independently prove the manuals' accuracy, hardware reset values,
physical side effects, electrical safety, timing or loaded-motor behavior.
No physical-board, flashing, live bus or oscilloscope validation was performed.
