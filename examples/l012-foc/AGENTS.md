# L012 FOC project constraints

- This is an independent Cargo project; do not turn it into a Cargo workspace.
- FOC-related firmware code under `src/` must not use 64-bit integer types or
  casts (`i64`, `u64`, `as i64`, `as u64`). Keep fixed-point and timing
  arithmetic within the MCU's 32-bit integer range, using staged quotient and
  remainder operations where a direct product would overflow.
- Keep the current-sense front end enabled before reading ADC1: OPA1 maps
  PA6/PA7 to PB0 and OPA2 maps PA4/PA5 to PB1, matching the reference C board.
- Validate changes with the project's `cargo run` runner when hardware is
  connected; do not manually invoke `probe-rs download` or `probe-rs attach`.
- Do not build or test this PAC-backed firmware for a host target. Use the
  configured `thumbv6m-none-eabi` target and `cargo run` for hardware checks;
  host builds fail on the ARM-only vector-table section by design.
