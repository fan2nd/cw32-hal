# v0.17.0 resource-composition validation

2026-10-03. Rust/Cargo 1.99.0, `thumbv6m-none-eabi`.
This record covers the [first resource-composition stage](resource-composition-v0.17.0.md),
not completion of the remaining RCC/PWM/PAC/peripheral roadmap.

## Independent public contracts

- Compared actual pinned Embassy OPA output channels, DAC splitting and ADC DMA
  composition with the relevant primary CW32 manuals.
- 60 ARM API probes passed: 17 valid programs and 43 intended compiler failures.
  They cover both chips, real DMA IRQ bindings, blocking/async finite ADC DMA,
  mutable-source refill/return, static-buffer/input restrictions through forget,
  ADC/channel/sequence reuse, DAC pairing, sibling independence and OPA lifetime/
  calibration restrictions. Static OPA → ADC DMA compiles; a short OPA borrow
  cannot be used for DMA.
- 22 schema/build-generation probes passed: three baselines and 19 malformed
  cases. They cover evidence, DAC target/channel existence, old/unknown schema
  fields, OPA output ambiguity, internal ADC sources incorrectly labeled as pads,
  and absent DMA request routes. L012 pad channels are now bounded to 0–11,
  F030 to 0–12. All four OPA/ADC routes and six DAC consumer pairings match the
  sourced data.

## Register and lifecycle checks

- 15 source-bound analog scenarios passed: split has no register writes, live
  source updates affect only the selected holding register, invalid codes have
  no writes, channel Drop preserves sibling/reserved/gate state, attached pins
  are released only by their owner, OPA borrowing does not change GPIO, and
  failed/interrupted calibration settling keeps output unavailable.
- 36 source-bound ADC/DMA scenarios passed across both chips. They check exact
  source/request/width/count/mode, clean completion and resource reuse, errors
  including simultaneous TC+TE, cancellation/timeout/Drop, forgotten transfers,
  ADC owner Drop/reconstruction, controller re-split quarantine, old-hook removal
  before channel reuse, rejected launch ownership, and mutable-copy handback.
  L012 multi-slot BULK and F030 multi-slot rejection have separate checks.
- These use modeled register behavior around production source. They do not
  establish silicon bus-drain timing, analog settling limits or DMA arbitration.

## Aggregate ARM and source checks

- Formatting and actual YAML → persisted JSON → re-read PAC regeneration/drift.
- Both PACs with runtime/metadata; both HALs minimum/full ARM release.
- Both HALs minimum/full warnings-denied rustdoc.
- Seven optional features individually on each chip.
- All six board programs, plus 05/06 motor-output opt-in, ARM release.
- Two additional linked ARM executables exercise the new finite ADC DMA and
  mutable copy code; L012 additionally links DAC→OPA→ADC composition. All 32 IRQ
  vector slots per chip match metadata, active DMA/GTIM1 entries are strong
  handlers, and Flash/RAM/stack bounds pass.
- Every example source and the complete `motor/` source directory are byte-equal
  to v0.16.0. Only example package versions changed. Original sampling,
  commutation, protection logic and executor placement were not migrated again.
- Generated peripheral/register modules match v0.16.0 byte-for-byte. Schema9
  changes connection metadata, not register layouts/defaults/access semantics.

The source-only archive was extracted into a fresh directory with no generated
trees or Cargo lock. A new Cargo target passed regeneration/drift, both-chip
PAC/HAL minimum/full release and strict docs, all six programs and both motor
opt-in builds. The final archive's 232 source files match both the maintained
tree and this compiled clean tree byte-for-byte; the final change after the
build was this validation record. Generated JSON/PAC trees also match exactly.

All focused probe assets stay outside the maintained repository. No checked-in
tests, Python build dependency, extra platform cfg or generated artifacts were
added. No board was flashed and no physical motor/electrical behavior or IRQ
latency was measured. The existing `proc-macro-error2` future-incompatibility
notice remains; current compilation succeeds.
