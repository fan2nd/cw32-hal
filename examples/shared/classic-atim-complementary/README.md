# Classic F030/A030 complementary PWM

Real Cortex-M0 firmware producing 1kHz complementary references with a requested
minimum 1000ns symmetric dead time. It owns PA5/PA7 (CH1) and PA3/PB1 (CH3).
Larger packages also own PA4/PB0 (CH2). F030 TSSOP20/QFN20 have no CH2B and use
only CH1+CH3. All supplied pairs connect while MOE is zero, then the application
sets interior comparisons and explicitly enables MOE.

Build, for example:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/classic-atim-complementary/Cargo.toml \
  --no-default-features --features cw32f030f8v7
```

The build script derives FLASH/RAM from the selected exact-package metadata.
The six chip features cover every F030/A030 exact package in this repository.
Use a logic analyzer on a suitable development board, without an attached power
stage. This linkable example has not been run on silicon and provides no brake,
shutdown, post-disable voltage or board-safety guarantee. See the
[classic API scope](../../../docs/classic-atim-complementary-pwm.md).
