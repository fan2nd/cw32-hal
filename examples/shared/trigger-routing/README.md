# CW32L010/L011 timer cascade and triggered ADC examples

These are standalone Cortex-M0+ firmware programs for CW32L010F8P6,
CW32L010F8U6, CW32L010Y8M6, CW32L011K8T6 and CW32L011K8U6. Build and link them with:

```sh
cargo build --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/trigger-routing/Cargo.toml \
  --no-default-features --features cw32l010f8p6
```

- `cascade`: BTIM1 UPDATE counts into BTIM2, wrapping every 100 updates. No wiring
  is required. Inspect the count with a debugger; overflow indications coalesce.
- `triggered_conversion`: one BTIM1 UPDATE starts one external-channel PA0 ADC
  conversion. Connect a board-safe voltage in 0..=VDD referenced to board ground.
  The default ADC configuration assumes only the minimum guaranteed supply and
  keeps the existing qualified clock/acquisition checks. Each completed result
  is available for debugger inspection before the next explicit arm.

The build script selects the exact package's generated memory map. These programs
have been compiled/linked, not executed or flashed. Timer periods are nominal
PCLK intervals, with startup/synchronization latency. The ADC example is polling,
not a lossless periodic stream. Cancellation/timeout permanently disarms an ADC
route; it cannot be immediately rearmed on an assumed conversion-abort guarantee.
See the [L010 route qualification](../../../docs/trigger-routing-l010.md) and
[own-family L011 qualification](../../../docs/trigger-routing-l011.md).
Select `cw32l011k8t6` or `cw32l011k8u6` to link for either L011 package.
