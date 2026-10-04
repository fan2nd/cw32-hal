# v0.23.1 validation: concise DMA channel token names

The public HAL tokens are now `DMA_CH1` through `DMA_CH4` on L012 and through
`DMA_CH5` on F030. Both `Peripherals` fields and `peripherals` types use this
spelling, including channel/interrupt implementations, shared clock traits,
startup reset and UART/SPI DMA request bounds. The HAL generator derives it
from the audited controller name and one-based channel number. No old-name
HAL aliases are exposed. See [the API migration](dma-channel-tokens.md).

## Builds and API verification

The maintained source passed:

- Rust formatting, explicit Rust-only YAML → persisted JSON → reread JSON → PAC
  regeneration, and a byte-for-byte generated-output drift check.
- Real `thumbv6m-none-eabi` release builds of both PAC configurations with
  runtime/metadata, and both HAL configurations with minimum and full features.
- Minimum/full ARM documentation for both chips with `RUSTDOCFLAGS=-D warnings`.
- All fourteen individually selected optional-feature combinations across both
  chips: rt, memory-x, metadata, unstable-pac, defmt, time-driver-gtim1 and
  time-driver-any.
- All six board binaries linked in release mode, plus 05 and 06 with
  `motor-output-enable`.

A clean source-only ZIP extraction, initially without generated outputs or
Cargo.lock, passed regeneration/drift, both-chip minimum/full release and strict
documentation builds, and all six examples plus both motor opt-ins using a
fresh target directory. Its normalized JSON and PAC outputs match the maintained
source byte for byte. Only this validation report was added after that rebuild;
no Rust, hardware data or other source changed afterward.

Independent external ARM probes passed for both chips:

- Every direct channel token supports blocking and correctly bound async
  construction, including reborrows and existing finite DMA endpoint APIs.
- All 98 UART/SPI channel-direction trait associations have their audited
  request selector, and ADC transfer construction compiles for every channel.
- All 48 prior ownership/lifetime/Binding negative cases are rejected for their
  intended reason. All 18 old `DMACHANNELn` field/type spellings are rejected;
  L012 also rejects the nonexistent `DMA_CH5`.
- Both release ELF images link. Their device vectors match metadata, and
  decoded Thumb handler calls dispatch precisely L012 channels 1/2 and 3/4,
  and F030 channels 1, 2/3 and 4/5 on their respective physical IRQs.
- Reexecuting the old and current HAL build scripts produces 21 outputs per
  chip that are byte-identical after only the intended token-name substitution.

The pre-existing third-party `proc-macro-error2` future-compatibility notice
remains. No new HAL warning is introduced.

## Scope and preservation

All 57 hardware-data files, 12 pinned vendor-evidence files, 48 normalized JSON
outputs, 102 generated PAC/metadata/runtime files and 92 HAL runtime source
files are byte-identical to v0.23.0. The hardware/PAC `DMACHANNELn` register-view
identities remain unchanged, as do addresses, channel counts/indices, request
selectors, physical IRQs, ownership parents and reset data.

Among 23 example Rust files, only six change, solely replacing the public token
spelling. Motor algorithms, ISR bodies, resource ownership and executor
organization are unchanged. Global initialization still performs the one DMA
reset; channel constructors do not reset their siblings. State persistence,
static-buffer ownership, shared-vector isolation and quarantine contracts are
unchanged. Earlier behavioral checks remain documented in
[v0.23.0 validation](validation-v0.23.0.md).

The source package excludes generated outputs, Cargo.lock, target directories,
external probes, Python files and test scaffolding. No hardware, motor,
electrical timing or bus-drain validation is claimed.
