# Passive RAM parity diagnostics

Select exactly one package feature and build for `thumbv6m-none-eabi`. The link
script comes from that exact part's reviewed memory map. For example:

```
cargo build --locked --target thumbv6m-none-eabi --no-default-features --features cw32f030c8t7
```

`RAM_PARITY_DIAGNOSTIC` retains the latest typed observation for a debugger.
The example does not enable an interrupt, acknowledge an error, scan SRAM,
reinitialize memory, inject a fault, or claim recovery from corrupted memory.
Normal Rust startup initializes its own data and stack; the observation is a
sequential register sample with the limitations documented by `ram::Status`.
This example is compiled and linked for verification, never flashed or run.
