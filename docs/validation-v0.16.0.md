# v0.16.0 type-level interrupt validation

2026-10-03. Rust/Cargo 1.99.0, `thumbv6m-none-eabi`.
The [design and upstream comparison](typelevel-interrupts.md) identifies what
already matched Embassy, the actual motor/example gap, and intentional private
runtime entries. This is a wiring/API migration, not a new DMA algorithm.

## Source and contracts

- Compared actual fixed Embassy `b12a6d9` interrupt traits, binding macro,
  Cortex-M interrupt executor and generated time-driver entry.
- The HAL continues to use the official Interrupt/Handler/Binding definitions.
  No substitute registry, runtime callback layer or erased fake Binding was added.
- Motor ADC identity reuses `adc::Instance`; all three L012 BTIM identities and
  exact GLOBAL IRQ types are generated from existing metadata.
- Wrong IRQ, wrong same-IRQ handler, absent Binding, invalid peripheral identity,
  and omitted unsafe acquisition are rejected by the real motor API.
- All 13 former manual external vector bodies in 02–06 are equal to the new
  Handler bodies after normalizing only typed acquire spelling, whitespace,
  comments and one formatting-only trailing comma. Sampling/commutation still
  occurs before task notification.
- Time-driver counter/queue/IRQ algorithms and ordering are equal to v0.15.0
  after normalizing value-level versus type-level IRQ spelling. Its reserved
  private vector follows the actual upstream generated-time-driver pattern.
- Core exception and fatal handlers, control/protection/overzero algorithms,
  protocol/frame queue and UI source are unchanged.

## Focused independent checks

- Five positive ARM API clients passed: macro shared/cfg wiring, actual motor
  source enables, and F030 ADC, shared GPIOA PA0/PA1 and shared DMACH23 channel2/3
  bindings.
- Seventeen intended ARM compiler failures passed. Cases include unknown empty
  vectors, unknown vectors whose handlers are cfg-disabled, wrong Handler IRQ,
  wrong Binding vector, missing same-vector Handler proof, cfg-disabled Handler
  proof, motor instance/IRQ mismatches and unsafe acquire enforcement. Two
  additional F030 cases reject the L012 ADC2_DAC name and the wrong DMA vector.
- A host probe using the production `bind_interrupts!` macro calls shared handlers
  A then B synchronously, exactly once each on both invocations; a cfg-disabled
  handler is absent. ARM IR also contains direct ordered A/B calls.
- Independently built 02–06 release ELF files contain all 13 strong GLOBAL FUNC
  IRQ symbols, with vector words matching the metadata: ADC1=12, BTIM1=20,
  BTIM3_HALLTIM=22, UART2=28.
- Additional current-source DMA/time executables for both chips check all 32
  external vector slots, strong DMA/GTIM1 handlers and Flash/RAM/stack bounds.

## Aggregate build and clean package

All checks below passed on the final source.

- Formatting, two-stage regeneration and byte-for-byte drift checks.
- Both PACs, ARM release with runtime/metadata.
- Both HALs, minimum/full ARM release and minimum/full warnings-denied rustdoc.
- Seven optional features individually with each chip, 14 further checks.
- All six independent board programs, plus 05/06 output opt-in, ARM release.
- New source-only ZIP extraction and new Cargo target: regenerate, complete
  release/strict-doc/example matrix and exact source/generated-tree comparison.

The new channel and timer identities do not alter register data: the schema8
normalized JSON and generated PAC/metadata/runtime trees also match v0.15.0
byte-for-byte. Only HAL build-time motor bindings are added.

All focused verification assets remain outside the maintained source package.
No repository tests, Python build dependency, host target or new platform cfg was
added. No board was flashed; ISR latency, electrical behavior and physical motor
operation were not measured. Binding proves dispatch, not resource exclusivity,
source acknowledgment, bounded runtime or physical safety.
