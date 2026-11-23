# RTC calendar firmware

The three existing Cortex-M0+ binaries below toggle PA4 on observed calendar-second transitions. Connect only a suitable indicator/load; PA4 is an example output, not a claimed board LED. Build with one exact package feature and target `thumbv6m-none-eabi`.

- `preserve_calendar`: attach to a compatible retained calendar and read it.
- `initialize_calendar`: explicitly provision an unset/stopped calendar. Replace the sample date with the intended time before use.
- `set_calendar`: explicitly correct an existing calendar, preserving its 12/24-hour format. Replace the sample date before use.

Classic-family firmware requires board startup/bootloader to have already loaded factory LSI trim. `LsiClock::new` checks the pinned calibration halfword, preserves trim, enables the source and waits for startup. An unmatched trim is an error; the example does not silently calibrate it. The LSI calendar's exact nominal rate is 32800/32768 ticks per SI second, before its own ±3% or F020 ±5% tolerance. This is unsuitable as an accuracy claim.

These existing L010/L011/L012 binaries use frozen HSIOSC and rated-source-safe prescalers. L010 also requires RTCLPM=0 for writes. No example enables alarms, writes compensation, enters deep sleep or claims battery retention. L010 ACCESS waiting is capped at32 reads and can require a retry; no unbounded retry is hidden here. A debugger breakpoint within a calendar write violates its timing assumptions. Links are compile evidence, not on-device rollover/access-window evidence. Do not flash without reviewing board pins, source calibration and initial time.


## Native live factory-LSI calendar

`live_lsi_calendar` is a separate ordinary binary for exactly
CW32L010F8P6/F8U6/Y8M6, CW32L011K8T6/K8U6 and CW32L012C8T6/C8U6.
Select one exact part with `native-lsi-source`; the build guard rejects all
other parts. The existing `hsi-source` marker and old example constructors
remain unchanged.

```sh
cargo build --manifest-path examples/shared/rtc-calendar/Cargo.toml \
  --target thumbv6m-none-eabi --release --offline --locked \
  --no-default-features --features cw32l011k8t6,native-lsi-source \
  --bin live_lsi_calendar
```

Before use, board startup or a bootloader must already have software-enabled
LSI, loaded its own factory trim and reached STABLE. Ordinary POR with LSI
stopped cannot run this example successfully. It uses default HSI system
initialization and read-only `LsiClock::new(p.SYSCTRL)`; it never selects
`Sysclk::LSI`, starts/trims the oscillator, changes WAIT/monitoring, or manipulates
IWDT. The RTC source is independently owned through SYSCTRL. Missing
prerequisites reach the normal panic handler; there is no startup fallback.

Review and replace the sample date/time before flashing. This is deliberate
`Rtc::initialize_if_unset` provisioning of an unset, stopped calendar, not
preserved-state attachment or correction of an existing calendar. RTC source,
divider and START changes must satisfy the existing downstream-recipient
functional handover. The loop retains `now()`, `source_clock_bounds()` and
`calendar_tick_bounds()` with `core::hint::black_box`. It uses no GPIO indicator
or additional peripheral.

SOURCE=2 and exact divisors `(1, 16400)` give a total calendar divisor of 32800
and nominal one-second transitions. The own rate ranges are L010
31,816–33,784 Hz, L011 29,520–41,000 Hz and L012 29,520–36,080 Hz. The divided
ranges are 0.97–1.03, 0.90–1.25 and 0.90–1.10 transitions per SI second.
They remain rate-only even with an HSI CPU; do not call strict duration helpers
or treat whole-Hz bounds rounded to 0 and 2 as the fractional rate envelope.
The own supply/ambient conditions and unresolved L011 factory-envelope versus
monitor-adjustment warning remain in force.

An observed unavailable RTC register domain returns `ClockDomainUnavailable`
and suppresses subsequent KEY cleanup after that loss; no rollback or relock is
promised. L010 new-LSI `set_datetime` refuses with `UnqualifiedWriteTiming`
before KEY/ACCESS writes. All seven new-source async waits refuse with
`UnsupportedAsyncAlarmSource` before registration or hardware access.
Synchronous direct alarms remain available, although this example does not
configure them. Existing HSIOSC/LSE constructor behavior is retained; downstream
exhaustive `CalendarClock` matches must include the added native `Lsi` variant.
There is no ordinary-POR startup, sleep, retention, wake, continuity or accuracy
promise. See the [full source contract](../../../docs/native-live-factory-lsi-rtc.md).

Stage79 main verification passed twelve actual library builds and seven ELF
links, all with exit zero. Generation, eight finite source/data gates and the
905-file zero-delta comparison passed separately. Independent main runtime
and source/metadata reviews accepted the frozen implementation. Three existing
unused LSE-constant warning headers remain in L010/L011 rows. At main acceptance
clean had not run; final-package completion requires a separate final-source
two-library/two-ELF clean receipt. Successful links do
not establish hardware operation or board qualification.
