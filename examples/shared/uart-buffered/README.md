# Continuous IRQ-buffered UART receive examples

These ordinary applications use direct `BufferedUart` or `BufferedUartRx`
owners. Each active receiver consumes a separate 64-byte static ring allocated
by `cortex_m::singleton!`; application reads use ordinary 16-byte local chunks.
Collection starts during construction and continues between reads while the
receive owner is live and UART interrupts can run. The examples use the default
115200 baud, eight data bits, no parity and one stop bit.

Select exactly one of the eight exact package features below. There are no
default chip features. The metadata-derived build script emits `memory.x` and
explicitly passes `-Tlink.x`; it does not enlarge the selected memory map.

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --no-default-features --manifest-path examples/shared/uart-buffered/Cargo.toml \
  --features cw32f002f3p7 --bin receive
```

The command is run from the repository root. Dependencies and checksums reuse
the existing source lock, including Embassy revision
`f16efeffe37581092ec184718e6fdb1620393214`. No additional HAL dependency is used.

## Applications

- `idle_queue`: uses the same native features and wiring as `idle_notify`.
  Starts four-slot continuous capture, reports marker overflow and drains data
  independently. Queued positions adjust as data is read; marker errors are
  historical and may repeat. Not compiled or hardware-tested.
- `idle_notify`: requires `idle-notify` and one of `cw32l010f8p6`, `cw32l011k8t6`
  or `cw32l012c8t6`; uses the `rx_only` routes. Arm before sending a short burst,
  await a new native idle marker, then consume exactly its unread byte prefix
  and recorded error. Later appended bytes remain for subsequent reads. The marker
  is a software observation position, not wire packet framing. A silent line may not wake, and long
  bursts can overflow the ring. Not compiled or hardware-tested.
- `replace_buffer`: after reading data/errors, attempt to replace the RX ring
  with a static ring of twice the capacity. On pending data/error, retain the
  rejected buffer and retry after another read; on success, reuse the old
  storage while RX continues. Uses the same routes as `rx_only`. Not compiled
  or hardware-tested; continuous traffic can prevent replacement.
- `receive`: construct a combined buffered UART and echo each available prefix of
  up to 16 bytes. It waits for at least one byte, not a full chunk; it does not
  identify packets or idle gaps. On L083 it binds the complete UART1/UART4 handler but owns only UART1.
  Leave UART4 and any previously configured external partner traffic quiescent.
- `split`: use owned `split` on L031 and the other selections, and borrowed
  `split_ref` on L012. A joined RX/TX exchange sends `send data` while an
  `embedded_io_async::Read::read` on the RX half awaits data. The trait returns available queued bytes
  after waiting for at least one. Only the returned prefix is echoed.
- `rx_only`: construct `BufferedUartRx` directly, using `new_with_rts` on L052
  and `new` without RTS on the other selections. Successful trait-read prefixes
  and receive errors pass through `core::hint::black_box` for inspection with a
  debugger. This application has no TX owner or serial error-report channel.
- `flow_control`: L010 only. Construct `BufferedUart::new_with_rtscts` and echo
  successful exact 16-byte chunks using `read_exact`. CTS has a pull-up; connect the peer's RTS to
  local CTS and its CTS to local RTS. A peer that does not assert CTS can leave
  writes or flushes waiting indefinitely.
- `shared_vector`: L083 only. Own both UART1 and UART4, each with its own consumed
  64-byte ring. One shared binding dispatches both. Two joined echo loops keep
  both owners live and use `read_exact` to echo independent 16-byte chunks.

`flow_control` requires `cw32l010f8p6`; `shared_vector` requires `cw32l083rct6`.
`receive`, `split` and `rx_only` use the selected feature's routes below. The
selected build/link matrix uses the nine chip/bin rows shown in the last column;
other combinations are source support, not additional build or hardware claims.

## Exact-package wiring and memory

Connect TX to the peer's RX, RX to its TX, and share ground. Use compatible
logic levels, not an RS-232 voltage-level connection. Pad numbers below refer
to the named chip package; they are not board-header positions.

| Feature | UART and qualified pins (AF, package pad) | FLASH / RAM bytes | Selected binary |
| --- | --- | --- | --- |
| `cw32f030c8t7` | UART2 TX PA2 (AF2, 12), RX PA3 (AF2, 13) | 65536 / 8192 | `receive` |
| `cw32f002f3p7` | UART1 TX PB1 (AF1, 6), RX PB0 (AF1, 5) | 16384 / 2048 | `receive` |
| `cw32l031c8t6` | UART2 TX PA2 (AF2, 12), RX PA3 (AF2, 13) | 65536 / 8192 | `split` |
| `cw32l052c8t6` | UART2 TX PA2 (AF2, 12), RX PA3 (AF2, 13); RX-only RTS PA1 (AF2, 11) | 65536 / 8192 | `rx_only` |
| `cw32l083rct6` | UART1 TX PA8 (AF2, 41), RX PA9 (AF2, 42) | 262144 / 24576 | `receive` |
| `cw32l010f8p6` | UART2 TX PA3 (AF1, 13), RX PA4 (AF1, 14); flow-control RTS PB1 (AF2, 12), CTS PB0 (AF2, 11) | 65536 / 4096 | `flow_control` |
| `cw32l011k8t6` | UART3 TX PA4 (AF1, 10), RX PA5 (AF1, 11) | 65536 / 6144 | `rx_only` |
| `cw32l012c8t6` | UART1 TX PA2 (AF1, 12), RX PA3 (AF1, 13) | 65536 / 8192 | `split` |
| `cw32l083rct6` | UART1 as above; UART4 TX PA4 (AF1, 20), RX PB5 (AF1, 57) | 262144 / 24576 | `shared_vector` |

FLASH starts at `0x00000000` and RAM at `0x20000000` for every row. In
particular, F002 retains its actual 16 KiB FLASH / 2 KiB RAM geometry. Pin and
memory authority is the existing exact-package metadata; see
[buffered RX documentation](../../../docs/uart-buffered-rx.md) for source evidence
and the complete API/lifecycle contract.

UART2 uses IRQ28 on F030, L031, L052 and L010. UART1 uses IRQ27 on F002 and
L012. L011 UART3 uses IRQ29. L083 UART1 and UART4 share IRQ27 under
`UART1_UART4`; the binding appears once in the common library and uses
`SharedInterruptHandler<UART1_UART4>`.

## Error handling and limits

Receive errors are application outcomes. The echo applications preserve the
error value for debugger inspection, write `receive error`, and continue. An
exact read can already have changed a prefix of its caller buffer when it
returns an error, but it returns no successful count. That partial chunk is
discarded. The RX-only application exposes errors to the debugger and resumes
reading. Constructor failure and TX failure use the examples' panic handler.

The ring retains bytes accepted before the first pending receive error. Reads
drain that prefix before observing the error. While the error is pending, the
ISR still drains and acknowledges hardware but discards subsequent receive
observations. Consuming the error permits later ISR observations to be queued
again. Software overflow is `Error::BufferOverflow`; it is different from the
native hardware `Error::Overrun`. A ring does not make reception lossless.

The ten families without documented native ORE can silently overwrite unread
hardware RDR before the ISR observes it, even when the software ring has space.
L010/L011/L012 have native ORE and NE; noise detection is limited to 16x/8x
oversampling. Errors can coalesce and do not provide an exact lost-byte count.
The static ring is consumed even on a construction error and is not returned
on drop. There is no unsafe static borrowing in these applications.

Hardware RTS reflects hardware RC, not the software queue's occupancy. The ISR
can acknowledge RC and resume the peer while the ring is full. Collection
depends on interrupt progress, and exact reads have no finite-wait guarantee.
These are not DMA rings and provide no idle markers, timeout, packet boundaries,
or software RTS watermark. The examples do not inject interrupts or errors and
do not test cancellation, teardown races or timing. Building and linking them
does not validate silicon loss, interrupt saturation, RTS timing or throughput;
no hardware timing validation is claimed.

## Verification boundary

Stage83 main verification passed all nine selected buffered ELF links above,
plus the L012 native-idle `receive` and L083 staged-DMA `shared_vector`
regressions, thirteen actual ARM HAL libraries and five focused source/data
gates. Independent main runtime and metadata/source reviews accepted the frozen
implementation. The L010/L011 library and buffered-ELF rows each retain three
existing RCC LSE-constant warnings; all other main rows have none. Main ran no
hardware generation, and all 905 generated hardware files stayed unchanged.
At main acceptance clean had not run; final-package completion requires a
separate final-source three-library/three-ELF clean receipt and the five focused
gates, as detailed in the [buffered receive contract](../../../docs/uart-buffered-rx.md).

## Borrowed receive prefixes

The `split` example uses `embedded_io_async::BufRead::fill_buf` and `consume`.
It holds an RX prefix while awaiting a write through the independent TX half,
then releases exactly that prefix. IRQs can append into free slots meanwhile,
but holding the prefix too long can overflow the ring. The storage remains
consumed static memory; this does not introduce nonstatic buffer reclamation.


## Borrowed ring example

The `borrowed` binary (CW32F002F3P7) uses the explicit unsafe
`BufferedUartRx::new_borrowed` constructor with a local array, waits synchronously
for data or an error, drops RX, and reuses the same array. The synchronous scope
never leaks or suspends an owner-containing future; its panic handler disables
interrupts and never resumes execution. These facts establish this example's
unsafe lifetime contract, not a guarantee for arbitrary async applications.

The existing examples retain the safe static constructors. Ordinary borrowed
construction follows Embassy's normal Drop/reuse model, but is explicitly unsafe
here: forgetting an RX owner or its containing task/future can leave its ISR
pointer live. Keep the storage allocated, stationary and exclusively reserved
until collection is stopped and the ISR registration detached. Dropping a TX
half alone is not enough. See the [buffer lifetime contract](../../../docs/uart-buffered-rx.md#api-and-buffer-lifetime).
