# Actual chiptool backend: v0.24.0

The PAC register implementation now comes from upstream chiptool, rather than a
local renderer that imitates its API. `cw32-gen` depends on the exact Git revision
`be1bff3e9e1b27b090e69bd9ac753c66fdcce678` and calls both
`chiptool::validate::validate` and `chiptool::generate::render`. The previous
handwritten block/address/array/field/enum renderer has been removed.

## Reference chain and provenance

These are separately identified snapshots, not an invented single dependency chain:

- The HAL architecture comparison uses [Embassy b12a6d9](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444).
  Its [embassy-stm32 manifest](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/Cargo.toml)
  selects stm32-data-generated tag `stm32-data-caa36afd62510b0e6315ee0dccd1f9c65fbcac83`.
- The generator architecture comparison uses [stm32-data e6a417f](https://github.com/embassy-rs/stm32-data/tree/e6a417fa643efaf031abc79590ea3e76bc4edbf2).
  Its [workspace manifest](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/Cargo.toml)
  pins chiptool `be1bff3e9e1b27b090e69bd9ac753c66fdcce678`.
- That exact chiptool pin is now the executable CW32 backend. The older
  `bcf538a2` references in historical audits describe earlier API comparisons and
  the adapted common register core, not the current renderer dependency.

The upstream [stm32-metapac-gen](https://github.com/embassy-rs/stm32-data/blob/e6a417fa643efaf031abc79590ea3e76bc4edbf2/stm32-metapac-gen/src/lib.rs)
loads shared register JSON by kind/version, expands inheritance, places fieldsets
under `regs` and enums under `vals`, then renders shared peripheral files through
chiptool. Per-chip files select those modules and supply instance addresses.
Normal consumer builds select already-generated Rust. CW32 follows these
boundaries while retaining a single host-generator crate.

## CW32 pipeline

1. `cw32_gen::data` loads the audited IP/chip/family YAML, applies the existing
   evidenced transformations and writes schema-11 JSON to disk.
2. `cw32_gen::pac` reopens every selected chip JSON and its shared register JSON,
   rejects invalid schema/reference/identity/layout/reset metadata and combines
   compatible shared `(kind, version)` definitions.
3. The private `pac::chiptool_backend` lowers this validated model to real
   chiptool `IR`, `Block`, `FieldSet`, `Enum`, register access markers and arrays.
   A register always gets its own fieldset identity, even when another register
   has the same fields. Register and fieldset bus widths are identical.
4. chiptool validates references, field/enum widths and enum values. Its
   coarse register/field overlap checks are disabled only after CW32 has
   checked every expanded byte/bit range and explicit narrow alias. Upstream
   compares base spans, which cannot express intentional narrow register
   views or nonzero-first-offset explicit field arrays.
5. `generate::render` emits all block structures, address accessors, regular and
   irregular register/field arrays, nested block references, field masks,
   getters/setters, enums and reserved-value conversions.
6. A narrowly scoped Rust AST pass preserves the extra CW32 contracts below.
   It parses the actual upstream token output with `syn`; it does not replace
   Rust text, reimplement bit masks, recalculate array offsets, or fall back to
   the old renderer. Unexpected structural shapes produce generation errors.
7. One shared file per kind/version is written under `peripherals/`. Chip roots
   retain their existing module selection, instances, constants and IRQ ABI.
   Metadata, the runtime vector table and `device.x` remain CW32-generated.

The internal lowering is not a new maintained source format. Normalized JSON
still contains the richer audited CW32 information, including reset evidence and
side effects. No in-memory shortcut bypasses the persisted JSON boundary.

## Why an extension is needed

The pinned [IR](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/ir.rs)
has register-level R/W/RW, but no reset value/mask, read-action or modified-write
semantics. Its [fieldset renderer](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/generate/fieldset.rs)
gives every value a zero `Default`; its [common module](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/generate/common.rs)
allows generic `modify` on every RW register.

The public [renderer options](https://github.com/embassy-rs/chiptool/blob/be1bff3e9e1b27b090e69bd9ac753c66fdcce678/src/generate/mod.rs)
provide an external common module, defmt selection and the no-std attribute.
They do not provide reset or per-register side-effect callbacks. Consequently:

- CW32 uses `CommonModule::External(crate::common)`, disables generated defmt
  references and skips the inner no-std attribute. The existing audited common
  access layer remains the one implementation used by every chip.
- Every upstream fieldset `Default` is removed, with an exact one-per-register
  identity check. Only complete documented reset words get a replacement.
  Unknown reset words get no `Default` and no reset-seeded `write`.
- Instance-specific GPIO reset types retain their const parameters and evidence.
  Different array-element reset words retain the indexed `GivenReset` policy.
  An array mixing known and unknown reset words exposes explicit `write_value`,
  without inventing one common default.
- Register accessors retain the existing read/write side-effect marker types.
  Generic `modify` remains restricted to ordinary readable/writable registers.
  W1C, W0C, keyed, command, mixed, write-once, read-clear, FIFO and latch policies
  do not silently become ordinary RMW.
- The AST pass preserves existing names, keyword raw identifiers and explicitly
  raw one-bit `u8` fields. The upstream default one-bit choice is `bool`.
  Debug output now uses upstream's decoded field presentation.
- Register offset/constants and reset evidence accessors remain compatibility
  additions. Cross-IP references are rebound from versioned generation names
  to each chip root's selected public kind modules.

Using the upstream options plus this checked adapter avoids maintaining a
chiptool fork. Taking unmodified upstream zero defaults would regress already
verified CW32 behavior; retaining the old handwritten renderer would fail to
reuse chiptool. A future upstream extension API can replace the AST adaptation
without changing the maintained YAML or the consumer build boundary.

## Deliberate boundaries

- No new hardware facts, reset values, routing, DMA ownership, HAL algorithms or
  example ISR behavior are introduced by this migration.
- CW32 still validates only its supported 8/16/32-bit register widths and bounded
  arrays. It does not inherit every permissive input accepted by upstream IR.
- Fieldset setters manipulate a local value; they are not proof that hardware
  permits writing reserved bits or fields. `write_once` is marked as a side
  effect and excludes RMW; it does not enforce a lifetime-wide one-write count.
- All generated JSON/Rust, Cargo.lock and target outputs remain ignored and are
  omitted from the source-only archive. Run `cargo run -p xtask -- regenerate`
  once before normal PAC/HAL consumer builds. The consumer PAC still has no
  generator build dependency and does not read YAML/JSON.
- Direct host dependencies and the chiptool Git revision are pinned. The lock
  is locally resolved; complete transitive reproducibility is not claimed.
  `serde_yaml` uses upstream's exact `0.9.34-deprecated` requirement to avoid
  Cargo's incompatible `0.9.34+deprecated` versus prerelease resolution.

See [the v0.24.0 validation record](validation-v0.24.0.md). No physical-board
verification is implied by code generation, compilation or RAM-backed tests.
