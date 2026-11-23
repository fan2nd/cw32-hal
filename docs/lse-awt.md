# Owned LSE AWT run-mode events

`awt::Awt` can retain an initialized, board-qualified `rcc::LseClock` for polling
or interrupt-backed coalesced underflow events on exactly eight selections.
This is a bounded run-mode API. Source readiness, retained events and elapsed
wall time are separate observations.

## Exact scope and existing register variants

| Exact part | Package | PC14 / PC15 positions | Canonical AWT IR | LSE supply / ambient |
| --- | --- | --- | --- | --- |
| CW32F020C6U7 | QFN48 | 3 / 4 | `awt_cw32f030_v1` | 1.65–5.5 V / −40–105°C |
| CW32F030C8T7 | LQFP48 | 3 / 4 | `awt_cw32f030_v1` | 1.65–5.5 V / −40–105°C |
| CW32A030C8T7 | LQFP48 | 3 / 4 | `awt_cw32f030_v1` | 1.65–5.5 V / −40–105°C |
| CW32L031C8T6 | LQFP48 | 3 / 4 | `awt_cw32l031_v1` | 1.65–5.5 V / −40–85°C |
| CW32L031C8U6 | QFN48 | 3 / 4 | `awt_cw32l031_v1` | 1.65–5.5 V / −40–85°C |
| CW32L031F8U6 | QFN20 | 1 / 2 | `awt_cw32l031_v1` | 1.65–5.5 V / −40–85°C |
| CW32R031C8U6 | QFN48 | 2 / 3 | `awt_cw32l031_v1` | 2.2–3.6 V / −40–85°C |
| CW32W031R8U6 | QFN64 | 61 / 62 | `awt_cw32l031_v1` | 2.0–3.6 V / −40–85°C |

These are the current standalone-AWT/exact-LSE intersection in
[`lse-qualified.yaml`](../cw32-data/lse-qualified.yaml). The private `awt_lse`
build admission checks the exact set, current qualification, `awt_source=3`,
existing IP version and typed `Source::Lse=3`. It does not add a hardware schema
or broaden admission whenever another package acquires LSE qualification.
W031 retains its 2.0 V LDO/DCDC intersection; this grants no RF qualification.

Generic aliases, other packages and the LPTIM families are excluded. Existing
`new`/`new_async` keep HSIOSC and all existing method signatures. F002/F003
remain on `awt_v1`, where encodings 2/3 are HEX_PB0/HEX_PB1; they gain no LSE API.
RTC's internal wake timer is a different block from standalone AWT.

## Source capability and construction

Initialize LSE through the existing RCC configuration, then acquire
`LseClock::new` for crystal mode or `new_bypass` for a digital input. Crystal
mode consumes SYSCTRL, PC14/OSC32_IN and PC15/OSC32_OUT; bypass consumes SYSCTRL
and PC14. The actual board must satisfy the existing source contract for
loading, drive, amplitude, waveform, voltage, temperature and aging. Nominal
32768 Hz and average ppm alone do not establish the required per-cycle bounds.
Typical crystal startup figures do not supply a maximum startup duration.
See [the active LSE contract](qualified-lse.md).

The additive constructors consume and retain that capability:

- `Awt::new_lse(peri, clock, config) -> Result<_, awt::Error>` constructs a
  Blocking owner.
- `Awt::new_async_lse(peri, irq, clock, config) -> Result<_, awt::Error>` also
  requires the genuine typed AWT interrupt binding and constructs an Async owner.
- `is_clock_ready() -> bool` is available on the admitted cohort, including
  AWT owners constructed through the old HSIOSC constructors.

Configuration validates reserved prescaler encoding zero and initialized RCC
before the new path observes `LseClock::is_ready()`. An unhealthy observation
returns `ClockNotReady` before any AWT or AWT-gate write. This neither diagnoses
the exact fault nor proves the next source edge. A fault can occur after the
observation and coexist with successful construction.

The existing gate policy then enables AWT configuration access and rejects
asserted reset without releasing foreign reset ownership. Local programming
stops EN, masks UD, selects the held source and Timer mode with EN clear, writes
ARR, clears old hardware/software events, and installs the requested event
mode. Async enables the typed NVIC vector after that common initialization.
No new LSE-edge acknowledgment, source-fault interrupt or post-programming
health error is introduced.

Invalid configuration, missing clocks or unhealthy-LSE admission leave AWT's
previous local hardware/static state intact. Existing gate-readback failures
may leave a gate enabled, and reset rejection follows gate enable. Every error
retains LSE, monitor LSI, source parameters, diagnostics and pad reservations.
Consumed tokens are not automatically returned; applications can use supported
reborrows when acquiring the clock and timer if later reacquisition is needed.

### GPIOC acquisition window

Acquiring or reacquiring `LseClock` calls the existing generated pad check
through `GPIOC.inspect_for_init`. It may temporarily open an originally closed
whole-GPIOC configuration/working gate to inspect PC14/PC15. Whole-bank
sampling, retained filters and armed edges can advance in that window,
including before an acquisition error. Restoring the gate does not undo that
progress. Reborrowing and reacquisition are therefore not observation-only
operations for those functional recipients.

AWT's constructor readiness check and `is_clock_ready()` do not inspect pads
or reopen GPIO banks. The AWT extension adds no recipient-bank scan.

## Counting source, configuration domain and bounds

Both existing AWT IPs select LSE with SRC=3, use Timer MD=3, accept PRS=1..15
(divide by 2..32768) and reject reserved PRS=0. The period is
`(ARR + 1) * 2^PRS` source cycles. EN activation reloads CNT from ARR.
`set_config` stops first, preserves the selected source, replaces divider/ARR,
clears old events and remains stopped; it cannot silently revert to HSIOSC.

LSE bounds come from the held source capability. Each configuration divides
those original rational bounds by the actual prescaler, without using rounded
hertz or dividing the previous tick again. `frequency`, `clock_bounds` and
`period_bounds_ns` describe the declared source while its board contract and
healthy counting hold. They do not measure frequency or prove uninterrupted
elapsed time after loss. Factory-LSI rate-only bounds are not admitted here.

AWT's configuration domain is PCLK with active-high `APBEN2.AWT` gating and
active-low `APBRST2.AWT` reset. The selected counting clock is independent.
The own manuals specify direct CR/ARR/CNT/IER/ISR/ICR access and no LPTIM-style
source-edge update acknowledgment. This supports the bounded inference that
local control/event access remains usable after counting-source loss while
CPU, PCLK, gate and reset domains still operate. It does not prove maximum bus
latency, a reliable frozen CNT value or execution after a system-clock fault.
If SYSCLK also uses LSE, its separate fault/fallback limits still apply.

## Readiness, events and source loss

`is_clock_ready()` directly samples the held `LseClock`'s existing enabled,
stable, parameter, sticky-fault and applicable qualified-monitor predicates.
The classic auxiliary and frozen system-source paths retain their distinct
policies. Existing x030/F020 mandatory CCS is HAL admission policy, not a
claim that silicon fixes the detector on. HSIOSC-created owners on this cohort
instead sample HSIEN and HSI.STABLE. None of these samples measures frequency,
guarantees a future edge or provides continuous supervision.

The query does not acknowledge UD/source faults, stop AWT, change interrupts
or repair the source. Health and UD/CNT are not observed atomically.

- `take_elapsed()` consumes the logical OR of retained software elapsed and
  hardware UD, including after source loss. An event may precede a fault or
  occur around it. It does not reveal an event count or timestamp.
- The ISR uses static active/event state and local IER.UD/ISR.UD only. It clears
  UD using the generated R1W0 no-op seed `0x3f`, latches the event and wakes
  outside the critical section. It never dereferences the source owner or
  reads LSE health. An old latched IRQ can be serviced after counting stops.
- `wait()` registers its waker and consumes the same retained event. With no
  event and no further counting it can remain pending indefinitely. Source
  loss itself produces no driver wake, and later polling does not turn the
  future into a health result. An independently waking valid timeout path is
  needed for a hard deadline; before/after health checks alone do not supply it.
- Cancelling `wait()` leaves counting and retention intact. `stop()` retains
  events. `start()` clears old hardware/software events and restarts through
  EN; `set_config()` also deliberately discards them. These local operations
  remain available when the source is observed unhealthy.
- `is_running()` reports EN, and `counter()` is a raw CNT sample. Neither
  proves liveness, useful counter progress or wall-clock passage.

The detector's documented edge window and sticky fault flags do not promise
automatic AWT fallback, EN clearing, a new AWT failure event or continued
underflows. No recovery, watchdog, executor or sleep guarantee is added.

## Drop, forgetting, reconstruction and RTC ownership

Normal Drop clears EN, masks local IER.UD and clears static active state in a
critical section. It does not gate/reset AWT, stop LSE/LSI, release oscillator
pads, clear source faults or disable/unpend NVIC. Ordinary field destruction
discards the held tokens without releasing that retained source hardware.

Safe `mem::forget` skips Drop. Counting and IRQs may continue; source ownership
fields are forgotten too. Static IRQ state contains no reference into the
forgotten object and retains an owned Waker clone, which can receive a stale
wake until replaced. With reborrowed tokens the original caller can later
regain token access. Token lifetimes therefore do not establish hardware
shutdown, and safe use does not require a promise never to forget an owner.

A successful reconstruction atomically stops/replaces local source,
configuration and event mode, clearing old retained events. Pending NVIC state
can still invoke the handler against the new local state. A pre-write failure,
including unhealthy admission, leaves a forgotten predecessor's state intact.
Reconstruction through an old HSIOSC constructor is permitted and still leaves
LSE and its other consumers running. There is no in-place source extraction or
conversion API. Reacquiring the LSE capability repeats the GPIOC window above;
sticky source faults remain, so this is not a fault-recovery procedure.

A live safe `Rtc` consumes its `CalendarClock`, including the same `LseClock`.
It cannot simultaneously lend an independent source owner to AWT through this
API. A dropped/forgotten RTC may remain a hardware LSE consumer; AWT
construction, configuration and Drop leave RTC registers/source and shared
oscillators untouched. Persistent LSE pad reservations survive token Drop and
forgetting and remain enforced by safe GPIO acquisition.

## Retained AWT recipients

On L031/R031/W031, GPIOA/B/C/F shared `FILTER.FLTCLK=7` selects standalone AWT
underflow. Per-pin filtering can survive GPIO/EXTI construction. Its effects
cover whole banks, including dormant, package-unbonded or internal bits.
Takeover, source/divider/ARR changes, start/stop, Drop or source loss can alter
or stall filtering and delay/suppress external edge delivery. Forgetting may
leave the producer driving those filters. Owning AWT, SYSCTRL and PC14/PC15
does not establish ownership or quiescence of those recipients.

Applications requiring preserved filter timing need a compatible functional
handover and a suitable retained producer. This is a functional limitation,
not an added all-recipient-quiescence constructor condition or Rust memory
safety obligation. No implicit bank scan, filter disable or EXTI change is
introduced; enabling a bank to inspect it can itself advance its recipients.

The own F020/x030 tables qualify FILTER values 0..6 only; the native FILTER7
definition is not imported there. ETR routes are AWT inputs. RTC AWTSRC,
AWTARR and AWTIMER belong to RTC's separate internal timer. No standalone
AWT-driven DMA/buffer recipient was found in the bounded own-manual review;
this is not a claim about undocumented silicon routes.

## Sources and verification boundary

[The new evidence record](lse-awt-evidence.json) binds five own manuals and
six own datasheets, exact package/electrical facts, hashes and page locators.
F030/A030 share x030 RM Rev2.5; F020 uses RM Rev1.4, L031 Rev1.6, R031 Rev1.3
and W031 Rev1.4. Physical PDF pages are one-based; printed labels are one less
for these manuals.

| Manual | Configuration / AWT gate | Timer / CR / flags | FILTER recipient |
| --- | --- | --- | --- |
| x030 | PDF56 / 84 | PDF168–169 / 173 / 174 | PDF158: no qualified value7 |
| F020 | PDF54 / 82 | PDF165–166 / 170 / 171 | PDF155: no qualified value7 |
| L031 | PDF55 / 79 | PDF160–161 / 165 / 166 | PDF142 / 150: AWT underflow |
| R031 | PDF57 / 81 | PDF162–163 / 167 / 168 | PDF144 / 152: AWT underflow |
| W031 | PDF56 / 80 | PDF161–162 / 166 / 167 | PDF143 / 151: AWT underflow |

[Historical timer evidence](low-power-timers-evidence.json), including its
older F020/F030/A030 `awt_v1` labels, remains byte-for-byte unchanged. The
current canonical split above and this new record supply current authority.

### Verification boundary

The [ordinary `lse_events` firmware](../examples/shared/low-power-timers/README.md)
first uses a real Blocking LSE owner through reborrowed tokens, polls for a
bounded iteration count and drops regardless of event arrival. It then
reacquires `LseClock` and constructs an Async owner with the typed AWT binding.
The initial bound is neither a deadline nor proof of any event; the async phase
can wait forever after source loss. Illustrative board declarations must be
replaced by actual qualified board facts. Compilation does not execute this
handover or establish electrical validity.

Stage80 main produced 11 ordinary ARM libraries (exact eight plus F002/F003
and generic F030 controls) and 8 real ELFs (six new-family `lse_events` links
plus unchanged F002 polling and F003 interrupt). The successful 11-library
prefix and resumed 8-ELF tail each exited zero with no compiler warnings.
Their 1,347-file source snapshots, modes/bytes and toolchain identities were
unchanged across the successful prefix and tail. The independent
[runtime review](lse-awt-runtime-review.json) and
[metadata/source review](lse-awt-metadata-review.json) accepted that frozen main
implementation.

The receipt sequence retains two earlier failures:

1. The first library attempt failed in the new build-time RCC assertion because
   it compared original JSON field spelling with sanitized Rust RCC references.
   It produced no library or ELF. The assertion was corrected to the actual
   reference spelling while retaining the same admitted hardware facts.
2. The corrected attempt completed all 11 libraries, then its first ELF stopped
   because the offline cache lacked locked `cordyceps 0.3.5`. An official
   `cargo fetch --locked` restored the dependency with an unchanged lockfile.
   Only the 8-ELF tail resumed; the successful library prefix was not rerun.

This is an accepted combination of matching successful receipts, not one
uninterrupted successful 19-row invocation. Earlier failures and the dependency
restoration remain in the external acceptance history.

Main used the same locked vendor cache through an absolute `CW32_SOURCES`
setting and one ordinary offline `./d gen-all`, which exited zero. Its external
comparison found no added, removed or changed files across all 905 generated
projection files, including all 54 chip records. Four separately invoked
finite gates also exited zero: current LSE AWT source/data, full locked-source
provenance, module layout and canonical data validation.

At main acceptance clean had not run. Final completion requires a separate
final-source receipt for F030C8T7 and L031C8T6: 2 libraries and 2 matching
ELFs, one independent ordinary generation using the same absolute source
cache, the whole54/905 comparison and the same four finite gates. Main
acceptance does not substitute for that clean receipt or publication. The
cumulative local recipe is unchanged and is not counted as execution.

These checks establish source correspondence and type/target/link
consistency. They cannot execute or prove source loss, cancellation races,
Drop/forget behavior, detector latency, cross-domain silicon behavior or board
clock accuracy. No HAL model, probe, runtime test, low-power wake or hardware
validation is claimed.
