# Finite native UART receive-until-idle firmware

These ordinary Cortex-M0+ applications use the production HAL, generated PAC,
pinned Embassy thread executor and real typed UART interrupt bindings. Select
exactly one of the three package features; there is no default chip.

| Feature | UART / IRQ number | TX AF1 / package pad | RX AF1 / package pad | Flash / RAM bytes |
| --- | --- | --- | --- | --- |
| `cw32l010f8p6` | UART2 / 28 | PA3 / TSSOP20 pad 13 | PA4 / TSSOP20 pad 14 | 65536 / 4096 |
| `cw32l011k8t6` | UART3 / 29 | PA4 / LQFP32 pad 10 | PA5 / LQFP32 pad 11 | 65536 / 6144 |
| `cw32l012c8t6` | UART1 / 27 | PA2 / LQFP48 pad 12 | PA3 / LQFP48 pad 13 | 65536 / 8192 |

Connect a voltage-compatible serial peer with common ground, crossing the peer's
TX to the MCU RX and its RX to the MCU TX. Check the board schematic for other
devices on these pins. Default settings are 115200 baud, eight data bits, no
parity and one stop bit, with RX pull-up and no RTS/CTS.

## Applications

- `receive` owns a combined async `Uart` and repeatedly calls its forwarding
  `read_until_idle` method with an ordinary local 64-byte buffer. On success,
  it sends `received NN\r\n`, the exact returned byte prefix, then `\r\n`.
  `NN` is a two-digit decimal count from 00 to 64. On a receive error it discards
  the unsuccessful buffer and sends `receive error\r\n`. Every response is
  flushed before the next receive. Send a short burst, then wait for the whole
  response before sending another; RX is not serviced while the reply is sent.
- `split`, available only for `cw32l012c8t6`, splits the UART into owned TX and
  RX halves. Each round joins `UartRx::read_until_idle` on a fresh local 64-byte
  response buffer with transmission and flushing of `read?\r\n`. RX is polled
  first. The peer should respond to that request with a finite burst and then
  stop transmitting. The application sends the same count/prefix or error
  report as `receive`, flushes it, and starts another round. The peer should
  respond only to `read?`, not to the report. TX and RX are concurrent during
  the request/response exchange; RX is not serviced during the report.

Configuration or transmit errors halt through the panic handler. Receive
errors are reported and the next iteration starts a new finite read; no
recovery of discarded bytes or frame synchronization is promised.

## Build and linker setup

After the repository's normal PAC generation, build from the repository root:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --manifest-path examples/shared/uart-idle/Cargo.toml \
  --features cw32l010f8p6 --bin receive
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --manifest-path examples/shared/uart-idle/Cargo.toml \
  --features cw32l011k8t6 --bin receive
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --manifest-path examples/shared/uart-idle/Cargo.toml \
  --features cw32l012c8t6 --bin receive
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --manifest-path examples/shared/uart-idle/Cargo.toml \
  --features cw32l012c8t6 --bin split
```

The build script requires one exact metadata memory map and writes its FLASH
and RAM banks to `OUT_DIR/memory.x`. Flash begins at 0 and RAM at `0x20000000`;
the bank lengths come from the selected package metadata, as shown above. The
script adds the output search path and explicitly supplies `-Tlink.x`, including
when Cargo is invoked with `--manifest-path` from the repository root. No local
linker override is needed. The standalone manifest and lockfile retain the
existing `uart-dma` dependency declarations and resolved versions, including
the pinned Embassy revision.

## Receive boundary and limits

`read_until_idle` finishes when the caller's finite buffer is full or native
RXIDLE is observed. It can return zero for an observed idle without a consumed
byte. A nonempty call discards the stale idle indication once at entry; idle
state from a previous call is not a retained packet boundary. Native idle
detection starts after a received character, so silence alone does not promise
completion, and a call can wait indefinitely without a full buffer or an idle
event. The separate programmable receive timer is not used; there is no
configurable duration or numeric timeout.

The task must service each byte. There is no background RX buffer, DMA staging,
ring, queue or continuous-stream reception. Delayed service can overwrite the
single receive register and report overrun. If RC and RXIDLE are observed
together, the HAL consumes the observed byte before returning idle success;
hardware does not identify whether that byte preceded or followed the earlier
idle. This is a best-effort observed boundary, not exact packet framing or a
losslessness guarantee. A full buffer may also split the peer's transmission.
Use paced short bursts and account for the gaps between finite reads.

Compilation and linking establish API, pin, interrupt and linker integration.
They do not demonstrate silicon execution, physical baud accuracy, throughput,
event timing, error injection, cancellation or forget behavior. These examples
are ordinary applications, not lifecycle probes. See the
[native idle receive contract](../../../docs/uart-idle.md).
