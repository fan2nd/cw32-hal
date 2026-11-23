# CW32L012 independent ordered ADC scans

Source implementation, 2026-10-08. Both exact parts (CW32L012C8T6 and
CW32L012C8U6) and their package-intersection alias use the same independently
qualified ADC1/ADC2 layout. This extends the historical single-read backend.
The later [finite software async API](adc-remaining-async.md) also covers both
converters. ADC2 adds [checked DAC-peer shared-vector admission](adc-l012-adc2-async.md),
with Stage91 main 5-library/10-ELF verification. At main acceptance clean and
final54 report binding had not run; final completion requires their separate
receipts. Both converters retain this blocking API.
No firmware execution, electrical qualification, average, calibration accuracy
or engineering-unit result is claimed.

## Public behavior and ownership

`blocking_read_sequence(&[(&BorrowedAdcChannel<'_, T>, SampleTime)], &mut [u16])`
returns `SequenceTiming`. One to eight slots execute once in caller order.
Every slot has its own mux selection, acquisition length and result register.
A channel can appear several times by referencing its existing owned or
borrowed handle. The result slice must have the same length as the sequence.
Single reads use exactly one slot through this same path. Existing internal
TS and BGR handles still return raw counts. No arbitrary numerical channel
constructor, DAC-output source, dual coupling, continuous conversion, external
trigger or DMA scan API is added by this blocking scope. ADC1/ADC2 interrupt
reads and scans are documented separately.

Both converters acquire the shared clock internally and preserve shared BGR state.
Constructors, reconfiguration and reads reject a sibling with SLAVE=1 before
ADC/BGR writes, even if that sibling is disabled. SLAVE=1 is a request to follow
the other converter's START; allowing it would trigger a resource the caller
does not own. The local owned converter is configured with SLAVE=0 and CONT=0.
An independently running sibling with SLAVE=0 remains permitted and untouched.
The preflight is a snapshot; raw users may not race either owned converter or
the sibling SLAVE field. Normal safe owners cannot enable slave operation.

## Validation, completion and cleanup

Sequence length, result length, every slot's timing, and the sibling preflight
are checked before local ADC changes. Channel preparation through `degrade_adc`
or `reborrow_adc` may already have configured its own GPIO as analog. This is
the existing channel contract, not an ADC side effect on timing rejection.
Initial configuration and sibling checks precede construction of the RAII
owner, so rejection by those checks does not run an ADC-writing destructor.
An ADC2 blocking replacement additionally retires a forgotten async owner's
static dispatch and bounded local conversion state before ordinary blocking
initialization. That retirement uses an async-lifecycle temporary: cleanup can
fail after local ADC writes and run its bounded destructor cleanup before
returning a terminal error. A terminal async fault remains latched across replacement
until chip reset; ordinary blocking-only acquisition retains its existing path.

Before START the driver stops/resets the local cursor, clears local flags,
programs the slot arrays and ENS=length-1, and selects one shared divider.
It waits for EOC, EOS and hardware-cleared START before copying any result.
ISR and RESULT are read-only; reading RESULT does not acknowledge completion.
The polling budget applies to the whole scan. Caller results remain unchanged
on validation failure or timeout.

Timeout cleanup uses a fixed number of local writes: START=0 stops conversion
and resets its slot cursor; interrupts, DMA requests, external triggers and
watchdog are disabled; all local CR fields are disabled and local flags clear.
A later scan reinitializes the local converter. CR is modified through typed
fields, preserving all live reserved bits, including documented reset bit 8=1.
ICR commands start from the generated no-op value 0x1F, clear only intended
fields, and preserve reserved bit 2=1. The documented ICR reset is 0x0F; AWDH
must additionally be 1 in a no-op command. No register-bit shadow is maintained.

No cleanup, drop or constructor resets the ADC group, disables the common gate,
or clears BGR/TS. Internal source enables only set their typed fields within a
critical section, preserving other analog owners. These sticky shared resources
intentionally retain power after owners drop. ADC EN itself also starts BGR;
this hardware side effect is documented in section 25.12.19.

## Timing proof

The existing own-family electrical facts remain unchanged. VDDA=VDD is the
sole reference; guaranteed minimum supply chooses the voltage band. At the
maximum possible ADCCLK, every slot must satisfy its clock limit, sample-rate
limit and minimum acquisition. At the minimum possible ADCCLK, every slot's
selected divider must remain at least 4 MHz. Exact rational ClockBounds arithmetic
is used before display rounding.

For each slot, find its fastest accepted divider among 1, 2, 4, 8. Select the largest
of those dividers for the scan. Increasing division only reduces frequency/rate
and increases acquisition. The slot that selected the largest divider also
proved its minimum frequency is at least 4 MHz; all slots share that same clock.
Thus the selected divider satisfies every slot, including a mixture of external,
TS and BGR sources. If any slot cannot meet both endpoints, reject the whole
scan before changes. No per-family facts are encoded in HAL cfg tables.

The ADC limits by minimum supply are 6/12/24/48 MHz and 200 k/500 k/1 M/1 M conversions
per second. External minimum acquisition is 1/0.5/0.25/0.125 us, respectively.
Each sample requires its selected 6..518 acquisition cycles plus 15 comparison
cycles. TS/BGR require at least 40 us acquisition at the fastest clock and a 50 us
settling delay before any scan containing them, even when their enable was
inherited. HCLK's maximum bound sizes this delay; converter startup additionally
covers 15 cycles of the slowest ADC clock. The declared HSI qualification interval
is -40..85 C and the retained L012 source envelope is ±2%.

`SequenceTiming.conversion_cycles` sums all acquisition/comparison cycles;
`maximum_conversion_time_ns()` rounds upward at the slowest qualified clock.
It excludes startup, source settling, trigger and software latency. The timeout
is an iteration budget, not that duration. High source impedance may require
more acquisition than the documented floor.

The datasheet's 96 MHz ADC maximum and 405-cycle total conflict with the current
manual's 48 MHz maximum and 518+15=533-cycle longest conversion. The driver retains
the stricter 48 MHz maximum and the manual's explicit sample encoding. Datasheet
TS startup maximum 40 us is covered by 50 us, despite the manual's approximate 30 us.

## Own-source provenance and representation

The authoritative hashes, original URLs, revisions, pages and SDK members are
recorded under `families.CW32L012` in `adc-scan-source-evidence.json` and checked
against `sources/evidence-sources.json`:

- CN user manual V1.4, sections 25.4–25.5 and 25.12, printed 575–582 and 589–599
  (PDF 601–608 and 615–625): ordering, slot count, per-slot samples/mux/results,
  normal/slave behavior, START/EOS/ICR and shared BGR.
- CN datasheet V1.0, sections 7.3.13 and 7.3.15, printed 60–61 and 63
  (PDF 63–64 and 66): supply, minimum clock, acquisition, ADC and TS startup.
- SDK V1.0.5 own header, ADC header/source and ADC examples: layout and encoding
  cross-checks. The inspected eight-slot SDK example uses continuous mode;
  single-scan completion comes from the manual, not that example. Repeated mux
  support follows independent slot selection without a uniqueness restriction;
  the inspected L012 examples do not independently demonstrate repeated muxes.

Authored SAMPLE.SQRCH and SQRCFR.SQRCH arrays are eight 4-bit fields. RESULT is
an eight-register array at 0x30 with 4-byte stride and 12-bit read-only fields.
They expand to the exact SDK SVD footprint. The preexisting ISR Read correction
is retained (own manual printed 597/PDF 623). The independent data verifier checks
all 18 expanded registers and 94 fields, ADC1/ADC2 bases and facts for all three
selections, source hashes, the ICR seed, and the canonical register-reuse digest.
No vendor file is included in the patch packet.

The shared per-slot sequence qualifier now reads offsets and widths from each
family's authored source record and projects facts to every ADC instance. The
accepted L010/L011 shapes are unchanged; their already documented START offset
is now explicit in the source record. Shared schema ancestry is unchanged.

## Verification

`examples/l012/adc-scan` provides two genuine exact-part firmware examples.
Both link for both exact packages, producing four ELF32 ARM executables.
The alias and exact parts compile the production HAL in bare, rt+defmt and
rt+defmt+time-driver-gtim1 combinations. Builds use the official installed Rust
toolchain offline, thumbv6m-none-eabi, no incremental cache and debug 0.
No HAL tests, harnesses, mocks or host execution of drivers are used.
