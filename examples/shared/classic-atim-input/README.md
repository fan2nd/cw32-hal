# Native classic ATIM polling inputs

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

These ordinary firmware examples use independently qualified ATIM digital
inputs on F030, A030, F003, L031, R031, L052 and L083. They do not generate
signals, loop back outputs or enable capture IRQs/DMA. Connect voltage-compatible
external signals and observe the named debugger variables.

`polling_capture` owns and polls all six native halves in CH1A, CH1B, CH2A, CH2B,
CH3A, CH3B order. `CAPTURE_TICKS` holds the last observed latch for each half;
`CAPTURE_OVERWRITE` holds its separately read sticky overwrite indication. Each
half is disabled before reading/acknowledging the latch and flags, then enabled
again. Edges during that deliberate gap are lost; earlier edges may already
have overwritten the latch. A false overwrite flag does not prove lossless
capture. Counter timestamps are modulo 65536 and cannot count missed wraps.

The three capture register-IP representatives use these six distinct pads:

| Part | CH1A | CH1B | CH2A | CH2B | CH3A | CH3B | AF |
| --- | --- | --- | --- | --- | --- | --- | --- |
| F030C8T7 | PA5 (15) | PA7 (17) | PA4 (14) | PB0 (18) | PA3 (13) | PB1 (19) | 7 |
| F003E4P7 | PB6 (13) | PB5 (14) | PB3 (16) | PB2 (17) | PA0 (18) | PA1 (19) | 4 |
| L052C8T6 | PA5 (15) | PA7 (17) | PA4 (14) | PB0 (18) | PA3 (13) | PB1 (19) | 7 |

Parentheses give the physical package pin number. The other selected example
parts have the same six-half software loop; R031 uses its independently
qualified PA10 for CH3A because PA3 is radio-reserved. Consult the emitted
`external-input-routes.txt` for the actual selection. Full library package
capabilities can be narrower than these eight example parts; optional capture
slots do not grant an unbonded pin or CH4 external capture.

`quadrature` owns CH1A/CH1B only. `POSITION` is the modulo ARR+1 count,
`DIRECTION` is the separately read direction (0 up, 1 down) and `WRAP_FLAGS`
contains independently observed native OVF/UNDF bits. These observations are
non-atomic, coalesced and do not form an unbounded signed position.

| Part | CH1A / CH1B | Physical pins | AF |
| --- | --- | --- | --- |
| F030C8T7 / A030C8T7 / L031C8T6 / L052C8T6 | PA5 / PA7 | 15 / 17 | 7 |
| F003E4P7 | PB6 / PB5 | 13 / 14 | 4 |
| R031C8U6 | PA5 / PA7 | 18 / 20 | 7 |
| L083MCT6 | PA5 / PA7 | 25 / 27 | 7 |

The own manual requires phase separation strictly greater than one native TCLK
pulse width and each input pulse width strictly greater than two TCLK pulse
widths. QEI fixes PRS=/1. Nominal PCLK and clock bounds are not per-cycle timing
guarantees, and the native input filter is not generic switch debouncing.
Apply the selected chip's electrical conditions. No maximum external edge rate
or hardware measurement is asserted here.

```sh
cargo build --release --locked --offline --target thumbv6m-none-eabi \
  --no-default-features --features cw32f030c8t7 --bin polling_capture
cargo build --release --locked --offline --target thumbv6m-none-eabi \
  --no-default-features --features cw32f003e4p7,embassy-cw32/time-driver-gtim \
  --bin quadrature
```

The second command reserves F003's sole GTIM while native ATIM QEI remains
available. This polling program does not claim to exercise the global-time
service. Select exactly one manifest part feature; F030C8T7 is the default.
Build scripts require the exact memory map and validate native input signal,
AF, distinct bonded pads and actual canonical GPIO token names. SDK spellings
such as PA00 are never inserted as Rust tokens. F003 PA5 keeps its existing
debug exclusion; its encoder pair is PB6/PB5.

The release profile uses `opt-level = "s"`, LTO, one codegen unit and no debug
information. Main/clean acceptance receipts are separate from these examples;
no board execution or silicon timing qualification is claimed. See the
[bounded timer event and input contract](../../../docs/timer-event-input.md).
