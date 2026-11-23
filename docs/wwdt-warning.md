# Window-watchdog warning

`WindowWatchdog::enable_early_warning` and `wait_early_warning` are available
on the ten supported families with WWDT: A030, F002, F003, F020, F030, L012,
L031, L052, L083 and R031. L010/L011 lack WWDT; W031 is unsupported.
The non-L012 extension is release-built; count/timing tests and wait ownership
checks are recorded in [software acceptance](validation-scope.md). Hardware warning
delivery remains unverified.

The typed `WDT => wdg::WindowInterruptHandler` binding installs permanent
service before setting irreversible IE. The handler clears only WWDT's W0C
POV and latches one coalesced notification. Cancellation or owner Drop removes
only the waiter; service persists until reset. Readback failure after the IE
attempt retains service. No callback holds caller-owned storage.

The source basis is the per-family manual table and register review in
[WWDT evidence](window-watchdog-evidence.md): each chapter's §x.3.5 defines
POV/reset at 0x40/0x3f and irreversible IE; §x.6 defines the sole W0C POV.
All three curated WWDT layouts expose the same IE bit10 and POV bit0; their
own approved CMSIS headers identify WDT IRQ0. Build-time version checks retain
family-specific register selection; this is not inference from layout alone.

The shared vector also serves IWDT. Independently enabled IWDT interrupts need
their own handler in the binding. This service never disables/unpends the vector,
feeds, stops, resets or gates a running watchdog. Keep the clock, gate and binding
valid until reset. One counter tick remains after POV; IRQ/task delivery before
reset is not guaranteed.

The example in `examples/shared/l012-wwdt` selects one exact package and intentionally
never feeds, so execution causes reset. It is not a late-feed or persistence
strategy. Hardware timing and shared-source coexistence require board acceptance.
