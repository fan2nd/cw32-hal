当前schema已升级为[v4](schema-v4.md)，加入审定reset defaults与实例覆盖；本文保留v3的其余模型说明。

# Normalized schema v3 and generated register contracts

The maintained input remains YAML. In v0.6.0, the single `cw32-gen` crate contains
both generation stages: `cw32_gen::data` writes schema-version 3 chip and register
JSON to disk; `cw32_gen::pac` reads those files back, validates version 3, and
renders Rust without loading YAML. The unified `cw32_gen::generate` entry point
always crosses that persisted JSON boundary; it does not pass the in-memory data
model directly to the renderer. This is a module boundary within one crate,
which includes the YAML dependency. The pure `cw32_gen::schema` module in
`cw32-gen/src/schema.rs` owns the model and shared validation; it is no longer a
separate package. `cw32_gen::schema::SCHEMA_VERSION` remains the single version
constant. Versions 1 and 2 JSON must be regenerated, not relabelled. Version 3 removes the Package model and stores chip pin capabilities directly, and supports 8/16/32-bit register bus transactions. Aliases, behaviors, clock/reset references and ownership relationships remain validated. The generated Rust metadata
also exposes `schema_version`.

## Chip identity and pins

`Chip.pins` is an explicit required list of `{name, port, number}` capabilities. It may be empty in a register-only synthetic fixture. Real chip golden tests require the complete audited set. There is no Package struct, package selector, package pad number or derived package intersection in source YAML, normalized JSON or generated metadata. Routes must refer to a declared chip pin and an existing peripheral.

Chip names and Cargo features are `cw32l012c8` and `cw32f030c8`, without temperature/package suffixes. Memory capacity remains chip-specific. Board software is responsible for checking physical bonding and wiring; the HAL token proves chip identity and ownership, not the PCB footprint. Legacy Package fields are rejected by strict deserialization and old JSON schema versions are rejected at the boundary.

## Register aliases

`Register.alias_of: Option<String>` names a canonical register in the same block.
It must be a distinct register with the same access direction; its byte range must be contained in the canonical register range, and the target must not itself be an alias. This permits explicitly declared overlapping width views such as CRC DR8/DR16/DR32 and GPIO high-byte access at offset +1. An unannotated overlap is still rejected. Different fields are allowed for different
hardware modes. I2C `SCR2`/`MCR2` and timer capture/compare mode views use this
contract. Unannotated overlapping byte ranges, missing targets, chains and cycles are
errors. Each named view still receives a PAC accessor and field namespace.

## Direction and side effects

Direction remains `access: rw | ro | wo` and controls reads/writes independently
of the following serialized enums:

- `write_behavior`: `ordinary`, `zero_to_clear`, `one_to_clear`, `one_to_set`,
  `toggle`, `command`, `keyed`, `mixed`, `write_once`.
- `read_behavior`: `ordinary`, `clear`, `fifo`, `latch`.

Both default to `ordinary` for existing source compatibility. This default means
no special behavior was annotated; it is not evidence that an arbitrary operation
is safe. Behavior annotations require hardware evidence. Read-only registers
cannot declare write behavior; write-only registers cannot declare read behavior.
Unknown enum spellings fail deserialization.

The PAC uses `Reg<Access, WriteBehavior, ReadBehavior, Width>`, with default `Ordinary`
markers and default `Width=u32` preserving existing `Reg<RW/RO/WO>` types. Read-clear uses the Rust marker
`ReadClear`. Only `Reg<RW, Ordinary, Ordinary>` has a typed `modify` method.
Zero/one-to-clear registers expose `clear(mask)`; the former writes `!mask`, and
the latter writes `mask`. One-to-set, toggle and command registers expose explicit
`set`, `toggle` and `command` operations. These operations, including ordinary
read/write/modify, remain **unsafe**: clocks, value validity, reserved bits,
ownership and synchronization remain caller obligations. In particular, keyed,
mixed and write-once semantics are described and typed, not magically enforced.
Raw address-level unsafe functions are escape hatches and must never be used to
bypass side-effect rules in safe HAL implementations.

Register `bit_size` is 8, 16 or 32 (default 32). The actual volatile pointer and returned/written value are respectively u8, u16 or u32: a narrow access is never implemented by reading a word and truncating it. Register alignment, alias byte extent and field range are checked against the selected width. Field, BoolField and EnumField descriptors use the matching integer width; they only transform values and perform no MMIO. A full
32-bit field uses `u32::MAX`, with no shifting by 32 or overflowing mask creation.
Generated identifiers such as the `REF` register use Rust raw identifiers when
lowercased (`r#ref`).

## Instance relationships

`Peripheral.clock_gate` and `reset` are optional references with the shape
`{ peripheral, register, field, bit }`. They must resolve to a writable single-bit
field at exactly the declared offset. YAML `clock` is accepted as an input alias
for `clock_gate`; normalized JSON always uses `clock_gate`. Legacy GPIO
`clock_bit` is preserved and must agree when both forms are present.

Generated `RegisterBit` metadata also includes the resolved register `offset` and
`shared`. `shared` is true when multiple instances reference the same target
peripheral/register/bit. HAL drivers must not reset a shared group as though they
owned an individual instance. Shared gates must not be disabled while a sibling
may still be active. Keys and reset polarity are hardware-specific driver
requirements; the generic bit-reference schema does not infer them.

`ownership_parent` represents overlapping hardware views such as
`DMACHANNEL1..4` under `DMA`. The parent must exist, and cycles are rejected.
Absolute register-address collisions between different instances require an
explicit ancestor relationship; a common ancestor does not permit two siblings
to overlap. PAC views remain available through unsafe MMIO, but HAL generation
must not issue independent safe ownership tokens for the child views.

Every peripheral's Rust metadata points at a shared register slice containing
offsets, bus widths, access enums, behavior enums, aliases and field ranges. This is generated
from the same validated JSON as the PAC, not a second maintained chip table.

## Build boundary

Consumer PAC builds use only pre-generated Rust and have no generator dependency. Validation remains in the production schema and generator.
