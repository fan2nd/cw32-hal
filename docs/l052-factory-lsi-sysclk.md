# Exact CW32L052 factory-LSI system clock

This contract covers init-only `Sysclk::LSI` on CW32L052C8T6 (LQFP48),
CW32L052R8S6 (LQFP64 7×7 mm) and CW32L052R8T6 (LQFP64 10×10 mm).
All three have 64 KiB Flash at 0x00000000 and 8 KiB SRAM at 0x20000000.
Generic CW32L052 and native L010/L011/L012 gain no LSI SYSCLK capability.
This L052 addition did not qualify L083; the later exact-five L083 capability
has its [own contract](l083-factory-lsi-sysclk.md). L052 has no system PLL. No RF, new driver, runtime clock
switching, sleep/wake restoration or hardware qualification is included.

Final main verification passed eight actual library builds and eight linked
ELFs with zero warnings and the 1,297-file build-input snapshot unchanged.
Generation and six finite Python source/data checks also passed. The
[runtime/source review](l052-factory-lsi-runtime-review.json) and
[metadata review](l052-factory-lsi-metadata-review.json) record their independent
main-source scope and dispositions. At main acceptance, clean replay had not
run; final-package completion requires a separate final-source
two-library/three-ELF clean receipt. This document does not assert that later
result. Older family results and the earlier L052 LSE-specific qualification
keep their historical scope. Their statements that they did not qualify LSI
remain valid. No software result is hardware qualification.

## Authority and rate provenance

The source authority is the selected own CN RM V1.5, CN DS V1.3 and SDK V1.4
in [the source lock](../sources/evidence-sources.json). The
[source mapping](../sources/SOURCES.md#exact-cw32l052-factory-lsi-sysclk)
records revisions, hashes and PDF/printed page locators. Manufacturer originals,
derived text and unapproved SDK members are external evidence and are not
redistributed here. SDK startup corroborates the factory address; it does not
prove that SDK `SystemInit` ran before Rust initialization.

The own factory envelope is nominal 32,800 Hz, minimum 31,816 Hz and maximum
33,784 Hz, at VDD 1.65–5.5 V, VDDA=VDD and ambient −40–85°C. The conditional
+105°C operating row does not extend the factory ±3% tolerance. The manual's
±10% tuning region is not substituted for that factory envelope. Board
conditions are declarations, not measurements. The source's establishment
time specification does not turn a polling-attempt budget into microseconds.

`ClockBounds::lsi()` carries rate-only provenance. SYSCLK/HCLK/PCLK preserve
the source bounds and exact integer divisors without inventing strict cycle
limits from rounded Hertz. Retained factory HSI has its independent bounds.
Pure `Config::frequencies` validates these facts without touching hardware or
implying that calibration was applied. It checks the selected LSI buses and
configured factory HSI at the requested final divisors, because the initializer
executes HSI at those divisors before committing LSI. L052 buses are limited
to 24 MHz below 1.8 V and 48 MHz at/above 1.8 V independently of Flash latency.
The own fixed CCS escape is HSI /6, at most 8.16 MHz before AHB/APB credit;
undivided factory HSI can reach 48.96 MHz and is not itself covered by the
48 MHz ceiling. Post-fault divider retention is not assumed.

All RTC LSI aliases on these exact three parts become rate-only under every
SYSCLK, including HSI, HSE and LSE. This is an intentional compatibility
change for `LsiClock::bounds`, `CalendarClock::Lsi`, `Rtc::source_clock_bounds`
and `calendar_tick_bounds`. Numerical RTC source facts and the exact nominal
32800/32768 calendar-tick fraction do not change. Board-qualified LSE keeps
its stored every-cycle provenance; native HSI-based RTC aliases on other
families keep their existing qualification. Existing exhaustive `Sysclk`
matches on the exact parts must handle the cfg-gated LSI variant.

Strict duration helpers retain their cycle-qualification assertions/refusals.
ADC's existing timing check refuses rate-only PCLK before its gate/reset or
ADC writes. Buffered complementary PWM calls its strict dead-time helper
before timer configuration, including a zero-dead-time request; this preserves
its existing assertion/panic semantics, not a new typed error. Earlier pin
wrapper activity is not excluded by that ordering. The fixed 1 MHz Embassy
time driver refuses the selected 32,800 Hz-derived clock through the existing
pure divisibility check before singleton acquisition or RCC writes. This is
not a blanket rejection of every possible rate-only time-driver source.

## Functional handover and observations

Use the existing early-initialization handover, before application peripherals,
DMA and interrupts rely on the transition. Incoming source, bus divisors and
Flash latency must already be legal for the actual board voltage. Software
cannot measure this, and CPU progress is required to poll or report a fault.
Other application HCLK/PCLK/HSI-derived clients and exported clock-dependent
signals must be quiescent through initialization.

Inspection can temporarily run each entire GPIOA/B/C/D/F bank through the
central RCC protocol. Sampling, configured filters and armed edge, level or
event paths may advance for every pin, including unrelated pins, before a
later refusal. The handover must permit the complete bank interval and any
external pin feedback. Restoring the gate cannot undo events or elapsed
hardware activity. This is an explicit peripheral-behavior contract within
the existing platform handover, not a new hidden Rust memory-safety condition.

RTC, AUTOTRIM and UART1/2/3 configuration gates are inspected and restored
without reset. LCD and LPTIM gates also control work: if off, they remain off
and local registers are not read. Their `Off` tags and gate/reset identity are
still compared. An on work gate is inspected without control writes. Cold
LCD requires EN=0 and BUMP=0 independently; cold LPTIM requires EN=0 and
completed ARST/SRST readback. Live factory-matching LSI can preserve active
scan/pump/timer controls because the source is not interrupted.

Full normalized snapshots retain RTC CR0/CR1/CR2/COMPEN, AUTOTRIM CR, each
UART CR2, work-on LCD CR0/CR1 and LPTIM CR/CFGR, all five GPIO FILTER words,
MCO and GPIOC AFRL. RTC WINDOW and LCD INTF are live status, excluded from
configuration equality and left untouched. UART TX/RX disable is not an idle
proof because timers, timeouts and auto-baud also consume its source. UART
START/TXBRK can progress and are not snapshot identities. Main admission
neither reads the RTC calendar nor opens its access window.

Cold policy excludes LSI-selected RTC, UART, AUTOTRIM, GPIO filters, MCO and
direct output, including parked selectors. Reserved encodings are rejected in
all entry classes. AUTOTRIM AUTO or enabled calibration is refused; enabled
timer mode requires a valid nonzero prescaler and preserved source. Its target
oscillator ownership is independent of the SRC selector. GPIO FLTCLK7 is
LPTIM PWM and is closed by the cold LPTIM work policy. There is no standalone
AWT: AUTOTRIM timer mode and the RTC-local wake timer are covered natively.

PC4/AF6 is the native direct LSI register route. It is unbonded on C8T6 and
physical pin24 on both R8 packages. Its field is conservatively checked on all
three without creating a C8T6 pin token. Cold PC4 AF0/1/2/3 are accepted;
AF4/5/7 and 8..15 are not qualified. Live factory-matching LSI may retain AF6.
Global MCO admission also closes internal timer MCO inputs without a GPIO
pad. PCLK-derived timers/ADC/SPI/I2C and GPIO/filter cascades are covered by
their source roots and functional handover; IWDT uses independent RC10K and
WWDT uses PCLK. External feedback remains a board responsibility.

No timer stop/start/reset, LCD pump manipulation, watchdog feed/start/reset,
IRQ mask/clear, ICR write, RTC unlock/calendar edit, RF access or peripheral
reset manufactures admission. Unrelated counters and flags may naturally
advance; they are not cleared or made reset-like.

## Factory admission and transition

The aligned HSI and LSI factory halfwords at 0x00100A00 and 0x00100A02 are
read once each before source mutation. Raw 0xffff is rejected before native
masking; zero remains a possible factory code. LSI uses native ten-bit TRIM;
WAIT and all other readable non-TRIM fields are preserved. A cold TRIM-only
write is permitted only after complete admission. Live LSI is never stopped
or retrimmed.

One immutable entry class distinguishes cold, live-ready and live-starting.
Cold requires request0, unselected LSI, both ready indications0, both external
detectors off, clear LSIRDY enable/status and no pending SYSCTRL IRQ4. Both
cold matching and mismatching trim take the full consumer proof. Live-ready
requires request1, coherent dual-ready and factory trim. Live-starting requires
request1, both ready0, unselected LSI, no detector and factory trim; readiness
cannot advance before the owned request phase. Selected-but-unready,
request0-but-ready, mismatching live trim and incoherent status all refuse.
Retained enabled external sources and their detectors must be coherent and
free of relevant sticky faults. STABLE is a startup latch, not proof of
continuing source validity.

The sequence keeps immutable entry identity separate from expected owned
deltas. Two complete source/consumer/source passes must agree; repeated
checks immediately precede cold trim and permanent request. Matching trim
skips only its write. It never substitutes for consumer admission.

1. Validate pure clocks, capture entry identity and both factory words, then
   collect and compare complete snapshots. Retain original request/parameter,
   IER, ready-event and gate/reset identities.
2. Set and verify conservative Flash WAIT2; install monotonic AHB/APB guards
   without replaying the captured source selector. Preserve slower divisors.
3. At the stopped-use edge, recheck the full proof, write only differing cold
   LSI TRIM and verify unchanged WAIT/non-TRIM fields. Recheck immediately
   before permanently requesting LSI. Preserve all other CR1 fields.
4. Wait for coherent dual-ready. An observed ready state cannot silently
   disappear or create a new permissive entry class. LSIEN stays requested.
5. If HSI trim differs, recheck retained AUTOTRIM/LVD HSI ownership, switch
   directly to factory LSI under guards, stop HSI with request and dual-ready
   acknowledgements, change HSI TRIM only, then restart and verify it. Never
   enable/select mismatching inherited HSI first. Matching trim skips only
   stop/trim/restart. Explicitly select factory HSI, then set its configured
   native divider. L052 HSI has no WAIT field.
6. Preserve enabled inherited HSE/LSE and their parameters/pads. Configure
   only an explicitly requested, genuinely cold auxiliary source under its
   complete target checks. HSE can intentionally retain the projected GPIOF
   pad gate and clear only owned FILTER bits. Expected state advances only
   by those known deltas. Cold auxiliary LSE retains its strict RTC reset-like
   contract in a target path with precise error handling; it does not weaken
   the main RTC-preservation contract or open off LCD/LPTIM gates.
7. Finish all sources and pads on guarded HSI. Set final divisors on explicitly
   selected factory HSI and recheck the tree. Set final Flash wait from the
   greater selected-LSI and configured-HSI upper HCLK, covering fixed CCS
   escape where relevant.
8. Recheck, modify SYSCLK to LSI only, acknowledge and barrier. No later
   source, oscillator, divider, Flash or detector-policy write follows.
   Final inspections restore their expected gates; normalized CR0 with final
   source and divisors is the last hardware observation before publication.

Only known owned fields may be old-or-target while their bounded write is in
flight. Non-owned bits retain exact identity; a third value refuses. External
faults and non-owned identity are checked before phase acknowledgements.
Inherited fallback is never normalized into a new entry or replayed source.
No frequency or successful clocks are published on failure.

## Errors and partial failure

Exact-target errors distinguish `InvalidEntryClock`, `InvalidLsiCalibration`,
`LsiCalibrationInUse`, `LsiClockInUse`, `LsiConfigurationTimeout`,
`LsiGateEnableTimeout` and `LsiGateRestoreTimeout`. Existing HSI calibration,
source readiness/switch, Flash, external-fault and retained-source errors keep
their documented meanings. `LsiTimeout` covers permanent-source readiness.

Central inspection resolves restoration failure first, then enable failure,
then independently observed original-gate mismatch, then relevant external
faults, then reset/identity and buffered semantic refusal. An ownership
predicate cannot hide failed restoration or a relevant observed source fault.
Work-gate changes are ownership refusal; no restoration is attempted on an
unowned LCD/LPTIM gate. Poll counts bound attempts, not elapsed startup time.

Failure can leave attempted trim, enabled requests, inspection gates, pads,
Flash guards or partial source state. HSI calibration can fail while running
on LSI with HSI stopped or incompletely restarted. There is no rollback or
safe retry promise: reset, and where POR-retained LSE controls require it,
power reset before another attempt. Loss of the execution clock may prevent
a return. No continued execution, automatic recovery, elapsed-time continuity
or low-power restoration is promised.

## Finite software acceptance and clean-package requirement

The [ordinary LSI example](../examples/shared/lsi-clock/README.md) extends its existing
RTC branch for exactly these three features, asserts 64 KiB/8 KiB and RTC
presence, retains `-Tlink.x`, and declares 1.65–5.5 V/−40–85°C. It keeps the
default HSI divider and enables no Embassy time driver. Lockfiles and
hosted CI are unchanged. The larger [local recipe](../ci/README.md) is future
reusable scope and must not be run to stand in for this finite acceptance.

The completed main library rows used `cargo build --locked --manifest-path
firmware/Cargo.toml -p embassy-cw32 --lib --target thumbv6m-none-eabi
--no-default-features`, release except the explicit debug row:

1. `cw32l052c8t6`, `cw32l052r8s6`, `cw32l052r8t6`: release, each with
   `defmt,time-driver-gtim1` (three).
2. `cw32l052c8t6`: debug with `defmt,time-driver-gtim1` (one).
3. Generic `cw32l052`: release with `defmt` only (one).
4. `cw32l083rct6`, `cw32l031c8u6`, `cw32f030c8t7`: release, each with
   `defmt,time-driver-gtim1` (three).

The eight completed main release/locked/thumbv6m ELFs used `--no-default-features`:

1. `examples/shared/lsi-clock`, bin `cw32-lsi-clock-example`: each exact L052
   feature with `defmt` (three).
2. `examples/shared/hse-clock`, `cw32l052c8t6`, bin `crystal` (one).
3. `examples/shared/lse-sysclk`, `cw32l052r8s6`, bin `crystal` (one).
4. `examples/shared/rtc-calendar`, `cw32l052r8t6`, bin `preserve_calendar` (one).
5. `examples/shared/pll-clock`, `cw32l083rct6`, bin `pll-uart` (one).
6. `examples/shared/lsi-clock`, `cw32l031c8u6`, bin `cw32-lsi-clock-example` (one).

One clean extraction of the final deterministic source ZIP repeats only the
C8T6/R8S6 release libraries and all three new LSI ELFs (two plus three).
Main rows ran serially in one task-owned shared target directory; each row's
actual artifact and generated `OUT_DIR` were copied before the next row started.
The clean replay has its own fresh source extraction, generated outputs and
independent target directory. Only verified external originals and the official
dependency download cache are shared; main generated outputs and targets are
not seeded into clean. Distinct per-row external verification records
bind exit status, command flags, observed artifact freshness, unchanged source
and the copied artifact/`OUT_DIR` identity. A shared-target Cargo result is not
a blanket claim that every dependency was rebuilt. At main acceptance the
clean replay had not run; its separate final-source receipt must establish the
actual two-library/three-ELF result. Inspect real entry/vectors, PT_LOAD ranges,
Flash/static RAM and stack headroom. A linked ELF proves compilation/linking only.

The first library attempt failed with E0308 because generated AUTOTRIM code
compared the typed PRS value with an integer (exit 101); a scoped emitter fix
used its native encoding. An earlier operating-envelope invocation failed
because its external evidence root was missing (exit 1), then passed after
that environment was corrected. Both failed attempts remain in the historical
records. The affected intermediate L052 builds each emitted five helper warnings;
the final same-sixteen-command rerun followed a scoped warning correction and
reported zero warnings. These attempts and superseded runs are not additional
successful final rows or tests.

Main completed generation plus the six finite Python source/data commands
below; the clean receipt must independently establish the same finite scope:
`./d gen-all`, `cw32-data/tools/validate.py`, `tests/validate_pac_inventory.py`,
`tests/verify_rcc_operating_envelope.py`, `tests/verify_rtc_remaining_evidence.py`,
`ci/verify-l052-lse-data.py` and `cw32-data/tools/source_provenance.py`, using the
same verified external locked evidence. No source fetch or discovery expansion
is included. The existing L052 LSE checker stays unchanged. Compare all 54
chip objects: historical 26 positive LSI projections become 29 only by adding
the three selected capability fields; all other facts remain identical.
Register/PAC APIs and lockfiles remain byte-identical. Account for metadata
include/deduplication churn semantically and compare complete main/clean
inventories. The final source ZIP must preserve receipt paths and executable
modes and repackage byte-identically after the clean run.

The independent metadata and runtime reviews bind the main authored source
and generated outputs. Final packaging must preserve both real review JSON
files under `docs/` and separately establish clean/main correspondence for the
final source ZIP. Main acceptance and later package completion remain distinct.
No HAL test, probe, model, aggregate validation, hardware timing or fault-recovery
execution was performed. The sixteen main Cargo results, generation and six
Python source/data checks are distinct evidence, not an aggregate test count.
