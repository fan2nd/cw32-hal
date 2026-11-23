# Algorithm timing crate

This is an independent Cargo package. It runs the current-loop and speed-loop
kernels on the L012 and reports min/max/average execution time through RTT. It
also measures the shared Q1.15 sine/cosine lookup and compares ordinary
`u32` division with the EAU integer divider.
SysTick is configured as a 24-bit core-clock down-counter; at the 96 MHz
control clock of the 5 V motor-control board it reports elapsed core cycles.
An EAU error is reported through RTT and ends only the EAU sampling. Its
statistics cover successful samples; the other measurements continue.

```sh
cargo run --release --offline
```

The crate does not enable PWM, ADC, or the motor power stage. EAU is accessed
through its non-owning shared accelerator API.

The HFI benchmark exercises the 06 scan, carrier tracker, current PI and SVPWM
with synthetic phase currents. Group 5 combines the speed PI with every sixteenth tracking tick for comparison; 06 runs the speed PI separately in BTIM1. SysTick brackets the
arithmetic and the compiler barriers. Sample generation, RTT, ADC interrupt
entry, bridge protection and timer register writes are outside the interval.

Historical measurements below used the 2 kHz carrier and speed loop.

| RTT group | Computation | Samples | Mean cycles | Max cycles |
| --- | --- | ---: | ---: | ---: |
| 0 | Ordinary scan tick | 58368 | 1265 | 1359 |
| 1 | Scan baseline or pulse endpoint | 3064 | 1481 | 1586 |
| 2 | Final scan tick including axis and polarity detection | 8 | 3563 | 3645 |
| 3 | Tracking without demodulation or speed PI (includes positive peak capture) | 3072 | 2661 | 2821 |
| 4 | Tracking with demodulation and PLL correction | 512 | 3150 | 3554 |
| 5 | Tracking with speed PI | 512 | 5109 | 5138 |
| 6 | Same samples as group 4, PLL division using EAU | 512 | 3209 | 3209 |

Measured on CW32L012 at 96 MHz with the release profile. A 16 kHz period
allows 6000 cycles. These maxima apply to the sampled inputs.

Before replacing scan index divisions with counters and an angle table, the final scan tick peaked at 6601 cycles. Groups 3–5 use software PLL division. Group 6 repeats the trajectory with hardware division and reports only the demodulation tick.
