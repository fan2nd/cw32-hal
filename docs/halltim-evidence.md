# CW32L012 Hall capture

This implementation is qualified only for CW32L012, CW32L012C8T6 and
CW32L012C8U6. The current selected Chinese V1.4 manual (June 2026), own V1.0
datasheet and SDK V1.0.5 are the authorities. Exact hashes, archive member chains,
revisions, printed/PDF pages and pinned Embassy files are in
[halltim-evidence.json](halltim-evidence.json). The capture timing figure on
printed page 416/PDF 442 and datasheet AF tables on printed page 36/PDF 39 were
rendered and inspected. No hardware was run.

The driver follows Embassy ownership conventions: `Peri`, a sealed instance,
generated legal pin traits and central `RCC_INFO`. It does not import STM32
trigger identities or register semantics. It owns one pin for each channel;
CW32L012 does not expose individual channel capture enable bits. The twelve
input routes are AF 9: CH1 PA3/PB2/PB5/PB13; CH2 PA4/PB6/PB10/PB14; CH3
PA5/PB7/PB11/PB15. The reviewed digital route projector filters by selected
physical package and retains the existing common-package intersection policy.
All twelve are available on each current exact L012 package.

Only PCLK and its documented divisors 1, 2, 4, 8 are used. `tick_clock()` propagates
the existing exact RCC factory-HSI envelope, including its supply and ambient
temperature limitations. It is not a measurement. ARR must be 1..0xFFFFFF and
the digital filter length 0..32767. The first filter is the documented 5/7 filter.
The second rejects pulses shorter than FLT2LEN ticks and guarantees passage only
at FLT2LEN+2 or longer; the two-tick boundary gap remains unspecified. The driver
does not promise a calibrated pulse width, filter latency or peripheral timing
outside the existing qualified operating envelope.

Any change in any of the three filtered inputs copies the counter to WIDTH and
clears CNT. CAPF is a coalescing flag. There is one WIDTH register and no capture
FIFO, per-input timestamp, overcapture flag or edge count. `try_capture()` clears
CAPF before reading WIDTH so a new edge can leave another event pending. That
does not make the read atomic or prove no lost edges: an edge before clearing
can overwrite the old sample, WIDTH can change before its read and STATE is
sampled later. `additional_capture_pending=false` is not a losslessness claim.
The raw/filtered states are live. `ObservedTransition` compares successive
software observations, including unchanged or multi-bit cases. It must not be
used as proof of motor direction or an uninterrupted six-step sequence.

Overflow is sticky in the driver from first observation until `restart`.
Since CAPF/OVF may coalesce and cannot establish temporal order, it invalidates
full elapsed-time interpretation conservatively without claiming which capture
interval overflowed. WIDTH remains a raw counter fragment. No fictitious
extended counter or inferred multiple-wrap count is supplied. `wait_capture`
is bounded by attempts, with no clock-duration meaning. Timeout leaves the
hardware running and preserves later observations. `stop` retains counter and
pending state; `start` resumes; `restart` explicitly discards the old run and
zeros CNT, which has write-zero-only semantics.

ICR uses source-backed `write_noop()` 0x0000FFFF and typed setters, preserving
unselected W0C flags and documented reserved reset bits. ISR/WIDTH/STATE are
read-only. Initialization explicitly stops EN, disables all DMA and interrupt
sources, initializes ARR/CCR/CNT and clears flags; it does not depend on the
manual's unusual DIER/CNT/ISR reset values. CR.SOFTCAP is not issued: the manual
calls it a software trigger, and this API does not infer WIDTH behavior.
The eight local MMS encodings are represented in the PAC from the own manual;
the capture driver holds the reset overflow selection and never configures a
consumer, TRGO pin or routing graph.

HALLTIM has independent APBEN2/APBRST2 bit 12 controls with a 0x5A5A clock key and
active-low reset. The driver refuses a held reset and never pulses or releases
one. Central RCC modification preserves all neighbors. Drop stops HALLTIM,
disables its requests, disconnects the three pads and releases only its own
unshared gate. BTIM3 shares interrupt 22, not the peripheral clock/reset;
no NVIC enable/disable/unpend action is performed. DMA and async/interrupt APIs,
PWM outputs, downstream triggers and software capture are unsupported.

The SDK convenience `HALLTIM_CR_BitField` masks conflict with both its own
CMSIS header and current manual. Those masks are not imported. The SDK init
struct's 8-bit filter length also truncates a documented 15-bit field; the HAL
uses checked `u16`. These differences and the current capture timing revision
are retained as evidence rather than silently copying SDK behavior.

No Chip/Peripheral/PAC metadata schema was changed. The independently rewritten
schema and its review ancestry are byte-for-byte preserved. The optional local
generator input `halltim_metadata` selects this source-qualified sidecar;
legacy omission and foreign-family/route rejection are covered by data-generator
checks. The existing `Pin` structure projects the 12 routes, and register-write
metadata projects the ICR seed. Normal ARM HAL builds and two exact-package
firmware links provide compilation evidence, not silicon validation.

Replay source/data qualification with:

    python3 tests/verify_halltim_evidence.py --sources /path/to/pinned/sources

The normal firmware is [examples/l012/halltim](../examples/l012/halltim/README.md).
