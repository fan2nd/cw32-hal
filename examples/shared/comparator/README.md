# External comparator polling example

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

Normal `no_std` Cortex-M0+ firmware, compiled and linked but not flashed or run.

Select exactly one package feature. This example declares a 3.3 V analog supply; adjust it and review the own-family electrical conditions before use. Apply analog inputs within ground and analog supply, with suitable overdrive and settling. `Comparator` owns VC1 and both pins until dropped.

| Package(s) | Positive | Negative |
|---|---|---|
| CW32F002F3P7, CW32F003E4P7 | PC2 | PB0 |
| CW32R031C8U6 | PA4 | PA5 |
| CW32L010F8P6 | PB5 | PA0 |
| CW32L011K8T6, CW32L012C8T6 | PA0 | PA2 |
| CW32F030C8T7, CW32A030C8T7, CW32F020C6U7, CW32L031C8T6, CW32L052C8T6, CW32L083MCT6 | PA0 | PA1 |

```
cargo build --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/comparator/Cargo.toml --no-default-features \
  --features cw32r031c8u6
```

R031's PA4/PA5 are source channels CH0/CH1 but use hardware mux4/mux5, selected through generated pin traits. L010/L011/L012 always report startup readiness `Unknown`; the loop exposes the observation for inspection without treating it as a settled control result. Their 0.5 µs typical startup is not a guarantee and no arbitrary delay is inserted. Other families use SR.READY, but a changed analog input still needs its propagation time. This example makes no interrupt, output-pin, divider, internal reference, or RF configuration.
