# Finite typed UART and SPI DMA

Version 0.21 adds finite DMA endpoints to the existing UART and SPI owners. The
request route is selected by generated `TxDma<Instance>` / `RxDma<Instance>`
traits, not a caller-supplied selector or peripheral address. All topology
channels can select all requests on their own controller, as documented by
HARDSRC for every channel y. The generated implementation retains the exact
peripheral instance and direction. Missing or ambiguous metadata fails the
build. This does not imply all 107 raw request selectors have safe drivers.

## Supported operations

| Endpoint | Memory words | Length | Completion required |
| --- | --- | --- | --- |
| UART TX | `u8`, eight payload bits | 1–65535 | DMA TC, then `TXBUSY=0` |
| UART RX | `u8`, eight payload bits | exactly 1 | DMA TC, RX disabled, final receive-error check |
| SPI full-duplex controller | `u8` or `u16` | equal lengths, 1–65535 | both DMA TCs, no bus errors, `TXE=1`, `BUSY=0`, `RXNE=0` |

Existing UART/SPI constructors, pin routes, blocking methods, interrupt-driven
methods and SPI `embedded-hal` implementations remain available. DMA methods
consume the existing driver and channel owners. An `Async` bus requires its
peripheral interrupt `Binding`; its `Async` channels each require their own
exact DMA interrupt `Binding`. A `Blocking` operation uses blocking channels.
DMA futures exist only for `Async`. Shared DMA vectors still require every
participating channel handler, and handlers service only their own channel.

The first UART DMA endpoint deliberately rejects nine-bit configuration. SPI
uses only the documented 8/16-bit pairs: DMA SIZE 0/1 and SPI WIDTH 7/15. Every
bus endpoint uses hardware-triggered BLOCK with REPEAT=1, fixed peripheral
address and incrementing memory. BULK is not valid for these paced endpoints.

## Ownership and cancellation

Every DMA endpoint takes the complete bus owner with lifetime `'static`, owned
channel(s) with lifetime `'static`, and exclusive `&'static mut` buffers. This
includes the TX source, so it can be returned and edited after successful TX.
Borrowed bus tokens, borrowed channel tokens and stack buffers cannot enter
these methods. SPI requires separate buffers; there is no in-place alias.

On successful completion the returned `Resources` bundle contains the original
bus, channel(s) and buffers. Before-start validation returns the same bundle in
`StartError.resources`, without starting either DMA channel. Once started:

- Normal success returns all resources only after the documented DMA and wire
  completion checks succeed.
- An incomplete cancellation, timeout, DMA error or peripheral error disables
  requests, requests DMA stop, permanently poisons every participating channel,
  and retains all owners and buffers for the remaining device lifetime.
- Drop retains the whole operation even if a terminal flag is already pending.
  Explicit `cancel()` can return resources when completion is already proven.
- `mem::forget` retains the driver, pins, clock leases, channel identities and
  buffers. An IRQ may still disarm completed requests, but cannot return owners.
  Safe code cannot reconstruct or use the consumed resources.
- Retaining a UART half also retains its independent clock and pin lease. A
  safely split opposite half can continue its normal independent operation.

Retained resources live in `ManuallyDrop`, including on errors. A guard is not
allowed to release pins/clocks while a pending DMA peripheral access is still
possible. There is no recover-after-abort API: neither EN=0, a compiler fence,
a CPU barrier, a UART disable nor a SPI buffer flush proves outstanding DMA
bus accesses have drained. The same documented boundary underlies the existing
finite ADC and memory-copy DMA contracts.

`blocking_wait(poll_budget)` bounds register observations. UART TX additionally
uses its explicit `drain_budget`; SPI uses the existing `Config::poll_limit`
for the final TXE/!BUSY wait, including for async operations. These are finite
poll budgets, not elapsed-time deadlines. The final wire-drain loop leaves
unrelated interrupts enabled; endpoint state changes use short critical sections.
Async waiting for DMA itself has no
built-in wall-clock deadline. Cancellation retains resources as described above.

## UART details and the RX boundary

TX requests are enabled only after the DMA channel is programmed and owns its
static resources. TC is cleared before the first word. DMA TC means the last
word reached TDR, which is not necessarily the last stop bit on the wire. The
terminal hook removes DMATX; the final TXBUSY check covers both TDR and the
shift register. UART TC is sticky and is not used as a substitute for TXBUSY.
A prior queued transmission causes a before-start Busy error; flush it first.

RX starts only with no pending data/error. It enables only error interrupts,
not CPU receive-data interrupts; the ISR never reads RDR while DMA owns it.
The DMA terminal hook removes DMARX and disables RX acceptance. The operation
checks errors again, clears RC only with RX disabled after clean DMA TC, then
re-enables RX when returning the receiver. Framing/parity are checked on both
families; L012 additionally checks noise and overrun. F030 has no hardware
noise/overrun flags, so a DMA receiver cannot promise lossless input when the
sender overruns the hardware. Senders must respect the finite receive window.

The one-frame RX limit is deliberate. L012 RM1.4 says a new frame while RC is
still set causes ORE, and a normal RDR read does not clear RC. Both reference
manuals and vendor SDK examples show multiword DMA reception, but do not state
how DMA-specific RC acknowledgement interacts with that rule. The L012 SDK
1.0.5 multiword example neither clears RC nor checks ORE. The one-frame endpoint
avoids claiming an unproven recurring acknowledgement/error protocol. This is
a conservative safe API boundary, not a claim that hardware multiword RX is
unsupported. There is no RX-until-idle, circular DMA or ring-buffer interface.

## SPI details

SPI must drain one RX word for every TX word, including write-only protocols.
Use equal-sized explicit dummy TX or discard RX buffers for one-way protocols.
Both endpoint configurations are validated before either channel starts. RX
DMA is armed before TX DMA, and DMARX is enabled before DMATX. A TX completion
alone never releases resources. Either channel's error wins over its peer's TC;
both channel identities are poisoned together on incomplete cancellation/error.

After both clean TCs, the driver verifies peripheral errors, waits for TXE with
BUSY clear, and rejects unexpected residual RXNE instead of silently discarding
it. This handles the final clock edge after the last receive event. Error IRQs
use the bus's existing bound handler; no user buffer pointer is held by that
handler. External chip select belongs to the caller and must be managed around
the finite transaction, including failure paths. There is no hardware-CS DMA,
SPI target DMA, uneven-length DMA or DMA `SpiDevice` adapter in this release.

## Usage shape

Construct the existing UART/SPI bus and DMA channel owners from moved singleton
and pin tokens, not `reborrow()`. For an async UART TX operation:

```rust,ignore
let transfer = tx.write_dma(tx_channel, buffer, 100_000)
    .map_err(|failure| failure.error)?;
let done = transfer.await?;
let tx = done.uart;
let tx_channel = done.channel;
let buffer = done.buffer;
```

For full-duplex SPI:

```rust,ignore
let transfer = spi.transfer_dma(tx_channel, rx_channel, write, read)
    .map_err(|failure| failure.error)?;
let done = transfer.await?;
let spi = done.spi;
let tx_channel = done.tx_channel;
let rx_channel = done.rx_channel;
let write = done.write;
let read = done.read;
```

The snippets show ownership flow; handle a start failure's returned resources
as appropriate. Use a sound one-time static allocator for buffers. Do not create
multiple mutable references to a `static mut`, steal duplicate singleton tokens,
or recover retained resources through the unsafe PAC.

## Evidence and verification scope

Primary sources are pinned in [DMA hardware evidence](dma-hardware-evidence.md):

- L012 RM1.4 §8.8.4: UART/SPI TX buffer-empty and RX buffer-nonempty selectors
  for every DMA channel; §8.4/8.8.3: BLOCK, REPEAT and memory widths.
- L012 §21.6 and §21.7.1.5/.6: independent UART DMA enables, explicit 8-bit
  examples, final TXBUSY wait. §21.3.3 and §21.9.11/.12: RC and receive errors.
- L012 §22.7.1/.2: 8/16-bit SPI frame widths and CR2 DMATX/DMARX; §22.3.8:
  DR write clears TXE, DR read clears RXNE, and BUSY describes buffered/shifted
  activity. No F030 CR1 enable encoding is imported into this IP.
- F030 RM2.5 §18.6 and §18.7.1.5/.6: UART byte DMA and TXBUSY completion.
  §19.5 and §19.6.1.3: two-channel SPI DMA, per-frame request pacing and final
  BUSY wait; §19.8.1: CR1 DMA enables and frame width.
- [Embassy USART at b12a6d9](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/usart/mod.rs)
  and [SPI at the same revision](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/spi/mod.rs)
  inform channel/request traits, interrupt-bound constructors, RX-before-TX
  sequencing, joined completion and error handling. Their borrowed-buffer Drop
  assumptions are not CW32 hardware evidence and are intentionally not copied.

Host MMIO protocol checks use the actual maintained UART/SPI/DMA/RCC code and
generated PAC/route tables through an external fixture. They verify request
selection, addressing/count/width, RX-before-TX arming, joined completion,
peripheral and DMA error precedence, busy-tail timeouts, validation without
start, clean reclamation, and retained pins/clocks on drop/forget/error.
Separate external ARM compile probes check ownership, buffer lifetime, route
sealing, word width and IRQ binding rejection. These checks establish software
contracts; no on-silicon DMA throughput, overrun margin or abort timing is claimed.
I2C DMA remains an unsafe raw endpoint only pending a complete finite-protocol
proof for starts, addresses, byte pacing, acknowledgements, STOP and error drain.
