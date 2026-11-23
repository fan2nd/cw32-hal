# Factory LSI system clock

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../../docs/w031-target-retirement.md).

This ordinary firmware selects `Sysclk::LSI` and exposes the resulting rate
bounds. Choose one exact part with `--no-default-features`:

- CW32F002F3P7, CW32F002F3U7, CW32F003F4P7, CW32F003F4U7 or CW32F003E4P7:
  system/bus bounds only. F002/F003 have no RTC and share the existing branch
  that uses no `LsiClock`, `CalendarClock` or calendar API. Generic F002/F003
  aliases are not qualified for this mode.
- CW32L010F8P6, CW32L010F8U6 or CW32L010Y8M6: live-entry reuse only.
  This branch declares 1.62–5.5 V and −40–85°C, leaves `Config.lse=None`, and
  calls ordinary `init`. It does not start/prepare/trim LSI or command IWDT.
  A normal-POR error is expected. Success exposes system/bus/HSI bounds and
  acquires the existing read-only `HsiOscClock` capability; it does not change
  RTC or invent a native `LsiClock`/LSI calendar variant. Each part retains
  64 KiB Flash and 4 KiB SRAM. See the [live-only contract](../../../docs/l010-live-factory-lsi-sysclk.md).
- CW32L011K8T6 or CW32L011K8U6: own live-entry reuse only, 64 KiB Flash and
  6 KiB SRAM. The branch declares 1.7–5.5 V and −40–85°C, leaves both
  `Config.hse=None` and `Config.lse=None`, and calls ordinary `init`.
  Original HSEEN/HSECCS/LSECCS must be zero; original software-requested,
  stable ten-bit factory LSI and HSI/LSI entry are required. Ordinary POR
  stops in the error handler. No LSI/request/IWDT preparation occurs. Success
  observes system/bus/HSI bounds and read-only raw 96 MHz `HsiOscClock`,
  without changing RTC. See the [own contract](../../../docs/l011-live-factory-lsi-sysclk.md).
- CW32L012C8T6 or CW32L012C8U6: own live-entry reuse only, 64 KiB Flash and
  8 KiB SRAM. The branch declares 1.7–5.5 V and −40–85°C and leaves
  `Config.lse=None`. Ordinary POR is expected to stop in the error handler.
  There is no LSI preparation, calibration, software-request write or IWDT
  operation before `init`. Successful initialization observes system/bus/HSI
  bounds and the existing read-only raw 96 MHz `HsiOscClock`; it does not
  initialize RTC. See the [own live-only contract](../../../docs/l012-live-factory-lsi-sysclk.md).
- CW32F020C6U7, CW32F030C8T7, CW32A030C8T7, CW32L031C8T6,
  CW32L031C8U6, CW32L031F8U6, CW32R031C8U6,
  CW32L052C8T6, CW32L052R8S6, CW32L052R8T6, CW32L083RBT6, CW32L083RCT6,
  CW32L083RCS6, CW32L083MCT6 or CW32L083VCT6: the existing calendar branch also
  acquires the same factory source through `LsiClock` and `CalendarClock::Lsi`,
  initializes an unset RTC and polls its date/time and source/divided bounds.
  Its demonstration epoch is 2026-10-09 00:00:00; replace it with the intended
  time. An already-running compatible calendar retains its date/time.

The default feature remains CW32F030C8T7. The firmware requires no GPIO wiring,
external oscillator, UART baud or other board peripherals. Support is init-only;
it does not provide runtime switching or sleep/wake recovery. Generic L031,
its other packages, generic R031/L052/L083 and generic L010/L011/L012
are excluded from LSI SYSCLK. The earlier Stage75 exclusion of every L011
describes that historical scope; its contract and receipts remain unchanged.

## Board declarations and rate bounds

The example declares VDD 1.65–5.5 V and ambient temperature −40–85°C on the
three L031, three exact L052 and five exact L083 parts. Exact L010 declares
1.62–5.5 V; exact L011 and L012 declare 1.7–5.5 V, all at −40–85°C.
R031C8U6 explicitly declares 2.2–3.6 V and −40–85°C;
the other existing example families retain −40–105°C. These are
source-qualified limits, not measurements or proof about a particular
board. Qualify the board across its actual complete operating envelope and
adjust the declarations accordingly before hardware use. Default AHB/APB
prescalers remain /1. F002/F003/L031/R031/L052/L083 retain factory HSIOSC /6, nominal 8 MHz, on
successful initialization; selecting LSI does not replace its independent
qualification.

The nominal LSI rate is 32,800 Hz. Factory full-range bounds are 31,160–34,440 Hz
for F002/F020 and 31,816–33,784 Hz for F003/F030/A030 and exact
L010/L031/R031/L052/L083. Exact L012 instead uses its own
29,520–36,080 Hz (±10%) range from DS CN V1.0 PDF58/printed55, Table7-19,
over 1.7–5.5 V and −40–85°C. Exact L011 uses 29,520–41,000 Hz, the own DS
CN V1.1 PDF51/printed48 factory −10/+25% envelope over the same PVT with
VDDA=VDD. The RM ±10% calibration-adjustment warning remains unresolved;
no new monitor or strict-cycle qualification follows. F003's bounds come from its
own datasheet Rev1.9, PDF pages 32 and 38: factory ±3% over the declared full
temperature range, independently of the other families. L031 uses its own DS
Rev1.9, PDF pages 38 and 47, with ±3% only over −40–85°C; the 25°C-only ±1%
row and conditional low-dissipation 105°C extension do not expand this envelope.
R031 uses its own DS CN V1.2 PDF54/printed53 table7-23 under PDF42/printed41
table7-4: ±3% over −40–85°C and 2.2–3.6 V; its 25°C-only ±1% row does not
expand that range. VDDA=VDD, VDDRF and all ground connections must satisfy
the actual board requirements; no firmware RF-power-state check supplies this proof.
L052 uses its own DS CN V1.3 PDF43/51 (printed42/50): ±3% over 1.65–5.5 V,
VDDA=VDD and −40–85°C. Its conditional +105°C operating row does not extend
this factory envelope. Its final configured HSI must also fit the voltage
ceiling with the final bus divisors.
L083 uses its own DS CN V1.9 PDF47/55 (printed46/54): the same numerical
factory-LSI envelope at 1.65–5.5 V, VDDA=VDD and −40–85°C, independently
qualified for its five exact parts. The +105°C operating row and adjustable
trim range do not extend factory tolerance. Configured HSI at final dividers
must fit 24 MHz below 1.8 V or 64 MHz at/above 1.8 V. Any requested external
source or dynamically relevant external fallback also requires raw 48.96 MHz
factory-HSI coverage without divider credit, hence VDD≥1.8 V. This example
requests neither external source. L083 retains Flash WAIT2 on success.
SYSCLK/HCLK/PCLK
bounds are rate-only; retained HSI keeps its own timing qualification. Exact
source bounds and divisors are retained through prescaling. For example, F003
AHB /128 and APB /8 would expose PCLK nominal/minimum/maximum 32/31/33 Hz while
retaining divisor 1024 internally; the example itself uses /1.

The calendar's nominal tick rate is 32800/32768 Hz, not a precision one-second
claim. Exact L031, CW32R031C8U6 and exact L052/L083 RTC LSI
aliases are rate-only under every SYSCLK, including HSI, HSE, LSE and L083 PLL.
Generic R031/L052/L083, other L031 packages and board-qualified LSE keep their prior
qualification. Strict cycle-duration helpers are not used. ADC timing
rejects these rate-only system clocks, and the fixed 1 MHz time driver rejects
selected LSI before singleton acquisition or RCC access. This crate enables
no time driver. Where present as a standalone peripheral, AWT retains its
independent HSIOSC timing qualification; L052/L083 instead have AUTOTRIM timer
mode and RTC-local wake logic. F003's
ATIM/IR presence does not grant new peripheral modes or timing guarantees.

## Initialization and failure effects

The L010 and L012 live-only branches require eligible original state and never
use cold admission. L012 preserves the continuous software LSI request and both
LSI_OUT routes (PB11 AF4 and PF3 AF2). Its shared ADC1/ADC2 work-gate inspection
may advance conversions/triggers before refusal; necessary HSE GPIOF windows
may affect PF3 output, filtering and events. Closed UART3, timer and output-bank
gates are not opened merely to prove idleness. Functional handover must permit
progress or disconnect affected recipients; restoration cannot undo events.
The independent pre-Rust bus-master/application-memory boundary still applies.

For the existing cold-capable branches, selecting the mode permits bounded whole-GPIOA/B/C inspection windows on
F002/F003, whole-GPIOA/B/C/F windows on the classic and exact L031/R031
parts, whole-GPIOA/B/C/D/F windows on exact L052, and whole-GPIOA/B/C/D/E/F
windows on exact L083. Sampling, filters
and armed events may advance, including before failure. Configuration and
locks are preserved; software does not clear flags, but flags can change
naturally. Restoring gates cannot undo this progress. Existing exclusive
clock/memory handover and a continuously legal execution clock remain required;
a critical section does not freeze hardware. System/bus clock changes also
change downstream timing, including F003 ATIM/IR, despite retained configuration.
A cold source with retained LSI-selected consumers or ready observers is
rejected; an already-running factory-matching source is reused without
TRIM/WAIT writes. No ready flag is cleared to obtain admission or after a new
start.

Exact L031/R031 additionally preserve configurable CCS/LSELOCK and inherited
HSE/LSE source/pad owners. Cold admission checks nine gates and eleven selectors,
including RTC and UART1/2/3 regardless of their local enable state. GPIOB FILTER
and PB11 AF share one inspection window. F8U6 has no bonded PB11; inspecting
its register is conservative policy. FILTER7 is documented AWT overflow and
is conservatively rejected; MCO7 is separately undocumented. Matching cold
trim does not bypass admission. The retained configured HSI tree must remain
legal with the final dividers and determines the conservative Flash wait along
with LSI. L031F8U6 still cannot request HSE.

Exact L052 uses a separate native snapshot and transition. Cold admission
checks RTC, AUTOTRIM, UART1/2/3, all five GPIO FILTER words, MCO, PC4 AF and
SYSCTRL ready/NVIC observers, even for matching cold trim. Off LCD/LPTIM
work gates stay off and their local controls are not read; gate-on cold LCD
requires EN=0 and BUMP=0 independently. PC4/AF6 is unbonded on C8T6 but
conservatively inspected without a pin token; both R8 packages bond it at pin24.
Live factory-matching LSI is never stopped or retrimmed. The whole-bank
handover includes unrelated pins, armed edge/level/event paths and external
loopbacks. No timer reset/stop, LCD pump write or IRQ/event clear manufactures
admission. Factory HSI, final divisors and Flash are complete before the final
source-only LSI switch. A failed attempt requires reset, or power reset where
POR-retained LSE controls require it; there is no rollback or safe retry claim.

Exact L083 independently checks six UARTs, six GPIO banks, RTC/AUTOTRIM/LVD,
MCO, full bonded LSI/PLL output routes and tagged LCD/LPTIM snapshots.
Off work gates remain unread and off. LSI routes are PC4 on every part,
plus PF2 on MCT6/VCT6 and PD5 on VCT6; unbonded native fields remain subject
to conservative AF0 guards without pin tokens. An inherited enabled PLL is
validated from actual reference/bins/parameters and output owners. Selected
PLL must first depart through proven HSI/HSE, then reach PLLEN0 and both
ready0 before any reference-affecting HSI change. Direct PLL-to-LSI is forbidden.
Permanent LSI readiness is retained through all later phases; after the final
source-only LSI commit, no source/divider/Flash/pad/policy writes are permitted.
Temporary inspection gates may still open and must restore before publication.
Failure may leave partial state and requires reset, including power reset for
POR-retained LSE controls. There is no rollback or source-loss recovery promise.

On R031, RFCLK has its independent dedicated 16 MHz oscillator. Selected LSI
can nevertheless affect RF host traffic through PCLK, the GPIOA working gate
and PA00..PA03. Finish/quiet host transfers and permit the whole-bank interval;
restoration or failure cannot promise unchanged RF signals or packets. Keep
inherited RF XTAL_OCLK available if it feeds HSE bypass. These are functional
limits of safe initialization, with no hidden Rust memory-safety precondition.
The initializer performs no RF register access, SPI command, RF reset, power
change, event clear or RF interrupt inspection.

An error stops at a debugger breakpoint. A failed hardware initialization can
leave attempted TRIM, an enabled inspection gate, conservative Flash/bus
guards or a permanent LSI request. HSI-calibration failure can leave execution
on the LSI bridge with HSI stopped or incompletely restarted. A partial HEX
configuration may also remain. RCC failure publishes no clocks or peripheral
tokens. No hardware error promises rollback; reset before retrying. Ready
status is a startup latch, not running clock-loss detection, and poll budgets
are not microseconds. Loss of the execution clock may prevent a return. Read
the complete [F002 contract](../../../docs/f002-factory-lsi-sysclk.md),
[F003 contract](../../../docs/f003-factory-lsi-sysclk.md),
[L031 contract](../../../docs/l031-factory-lsi-sysclk.md),
[R031 contract](../../../docs/qualified-r031-lsi-sysclk.md),
[W031 contract](../../../docs/qualified-w031-lsi-sysclk.md),
[L052 contract](../../../docs/l052-factory-lsi-sysclk.md),
[L083 contract](../../../docs/l083-factory-lsi-sysclk.md), or
[classic contract](../../../docs/factory-lsi-sysclk.md) before hardware use.

## Ordinary firmware builds

From the repository root:

```sh
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f002f3p7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f002f3u7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f003f4p7
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f003f4u7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f003e4p7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f020c6u7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f030c8t7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32a030c8t7,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l031c8t6,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l031c8u6,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l031f8u6,defmt
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32r031c8u6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l052c8t6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l052r8s6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l052r8t6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083rbt6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083rct6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083rcs6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083mct6,defmt --bin cw32-lsi-clock-example
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083vct6,defmt --bin cw32-lsi-clock-example
```

`build.rs` derives `memory.x` from the exact package's generated memory facts
and passes `-Tlink.x`. F002's own datasheet Rev1.2 tables 3-1 and 6-1 specify
16 KiB Flash at 0x00000000 and 2 KiB SRAM at 0x20000000 for both packages. F003's
own datasheet Rev1.9 PDF pages 5, 8, 27 and 62 specify 20 KiB Flash at
0x00000000 and 3 KiB SRAM at 0x20000000 for all three exact packages. The F4
packages also match their DFP device entries; E4P7 is supported by the datasheet
without a matching DFP device. L031's own DS Rev1.9 PDF pages 10, 32 and 77–78
specify 64 KiB Flash at 0x00000000 and 8 KiB SRAM at 0x20000000 for the three
exact packages. R031C8U6 uses its own DS CN V1.2 PDF11/35 and own PDSC
for the same exact 64 KiB Flash / 8 KiB SRAM at those bases. The three exact L052 packages use their own DS CN V1.3
package/memory facts for 64 KiB Flash / 8 KiB SRAM at those same addresses.
Own L083 DS CN V1.9 PDF10/86 specifies 128 KiB Flash for RBT6 and 256 KiB
for RCT6/RCS6/MCT6/VCT6, all with 24 KiB SRAM at those same bases.
The build script asserts each package's limits and RTC presence, while
preserving F002/F003's own limits and RTC absence.
Release uses size optimization, LTO and one codegen
unit. A Flash overflow must be resolved within the real capacity.

The `defmt` feature exercises HAL formatting implementations without requiring
a logging transport. These commands are a build recipe, not evidence that they
have run. Compilation and ELF inspection establish source/link integration
only; actual load segments, Flash/RAM use, remaining stack space, entry and
vectors must be inspected before claiming a binary fits. No hardware startup,
electrical rate, RTC operation, real-time behavior or runtime error-path
validation is claimed. Final exact-L052 main verification passed eight actual
library builds and eight ELF links with zero warnings, plus generation and six
finite Python source/data checks. At main acceptance, clean replay had not run;
final-package completion requires a separate final-source two-library/three-ELF
clean receipt. The full future recipe is not that execution record. See the
[runtime/source review](../../../docs/l052-factory-lsi-runtime-review.json) and
[metadata review](../../../docs/l052-factory-lsi-metadata-review.json).


For the exact-five L083 addition, main generation, seven finite source/data
commands, ten actual library builds, ten linked ELFs and independent implementation
reviews passed. At main acceptance clean had not run; final-package completion
requires a separate clean receipt for RCT6/VCT6 release libraries and
RCT6/MCT6/VCT6 release LSI ELFs. The
[L083 contract](../../../docs/l083-factory-lsi-sysclk.md) records this finite scope,
which differs from the accumulated future local recipe. Prior family results
are not relabeled as L083 acceptance or hardware evidence.

## Exact L010 live-entry build scope

Select exactly one of `cw32l010f8p6`, `cw32l010f8u6` or `cw32l010y8m6`:

```sh
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32l010f8p6,defmt --bin cw32-lsi-clock-example
```

A linked image checks compilation and exact linker limits only. It does not
prove normal-reset boot, successful live handover, watchdog survival, oscillator
continuity or electrical behavior. For the earlier exact L010 addition, main verification passed eight
actual library builds and seven ELF links, with six finite source/data/layout
gates and independent runtime/metadata reviews accepted. The main logs retain
three existing unused LSE-constant warnings in L010/L011 rows. At main acceptance
clean had not run; final-package completion requires a separate final-source
two-library/two-ELF clean receipt. Historical receipts above retain their own
source scope; the aggregate future recipe is not this stage's acceptance.


## Exact L012 live-entry build scope

Select exactly one of `cw32l012c8t6` or `cw32l012c8u6`; retain the existing
crate and binary `cw32-lsi-clock-example`:

```sh
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32l012c8t6 --bin cw32-lsi-clock-example
```

The exact linker regions are Flash 0x00000000/65,536 bytes and SRAM
0x20000000/8,192 bytes for both packages; no linker-memory enlargement is allowed.
Main verification passed seven actual libraries and six linked ELFs, including
both L012 LSI images, plus six finite source/data gates and independent runtime
and metadata/source reviews. At main acceptance clean replay had not run;
final-package completion requires a separate final-source two-library/two-LSI-ELF
clean receipt (L012C8T6 and retained L010F8P6). Board qualification remains
unverified. See the [exact matrix](../../../docs/l012-live-factory-lsi-sysclk.md#finite-verification-scope).
A successful link does not execute POR rejection or live handover. No hardware,
watchdog survival, oscillator continuity or elapsed-time guarantee is claimed.


## Exact L011 live-entry build scope

Select exactly one of `cw32l011k8t6` or `cw32l011k8u6` using the existing
crate and binary. This command is one of the verified main configurations:

```sh
cargo build --release --locked --manifest-path examples/shared/lsi-clock/Cargo.toml \
  --target thumbv6m-none-eabi --no-default-features \
  --features cw32l011k8u6,defmt --bin cw32-lsi-clock-example
```

Both exact linker regions are Flash 0x00000000/65,536 bytes and SRAM
0x20000000/6,144 bytes. Do not enlarge them to hide overflow. Retained RTC
SOURCE1 refuses after its actual configuration-gate window is restored;
SOURCE3 needs already factory-ready raw HSI and actual PSC1 raw ≥97. The
example neither supplies that entry nor alters RTC to obtain it. A link does
not execute these refusals or establish watchdog survival or continuity.

Final main verification passed seven actual libraries and seven linked ELFs,
all with exit zero; generation and seven finite data gates passed separately.
Three existing LSE-constant warning headers remain. At main acceptance clean
had not run; completion requires a separate final-source two-library/two-ELF
receipt using K8U6 and retained L010F8P6 libraries and their LSI ELFs. All exact
rows and source, rate-only, board and failure limits are in the
[public contract](../../../docs/l011-live-factory-lsi-sysclk.md#finite-verification-scope).
