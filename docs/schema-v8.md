# Normalized schema v8: explicit DMA topology

Version 8 adds optional `dma` metadata to a peripheral instance. The maintained datasets attach it only to the DMA controller. It is preserved in normalized chip JSON and emitted unchanged in meaning by PAC metadata generation. Version 7 JSON must be regenerated; it is rejected rather than silently read without DMA topology.

## Fields

- `Peripheral.dma: Option<DmaController>`: `null` for non-DMA instances.
- `DmaController.channels`: ordered channel records, sorted by `index` during source loading.
- `DmaController.requests`: controller-wide routes, sorted by `selector` during source loading. Every listed route is available on every listed channel; this schema must not be used to imply uniformity on hardware with a restricted mux.
- `DmaController.source: String`: nonempty authoritative topology/request evidence.
- `DmaChannel.peripheral: String`: existing channel register-view name, e.g. `DMACHANNEL1`.
- `DmaChannel.number: u8`: one-based hardware channel number.
- `DmaChannel.index: u8`: zero-based parent `CH` bank array index.
- `DmaChannel.interrupt: String`: physical external IRQ name. Multiple channels may share it.
- `DmaRequest.peripheral: String`: existing request-producing peripheral name.
- `DmaRequest.signal: String`: identifier naming its request signal, not a pin or necessarily a unique peripheral event.
- `DmaRequest.selector: u8`: raw `TRIG.HARDSRC` encoding, constrained by the modeled field width.

PAC metadata uses the same field names, `&'static str` instead of `String`, and `&'static [T]` instead of vectors. Request names in the HAL are generated from `<peripheral>_<signal>`. Neither metadata nor a request enum certifies an endpoint address, peripheral access width, request-enable sequence, transfer lifetime, or safe stopping.

## Validation

The schema requires a root DMA controller, nonempty evidence and tables, a `CH` subblock array and writable scalar HARDSRC selector. Channels must cover the bank exactly, with unique indices, hardware numbers and aliases. Each alias must refer to the declared controller, matching subblock version and address. Its IRQ must agree with both its alias binding and the controller's bindings. TC/TE bit positions must match the channel's declared number/index. Omitted channel aliases are rejected.

Requests must refer to existing peripheral instances and have valid, collision-free generated identifiers, unique selector values and in-range encodings. Shared IRQs are allowed; duplicate request selectors are not. Variant-specific combined events are modeled as a single shared request instead of pretending they are independently selectable.

## Regeneration

Run `cargo run -p xtask -- regenerate` from the workspace root. The two-stage pipeline remains YAML → persisted normalized JSON → PAC and metadata; direct YAML-to-PAC shortcuts are not introduced. Use a fresh output directory when invoking `cw32-gen generate` directly after changing register definitions, because the generator deliberately rejects overwriting a different definition with the same kind/version in an existing publication set.

See [DMA hardware evidence](dma-hardware-evidence.md) for exact source pins, all channel/request tables, access discrepancies and the absence of a documented disable/drain guarantee.
