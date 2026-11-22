# Bounded L010/L011 ADC scans

The L010/L011 ADC API supports one software-triggered scan containing 1–8 ordered
slots. Each slot independently selects a package-qualified channel and sample
time. The same channel handle may appear repeatedly. Results are returned in
slot order, corresponding to the hardware RESULT array, not sorted by input mux.
Samples are sequential, not simultaneous.

`Adc::blocking_read_sequence(&sequence, &mut results)` accepts a slice of
`(&BorrowedAdcChannel, SampleTime)` pairs and exactly one `u16` destination per
slot. `degrade_adc()` consumes an owned pin/internal-source handle;
`reborrow_adc()` retains an exclusive borrow of the original handle. The scan
borrows these handles until the blocking call finishes, including when a handle
is referenced by more than one slot. No raw-channel-number constructor exists.
The same method supports a one-slot sequence. Existing `blocking_read` uses that
path and restores a single-slot configuration after a longer scan.

An example with owned channels is in
[`scan_owned.rs`](../examples/adc-scan/src/bin/scan_owned.rs). An eight-slot
example uses borrowed external pins, both internal sources, repeated channels,
and a following single conversion in
[`scan_borrowed.rs`](../examples/adc-scan/src/bin/scan_borrowed.rs). These are real
standalone Cortex-M firmware images. They were linked, never flashed or run.

## Timing and ownership

The entire sequence uses one actual ADC clock divider. Every slot is validated
through the existing `Config::timing` checks first. Each check chooses the fastest
allowed divider while enforcing maximum actual clock, minimum actual clock where
specified, voltage-dependent conversion rate and acquisition minimum. The scan
then chooses the largest accepted divider. Slowing an already-accepted slot
preserves its upper-frequency/rate limits and acquisition minimum; the selected
slowest divider itself passed the lower-frequency check. No nominal-only
approximation or averaging across slots relaxes these bounds.

`SequenceTiming` returns the common prescaler, full `ClockBounds`, slot count and
sum of acquisition-plus-comparison cycles. Its worst-case duration uses the
slowest qualified actual clock. `Adc::timing()` describes the last slot at that
common clock. Both durations exclude startup, source settling and software
execution. `Config::timeout` is one finite poll budget for the whole sequence,
not a wall-clock timeout or a fresh budget for each slot.

Existing electrical constraints continue to apply: VDD on L010, tied VDDA=VDD on
L011; declared supply voltage and actual input/source-impedance limits; the
source-qualified HSI operating envelope; and L011's 4 MHz minimum ADC frequency.
TS and BGR each require at least 40 microseconds acquisition. Their startup delay
is separate: newly enabled TS and every scan using BGR wait 50 qualified
microseconds. Shared BGR is never cleared and a pre-existing/shared ADC gate is
never disabled. No ADC reset is asserted. Reconfiguration, timeout and drop
retain this policy. Source startup's documented approximate BGR value still
requires board/silicon qualification for analog accuracy.

Native ADC timing requires cycle-qualified PCLK. `Error::UnqualifiedCycleTiming`
rejects rate-only sources, including exact-L010 live LSI, after existing limits
and uninitialized-clock validation but before acquisition/rate selection or the
constructor's first gate, ADC/BGR, trigger/watchdog or IRQ change. L010's minimum
ADC clock is zero; that numeric minimum cannot replace this semantic check.
Blocking/async fallible constructors, scan timing and the existing BTIM1-triggered
constructor share this boundary and return errors to the caller.
Caller-created timer owners or analog channel pins may already have changed
hardware, and consumed tokens or earlier preparation are not restored. A rejected
`Btim1Adc::new` also drops the supplied `Btim1Update`, running its existing
BTIM1 stop, request-disable and gate-release cleanup; the admission guard precedes
ADC setup. Manually constructed public timing values retain their strict-helper
preconditions.

The constructor validates its initial 390-cycle acquisition before acquiring the
clock. The default nominal 4 MHz L011 clock does not satisfy its actual 4 MHz
lower ADC bound after HSI tolerance; an appropriate clock configuration is still
required. The new examples declare a 3.0–3.6 V board and HSI /8, with an ADC maximum
of 24 MHz, so both selected families have a valid divider. Those declarations
must be changed to match the actual board before running the images.

## Completion, rejection and retry

Empty/oversized sequences, mismatched result lengths and any invalid slot timing
return an error before ADC register changes. A caller may already have put pins
in analog mode while constructing channel handles. These errors leave the
existing ADC configuration and result buffer unchanged.

Before a conversion, START=0 stops the peripheral and explicitly resets its
sequence cursor. SQRCFR and SAMPLE are written through generated eight-element
field arrays; CR.ENS contains length minus one and CONT stays zero. Only typed
PAC fields are used, with no HAL register-pointer arithmetic, raw word masks,
shadow control register or generic I/O adapter. One software START begins the
entire sequence.

Success requires EOC, EOS and hardware-cleared START. Only then are all selected
RESULT entries copied to the caller's buffer and EOC/EOS acknowledged through
ICR's W0C fields. Reading ISR or RESULT has no documented flag-clear effect.
There is no overrun flag in this IP. A timeout requests stop/cursor reset,
disables ADC/TS and clears flags while preserving shared BGR/gate ownership;
it never returns a partially filled buffer. A later read reinitializes the ADC.
There is no DMA or interrupt writer accessing the caller's buffer.

The separately owned L010 BTIM1-triggered ADC route still programs one slot. Its
public lifetime, static resource ownership, and terminal cancellation rules are
unchanged; it uses the generated slot/result array index zero.

## Source and architecture boundary

Own-manual register and behavioral evidence, exact official URLs, printed/PDF
page numbers, SDK revisions and verified SHA-256 values are in
[`adc-scan-source-evidence.json`](adc-scan-source-evidence.json). L010 CN1.2
pp506–509/516–525 and L011 CN1.1 pp508–511/517–527 independently establish
CR.ENS, per-slot sampling/mux selection, RESULT order, EOS and explicit cursor
reset. The L011 SDK1.0.3 example additionally demonstrates repeated BGR slots;
L010 repetition follows its own independently programmable-slot contract.

The authored `adc-sequences.json` policy is selected explicitly by the two data
input manifests. Data generation validates its source/version/array/flag shape,
projects `AdcLimits.sequence`, and metapac transports it to the existing HAL
constant generation. The runtime gets `MAX_SEQUENCE_LEN` from that generated
metadata. Other ADC metadata does not gain scan capabilities. Curated RESULT
register arrays and SAMPLE/SQRCFR field arrays preserve every scalar vendor
address, width and access permission; independent SVD parity expands every array
element for verification. Register-array overlap validation now checks individual
spans, including overlap between elements, with checked offset arithmetic.

The organization follows the actual pinned Embassy STM32 source at
`f16efeffe37581092ec184718e6fdb1620393214`: version-specific register behavior,
sealed channel ownership, channel/sample pairs and typed PAC sequence fields.
Its common `adc/mod.rs`, `adc/v2.rs`, `adc/v3.rs` and
`adc/configured_sequence.rs` were inspected. CW32 retains its already separate
ADC architectures and introduces no new generic engine. STM32's configured DMA
sequence API is not imported: it relies on different peripheral/DMA terminal and
cancellation behavior.

Classic F002/F003/F020/F030/A030 hardware has 1–4 ordered scan slots; classic
L031/R031/W031/L052/L083 has 1–8. Those use MODE=4 and SQR/SQR0/SQR1, and the
reviewed classic sections do not establish the same explicit abort cursor reset.
L012 has eight-slot ADC1/ADC2 hardware but different register offsets, ENS position,
longest sample duration, SLAVE synchronization and common BGR/TS ownership.
Their scan APIs remain unsupported. This patch does not add continuous scans,
external-triggered scans, async scans, IRQ sequencing, DMA, calibrated units or
RF support. Existing single conversions on those architectures are preserved.

The earlier `adc-low-sequence-audit.md` is historical; its one-slot-only and
nominal-clock statements describe that older snapshot. Current clock requirements
are documented in `adc-hsi-clock-bounds.md` and this scan contract.
