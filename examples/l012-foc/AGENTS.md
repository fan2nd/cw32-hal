# L012 FOC project constraints

- This directory contains independent Cargo projects; do not turn it into a
  Cargo workspace. The complete speed-closed-loop firmware is in
  `speed-loop/`, shared algorithms are in `foc-algorithms/`, and the timing
  probe is in `algorithm-timing/`.
- FOC-related firmware code under `speed-loop/src/` should keep stored state and the fast
  ISR path within the MCU's 32-bit integer range.  A 64-bit intermediate is
  allowed when the fixed-point math requires it, such as a Q32×Q32 product or
  the PID integral accumulator before a 32-bit clamp; do not use 64-bit state
  merely to avoid sizing the fixed-point operation correctly.
- Keep the current-sense front end enabled before reading ADC1: OPA1 maps
  PA6/PA7 to PB0 and OPA2 maps PA4/PA5 to PB1, matching the reference C board.
- Validate changes with the project's `cargo run` runner when hardware is
  connected; do not manually invoke `probe-rs download` or `probe-rs attach`.
- Do not build or test this PAC-backed firmware for a host target. Use the
  configured `thumbv6m-none-eabi` target and `cargo run` for hardware checks;
  host builds fail on the ARM-only vector-table section by design.
