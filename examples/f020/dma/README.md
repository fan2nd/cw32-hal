# F020 owned DMA examples

Two ordinary applications use the native two-channel F020 DMA controller:

- `owned_copy` starts simultaneous finite copies on CH1 and CH2, compares the
  successfully returned buffers, and reuses the returned channel capabilities
  and buffers. It repeatedly covers `u8`, `u16`, and `u32` words.
- `uart_tx` owns two independent staged DMA transmitters. Two Embassy tasks
  repeatedly send ordinary flash and local slices, then flush each transmitter
  before reusing its private stage for the next message.

Select exactly one feature: `cw32f020c6u7`, `cw32f020k6u7`, or
`cw32f020f6u7`. There is no default or generic-family feature. All three exact
parts have 32 KiB flash at `0x00000000` and 8 KiB SRAM at `0x20000000`.
`build.rs` checks the selected exact metadata and derives `memory.x` from it.
The generic `cw32f020` selection has no qualified memory map and cannot admit
safe copies or static DMA staging.

## Build

Run from this directory after the repository's ordinary PAC generation, with
the repository's pinned Rust toolchain and dependency cache:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32f020c6u7 --bin owned_copy
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32f020c6u7 --bin uart_tx
```

Replace the feature with `cw32f020k6u7` or `cw32f020f6u7` for that exact part.
Both binaries are available for all three features. These are application
build commands; build success alone does not establish silicon transfer or
wire behavior. Flashing/running requires a matching board and suitable wiring.

## Owned SRAM copy

`owned_copy` consumes only the two physical channel singletons. CH1 binds
`DMACH1` and CH2 binds `DMACH2`; no third channel or shared DMA vector exists.
Each `cortex_m::singleton!` returns a separate exclusive static allocation.
The six arrays contain 64-byte `u8` pairs, 32-word `u16` pairs, and 32-word
`u32` pairs, totaling 512 bytes before executor, driver, and stack storage.
Actual linked RAM use must still fit the selected 8 KiB region.

Each loop first starts a CH1 `u16` copy and a CH2 `u8` copy before awaiting
either. Once both complete successfully, it starts a CH1 `u32` copy and another
CH2 `u8` copy before awaiting either. The next loop reuses the same returned
owners. Assertions compare every successful source/destination pair. There is
no board LED requirement or separate register-level validation path.

Safe admission comes from normal HAL reset/clean-runtime initialization.
Inherited or dirty DMA state is rejected. Only successful clean completion
returns channel and buffer ownership. An error retains the affected resources;
there is no abort, timeout, recovery, or replacement-buffer example. Dropping
an unresolved owned-copy future can block indefinitely. The panic handler
halts without attempting to reclaim or restart anything.

## UART TX wiring and behavior

The two UART outputs use the following exact-package routes. Connect each TX
to a suitable 3.3 V serial receiver and share ground. Default UART settings
are 115200 baud, eight data bits, no parity, and one stop bit.

| Part feature | Stream A: CH1 / DMACH1 | Stream B: CH2 / DMACH2 |
| --- | --- | --- |
| `cw32f020c6u7` (QFN48) | UART1 TX, PA8/AF2, pad 29 | UART2 TX, PA2/AF2, pad 12 |
| `cw32f020k6u7` (QFN32) | UART1 TX, PA8/AF2, pad 18 | UART2 TX, PA2/AF2, pad 8 |
| `cw32f020f6u7` (QFN20) | UART2 TX, PA6/AF2, pad 9 | UART3 TX, PA2/AF1, pad 5 |

Each transmitter consumes its own UART, pin, physical DMA channel, and
32-byte static SRAM stage. The two stages total 64 bytes. The tasks' ordinary
local message arrays and the shared read-only flash message are both larger
than a stage. `write` copies each caller slice into finite private SRAM chunks;
the caller's slice is never handed directly to DMA. The local array identifies
stream A or B and increments a decimal message counter after each flush.
The two tasks progress independently; chunk boundaries may have gaps.

DMA completion establishes delivery to TDR. `flush` additionally waits for
the UART buffer and shifter to finish, so it is used before the next iteration.
Cancelling a write leaves its already-started chunk owned by the driver; a
later write or flush must settle that chunk before stage reuse. Errors retain
the stage, channel, UART, pin, and clocks. There is no finite completion-time
or throughput guarantee, and ordinary owner Drop is not a flush.

C6/K6 bind the actual `UART1` and `UART2` vectors; F6 binds `UART2` and `UART3`.
F6 does not try to use UART1's excluded PA14 route. No RX DMA, combined UART
DMA, SPI DMA, flow-controlled DMA, buffered/DMA combination, raw hardware
request, or lifecycle probe is introduced here.

## Linked-image checks

For either binary, inspect the actual ELF's nonempty executable text, reset
entry, and flash/SRAM ranges using the generated `memory.x`. The vector table
must point IRQ9 (offset `0x64`) at `DMACH1` and IRQ10 (`0x68`) at `DMACH2`.
Reserved IRQ11 (`0x6c`) must not contain a DMA handler. For `uart_tx`, additionally
check UART1/2 at offsets `0xac`/`0xb0` on C6/K6, or UART2/3 at `0xb0`/`0xb4` on F6.
These are linked-code checks, not hardware execution claims.

Stage84 main linked all six exact-part/binary combinations successfully,
with zero warnings and the stated ELF placement/vector checks passing.
The complete main matrix passed 15 actual ARM libraries and 16 ELF links;
generation and five finite source gates passed in documented segments.
Independent runtime and metadata/source reviews accepted the main work.
At main acceptance clean had not run; final-package completion requires a
separate final-source 2-library/4-ELF clean receipt. Full warning/failure
history and the generated delta are retained in
[the verification boundary](../../../docs/f020-dma-owned.md#verification-boundary).

See [F020 DMA ownership](../../../docs/f020-dma-owned.md),
[safe owned copies](../../../docs/dma-safe-owned-copy.md), and
[staged UART TX](../../../docs/uart-dma-tx.md) for the native source qualification
and ownership limits.
