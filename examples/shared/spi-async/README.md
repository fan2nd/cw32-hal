# SPI byte master with interrupts

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

Two ordinary loopback applications use the nine remaining SPI families' existing
full-duplex master configuration and exact package pin routes. `async_master`
uses interrupt-assisted PIO and `embedded_hal_async::spi::SpiBus<u8>`; it needs
no DMA channel or private staging buffer. `blocking_master` uses the existing
blocking bus and includes one 16-bit in-place loopback exchange. The existing
[F030/A030/L083 paired DMA examples](../spi-dma/README.md) remain separate.

These applications use the default internal clock configuration, mode 0,
MSB-first order, no MISO pull, no sample delay, and a requested 1 MHz SCK
ceiling. The existing clock-envelope solver may select a lower actual rate.
Clock bounds do not qualify board wiring, voltage, signal integrity or latency.
They are source/build applications, not on-hardware or electrical validation.

## Wiring

For each bus listed for your selected package, connect its MOSI pin directly to
its own MISO pin. Do not connect two push-pull outputs together. No SPI slave is
needed. SCK and the active-low software CS can be observed with an appropriately
rated logic analyzer; leave them unconnected otherwise. Disconnect real slave
devices first: the byte strings and 16-bit blocking exchange are loopback data,
not a command protocol suitable for an arbitrary device. Check the exact board
schematic and electrical limits before running any firmware.

Every selected bus is used. Multi-bus rows require separate MOSI–MISO jumpers.
All pins are separate within a row. The CS column is GPIO, not hardware NSS.

| Feature | Package; Flash/RAM | SPI instance | SCK | MOSI | MISO | AF | CS |
| --- | --- | --- | --- | --- | --- | --- | --- |
| cw32f020c6u7 | QFN48; 32/8 KiB | SPI1 | PA5 | PA7 | PA6 | 5 | PB0 |
| same | same | SPI2 | PA2 | PA1 | PA0 | 5 | PB1 |
| cw32f002f3p7 | TSSOP20; 16/2 KiB | SPI | PA0 | PA1 | PA4 | 3 | PB0 |
| cw32f003e4p7 | TSSOP24; 20/3 KiB | SPI | PA0 | PA1 | PA4 | 3 | PB0 |
| cw32l010f8p6 | TSSOP20; 64/4 KiB | SPI | PA5 | PA6 | PA3 | 3 | PA4 |
| cw32l011k8t6 | LQFP32; 64/6 KiB | SPI | PA5 | PA7 | PA6 | 3 | PB0 |
| cw32l012c8t6 | LQFP48; 64/8 KiB | SPI1 | PA5 | PA7 | PA6 | 3 | PB0 |
| same | same | SPI2 | PA2 | PA1 | PA0 | 8 | PB1 |
| same | same | SPI3 | PA9 | PA11 | PA10 | 8 | PB2 |
| cw32l012c8u6 | QFN48; 64/8 KiB | SPI1 | PA5 | PA7 | PA6 | 3 | PB0 |
| same | same | SPI2 | PA2 | PA1 | PA0 | 8 | PB1 |
| same | same | SPI3 | PA9 | PA11 | PA10 | 8 | PB2 |
| cw32l031c8t6 | LQFP48; 64/8 KiB | SPI1 | PA5 | PA7 | PA6 | 5 | PB0 |
| cw32r031c8u6 | QFN48; 64/8 KiB | SPI1 | PA5 | PA7 | PA6 | 5 | PB0 |
| cw32l052c8t6 | LQFP48; 64/8 KiB | SPI1 | PA5 | PA7 | PA6 | 5 | PB0 |
| same | same | SPI2 | PA2 | PA1 | PA0 | 5 | PB1 |

The checked sources are each family's own
[serial AF sidecar](../../../cw32-data/af/),
[physical package map](../../../cw32-data/pinouts/README.md),
[exact part memory catalog](../../../cw32-data/parts.yaml) and generated package
metadata. F020's six routes are Table 5-3, printed p26 / PDF p27 of its own
current datasheet, as located in
[the F020 AF evidence](../../../docs/f020-serial-af-evidence.json).
No route disables debug, reset or oscillator functions. L010 uses PA3 for MISO
and PA6 for MOSI; PA7/PA8 remain SWD, PA0/PA1 remain available to the HSE wiring,
and PB0/PB1 remain available to the LSE wiring. Do not substitute L011's route.
F002/F003 likewise retain their different PA2/PA5 SWD assignment.

L012 C8T6 and C8U6 each select their own exact feature and metadata name;
C8U6 does not enable C8T6 as an alias. L012's own CN datasheet V1.0 Table 5-3,
printed p36 / PDF index38, supplies the selected AFs. Table 5-2, printed
pp32–34 / PDF indices34–36, supplies the same selected positions on both
packages: SPI1 SCK/MOSI/MISO pads15/17/16 and CS pad18; SPI2 pads12/11/10 and
CS pad19; SPI3 pads30/32/31 and CS pad20. GPIO CS does not qualify hardware
NSS. The generated pin metadata does not itself contain physical pad numbers.



## Application behavior

Each loop selects one bus, performs a seven-byte transfer against a three-byte
TX slice, a four-byte in-place transfer, an immutable-slice write, a read, the
empty byte operations and a final flush. The shorter TX is padded with zeroes;
write-only receives are discarded; reads generate clocks with zero dummy bytes.
The received arrays go through `black_box` for debugger inspection. No returned
value is asserted and no retry or peripheral recovery is added by the example.

The blocking application additionally exchanges two `u16` words using the
existing blocking method. The next byte operation restores its byte width.
There is no wider-word async API. CS stays low through each complete sequence
and becomes high only after all calls succeed.

L012 binds SPI1 separately and binds `SPI23` once to the complete
`SharedInterruptHandler<interrupt::typelevel::SPI23>`. Both SPI2 and SPI3
owners remain alive in the same main scope, and both are called on every pass.
The application uses the buses in sequence; it does not claim a runtime
concurrency or interrupt-latency measurement.

No operation is cancelled by these applications. If cancellation is later
added, keep the original device selected and await successful `flush` before
changing CS or releasing a device transaction. An error does not prove wire
idle: the applications park without unwinding, dropping the bus, changing CS
or retrying data. A dropped IRQ-PIO driver can destructively truncate a frame;
that is not a successful flush. `SpiBus` compatibility does not establish
cancellation-safe composition with a generic device adapter that releases CS
or its lock when a future is dropped. A hardware or clock fault may prevent
completion indefinitely; this example adds no timeout or throughput promise.

## Build

Select exactly one of the ten exact features above. A family alias, a
package-less memory-size alias, or multiple selections cannot substitute for
one of these packages. `build.rs` checks the selected chip name, family, SPI IP,
all controller addresses and IRQ owners, selected signal/AF pins, distinct CS
pins, and the exact metadata Flash/RAM regions. It derives `memory.x` from those
regions and emits `-Tlink.x`, including when built from the repository root.
Flash begins at 0x00000000 and RAM at 0x20000000; no bootloader offset or enlarged
memory fallback is supplied. F002 retains its actual 16 KiB Flash / 2 KiB RAM.

The release profile deliberately uses size optimization (`z`), LTO and one
codegen unit, with debug and incremental output disabled. The Cargo lock is
derived from the existing SPI-DMA example lock: the new root package adds only
a direct edge to the already locked `embedded-hal` 1.0.0. No dependency version,
source or checksum changes, and the existing pinned Embassy revision is used.

From the repository root, for example:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/spi-async/Cargo.toml --no-default-features \
  --features cw32f002f3p7 --bin async_master --message-format=json
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/spi-async/Cargo.toml --no-default-features \
  --features cw32l012c8t6 --bin blocking_master --message-format=json
```

Both bin names are explicit manifest targets. Builds only compile and link;
no flashing, execution or connected-device transaction is implied. Actual
acceptance commands, results and ELF section sizes belong to the separate
build receipts rather than this example's source.

The finite Stage94 main matrix includes `async_master` for exact C8T6 and
C8U6, retaining all three PIO buses and the complete SPI23 binding. Both
continue to use `new_async`; adding native DMA support does not change
that constructor's backend. C8U6's `blocking_master` route cfgs are widened
consistently with the new feature, preserving `new_blocking` and its
existing behavior. No C8U6 blocking ELF is selected in the Stage94 main or
clean matrices, so this feature-consistency change makes no new blocking
acceptance claim. The existing lockfile and release profile are unchanged.
The separate [native L012 DMA companion](../../l012/spi-dma/README.md) supplies
the four staged DMA/PIO deployments; its build receipts do not establish
execution or acceptance of this crate's blocking binary.

## Wider PIO frames

The separate `async_words` example exercises `word::U4`, `word::U9` and `u16` on the
same PIO owner, with typed trait calls and an unequal-length 9-bit transfer.
The backend settles each previous frame before changing width. This is a
loopback example; an attached slave must explicitly support its selected frame
widths and framing. Existing DMA owners remain byte-only and reject wider word
types before doing any work, even for empty calls. See
[the async word-width contract](../../../docs/spi-async-word-widths.md).
