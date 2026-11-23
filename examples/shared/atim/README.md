# ATIM main PWM and polling counter examples

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

These are complete Cortex-M0+ firmware examples; they have only been compiled
and linked, never flashed or executed on hardware. Select exactly one feature.
The build uses the exact-package metadata's qualified FLASH/RAM map.

```sh
cargo build --manifest-path examples/shared/atim/Cargo.toml --release \
  --target thumbv6m-none-eabi --no-default-features --features cw32l010f8p6
```

`main_pwm` outputs no more than 1 kHz under the qualified clock envelope, 25% active-high PWM. Selection
uses the qualified PCLK upper bound; its nominal frequency is intentionally lower
than the requested ceiling. The output is PB6/AF4 on CW32F003E4P7, PB4/AF7 on
CW32L010F8P6, and PA5/AF7 on the other supported packages below:

- CW32F030C8T7, CW32A030C8T7, CW32F003E4P7
- CW32L031C8T6, CW32R031C8U6
- CW32L052C8T6, CW32L083MCT6
- CW32L010F8P6, CW32L011K8T6, CW32L012C8T6

Classic ATIM exposes CH1A–3A via `new3`; L010/L011/L012 exposes main CH1–4 via
`new`. Complementary output, break protection and dead time are not provided.
Do not connect this example to a power stage. Check the actual board supply,
ambient temperature, external load/current ratings and pad bonding before use.
Defaults use the conservative RCC operating envelope, not measured clocks.

`polling_counter` uses a /64 prescaler and 65536-tick period with no pad routing.
Flags coalesce multiple overflows; this is not a lossless event counter or an
Embassy time driver. No interrupt or DMA is enabled. The executable intentionally
polls forever, like a typical minimal firmware main loop.
