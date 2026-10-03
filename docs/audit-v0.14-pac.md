# v0.14.0 generator and PAC comparison

Compared actual [chiptool generation](https://github.com/embassy-rs/chiptool/tree/bcf538a2e7b8584ae874ee9ab72efb1576fc6152/src/generate)
with the [pinned generated stm32-metapac](https://github.com/embassy-rs/stm32-data-generated/tree/e463add8cc54375f61c6f5f83d6b589e7fc68be2/stm32-metapac),
including common access, blocks, fieldsets, enums, metadata and consumer build.
The data-stage comparison is in the [data audit](audit-v0.14-data.md).

| Existing module/area | Comparison and correction | Verified boundary |
| --- | --- | --- |
| `cw32-gen/src/schema.rs` and `data/{mod,extensions}.rs` | Shared IP contracts, explicit instance facts and validated transforms remain separate; schema 7 adds sourced comparator topology and stricter namespace/target/path checks | One generator crate, actual persisted JSON, no package layer or generic importer fabricated |
| `cw32-gen/src/lib.rs` and `main.rs` | The multi-chip stage reloads every chip JSON and aggregates shared definitions; it never reuses the YAML-stage in-memory object as the PAC input | JSON-only copied input generates compilable standalone output without YAML |
| `pac::load_json` | Loads only versioned chip/register documents and rejects invalid references/schema before rendering | Unknown source keys, conflicting/shared identities and output collisions fail closed |
| Common `Reg` | Typed volatile value, sealed R/W/RW, closure write/modify and exact bus width follow chiptool's useful access shape | Hardware writes are explicit; no automatic reset OR, unproved RMW, or cosmetic raw-u32 closure facade |
| Defaults/reset specialization | Pinned chiptool defaults fieldsets to zero; CW keeps documented full words, unknown suppression and element/instance-specific policy | This is a deliberate safety extension, not a claim that upstream supplies accurate reset data |
| Shared reset evidence | Aggregate instance specializations across all selected chips; deterministically combine same-value provenance | Different reset values remain distinct types/policies; input order cannot change output or erase evidence |
| Peripheral modules | Emit each `(kind,version)` Rust definition once; chip roots choose modules and instances | 48 chip-level selections use 46 definitions; IWDT/WWDT each have one shared module |
| Arrays/subblocks/widths | Preserve explicit regular/irregular offsets, repeated blocks, indexed fields and narrow aliases | All-known varying defaults use indexed policy; mixed known/unknown arrays require explicit write values |
| Enums | Safe variants, `from_bits` with None for reserved encodings, masked setters and new `to_bits` | Infrastructure tested with sparse synthetic enums; maintained hardware multi-bit fields currently remain raw |
| Metadata | Shared types and one register table per IP; retain descriptions, field kind/values and comparator topology | GPIO capability generation now checks semantic Bool kind as well as layout/access |
| Runtime and IRQ | Validated discriminants, sparse zero slots and linker aliases remain generated | Sparse-vector fixture plus real both-chip ELF slot checks; no fabricated handler implementation |
| `cw32-metapac/{build.rs,src/lib.rs}` | Select generated Rust using build-time include paths; ordinary consumer has no parser/generator dependency | PAC required always, metadata/runtime artifacts required only for their features; isolated consumer-only builds verified |
| `xtask` and ignore rules | Regenerate/check the two owned trees; never maintain generated Rust by hand | Deterministic full-file drift checks; current and legacy generated paths remain ignored |

## Generated layout and migration

```text
cw32-metapac/generated/
  common.rs
  metadata_types.rs
  peripherals/<kind>_<version>.rs
  registers/<kind>_<version>.rs
  chips/<chip>/pac.rs
  chips/<chip>/metadata.rs
  chips/<chip>/rt.rs
  chips/<chip>/device.x
```

All module references stay inside the PAC crate; there is no cross-crate path
inclusion. The generated tree contains 102 files: 46 PAC IP modules, 46 metadata
IP modules, 8 chip files, common access and metadata types. It is excluded from
source archives and recreated with `cargo run -p xtask -- regenerate`.

`cw32-gen generate SOURCE CHIP|all JSON_ROOT PAC_ROOT` now uses the layout above
inside PAC_ROOT. The `data` command is unchanged. Standalone
`cw32-gen pac CHIP_JSON PAC_DIR` still places pac.rs/metadata.rs/rt.rs/device.x
directly in PAC_DIR with shared sibling files/directories. Copy the complete
PAC_DIR, not only pac.rs. Consumer builds no longer copy monolithic chip files
to OUT_DIR or read the obsolete `src/chips` tree.

## Verification

993 accessor instances, including repeated peripheral/subblock use, were checked
with real emitted accessors in memory-backed probes. Counts are not 993 distinct
hardware register definitions. Reset selection, width, adjacent-byte preservation,
addressing, bounds and typed operation permissions were exercised. Separate
negative probes cover forbidden access/RMW and unknown reset initialization.

Synthetic cases cover irregular offsets, partial reset knowledge, sparse enums,
IRQ holes, cross-chip reset aggregation, input order and conflicting shared
identities. These probes establish code generation and memory operations, not
real device side effects. See [complete validation](validation-v0.14.0.md).
