# embassy-cw32

Experimental CW32 data, generated PAC and Embassy-style HAL workspace.
Support is qualified by chip, package and operating mode; PAC register presence
does not imply a complete HAL driver. CW32W031/CW32W031R8U6 are unsupported and
are not aliases for L031.

## Workspace

- `cw32-data/`: curated versioned register YAML, chip/source manifests, verified
  part facts and routing/electrical YAML, acquisition and validation tools
- `sources/`: canonical source/version/hash catalog; downloaded originals in ignored `sources/vendor/`
- `cw32-data-serde/`: upstream-shaped chip metadata schema
- `cw32-data-macros/`: independently implemented metadata proc macro with the
  required upstream-compatible behavior and recorded predecessor provenance
- `cw32-data-gen/`: verified-input SVD import and normalization with chiptool
- `cw32-metapac-gen/`: actual upstream-derived chiptool PAC renderer
- `cw32-metapac/`: generated PAC, per-chip features, static metadata, vectors
- `embassy-cw32/`: ownership, type-level interrupts, and implemented HAL drivers

- `tools/`: capability declarations, source audits and source packaging.
- [`examples/`](examples/README.md): firmware examples with per-package pin and memory configuration.

Authored hardware YAML is authoritative. Vendor imports produce review
candidates rather than replacing it. The root Cargo workspace contains schema
and generators; `firmware/Cargo.toml` contains HAL/PAC. `cw32-metapac/` is generated.

## Scope

See [capability declarations](tools/hal-capabilities.yaml),
[inventory format](docs/functional-coverage.md) and [pending work](todo.md).

- UART: blocking/IRQ, buffered receive and qualified finite DMA.
  L010/L011/L012 support nine-bit PIO, single-wire half duplex and automatic RS485 DE. L012 UART DMA
  remains TX-only. Software checks are recorded in [validation scope](docs/validation-scope.md).
- Buffered UART on L010/L011/L012 returns an unread-queue position and error
  snapshot on idle, with optional four-marker continuous capture and explicit
  overflow. This does not identify wire packet boundaries.
- SPI/I²C: qualified blocking/async master modes; SPI DMA is byte-only.
- ADC/timers: finite ADC scans, qualified capture, overflow, PWM and trigger routes.
- Window-watchdog warning notification is implemented for all ten supported
  WWDT families; hardware warning delivery remains unverified.
- RCC/RTC and analog/math peripherals: only documented sources, modes and limits.
- `BufferedUartRx::into_buffer` returns storage after RX shutdown;
  `try_replace_buffer` swaps an empty running ring. Both are
  [covered by software acceptance](docs/uart-buffer-reclaim.md).

DMA cancellation may retain resources. Generic chip aliases may lack exact
memory or package qualification. Hardware results, timing, analog accuracy and
cryptographic properties require their own evidence. Low-power and RF work is
deferred.

## Development

No CI is configured. Local tests, builds and data/PAC generation are authorized.
See [validation scope](docs/validation-scope.md).

Source tooling requires Python 3.11+, Bash and a Rust toolchain; firmware targets
`thumbv6m-none-eabi`. Dependencies are pinned. `./d fetch-sources` acquires pinned
inputs; `./d gen-all` generates data and PAC. Generation invokes Cargo.
Direct data-generator calls should pass
`--root /path/to/workspace` explicitly.

Use exact part features for package routes and memory limits. Fallible HAL
constructors return `Result`. Firmware examples provide their own build/linker
configuration. Generated data and successful compilation are separate milestones.

## Sources and licensing

Project-authored Rust uses MIT OR Apache-2.0. Upstream and vendor permissions are
component-specific; see [NOTICE](NOTICE.md), [source catalog](sources/README.md)
and [file provenance](docs/upstream-file-provenance.json). Vendor archives, PDFs
and SVDs are excluded from source packages except explicitly approved members.
