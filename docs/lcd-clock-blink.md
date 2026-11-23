# LCD held LSE and blink/event intervals

Scope: the existing three exact L052 and five exact L083 LCD package features.
No new oscillator or LCD routing cohort is inferred. Internal resistor bias,
owned pads and declared glass/supply limits remain mandatory. No hardware tests
have been performed for these additions.

## Source basis

The pinned own L052 Chinese manual V1.5 (SHA-256
`4bac53df4db69a3b76c833cb14dd45b0e5c7f9884506c83a5cda6ea00a859f41`)
§26.3.2 p535 selects LSI/LSE; §26.3.7 p548 and §26.4 p549 specify blinking
and the shared interrupt interval. The own L083 Chinese manual V2.0 (SHA-256
`9930bf1755f3bbf8933163c2d0da57fd9a4f3250a358a4c0bfc75ed4eda3a0a3`)
has the corresponding text in §27.3.2 p543, §27.3.7 p557 and §27.4 p558.
The RW BLINKEN/BLINKCNT register descriptions and blink sections give no
LCD-disabled or busy-handshake prerequisite; transition phase is not specified.
These are the source authorities already pinned by `docs/lcd-evidence.json`.

## Clock ownership

`Lcd::new` keeps LSI behavior. `new_with_lse(..., config, clock)` consumes an
existing `rcc::LseClock` and retains it for the complete display lifetime. It
checks current source health, selects LSE while LCD is disabled, and never calls
`enable_lsi`. Only `cfg(rcc_lse)` exposes the constructor. The lease is not cloned,
released to a second driver, or converted into a live clock-retuning API.
LCD Drop disables display and pins before its lease fields are dropped. Existing
LSE clock/pad reservations remain retained according to `LseClock`'s contract,
including failed acquisition; this is not cold-start or sleep support.

`typical_source_frequency` now reports selected source nominal frequency:
typical LSI or the held LSE board declaration. `source_bounds` returns `None`
for LSI and the healthy-source declared LSE bounds for LSE. Neither measures
frequency or promises continuous health. Source loss can leave async waits
pending indefinitely. Existing `ScanFrequency` variant names express the
manual's nominal LSI rates/divisors, not exact output Hz with LSE.

## Blink and honest event names

`set_blink(enabled, event_interval_frames)` accepts 1 through 64 and encodes
BLINKCNT = interval - 1. The same refresh interrupt interval applies with
blinking disabled. The manuals call frame rate / interval the blink frequency;
no full visible/blank cycle factor, duty, phase or exact transition is inferred.
The setter masks only LCD IE, retires a forgotten wait, clears its old event,
and preserves source, scan, DMA, unrelated control fields, pins and display RAM.
Invalid input performs no register or waiter-state change.

`wait_next_event` / `wait_next_event_async` wait for a fresh coalesced refresh
interval event, not an event count or a particular frame number.
`present_on_event` / `present_on_event_async` copy the buffer afterward.
A fresh wait discards a preexisting event, not the hardware interval phase:
its first notification may occur in less than a complete interval.

`wait_next_frame`, `wait_next_frame_async`, `present` and `present_async` require
interval 1. Otherwise they return `Error::EventIntervalNotOne` before arming IE,
clearing flags or writing display RAM. **Breaking API:** the two asynchronous
frame-named methods now return `Result<(), Error>`; callers must handle it.
Blocking event timeout retains the existing `FrameTimeout` error variant.
No method silently changes blink interval to obtain frame events.

All async methods retain the original cancel/forget/Drop safety model. Wakers
are owned static values, never references to caller buffers; clone/drop/wake
are outside critical sections. Shared AUTOTRIM flags and NVIC enable state
are untouched. Cancellation before presentation writes no RAM. LCD RAM is live:
there is still no atomic latch, complete-frame display or analogue-settling
claim. A software test is not hardware or optical validation.

## Example and validation

`examples/shared/lcd/src/bin/lse_blink.rs` shows held PC14/PC15 crystal ownership,
explicit board assumptions and 32-frame blink/event intervals. The existing
blocking and async binaries keep one-frame behavior; async calls handle Result.
Use actual exact package features and the repository's release example build.

Temporary host tests extract actual production State, ISR, FrameWait, frame/event
waits, blink setters/getters and owner Drop, replacing MMIO only. They cover all
64 valid intervals, 0/65/255 rejection without writes, unrelated CR1 and shared
source preservation, checked frame rejection, stale events, cancellation,
waker replacement, event-before-registration, forget/restart/Drop and setter
retirement. They do not validate oscillator startup or glass waveforms.
