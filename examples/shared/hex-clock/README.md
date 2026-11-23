# Direct digital HEX firmware

Real Cortex-M0+ UART/LED firmware for all five qualified exact packages:
CW32F002F3P7/F3U7 and CW32F003E4P7/F4P7/F4U7. `pb0` and `pb1` differ only in
which independent digital clock input is connected. Neither is a crystal mode.

Connect a continuous, already stable 24 MHz digital clock to PB0 (`pb0`) or PB1
(`pb1`), UART1 TX on PB2 to a compatible serial receiver (115200 baud, 8N1),
and an LED with suitable resistor to PA0. Meet the own package pin table.

The examples require a board-qualified complete actual frequency envelope of
23,999,280–24,000,720 Hz (nominal ±30 ppm) across VDD 3.0–3.6 V and ambient
−20–70°C. Adapt these declarations to your hardware. Include tolerance,
temperature, aging, loading, source supply and short-term cycle variation.
Meet all waveform limits simultaneously: 40–60% duty, at least 15 ns high and
low, at most 20 ns rise/fall, high 0.7×VDDIOx..VDDIOx and low VSS..0.3×VDDIOx,
and the own I/O ratings. A positive error at nominal 24 MHz needs Flash WAIT1.

The HAL reserves the configured HEX input and independently retains any active
AWT-selected HEX input for the whole boot. With active AWT on either input,
only an exact already-enabled/ready matching system HEX configuration is reused;
otherwise initialization rejects the request. Safe GPIO construction checks the
frozen reservation before pad writes. Raw PAC access can invalidate it.

There is no documented automatic fallback or runtime clock-loss recovery.
STABLE is a startup latch. If HEX stops after selection the CPU can stop and a
poll iteration budget no longer provides progress. Keep the clock present.
No runtime changes, PLL, DeepSleep entry/resume or flashing are demonstrated.
The busy-loop LED delay is visible activity, not a calibrated timebase.

Build from this directory, choosing one chip and binary:

```sh
cargo build --locked --release --target thumbv6m-none-eabi --no-default-features \
  --features cw32f002f3p7 --bin pb0
```
