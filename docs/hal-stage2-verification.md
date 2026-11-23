> Historical software-verification references: HAL tests and fixture harnesses cited below were deleted on 2026-10-08. Their reported results apply to the dated source snapshot only; source facts remain unchanged. See [current HAL layout and build scope](hal-production-layout.md).

# HAL stage 2 software verification

> Superseded for hardware use: later source review found a live-HSI-trim
> initialization defect and F020 PAC corrections. These historical test results
> do not establish hardware correctness. See `stage3-corrections.md`.

Verified on 2026-10-08 with `./ci/check-hal.sh`, exit status **0**.
[Complete regression log](verification-logs/hal-stage2.log).

This is the stage 2 source checkpoint. Its HAL declares RCC/time types, GPIO,
EXTI, CRC, UART, SPI, and low-level DMA. `time` contains frequency types, not an
Embassy time driver. ADC, I2C, and timer/PWM files present but not declared at this
checkpoint are **outside these results**. Later changes need their own rerun.

## Toolchain and pinned dependencies

- rustc 1.99.0, commit `b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`
- Cargo 1.99.0, commit `5f94df478` (2026-08-27)
- Host: `x86_64-unknown-linux-gnu`; embedded target: `thumbv6m-none-eabi`
- LLVM 23.1.1
- Embassy HAL internals, sync, and example executor:
  `f16efeffe37581092ec184718e6fdb1620393214`
- Example executor's pinned `unitrait` dependency:
  `bb96601b3d293fa27d1ca2891f6ea148e534e433`
- Root and standalone example dependency resolutions use their own Cargo.lock.

## Passed matrix

The script derives its chip list from `embassy-cw32/Cargo.toml`. All 12 current
selections passed:

- `cw32a030`, `cw32a030c8t7`
- `cw32f030`, `cw32f030c8`, `cw32f030c8t7`
- `cw32f030f6`, `cw32f030f6p7`
- `cw32f030f8`, `cw32f030f8v7`
- `cw32f030k8`, `cw32f030k8t7`, `cw32f030k8u7`

For **each** selection:

- 88 host unit tests passed.
- 2 interrupt-binding integration tests passed.
- 5 compile-fail documentation tests passed.
- Optimized `thumbv6m-none-eabi` library build with `rt` passed.

The legacy `cw32f030c8`-gated UART API integration test also passed. The other
selections instead have that test filtered out; they are not reported as having
run it.

For **both** `cw32f030c8t7` and `cw32a030c8t7`:

- Optimized target library build with `rt,defmt` passed.
- UART compile contracts passed: one positive control and seven intended Rust
  diagnostic failures.
- SPI compile contracts passed: one positive control and seven intended Rust
  diagnostic failures.

Host tests do not execute peripheral MMIO. Expected-failure contracts check the
specific diagnostic, and require a positive control first. Defmt builds verify
library compilation, not a logging transport or a defmt-enabled firmware link.

## Linked firmware

Both binaries in [`examples/f030/basic`](../examples/f030/basic/README.md) were fully
linked with the exact C8T7 64 KiB FLASH / 8 KiB SRAM map:

- `blocking`: 4,380 bytes FLASH and 24 bytes static RAM.
- `async_gpio_edge`: 2,824 bytes FLASH and 584 bytes static RAM.

These numbers exclude run-time stack use and are not a stack-safety measurement.
The automated ELF check confirmed ARM executable type, FLASH load placement,
RAM limits, the 48-word vector table at address zero, initial stack pointer
`0x20002000`, and matching reset vector/entry. The async binary's GPIOA vector
points to its real handler rather than `DefaultHandler`, and the linked code
contains the genuine Embassy executor. No `embassy-time` or time-driver package
is in the example resolution.

The example manifest enables the pinned executor's actual
`platform-cortex-m,executor-thread` features and Cortex-M's single-core
critical-section implementation. The async example wakes from a GPIO edge; no
polling executor, invented clock source, or DMA wrapper stands in for hardware.
Wiring, package lead numbers, build commands, bounce/missed-edge behavior, and
DMA cancellation limitations are documented alongside the examples.

## Scope and packaging notes

- The only warning in this checkpoint is an unused GPIO alternate open-drain
  helper reserved for subsequent I2C integration.
- A snapshot export initially omitted the example's nested Cargo configuration.
  Restoring `examples/f030/basic/.cargo/config.toml` fixed the export; the complete
  regression above then passed. Source archives must retain that file while
  excluding the repository-root Cargo cache. The script now checks its presence.
- `./d` was unchanged. These HAL results complement the separate data/PAC source,
  metadata, and generation checks; they do not replace or re-claim those runs.
- No firmware was flashed or run. No physical interrupt, bus waveform, timing,
  power, analog accuracy, DMA cancellation, or silicon-correctness claim follows
  from these host tests and target links.
