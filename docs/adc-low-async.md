# L010/L011 interrupt-driven software ADC

Source-qualified scope, 2026-10-09. The existing L010/L011 sequence backend exposes
software single reads and ordered 1–8-slot scans through `Adc<'d, T, Async>`.
Each family still selects its own PAC and independently qualified electrical
metadata. Existing blocking constructors and reads remain available. This document
supersedes older statements that software IRQ reads/scans on these two families
are unimplemented. The separate [classic/L012 ADC1 scope](adc-remaining-async.md)
records their own-source implementation and L012 ADC2 boundary.

## API and ownership

Bind the metadata-selected ADC interrupt with the normal HAL macro:

```rust,ignore
bind_interrupts!(struct Irqs {
    ADC => adc::InterruptHandler<peripherals::ADC>;
});
let mut adc = adc::Adc::new_async(p.ADC, Irqs, config)?;
let sample = adc.read(&mut pin, adc::SampleTime::Cycles102).await?;
let timing = adc.read_sequence(&sequence, &mut samples).await?;
```

`new_async(peripheral, binding, config)` returns `Result<Self, Error>`. The binding implements
`Binding<T::Interrupt, InterruptHandler<T>>`. `read(channel, sample_time)` accepts
the existing `BorrowedChannel` forms and returns `Result<u16, Error>`.
`read_sequence(&[(&BorrowedAdcChannel<T>, SampleTime)], &mut [u16])` returns
`Result<SequenceTiming, Error>`. A sequence has 1–8 slots and the output slice must
have exactly the same length. Each slot can choose its acquisition time; repeated
references to one channel handle are valid and do not duplicate pin ownership.
The common clock must satisfy every requested slot.

The future retains exclusive access to the ADC and the required pin/channel,
sequence and output borrows until completion or drop. Channels remain
package-qualified and GPIO analog setup uses the existing typed route. No raw
mux constructor, buffer pointer in the ISR, DMA transfer, dynamic ISR registry or
cross-family register adapter is introduced. Async construction returns
`Error::UnsupportedConfiguration` for inherited hardware-trigger or watchdog-enable
state instead of adopting it. Watchdog status flags remain untouched by owned
completion acknowledgment.

`triggered::Btim1Adc` remains a separate owner of ADC, BTIM1 and its input pin, with
its existing terminal cancellation policy. Both owners require the unique ADC
token. There is no conversion or escape hatch between the software async owner
and that externally triggered owner.

## Interrupt and result contract

Only EOS is enabled as a completion interrupt, for both one slot and eight slots.
The handler checks its source enable and pending status, masks EOS under the
appropriate critical section and wakes the per-instance upstream `AtomicWaker`.
It checks masking with one conservative readback. If EOS remains enabled, it
latches the per-instance terminal fault and disables only the metadata-proven
dedicated ADC vector before waking the task, preventing an interrupt storm.
Normal cancellation never disables NVIC. Task-side setup/stop/finalization
readbacks use the finite `Config.timeout` budget.
The handler leaves EOS status retained for the task. A spurious or stale pending IRQ with
no enabled EOS does nothing. EOC is observed by the task, not enabled as a
per-slot wake source. The handler never copies results, holds a user buffer
pointer, disables an unrelated NVIC line or touches a comparator/timer owner.

The task registers its waker before its final completion check. A completion
before registration remains observable in the retained EOS flag, preventing a
missed-wake dependency. After EOS, the future requires EOC and `START=false`
before reading exactly the requested RESULT slots. If those terminal observations
are not yet all visible, it uses a finite synchronous finalization poll. It must
not return `Pending` indefinitely after EOS has been masked, since another IRQ is
not guaranteed. Exhausting that poll returns an error and runs cleanup.

No output is copied until all terminal observations succeed; there is no await
between the final check and complete ordered copy. The task stages results locally,
verifies completion masking and acknowledgment, then copies the whole requested
slice. Cancellation or error leaves the caller slice unchanged. Samples are
right-aligned 12-bit raw counts, including
temperature and bandgap readings. ISR/RESULT reads do not acknowledge events.
Typed ICR writes clear EOC/EOS with zero and preserve AWDL/AWDH with one, following
the own-family R1W0 contract; ISR is read-only.

With `CONT=false`, all hardware triggers disabled and no raw register interference,
one software start executes one sequence and stops. Each slot has a retained
result register, so delayed task collection cannot be overwritten by a later
automatic scan. No OVR flag, FIFO, independent BUSY/abort-ack flag or generation
counter is defined in the reviewed ADC chapters. This is not a streaming or
external-interference detection guarantee.

## Cancellation, forgotten futures and terminal faults

Drop masks owned EOC/EOS enables, writes `START=false`, performs bounded control
readback, shuts down owned conversion/temperature circuitry as needed, and clears
only EOC/EOS. The manuals document software START zero as stop plus sequence-cursor
reset, so a successful logical cancellation permits reuse. The next read
reestablishes stopped/cleared state, programs the requested common clock and slots,
reinitializes the ADC and settles required analog sources before a fresh start.
Configuration or sequence rejection does not expose partial output.

If required stop, completion-mask or reinitialization readback cannot be
established within the finite budget, the driver latches a terminal fault in
per-instance state. Drop cannot return an error, so later operations report the
fault as `Error::Faulted`. Reconfiguration, dropping/replacing the owner and switching between
blocking and async software owners cannot clear it; recovery requires chip reset.
An ordinary successful cancellation does not itself poison the owner.

Forgetting a future skips its destructor and does not promise peripheral
quiescence. The ISR still has no pointer to caller memory. Every later software
operation explicitly establishes fresh stopped/cleared state rather than relying
on the earlier future having been dropped normally.

START readback establishes the RW control field under the documented logical
stop/reset behavior. It is not an independent hardware drain acknowledgment.
Neither manual supplies a maximum abort latency, an instantaneous analog-sampling
cutoff or an in-flight result-write drain handshake. No guessed delay based on
conversion cycles upgrades these limits. Concurrent raw PAC access that changes
conversion, trigger, clock, interrupt or shared-reference state is outside the safe
ownership guarantees.

## Deadlines and progress

`Config.timeout` is a finite synchronous poll-attempt budget for gate/setup,
stop and EOS finalization checks. It is not microseconds, an IRQ retry count,
or a bound on total await time. If EOS never arrives, an unwrapped ADC future may
remain pending indefinitely. Callers requiring a deadline compose with the pinned
`embassy_time::with_timeout` or `embassy_futures::select`; the losing/dropped ADC
future executes its cleanup guard. A timeout needs a functioning caller-provided
time/wake source. The HAL ADC itself does not require an Embassy time driver.

Progress requires running ADC/PCLK, enabled CPU interrupts, the correct typed IRQ
binding, a functioning executor and eventual scheduling. A caller deadline only
takes effect when polled; bounded synchronous startup/finalization/cleanup work
contributes latency. There is no promised maximum IRQ latency, executor latency
or complete-operation wall time. Ordinary run-mode IRQ use is not qualification
for deep sleep, stopped clocks or debugger halts.

## Electrical and neighboring-owner limits

The existing family-specific supply bands, acquisition floors, sample-rate limits
and exact rational `ClockBounds` apply to every slot. Fast-clock bounds enforce
maximum rate/clock and minimum acquisition; slow-clock bounds enforce L011's
4 MHz floor. L010 has no invented minimum clock. L011 retains the manual's
conservative 48 MHz ceiling despite its datasheet's conflicting 96 MHz maximum,
its 1 MSPS limit and the board obligation `VDDA=VDD`. L010 retains its independently
qualified 2 MSPS upper band. These maxima remain subject to supply bands.

Available divisors are PCLK/1, /2, /4 and /8; samples are 6–390 cycles plus 15
comparison cycles. TS and BGR require at least 40 microseconds acquisition,
separate from startup. ADC startup retains max(2 microseconds, 15 ADC cycles),
computed with fast HCLK/slow ADC bounds after selecting the requested clock.
Internal-source settling retains 50 microseconds, including inherited BGR use.
The factory-HSI envelope remains the qualified ±2% over −40..+85 °C under each
family's supply conditions. No wider temperature/trim/accuracy claim is added.
High-impedance inputs may need longer acquisition or buffering; consult the
existing [clock bounds](adc-hsi-clock-bounds.md) and
[scan electrical contract](adc-scan-sequences.md).

ADC_CR.BGREN and the ADC clock also serve comparator internal-reference users.
Critical-section CR updates preserve live BGREN. The driver never resets the ADC
block or clears inherited/enabled BGR, preserves an inherited clock gate, and
retains the gate once BGR has been used. Cancellation can shut down owned EN/TS
while preserving this shared resource. Retained BGR/gate power is intentional.

## Primary-source basis

Both families were checked against their own selected manual, datasheet and SDK;
matching register semantics are not inferred from a neighbor.

The checked source URLs, artifact/member hashes, source revisions and pinned
upstream references are retained in the
[audited evidence packet](adc-low-async-evidence.json).

Original sources:

- [L010 RM Rev1.2](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf),
  SHA-256 `b66ae2b2837cf22aede7f19312b82659ea10f96960bfe7965de8733bb72513fa`.
- [L011 RM Rev1.1, June 2026 artifact](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf),
  SHA-256 `b245887e9037caf267f739e25bd3c354c2579d23315fc06389e055f371159f4f`.
  The older September 2025 artifact has the same printed revision but different
  bytes; it is not the selected input.
- [L010 DS Rev1.3](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf),
  SHA-256 `6fbefd334a86a1fafbec8ead5b6bd44dc99dfe564b604f69ec1edaa4ec789b86`.
- [L011 DS Rev1.1](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf),
  SHA-256 `0b7414049824881920fc38f829029e3ba0af88feb4536fb27351df0d60f688a5`.
- [L010 SDK V1.0.9](https://www.whxy.com/uploads/files/20260806/CW32L010_StandardPeripheralLib_V1.0.9.zip)
  and [L011 SDK V1.0.3](https://www.whxy.com/uploads/files/20251016/CW32L011_StandardPeripheralLib_V1.0.3.zip),
  own `Libraries/inc/cw32l010.h` / `cw32l011.h`: dedicated `ADC_IRQn=12`.
  Both contain `Examples/ADC/adc_sqr_irq_sw`. Their continuous-mode examples
  corroborate EOS/vector use only, not lossless collection or cancellation safety.

Page numbers below are printed / 1-based PDF pages:

| Contract | L010 RM Rev1.2 | L011 RM Rev1.1 |
| --- | --- | --- |
| Stop/reset cursor; continuous overwrite warning | §20.5.1, 507 / 508 | §20.5.1, 509 / 510 |
| Single sequence, retained slot order, EOC/EOS, automatic START zero | §20.5.2, 508–509 / 509–510 | §20.5.2, 510–511 / 511–512 |
| Interrupt sources | §20.10 Table20-6, 514 / 515 | §20.9 Table20-6, 515 / 516 |
| Software START zero stops/resets | §20.12.2, 517 / 518 | §20.11.2, 518 / 519 |
| IER enables and read-only ISR | §20.12.8–9, 522 / 523 | §20.11.8–9, 524 / 525 |
| ICR zero-clears / one-preserves | §20.12.10, 523 / 524 | §20.11.10, 525 / 526 |
| Read-only RESULT0–7 | §20.12.11–18, 523–525 / 524–526 | §20.11.11–18, 525–527 / 526–528 |

The Embassy pattern is pinned at
[`f16efeffe37581092ec184718e6fdb1620393214`](https://github.com/embassy-rs/embassy/tree/f16efeffe37581092ec184718e6fdb1620393214):
`embassy-stm32/src/adc/mod.rs` for per-instance waker, typed binding and scoped
cleanup; `embassy-stm32/src/adc/v1.rs` for enabled-source filtering and masking
before wake; `embassy-futures/src/select.rs` and `embassy-time/src/timer.rs` for
caller composition. Those references provide Rust/Embassy structure, not CW32
register or electrical evidence.

## Examples and qualification boundary

The ordinary [ADC examples](../examples/shared/adc-scan/README.md) include a single async
read and an eight-slot borrowed mixed-source scan, with timeout/select composition
and reuse. Their optional `async` feature selects the same pinned executor/time/
futures revision and the existing GTIM1 time driver. It reserves GTIM1 and its IRQ;
the driver's [run-mode and blackout contract](time-driver.md) applies separately.
The existing blocking examples do not require this feature.

These are source contracts and firmware examples, not HAL mocks, protocol models,
test harnesses or evidence of runtime execution. No silicon was exercised here.
Target compilation and final source review are separate verification steps, and
their outcomes must be reported independently. Continuous/periodic acquisition,
hardware-triggered scan reuse, DMA, ring buffers, calibrated units, RF, deep-sleep
wake and lossless throughput remain outside this implementation. Classic ADC and L012
ADC1 IRQ support are covered by [their own-source extension](adc-remaining-async.md);
L012 ADC2 remains blocking pending shared ADC2_DAC ownership.
