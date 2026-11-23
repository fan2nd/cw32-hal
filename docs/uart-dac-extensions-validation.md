# L012 half-duplex UART and DAC interface validation

Date: 2026-10-14 (UTC). Baseline:
`67f280b057f2f52e259b6fafdee20a08c2296520`.
This batch covers L012 single-wire PIO UART, borrowed DAC channel views and
explicit DAC software-triggered loading. It does not qualify other UART
families, buffered/DMA half-duplex, bus arbitration, external DAC trigger routes,
DAC DMA or DAC interrupts.

## Build environment

The existing official Rust 1.99.0 toolchain, thumbv6m-none-eabi target, pinned
SDK/SVD inputs and generated PAC were reused. This receipt is independent of
prior batches; their compilation, host tests and UART Miri results are not
relabelled as tests of these changes.

## ARM compilation and links

All 113 ARM library build commands passed: the 54 existing chip selections
with runtime, the same 54 with runtime plus defmt, and five qualified GTIM1
time-driver selections (L083, L052, L010, L011 and L012). The matrix command
exited zero and printed its completion marker. Example counts below are
completed build commands, not hardware runs.

| Example group | Chip-selection build commands | Result |
|---|---:|---|
| UART half-duplex | 2 | pass |
| UART nine-bit regression | 2 | pass |
| UART buffered regression | 8 | pass |
| UART DMA regression | 21 | pass |
| UART DMA TX regression | 7 | pass |
| UART flow-control regression | 7 | pass |
| UART idle regression | 3 | pass |
| L012 analog/DAC, all bins | 2 | pass |
| L012 ADC2/DAC coexistence regression, async feature | 2 | pass |

All 54 accepted example build commands exited zero. These are 54 build commands, not 54 distinct chips or emitted ELF files.
Each invokes all bins for one exact chip selection with
`cargo build --locked --release --bins --target thumbv6m-none-eabi
--manifest-path examples/GROUP/Cargo.toml --no-default-features --features CHIP`.
The ADC2/DAC regression group additionally enables the existing `async`
feature so its two DAC-using async examples are actually compiled, rather than
skipped through required-features. Library validation uses `bash ci/check-hal.sh --matrix-only`.
No firmware was executed by these build commands.

## Actual production API probes

All 33 API checks met their exact expected outcomes: nine scoped UART probes,
twelve non-L012 absence probes and twelve DAC ownership/mode probes.

UART probes use the actual HAL, generated L012 PAC and Rust target:

- Blocking and async byte/u16 methods compile.
- UART1 with PA3 fails specifically because PA3 does not implement TxPin<UART1>
  (E0277). PA3 exists on the selected chip; this is not a missing-pin error.
- Missing typed IRQ binding fails specifically because unit does not implement
  Binding<UART1, InterruptHandler<UART1>> (E0277).
- split and split_ref are absent (E0599), the underlying uart field is private
  (E0616), and the owner does not implement Deref (E0277).
- Two live mutable futures are rejected (E0499); moving/dropping the owner before
  subsequent future use is rejected (E0505).
- Twelve other family representatives reject the HalfDuplexUart import with
  E0432 because the type is not exposed. These probes have no pins or peripheral
  constructor references, so their failures cannot be caused by a missing route.

The representatives are A030C8T7, F002F3P7, F003E4P7, F020C6U7, F030C8T7,
L010F8P6, L011K8T6, L031C8T6, L052C8T6, L083MCT6, R031C8U6 and W031R8U6.
The positive API target is L012C8T6; examples additionally cover L012C8U6.

DAC probes use the actual Dac and DacChannel types, not substitute Rust structs:

- Both optional channel views can be used independently, including across await;
  after their borrows end the parent can be reused and consumed into software
  trigger mode.
- Parent set/re-split while a proxy is later used fails with E0499.
- Parent drop or software-mode conversion while a proxy is later used fails
  with E0505.
- A proxy cannot escape its local owner (E0515), clone (E0599), or be forged by
  initializing its private owner marker (E0451).
- SoftwareTriggeredDac has neither direct set nor split (E0599), and its inner
  direct owner cannot be accessed (E0616).

All negative probes are checked for their intended diagnostic class, rather
than merely treating an arbitrary failed compilation as a successful test.
Compiler ownership checks do not execute registers or analog behavior.

## Actual-source host and structural checks

- Six UART wrapper-method tests passed using extracted production methods and
  a mocked ordinary Uart. They cover byte-mode rejection before flush, supported
  empty-read flush ordering, byte/u16 read ordering, unpolled futures and pending
  flush cancellation/retry. The mock does not execute the ordinary UART ISR,
  physical bus turnaround, GPIO or register-level receive logic.
- Four extracted configuration/DataBits tests passed for the L012 cfg set,
  covering parity/data width, byte-only restrictions, payload masks and Down
  pull rejection. The run specifies both uart_cw32l012_v1 and cw32l012. An initial
  harness invocation omitted the family cfg, so its Down-pull case failed; that
  invalid invocation was corrected and excluded from acceptance.
- Structural assertions on the actual half-duplex constructor check preflight
  before UART/GPIO changes, one owned pin, high latch before open-drain AF setup,
  no push-pull helper, both direction-state activations and SIGNAL configuration.
  These source assertions are not GPIO measurements or register execution.
- Six DAC source-extracted staging/trigger scenarios passed with modeled MMIO:
  invalid value and zero budget cause no writes; staging does not immediately
  change modeled output; trigger timeout preserves pending state; pending work
  rejects restaging/retriggering; waiting does not submit another command; and
  channel/mode guards reject invalid operations. Per-channel command selection
  is checked without replaying a sibling command.

The DAC harness extracts SoftwareTriggeredDac methods plus the actual channel
read/write helpers. It substitutes owner and register definitions and does not
exercise conversion or owner Drop. Preservation of the DMA-interrupt-disabled
condition during conversion/Drop was checked by source review, not proved by
that host model. Mock digital values do not validate real DAC alignment,
conversion accuracy, DHR-to-DOR latency or analog voltage.

No permanent HAL test framework was added. Temporary harness diagnostics from
unused stand-in methods are not product-library warnings. No Miri run is claimed
for this batch.

## Limits

No board was flashed. UART requires an external pull-up and application-controlled
turn-taking; arbitration, collisions, peer timing and lossless reception were
not tested or promised. A read can wait indefinitely for TX flush. DAC software
trigger polling is a register-read budget, not a time deadline or analog-settling
wait. No external timer trigger, DMA, underrun interrupt, waveform or electrical
qualification is asserted.

## Source inventory and formatting

`python3 ci/update-functional-coverage.py --validate-only` passed for
13 families, 458 peripheral functions and 2419 rows.
Inventory fingerprint: `466ce8dd645ef212ef569a20ab74a771351915285d9b03054b2198976159797a`.
This validates maintained source declarations, not runtime behavior.
`git diff --check` passed. Existing generated RCC_LSE_SYSCLK_* unused-constant
warnings remain; no warning-free claim is made.

## Final source and example identity

The full HAL and affected-example inventories were checked for drift across
acceptance. The source and example hashes below remained unchanged. Final
generation/provenance checks are performed after maintained documentation
closes; their separate publication gate does not require repeating compilation
when generated executable/data outputs remain identical.

| File | SHA-256 |
|---|---|
| `embassy-cw32/src/usart.rs` | `afd22f29ce426e57bbce9b61db86373b55f026ba4e3953981282670318753d87` |
| `embassy-cw32/src/dac.rs` | `d650942650a75ec9f06dbefd7c0963262ac3d5d4a0341dd7128f9718ef893aca` |
| `examples/shared/uart-half-duplex/src/bin/async_turns.rs` | `53297313bab153d1997c5e8263273d00a3c99fdf9cd7d410873af7f165d11e53` |
| `examples/shared/uart-half-duplex/src/bin/blocking.rs` | `f0dcc649253f63806ed7f51cbef4ec8d51d4bc1d837a26b3ed103400704a6d5d` |
| `examples/l012/analog/src/bin/dac_split.rs` | `ff148d4dd3ee184352a8a79ee252cf13186037cbe14c1ae8fd5089daab0a350f` |
| `examples/l012/analog/src/bin/dac_software_trigger.rs` | `d2ce2777f9f21c874cf8203216610f71ab88ff22d387175eaaa55b125c4a802b` |
