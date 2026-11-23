# CW32L010 typed timer cascade and ADC trigger

This source-backed implementation qualifies exactly two internal routes on
CW32L010 and its three exact package selections. It has not run on silicon.
L011 is separately qualified in `trigger-routing-l011.md`; other families receive
no route merely because a PAC layout is shared.

## Source and destination encodings

- Source: BTIM1 UPDATE through CR2.MMS=2, bits 6:4.
- BTIM2 TRGI: SMCR.TRGISRC=8, bits 10:7, SMS=3 external rising-edge counting.
  No inversion, input filter, reset source or consumer prescaling is selected.
- ADC conversion start: TRIGGER.BTIM1TRGO=1, bit 13. This is an independent
  enable, not an EXTSEL value or a cross-timer ITR index. Other trigger enables
  remain clear; CONT=0 and ENS=0 select one conversion of slot zero.

Sources are the [official L010 manual Rev1.2](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf)
and [own-family SDK V1.0.9](https://www.whxy.com/uploads/files/20260806/CW32L010_StandardPeripheralLib_V1.0.9.zip).
The manual's one-based PDF pages are 163 (enable/one-shot), 165 (update), 166 (TRGO),
168 (external counting), 181 (§11.9.2, MMS), 182–183 (§11.9.3, selector/mode),
511 (hardware ADC start), 517 (§20.12.1, single sequence),
518 (§20.12.2, software stop/sequence reset), and 521 (§20.12.7, enable).
Exact PDF and SDK/CMSIS hashes are in `cw32-data/triggers/cw32l010.yaml` and
already occur in the existing acquisition lock. No vendor originals are bundled.
The data source verifier independently opens the original PDF and headers.

The optional `Trigger.registers` schema preserves the legacy JSON shape when
absent. `TriggerRegister` is defined once in the project-authored schema and reused
by the PAC generator. The source normalizer qualifies the entire sidecar hash,
exact source event, destination-local encoding, real field widths/access,
hardware version, complete route set, and conflict/reserved exclusions before
projecting either route. Static metadata and the selected chip's build script
then generate `Btim2TriggerInput` and ADC `TriggerInput` independently.

## Ownership and ordering

`Btim1Update` owns static BTIM1 and exposes only immutable timing information.
`Btim2Cascade` consumes it and static BTIM2. `Btim1Adc<P>` consumes it, static
ADC, and a package-qualified static external pin. Safe code cannot retrieve a
mutable timer, blocking ADC, pin or token while a route is live. Forgetting an
owner leaks its static resources instead of ending a borrowed lifetime.
Public Copy PAC values remain outside this HAL ownership boundary and must not
concurrently change any active timer, ADC, pin configuration or clock.

Cascade start/restart detaches BTIM2 before stopping or preparing the source and
latching either prescaler. It reconnects only after all software update commands,
then starts BTIM1. Stop/drop disables BTIM2 counting before SMS=0 can select internal PCLK, then
disconnects the destination before stopping the source. MMS=0 forwards
software UG as TRGO, so selecting MMS=0 alone is not sufficient isolation.
The shared BTIM gate remains enabled and its group reset is never asserted.
Overflow flags coalesce and do not form a lossless event log.

The ADC wrapper is a distinct public owner with no blocking-read, set-config,
mutable timer, internal-channel, DMA or async API. It privately reuses the
existing ADC clock/analog lifecycle. Current ClockBounds, supply/acquisition
limits, bandgap preservation and central RCC_INFO controls remain in force.
Arm detaches the ADC before preparing BTIM1, clears stale state, enables only
BTIM1TRGO, and starts BTIM1 in one-shot mode. It never software-starts conversion.
Poll returns a sample only after source EN is clear, EOC and EOS are set, and
ADC START is clear. A completed sample is consumed once; another explicit arm
can then acquire another sample. Busy arm leaves the acquisition untouched.

Disarm, drop and timeout detach ADC first, stop BTIM1, write ADC START=0 and
clear stale results/status. The manual describes START=0 as stopping conversion
and resetting sequence position, but provides no quantified pipeline-drain or
atomic cancellation guarantee. Therefore disarm/timeout is terminal for that
owner: `arm()` returns `Disarmed`, and all static resources stay owned. This
avoids basing an immediate rearm on an assumed hardware abort acknowledgement.

## Timing and verification limits

Timer intervals use nominal PCLK and expose the current qualified clock envelope.
The manual documents one PCLK of enable delay; synchronization adds unbounded-in-
this-API latency. ADC timing excludes trigger/startup latency. `wait(budget)` is
an iteration limit, not a wall-clock deadline. There is no lossless periodic
sampling, DMA cancellation or cycle-exact first-event claim.

Verification consists of data/schema projection fixtures, original-source replay,
normal ARM production builds for all four L010 feature selections, unaffected
family builds, and genuine `cascade`/`triggered_conversion` firmware links for
all three exact L010 packages. No HAL tests, models, harnesses or mocks are used.
Firmware was not flashed or executed. Run the standalone examples documented in
`examples/shared/trigger-routing/README.md`; the normal `ci/check-hal.sh` includes them.
