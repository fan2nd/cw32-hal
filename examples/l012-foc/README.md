# CW32L012 JD4631 FOC example

This example ports the supplied C project to the current Embassy-style HAL. It
uses the L012 buffered ATIM complementary PWM owner, blocking ADC1/ADC2 reads,
blocking SPI3 for the SSI encoder, the CORDIC for sine/cosine, and a BTIM1
overflow as the control-loop tick. The six PWM routes match the supplied board:

| Function | Pin |
| --- | --- |
| ATIM CH1/CH1N | PB5 / PA15 |
| ATIM CH2/CH2N | PB6 / PB3 |
| ATIM CH3/CH3N | PB7 / PB4 |
| ADC1 current A/B | PB0 / PB1 |
| ADC2 bus voltage / speed pot | PA8 / PA10 |
| MT6701 SSI SCK/MISO/CS | PB8 / PC14 / PF0 |

The motor constants are taken from `JD-4621.jpg`: 22 pole pairs, 6.7 Ω phase
resistance, 1.75 mH phase inductance, 12 V nominal bus and 1.1 A rated current.
The image names a 5048A encoder; the supplied C project uses a 14-bit MT6701
SSI frame, so the example keeps the encoder electrical protocol in the board
layer and uses the 14-bit frame currently present in the project.

The bridge is held off for ADC offset calibration and at power-on. The firmware
will not align or enable the bridge until the speed potentiometer on ADC2 PA10
has stayed at zero for 100 ms. After that arming step, the potentiometer maps to
0..800 rpm and a 1 kHz speed PI generates the torque-current command; returning
the potentiometer to zero commands a stop. Immediately after alignment, the
firmware applies a short, low-current positive-Iq probe and uses the encoder
angle delta to compensate motor/encoder direction automatically. The ADC
current scale and amplifier gain must be calibrated for the actual shunt/OPA
circuit before applying power.

Build without flashing or executing hardware:

```sh
rustc --edition 2024 --test examples/l012-foc/src/foc.rs -o /tmp/cw32-foc-math-tests \
  && /tmp/cw32-foc-math-tests
cargo build --manifest-path examples/l012-foc/Cargo.toml --release \
  --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6 --bin foc --offline
```
