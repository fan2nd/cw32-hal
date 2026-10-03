# Internal ADC sources

The ADC owner now provides `blocking_read_internal(source, sample_time, delay,
poll_limit)` in either mode and `read_internal(source, sample_time, delay).await`
in Async mode. Both return a raw, right-aligned 12-bit `u16`. The delay must be a
real `embedded_hal::delay::DelayNs` implementation; a no-op delay does not meet
the electrical contract. The poll limit counts observations, not elapsed time.

Internal sources implement the sealed `InternalSource` contract. They cannot be
constructed from channel numbers and do not implement `AdcChannel` or
`BorrowedChannel`. No public internal-source sequence, hardware trigger,
watchdog, paired-ADC or DMA API is exposed. Each read uses a private one-element
sequence and the same conversion completion/cancellation machinery as an
ordinary read. A pending future retains the ADC, source and its dependencies.
Timeout or cancellation stops the conversion before releasing those borrows.
Existing external-pin/OPA sequences and static DMA quarantine remain unchanged.
An active or poisoned DMA lease rejects an internal read before source/ADC MMIO.

## CW32L012

| Source | Internal channel | Resource contract |
| --- | --- | --- |
| `VrefInt::new(&bandgap)` | IN15 | Shared borrow of the existing `analog::Bandgap` owner |
| `Temperature::new(&bandgap)` | IN14 | Shared borrow of the same BGR owner; coexists with OPA/VC users |
| `DacInput::new(&mut dac_source_1)` | IN13 | Exclusive borrow of the actual `DacSource<D, 1>` |
| `DacInput::new(&mut dac_source_2)` | IN12 | Exclusive borrow of the actual `DacSource<D, 2>` |

BGR/TS reads set their enable bits using a critical-section read/modify/write
and wait 32 microseconds on every operation. The reference manual specifies
approximately 30 microseconds for both startup paths (§25.9 and §25.12.19).
Both also require at least 40 microseconds of actual sample time (§25.8 and the
ADCx_SAMPLE footnote in §25.12.3). Too-short sample times return
`Error::SampleTimeTooShort` before register writes. Validation compares sample
cycles against the owner's retained PCLK and divider using integer cross
multiplication, without rounding a fractional ADC clock down.
At 6 MHz, `Cycles262` is about 43.7 microseconds and is sufficient; `Cycles198`
is not. Faster conversions are not silently substituted.

BGREN and TSEN remain enabled when the source token is dropped. BGR is shared
with ADC/VC/OPA and can also be activated by hardware; the second ADC may still
be converting after a forgotten future. Avoiding a shared shutdown deliberately
costs some power. There is no safe shutdown method in this API.

`DacInput` borrows the actual source, rather than a mutable reference to a
shared reference. This prevents every `DacSource::set(&self)` alias and existing
OPA/VC source dependency from being used while the guard is live. The DAC owner,
clock and any external output pin remain retained. Each ADC operation waits
10 microseconds after the most recent possible write, using the same
conservative delay policy as `Dac::new`; this is not a characterized maximum
settling time or a guarantee for every load. Dropping the input guard makes the
existing dynamic DAC source available again, without modifying the DAC.

```rust
let bandgap = analog::Bandgap::new(p.BGR, &mut delay);
let mut source = adc::Temperature::new(&bandgap);
let raw = adc.blocking_read_internal(
    &mut source, adc::SampleTime::Cycles262, &mut delay, 10_000,
)?;

let (mut one, mut two) = analog::Dac::new(p.DAC, &mut delay).split();
let mut dac_source = one.source();
dac_source.set(2048)?;
{
    let mut input = adc::DacInput::new(&mut dac_source);
    let raw = adc.blocking_read_internal(
        &mut input, adc::SampleTime::Cycles70, &mut delay, 10_000,
    )?;
}
dac_source.set(1024)?;
```

## CW32F030

`VrefInt::new()`, `Temperature::new()` and `Vdda::new()` select IN15, IN14 and
IN13 respectively. These values do not themselves touch hardware or assert
that a source is enabled. The dedicated read exclusively owns the ADC control
and conversion lifetime and activates the selected signal as part of the read.

Every internal conversion uses MODE=0 (single-channel single-shot), BUF=1,
right alignment and the existing VDDA reference, as required by RM §22.4.3.
No source can enter the public MODE=4 scan path. The 200 ksps follower limit
in §22.13.1 is checked using the selected sample cycles plus the 19-cycle
comparison phase. The driver's existing 500 kHz maximum ADCCLK already keeps
all settings below about 20.84 ksps.

BGR and temperature reads enable BGREN; temperature reads also enable TSEN
and preserve it through the conversion control writes. BGREN remains sticky
and is never cleared by ADC drop. TSEN may remain set until the next external
configuration or ADC drop. BIAS, reserved CR0 fields and unrelated peripheral
or shared NVIC state are preserved. Each internal read waits 40 microseconds:
RM §23.3.1 gives roughly 20 microseconds for BGR startup and §22.4.1 gives
roughly 40 microseconds for ADC startup. The temperature section (§22.10)
specifies no independent maximum TS settling time, so the chosen extra wait
does not establish a sensor accuracy guarantee.

```rust
let mut temperature = adc::Temperature::new();
let raw = adc.blocking_read_internal(
    &mut temperature, adc::SampleTime::Cycles10, &mut delay, 10_000,
)?;
// With Adc<I, Async> and a checked ADC interrupt binding:
let raw = adc.read_internal(
    &mut temperature, adc::SampleTime::Cycles10, &mut delay,
).await?;
```

`Vdda` samples VDDA/3 against VDDA. Its code is therefore ratiometric and does
not measure the absolute supply voltage. `VrefInt` is a nominal 1.2 V source,
not an assumed exact calibration constant.

## Calibration and upstream alignment

The implementation was compared with
[Embassy ADC internal sources at the pinned commit](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/adc/internal.rs).
It follows named, sealed source types and the existing `Adc<I, Mode>` owner.
It intentionally does not copy STM32's generic `AdcChannel` implementation or
its unowned numeric DAC selection: CW32 internal-mode restrictions and shared
analog resources require the narrower read contract above.

Neither CW32 ADC chapter specifies a software self-calibration command for
this path. The documented temperature calibration is factory data, not an
initialization procedure that this driver can safely imitate. No temperature,
VDDA or millivolt conversion, factory-data read, or accuracy claim is exposed.
L012 §25.9 uses temperature byte 0x001007CD and trim word 0x001007CE; §25.8
uses a per-chip BGR voltage word at 0x001007D2. F030 §22.10 uses temperature
byte 0x00012609 and trim words at 0x0001260A/0x0001260C for the internal
1.5 V/2.5 V references. Those F030 formulas do not directly apply to this
driver's VDDA-reference readings. A future calibrated API needs separately
validated factory data, reference selection and units.

Hardware evidence: CW32L012 reference manual Rev 1.4, §§25.8, 25.9,
25.12.3, 25.12.19 and ADC channel selection; CW32x030 reference manual Rev 2.5,
§§22.4.1–22.4.3, 22.5.1, 22.10, 22.13.1 and 23.3.1. Tests are software/MMIO
and ownership checks only; no target has been flashed or electrically measured.
