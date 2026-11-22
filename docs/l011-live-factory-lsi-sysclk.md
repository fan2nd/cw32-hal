# Exact CW32L011 live factory-LSI system clock

This initialization-only contract covers CW32L011K8T6 (LQFP32) and
CW32L011K8U6 (QFN32), each with 64 KiB Flash at 0x00000000 and 6 KiB SRAM at
0x20000000. Generic L011 gains no LSI SYSCLK capability. The separate earlier
L010/L012 and L011 HSI/HSE/LSE contracts retain their scope. In particular,
the Stage75 L012 contract's exclusion of all L011 describes its historical
acceptance point; this addition does not rewrite its evidence.

Final main verification passed seven actual HAL library builds and seven linked
ELFs, all with exit status zero. Generation and seven finite data gates passed
in separately recorded invocations. The matrix retains three existing unused
LSE-constant warning headers; five newly introduced warning diagnostics were
removed before the final main run. At main acceptance clean had not run;
completion requires a separate final-source two-library/two-ELF receipt.
These software results do not establish hardware behavior or board qualification.

## Original entry and source bounds

`Config.sys=Sysclk::LSI` requires `Config.hse=None` and `Config.lse=None`.
These exclusions are pure validation before singletons or RCC MMIO. Original
HSEEN, HSECCS and LSECCS must be zero; original SYSCLK must be HSI or LSI.
No new HSE configuration, HSE GPIO inspection or LSE monitor is admitted.
Inherited unmonitored LSE enable/lock/parameters/readiness and pad reservations
remain protected. CLKCCS is preserved; it does not enable either detector.

The immutable original state must already contain software LSIEN=1, current
LSI.STABLE=1 and ten-bit TRIM equal to the own factory halfword at 0x001007C2.
Reject raw 0xffff before masking; zero is valid. Preserve the entire non-STABLE
LSI word, including WAIT and reserved bits. Later readiness cannot make an
originally ineligible entry eligible. There is no LSI parameter, software
LSIEN, watchdog gate/reset/read/feed/command or cold-start operation to create
admission. Ordinary POR and automatic-only requests are refused.

Own DS CN1.1 Table 7-19 qualifies nominal 32,800 Hz and rate-only
29,520–41,000 Hz (−10/+25%) at 1.7–5.5 V, VDDA=VDD and −40–85°C.
The RM ±10% calibration-adjustment warning and conflicting trim-step/range
statements remain unresolved; the DS factory envelope does not withdraw
them, clip the maximum to 36,080 Hz, or prove every arbitrary trim or analog
operating point legal. Actual incoming electrical operation and the board's
complete declared envelope require qualification. These bounds do not grant
cycle duration, jitter, output continuity or strict downstream timing.

Configured factory HSI is independently validated under final bus dividers.
Raw HSIOSC retains 96 MHz nominal and 94.08–97.92 MHz bounds. The default and
effective fallback /24 give 3.92–4.08 MHz; validate the full 4.08 MHz without
retained AHB/APB-divider credit. This conservative budget does not assert that
LSI failure invokes external-clock fallback or that hardware preserves or
rewrites any divider, enable or selector. Actual buses retain their own
24 MHz below 1.8 V or 96 MHz at/above 1.8 V limits.

## Retained owners and functional handover

Actual RTC, ADC and VC configuration windows check their gate and active-low
reset state around register observations. The existing bounded borrowing
protocol restores an originally closed gate before interpreting buffered
semantics, attempts restoration even after enable-readback failure, and never
deliberately closes an originally open gate. Enable/restore failure takes
priority over buffered faults or semantic refusals. Successful restoration
cannot undo functional progress during the window.

- RTC SOURCE1 (HSE) always refuses after its real restored window, even with
  calendar START=0 or AWT disabled. SOURCE4..7 have no documented assignment
  and refuse. SOURCE0 retains its independent legal LSE controls; no new
  declaration or source qualification is invented.
- RTC SOURCE3 (raw HSIOSC) requires originally enabled/stable factory-correct
  HSI with no start/retrim. Actual eight-bit PSC1 raw must be at least 97:
  97.92 MHz ≤1 MHz ×(raw+1), hence divisor≥98. Widen before adding one so
  raw 255 gives 256. HSI DIV, SYSCLK and profile divisor 120 are not substitutes
  for the real retained register. RTC WAIT2 is date/time/AWTARR load status,
  not oscillator readiness or source ownership.
- RTC SOURCE2 retains the live LSI root. The 41,000 Hz upper bound satisfies the 1-MHz
  prescaler ceiling for every valid divisor 1..256. No source/PSC/calendar/AWT/
  alarm/output/flag write or migration occurs. This ceiling check proves
  neither 2-Hz calendar ticks, wall-clock accuracy nor AWT duration.
- Active ADC, SYSCLK-dependent LVD filter, PCLK-dependent VC filter and any
  enabled VC blanking route retain their concrete vetoes. Necessary windows
  may advance peripheral work before a refusal; no reset creates idleness.
- UART1/2/3 SOURCE3, LPTIM ICLKSRC3, RTC SOURCE2/AWT, MCO4, LSI filters,
  ready/IRQ and native timer/cascade observers can retain the unchanged root.
  No new UART/LPTIM/GPIO/IWDT work gate opens merely to inspect it. There is
  no direct bonded LSI_OUT pad; MCO and internal timer observations remain
  distinct. PB0 AF3 raw-HSI output and downstream recipients retain the
  disclosed HSI-preparation handover limit.

The existing platform/runtime entry establishes bus-master and application
memory ownership before Rust. This adds no hidden memory-safety requirement
to safe `init`. Functional handover must permit affected progress
or disconnect recipients. Critical sections do not freeze hardware; unchanged
LSI does not promise UART baud correctness, filter duration, unchanged outputs
or watchdog survival. L011 IWDT is LSI-clocked; no L012 RC10K fact applies.

## Guarded transition and failure

One immutable source snapshot precedes configuration gates. Separate expected
phase state advances only acknowledged owned fields. Preserve every non-owned
CR0/CR1/CR2 bit, IER/MCO, inherited HSE word and LSE identity; normalize only
write-only KEY and explicitly owned pending fields. Preserve CR2 bit7 despite
the RM RFU versus CMSIS/SVD RTCLPM naming conflict. Check relevant external
faults before and after collected observations without clearing flags. Whole
ISR equality is inappropriate; historical LSIRDY is not current readiness.

FLASH is deliberately enabled and stays enabled. Prove its actual gate and
reset state before and after every access. Capture its authoritative non-WAIT/
KEY word only at the first valid access without replacing the original source
snapshot. Initial WAIT0..3 must match the original SYSCTRL FLASHWAIT mirror.
Preserve all other Flash bits, including reserved fields; L011 has no imported
L012 FETCH/CACHE controls. During an owned WAIT transition accept coherent
old/old or target/target pairs, never mixed pairs or reversal.

Guarded HSI preparation may use the existing stopped-HSI retrim bridge, but
never prepares LSI. Both HSI→live LSI and already-LSI→prepared HSI→same live
LSI use the full sequence. Final buses and Flash WAIT are established under
factory-correct HSI, covering target LSI, configured HSI and full 4.08-MHz
fallback. The final source-only LSI selection is the last clock/divider/Flash
mutation. Only complete read-only checks and barriers follow; no GPIO gate,
HSE matcher, LSE helper, repair, rollback or reselection occurs before the
single publication of frozen clocks.

`UnsupportedLsiHseConfiguration` and `UnsupportedLsiLseConfiguration` report
pure requested-source exclusions. Hardware refusals use `InvalidLsiCalibration`,
`LsiNotReady`, `LsiClockSwitchTimeout` and existing entry/owner/Flash errors.
`LsiConfigurationGateTimeout { enable_failed, restore_failed }` preserves
both gate diagnostics. `RtcClockInUse` covers SOURCE1, SOURCE3 disturbance or
insufficient actual PSC1; `RetainedRtcConfiguration` covers SOURCE4..7.
Unexpected non-owned drift is never accepted as a new phase baseline.

Hardware refusal consumes initialization ownership and requires reset before
retry, even if no clock-control write occurred. Failure publishes no new RCC
clocks; partial HSI setup, enabled gates, live LSI and inherited LSE reservations
may remain. Poll budgets count running CPU iterations, not elapsed time.
STABLE can remain set after running loss, and clock loss may stop the CPU
before an error returns. No cold/POR startup, runtime switching, sleep/resume,
source-loss recovery, oscillator continuity or watchdog survival is promised.

## RTC identity, strict consumers and ordinary example

Native RTC remains SOURCE3/raw HSIOSC, nominal 96 MHz and 94.08–97.92 MHz, with
profile divisors 120/400000. `RTC::SOURCE_IS_LSI` stays false under every SYSCLK;
`HsiOscClock` and existing calendar aliases retain their independent strict
source bounds. No native `LsiClock` or `CalendarClock::Lsi` is introduced.

LSI-derived bus bounds retain rate-only qualification through exact rational
divisors and outward-rounded public getters. Strict duration helpers remain
unavailable. Existing sequence ADC and buffered complementary PWM reject
missing cycle qualification, including zero dead time. Ordinary rate-based
timers are not universally prohibited. The fixed 1-MHz time driver rejects
every 32,800/D plan in pure validation before singleton acquisition/MMIO.

The existing `examples/lsi-clock` crate and `cw32-lsi-clock-example` binary
add only the two exact L011 features. Their linker limits remain exactly
65,536-byte Flash and 6,144-byte SRAM. The branch declares the own envelope,
leaves both external-source configurations absent, calls ordinary `init`
and observes clocks/read-only `HsiOscClock` on success. It never prepares entry,
commands IWDT or changes RTC. Ordinary POR follows its error handler.

## Finite verification scope

All fourteen main rows below passed with exit status zero. The 1,334 source
files and 905 generated files remained unchanged during the final matrix.
Libraries use `cargo build --locked
--manifest-path firmware/Cargo.toml -p embassy-cw32 --lib
--target thumbv6m-none-eabi --no-default-features`; release adds `--release`.

| Row | Profile | Features |
| --- | --- | --- |
| M-L1 | release | cw32l011k8t6,rt,defmt |
| M-L2 | release | cw32l011k8u6,rt,defmt |
| M-L3 | debug | cw32l011k8t6,rt,defmt,time-driver-gtim1 |
| M-L4 | release | cw32l011,rt,defmt |
| M-L5 | release | cw32l010f8p6,rt,defmt |
| M-L6 | release | cw32l012c8t6,rt,defmt |
| M-L7 | release | cw32f030c8t7,rt,defmt |

All ELFs use `cargo build --release --locked --target thumbv6m-none-eabi
--no-default-features` plus these manifest, binary and feature arguments:

| Row | Manifest | Binary | Features |
| --- | --- | --- | --- |
| M-E1 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l011k8t6,defmt |
| M-E2 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l011k8u6,defmt |
| M-E3 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l010f8p6,defmt |
| M-E4 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l012c8t6,defmt |
| M-E5 | examples/hse-clock/Cargo.toml | crystal | cw32l011k8t6 |
| M-E6 | examples/lse-sysclk/Cargo.toml | crystal | cw32l011k8t6 |
| M-E7 | examples/rtc-calendar/Cargo.toml | preserve_calendar | cw32l011k8t6 |

Independent final-source clean is exactly M-L2/M-L5/M-E2/M-E3: two libraries
plus two ELFs after fresh generation, with no copied generated or target
artifacts. Record actual commands, exit codes, warnings, final source hashes,
fresh artifact hashes and ELF memory limits. Compilation does not exercise
runtime refusals, retained-client timing or the hardware transition.

One `./d gen-all` and these seven finite data gates passed in separate
invocations; no aggregate-script execution supplies those results:

1. `ci/verify-l011-lse-data.py`, including the independent own live-LSI section
2. `ci/verify-l010-lse-data.py`
3. `ci/verify-l012-lse-data.py`
4. `tests/audit_generated_parity.py`
5. `cw32-data/tools/source_provenance.py`
6. `cw32-data/tools/validate.py`
7. `tests/test_module_layout.py`

The permanent checker covers all 54 current selections, exact-two admission
and generic absence from current own sources. The one-time external comparison
passed across all 905 generated files: exactly the two L011 chip JSONs and
shared `cw32-metapac/src/chips/metadata_0020.rs` changed; the other 902 files
were unchanged, with no additions or deletions. Removing only the two new
`lsi_sysclk` objects restored complete equality of all 54 chip objects. Pin
arrays, register/RTC/LSE values, memory, features and transport models stayed
unchanged. No historical generated hashes become checker dependencies.

The retained warning headers name `RCC_LSE_SYSCLK_LSE_EDGES`,
`RCC_LSE_SYSCLK_LSI_CYCLES` and `RCC_LSE_SYSCLK_MARGIN_LSE_EDGES`; this matrix
is not warning-free. The external history retains three earlier L011 checker
preparation failures involving CMSIS trailing commas, the SVD's dual IRQ
entries and PDSC memory identity, plus the first main attempt's diagnostic
collection defect and subsequent warning correction. Those attempts are not
added to the successful final main count. Main-source and later final-source
identities remain separate; these status updates are not retroactive build inputs.

No aggregate recipe, HAL test, probe, register model or hosted CI supplied
these finite results. The [independent runtime/source review](l011-live-factory-lsi-runtime-review.json)
and [metadata review](l011-live-factory-lsi-metadata-review.json) record their
own scope and dispositions. At main acceptance clean had not run; completion
requires a separate final-source two-library/two-ELF receipt. Neither compilation
nor data equality establishes the electrical transition.

## Own source authority

See the [source reading index](../sources/SOURCES.md#exact-cw32l011-live-factory-lsi-sysclk),
[locked evidence authority](../sources/evidence-sources.json) and
[qualification record](l011-live-factory-lsi-qualification.json). The selected
own editions are RM CN1.1 (June 2026 cover), DS CN1.1 (July 2025 cover), SDK1.0.3
and its selected SVD/CMSIS members. Vendor originals stay outside the distributed
source tree; no startup/example C execution is inferred for Rust.

DS PDF 51/printed 48 supports the factory envelope; RM PDF 54/printed 53 retains
the calibration warning. DS revision-history physical PDF 72 is unchanged while
its printed locator changes from 1 to 69, with both values and rendered evidence
retained in the new proof. Other authority edits are evidence-reference
appends only. Earlier source identities, licenses, member chains and historical
LSE proof booleans remain unchanged. Earlier acceptance evidence and counts
retain their own scope; none constitutes acceptance of this addition.
