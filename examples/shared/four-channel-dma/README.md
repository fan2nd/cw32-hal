# Native four-channel owned DMA examples

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

Two ordinary applications use the native four-channel DMA controller on the
source-qualified CW32L031, CW32L052, CW32R031 selections:

- `owned_copy` starts finite copies on all four channels before awaiting any,
  compares the cleanly returned buffers, and reuses those same owners.
- `uart_tx` runs two or three independent transmitters. Each Embassy task
  repeatedly writes an ordinary flash slice and a local slice larger than its
  private 32-byte SRAM stage, then flushes the UART.

Select exactly one of the ten exact features below. There is no default
chip or generic-family feature. Each exact part has 64 KiB flash at
`0x00000000` and 8 KiB SRAM at `0x20000000`. The build script checks the exact
metadata identity, memory kinds and intervals, channel/vector topology, and
the actual UART TX signal, pin, AF, request, and vector for each used route.
It derives `memory.x` from the selected metadata and explicitly links `link.x`.
The generic HAL selections have no qualified memory map; safe copy admission
and static DMA staging reject them, and this package supplies no generic map.

## Build

After the repository's ordinary PAC generation, run from this directory with
the repository's pinned Rust toolchain and dependency cache:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32l031f8p6 --bin owned_copy
cargo build --offline --locked --release --target thumbv6m-none-eabi --no-default-features --manifest-path Cargo.toml --features cw32l052c8t6 --bin uart_tx
```

Both binaries are available for each exact feature in the route table. Replace
the feature with the one matching your part. These commands describe how to
build; the actual main acceptance includes `uart_tx` on all eleven exact parts
and `owned_copy` on L031F8P6, L052C8T6, R031C8U6 and W031R8U6. These historical 15 linked
ELFs passed independent code, memory and active-vector inspection without
warnings. They are part of the recorded 27-command main matrix, not additional
commands. See the [actual verification boundary](../../../docs/four-channel-dma-owned.md#verification-boundary)
and its independent review receipts. Clean had not run at main acceptance and
requires a separate final-source receipt. Hardware transfers and UART wire
output have not been validated.

## Owned SRAM copy

`owned_copy` consumes all four physical channel singletons and eight separate
exclusive static allocations:

| Channel | Source and destination | Vector |
| --- | --- | --- |
| CH1 | two 64-element `u8` arrays | DMACH1 |
| CH2 | two 32-element `u16` arrays | DMACH23 |
| CH3 | two 32-element `u32` arrays | DMACH23 |
| CH4 | two 64-element `u8` arrays | DMACH4 |

The arrays total 640 bytes before executor, driver, and stack storage. Both
CH2 and CH3 handlers are bound to DMACH23, each retaining its own terminal
flags and state. CH4 has its own DMACH4 vector. There is no CH5.

Every loop changes the first source word, launches all four finite copies,
then awaits them and compares each complete source/destination pair. Only
clean completion returns the channels and allocations for the next loop.
Normal HAL reset/clean-runtime initialization supplies safe admission;
inherited or dirty DMA state is rejected. Errors permanently retain affected
resources. Dropping an unresolved copy can block indefinitely. The panic
handler halts without reclaiming, restarting, or recovering unfinished DMA.

## UART TX routes and behavior

The exact selections use these routes. Each active output has its own UART,
pin, physical DMA channel, and 32-byte static stage. UART2 uses CH2 and UART3
uses CH3 with both handlers on DMACH23. UART1, where available, uses CH4 and
DMACH4. The individual native UART1/2/3 interrupt handlers are also bound.

| Exact feature | UART1 / CH4 | UART2 / CH2 | UART3 / CH3 |
| --- | --- | --- | --- |
| `cw32l031c8t6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l031c8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l031f8p6` | omitted | PA6 AF2 | PA10 AF6 |
| `cw32l031f8u6` | omitted | PA6 AF2 | PA10 AF6 |
| `cw32l031k8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l031k8v6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |
| `cw32l052c8t6` | PA8 AF2 | PA6 AF2 | PA9 AF1 |
| `cw32l052r8s6` | PA8 AF2 | PA6 AF2 | PA9 AF1 |
| `cw32l052r8t6` | PA8 AF2 | PA6 AF2 | PA9 AF1 |
| `cw32r031c8u6` | PA8 AF2 | PA6 AF2 | PA10 AF6 |

F8P6/F8U6 have no safe UART1 TX route; the excluded PA14 route is not exposed
as an escape. PA13, PA14, and PF3 remain excluded. There are 28 active routes
across the ten exact selections. L052 retains its own native UART version
and PA9 AF1 UART3 route; the other two families use PA10 AF6.

Connect each used TX pin to a suitable serial receiver and share ground,
following the selected part's electrical limits and package pinout. Default
UART settings are 115200 baud, eight data bits, no parity, and one stop bit.
Check board wiring before flashing; the examples do not configure a board LED
or an RF device. R031's existing GPIOA-bank RCC effects still apply, even
though these routes avoid its internal RF pins PA0..3.

F8 uses a two-task pool and 64 bytes of static staging; the other selections
use a three-task pool and 96 bytes of static staging. Each task identifies its
UART in an ordinary local message and increments a decimal counter after
each flush. Both caller messages exceed the 32-byte stage. The driver copies
them on the CPU into finite private SRAM chunks; DMA never holds the caller
slice pointer. The UART tasks progress independently, and chunks may have gaps.

DMA completion reaches TDR. `flush` additionally waits for the UART buffer
and shifter to finish. A cancelled write leaves the already-started chunk
owned by the driver; a later write or flush settles that chunk before stage
reuse. Errors quarantine the resources, and pending/error owner Drop retains
the UART configuration, pin, stage, channel, and clocks. There is no abort,
timeout, recovery, finite completion-time, or throughput guarantee. Ordinary
owner Drop does not promise to finish the last wire frame.

These applications use admitted owned copies and staged UART TX. They add no
raw-DMA operation, RX DMA, combined RX/TX DMA, SPI DMA, buffered/DMA mixture,
ring attachment, flow control, error injection, or lifecycle probe.

## Linked-image checks and limits

For each actual ELF, inspect executable text, reset entry, initial stack,
loadable/allocated ranges, and RAM use against its generated `memory.x`.
`owned_copy` must resolve IRQ9/10/11 (vector offsets `0x64`/`0x68`/`0x6c`) to
DMACH1/DMACH23/DMACH4. `uart_tx` uses IRQ10 and, except on F8, IRQ11; its unused
DMA vectors may resolve to the default handler. Active UART1/2/3 vectors are
IRQ27/28/29 at `0xac`/`0xb0`/`0xb4`; F8 uses only UART2/3. Verify actual handler
symbols and both DMACH23 channel bindings rather than inferring them from a
manifest or a successful library build.

Build and static ownership checks cannot establish on-silicon transfer,
error-time bus drain, worst-case dynamic stack usage, throughput, bounded
cancellation, or electrical/RF behavior. Those remain unmeasured.

See [safe owned copies](../../../docs/dma-safe-owned-copy.md) and
[staged UART TX](../../../docs/uart-dma-tx.md) for the ownership and cancellation
contracts shared by these applications.
