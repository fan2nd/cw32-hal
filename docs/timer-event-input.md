# Counter overflow events and native classic ATIM inputs

## Scope and sources

The existing `Timer<T, Async>` overflow API extends from BTIM to 34 own-family
GTIM instance associations and eleven ATIM associations. It owns a continuous,
edge-aligned, internal-PCLK up-counter and reports a coalesced indication that
at least one wrap was observed. It is not a wrap counter, timestamp stream,
global time driver, stopped-clock service or sleep/wake guarantee.

F030, A030, F003, L031, R031, W031, L052 and L083 additionally expose native
classic ATIM polling input owners. These cover six independent halves,
CH1A/CH1B/CH2A/CH2B/CH3A/CH3B, and a CH1A/CH1B quadrature pair. The three actual
classic register versions are `v1`, `cw32f003_v1` and `cw32l052_v1`. Buffered
GTIM/ATIM and classic GTIM owners retain their existing interfaces.

The [own-source record](timer-event-input-source-evidence.json) and
`cw32-data/classic-atim-input.yaml` distinguish register semantics, input-role
qualification, GPIO AF routes and exact package bonding. The direct optional
`ClassicPairedV1` metadata enum grants the native protocol, not all six physical
pads on every package. It is present on 37 of the 54 current selections. Eight
small/common-alias selections lack CH2B; absent optional capture halves do not
erase the others. Generic aliases retain their common-package intersection and
do not provide a unique linker memory map. Debug, reset, oscillator and radio
exclusions remain in force. No existing PWM trait grants input use, and no CH4
external capture, fake GTIM mux, index or break input is added.

## Counter event ownership

`Timer::new` remains Blocking. `Timer::<T, Async>::new_async`
requires a binding from the exact `T::Interrupt` to
`timer::OverflowInterruptHandler<T>`. Async construction returns a stopped
counter with only its own overflow request enabled. `wait_overflow().await`
consumes one event; `take_overflow() -> bool` consumes retained software or
positively observed live hardware indication in either Blocking or Async mode.
Repeated wraps and acknowledgment races may coalesce.

Each hardware bank has its own static active flag, event latch and waker.
Handlers return before bank MMIO when ownership is inactive. Active service
checks its own request and flag, acknowledges only a positively observed event
and wakes after leaving the critical section. No state points into an owner
or wait future. L012 GTIM3 and GTIM4 share GTIM34 but have distinct handlers and
state. Every independently enabled source requires dispatch; an individual
owner never disables, unpends or reprioritizes the whole vector.

Successful construction first completes pure clock/rate/period/route checks,
then retires old event state before RCC mutation in the same critical section.
That applies to Blocking/Async counters, PWM, buffered/classic input and the
direct classic complementary constructor. A failed pure check leaves the
previous timer/event state unchanged; by-value tokens are still consumed, so
use `reborrow()` to retain them. Pin wrappers can already have selected digital
input/disconnected GPIO state. RCC gate/reset readback retains its existing
behavior and is not a new checked reset-success guarantee.

Start/stop preserve CNT, pending indications and request enable. Counter writes
validate against ARR and preserve events. Valid timing changes mask only the
overflow request, retire prior/software-update indications, reset/reconfigure
and restore prior running/request state. Invalid changes mutate none of those
states. Explicit `clear_overflow` discards indications; it may selectively
acknowledge without a prior status observation. Classic ATIM preserves UIE in
command-safe CR writes; deliberate UG runs with the request masked and synthetic
UIF retired. Classic GTIM does not acquire an invented UG operation.

Cancelling or forgetting a wait preserves counting, requests and events. A later
wait replaces the static waker. Forgetting an owner can leave hardware and
wakes active until successful reconstruction. Drop retires active state before
masking requests/stopping/gating its bank. Independent GTIM/ATIM gates may close;
BTIM shared gate/reset policy stays intact. No event count, lossless history or
clock-loss recovery follows from this ownership discipline.

## Native six-half capture

`input_capture::ClassicAtimCaptureInput::from_pin(pin, pull)` owns a separately
qualified digital AF input. `ClassicAtimInputCapture::new` takes the
timer, six optional wrappers in CH1A/CH1B/CH2A/CH2B/CH3A/CH3B order and a maximum
counter tick frequency. It chooses among native divisors /1,/2,/4,/8,/16,/32,
/64,/256 and runs an internal up-counter with ARR=65535. Inputs start disabled.

Use `ClassicAtimChannel` to enable/disable a supplied half, select rising,
falling or both edges, set a `low_level::ClassicAtimFilter`, read its capture
value, inspect/clear its capture flag and independently inspect/clear its
sticky overwrite flag. The five filter variants are `NoFilter`, `PclkN3`,
`PclkDiv4N3`, `PclkDiv16N3`, `PclkDiv64N3`. Their sampling clocks derive from
PCLK, independently of counter prescaling. Omitted-half operations panic before
timer MMIO; `is_enabled` alone returns false for an omitted half.

A CCR read returns the latest latch and acknowledges neither flag. Capture F
and overwrite E each have a selective acknowledgment. A live edge can replace
the latch between status/data reads or arrive before a flag clear; a false E
is not evidence of losslessness. To stabilize a half's latch/status, disable
that half before reading and acknowledging it, then re-enable. Edges during the
disabled interval are deliberately discarded. A critical section alone cannot
prevent hardware captures. Timestamps are modulo 65536; wrapping subtraction
represents elapsed ticks only when fewer than one full period elapsed.

Filter/edge reconfiguration temporarily disables only that half and preserves
its old flags and the other halves. Native EN=0 does not stop capture
acceptance. Drop therefore zeros all six BKS fields while the gate is live,
then stops, disconnects owned pins and drops the inner timer. No async capture,
capture IRQ binding, DMA, capture prescaler or stream is provided.

## Native quadrature

`qei::ClassicAtimQei::new` owns ATIM and two independently qualified
CH1A/CH1B pins. `ClassicAtimQeiConfig` supplies both pulls, one shared native
filter, both phase inversions, mode and `auto_reload`. Modes count CH1A edges,
CH1B edges or both (native SMS4/5/6); ARR1..65535 is accepted, with default
65535 and fixed PRS=/1. ARR0 is rejected before mutation. Initialization writes
CNT=0 after starting native decoding, then retires initial UIF/OVF/UNDF; external
transitions can already change the first subsequent count observation.

`count`, `read_direction`, `reset`, `period_ticks` and `kernel_clock_bounds`
are polling operations. Count is modulo ARR+1; direction is CR.DIR, observed
separately. Overflow and underflow methods use native OVF/UNDF, not the counter
API's UIF. Sticky flags cannot count missed wraps or extend a signed position.
Reset races external transitions and is not an index reset. There is no public
start/stop, mode/filter/polarity reconfiguration or external rate setter.

Both capture-acceptance fields stay disabled during QEI: the native decoder is
selected independently by SMS, and the polling QEI API owns no capture stream.
Drop stops EN, disconnects SMS, zeros all six acceptance fields, disconnects
both pins and then retires/masks/gates the inner timer.

Own manuals require phase separation strictly greater than one TCLK pulse
width, and each input pulse width strictly greater than two TCLK pulse widths.
These are functional input conditions alongside the chip's electrical limits,
not extra Rust memory-safety preconditions. With PRS=/1, TCLK uses PCLK; nominal
clock rates and whole-rate bounds are not per-cycle timing guarantees. The
native filter is not generic switch debouncing. No electrical phase truth
table, silicon validation or maximum external edge-rate guarantee is inferred.

## Examples and acceptance boundary

[The counter example](../examples/shared/timer-overflow/README.md) owns a GTIM plus
ATIM where present, only GTIM on F002/F020, and GTIM3/GTIM4/ATIM on L012. L012
also executes an Embassy time consumer while GTIM1 remains reserved. The
[native input examples](../examples/shared/classic-atim-input/README.md) poll six real
bonded halves and a CH1A/CH1B pair; F003 uses PB6/PB5 AF4, retaining debug PA5
exclusion. Build scripts validate actual GLOBAL IRQs or qualified input routes
and emit every region of one exact package memory map. They do not enlarge RAM
or FLASH, manufacture signals or assert lifecycle behavior.

Main acceptance records 16 actual standalone ARM libraries and 27 ordinary
firmware ELF links, with every selected successful command exiting zero and
its real compiler artifact retained. Seven event ELFs cover all six GTIM and
five ATIM register versions, including F002's no-ATIM/small-memory case and
L012 shared GTIM34 plus reserved GTIM1 time. Three native capture ELFs own all
six bonded halves on F030C8T7, F003E4P7 and L052C8T6; eight QEI ELFs cover the
eight independently qualified families, including F003 with GTIM reserved.
Nine existing ordinary examples cover changed BTIM/shared-vector, classic and
buffered input, and complementary constructor routes. Dependency HAL archives
retained beside ELFs are not counted as extra standalone library commands.

Ten unique finite source/data gates have successful receipts. One main
generation produced 910 canonical files, with 77 admitted metadata changes and
833 files byte-identical. Inverse checks removed only the new qualification and
routes and recovered the complete old JSON values/array order and Rust metadata
bodies. Canonical PAC register bodies and write policies did not change.
The three existing L010/L011 RCC `dead_code` warnings for
`RCC_LSE_SYSCLK_LSE_EDGES`, `RCC_LSE_SYSCLK_LSI_CYCLES` and
`RCC_LSE_SYSCLK_MARGIN_LSE_EDGES` remain in applicable rows; main acceptance is
not a warning-free claim.

At main acceptance clean had not run. Final-package completion requires a
separate final-source two-library/six-ELF receipt after fresh generation in the
clean extraction. Main and clean evidence have distinct purposes; neither is a
HAL lifecycle test or hardware run. Compilation/linking establishes scoped
type/API/vector integration and real memory fit, not hardware delivery, input
timing, losslessness, silicon errata, low-power progress or an output shutdown
waveform. Existing BTIM/PWM/input histories retain their original scope.

## Verification history

The initial runtime source review found an unconditional
`Icr::write_noop()` call in `advanced/mod.rs`. That helper exists on classic ATIM
but is absent from the actual buffered ATIM PACs. Before ARM compilation, the
runtime correction retained the classic helper and selected the already
qualified buffered `Self::FLAGS` seed `0x00ff3fff` before clearing only UIF.
This was a static integration blocker and source correction, not a reported
compiler failure or a change to generated PACs/register-write policies.

The first data phase passed D01–D07, then D08 failed the existing complete AF
projection comparison for the 37 native-classic selections. The independent
`tests/reviewed_metadata.py::af_projection` expected-route helper had not yet
included the newly approved native-input catalog. A bounded 36-line addition
reconstructed exact CAPnA/B expectations from the hash-bound catalog/own-source
record and independently projected bonded pins. It retained every existing
helper line, route ordering and full actual-versus-expected equality. No new
test case, HAL test, runtime probe or model was introduced, and source proof,
canonical facts and generated outputs did not change. D08 then passed, followed
by the previously pending D09/D10. Acceptance therefore comprises ten successful
unique gates from seven preserved successes plus three continuation commands,
with eleven actual attempts and the original failed D08 retained. It does not
claim one unchanged checker tree across all ten commands.

The first ARM phase passed M01–M23 (16 libraries and seven event ELFs), then M24
exited 101 before compilation because `--locked` rejected the classic input
workspace's lock graph. Mechanical lock copying had retained the old input
workspace's optional time graph while omitting its local forwarding feature.
The correction added only `time-driver = ["embassy-cw32/time-driver-gtim"]`
to the new example manifest. The original 54-package lockfile stayed
byte-identical; no registry version, Git revision, HAL feature, source proof or
matrix command changed. Offline metadata resolution checked both the exact M24
and M29 feature selections, and offline lock generation reproduced the original
lock bytes. The local forwarding feature is dormant in M24; M29 retains its
explicit dependency feature and sole-GTIM reservation.

M24–M43 then supplied twenty successful ELF commands. The full main result
retains 43 successful selected rows from 44 actual attempts, the failed M24
logs, and the original 23 successful artifacts at their original paths. The
manifest-only transition leaves those first 23 commands' actual compilation
inputs unchanged. Generated files, toolchain and Cargo configurations remained
stable. This is a documented continuation across two manifest snapshots, not
an uninterrupted same-whole-tree run or a reason to add new matrix rows.
