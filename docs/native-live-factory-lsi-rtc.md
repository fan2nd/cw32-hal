# Native live factory-LSI RTC

This source capability covers exactly CW32L010F8P6/F8U6/Y8M6,
CW32L011K8T6/K8U6 and CW32L012C8T6/C8U6. It adds an owned native LSI
calendar source to the existing RTC driver. Generic L010/L011/L012 aliases
and every other chip are excluded from this new capability.

Stage79 main verification passed twelve actual HAL library builds and seven
linked firmware ELFs, with all nineteen commands exiting zero. Generation and
eight finite source/data gates passed in separate invocations; all 905 generated
data/PAC files stayed byte-identical, with no added or removed path. Independent
main runtime and source/metadata reviews accepted the frozen implementation.
Three existing unused LSE-constant warning headers remain in the L010/L011
rows; this is not a zero-warning result. At main acceptance clean had not run;
final-package completion requires a separate final-source two-library/two-ELF
clean receipt. No software result establishes on-device
behavior, oscillator accuracy, or board qualification.

The first main attempt stopped at its first library with E0507 in the new
build-time field comparison and produced no library artifact. Comparing the
non-Copy access values by reference preserved the intended equality check.
The corrected nineteen-row main run produced the twelve libraries and seven
ELFs above; the failed attempt remains separate evidence.

## Source ownership and live acquisition

After normal HAL initialization, `rcc::LsiClock::new(sysctrl)` takes the existing
unique `SYSCTRL` lease. The native constructor has one argument and performs
reads only. The oscillator must already be software-enabled, stable, and at
its own factory trim, for example following qualified board startup or a
bootloader. An ordinary POR with LSI stopped does not satisfy this prerequisite.
An automatic hardware request alone is insufficient. The capability neither
starts nor waits for LSI and never changes TRIM, WAIT, a detector/monitor,
oscillator requests, or IWDT gates, resets, or commands.

The clock's ownership and rate are independent of SYSCLK. An HSI CPU with an
LSI calendar is supported. Selecting `Sysclk::LSI` does not construct or own
this RTC source, and its separate initialization-only qualification does not
provide cold-LSI startup. The example uses default HSI system initialization.

Acquisition first requires initialized clocks. Within one critical section it
reads the own factory halfword at `0x001007C2`, rejects raw `0xffff`, and
normalizes it using the family's typed TRIM setter: ten bits on L010/L011,
nine on L012. This setter changes only a local register value. It then reads
CR1, LSI, LSI, CR1, in that order. Both sampled trims must match the normalized
factory value, both CR1 samples must have LSIEN set, and both LSI samples must
have STABLE set. Zero factory trim is valid. An incompatible trim returns
`RtcClockError::IncompatibleCalibration`; absent enable/readiness returns
`NotReady`. Neither error writes hardware.

Every existing RTC source-readiness check repeats these observations, including
the factory-halfword comparison. Loss of enable, stable status, or calibration
compatibility returns `RtcError::ClockNotReady`. These finite samples do not
measure frequency or establish continuous health, time progression, or
atomicity against foreign NMI, debugger, or unsafe register changes. They leave
inherited WAIT, detector routes, monitoring, interrupts, and consumers intact.

The capability has no pad lease and is not cloneable. `Rtc` owns both the RTC
token and the converted clock. Drop does not stop, gate, or reset the source;
forgetting a borrowed capability does not revoke physical oscillator or calendar
activity. Reborrow tokens when ownership must be retained after failed acquisition
or construction.

## Own source bounds and native division

The source is nominally 32,800 Hz. All bounds below are average-rate envelopes,
conditional on unchanged factory trim and the device's own qualified electrical
and ambient conditions. The HAL does not measure those conditions.

| Family | Factory rate, Hz | Supply, mV | Ambient, C | Calendar second transitions per SI second |
| --- | --- | --- | --- | --- |
| L010 | 31,816–33,784 | 1,620–5,500 | −40–85 | 0.97–1.03 |
| L011 | 29,520–41,000 | 1,700–5,500 | −40–85 | 0.90–1.25 |
| L012 | 29,520–36,080 | 1,700–5,500 | −40–85 | 0.90–1.10 |

Keep the own supply conditions, including VDDA=VDD where required, and the
initialization contract. Native LSI RTC bounds reuse the existing `RCC_LSI_*`
facts and `ClockBounds::lsi()`; they do not reuse native RTC HSIOSC facts.

SOURCE is 2. The exact qualified divisors are `(1, 16400)`, encoded as
PSC1=0 and PSC2=16399. The native 2 Hz TICKCLK produces nominal one-second
calendar transitions, for a total calendar divisor of 32800. The 41,000 Hz
worst own-family source rate remains below the RTCCLKD 1 MHz ceiling with
divisor one. This rate comparison supplies no per-cycle qualification.

`source_clock_bounds()` and `calendar_tick_bounds()` remain rate-only under
every supported SYSCLK. Calendar division preserves the exact fraction and
`has_cycle_timing_bounds()==false`. Whole-Hz getters return nominal 1 and
outward-rounded lower/upper 0 and 2; these rounded endpoints are not the
fractional rates in the table. `minimum_duration_ns` and `maximum_duration_ns`
panic for rate-only bounds. Do not call them on this source or its calendar
bounds, or interpret the rate as an individual-period, jitter, or accuracy
guarantee. Classic LSI RTC retains its separate fixed /32768 divider and exact
nominal 32800/32768 rate.

L011's own datasheet Table 7-19 specifies factory error −10/+25%, while its
manual warns about adjustment outside 32.8 kHz ±10% and gives different
trim-step/range statements. This conflict remains unresolved. Do not clip the
factory maximum to 36,080 or 36,000 Hz, infer arbitrary-trim qualification, or
claim every factory-envelope value satisfies a universal monitoring/electrical
legality condition. This capability reports the qualified factory rate and
calendar division; it neither starts nor requalifies HSE/LSE monitoring.

## Preserved attachment and explicit stopped initialization

`Rtc::attach_preserving_state` can enable only the RTC APB configuration gate.
It requires a running compatible calendar, SOURCE=2, exactly `(1, 16400)`,
inactive compensation, and bounded repeated consistent calendar reads. The
alternative pair `(2, 8200)` and the reset/LSE pair `(1, 16384)` are rejected.
Inactive compensation parameters may remain; COMPEN.EN must be clear. Attach
never resets, unlocks, stops, migrates, sets time, disables compensation,
acknowledges events, or clears RESETFLAG. Incompatible retained state is an
error, not permission to repair it.

`Rtc::initialize_if_unset` is explicit provisioning of a stopped calendar with
DATE=0, CR0 clear except H24, CR2=0, compensation=0 and IER=0. L011/L012 also
reject pending ISR flags. L010 retains the RTCLPM=0 and ACCESS-free requirements.
For this source, all three drain WAIT before observing DATE; L010's stopped
direct-access exception removes ACCESS but not active-load exclusion. Inactive
alarm matches and AWT reload values are preserved. There is no peripheral reset,
POR assumption, flag clearing, or migration of an active calendar.

Initialization stages the first prescaler without reducing the incoming
division, confirms its readback, confirms SOURCE=2, then installs and verifies
the final pair. SOURCE readback must succeed before PSC1 is reduced. Incoming
clocking must already be legal; initialization cannot undo an earlier overclock.
H24 and DATE/TIME are configured while stopped; only the final START write
starts the calendar. Failure can leave partial configuration, a staged divider,
or the new source selected. There is no rollback or elapsed-time continuity.

Source/prescaler changes can affect RTC_OUT, RTC_1Hz and timer/event consumers
before START. The existing external-recipient functional handover applies to
the operation and later RTC use: recipients must be absent or their effects
must belong to the requested operation. SYSCTRL/RTC ownership alone does not
own independent recipients, and the API does not inspect or disconnect them.
Stopped counters or closed work gates do not prove their absence. This is a
functional limitation, not an extra memory-safety precondition for safe Rust.

## Domain errors and write cleanup

For this source, concrete RTC observations and writes check the existing APB
gate and reset predicates around their relevant boundaries. An observed closed
gate or asserted reset returns `RtcError::ClockDomainUnavailable` before a
buffered observation can be interpreted or used to choose a write. It takes
priority over errors inferred from unavailable/zero-valued registers, including
readback and synchronization failures. These checks do not reopen gates or
release reset and are not a continuous monitor or arbitrary bus-corruption
guarantee. A persistently zero SOURCE in an observed available domain cannot
acknowledge the SOURCE=2 write: initialization returns `WriteFailure` before
final divisors, H24, DATE/TIME, or START writes.

Native-LSI unlock and explicit normal relock check the domain around the KEY
sequence. Observed domain loss stops subsequent KEY writes; a normal relock
domain failure also returns `ClockDomainUnavailable`, overriding a buffered
operation result. Destructor fallback skips remaining KEY writes after domain
loss and cannot report an error. Domain loss may leave RTC unlocked or partially
configured. No returned error promises relock, rollback, restored ownership,
or gate/reset recovery.

## Writes, alarms, and compatibility

On L010 with this source, `set_datetime` returns `UnqualifiedWriteTiming`
after its initial read-only validity checks and before KEY, ACCESS, DATE or
TIME writes. The running ACCESS transaction has an own-manual one-second
limit, and neither a finite poll count nor a fast/cycle-qualified HCLK alone
proves worst-case execution time. There is no hidden stop/restart correction
or separate stopped-correction route; use explicit stopped-unset provisioning.
The old HSIOSC/LSE ACCESS behavior is unchanged and has its existing unresolved
timing limitation under sufficiently slow or unqualified CPU clocks.

L011/L012 retain their direct stop/WAIT/write/restart correction protocol,
including its discontinuity and partial-failure contract. Their reserved CR1
bits 1:0 do not acquire L010 ACCESS/WINDOW behavior. Synchronous `set_alarm_a`,
`set_alarm_a_enabled`, `alarm_status` and `clear_alarm` remain available under
their existing direct-access rules and the new-source domain safeguards.

For all seven new sources, polling `wait_for_alarm` first returns
`UnsupportedAsyncAlarmSource`, before RTC access, NVIC changes, ACTIVE/WAKER
mutation, handler registration, or waiter/cleanup-guard creation. An unpolled,
cancelled, or forgotten new-source future installs no waiter. Existing HSIOSC
and LSE async behavior is unchanged. Peripheral async metadata does not
override this source-specific refusal.

Native `CalendarClock::new(sysctrl)` still constructs HSIOSC. `HsiOscClock`
and existing LSE constructors, pads, bounds and divisors retain their behavior;
classic `LsiClock::new(sysctrl, poll_budget)` still enables and polls its
pretrimmed source. Native Instance source facts remain HSIOSC SOURCE=3 at
48 MHz on L010 or 96 MHz on L011/L012. They describe that default source,
not the new LSI variant. Generic native aliases still denote HSIOSC.

The exact-seven `CalendarClock` enum is exhaustive. Adding `Lsi` changes its
public shape: downstream matches over only `HsiOsc` and `Lse` must add an
`Lsi` arm. Unchanged constructor behavior is not a claim of universal source
compatibility. Existing error enums remain non-exhaustive.

This feature adds no cold startup, oscillator measurement, compensation,
subseconds, HSI shutdown, watchdog manipulation, sleep, retention, wake,
continuous loss detection, recovery, or ordinary-POR success guarantee.

## Example and fixed verification scope

The ordinary [`live_lsi_calendar`](../examples/shared/rtc-calendar/README.md) binary
requires one exact chip feature and `native-lsi-source`. It uses default HSI
initialization, read-only LSI acquisition, explicit stopped-unset initialization,
and `black_box` retention of `now()` and the two bounds. It uses no GPIO,
watchdog, async wait, oscillator startup or trim programming. Review the sample
date/time and the live-LSI bootloader prerequisite before flashing.

The main rows below passed using target `thumbv6m-none-eabi` with
`--offline --locked --no-default-features`. At main acceptance clean had not
run; its separate final-source outcome is recorded independently. Links are
compile evidence only.

- Main libraries: release each exact-seven `PART,rt` through
  `firmware/Cargo.toml -p embassy-cw32`; dev `cw32l011k8t6,rt,defmt`;
  release each generic `cw32l010`, `cw32l011`, `cw32l012` with `rt`; release
  `cw32f030c8t7,rt`. Total: twelve actual library builds passed.
- Main ELFs: release `examples/shared/rtc-calendar/Cargo.toml --bin live_lsi_calendar`
  with `PART,native-lsi-source` for `cw32l010f8p6`, `cw32l011k8t6`,
  `cw32l012c8t6`; existing `examples/shared/l010-lse-clock/Cargo.toml --bin hsi_calendar`
  with each same part; existing `examples/shared/rtc-calendar/Cargo.toml
  --bin preserve_calendar --features cw32f030c8t7`. Total: seven ELF links passed.
- Separate final-source clean replay: release `cw32l010f8p6,rt` and
  `cw32l011k8t6,rt` libraries, plus one new `live_lsi_calendar` link for each.
  Planned separate final-source scope: two builds and two links; outcome
  recorded separately.

The source policy is [`rtc-live-lsi-qualified.yaml`](../cw32-data/rtc-live-lsi-qualified.yaml);
the own-source/page bindings are in
[`native-live-factory-lsi-rtc-qualification.json`](native-live-factory-lsi-rtc-qualification.json).
The policy validates current source facts without a new projected metadata
field; all existing generated chip/PAC semantics remain outside its delta.
