# Ordered blocking scans on classic ADCs

The later [finite software async API](adc-remaining-async.md) preserves these
channel, slot and electrical limits on all ten classic lines.

The classic ADC HAL supports one software-triggered scan, returning results in
caller-specified slot order. Repeating a sealed channel handle in several slots
repeats that mux without duplicating the owned pin token. Existing
`blocking_read`, constructors, reference selection and internal-source APIs
remain available. No firmware was run as part of qualification.

| Families | Result slots | Sequence registers |
| --- | ---: | --- |
| F030/A030, F020, F002, F003 | 4 | SQR with four mux fields and ENS |
| L031/R031/W031, L052, L083 | 8 | SQR0 for slots0–3 and ENS; SQR1 for slots4–7 |

Each family's own manual independently establishes MODE=4 as a single sequence
scan, slot-indexed RESULT registers, EOS completion and automatic START clear.
MODE=2 is single-channel continuous operation and is not used. The shared
CW32x030 manual explicitly covers F030 and A030. Sources, original PDF hashes,
printed/PDF pages and exceptions are in
[classic-adc-scan-source-evidence.json](classic-adc-scan-source-evidence.json).
The L010/L011/L012 ADC architectures are outside this implementation.

## API and analog limits

`blocking_read_sequence(&[(&channel, SampleTime)], &mut [u16])` borrows sealed
owned or borrowed channel handles. The result length must equal the slot count,
which must be 1 through `MAX_SEQUENCE_LEN`. All slots must use the same sample
time because classic SAM, CLK, BUF and REF are common controls. The same shape
as the low-power API makes hardware differences explicit through validation.

Multi-slot scans require external channels with `Config::input_buffer == false`.
Own accuracy sections require single-channel single-shot operation for internal
and weak sources, while another paragraph discusses slower buffered multichannel
weak-input use. This bounded API conservatively excludes that ambiguous mode.
One-slot sequences use MODE=0 and retain the existing internal/weak-source
follower, startup and acquisition protections. F002 supports supply/3 only;
TS/BGR measurement and internal references are not invented from copied SDK
macros. R031 uses its own reviewed external mux4–12 mapping.

The existing selected-family supply/reference clock bands and HSI `ClockBounds`
remain authoritative. Acquisition and follower conversion-rate limits continue
to apply to single reads; temperature acquisition uses the existing >=5 us
requirement. The 50 us temperature and 25 us BGR waits are explicit software
guards; the BGR manual statement is approximate, not a characterized maximum.
They now come from reviewed metadata, alongside slot counts and channel IDs.
Board source drive, settling, VDDA/VDD, temperature and pin/reference voltage
limits remain physical requirements. Raw counts are not calibrated temperatures
or voltages. SequenceTiming sums acquisition/comparison cycles; its duration
bound excludes startup, settling, trigger/software and between-slot overhead.

## Completion, failure and ownership

Length, result-length, sample-time, channel-mode and timing validation precedes
ADC register writes. GPIO preparation when creating/reborrowing a channel can
occur before that validation. A rejected call leaves ADC state and the entire
caller result buffer unchanged.

Startup masks all trigger requests before stopping START and disabling EN,
while retaining a documented shared BGREN. It drains all result slots, clears
real ICR event fields and applies fresh configuration before waiting for READY.
Scans wait for EOS and hardware-stopped START; single reads use EOC and stopped
START. Overrun is checked before accepting completion. Only a completed sequence
copies slot results to the caller. Timeout or overrun stops triggering and
conversion, disables owned analog controls and preserves the entire output
buffer. A later operation retries initialization. Poll budgets are finite
iteration counts, not elapsed-time deadlines; the conversion budget covers the
whole scan. ICR is R1W0 and READY has no corresponding clear field. The driver
makes no claim that reading RESULT clears EOC/EOS or that DISCARD controls scan
overwrite; the manuals do not establish those behaviors.

Peripheral `Peri` ownership and generated package-qualified pin traits remain
intact. The driver never resets ADC. Clock acquisition and release use central
RCC with bounded readback. An inherited clock gate remains enabled. On
x030/F020/F003, shared BGREN is preserved after reference/channel use, errors,
configuration changes and drop; its gate remains enabled. Other ADC reference
and temperature controls remain exclusively owned by this ADC driver. There is
no permission for concurrent raw ADC configuration or internal-source control.

## Typed PAC and compatibility

The old classic Io/Engine and raw register mirrors are removed. Typed PAC fields
are generated from authored YAML. MODE exposes source-qualified SINGLE and SCAN
variants. RESULT0…RESULT3/7 become the bounds-checked `result(index)` array;
sequence nibbles become `set_sqr(index, mux)` within their actual register. The
eight-result SQR registers remain distinct so ENS is never exposed on SQR1.
L052's separate common RESULT@0x5c is named `commonresult()` to distinguish it
from the ordered result array. Direct PAC users of those old scalar accessors
must use the new names/indexes. No HAL single-read signature changes.

F002's reserved CR0 BGREN/TSEN fields are removed through a separately reviewed
minimal correction, with an explicitly ADC-scoped source-parity allowance. The
eight-result variants already omitted reserved BGREN correctly. No undocumented
F020 14-bit extension is accessed.

Unsupported: continuous, externally triggered and DMA scans,
per-slot reference/buffer/acquisition settings, external-reference pin ownership,
calibrated physical units, simultaneous sampling and RF changes.

## Firmware verification

`examples/shared/classic-adc-scan` contains ordinary exact-package firmware for both
borrowed and owned channels. Each program selects two reviewed bonded inputs,
alternates them through the full hardware sequence length and performs a normal
supply/3 single read between scans. The build emits the actual input routes and
links a device-sized memory map. Retained ARM ELFs and build receipts establish
normal compile/link compatibility, not physical execution or analog accuracy.
No HAL test, mock, fixture, synthetic register image or cfg(test) was added.
