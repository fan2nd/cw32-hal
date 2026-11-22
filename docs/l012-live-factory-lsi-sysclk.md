# Exact CW32L012 live factory-LSI system clock

This init-only contract covers only CW32L012C8T6 (LQFP48) and CW32L012C8U6
(QFN48), each with 64 KiB Flash at 0x00000000 and 8 KiB SRAM at 0x20000000.
Generic CW32L012 and every CW32L011 remain excluded. The existing HSI/HSE/LSE
contracts, auxiliary-LSE qualification and native RTC identity stay unchanged.
This mode reuses an eligible original live source. Ordinary POR is refused.

Main verification passed seven actual libraries, six linked ELFs and six finite
source/data gates. Independent runtime and metadata/source reviews accepted the
main-source implementation and actual generated differences. At main acceptance
clean replay had not run; final-package completion requires a separate
final-source two-library/two-LSI-ELF clean receipt. Board qualification remains
unverified. No hardware execution, oscillator-continuity or watchdog-survival
result is claimed.

## Original admission and board conditions

`Config.sys=Sysclk::LSI` requires `Config.lse=None`. A new LSE declaration is
refused during pure validation, before evaluating its bounds or acquiring
singletons. Inherited LSE enable/lock/monitor/parameters/pads and existing RTC
owners are preserved; this target does not start LSE or create a new monitor.
An enabled inherited HSE still requires its exact declaration and ready,
unchanged parameters/pads. The incoming execution clock, buses, Flash latency
and retained sources must already be electrically legal and remain available.

Original entry must have software LSIEN=1, STABLE=1 and own factory-matching
nine-bit TRIM. The halfword at 0x001007C2 is read as 16 bits; raw 0xffff is erased
and fails before masking, while zero is valid. Preserve the complete original
LSI image except current STABLE, including RFU9 and WAIT[11:10]. Relevant
external faults precede admission; erased factory storage precedes original
TRIM mismatch, then absent software request, then absent original STABLE.
Later readiness never upgrades an ineligible original observation.

Neither the initializer nor the example writes SYSCTRL.LSI, explicitly sets or
clears software LSIEN, or performs an IWDT register/gate/reset/command/feed/pause
operation to obtain admission. Automatic-only requests and stopped startup are
refused. The own RCC manual's IWDT KR=0xCCCC automatic-LSI-request clause and
the IWDT chapter's separate RC10K counter source both remain facts. RUN/update
flags do not establish bounded request or release history.

Own DS CN1.0 Table 7-19 qualifies nominal 32,800 Hz and rate-only 29,520–36,080 Hz
at 1.7–5.5 V, VDDA=VDD and ambient −40..85°C. Board declarations are assertions,
not measurements; qualify the actual board throughout its complete envelope.
Factory matching does not qualify cycle duration, jitter or frequency accuracy
beyond these source bounds. Preserve rational source/divisor bounds internally.

Configured factory HSI defaults to /12: nominal 8 MHz, 7.84–8.16 MHz. Raw HSIOSC
is 96 MHz, 94.08–97.92 MHz; legal incoming HSIOSC 90–100 MHz is a separate fact.
Reset and effective fallback /24 give nominal 4 MHz, upper 4.08 MHz. Validate
configured HSI with final bus divisors and the full 4.08-MHz fallback without
divider-retention credit, independently of slow final LSI. Actual HCLK/PCLK
must fit 24 MHz below 1.8 V or 96 MHz at/above 1.8 V. HSI/1 with insufficient final
division remains invalid. There is no system PLL or new external fallback mode.

## Retained owners and functional handover

The supported runtime/boot integration establishes safe bus-master and
application-memory ownership before Rust starts. This is the existing platform
entry boundary, not a hidden memory-safety duty added to safe `init`.
Functional handover must permit the following progression or disconnect affected
recipients before initialization, including before refusal and after failure.

- RTC SOURCE3 owns raw HSIOSC even when START=0. Any needed HSI start/retrim or
  initially-unready wait is refused; actual PSC1+1 must be at least 98 for the
  97.92-MHz upper input and 1-MHz prescaler ceiling. SOURCE1 requires declared
  enabled HSE and its actual prescaler bound. SOURCE2 may retain unchanged live
  LSI. No RTC date/time/PSC/AWT/alarm/output/flag mutation or migration occurs.
- Independent HSI disturbance also refuses MCO3, enabled HSIRDY and I2C1/2
  master or slave raw CLKSRC1/3, irrespective of MEN/SEN. Factory-ready DIV-only
  adjustment preserves raw HSIOSC. The I2C master raw3 manual/SDK conflict is
  unresolved. PB0 AF3 HSIOSC_OUT and downstream recipients remain handover limits.
- ADC1/ADC2 share a configuration-and-work gate. Opening it to inspect EN can
  resume conversions or triggers before active-ADC refusal. LVD stays ungated;
  PCLK-dependent LVD/VC filters, any VC blank-trigger enable, OPA calibration
  and DAC trigger/DMA/wave owners retain their existing semantic vetoes.
- UART1/2/3 SOURCE3, enabled LPTIM ICLKSRC3, MCO4, RTC SOURCE2/AWT, LSIRDY,
  SYSCTRL IRQ4 and LSI-filtered GPIO/LVD/VC may retain the unchanged live root.
  EN with a zero analog filter count can still request it automatically. No new
  cold-requester scan or consumer stop/flag clear is used. Closed UART3, timer
  and output-bank gates stay closed when their only purpose would be inspection;
  their closure cannot prove inactivity or justify rejecting every live root.
- Both direct LSI_OUT routes are PB11 AF4/physical22 and PF3 AF2/physical44 on
  both packages. Complete existing Pin tuples carry both; legacy scalar
  `lsi_output_pin=Some("PB11")` is a nonexhaustive compatibility anchor. No pad
  is acquired by LSI SYSCLK itself. HSE uses PF0/PF1; necessary GPIOF intervals
  affect the entire bank and can resume/gate PF3 output, filters or events.
  Unbonded internal bank bits, external recipients and every documented native
  timer/cascade alternative remain covered by handover. L012's conflicting
  ATIM/BTIM input tables do not inherit L010's universal input9 assumption.

Gate restoration cannot undo sampling, trigger progress, pin pulses, capture or
brake flags, IRQ requests or watchdog countdown/reset. Keeping an LSI route does
not promise output continuity, fixed downstream rates or correct application
behavior across changed HCLK/PCLK. Critical sections do not freeze hardware.

## Guarded transition and failure

One immutable original snapshot precedes peripheral or FLASH gate writes.
Separate expected phase state covers only owned HSI/HSE, source/dividers and
Flash transitions. Full guards preserve live LSI request/parameters/readiness,
complete inherited LSE, IER/MCO and non-owned CR0/CR1/CR2 policy. Relevant HSE/LSE
faults are checked before and after collected images; neither FAIL nor FAULT is
cleared. Reads are non-atomic observations and do not prove analog continuity.

Every owned clock/source/divider/Flash write and each explicit new-target
acknowledgment iteration has applicable phase guards. Only named pending fields
may progress from prior to requested values; unexpected values and reversal
after acknowledgment fail immediately. Poll budgets count CPU iterations while
the CPU runs, not elapsed time. Loss of current LSI STABLE is an immediate error,
not a request to wait, restart, retrim or repair it.

Actual RTC, ADC, VC, OPA, DAC and conditionally needed I2C inspection windows
check their own active-low reset and operational state around register reads.
Each borrowed gate restores its incoming state independently; an originally
open gate is never deliberately closed. Detailed enable/restore failures precede
post-window source guards and buffered semantic refusals. HSE GPIOF windows use
the same error distinction. Full guards bound whole helper windows, not every
internal gate-poll or individual pad RMW. No reset pulse manufactures admission.

FLASH is deliberately enabled and stays enabled. Its reset/gate state is
checked around each actual access. The first authoritative FLASH.CR2 image is
captured only after access is valid; all non-WAIT bits, including FETCH/CACHE/
CACHEINVALID/RFU, remain original, normalizing only write-only KEY. Initial
FLASH WAIT and SYSCTRL FLASHWAIT must agree. WAIT3 protects the transition;
each paired acknowledgment permits only matching old or matching requested
values, rejects mixed/other pairs immediately and retains independent controls.

Final bus dividers and WAIT are established under verified factory HSI. Final
WAIT covers target-LSI HCLK, configured-HSI HCLK and the full 4.08-MHz fallback.
The last clock/source/divider/Flash mutation selects LSI. Afterward there is no
clock repair, rollback, source request, retuning, stale reselection or LSE helper
call. Required borrowed GPIO gate restoration and read-only final HSE pad and
complete source/policy/divider/Flash checks precede the sole clock publication.

The new exact-gated errors are `UnsupportedLsiLseConfiguration`,
`InvalidLsiCalibration`, `LsiNotReady`,
`LsiConfigurationGateTimeout { enable_failed, restore_failed }` and
`LsiClockSwitchTimeout`; existing source, owner and Flash diagnostics remain.
An invariant change wins over simultaneous LSI readiness loss. A still-valid
prior state may exhaust its specific timeout; unrelated identity changes fail
immediately. New live-LSI paths do not report LSE-target-specific errors.

RCC failure publishes no new clocks, consumes initialization ownership and
requires reset before retry. Partial HSI/HSE preparation, enabled gates, live
LSI, inherited LSE and pad reservations can remain. Source loss may leave
STABLE set, stop the CPU before an error returns or invalidate frozen timing.
No normal-reset boot, cold startup, runtime switching, sleep/resume, recovery,
watchdog survival or physical clock-continuity guarantee is made.

## Native RTC and strict timing

The existing shared RTC source identity remains raw HSIOSC3, 96 MHz nominal and
94.08–97.92 MHz with strict cycle bounds independent of SYSCLK or HSI.DIV.
`HsiOscClock` is unchanged. Exact parts retain `CalendarClock::HsiOsc` and held
`CalendarClock::Lse`; the no-LSE native `CalendarClock` alias remains
`HsiOscClock`. No native `LsiClock` or `CalendarClock::Lsi` is introduced.
The existing calendar profile remains first120/second400000, total96000000;
retained RTC checks use its actual PSC1, not this profile as an assumption.

Unchanged native sequence ADC and buffered complementary PWM reject missing
cycle qualification before their relevant peripheral timing mutations, including
zero-dead-time PWM. Prior pin wrappers/owners can have their existing effects;
no rollback is added. Ordinary rate-oriented timing gains no duration guarantee.
The fixed 1-MHz time driver rejects every 32,800/D nominal plan in pure preflight
before singleton acquisition or RCC MMIO. Its library build exercises compilation
only. Existing L010/L011 strict-consumer guards and other RCC backends are preserved.

## Example

The existing `examples/lsi-clock` crate/binary `cw32-lsi-clock-example` adds only
`cw32l012c8t6` and `cw32l012c8u6`. Its F030 default, previous features, electrical
values, calendar behavior and dependencies remain unchanged. The new branch
declares the own board envelope, selects LSI with no LSE declaration and calls
ordinary `init`; it never prepares LSI or commands IWDT. Ordinary POR is
expected to stop in its error handler. Success observes system/bus/HSI bounds
and acquires read-only `HsiOscClock`, without changing RTC. The build script
asserts exact 64-KiB/8-KiB regions and native raw 96-MHz RTC identity.

## Finite verification scope

All thirteen main rows below passed in the corrected main-v2 source. Every
library uses `cargo build --locked --manifest-path firmware/Cargo.toml -p
embassy-cw32 --lib --target thumbv6m-none-eabi --no-default-features`. Release
adds `--release`; debug does not. Exact features are:

| Main library | Profile | Features |
| --- | --- | --- |
| 1 | release | cw32l012c8t6,rt,defmt |
| 2 | release | cw32l012c8u6,rt,defmt |
| 3 | debug | cw32l012c8t6,rt,defmt,time-driver-gtim1 |
| 4 | release | cw32l012,rt,defmt |
| 5 | release | cw32l010f8p6,rt,defmt |
| 6 | release | cw32l011k8t6,rt,defmt |
| 7 | release | cw32f030c8t7,rt,defmt |

Each ELF uses `cargo build --release --locked --target thumbv6m-none-eabi
--no-default-features` with the following manifest, binary and feature:

| Main ELF | Manifest | Binary | Feature |
| --- | --- | --- | --- |
| 1 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l012c8t6 |
| 2 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l012c8u6 |
| 3 | examples/lse-sysclk/Cargo.toml | crystal | cw32l012c8t6 |
| 4 | examples/rtc-calendar/Cargo.toml | preserve_calendar | cw32l012c8t6 |
| 5 | examples/hse-clock/Cargo.toml | crystal | cw32l012c8u6 |
| 6 | examples/lsi-clock/Cargo.toml | cw32-lsi-clock-example | cw32l010f8p6 |

The independent final-source clean ceiling is exactly library rows 1/5 and ELF
rows 1/6, in a clean source/target after fresh generation. Capture actual command
statuses, warnings, complete source hashes, fresh artifacts, generated outputs
and ELF memory limits. Do not reuse prior outputs or replace a build with
`cargo check`. No accumulated aggregate recipe supplies these finite results.

The six finite source/data gates passed: own L012/L010/L011 checks, current data
validation, all 43 locked hardware-original provenance and module layout. Full
54-object and 904-file comparison found only two new LSI records and four appended
Pin tuples, in the two exact L012 chip JSONs and shared
`cw32-metapac/src/chips/metadata_0021.rs`. All other 901 files and 52 chip objects
were identical; no files were added or removed. Removing precisely the allowed
fields/tuples restored complete-object equality, including historical
LSE/RTC/electrical facts. Independent runtime and metadata/source reviews accepted
this main-source scope. No HAL tests, probes, register models, hosted CI or
hardware execution were performed for this acceptance.

The first main-v1 library failed because new build-time IR block assertions used
raw `SYSCTRL`/`GPIO` names. The corrected predicates use the existing sanitized
`Sysctrl`/`Gpio` names; this correction changed no PAC or source-data facts.
Earlier generation stopped on missing printed-page citation companions; the
reviewed correction added printed-page companions to 39 policy and 8
qualification source entries without changing hardware facts. These failures remain part of the verification history.
Main-v2 L012 rows emitted no warnings. The retained L010 library, L011 library and
L010 LSI ELF each retained the three unused-constant warning headers for
`RCC_LSE_SYSCLK_LSE_EDGES`, `RCC_LSE_SYSCLK_LSI_CYCLES` and
`RCC_LSE_SYSCLK_MARGIN_LSE_EDGES`; the matrix is not warning-free.

## Own source authority

The existing [source catalog](../sources/SOURCES.md) and
[locked evidence registry](../sources/evidence-sources.json) retain vendor inputs
outside the public source tree. One-based physical PDF/printed locators use the
own editions, not L010 page mappings:

- CN RM1.4, SHA-256 `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`:
  58/32 running-source restriction;59/33 calibration;72/46 IWDT automatic LSI
  request;75/49 own LSI fields;453/427 RC10K counter source.
- EN RM1.0, SHA-256 `f56d5ed899dd090b469fac6a09084aab9f5068ae3b56cf7562b83997bd2f1088`:
  60/34 running restriction/selectors;63/37 factory source;76/50 request;
  77/51 and135/109 Flash alias/table;79/53 LSI;88/62 UART3 gate;
  91/65 active-low Flash reset;495/469 RC10K;626/600 and636/610 I2C conflict.
- CN DS1.0, SHA-256 `08a605fe8843d1eefa2b1115474b5a74df2833068f0ad6f35225e4f39377da76`:
  39/36 and40/37 output routes;47/44 supply/bus limits;58/55 factory envelope.
- Own SDK1.0.5 selected SVD/header/PDSC/SYSCTRL/IWDT and other bound members
  corroborate native identities. SDK initialization is not executed startup
  evidence and does not resolve manual conflicts. Raw write-only KEY and
  read-only STABLE facts stay separate from operational semantics.

The frozen design/evidence and later real verification outputs are external
artifacts; they do not embed vendor manuals, compiler products or external
executor paths in the distributed source. Historical LSE proof records keep
their original meaning and are not relabeled as live-LSI acceptance.
