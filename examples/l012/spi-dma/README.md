# Native L012 staged SPI DMA applications

Four ordinary firmware applications use native SPI1/SPI2/SPI3 on the exact
CW32L012C8T6 (LQFP48) and CW32L012C8U6 (QFN48) packages. Each package has 64 KiB
Flash and 8 KiB SRAM. Transfers are finite, full-duplex, eight-bit master
operations with private static SRAM staging. Retained PIO peers use the
ordinary `Spi::new_async` constructor and no DMA channel.

## Separate deployments

Each row is a separate binary, with all its owners retained in one infinite
application loop. Buses exchange sequentially while their peers remain owned.
Nothing is dropped and reconstructed to re-admit a channel.

| Deployment / binary | DMA owners, RX then TX | Retained PIO owners | Stage bytes |
| --- | --- | --- | --- |
| A / `spi1_dma_pio23` | SPI1 RX CH1, TX CH2 | SPI2 and SPI3 | 40 |
| B / `spi2_dma_pio3` | SPI2 RX CH1, TX CH3 | SPI3 | 40 |
| C / `spi3_dma_pio2` | SPI3 RX CH2, TX CH4 | SPI2 | 40 |
| D / `spi23_dma_pio1` | SPI2 RX CH1, TX CH2; SPI3 RX CH3, TX CH4 | SPI1 | 80 |

The constructor's channel argument order is TX then RX. Each DMA owner receives
its own distinct `cortex_m::singleton!` TX `[u8; 16]` and RX `[u8; 24]` arrays;
the smaller array determines the 16-byte chunk capacity. Caller buffers remain
ordinary borrowed slices. A/B/C consume two of the four native channels; D
consumes all four. Three simultaneous full-duplex DMA buses would require six
channels and are outside this hardware's capacity. RX has higher priority than
its paired TX in every row, without a lossless-throughput guarantee.
Both channel admissions are checked before either is claimed. Channel rejection
leaves both admission states unchanged, but constructor inputs remain consumed
and later SPI configuration failure does not roll back successful admission.

SPI1 binds its dedicated handler on IRQ25. Every selected SPI2/SPI3 deployment
binds the complete `SharedInterruptHandler<interrupt::typelevel::SPI23>` on
IRQ26, including mixed PIO/DMA ownership. DMACH12 is IRQ9 for CH1/CH2; DMACH34
is IRQ10 for CH3/CH4. B and C each pair channels across these two vectors.
When a vector has two channel handlers, each retains its separate channel
ownership. IRQ11 is CORDIC, never a DMA vector.

## Exact wiring and source boundary

For loopback, connect each used MOSI only to its own MISO. Disconnect external
SPI slaves before running these arbitrary byte exchanges. Do not connect two
push-pull outputs together. SCK and GPIO CS can be observed with a suitably
rated logic analyzer or left unconnected. Check the board schematic and
electrical limits before any hardware use.

Both exact packages expose these selected routes. GPIO CS is an application
pin; it does not qualify a hardware NSS route.

| Controller | SCK / MOSI / MISO | AFs | Physical pads | CS / pad | DR address | RX / TX requests |
| --- | --- | --- | --- | --- | --- | --- |
| SPI1 | PA5 / PA7 / PA6 | 3 / 3 / 3 | 15 / 17 / 16 | PB0 / 18 | `0x4000081c` | 6 / 7 |
| SPI2 | PA2 / PA1 / PA0 | 8 / 8 / 8 | 12 / 11 / 10 | PB1 / 19 | `0x4000041c` | 8 / 9 |
| SPI3 | PA9 / PA11 / PA10 | 8 / 8 / 8 | 30 / 32 / 31 | PB2 / 20 | `0x4000341c` | 10 / 11 |

L012's own CN datasheet V1.0 Table 5-3, printed p36 / PDF index38, supplies the
selected AFs. Table 5-2, printed pp32–34 / PDF indices34–36, supplies these
package positions. See the [native SPI DMA evidence](../../../docs/l012-spi-dma-evidence.json),
[L012 AF sidecar](../../../cw32-data/af/) and
[physical package maps](../../../cw32-data/pinouts/README.md). Physical pad numbers
are source-ledger facts; generated pin metadata has no physical-position field.
The selected routes preserve debug, reset, BOOT and oscillator pads.

Native DMA SIZE selects the SRAM element width. Byte staging does not imply
CPU byte MMIO or byte peripheral accesses: native DR remains a typed 32-bit
register container with a 16-bit payload. L012's own register/request evidence
qualifies these endpoints; the classic-family register layout is not reused.

## Application behavior and ownership

Each bus has a separate GPIO CS initialized high. Its ordinary exchange calls
all five actual `embedded_hal_async::spi::SpiBus<u8>` methods:

- A 43-byte RX against shorter TX, padding the remaining TX bytes with zeroes
- A longer immutable TX against a seven-byte RX, discarding excess RX
- A 35-byte in-place transfer, a flash-resident write longer than 16 bytes,
  and a nonempty read that transmits zero dummy bytes
- Empty write, read, transfer and in-place operations, then a successful flush

The same exchange runs on retained PIO peers. Received arrays are preserved
with `core::hint::black_box` only; no loopback value or device response is
asserted. These are ordinary applications, with no cancellation, `forget`,
dirty-admission, fault-injection or lifecycle probe.

CS stays asserted through the complete exchange and final successful flush.
Any initialization, construction, transfer or flush error parks without
unwinding, releasing resources, changing CS, resetting clocks or retrying
bytes. If cancellation is later added, the original device must remain
selected until a subsequent successful flush. Static staging protects caller
buffer lifetime; it does not by itself prove wire drain or establish `Send`
or `Sync`. An unresolved/error path does not authorize CS release. Generic
device adapters that release CS or locks when a future is dropped do not
satisfy this contract.

The examples use default internal clocks, mode 0, MSB first and a requested
1 MHz SCK ceiling. Arbitration, CPU copies and scheduling can introduce chunk
gaps. No streaming, timeout, recovery, measured throughput, simultaneous
traffic, interrupt-interleaving or electrical validation is claimed. See
[the driver contract](../../../docs/spi-dma.md) for retained cancellation/error
ownership and the native completion boundary.

## Build and acceptance boundary

Select exactly one of `cw32l012c8t6` and `cw32l012c8u6`. `build.rs` requires the
matching exact metadata name and L012 line, checks the three native SPI bases
and register versions, all six request selectors, the selected pins/AFs and
distinct CS pins, complete SPI vector ownership, four native DMA channels and
their two actual vectors, and CORDIC IRQ11. It derives `memory.x` only from
the selected package's Flash `[0, 0x10000)` and SRAM
`[0x20000000, 0x20002000)` metadata. There is no generic-family memory fallback.

All four bin names and source paths are explicit manifest targets. The release
profile uses size optimization (`z`), LTO and one codegen unit, with debug and
incremental output disabled. The new lockfile retains the existing classic
SPI-DMA example's complete pinned dependency graph, changing only the root
package name. No dependency version, checksum, source or Embassy revision is
upgraded.

For example, from the repository root:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/l012/spi-dma/Cargo.toml --no-default-features \
  --features cw32l012c8t6 --bin spi1_dma_pio23 --message-format=json
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/l012/spi-dma/Cargo.toml --no-default-features \
  --features cw32l012c8u6 --bin spi23_dma_pio1 --message-format=json
```

The finite Stage94 software matrix selects A/B/C/D on both exact packages in
main and A/B/D on both packages in clean acceptance. Ordinary pure PIO rows
live in [the separate SPI async crate](../../shared/spi-async/README.md); existing native
copy/UART-TX and classic SPI-DMA applications remain separate regressions.
Only corresponding retained build receipts can establish successful links.
These commands and sources make no hardware-execution claim and do not
exercise cancellation, faults, IRQ races, generic rejection or data integrity.
