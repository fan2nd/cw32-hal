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
