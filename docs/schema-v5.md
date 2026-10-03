# Normalized schema v5 and PAC structure

One `cw32-gen` crate still owns schema/data/pac modules. Stage1 writes chip and shared register JSON to disk; stage2 rereads only that versioned JSON. Old schemas are rejected. The source-only distribution contains neither generated JSON/PAC nor host validation probes.

## Indexed source model

- `Field.array: {len, stride}` describes repeated bits within one register. Indices are local and zero-based.
- `Register.array` is either `{len, stride}` (bytes) or `{offsets: [...]}`. Explicit offsets preserve source index order, including descending physical offsets such as GPIO `[AFRL, AFRH]`.
- An indexed register replaces its former scalar entries. Its `elements` list preserves each original name, description and complete reset value/source/note in index order. The common fieldset/access/bus width/read behavior/write behavior must be the same; genuinely different layouts remain distinct types/accessors.
- `Block.blocks` contains nested block items with `name`, `offset`, `block`, `version`, `array`, `description` and `source`. DMA `ch(n)` reuses the existing `dmachannel` model, with the parent/channel ownership relationship preserved. It does not manufacture channel ownership tokens.
- PAC generation does not infer arrays from names or regexes. Every group, stride, irregular offset and nested block is persisted explicitly in YAML and normalized JSON.

Validation checks array counts, positive strides, bounded/checked offset arithmetic, duplicates, alignments, byte overlaps, alias containment, recursive cycles/version references, namespace collisions, exact element records, full reset widths/provenance and instance ownership overlaps. Array aliases match corresponding canonical elements, or a scalar canonical byte range for narrow views such as `odr_byte(n)`. Clock/reset bit references and per-instance reset overrides currently target scalar registers; the present GPIO instance overrides remain scalar and lossless.

## Typed access

The common core is adapted from pinned chiptool. `common::Reg<T, A, ...>` is value-first; access markers are `R`, `W`, `RW`. `read()` returns `T`; `write_value(T)` writes an explicitly constructed transparent fieldset. `write(|w| ...)` starts from its audited reset initializer; `modify(|w| ...)` reads once and writes once. Methods are safe like upstream; unsafe `from_ptr` construction establishes the MMIO address/layout contract. Direct PAC users must still coordinate hardware ownership, clocks and side effects.

Root constants such as `GPIOA` have peripheral types such as `gpio::Gpio`; fieldsets live in `gpio::regs`, enum values in `gpio::vals`. Register accessors take the copyable peripheral by value. Pure field getters/setters include bounds-checked indexed forms. They manipulate the in-memory value; register access direction gates hardware operations.

CW-specific safeguards remain explicit extensions:

- Only ordinary read/write registers expose `modify`; command, keyed, W0C/W1C and read-side-effect registers do not.
- Unknown reset values have no invented Default or reset-seeded write. Explicit `write_value` remains available in the allowed direction.
- A register array with one known shared reset uses the normal zero-sized Default policy. When all element resets are known but differ, its handle carries `GivenReset<T>` selected by index; the common fieldset has no false Default. F030 GTIM CCR and L012 RTC alarms use this path.
- Per-element `RESET_VALUES`, `RESET_SOURCES`, `RESET_NOTES` remain available alongside metadata. GPIO instance-specific known defaults retain their type specialization.
- Width-accurate transparent `u8/u16/u32` fieldsets produce actual narrow volatile transactions.
- Incompletely described enum encodings return `Option` rather than fabricating a reserved enum value.

See [coverage](register-array-coverage.md), [PAC/GPIO usage](pac-gpio-v0.13.md) and [current alignment boundaries](embassy-api-alignment.md).
