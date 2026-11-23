# L012 command/FIFO I2C master examples

`async_master` uses the ordinary owned seven-bit asynchronous API and real
`embedded_hal_async::i2c::I2c::transaction` trait. `blocking_master` performs the
same register reads through the ordinary blocking API. Each binary constructs
and uses both I2C1 and I2C2. Only exact CW32L012C8T6 and CW32L012C8U6 packages are
selectable here; both have 64 KiB Flash and 8 KiB RAM.

## Qualify the board before use

Wire a powered target at unshifted seven-bit address `0x48` on each separate bus.
The example assumes that a one-byte pointer-only write of `0x00` selects a
two-byte register without a write-cycle delay. The target must support both a
subsequent separate read and a repeated-START read. Change `TARGET_ADDRESS`,
`REGISTER_POINTER` and the read lengths to match the actual target's datasheet.
These are example assumptions, not detected hardware or measured properties.

Connect a common ground, appropriate target power, and external pull-ups from
SCL and SDA to a voltage legal for both MCU and target. Choose their resistance
for the actual voltage, capacitance, rise/fall times and noise environment.
Use one target on each of the two separate buses. The same address on both buses
is intentional; do not connect the buses together.

| Exact chip feature | Package | Controller | SCL / SDA | AF | Package pin positions |
| --- | --- | --- | --- | --- | --- |
| `cw32l012c8t6` | LQFP48 | I2C1 | PA4 / PA5 | 2 | 14 / 15 |
| `cw32l012c8t6` | LQFP48 | I2C2 | PB0 / PB1 | 8 | 18 / 19 |
| `cw32l012c8u6` | QFN48 | I2C1 | PA4 / PA5 | 2 | 14 / 15 |
| `cw32l012c8u6` | QFN48 | I2C2 | PB0 / PB1 | 8 | 18 / 19 |

The L012 datasheet's own package/AF tables and SDK establish these bonded routes.
They are admitted by the current safe generated pin interface. Async bindings
use the dedicated I2C1/IRQ23 and I2C2/IRQ24 vectors. These examples instantiate
these two routes per package, not every promoted L012 route.

`config()` requests a 100 kHz ceiling using only the frozen RCC PCLK envelope.
It declares worst-case SCL and SDA rise bounds of 100 ns, disables the digital
filters and leaves weak internal pull-ups off. The rise bounds require board
qualification and must be increased if either bus is slower. Disabling filters
does not promise spike suppression. The timing solver's qualified input clock
and electrical assumptions still apply; an accepted configuration or a linked
ELF is not evidence of physical waveform compliance.

## Read sequence and limits

Each loop runs all four forms on I2C1 and then I2C2:

1. `write` the register pointer and complete STOP.
2. `read` two bytes in a separate transaction.
3. `write_read` the pointer and two-byte response with repeated START.
4. `transaction` with `Write`, `Read`, and an adjacent `Read`. The async binary
   makes this call through the real embedded-hal-async trait. Both read buffers
   share one receive command and one address phase; only their final byte is
   NACKed. A direction change uses repeated START, followed by one final STOP.

The blocking binary uses the corresponding `blocking_*` methods. `Readings`
keeps raw bytes for inspection at `black_box`; interpretation depends on the
target. A configuration or transfer error halts without retrying bytes that may
already have reached the target. Contiguous read runs are limited to 256 bytes.

Async TXE, RXNE and initial STOP-event waits have no built-in deadline. Initial
bus-idle checks, final STOP/idle confirmation, retirement and Drop use finite
synchronous sample budgets. `poll_limit` counts samples, not elapsed time or a
whole-transaction limit. A separate application timeout can cancel a suspended
future but cannot preempt a synchronous loop or destructor. Cancellation may
leave target effects: MEN-disable can drain active/queued work before stopping.
Confirmed local shutdown or held master reset permits checked restoration before
reuse; unconfirmed shutdown/reset/configuration leaves a sticky fault until chip
reset. Neither cancellation nor reset promises physical bus recovery.

These are ordinary hardware firmware examples. No hardware execution, measured
timing, waveform, cancellation-on-silicon, low-power or RF validation is claimed.
The [L012 command/FIFO contract](../../../docs/l012-i2c-async.md) documents the
source-qualified API and lifecycle limits.

## Build

From the repository root, choose exactly one of the two chip features and one
binary:

```sh
cargo build --offline --locked --release \
  --manifest-path examples/l012/i2c/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32l012c8t6 --bin async_master

cargo build --offline --locked --release \
  --manifest-path examples/l012/i2c/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32l012c8u6 --bin blocking_master
```

The manifest has no generic-family feature. `build.rs` requires one exact
package selector, matching metadata name/line, the two qualified routes and
dedicated vectors, and exactly the selected part's 64 KiB Flash at `0x00000000`
and 8 KiB RAM at `0x20000000`. It emits `memory.x` from those metadata banks and
passes `-Tlink.x`. Forwarding the dependency's generic family feature cannot
satisfy the example's exact-package requirement.

The dependency lock retains the already pinned Embassy revision and existing
validated dependency versions. Release settings match the classic I2C examples:
size optimization `z`, LTO and one codegen unit. There is no time-driver feature
or enlarged memory fallback. Local Cargo configuration sets only the target
default. These commands are usage instructions, not successful-build receipts.

## Ten-bit master example

`ten_bit_master` demonstrates explicit Address::TenBit and both blocking/async
TenBitAddress traits, using example target 0x2a5 on the existing two separate
buses. It performs pure reads and address probes, not an assumed sensor register
protocol. Configure matching targets before execution; it has not been run on
hardware. The original two seven-bit binaries are unchanged. See the
[ten-bit contract](../../../docs/l012-i2c-ten-bit.md) for initial-read sequencing,
SDK differences, validation scope and cancellation limits.
