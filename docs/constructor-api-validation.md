# Constructor API validation

Validated on 2026-10-14 against the constructor cleanup based on
`8fb182098892e4547c51e7c9856b9061161fdf43`.
See [the API migration](constructor-api-migration.md) for the breaking changes.
No firmware was executed and no hardware behavior is newly qualified.

## Environment and source checks

- Rust 1.99.0 (`b940084d7`, 2026-09-28), Cargo 1.99.0,
  rustfmt 1.10.0; target `thumbv6m-none-eabi`.
- Pinned official vendor inputs were acquired and `./d gen-all` passed,
  including the canonical source-lock check.
- Changed Rust sources were formatted. `git diff --check` passed.
- A one-off comparison of the 17 changed HAL source files checked 723 retained
  function signatures/bodies against the base commit, normalizing only the
  constructor identifier mapping, whitespace and trailing commas. Their
  implementation bodies are unchanged; 29 old infallible wrappers/aliases
  were removed and 33 public fallible constructors renamed. This source check
  is not a hardware test or a replacement for compilation.

## ARM compilation and real example links

`bash ci/check-hal.sh --matrix-only` passed with exit status 0: **113 ARM
release-library build commands**. This includes all 54 declared chip features
with `rt`, then `rt,defmt` (108 builds), plus five selected GTIM1 time-driver
configurations. The five extra selections are L083MCT6, L052C8T6, L010F8P6,
L011K8T6 and L012C8T6. Existing warnings were limited to unused
`new_admitted_pair` and three generated `RCC_LSE_SYSCLK_*` constants.

This matrix builds libraries only; it is separate from the real firmware
link commands below.

ADC/SPI/I2C firmware: 40 of 41 build commands passed. Each command used:

```text
cargo build --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/<group>/Cargo.toml \
  --no-default-features --features <selection>
```

The selections were:

| Example group | Selections | Result |
| --- | --- | --- |
| adc-scan | L010F8P6, L011K8T6; `async` | 2/2 |
| classic-adc-scan | F030C8T7, A030C8T7, F020C6U7, F002F3P7, F003E4P7, L031C8T6, R031C8U6, W031R8U6, L052C8T6, L083RCT6; `async-gtim` for F002/F003, `async-gtim1` for the others | 9/10; see FLASH limit below |
| l012-adc-scan | L012C8T6, L012C8U6; `async` | 2/2 |
| spi-async | F020C6U7, F002F3P7, F003E4P7, L010F8P6, L011K8T6, L012C8T6, L012C8U6, L031C8T6, R031C8U6, W031R8U6, L052C8T6 | 11/11 |
| classic-i2c | F030C8T7, A030C8T7, F020C6U7, F002F3P7, F003E4P7, L010F8P6, L011K8T6, L031C8T6, R031C8U6, W031R8U6, L052C8T6, L083RCT6 | 12/12 |
| l012-i2c | L012C8T6, L012C8U6 | 2/2 |
| l012-spi-dma | L012C8T6, L012C8U6; includes mixed PIO/DMA constructor callers | 2/2 |

Chip feature names are the lowercase part names prefixed with `cw32`.
Counts above are Cargo invocations, not binary counts. This is a focused
example selection, not the repository's entire example matrix.

Timer/PWM/capture/QEI/CRC/trigger and F030 application validation passed 14
additional build commands:

- `atim`: F030C8T7 and L012C8T6.
- `atim-complementary`: L012C8T6.
- `classic-atim-complementary`, `classic-timer-input`, `classic-atim-input`,
  and `crc-checksum`: F030C8T7 each.
- `timer-input` and `timer-overflow`: L012C8T6 each.
- `low-power-timers` and `trigger-routing`: L010F8P6 each.
- `low-power-timers --bin btim_shared`: L012C8T6 and L052C8T6, each with
  `btim-shared`, covering both edited conditional branches.
- `examples/f030/basic`: one command builds `--bin blocking --bin lvd_ir_bursts`,
  from that directory so its linker configuration is loaded. An earlier run
  from the repository root lacked the linker script and is not counted.

The first eleven commands used the same `--bins` pattern as above. All edited
bins gated by `required-features` were included: the ADC async examples and
both `btim_shared` selections. Untouched optional examples were not newly
qualified by this check.

**Overall: 54 successful real-example build/link commands, one existing
FLASH-capacity failure, and one successful typecheck for that failing selection.**

### Pre-existing F002 example size limit

`classic-adc-scan` with `cw32f002f3p7,async-gtim` failed linking for two
binaries: `owned_ordered` and `async_single` exceed the FLASH region, with
final linker overflow reports of 1,024 and 2,784 bytes respectively. The same
command at the unchanged base commit, with the same toolchain and generated
PAC, reproduced both failures and all reported overflow sizes exactly.
This is not a newly introduced constructor type error. A separate `cargo check`
with the same `--bins`, target and feature selection passed for the modified
source, but it does not resolve the firmware size limit or count as a link pass.

## Host generator/data tests

- `cw32-data-gen`: 105/106 tests passed. The sole failure,
  `tests::normal_generation_preserves_and_uses_authored_yaml`, is an existing
  synthetic fixture missing `rtc-live-lsi-qualified.yaml`; an isolated test at
  the unchanged base commit reproduces it.
- The other three host workspace packages passed 27 unit tests and four
  compile-fail documentation tests.

The existing fixture was not altered as part of this API-only change. No HAL
unit-test harness, synthetic register model or new compile-contract fixture
was added. Host generator/data tests do not qualify peripheral behavior.

## Local receipts

Per-command outputs are local build products, not committed historical evidence:
`build/constructor-validation/`. Relevant files include `toolchain.txt`,
`generation.log`, `hal-matrix.log`, `serial-adc-examples.json`, the individual
example logs, `baseline-f002-adc.log`, `f002-adc-typecheck.log`,
`host-tests.log`, `host-baseline-test.log` and `host-other-tests.log`.
The timer/example summary and individual `example-*.log` files include the
corrected F030 link and both shared-timer branches. Host and matrix `.exit`
receipts preserve the reported process outcomes. Existing dated reports are
not overwritten by these new local results.
