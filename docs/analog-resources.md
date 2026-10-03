# Composing DAC, OPA and ADC resources

The L012 analog API supports independent DAC-channel ownership and sampling an
OPA output through the existing ADC channel API. Source guards retain every
required peripheral, output pin and bandgap lifetime. These APIs do not establish
board calibration, analog accuracy, conversion timing or motor safety.

## Independent DAC channels and live thresholds

`Dac::new` owns the unique DAC peripheral, initializes both internal channels to
zero with triggers/DMA/waves disabled, and waits 10 µs. External output pads stay
disconnected until attached. The startup delay remains a conservative software
choice around a typical specification, not a characterized maximum.

`Dac::split(self)` returns `(DacChannel<'d, 1>, DacChannel<'d, 2>)`, transferring
any attached output pins without writing registers. Either channel can be moved,
updated, attached to its verified external pin, or dropped independently. Both
retain the lifetime of the original DAC peripheral borrow. No peripheral or GPIO
token can be recovered or duplicated through this API.

The unsplit owner retains `set`, `set_pair`, `output_code`, `with_output1` and
`with_output2`. Both arguments of `set_pair` are validated before the single dual
holding-register write. Independent owners only write their own holding register.

Calling `channel.source()` reserves that channel with an exclusive lifetime-bound
`DacSource<'_, C>` guard. OPA and comparator constructors borrow the guard, while
`source.set(code)` takes a shared reference and remains available during their
lifetimes. The guard offers no enable, disable, route or pin-release operation.
Consequently an active consumer prevents dropping/reconfiguring its source, while
the sibling channel stays independently usable.

```rust,ignore
let (mut ch1, mut ch2) = analog::Dac::new(p.DAC, &mut delay).split();
let threshold = ch1.source();
threshold.set(1024)?;
let mut comp = analog::Comp::new_blocking_with_dac(
    p.VC1, p.PA0, &threshold, &bandgap, Default::default(),
)?;
comp.enable(&mut delay);
threshold.set(1536)?; // permitted while comp retains its source dependency
ch2.set(2048)?;      // independent channel
```

The DAC source is typed by channel. Generated, sealed connection traits restrict
OPA1 to channel 1, OPA2 to channel 2, VC1/VC3 to channel 1 and VC2/VC4 to channel 2.
Those connections come from sourced instance metadata rather than runtime
instance-number arithmetic. Multiple consumers can borrow the same correctly
paired source guard. A code update changes their shared analog signal; the caller
must allow the DAC and downstream analog response/settling time before treating a
sample or comparator result as stable. The API does not promise phase-atomic live
threshold updates or synchronous dual-channel updates after splitting.

Every shared DAC CR0/CR1 update after construction uses a critical-section RMW.
Dropping one channel disconnects only its external route, clears only its enable
bit, then disconnects only its owned pin. Sibling fields and reserved bits are
preserved. Each channel retains an RCC guard, so the gate remains enabled until
the final channel is safely shut down. The whole DAC has no separate shutdown
that can run after its channels have been transferred.

## OPA output as an ADC channel

`Opa::output(&mut self)` returns an `OpaOutput<'_, I>` only while the OPA is enabled,
not calibrating and known to have completed startup/settling. This output guard
implements the sealed `AdcChannel` contract for every verified ADC route of the
OPA's owned output pad. It can be passed to blocking/async reads, or reborrowed or
erased for a sequence using the existing channel API.

```rust,ignore
let mut opa = analog::Opa::follower(
    p.OPA1, p.PA3, p.PB0, &bandgap, Default::default(), &mut delay,
)?;
let mut output = opa.output()?;
let raw = adc.blocking_read(&mut output, sample_time, poll_budget)?;
```

The borrow does not consume, recreate or reconfigure a second PB0/PB1 token. It
keeps the OPA, its input/output pins, bandgap and optional DAC source dependency
alive for the entire ADC borrow, sequence or future. The owner cannot be
calibrated or dropped while its output remains borrowed. ADC type erasure retains
that lifetime. Finishing or dropping a temporary ADC channel borrow leaves the
OPA enabled and its pad owned by the OPA.

The build script joins OPA `OUT` and ADC `IN` metadata by their actual analog pad:

| OPA output | Owned pad | ADC1 channel | ADC2 channel |
| --- | --- | --- | --- |
| OPA1 | PB0 | 8 | 3 |
| OPA2 | PB1 | 9 | 4 |

This is sampling the documented OPA output pad, not a newly invented internal ADC
mux channel. Existing pin ownership still prevents an external DAC output and an
OPA from driving the same pad simultaneously.

OPA calibration marks the output unready before triggering hardware. Only an
observed busy-to-idle completion followed by the settling delay restores
readiness. Timeout or an interrupted settling delay leaves `output()` unavailable;
a successful subsequent calibration or reconstruction is required. Zero budget
and invalid arguments do not disturb a previously ready output. These readiness
checks do not certify calibration quality or analog settling after a live input
or DAC-code change.

## Evidence and verification boundary

- [Pinned Embassy `opamp.rs`, lines 580–595](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/opamp.rs#L580-L595)
  implements the sealed ADC-channel contract for borrowed opamp outputs.
- [Pinned Embassy `dac/mod.rs`, lines 1021–1035](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dac/mod.rs#L1021-L1035)
  transfers independently owned DAC channels through `split`.
- [CW32L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf)
  Table 25-4, printed p578 (PDF p604), supplies ADC input routes; Figure 29-1 and
  Table 29-1, printed p646 (PDF p672), supply OPA internal DAC input and output-pad
  routes. Section 29.4, printed p647, explicitly describes sampling OPA output
  through ADC. DAC operation and control/data register semantics are in chapter
  26; comparator source selection and pair metadata are described in chapter 27.

External source-bound register probes exercise split/no-write behavior, typed
single-channel updates, invalid-code rejection, sibling/reserved-bit preservation
on output attachment and both drops, pin shutdown, retained gates, output-borrow
readiness, calibration timeout/recovery, interrupted settling, and updates through
an OPA's live DAC dependency. Host instrumentation substitutes register reads and
writes while including the actual driver sources and generated register types;
clocks, pin callbacks and calibration-status progression are controlled by the
probe. ARM compilation and negative ownership/pairing probes check the real
public API. These checks do not replace electrical or hardware timing tests.
