# Hardware CRC checksum

Normal `no_std` Cortex-M0+ firmware. The example owns CRC, resets its selected
fixed preset, feeds `CW32 CRC`, and saves the result in `CRC_CHECKSUM` for debugger
inspection. It retains the driver while idling. There are no expected-result
assertions, software CRC implementation, register models, or test harnesses.

Select exactly one package feature:

- `cw32f002f3p7`: CCITT preset and byte payloads using documented word-sized stores
- `cw32f020c6u7`: CCITT preset with native byte, halfword and word transactions
- `cw32f030c8t7`: CRC32 preset with native byte, halfword and word transactions

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/crc-checksum/Cargo.toml --no-default-features \
  --features cw32f020c6u7
```

Linker memory bounds come from the selected exact-package PAC metadata.
Compilation and linking do not execute the firmware or validate the silicon.
