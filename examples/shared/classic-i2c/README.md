# Classic I2C master examples

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

`async_master` uses the controller's interrupt-driven seven-bit master API.
`blocking_master` performs the same ordinary transfers with the blocking API.
Each binary owns and uses every classic I2C controller on the selected package.
L012 has a different controller and is not an example feature here.

## Wire a real target

The example board contract is a powered I2C target at unshifted seven-bit address
`0x48` on each selected bus. Its register pointer is one byte, `0x00`, and that
register returns two bytes. A pointer-only write must select that register without
requiring a write-cycle delay. A following read and a repeated-START read must
both be supported by the target. Set `TARGET_ADDRESS`, `REGISTER_POINTER` and the
read lengths in the source to match your actual target's datasheet before use.
These constants are example board choices, not detected or measured values.

Connect the listed SCL/SDA pair, a common ground and appropriate target power.
Add external pull-ups from both lines to a voltage legal for the MCU and target;
choose their resistance for the actual supply, capacitance and required rise time.
The default configuration requests a 100 kHz SCL ceiling and leaves weak internal
pull-ups disabled. A dual-controller package needs a target on each of its two
separate buses; using the same address on these separate buses is intentional.
Do not connect the two buses together.

| Exact chip feature | Controller | SCL / SDA | AF | Package pin positions |
| --- | --- | --- | --- | --- |
| `cw32f030c8t7` | I2C1; I2C2 | PA9 / PA10; PA1 / PA2 | 3 | 30 / 31; 11 / 12 |
| `cw32a030c8t7` | I2C1; I2C2 | PA9 / PA10; PA1 / PA2 | 3 | 30 / 31; 11 / 12 |
| `cw32f020c6u7` | I2C1; I2C2 | PA9 / PA10; PA1 / PA2 | 3 | 30 / 31; 11 / 12 |
| `cw32f002f3p7` | I2C | PB1 / PA6 | 3 | 6 / 19 |
| `cw32f003e4p7` | I2C | PB1 / PA6 | 3 | 8 / 23 |
| `cw32l010f8p6` | I2C1 | PA1 / PA0 | 2 | 6 / 5 |
| `cw32l011k8t6` | I2C | PB6 / PB7 | 3 | 29 / 30 |
| `cw32l031c8t6` | I2C1 | PA9 / PA10 | 3 | 30 / 31 |
| `cw32r031c8u6` | I2C1 | PA9 / PA10 | 3 | 33 / 34 |
| `cw32l052c8t6` | I2C1; I2C2 | PA9 / PA10; PA1 / PA2 | 3 | 30 / 31; 11 / 12 |
| `cw32l083rct6` | I2C1; I2C2 | PA9 / PA10; PA1 / PA2 | 3 | 42 / 43; 15 / 16 |

The routes use the selected package's bonded pins. The singleton names `I2C` and
`I2C1` are the actual HAL peripheral names for the respective chip, not aliases
for an extra controller. Async bindings use the matching dedicated I2C vector.

## Read sequence

Each iteration performs these operations on the first bus, then the second bus
where present:

1. Write the register pointer, complete STOP, then read two bytes in a separate
   transaction.
2. Call `write_read` to select the register and read it through repeated START.
3. Call `transaction` with `Write`, `Read` and adjacent `Read` operations. The
   write-to-read change uses repeated START. Both reads share one read address
   phase; the first byte is ACKed and the final byte is NACKed before STOP.

`Readings` retains raw bytes for inspection at `black_box`; interpretation and
physical units depend on the target. A configuration or transfer error halts the
example without retrying already transferred bytes. Async SI waits have no
internal deadline; their progress depends on the actual hardware and interrupt.
The configuration's `poll_limit` bounds synchronous samples, not elapsed time.
These are ordinary hardware examples, with no on-board execution, electrical,
timing, cancellation or bus-recovery validation claim.

## Build

From the repository root, choose exactly one of the 12 chip features:

```sh
cargo build --offline --locked --release \
  --manifest-path examples/shared/classic-i2c/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32f030c8t7 --bin async_master

cargo build --offline --locked --release \
  --manifest-path examples/shared/classic-i2c/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32f002f3p7 --bin blocking_master
```

The package uses the existing pinned Embassy revision and a dependency lock
derived from the existing example lock. `build.rs` emits `memory.x` from the
selected package's own generated metadata and passes `-Tlink.x` explicitly.
For F002 this retains its actual 16 KiB FLASH and 2 KiB RAM limits. There is no
time-driver feature, substitute chip or enlarged memory map. The local Cargo
configuration supplies only the target default when run from this directory.
The commands above are usage instructions, not records of successful builds.

`retry_probe` permits two additional address-only probe attempts on AddressNack,
then reports the result and stays idle. The target must permit repeated probes.
It adds no backoff or bus recovery; the new path is uncompiled and untested.
See [explicit retry contract](../../../docs/i2c-explicit-retry.md).
