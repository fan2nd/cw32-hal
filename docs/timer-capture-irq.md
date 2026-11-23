# Coalesced capture IRQ notification

## Current API and eligibility

`InputCapture::wait_for_capture(channel, irq_binding).await` waits for an
already-owned four-channel input's pending capture flag. The binding must map
that timer's exact `OverflowInstance::Interrupt` to
`timer::input_capture::CaptureInterruptHandler<T>`. The shared trait supplies the
existing qualified GLOBAL vector, not an overflow-only interrupt line. The
method does not change the existing constructor or require a new mode parameter.

Eligibility is the intersection of existing `InputInstance` and
`OverflowInstance`: qualified classic GTIM instances on F002, F003, F020, F030,
A030, L031, L052, L083, R031 and W031; buffered GTIM and buffered ATIM instances
on L010, L011 and L012. Package-qualified input ownership remains mandatory.
A timer reserved by the time driver cannot be acquired. Native six-half classic
ATIM, BTIM, capture DMA and non-qualified pins are not added.

The existing generation checks require precisely one GLOBAL association and
verify its NVIC number and bank ownership. ATIM uses ATIM IRQ13; GTIM/GTIM1 uses
IRQ16, GTIM2 IRQ17, GTIM3 IRQ18 and GTIM4 IRQ19, except L012 GTIM3/4 share
GTIM34 IRQ18. Every independently enabled owner on a shared vector requires its
own handler in the dispatch. The capture handler never disables, unpends or
reprioritizes the whole vector and does not access the other bank.

## Register/source basis

This delta reuses the own-family input/IRQ qualification, not STM32 register
layouts or new assumptions based on register names:

- [Classic GTIM source record](classic-input-research/classic-gtim-input-evidence.json)
  indexes each own manual's IER, ISR, CCR and ICR sections. Classic IER CC1–CC4
  individually control capture requests; ISR CC1–CC4 are pending flags. CCR
  reads do not acknowledge them. There is no qualified overcapture detector.
- [Buffered input source record](timer-input-source-evidence.json) indexes IER (L012 ATIM: DIER)
  (`CC1IE`–`CC4IE`), ISR and CCR for L010/L011/L012 GTIM and ATIM. CCR read-clear
  semantics and independently sticky overcapture flags remain unchanged.
- [Counter/IRQ source record](timer-event-input-source-evidence.json) records
  each family's GLOBAL vector, NVIC table and shared-vector ownership. The
  runtime still uses the selected family's typed PAC, without layout casts.

Only the selected request bit is changed with an IER/DIER read/modify/write. Neither
the handler nor the wait reads CCR, writes ICR or modifies overflow requests.
Existing historical source records are unchanged. This delta does not claim a
new replay of the original PDFs or any silicon validation.

## Lifetime, cancellation and event semantics

The owner retains the timer and pins. A wait borrows it mutably, permits one
outstanding channel wait, and checks ownership before touching hardware. Its
waker and one-channel armed mask live in the existing per-bank static state;
there are no caller pointers or ISR accesses to the future. Before a new wait,
only a previously armed request is masked, including one left by a forgotten
wait. The waker is registered before checking status and enabling the request
within a critical section. An edge after the check leaves pending status for
the interrupt once the critical section exits.

The handler returns without MMIO if no wait is armed. For an armed channel with
pending capture status, it masks that channel's request and wakes outside the
critical section. It leaves capture acceptance and status intact. A subsequent
poll returns immediately while the pending flag is set. Read the timestamp
with `get_capture_value`; classic GTIM also needs `clear_input_interrupt`.
Calling the wait again without acknowledging the flag returns immediately.

Dropping/cancelling a polled future masks its channel request without clearing
capture status, stopping the counter or disabling capture acceptance. Forgetting
it can leave one request armed until an event, another wait, owner Drop or
successful reconstruction. A registered static waker can remain retained after completion, cancellation
or wake until a later wait replaces it. Owner Drop retires the mask and disables capture requests
before stopping or gating hardware. Common timer reconstruction retires this
state before RCC mutation, including reconstruction as a non-capture owner.

Every indication is coalesced. A new edge can overwrite CCR before application
code reads it; the handler does not snapshot timestamps or count edges. Disable
the channel before reading/acknowledging if a stable latest-value snapshot is
needed, accepting the deliberate gap. No bounded latency, lossless history,
wall-clock timeout or stopped-clock wake guarantee is made.

## Example and verification status

`examples/shared/timer-overflow/src/bin/capture_event.rs` uses an exact generated
CAP1/CH1 route and GLOBAL IRQ binding, waits for an external edge, gates capture,
reads the latest timestamp, acknowledges classic status and resumes capture.
It produces no stimulus and does not assume loopback. The generated
`capture-route.txt` identifies the selected package's input. On L012 this owner
uses GTIM3 and leaves the separately reserved GTIM1 time driver intact.

Compilation/linking and focused state checks for this delta are recorded by the
current batch validation receipt. Earlier counter-overflow and polling-input
receipts do not validate this new path. Hardware tests have not been performed.

## Deferred I2C candidates

The current classic source-qualified state table describes seven-bit SLA+R/W
phases; it does not qualify the ten-bit high-prefix/low-address/read-restart
sequence or its address-NACK classification. L012 also needs its own command
sequence and FIFO/error analysis before a ten-bit interface can be claimed.
Do not infer support solely from slave address field width. Slave operation
requires a separate ownership and bounded buffering contract.

GPIO bus unwedge is not equivalent to existing bounded local I2C retirement.
It needs a deliberate single-master/target side-effect policy, released-line
observation, stretching deadlines and pin/peripheral transition evidence. No
automatic retry is added: repeating a partial transaction can duplicate target
side effects. These remain evidence/design work, not implemented features.
