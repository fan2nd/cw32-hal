# DAC instance identity and resource ownership

The L012 DAC uses a sealed, metadata-generated `DacInstance` capability. Its
constructor consumes `Peri<'d, T>` where `T: DacInstance`, and every register
access uses that instance's `regs()`. Clock acquisition uses `T::acquire()`;
external pins require `SignalPin<T, C>`. There is no concrete `pac::DAC` or
`peripherals::DAC` dependency in the DAC driver.

The types retain the peripheral identity throughout resource composition:

```rust
Dac<'d, T>
DacChannel<'d, T, C>
DacSource<'a, T, C>
adc::DacInput<'a, 'd, T, C>
```

Normal construction still infers `T` from the peripheral token:

```rust
let dac = analog::Dac::new(p.DAC, &mut delay);
let (mut one, mut two) = dac.split();
let source = one.source();
source.set(2048)?;
two.set(1024)?;
```

Explicit type annotations now include the DAC identity, for example
`Dac<'d, peripherals::DAC>` or `DacSource<'a, peripherals::DAC, 1>`.
`Dac::split` consumes the whole owner and transfers two disjoint channel
capabilities. It does not clone, duplicate or reconstruct a `Peri` token.
Both channels retain the original peripheral lifetime and their own clock
guard; dropping one disables and disconnects only that channel. The last live
clock guard releases the shared DAC gate under the existing RCC contract.

## Pin and internal-source routes

The generated `DacSourceInstance<D, C>` implementation belongs to the OPA or
comparator that can consume channel `C` of DAC peripheral `D`. Both the source
peripheral and channel are part of the bound. Generation resolves the target
named in each audited connection and validates its IP kind and version.
A channel number alone cannot authorize a connection to another DAC identity.

OPA and comparator constructors verify this bound before storing a shared
dependency reference. Erasing the concrete dependency type inside those owners
preserves the actual source borrow. The source can still update its one holding
register while consumers are live, but its channel cannot be reconfigured or
dropped during that borrow. No consumer gains a second peripheral token.

The ADC's `DacInput` holds `&mut DacSource<'d, T, C>`, retaining both the actual
source lifetime and its peripheral type. This exclusive reservation prevents
source writers and OPA/VC source borrows from coexisting with a live ADC input.
Dropping the input releases only the reservation and does not alter DAC state.

The ADC internal-source contract remains deliberately limited to the audited
L012 global DAC: channel 1 is IN13 and channel 2 is IN12 on both L012 ADCs.
Build-time validation requires a single L012 DAC for that bridge. These fixed
internal ADC selections are not a claim that an arbitrary future DAC can feed
every ADC. Supporting multiple internal DAC domains or different ADC routes
requires corresponding route metadata and an ADC-specific source contract.

## Comparison with the pinned Embassy implementation

The reference is
[Embassy STM32 DAC at b12a6d9efcd2711037abca1b63a661a9ef726444](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dac/mod.rs).
Its `Dac::new_blocking` and `Dac::new_internal` constructors are generic over
`T: Instance`. Its generated sealed instance implementation supplies register
and RCC identity through `Info`; the stored owner and channels erase `T`.
Therefore, an erased upstream owner is not evidence that a constructor should
hardcode a concrete peripheral.

CW32 follows the same instance-driven register, clock and pin selection, while
retaining `T` in the owner, channels and source guards. Retaining that identity
keeps the DAC-to-OPA/VC pairing available to the type checker after a split.
The CW32 API still has its existing two-channel startup, raw 12-bit codes,
settling delays and synchronous writes. It does not add STM32 waveform,
trigger, DMA or ring-buffer capabilities.

## Shared bandgap scope

`Bandgap` remains a concrete owner of the one L012 BGR domain. It is a shared
startup witness with sticky enable behavior: ADC temperature/reference inputs,
OPAs and comparators borrow that same witness, and its drop never shuts their
source down. It has no independent peripheral gate or reset. Genericizing the
witness without adding explicit consumer-domain relationships could accept a
witness for the wrong source. This shared-domain contract is separate from
the per-instance DAC, OPA and comparator contracts.

## Validation boundaries

External compile probes cover generic construction and pin bounds, generated
OPA/VC source pairing, split ownership, retained tokens and pins, live shared
source updates, and exclusive ADC input reservations. Source-based register
models check preserved writes and drop behavior. A second synthetic DAC
identity with a distinct register range and clock verifies that the driver
does not fall back to the concrete production DAC; this is a test fixture,
not a claim of a second DAC on the supported silicon.

These checks establish software routing and ownership behavior. They do not
measure analog accuracy, startup maxima, board loading or motor safety.
