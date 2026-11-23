# Safe staged UART TX DMA firmware

This standalone Cortex-M0+ application repeatedly sends two lines on UART2 TX
(PA2 AF2 on x030, PA6 AF2 on L083), using DMA channel 1. On x030 it binds the
UART2 and DMACH1 handlers; on L083 it binds the DMACH1 handler and the matching
`SharedInterruptHandler<UART2_UART5>` once, keeping the unconstructed UART5
partner inactive.
It links the production HAL, generated PAC, Cortex-M runtime and pinned Embassy
executor. Choose exactly one feature: `cw32f030c8t7`, `cw32a030c8t7`,
`cw32l083rbt6`, `cw32l083rcs6`, `cw32l083rct6`, `cw32l083mct6` or
`cw32l083vct6`. There is no default or generic L083 chip selection.

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-dma-tx/Cargo.toml \
  --no-default-features --features cw32f030c8t7
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-dma-tx/Cargo.toml \
  --no-default-features --features cw32a030c8t7
for chip in cw32l083rbt6 cw32l083rcs6 cw32l083rct6 cw32l083mct6 cw32l083vct6; do
  cargo build --offline --locked --release --target thumbv6m-none-eabi \
    --manifest-path examples/shared/uart-dma-tx/Cargo.toml --no-default-features --features "$chip"
done
```

`build.rs` writes the example's own `memory.x` from the selected generated
memory metadata and supplies `-Tlink.x`. All seven exact parts have qualified memory
maps; no family-capacity guess or root-working-directory linker file is used.
The standard source build first needs the generated PAC, as with other examples.

Use a voltage-compatible serial peer and common ground; connect the TX pin to the
peer's RX. The configuration is 115200 baud, eight data bits, no parity and one
stop bit, with the default HSI/PCLK clock configuration. No RX or CTS pin is
configured. Check the board schematic for other devices connected to the TX pin.
PA6 is bonded on all five qualified L083 packages.

`cortex_m::singleton!` allocates an exclusive 32-byte static SRAM staging buffer
once. `UartTx::new_with_dma` consumes it together with the UART, matching TX pin
and compatible DMA channel. The application then calls the usual safe
`write(&[u8]).await` method with a read-only flash message and a mutable ordinary
local array. Neither input needs to be static; locals live in the async task's
frame while borrowed. Both messages exceed 32 bytes and therefore use multiple
hardware chunks. After both writes, `flush().await` waits for the final wire
transfer before the local digit changes and the next cycle starts.

The CPU copies each byte into private staging. Chunk boundaries can introduce
gaps while the executor wakes, copies and rearms DMA; no continuous-stream,
throughput or finite completion-time guarantee is made. DMA completion alone
means the bytes reached the UART transmit data register, not that the final stop
bit has left the pin. Use `flush` when wire completion matters.

Cancelling or forgetting a polled write may leave a transmitted prefix and the
current staged chunk running. The driver retains that chunk, its request and
its exclusive static storage; the unscheduled remainder is not sent. A later
`write` or `flush` waits for that same chunk's clean completion before reuse.
This is not an early-abort API. TE, simultaneous TE+TC or an inconsistent
completion permanently quarantines the affected resources. Later calls do not
restart that channel, and dropping an unfinished/quarantined driver retains its
required resources and clocks. The example halts on an error.

Normal hardware-reset or trustworthy clean-runtime entry is required. Startup
admission rejects dirty state but cannot prove or repair arbitrary active
bootloader DMA history. The example performs no raw register access or unsafe
application calls. It is production firmware, not a HAL test, register model or
compile fixture. Compilation/linking does not establish hardware execution,
electrical behavior, performance or liveness; no flashing or execution is part
of CI. UART RX DMA and SPI DMA have separate applications in `../uart-dma` and
`../spi-dma`. CTS with DMA and early abort remain outside this staged TX API.

See [the API contract and source evidence](../../../docs/uart-dma-tx.md).
