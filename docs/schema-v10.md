# Normalized schema 10

Schema 10 persists semantic enum evidence, regular/explicit field arrays and evidenced peripheral clock domains. Stage 1 writes normalized JSON; stage 2 reads and validates it independently. Schema 9 and unknown keys are rejected. See the [46-IP field audit](pac-fields-v0.18.0.md) for the implemented hardware coverage.

## Field arrays

`Field.array` accepts either `{len, stride}` or `{offsets: [...]}`. Offsets are relative to `bit_offset`; the list order defines index order and is never sorted. This is the same representation as register/subblock arrays, but the units are bits and the enclosing register width bounds the expansion. Explicit offsets support irregular and descending layouts.

Each indexed field requires `elements` and a nonempty `array_source`. Each element stores its source name, absolute bit offset, width, access, kind, values and description. The validator checks element count, unique source identities across a register, checked offset arithmetic, field bounds, overlaps (including overlaps inside an irregular array), width/access/kind/value equivalence, and nonempty evidence. Scalar fields cannot carry array elements/evidence. Mixed regular/explicit keys, empty arrays, duplicate offsets, zero stride and generated accessor/type collisions are rejected. Equal widths alone do not make unlike functions homogeneous.

PAC accessors have `field(n)` / `set_field(n, value)` forms. Both perform a bounds assertion before computing the shift. The bit setter changes only the indexed element and preserves all other bits in its value word. Metadata uses the same `Array::{Regular, Explicit}` representation plus `FieldElement` records; the persisted JSON retains all evidence and identities. Existing read-side effects, write policies, bus widths and default initialization remain register-level facts.

For a shared register array, element names describe the representative first register's fieldset. The outer `Register.elements` gives the individual register identities and sources. This prevents pretending CCR1–4 have the additional CCR5/6 grouping fields.

## Enum values

An enum field has `kind: enum`, a nonempty `values` list and a nonempty `values_source`. Values must fit the field width, with unique encoding and Rust variant names. User-supplied names beginning `_RESERVED_` and names colliding with conversion methods are rejected. Raw/boolean fields cannot have enum values or enum evidence.

Dense enum representations include every reserved encoding, use the smallest integer width, and have total masked `from_bits` plus `to_bits`. Sparse fields use the pinned chiptool transparent-newtype policy. Getters return the value directly rather than `Option`; setters accept the typed value and call `to_bits`. Pure fieldset setters do not alter the allowed MMIO direction or side-effect policy. Reserved patterns round-trip without asserting that hardware permits writing them.

## Peripheral clocks

`Peripheral.clock_tree` is optional and contains `bus_clock`, optional `kernel_clock`, and a required nonempty `source`. Clock sources are the closed snake-case enum `hclk` / `pclk`. Unknown fields or source names are rejected. Absent metadata means unmodeled; it never authorizes inference of a kernel clock from the location of an enable bit. Generated PAC metadata has matching `ClockTree` and `ClockSource` types.

DMA topology validation continues checking original TC/TE channel identities and their physical positions after grouping. A shape-valid array cannot silently change channel-to-interrupt mapping.
