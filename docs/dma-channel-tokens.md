# Direct DMA channel singletons in v0.23.0

DMA channel ownership now begins at `Peripherals`, alongside GPIO pins and other
peripheral instances. The application does not obtain a controller and partition
it later.

```rust,ignore
let p = embassy_cw32::init(Default::default());
let mut channel = embassy_cw32::dma::Channel::new_blocking(p.DMACHANNEL2);
```

For interrupt-driven transfers on L012:

```rust,ignore
use embassy_cw32::{bind_interrupts, dma, peripherals};

bind_interrupts!(struct Irqs {
    DMACH12 => dma::InterruptHandler<peripherals::DMACHANNEL1>,
               dma::InterruptHandler<peripherals::DMACHANNEL2>;
});

let first = dma::Channel::new(p.DMACHANNEL1, Irqs);
let second = dma::Channel::new(p.DMACHANNEL2, Irqs);
```

The second handler is a separate proof even though both channels use the same
physical vector. One channel's binding does not permit constructing the other.
F030 uses DMACH1 for channel 1, DMACH23 for channels 2/3 and DMACH45 for 4/5.

## Where the identities come from

The pinned Embassy [build script](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs#L288-L291)
adds one singleton for every `METADATA.dma_channels` entry, then emits those
identities in its peripheral definitions and `Peripherals` structure. CW32 now
uses that same ownership granularity. It takes names from the existing audited
`DmaChannel.peripheral` records: `DMACHANNEL1` through `DMACHANNEL4` on L012 and
through `DMACHANNEL5` on F030. No invented DMA1 controller or duplicate token
alias is needed. The `dma::Instance` types and existing typed request bounds
retain their names.

The YAML, persisted JSON and PAC metadata already contain the channel number,
register-bank index, parent, request selectors and physical interrupt. Their
schema and register data do not change in this release. The HAL build script
uses those records to emit the singleton fields, instance/interrupt identities
and UART/SPI request traits. ADC retains its actual generated request identity.

`ownership_parent: DMA` continues to describe the physical bank and shared reset
domain. It does not require a public HAL owner for the whole controller. The
PAC still exposes the real controller registers and channel register views.
The HAL has no `Peripherals::DMA` field, `peripherals::DMA` token type,
`dma::Controller`, `dma::Channels` or `dma::split` function. Thus there is no
second safe ownership path covering all channel tokens. This is an explicit
CW32 choice: the pinned upstream build script also retains some controller
singleton names; its commented-out DMA exclusions are not evidence that every
upstream controller name is hidden.

## One reset, independent lifetimes

Global `try_init` resets the complete DMA domain exactly once while initialization
is exclusive, after RCC initialization and before returning any peripheral token.
The private initialization function is unsafe because it may only be used before
channel ownership escapes. A failed hardware initialization cannot later retry
and expose another singleton set.

`Channel::new` and `Channel::new_blocking` acquire the same counted physical gate
without requesting reset. Constructing a later sibling therefore does not clear
an active transfer, endpoint lease, pending flag or quarantined channel state.
Dropping the last clean channel releases the gate; constructing another channel
afterward enables it without resetting the controller. Unused singleton tokens
do not retain a clock. Forgetting a constructed owner retains its guard, and
quarantine permanently pins the gate when bus inactivity is unproven.

Each handler and transfer cleanup remains channel-local. Shared IRQs are not
unpended, reprioritized or disabled by another channel's constructor or Drop.
Each channel has one persistent software state, including across constructing a
new owner from a reborrowed token. Cancellation cannot clear poison by dropping
and reconstructing the owner.

These changes do not establish a DMA abort/drain guarantee. Safe buffers remain
static and are returned only after clean completion. Finite ADC/UART/SPI
endpoints retain their documented resource ownership and quarantine contracts;
see [DMA](dma.md) and [bus DMA](bus-dma.md).

## Application migration

Replace:

```rust,ignore
let channels = dma::split(p.DMA);
let channel = dma::Channel::new_blocking(channels.ch2);
```

with:

```rust,ignore
let channel = dma::Channel::new_blocking(p.DMACHANNEL2);
```

A function that formerly accepted the whole controller should accept the actual
channel token it uses. A function already accepting
`Peri<'d, peripherals::DMACHANNEL2>` keeps that signature. No runtime channel
registry, channel-number allocator or extra owner wrapper is introduced.

Examples 02–05 construct channel 2 directly; example 06 passes `p.DMACHANNEL2`
to its existing motor startup function. The DMA reset moves to global init;
channel register programming, ADC request setup, raw repeating-transfer contract,
ISR bodies, control algorithms, protection logic and executor priorities retain
their existing order and behavior. Validation is recorded in
[v0.23.0 checks](validation-v0.23.0.md); no physical-board verification is claimed.
