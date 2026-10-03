# Normalized schema v4

Version4 retains the schema3 chip pins, width, alias, ownership, behavior and kind/version architecture. It adds per-register `reset_value: Option<u32>`, `reset_source: Option<String>` and `reset_note: Option<String>`, plus each peripheral's `register_resets` list of authoritative instance-specific values.

A missing/null register reset is unknown, never zero. A known value requires evidence and must fit the actual bus width. Aliases must agree with their canonical physical byte slice, including effective per-instance overrides. Overrides name a real register exactly once and carry their own source and optional note.

Stage1 writes shared register documents and chip documents to disk. Stage2 rejects old versions and rereads these documents; reset data is not reconstructed from YAML, field masks, SDK routines or SVD inherited zeros. Generated metadata preserves shared values and instance overrides separately. HAL build scripts continue to consume metadata rather than YAML.

See [register reset defaults](register-reset-defaults.md) for source coverage, conflicts, instance specialization and the write API. [Schema3](schema-v3.md) remains the historical description of unchanged architecture; its version number is superseded here.
