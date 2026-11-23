# L012 hardware arithmetic firmware

These are standalone `no_std`, `no_main` Cortex-M0+ firmware examples, with the
runtime, single-core critical sections and memory map of an exact supported part.
Select `cw32l012c8t6` (LQFP48) or `cw32l012c8u6` (QFN48). No external wiring or
special clock setup is used; electrical assumptions are those of HAL defaults.

- `eau_integer`: quotient/remainder and unsigned integer magnitude calculations
- `cordic_fixed_point`: a Q1.31 waveform plus scaled CORDIC operations

Inspect local result variables in a debugger. Errors remain Results; busy state
halts the application rather than claiming recovery or aborting the accelerator.
A polling budget counts CSR reads and is not an elapsed-time deadline. At exact
unit-valued trigonometric endpoints the manual does not specify saturation or
rounding. This example makes no numerical-accuracy claim.

Build without executing hardware:

```
cargo build --manifest-path examples/l012/math/Cargo.toml --locked \
  --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6 --bins
```

No device execution was performed. See `docs/l012-math-evidence.md` for formats,
source conflicts, domain restrictions and verification results.

`cordic_async` instantiates all eight IRQ-backed `*_async` methods. Cancellation
masks IE but does not abort hardware; see [the async contract](../../../docs/cordic-async.md).
