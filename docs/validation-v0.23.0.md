# v0.23.0 validation: direct DMA channel singletons

This release corrects DMA ownership granularity. `Peripherals` directly contains
four L012 or five F030 channel tokens from audited metadata. Controller reset
runs once in global initialization; channel construction never requests reset.
The whole-controller HAL token and split API are removed. See
[the migration and upstream comparison](dma-channel-tokens.md).

## Integrated builds

The maintained source passed:

- Rust formatting and Rust-only YAML → persisted JSON → reread JSON → PAC
  regeneration, followed by a byte-for-byte drift check.
- Real `thumbv6m-none-eabi` release builds of both PAC configurations with
  runtime/metadata, and both HAL configurations with minimum and full features.
- Minimum/full ARM documentation on both chips with `RUSTDOCFLAGS=-D warnings`.
- All fourteen individually selected optional-feature combinations across both
  chips: rt, memory-x, metadata, unstable-pac, defmt, time-driver-gtim1 and
  time-driver-any.
- All six board binaries linked in release mode, plus 05 and 06 with
  `motor-output-enable`.

A clean source extraction, initially without generated files or Cargo.lock,
passed regeneration/drift, both-chip minimum/full ARM and strict documentation
builds, and all six examples plus 05/06 motor opt-in again using a fresh target
directory. Its normalized JSON and PAC outputs match the maintained source byte
for byte.
Only this validation document was added after that build; no Rust or hardware
data changed afterward.

The existing third-party `proc-macro-error2` future-compatibility notice remains;
it is not a failed build or a newly introduced HAL warning.

## Focused ownership and interrupt verification

External checks were kept outside the source package.

- Eighteen runtime scenarios across the two DMA IPs executed the production DMA
  module, engine and hardware backend, actual generated channel bindings,
  extracted production ClockResource implementation and typed PAC values with
  the MMIO transport intercepted. They checked no reset or peer writes when a
  later channel is constructed, final clean drop/reacquisition, forgotten owner
  counts, persistent Busy/Poisoned state, quarantine pinning, endpoint retention
  and source-local shared-vector flag clearing.
- Eleven additional cases executed the actual global initialization control flow
  with instrumented RCC/DMA/time hooks. They checked ordering before token
  publication, repeat/reentrant/concurrent acquisition, pure validation and
  timebase rejection, and failed hardware initialization. At most one startup
  DMA reset and one singleton set were observed. These hooks are a control-flow
  model, not an oscillator or silicon execution test.
- Two real ARM positive API builds covered every direct channel field, Blocking
  and Async modes, reborrowed tokens, typed finite ADC/UART TX/UART RX/SPI DMA
  endpoints and resource return/reuse, plus multi-handler shared IRQ bindings.
- Forty-eight isolated negative ARM checks were rejected for their intended
  ownership or trait reason: moved tokens, escaping reborrows, missing/wrong
  vector or peer Binding, removed controller/split APIs, private startup access,
  nonstatic buffers and endpoint-resource aliasing.

- Two release ARM ELF images linked successfully. All 32 device-vector slots
  match generated metadata; stack/Flash/RAM bounds and strong DMA symbols were
  checked. Decoded Thumb call targets confirm DMACH12 dispatches exactly L012
  channels 1/2 and DMACH34 dispatches 3/4; F030 DMACH1 dispatches channel 1,
  DMACH23 dispatches 2/3 and DMACH45 dispatches 4/5. No reserved Flash-page
  overlap or extra DMA peer dispatch was found.

## Scope and preservation

All 57 hardware-data source files, 48 normalized JSON files and 102 generated
PAC/metadata/runtime files are byte-identical to v0.22.0. The existing schema11
channel topology supplies the direct singleton names, actual controller gate and
reset, per-channel IRQ identities and typed request associations. No register
layout, request selector, reset value or new alias is introduced.

Across the 29 motor/example Rust files, 24 are byte-identical. The five changed
main files only remove the `dma::split` acquisition and replace `channels.ch2`
with the corresponding direct `Peripherals.DMACHANNEL2` token. ADC setup,
DMA transfer programming, algorithms, fault logic, synchronous ISR bodies and
P1 motor/thread UI executor organization are preserved. DMA reset now happens
inside global initialization. All six examples continue to contain no direct
PAC accesses.

The source package contains no generated outputs, Cargo.lock, targets, external
probes, Python project or checked-in test scaffolding. No silicon, electrical
clock, bus-drain or physical motor validation is claimed. The existing static
buffer/quarantine boundary, limited finite peripheral DMA endpoints and
runtime-clock/DeepSleep limitations remain documented.
