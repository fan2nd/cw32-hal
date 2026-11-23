# Safe finite-chunk UART DMA firmware

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

These real Cortex-M0+ applications use the production HAL, generated PAC,
pinned Embassy executor and real UART/DMA interrupts. The UART2 applications
support F030/A030, all five qualified exact L083 packages, and the thirteen
exact F020/L031/L052/R031 selections below. Standalone receive selects
the package-qualified UART shown in the route table.

- `cw32-uart-dma-example`: UART2 PA2 TX / PA3 RX on x030, or PA6 TX / PA7 RX
  on L083 and the thirteen retained exact selections; independent static 32-byte
  staging for each direction. x030/L083 use TX CH2 and RX CH3 on DMACH23.
  F020 uses TX CH2/DMACH2 and RX CH1/DMACH1. L031/L052/R031 use TX CH3
  and RX CH2, with both handlers bound to their shared DMACH23 vector.
  It first uses `split_ref`, then owned `split` halves to repeatedly exchange
  16-byte packets concurrently.
  RX is polled before TX and TX is flushed after each exchange. Both caller
  buffers are ordinary local arrays; both static stages and owners remain live.
- `receive_only`: UART2 PA3 RX on x030, UART2 PA7 RX on L083, or the new
  exact-part route below; static 16-byte staging and DMA CH1/DMACH1. It repeatedly
  receives fixed packets through the standalone `UartRx::new_with_dma` API.
- `shared_vector` (L083 only): UART1 PA8 TX / PA9 RX and UART4 PA4 TX / PB5 RX
  exchange independent 16-byte packets concurrently. UART1 uses TX channel 2
  and RX channel 1; UART4 uses TX channel 5 and RX channel 4. All four private
  32-byte staging allocations and both UART owners remain live outside the
  joined exchanges and wire-completion flushes. DMACH1, DMACH23 and DMACH45
  bind the handlers for all four selected channels.

On L083, UART2 binds `SharedInterruptHandler<UART2_UART5>` once and leaves the
unconstructed UART5 partner inactive. The two-port example binds exactly one
`SharedInterruptHandler<UART1_UART4>` for both constructed partners. Neither
application binds the x030 single-instance UART handler on L083. All selected
pins are bonded on each of the five exact L083 packages.

All thirteen retained selections use UART2 PA6 AF2 TX / PA7 AF2 RX in the combined
application. Their standalone `receive_only` routes are:

| Exact feature selections | Standalone RX route | UART interrupt |
| --- | --- | --- |
| `cw32f020c6u7`, `cw32f020k6u7` | UART1 PA9 AF2 | UART1 |
| `cw32f020f6u7` | UART2 PA7 AF2 | UART2 |
| `cw32l031c8t6`, `cw32l031c8u6`, `cw32l031f8u6`, `cw32l031k8u6`, `cw32l031k8v6` | UART1 PA9 AF2 | UART1 |
| `cw32l031f8p6` | UART2 PA7 AF2 | UART2 |
| `cw32l052c8t6`, `cw32l052r8s6`, `cw32l052r8t6` | UART3 PA10 AF1 | UART3 |
| `cw32r031c8u6` | UART1 PA9 AF2 | UART1 |

The L031F8U6 standalone route is valid even though that package has no safe
UART1 TX route. F020F6U7 and L031F8P6 have no safe UART1 RX or TX, so their
standalone applications use UART2. No PA13, PA14 or PF3 route is added.
F020's two channels are used together only by the combined application;
`receive_only` is separate firmware, with its own CH1 owner.

Connect a voltage-compatible serial peer with common ground. Default settings
are 115200 baud, eight data bits, no parity and one stop bit. A PA2-to-PA3 x030
or PA6-to-PA7 L083/classic-family loopback can exercise the UART2 exchange
application. For `shared_vector`, use separate PA8-to-PA9 and PA4-to-PB5 loopbacks, or two
independent peers. Both ports must finish a packet before the next round begins.
This is not continuous-stream or lossless validation. A peer must account for
finite armed chunks and rearm/copy gaps. None of these applications enables
RTS/CTS, and all halt on errors. Check the board schematic for other devices
on these pins.

Choose one exact package, after generating the PAC normally:

```sh
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-dma/Cargo.toml --no-default-features --features cw32f030c8t7
cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-dma/Cargo.toml --no-default-features --features cw32a030c8t7
for chip in cw32l083rbt6 cw32l083rcs6 cw32l083rct6 cw32l083mct6 cw32l083vct6; do
  cargo build --offline --locked --release --bins --target thumbv6m-none-eabi \
    --manifest-path examples/shared/uart-dma/Cargo.toml --no-default-features --features "$chip"
done

# New combined application and the RX-only UART1 package edge:
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-dma/Cargo.toml --no-default-features \
  --features cw32f020c6u7 --bin cw32-uart-dma-example
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-dma/Cargo.toml --no-default-features \
  --features cw32l031f8u6 --bin receive_only
```

Each exact L083 feature enables the example-local `l083` switch, which includes
`shared_vector` in `--bins`; that switch alone is not a chip selection. The
generic `cw32l083` selection is deliberately absent because it has no exact
qualified SRAM extent for safe staging admission. The five new generic family
aliases are likewise absent; their HAL metadata retains no qualified SRAM map.

The build script checks each new selected signal/AF route and package bond,
then writes a package-qualified `memory.x` from generated metadata and supplies
`-Tlink.x`. The exact F020 parts have 32 KiB Flash; the other ten retained parts
have 64 KiB Flash. All thirteen have 8 KiB SRAM at 0x20000000. No generic
memory fallback is provided.

Stage92 main passed eight standalone ARM libraries and 21 ELF links, all 29
commands exiting zero without warnings. The 21 ELFs cover all fourteen new
combined selections, five family-representative receive-only images, F030 main
and L083 shared-vector firmware. Independent runtime and metadata/source reviews accepted the bounded main
result.
At main acceptance final capability reports and the separate final-source
two-library/three-ELF clean replay had not run. Historical TX-only results retain
their original scope. Current own-family source qualification and the separate
Stage92 verification boundary are documented in
[classic UART DMA RX](../../../docs/classic-uart-dma-rx.md).

Compilation and linking do not demonstrate silicon execution, error timing,
throughput, losslessness or liveness. These applications
are not HAL tests, register models, cancellation harnesses or early-abort APIs.
The new classic UARTs have no overrun report. Finite rearm/copy gaps and silent
RDR overwrite remain possible; DMA completion is not a lossless-stream claim.
R031 GPIOA whole-bank handover limits still apply.
See [the RX and shared ownership contract](../../../docs/uart-dma-rx.md).
