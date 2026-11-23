# Coalesced GTIM/ATIM overflow events

`overflow_events` is an ordinary Embassy application using the existing
`Timer<T, Async>`, typed `OverflowInterruptHandler<T>`, `wait_overflow()` and
`take_overflow()` API. It waits for housekeeping notifications from a GTIM and
ATIM together where both exist. F002/F020 own only their GTIM. These are
coalesced indications that at least one wrap happened, not wrap counts or an
elapsed-time service. Running PCLK, interrupt delivery and executor service are
required; no sleep/wake or stopped-clock progress is promised.

The build script validates each selected timer's real GLOBAL interrupt and
writes `event-bindings.rs`, `event-bindings.txt` and the exact package's
`memory.x` into Cargo's OUT_DIR. It never enlarges FLASH or RAM. The default is
F030C8T7; the manifest lists thirteen exact part features. Use one part at a time.

```sh
cargo build --release --locked --offline --target thumbv6m-none-eabi \
  --no-default-features --features cw32f030c8t7 --bin overflow_events
cargo build --release --locked --offline --target thumbv6m-none-eabi \
  --no-default-features --features cw32l012c8t6,embassy-cw32/time-driver-gtim1 \
  --bin overflow_events
```

L012 deliberately provides the GTIM3 + GTIM4 + ATIM combination. Its chip feature
also activates the already pinned optional Embassy time dependency and reserves
GTIM1 for the HAL time driver. Both GTIM3 and GTIM4 handlers dispatch from
GTIM34. The loop calls `embassy_time::Timer::after_millis` while all three event
owners are alive. The explicit HAL feature in the second command repeats that
same reservation; it does not add another feature or another timer owner.

`HOUSEKEEPING_EVENTS` is a debugger-visible latest-loop bitset: bit 0 is the
primary GTIM (GTIM3 on L012), bit 1 is ATIM and bit 2 is L012 GTIM4. The program
waits on every owned bank, cancels unselected waits, then polls each owner for
notifications already pending. Cancellation retains counting and events;
multiple wraps can still coalesce. The bitset is an application observation,
not an ISR count or a lifecycle assertion.

The release profile is size optimization (`opt-level = "s"`), LTO, one codegen
unit and no debug information. Dependencies reuse the repository's pinned
Embassy revision. Compilation and linking demonstrate API/vector integration;
they do not establish silicon event delivery. See the
[bounded timer event and input contract](../../../docs/timer-event-input.md).

## Capture notification

The additional `capture_event` binary uses the primary timer as an owned direct
external-input capture instead of an overflow counter. Its generated
`capture-route.txt` gives the exact package pad and GLOBAL IRQ. It awaits a
coalesced indication, disables capture briefly for a latest-value read, then
resumes; it is not an edge counter or lossless stream. No signals are generated.
See [the capture IRQ contract](../../../docs/timer-capture-irq.md). This new binary
is not covered by historical overflow-only validation claims above.
