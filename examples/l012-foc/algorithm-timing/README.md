# Algorithm timing crate

This is an independent Cargo package. It runs the current-loop and speed-loop
kernels on the L012 and reports min/max/average execution time through RTT.
SysTick is configured as a 24-bit core-clock down-counter; at the 96 MHz
control clock the reported cycle counts are converted to microseconds.

```sh
cargo run --release --offline
```

The crate does not enable PWM, ADC, or the motor power stage.
