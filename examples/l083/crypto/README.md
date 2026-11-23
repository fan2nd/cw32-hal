# L083 hardware-word AES and raw TRNG examples

Build either binary for an exact package; for example:

```
cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32l083rct6 --bin aes-blocks
cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32l083rct6 --bin trng-samples
```

`build.rs` derives the linker memory map from the exact package metadata.
`aes-blocks` uses all three key sizes and both hardware directions, exposing
results to a debugger through `black_box`. It only demonstrates a round-trip.
`trng-samples` returns raw unassessed hardware words; errors never yield a sample.
Neither example is a cryptographic protocol, entropy certification, known-answer
validation, test harness or software substitute for the peripheral. No flashing
or hardware execution is part of this delivery.
