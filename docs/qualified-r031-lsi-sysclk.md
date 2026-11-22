# Init-only factory LSI SYSCLK on CW32R031C8U6

Status: main implementation accepted in the separate
[runtime/source review](r031-factory-lsi-runtime-review.json) and
[metadata review](r031-factory-lsi-metadata-review.json). Package completion
additionally requires final-source clean replay and deterministic packaging,
tracked in a separate final addendum. The immutable main receipts record
clean replay as pending at main acceptance; a later clean result does not
rewrite those dispositions. This public contract summarizes the frozen R031
design (SHA-256
`458d3354e4f6bc610e58c4b6372124a5243b983ad43babe7f3d100da6f6eddf0`).
Historical L031, classic, F002/F003 and LSE reviews retain their own scope.
Source correspondence and builds do not establish hardware startup, measured
oscillator accuracy, RF continuity or recovery.

## Exact scope and source authority

Only **CW32R031C8U6, QFN48, 64 KiB Flash and 8 KiB SRAM** gains the existing
`rcc::Sysclk::LSI` selection. Flash is `[0x00000000, 0x00010000)` and SRAM is
`[0x20000000, 0x20002000)`. HSI remains the default. Generic R031 and every
W031 remained outside the original R031 qualification; exclusion made no claim
of a silicon blocker. That historical acceptance did not reconsider W031 RF
supply modes. CW32W031R8U6 now has a separate [own-source W031 contract](qualified-w031-lsi-sysclk.md);
generic W031 stays excluded. Existing qualified
L031/classic/F002/F003 paths retain their independent boundaries.

The authority is [evidence-sources.json](../sources/evidence-sources.json).
No source download, source re-pinning, PAC register-layout/access change,
new metadata schema, public configuration knob or RF driver is introduced.
Own source identities are:

| Own source | SHA-256 |
| --- | --- |
| [R031 RM CN V1.3](https://www.whxy.com/uploads/files/20240920/CW32R031_UserManual_CN_V1.3.pdf) | `fbee9b6942be9fa09f00c946705644d5356c4249f3e2280e3dfe5f0cb342eddb` |
| [R031 DS CN V1.2](https://www.whxy.com/uploads/files/20251230/CW32R031_DataSheet_CN_V1.2.pdf) | `88759314fa4cf8b6caf7098df4829489179e27aa3752a13de85ce955b29c5805` |
| [R031 SDK V1.1](https://www.whxy.com/uploads/files/20240115/CW32R031_StandardPeripheralLib_V1.1.zip) | `cec9df232d64b53b638c8a372f50fde1fd677f72c04bb3ae467b8138d8433082` |
| SDK member `cw32r031/IdeSupport/MDK/WHXY.CW32R031_DFP.1.0.2/SVD/CW32R031.svd` | `0d9273507521d7f63e7614e9ddd445a315ea3206689c86fad9f45b60a44e656a` |
| SDK member `cw32r031/IdeSupport/MDK/WHXY.CW32R031_DFP.1.0.2/WHXY.CW32R031_DFP.pdsc` | `efdd38b9ecf017f74d8ae86682588d772503fe0fe07b3af56964133327c35c9f` |

Page references below are one-based PDF/printed pages. DS11/10 table3-1 and
DS35/34 table6-1, together with the own PDSC, establish the exact package and
memory. The separate [qualification policy](../cw32-data/lsi-sysclk-qualified.yaml)
binds own facts; shared `cw32l031_v1` identity alone does not grant admission.
R031 validation covers its exact part, electrical envelope, 53 native fields,
access evidence, ten bases, nine gate/reset pairs, eleven selectors and
SYSCTRL IRQ4. RM93/92 calls the function RCC; the PAC's own interrupt name is
SYSCTRL, vector20 at 0x50. No RCC IRQ alias is created.

## Electrical and RTC compatibility contract

DS54/53 table7-23 under DS42/41 table7-4 qualifies factory LSI at nominal
**32,800 Hz**, **31,816–33,784 Hz**, **2.2–3.6 V** and **−40–85°C**. The
25°C-only ±1% row, RM ±10% adjustment range, duty-cycle and startup statements
do not establish an every-cycle or jitter bound. The RF 250 kbps −40–70°C row
does not redefine this MCU qualification. Board supply, VDDA=VDD, VDDRF and
all ground connections must satisfy the own board requirements; no RF-state
probe substitutes for that qualification. Neither voltage, temperature nor
frequency is measured by these APIs.

Exact rate numerators and integer divisors are preserved. SYSCLK/HCLK/PCLK
are rate-only. On this exact R031 part, `LsiClock::bounds`,
`CalendarClock::Lsi`, RTC source and divided calendar-tick bounds also become
rate-only under **every SYSCLK**, including HSI, HSE and LSE. This deliberate
compatibility change makes `has_cycle_timing_bounds()` false and retains the
strict duration helpers' existing refusal/assertion behavior. The nominal
calendar ratio remains **32800/32768 Hz**. There is no certified one-second
period, compensation, RTC migration or elapsed-time continuity guarantee.
Generic R031, every W031, excluded L031 and board-qualified LSE retained their
previous timing qualification in the original R031 change. The separate exact
W031 qualification now changes only that part's LSI aliases; it does not rewrite
the historical R031 review or alter R031's envelope.

Retained factory HSI is checked independently at the final AHB/APB dividers:
48 MHz ±2% and a 48 MHz bus ceiling throughout this R031 supply envelope.
HSI /1 with AHB /1 is refused because 48.96 MHz exceeds that ceiling, even
with slow final LSI. Default HSI /6 gives 7.84–8.16 MHz. The final Flash wait
covers the greater upper HCLK from selected LSI and configured retained HSI.
RM109/108 WAIT0/1/2 limits of 24/48/72 MHz do not grant a 72 MHz device clock.

Classic ADC retains its rate-only refusal before gate acquisition/init writes.
Existing PWM duration defenses remain; no complementary-PWM mode is added.
The fixed 1 MHz time driver rejects selected LSI with
`UnsupportedTimeDriverClock` during pure preflight, before `Peripherals::take`
and before every RCC MMIO access. Building the library with that feature does
not qualify LSI plus time-driver firmware or execute the rejection. UART,
SPI, I2C, timer and WWDT retain their existing frequency/divisor feasibility
checks. AWT HSIOSC and IWDT's independent RC10K retain their own source facts.

## Functional handover and RF boundary

The established [safe-init handover](qualified-l031-hse.md#pads-and-transition)
applies. Firmware enters after reset or a low-level handover that made bus
masters quiescent before Rust uses application memory. The incoming execution
clock must remain electrically legal and available. Clock/pad/consumer changes
and asynchronous loss of required sources are outside the bounded handover.
Normal interrupt masking does not stop NMI, DMA or autonomous peripherals.
These are visible functional limits of safe `init`, without a new
hidden Rust memory-safety precondition.

Own RM49/48 Figure4-1, RM498/497 Figure25-1 and DS10/9 Figure3-1 distinguish:

- External RFXC1/RFXC2 crystal → dedicated 16 MHz oscillator → RFCLK → RF.
  The RF synthesizer/PLL is separate from the four-source MCU SYSCLK mux.
- The RF oscillator → /1,/2,/4,/8 → RF XTAL_OCLK. RM533/532 RF_CAL[15:12]
  and DS14/13 describe its optional 16/8/4/2 MHz output and board connection
  to MCU HSE bypass. Its owner must retain power/configuration/availability
  when inherited HSE depends on it; initialization cannot verify or repair RF.
- MCU SYSCLK → AHB/APB → PCLK → GPIOA working gate → PA00..PA03 → RF host
  SPI. DS16/15 table4-4 maps MISO/MOSI/SCK/CS in that order. RF IRQ is a pad
  that may be board-connected to a GPIO.

There is no documented direct MCU LSI→RFCLK root. The indirect PCLK/GPIOA
host effect remains real. Finish/quiet RF host transfers and other
clock-sensitive work, and permit the complete whole-bank inspection interval.
Sampling, filtering and armed events may advance before success or failure.
Restoring the gate cannot undo this progress or promise unchanged RF signals,
uninterrupted packets, RF idleness or RF clock continuity. The initializer
performs no RF register read/write, SPI command, RF power change/reset,
RF/GPIO event clear, RF pin reconfiguration or RF interrupt inspection to
obtain admission. RF driver/protocol work stays deferred.

## Unchanged native admission and transaction

The shared native runtime and generated strict-window algorithm remain
unchanged. Only an explicitly requested `Sysclk::LSI` creates the target
transaction. Existing R031 HSI/HSE/LSE paths gain the RTC-alias compatibility
change above, without new selected-LSI restrictions.

The source uses the own native16 factory read at `0x00100A02` exactly once,
rejects raw `0xffff` before ten-bit masking, and accepts zero. Erased-data
refusal is conservative software policy, not a silicon failure diagnosis.
A stopped admitted source can change only TRIM; WAIT and reserved bits remain.
A live mismatch is never stopped/retrimmed. Cold entry requires no software
request or selection, both stable views clear, no detector owner, and clear
LSIRDY IER/ISR and SYSCTRL NVIC observer. Matching cold trim still needs two
complete passes and the final use-edge proof. Live matching sources may retain
active clients; they are not thereby proved idle or continuous.

Nine strict windows are ordered RTC, AWT, UART1/2/3 and GPIOA/B/C/F. They
inspect those nine source/filter selectors plus MCO and PB11 AF; PB11 shares
GPIOB's FILTER window. The accepted cold values are RTC 0/4/5/6/7,
AWT 0/2/3/4, UART 0/1/2, GPIO FILTER 0/1/2/3/4/6,
MCO 0/1/2/3/5/6/8/9 and PB11 AF 0/3/5/6/7. FILTER7 is documented AWT
overflow and is conservatively refused; MCO7 is undocumented. Local disables,
closed gates, START=0 and physical output inactivity do not replace selector
proof. IWDT RC10K, VC PCLK/RC150K and LVD HSIOSC/RC150K are separate roots;
MCU AUTOTRIM/LCD/LPTIM/system PLL/HEX absences exclude RF internals.

The central inspector saves, boundedly enables and restores each gate, with
readback even after failed enable. RTC/AWT/UART gates control configuration;
GPIO gates run the whole bank. Active-low asserted resets refuse admission
without being released. Once a window is attempted, enable/restoration errors
win, then restored gate identity, relevant external fault, post-window reset
and buffered semantic refusal. Restore failure cannot be hidden. No ready
flag, IRQ enable or pending state is cleared. Existing retained-HSE/RTC and
pad helpers retain their historical bounded contracts; not every old closure
has strict in-window reset checks.

The frozen P0–P9 sequence validates before ownership; captures immutable
entry/classification; installs Flash/bus guards; prepares LSI; permanently
requests HSI+LSI together before HSI escape; retains the existing guarded HSI
calibration bridge; handles requested/inherited HSE; handles optional LSE on
guarded HSI; installs final buses while explicitly on legal HSI; then checks
Flash and selects LSI last. Complete normalized expected source state advances
only by acknowledged owned writes. Cold ready events may rise only after the
owned request; captured ready is sticky. Inherited external enables, monitors,
pads and source parameters remain checked. Fresh optional LSE uses the
existing whole-register helper, with bounded target checks before/after it;
this does not invent a preserve-reserved guarantee or a callback at LSEEN.
No oscillator/source/divider write follows the final LSI selector write.

RCC errors publish no new clocks or peripheral tokens. Hardware may retain
Flash/bus guards, gate intervals, trim attempts or permanent LSI request.
A failed HSI bridge can leave LSI selected while HSI is stopped or not fully
restarted. Reset before retrying hardware initialization; no rollback,
CPU-progress, source-loss recovery or low-power restoration is promised.
The generic later time-driver error after RCC publication remains distinct
from the selected-LSI refusal in pure preflight.

## Example and finite verification evidence

The existing [LSI example](../examples/lsi-clock/README.md) adds the exact
feature, linker guard and RTC branch. R031 declarations explicitly use
2200–3600 mV and −40–85°C and require actual board qualification; prior example
envelopes are unchanged. The demonstration epoch/new-or-preserved calendar
flow is unchanged. Selected-LSI firmware enables no time driver.

Main verification completed exactly **six ordinary library builds**, retaining
six actual rlibs, and **seven real linked ELFs**. All thirteen commands passed
with zero warnings; the 1,291-file build-input snapshot stayed unchanged.
All use `--locked --target thumbv6m-none-eabi
--no-default-features`. Library commands use `cargo build --manifest-path
firmware/Cargo.toml -p embassy-cw32`; all are release except the explicit debug
row. ELF commands use `cargo build --release --manifest-path` with the listed
manifest, exact `--features` and explicit `--bin`.

| Library | Profile | Features |
| --- | --- | --- |
| L1 | release | `cw32r031c8u6,defmt,time-driver-gtim1` |
| L2 | debug | `cw32r031c8u6,defmt,time-driver-gtim1` |
| L3 | release | `cw32r031,defmt` |
| L4 | release | `cw32l031c8t6,defmt,time-driver-gtim1` |
| L5 | release | `cw32l031f8u6,defmt,time-driver-gtim1` |
| L6 | release | `cw32w031r8u6,defmt,time-driver-gtim1` |

| ELF | Manifest | Features | Binary |
| --- | --- | --- | --- |
| E1 | `examples/lsi-clock/Cargo.toml` | `cw32r031c8u6,defmt` | `cw32-lsi-clock-example` |
| E2 | `examples/rtc-calendar/Cargo.toml` | `cw32r031c8u6` | `preserve_calendar` |
| E3 | `examples/lse-clock/Cargo.toml` | `cw32r031c8u6` | `crystal_calendar` |
| E4 | `examples/lse-sysclk/Cargo.toml` | `cw32r031c8u6` | `crystal` |
| E5 | `examples/hse-clock/Cargo.toml` | `cw32r031c8u6` | `crystal` |
| E6 | `examples/lsi-clock/Cargo.toml` | `cw32l031c8t6,defmt` | `cw32-lsi-clock-example` |
| E7 | `examples/lse-sysclk/Cargo.toml` | `cw32w031r8u6` | `crystal` |

Package completion requires one clean final-source replay: reproduce generated
data/PAC first, then exactly **one release library and two ELFs**, L1/E1/E2
with the same options. At main acceptance this replay had not run. Its result
belongs to the separately tracked final-source addendum, not a revision of
the immutable main reviews. The broader [local script](../ci/check-lsi-clock.sh) has its own
future-recipe totals and is not the required acceptance run. Binary names
are not features; rtc-calendar, lse-clock and hse-clock have no defmt feature.

The own-source main review checks locked source/member provenance, all 53
fields/access, nine gates, ten bases, SYSCTRL IRQ4 and exact package/memory
facts. Comparing all 54 generated chip objects found only the intended
CW32R031C8U6 `lsi_sysclk` addition; the other 53 objects and every unrelated
R031 fact are unchanged. Generated register/access and PAC register/peripheral
bytes are unchanged; metadata-only output is recorded separately. Old-family
helper and shared-runtime preservation remain explicit review findings.

All six focused source/data checks passed: authored-data validation, PAC
inventory, RCC operating envelope, remaining RTC evidence, native L031-family
LSE data and locked-source provenance. This finite slice is not a full
`d test`, `d check` or `d audit-current` run. Actual main rlibs/ELFs, command
options and hashes are retained; clean artifacts must be retained independently.
Final deterministic packaging and clean-replay comparison remain separately
required for package completion.

Inspect entry/Thumb reset, vector/SP, PT_LOAD physical and virtual bounds,
section-to-segment mappings and all allocated sections, including initialized
RAM data's Flash load image. R031's true 64 KiB/8 KiB limits must hold without
link-region expansion. Corresponding main/clean loadable contents, entry and
load addresses must match; whole-file debug bytes need not. Vendor originals,
renders, generated data/PAC and compiler outputs stay outside distributable
source. No HAL test, compile snippet, probe, executable runtime model, fault
injection, network fetch or hardware execution is part of this finite slice.
