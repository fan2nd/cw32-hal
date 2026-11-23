# Optional Embassy run-mode time driver

The first driver uses a whole 16-bit GTIM, nominally 1 MHz, with the exact pinned
Embassy `Driver::{now,schedule_wake}` and `Queue` APIs. Enable
`time-driver-gtim` on F002/F003 or `time-driver-gtim1` on other supported families.
Both imply `rt` and the same optional `embassy-time-driver` 0.2.2 and
`embassy-time-queue-utils` 0.3.2 Git revision already used by the HAL:
`f16efeffe37581092ec184718e6fdb1620393214`. The firmware example also pins
`embassy-time` 0.5.1 and `embassy-executor` 0.10.0 to that revision.

This is a source-reviewed, compile/link-checked run-mode implementation. No board
has been flashed or executed. It does not establish hardware timing, accuracy,
errata qualification, or compatibility with arbitrary application interrupt loads.

## Ownership, initialization and clocks

The selected peripheral is absent from public `Peripherals`, PWM/counter instance
implementations, timer-pin bindings and DMA request bindings. Generated metadata
selects a private token/alias, actual typed PAC constant, exact IRQ name/number,
and the existing central RCC_INFO gate/reset policy. The vector is owned by the
global driver. A second global time driver or vector definition will conflict at
link time. There is no timer constructor, clock-switching API, channel sharing,
`time-driver-any`, or GTIM3/GTIM4 selector. L012 GTIM3/4 share a vector and need a
separate ownership/dispatch design.

`init` validates the exact source/divider rational before taking the singleton
set. `UnsupportedTimeDriverClock` rejects unsupported nominal 1 MHz configurations.
After RCC initialization it enables/resets/initializes the timer before returning
any peripherals. A timer clock/reset failure returns `TimeDriverClockFailure`;
like an RCC hardware failure, it consumes ownership and requires a reset before
retrying. Invalid configuration consumes no singleton. The IRQ priority is P0;
applications must retain its ownership and bounded service latency.

Classic PRS timers (x030/F020/F002/F003) divide by powers of two. Under currently
supported RCC limits, exact nominal 1 MHz requires PCLK of 8, 4, 2 or 1 MHz.
Nominal 48/24/12/6/3 MHz cannot be divided to 1 MHz by those timers. Linear PSC
variants (L010/L011/L012/L031/R031/W031/L052/L083) require a positive exact integer
MHz PCLK and PSC+1 in 1..65536. Both obey the existing qualified RCC electrical
limits. `ClockBounds::exact_divisor_for` compares the undivided nominal HSI source
against TICK_HZ × exact source/bus divisor, with checked wide arithmetic. It does
not reconstruct a clock from the rounded public Hertz value: HSI /14, for example,
is rejected when no exact integer timer divisor exists. Defaults work on all 13
families (nominal PCLK 8 MHz except L010/L011 at 4 MHz). RCC is never auto-changed.

`time_driver::tick_bounds()` returns the propagated actual frequency envelope
once initialized. F002/F020 have ±5% source bounds; the other reviewed families
have ±2%, conditional on factory trim, specified ambient/supply intervals and
unchanged clocks. A nominal 1 ms tick delay can elapse after about 0.952 ms or
0.980 ms respectively at the fast endpoint. Embassy Duration is nominal and
is neither a guaranteed minimum wall-time delay nor a UTC clock.

## Epoch and monotonicity contract

The pinned Driver is a **safe trait**, not an unsafe trait. Its `now()` contract
requires nondecreasing values, a practical overflow horizon of at least about
10,000 years, and no failure before hardware initialization. The implementation
uses the exact current API, not the historical alarm-allocation API.

ARR is 0xffff and CH1 compares at 0x8000. A u64 half-period count P increments
once for each observed overflow and CH1 flag. Under a global critical section,
the timestamp candidate is `(P * 32768) + (CNT xor ((P & 1) << 15))`.
A single pending half-boundary is inferred from parity, so ISR/CNT races do not
require treating the UIFCPY bit as extra counter width. CNT/ARR/CCR remain 16-bit.
The u64 arithmetic saturates only at its roughly 584,542-year 1 MHz horizon.
The upstream u32 half-period storage would wrap after about 4.46 years at this
rate and is deliberately not reused.

Every regular `now()` reads the epoch and counter while holding the same critical
section as the ISR, then publishes `max(candidate, last_published)`. This last-value
clamp ensures time cannot roll backward when a blackout loses wraps. It does not
recover the missing elapsed time: the clock can freeze temporarily or remain
behind the real counter timeline, and deadlines can be late. Driver monotonicity
is distinct from faithful elapsed-time accounting.

Before initialization, `now()` returns zero under that same serialization without
any timer/core MMIO. The context check reads the core IPSR register directly.
There is no unsynchronized pre-init fast path that could return zero after an
interrupt initializes and observes a positive timestamp.

Cortex-M0/M0+ PRIMASK does not serialize NMI/HardFault. Those contexts therefore
never enter the epoch/queue critical-section path or touch GTIM. `now()` returns
the last stable published timestamp. Two pairs of atomic u32 words hold u64
snapshots; a release store switches the active slot after both words have been
written. A fault/NMI reads the active slot, and the interrupted single-core writer
cannot resume until that reader returns. No spinning seqlock or non-atomic u64
read is used. This is a stale observation, not an elapsed-time or fault-recovery
promise. SRAM corruption or a violated critical-section implementation contract
is outside the normal Rust/platform execution model.

## Required blackout bound and interactions

From each half-boundary through completed service, the total latency must remain
**strictly below 32768 actual timer ticks**, including IRQ masking, higher-priority
work, flash stalls, queue and Waker callbacks, and ISR execution itself. At the
fastest qualified direct-HSI endpoint, conservative wall-time budgets are strictly below:

- F002/F020 (±5%): 31.207619 ms
- Other reviewed direct-HSI profiles (±2%): 32.125490 ms

These numeric wall-time examples require cycle-timing qualification and do not
apply to the new L083 PLL rate-only envelope. For PLL, the service requirement
remains strictly below 32768 actual timer ticks; converting it to a guaranteed
wall-time limit requires separate qualification. The PLL's specified cycle-to-cycle
jitter alone does not supply an absolute-period bound. This does not assert that
all multi-cycle durations are unbounded.

The nominal 32.768 ms period is not the guaranteed wall-time budget. These are
mathematical constraints; no measured application latency is asserted. Two or
more unserviced boundaries cannot be reconstructed reliably from one-bit flags.

Long critical sections, debug halts, gating/stopping the timer, runtime clock
changes and deep sleep have no elapsed-time/alarm guarantee. In particular, the
existing `Flash::operate` holds a critical section across a BUSY loop with no
qualified maximum completion time; FLASH fetch also stalls while busy. The
sources give typical timings, not a guaranteed worst-case bound. Consequently,
program/erase cannot presently be qualified concurrently with this timebase's
accuracy/timeliness contract. An arbitrary finite poll budget in another driver
also does not by itself prove this wall-time bound. DMA draining Drop and any
other unbounded work while interrupts cannot run need the same application audit.
No source change here claims to make those operations bounded.

## Compare flags, queue and rearming

Classic GTIM CH1/CH2 use CMMR mode 0xA, which actually generates comparison events
while forcing the internal output low. Classic mode zero means no function.
Buffered GTIM uses output/frozen mode with compare preload disabled and no pin
outputs enabled. It sets URS overflow-only and issues one stopped UG to latch PSC
and reset CNT. GTIM EGR remains RW as its own tables specify, but the command is
always a direct write, never a read-modify-write.

The ISR snapshots status once and sends one typed ICR W0C command clearing only
observed owned overflow/CH1/CH2 flags. All unobserved flags and documented reserved
reset values are preserved. It then advances the epoch and checks the software
queue against a fresh timestamp. An observed CH2 flag never proves an absolute
deadline has been reached.

Rearming disables CH2 IRQ, clears stale CH2 only, writes CCR2 immediately, and
enables it only for a future deadline fewer than 49152 ticks away. It then samples
`now()` again. If equality was missed while programming, it disables CH2 and loops
through `Queue::next_expiration` until a future alarm is successfully established.
Clear-before-write and the final timestamp check cover the compare programming
race. Early/stale matches cause queue rechecking. Distant deadlines are revisited
at half-boundaries. `u64::MAX` explicitly disables CH2; arithmetic never adds a
horizon to a nearly overflowing absolute timestamp. CNT is never reset and UG
is never forced after the clock starts.

There is exactly one `Mutex<CriticalSectionRawMutex, RefCell<Queue>>`. The epoch
and published timestamp are separate from its mutable borrow so Waker callbacks
can read time. Pre-init schedule calls retain queue entries and do no timer MMIO;
initialization services them after starting the clock. Generic queue capacity
pressure retains its upstream behavior (a displaced task is woken early).

The trait's brief schedule_wake documentation does not expressly specify spurious
wakes. Its pinned generic Queue does wake future entries under capacity pressure,
and `Timer::poll` rechecks the absolute deadline before Ready. This driver uses
those exact queue semantics, but does not infer an arbitrary early-wake fallback
for NMI or reentrant scheduling.

The pinned Queue invokes wake/clone/drop callbacks while its mutable borrow is
held and offers no callback-drain/deferred-delivery API. Reentrant schedule_wake is
therefore detected by try_borrow_mut and **panics deterministically**, matching the
upstream mutable-borrow boundary. NMI/HardFault schedule_wake is likewise rejected
with a panic before touching the protected queue. It does not call the offending
Waker recursively, silently drop a required future wake, spin on an interrupted
owner, or access aliased state. Deferring arbitrary Wakers would need additional
owned storage or a changed upstream queue API; neither is implemented here.
These are explicit safe-API failure modes, not an unsafe caller obligation or an
assertion that arbitrary Wakers are sound because a comment excludes them. The
Driver contract's never-fails requirement is specifically on now(); now() remains
separate from the queue borrow and can be called by Waker callbacks and fault
handlers. As with all other safe callback APIs, a user Waker/panic handler may
itself diverge, panic, or recurse; this driver does not add a recursive wake path.
No new unsafe Sync implementation or untyped register adapter is introduced.

By default the pinned integrated Queue accepts Embassy task wakers and requires
exactly one integrated timer queue in the system. The HAL chooses no generic
capacity. Applications using another executor select a `generic-queue-*` feature
on the same pinned `embassy-time-queue-utils` instance. The example's
`generic-queue` feature demonstrates an application-selected capacity of eight.

## Evidence, data changes and validation

`time-driver-evidence.json` carries all 13 own-family manual URLs, revisions,
hashes and numbered pages, actual resource/IRQ/clock facts, plus exact upstream
file hashes and commit links. `timer-command-evidence.json` records the PAC-only
corrections. Existing field-access metadata removes four buffered CNT.UIFCPY
setters while retaining CNT low16 writes and UIFCPY reads. Existing register-writes
metadata supplies typed ICR reset/no-op seeds for all 12 timer variants. There is
no schema extension, canonical register YAML/IR remap, or rewrite-provenance
change. Classic GTIM seed 0x3ff preserves reserved-one bits7:8; it differs from
the defined clearable flag mask0x27f. Buffered GTIM uses0x00f01e5f, classic BTIM7,
and buffered BTIM0x41. No unrelated PAC field is changed.

`ci/check-time-driver.py` performs production ARM feature builds, expected-invalid
feature selections, generated resource ownership checks, and real firmware links.
`examples/shared/embassy-time` uses the actual pinned Embassy executor and Timer with two
concurrent tasks and deadlines both within and beyond the 16-bit counter period.
No HAL tests, mock MMIO, test-only adapter, or firmware execution are involved.
Receipt manifests retain final source hashes and every compile/link log. They
prove source/build properties only, not physical timing or the application blackout
bound. A separate always-on source/sleep-coordination design is required for a
deep-sleep clock.
