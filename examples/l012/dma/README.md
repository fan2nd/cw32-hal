# CW32L012 native owned DMA examples

Two ordinary applications use the native four-channel CW32L012 DMA controller:

- `owned_copy` starts finite copies on all four channels before awaiting any,
  compares the cleanly returned buffers, and reuses those same owners.
- `uart_tx` runs three independent transmitters. Each Embassy task repeatedly
  writes an ordinary flash slice and a local slice longer than its private
  32-byte SRAM stage, then flushes the UART.

Select exactly one of `cw32l012c8t6` (LQFP48) and `cw32l012c8u6` (QFN48).
There is no default or generic-family feature. Each exact part has 64 KiB
flash at `0x00000000` and 8 KiB SRAM at `0x20000000`. The build script checks
the selected exact metadata, memory kinds and intervals, native peripheral
versions and bases, both shared DMA vectors, and all three UART TX routes.
It derives `memory.x` from that metadata and explicitly links `link.x`.
The generic HAL selection has no qualified SRAM map; safe owned copy
admission and DMA staging reject it. This package supplies no generic map.

## Build

After the repository's ordinary PAC generation, build from this directory
with the repository's qualified Rust toolchain and dependency cache:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32l012c8t6 --bin owned_copy
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32l012c8t6 --bin uart_tx
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32l012c8u6 --bin owned_copy
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32l012c8u6 --bin uart_tx
```

These are build instructions, not execution receipts. The implementation
adds the two binaries for both exact features; actual main and clean results
belong to the [verification record](../../../docs/l012-dma-owned.md).
Hardware transfer and UART wire output have not been validated.

## Owned SRAM copy

`owned_copy` consumes all four physical channel singletons and eight separate
exclusive static SRAM allocations:

| Channel | Source and destination | Shared vector |
| --- | --- | --- |
| CH1 | two 64-element `u8` arrays | DMACH12, IRQ9 |
| CH2 | two 32-element `u16` arrays | DMACH12, IRQ9 |
| CH3 | two 32-element `u32` arrays | DMACH34, IRQ10 |
| CH4 | two 64-element `u8` arrays | DMACH34, IRQ10 |

The arrays total 640 bytes before executor, driver, and stack storage. Each
shared vector binds both of its channel handlers, which retain their own
terminal flags and state. There is no fifth channel. IRQ11 is CORDIC and is
not a DMA interrupt.

Every loop changes the first source word, launches all four finite copies,
then awaits them and compares each complete source/destination pair. Only
clean completion returns the channels and allocations for the next loop.
Normal HAL reset/clean-runtime initialization supplies safe admission;
inherited or dirty DMA state is rejected. The bounded native profile uses
one block with REPEAT=1 and disables restart and source/destination reload.
Errors permanently retain affected resources. Dropping an unresolved copy
can block indefinitely. The panic handler halts without reclaiming,
restarting, or recovering unfinished DMA.

## UART TX routes and behavior

Both exact packages use the same routes. Each output owns a separate UART,
pin, physical DMA channel, and 32-byte static stage:

| UART | TX pin / AF | Package pad | DMA channel / vector | TX request | UART IRQ |
| --- | --- | --- | --- | --- | --- |
| UART1 | PA9 / AF1 | 30 | CH1 / DMACH12 | 1 | 27 |
| UART2 | PA7 / AF1 | 17 | CH2 / DMACH12 | 3 | 28 |
| UART3 | PA8 / AF1 | 29 | CH4 / DMACH34 | 5 | 29 |

Both CH1 and CH2 handlers are bound to DMACH12. DMACH34 binds only CH4 in
`uart_tx`; CH3 is unused in this binary. The copy application uses both
DMACH34 siblings. All three individual UART interrupt handlers are bound.
PA13 and PA14 remain excluded from the safe pin routes. The selected pins
share the existing GPIOA bank clock contract without overlapping one another.

Connect each TX pin to a suitable serial receiver and share ground, following
the selected part's electrical limits and package pinout. Check board wiring
before flashing: the package pin table does not establish how a board routes
its UARTs. Default UART settings are 115200 baud, eight data bits, no parity,
and one stop bit. These applications configure no board LED or RF device.

The three-task pool uses 96 bytes of static staging. Each task identifies its
UART in a local message and increments a decimal counter after each flush.
Both caller messages exceed the 32-byte stage. The driver copies them on the
CPU into finite private SRAM chunks; DMA never holds the caller slice
pointer. The three tasks progress independently, and chunks may have gaps.

This native profile transfers SRAM bytes to TDR. It does not establish
arbitrary byte MMIO access: CPU peripheral access remains typed 32-bit, and
TDR's special IDLE/BREAK commands and nine-bit payloads are outside this
example. DMA completion reaches TDR. `flush` additionally waits for UART
TXBUSY to clear after retiring the DMA chunk.

A cancelled write may discard its unscheduled remainder while retaining its
already-started chunk; a later write or flush settles that chunk before stage
reuse. Errors quarantine resources. Pending/error owner Drop retains the
UART configuration, pin, stage, channel, and clocks. Clean owner Drop does not
promise to finish the last wire frame. No abort, timeout, recovery, finite
completion-time, or throughput guarantee is provided.

These applications use the existing owned-copy and staged-TX APIs. They add
no raw-DMA operation, RX DMA, combined RX/TX DMA, SPI DMA, buffered/DMA mixture,
ring attachment, flow control, error injection, or lifecycle probe.

## Linked-image checks and limits

For each actual ELF, inspect executable text, reset entry, initial stack,
loadable/allocated ranges, and RAM use against the generated `memory.x`.
Both binaries must resolve DMACH12/DMACH34 at IRQ9/10 (vector offsets
`0x64`/`0x68`) to real handlers. Inspect both channel bindings on DMACH12,
both on DMACH34 for `owned_copy`, and CH4 alone on DMACH34 for `uart_tx`.
IRQ11 at `0x6c` remains CORDIC; these applications do not bind it.
`uart_tx` also binds UART1/2/3 at IRQ27/28/29 (vector offsets
`0xac`/`0xb0`/`0xb4`). Inspect actual handler symbols and vector words rather
than inferring them from a successful library build.

Build and source checks cannot establish on-silicon transfer, error-time bus
drain, worst-case dynamic stack usage, throughput, bounded cancellation, or
electrical behavior. Those remain unmeasured.

See [safe owned copies](../../../docs/dma-safe-owned-copy.md) and
[staged UART TX](../../../docs/uart-dma-tx.md) for the ownership and cancellation
contracts shared by these applications.
