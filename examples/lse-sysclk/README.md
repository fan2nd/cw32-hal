# LSE SYSCLK and the same oscillator for RTC

Current support: CW32W031 / CW32W031R8U6 have been retired. Historical W031 qualification and verification records below do not describe supported targets. No L031 feature alias or pin/electrical replacement is provided; see [target retirement](../../docs/w031-target-retirement.md).

Two ordinary bare-metal firmware examples for CW32F020C6U7, CW32F030C8T7,
CW32A030C8T7, CW32L031C8T6, CW32L031C8U6, CW32L031F8U6, CW32R031C8U6,
CW32L052C8T6, CW32L052R8S6, CW32L052R8T6, CW32L083RBT6,
CW32L083RCT6, CW32L083RCS6, CW32L083MCT6, CW32L083VCT6, CW32L010F8P6,
CW32L010F8U6, CW32L010Y8M6, CW32L011K8T6, CW32L011K8U6, CW32L012C8T6 and
CW32L012C8U6: twenty-two exact packages in total. Family aliases and other
packages are excluded. Build with one exact feature, for example:

```
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f020c6u7,defmt --bin crystal
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32f030c8t7,defmt --bin bypass
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l052r8s6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l083rbt6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l010f8p6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l010f8u6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l010y8m6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l011k8t6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l011k8u6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l012c8t6,defmt --bins
cargo build --release --locked --manifest-path examples/lse-sysclk/Cargo.toml --target thumbv6m-none-eabi --no-default-features --features cw32l012c8u6,defmt --bins
```

These are board declarations, not measured qualification. Both examples assert
VDD 3.0–3.6 V, ambient −20…70 °C and nominal 32768 Hz with every-cycle frequency
between 32766 and 32770 Hz over supply, load, temperature, aging and short-term
variation. Replace these with the actual qualified board data. On the retained
fifteen packages, HSI /6 and AHB/APB /1 retain the existing defaults; HSI and
factory detector LSI remain enabled. Their original configuration is unchanged.
The three native L010 packages use HSI /12 and the distinct policy below.
The two native L011 packages use HSI /24 and their own monitoring and handover
contract below. The two native L012 packages retain their own configured HSI /12
and handover contract. All twenty retained pre-L012 configurations and pin selections
are unchanged.

On the fifteen retained packages, `crystal` requires the board's 32768 Hz crystal, load capacitors and layout on
PC14/PC15. Their physical positions are 3/4 on the classic and L031 48-pin
packages, 1/2 on L031 QFN20, 2/3 on R031 QFN48. On
all three L052 packages, including both 64-pin R8 packages, PC14/PC15 are
physical pins 3/4. The own L083 package metadata places PC14/PC15 at pins 3/4
on RBT6/RCT6/RCS6/MCT6 and 8/9 on VCT6. The actual crystal and board must
qualify Strong drive, Normal amplitude and the selected startup
count. L052 additionally declares independent startup drive Normal and startup
amplitude Large, deliberately distinct from run drive Strong and run amplitude
Normal. Both pairs and the 16384-cycle startup count require board-specific
qualification; these values are demonstration assertions, not a recommended
universal preset. All four analog fields are installed before enable, never
rewritten at STABLE; the precise hardware phase-switch instant is not promised.
Bypass keeps the same explicit fields for exact-source reuse. L083 has one
analog bank: the shared configuration supplies only run drive/amplitude and
WAITCYCLE there, with no L052 startup-drive/startup-amplitude fields.

On the fifteen retained packages, `bypass` requires an external digital clock on PC14. On the classic
three parts the reviewed input limits are: high level
0.7×VDDIO…VDDIO, low level VSS…0.3×VDDIO, high and low pulse widths at least
450 ns and rise/fall times at most 50 ns, plus the device's full I/O requirements.
The classic source must retain 45–55% duty and satisfy those limits every cycle.
For the four L031/R031 parts, qualify the corresponding own-datasheet
waveform and operating-condition rows bound by their source contract.
For L052, qualify VDD 1.65–5.5 V with VDDA=VDD and ambient −40…85 °C; the
example asserts the narrower interval above. Its own bypass limits are high
0.7×VDDIOx…VDDIOx, low VSS…0.3×VDDIOx, high/low pulses at least 450 ns,
rise/fall at most 50 ns and 45–55% duty, throughout every declared condition.
The own L083 limits are high 0.7×VDDIOx…VDDIOx, low VSS…0.3×VDDIOx,
high/low pulses at least 450 ns, rise/fall at most 50 ns and 45–55% duty.
Its auxiliary source envelope is 1.65–5.5 V with VDDA=VDD and −40…85 °C;
selecting LSE as SYSCLK requires a declared minimum VDD of at least 1.8 V.
The unchanged example conditions above satisfy this narrower system-target rule.
The 1 MHz input ceiling does not change this example's nominal 32768 Hz source.
PC15 is not consumed by bypass; inherited reservations still apply.

Both call only `Config.lse` plus `Sysclk::LSE`, then borrow the same source with
`LseClock` for RTC. The example epoch is deliberate demonstration provisioning;
replace it with intended wall time. A compatible running RTC is preserved by
`initialize_if_unset`, but fresh LSE admission still requires its documented RTC
admission state. Arbitrary warm handover is not supported.

No fixed 1 MHz time driver is selected. Poll budgets are iterations, not elapsed
startup deadlines. After LSE loss/fallback, published LSE bounds and downstream
timing assumptions are invalid. Errors can leave partial state and require reset;
ordinary reset may retain LSE and POR may be needed. Whole-bank GPIO inspection
can advance sampling/filter/events. Matching cold LSI on L031/R031 can
also resume parked direct consumers without a TRIM/WAIT write. See the
[classic contract](../../docs/classic-lse-sysclk.md) and the separate
[five-package contract](../../docs/l031-r031-w031-lse-sysclk.md).

On L052, admission can run the whole GPIOA/B/C/D/F banks. Matching stopped LSI
can resume UART1..3 SOURCE3 (native SORCE), an admitted manual AUTOTRIM timer
SRC1, GPIO FLTCLK5, MCO SOURCE4, PC4/AF6 on R8S6/R8T6 only, and enabled,
already work-ungated LPTIM ICLKSRC3 or LCD CLKCS0. Closed LCD/LPTIM work gates
stay closed. Newly resumed RTC SOURCE2 is excluded by the accepted state
combinations: cold LSE needs pristine RTC SOURCE0, while reused LSE already
needs ready LSI. The existing pre-Rust quiescent bus-master entry model remains
in force. Configured HSI must be legal at final bus dividers; the separate fixed
fallback budget uses undivided 8.16 MHz and assumes no retained divider values.
See [the exact-three L052 contract](../../docs/l052-lse-sysclk.md).

On the five L083 parts, admission independently covers the configured HSI at
final bus dividers and conservative raw factory-HSI fallback at 48.96 MHz.
The fallback bound grants no HSI/AHB/APB divider credit, even with CLKCCS off;
Flash WAIT2 remains installed after LSE selection. The source declaration's
historical 1.65 V minimum is unchanged for auxiliary LSE use. This new system
target additionally requires the inherited unchanged HSI to be independently
legal for the handoff, even if it was idle, and all affected outputs/consumers
to be quiescent. An inherited enabled PLL escapes through that unchanged ready
HSI, then is intentionally disabled; both PLLEN=0 and PLL.STABLE=0 must be
observed before its reference can change. This stops dedicated PLL outputs
while preserving PLL configuration. It supplies no independent PLL-output
lifecycle, runtime switching or fault recovery. No CR0 write follows the final
intentional LSE selection; electrical fallback headroom does not preserve RTC
or peripheral timing. See the [L083 system contract](../../docs/l083-lse-sysclk.md)
and [own source and package facts](../../docs/qualified-l083-lse.md).

## Native L010 SYSCLK examples

CW32L010F8P6, CW32L010F8U6 and CW32L010Y8M6 use the native example branch.
They use the existing single `Config.lse` declaration and native
`LseFaultDetection::StartupOnly`, with running drive `Level2`, independent
startup drive `Level10`, 16384 startup cycles and a 20000000-iteration poll
budget. There is no amplitude field. Both crystal and bypass keep these exact
parameters. They are demonstration assertions requiring actual crystal/load,
startup and waveform qualification, not a universal preset or measured board
result. The 1.50 s crystal startup figure is typical only, not a guaranteed
maximum or a meaning assigned to the poll budget.

Crystal uses PB1/OSC32_IN and PB0/OSC32_OUT; bypass uses PB1 only. Input/output
physical pins are 12/11 on F8P6 (TSSOP20), 9/8 on F8U6 (QFN20), and 10/9 on
Y8M6 (SOP16). Bypass must qualify high level 0.7×VDDIO…VDDIO, low level
VSS…0.3×VDDIO, high/low pulse widths at least 450 ns, rise/fall times at most
50 ns and 45–55% duty every cycle. The native 100 kHz bypass ceiling does not
change the nominal 32768 Hz declaration. The declared interval above fits the
own L010 1.62–5.5 V and −40…85 °C source envelope; declaration is not
measurement. Inherited crystal or pin-lock reservations may still retain PB0
on bypass or failure.

StartupOnly requires inherited LSECCS=0 and leaves it clear. STABLE counts
startup progress and may remain set after later source loss; later readiness
reads do not establish a continuing clock. Losing this SYSCLK can stop the CPU
without any error return or functioning timeout. Inherited CLKCCS=1 does not
create detection when LSECCS=0. No automatic fallback or elapsed-time
continuity is promised. `MonitoredExistingRoutes` is a separate supported
declaration requiring an already stable, unchanged, legally operating LSI at
entry, including automatic requests with LSIEN=0. Its inherited-legal maximum
is 36080 Hz and it needs `256 * LSE_min_hz > 129 * 36080`; neither STABLE nor
factory matching measures its frequency or individual-cycle jitter. The
initializer does not cold-prepare or factory-calibrate that monitor.

Native defaults retain HSI /12, AHB /1 and APB /1, with factory-calibrated HSI
enabled after successful initialization. Establishing HSI factory trim can
temporarily request unchanged LSI even for StartupOnly. The incoming source,
Flash/buses, HSI trim and unchanged LSI must already satisfy the original
legal-entry contract; in particular the LSI handover uses the own 32.8 kHz
±10% legal regime, not the factory ±3% specification. No LSI TRIM or WAIT
write is added, and a ready poll does not prove physical legality. Before a
first request when entry LSI was nonstable, retained RTC SOURCE2, UART1/2
SOURCE3, enabled LPTIM ICLKSRC3, MCO SOURCE4, an enabled LSIRDY IRQ request,
and enabled LSI-filtered VC/LVD users are rejected. If HSI must start or
restart, retained raw-HSIOSC RTC SOURCE3, MCO SOURCE3 and HSIRDY IRQ owners
have their own admission requirements.

These visible checks do not establish universal inactivity. The functional
handover must permit or disconnect remaining GTIM/ATIM selector9 roots,
whole-bank GPIO filters, an uninspected retained IWDT and external/downstream
observers across temporary LSI requests and restoration. New RTC SOURCE0 LSE
startup also retains the RTC_OUT/RTC_1Hz, timer/ADC/GPIO cascade and external
observer contract. Real untouched reset history can establish reset routes;
register resemblance, absent tokens, closed gates or interrupt masking cannot.
GPIOB inspection can resume sampling/filter/events for the whole bank, including
exact reuse and an eventual error. Restoring its gate does not undo progress.
Dormant timer work gates are not opened merely for inspection.

For monitored operation with inherited CLKCCS enabled, the own documented
effective fallback is HSI4MHz. The target independently budgets the full
factory-qualified upper bound 4.08 MHz for both buses and Flash, without
divider credit or an assumption about register changes on fallback. Requested
HSI remains separately legal at final divisors. This electrical headroom is
also checked with CLKCCS clear and for StartupOnly; it does not promise that
fallback occurs. Final divisors are installed on calibrated HSI before the
last LSE selection; there is no later CR0 write. Existing fault/brake/IRQ
routes can affect observers before an error, and source loss or fallback
invalidates published LSE timing. Errors return no new clocks or tokens but
may retain partially changed state, requiring reset before retry; ordinary
reset may retain LSE and POR may be needed. See the
[L010 system contract](../../docs/l010-lse-sysclk.md),
[native LSE handover](../../docs/qualified-l010-lse.md) and the public
`init` contract before adapting these examples.

## Native L011 SYSCLK examples

CW32L011K8T6 (LQFP32) and CW32L011K8U6 (QFN32) share the native configuration
branch, with running drive `Level2`, independent startup drive `Level10`,
16384 startup cycles, `StartupOnly` and a 20000000-iteration poll budget.
The board must qualify both drives, crystal/load/startup or bypass waveform,
and every-cycle 32766–32770 Hz across the stated 3.0–3.6 V and −20…70 °C.
These are demonstration declarations, not measurements or universal settings.
The own source envelope remains 1.7–5.5 V and −40…85 °C. The 1.50 s crystal
startup value is typical only; no maximum follows from the poll budget.

Crystal uses PC14/OSC32_IN and PC15/OSC32_OUT, physical pins 2/3 on both parts.
Bypass uses PC14 only; it does not acquire PC15, although inherited reservations
still apply. Qualify high 0.7×VDDIO…VDDIO, low VSS…0.3×VDDIO, high/low pulses
at least 450 ns, rise/fall at most 50 ns and 45–55% duty every cycle.
The existing 100 kHz ceiling is the stricter DS bound; the RM permits up to
1 MHz. That disagreement does not raise the accepted limit. L011 raw PINLOCK
rejects a requested source, including reuse; do not infer fault-time pad safety.

StartupOnly requires inherited LSECCS=0 and leaves it clear. LSE loss may leave
STABLE set and stop the CPU with no error return or functioning timeout, even
with CLKCCS enabled. The separate `MonitoredExistingRoutes` declaration requires
stable LSI and a non-erased own factory-halfword match at the first native
preflight, before configuration-gate writes. Its reference is the unchanged
10-bit TRIM from 0x001007C2 with the inherited WAIT field; LSIEN may be 0 for an
automatic hardware request. Its own DS factory range is 29520–41000 Hz, so
`256 * LSE_min_hz > 129 * 41000` requires integral minimum 20661 Hz. Hardware
counts 128 LSE edges in 256 LSI periods; 129 is the separate software margin.
STABLE and factory matching do not measure frequency or per-window jitter.
The initializer never auto-prepares or factory-calibrates this monitor.

Ordinary legal cold entry remains supported with StartupOnly. Defaults retain
HSI /24 (numeric divisor 24, encoding 14), AHB /1 and APB /1; successful init
keeps factory-calibrated HSI enabled. The existing HSI calibration bridge may
temporarily request unchanged, electrically legal LSI without writing its
TRIM or WAIT. The RM legal-adjustment contract is separate from the DS factory
monitor range: RM 32.8 kHz ±10%, its other 30–36 kHz/approximately 0.4% trim-step
description, and the DS 0.16% trim-step/factory −10/+25% specification are not
silently reconciled. Board/platform legality is an entry prerequisite, including
after real reset. Neither reset trim nor a later STABLE read establishes the
factory-monitored entry fact. StartupOnly does not require factory LSI. Rust
startup is not the SDK SystemInit routine that explicitly loads both factory
trims, and SDK erased-halfword replacements do not qualify factory trim.

When the first LSI request was nonstable at entry, checks after retained-owner
inspection and again immediately before LSIEN reject RTC SOURCE2 (including its
AWT, regardless of START/AWTEN) and reserved SOURCE4…7; UART1/2/3 SOURCE3
regardless of RXEN/TXEN; enabled LPTIM ICLKSRC3; MCO SOURCE4; enabled LSIRDY IRQ;
and enabled LSI-filtered VC1/VC2/LVD even with zero filter count. Later STABLE
does not skip that second admission. Held reset in an inspected domain rejects
without release; each configuration gate is restored independently and a failed
restore retains its specific error. Stable legal automatic LSI clients remain
admissible with LSIEN=0.

HSI ownership is separate. RTC SOURCE3 owns raw HSIOSC even with START=0 and
requires already enabled, stable, factory-matching HSI. Necessary HSI start,
restart or waiting for initially unready HSI also rejects MCO SOURCE3 and
enabled HSIRDY IRQ. A factory-ready divider-only change preserves raw HSIOSC.
Active ADC, SYSCLK-filtered LVD and PCLK-filtered or timer-blanked comparators
retain their own vetoes. L011 also has dedicated PB0 AF3 HSIOSC_OUT: that
observer can see HSI start/retrim interruptions even with MCO disabled.

These checks do not prove universal inactivity. The functional handover must
permit or disconnect GTIM/ATIM LSI_OUT selector9, whole-bank GPIO LSI filters,
retained IWDT and cascaded timer/ADC/GPIO or external observers across the
temporary LSI request and restoration. Dormant timer working gates are not
opened merely to inspect them. Native source-zero RTC admission and RTC_OUT
PA1/PA3 AF3 observers remain part of the handover; a quiet RTC image does not
prove output-root disconnection. The native quiet record excludes H24 and does
not interpret reserved CR1 bits1:0 as ACCESS/WINDOW; DATE/TIME/PSC/AWTARR and
flags remain preserved without unlock, commands or reset. GPIOC inspection can advance whole-bank
sampling/filter/events before an eventual error, and gate restoration cannot
undo that progress. The pre-Rust bus-master/memory-ownership boundary remains
the existing platform entry contract.

The own documented effective fallback is HSI4MHz when selected LSE fails with
CLKCCS enabled. The software independently budgets full 4.08 MHz for buses and
Flash without divider credit, even when CLKCCS is clear, and checks configured
HSI at final divisors separately. This does not establish hardware changes to
HSI.DIV, enables or bus dividers, or promise execution continues. Initial WAIT3
is conservative; final WAIT is 0/1/2/3 according to the maximum qualified LSE,
configured-HSI and fallback HCLK bounds. The default HSI /24 path uses WAIT0.
Final divisors precede LSE selection; no later CR0 write is permitted. Existing
fault/brake/IRQ routes remain live. No automatic flag clearing, output repair,
RTC migration, rollback or bounded fault-to-fallback time is promised. Loss or
fallback invalidates frozen timing; errors publish no clocks, may retain changed
hardware/gates/pads and require reset before retry. Ordinary reset may retain LSE.

The existing raw-HSIOSC calendar remains SOURCE3 at nominal 96 MHz with bounds
94.08–97.92 MHz and actual divisors 120 and 400000. Held LSE calendar capability
continues to borrow this same physical source with first divisor 1 and second
16384 (total 32768). SYSCLK selection does not reset or migrate a retained
RTC/AWT owner. See the [L011 system contract](../../docs/l011-lse-sysclk.md),
[native L011/L012 handover](../../docs/qualified-l011-l012-lse.md) and public
`init` contract before adapting these examples.

## Native L012 SYSCLK examples

CW32L012C8T6 (LQFP48) and CW32L012C8U6 (QFN48) reuse the native branch:
independent running `Level2` and startup `Level10` drives, 16384 startup cycles,
`StartupOnly` and a 20000000-iteration poll budget. There is no amplitude field.
Both modes explicitly declare 3.0–3.6 V, −20…70 °C and every-cycle
32766–32770 Hz. Replace these declarations with actual board-qualified data;
they are not measured board validation or universal crystal/startup settings.
The own source envelope is 1.7–5.5 V with VDDA=VDD and −40…85 °C.
The 1.50 s crystal startup value is typical, not a maximum or a poll deadline.

Crystal owns PC14/OSC32_IN pin3 and PC15/OSC32_OUT pin4 on both packages.
Bypass owns PC14 only; inherited PC15 reservations still apply. The board must
qualify the selected crystal/load and both drives, or the external waveform:
high 0.7×VDDIO…VDDIO, low VSS…0.3×VDDIO, high/low pulses at least 450 ns,
rise/fall at most 50 ns and 45–55% duty throughout the declared conditions.
The accepted bypass ceiling remains the stricter DS 100 kHz limit despite the
RM's 1 MHz statement. Requested new or reused LSE rejects raw PINLOCK; no
fault-time pad guarantee follows from that refusal.

StartupOnly requires inherited LSECCS=0 and leaves it clear. Later loss can
leave STABLE set and halt the CPU without an error return, even with CLKCCS
set. It adds no factory-LSI requirement. The separate
`MonitoredExistingRoutes` policy requires already stable LSI with a non-erased
own factory halfword at 0x001007C2 matching the native 9-bit TRIM, and unchanged
TRIM/WAIT from entry before configuration-gate writes. LSIEN may be clear when
automatic consumers request LSI. The own factory 32.8 kHz ±10% envelope is
29520–36080 Hz; `256 * LSE_min_hz > 129 * 36080` requires integral minimum
18181 Hz. Hardware counts 128 edges; the extra edge is a separate engineering
margin. No automatic monitor preparation, frequency measurement or guaranteed
per-window jitter bound is supplied.

L012's configured default is HSI /12, nominal 8 MHz with factory bounds
7.84–8.16 MHz, and AHB/APB /1. Its silicon reset divider and documented effective
fallback are separately /24, nominal 4 MHz. Successful initialization retains
factory HSI. The existing HSI-calibration bridge can temporarily request
unchanged, electrically legal LSI without writing LSI TRIM or WAIT; it does not
qualify a monitor. Legal incoming HSIOSC 90–100 MHz, buses and Flash latency
remain platform entry conditions. Enabled inherited HSE needs an explicit
declaration and exact ready-source/pad reuse, independent of RTC ownership.

An entry-nonstable first LSI request keeps that classification through repeated
consumer admission immediately before LSIEN. Refusals cover RTC SOURCE2/reserved
sources, UART1/2 SOURCE3 regardless of enables, operational UART3 SOURCE3,
either I2C1/2 master/slave raw source1/3, enabled LPTIM ICLKSRC3 regardless of
count mode, MCO4, LSIRDY, and enabled LSI-filtered LVD or any VC1..4 even with
zero filter count. Needed HSI start/retrim/unready waiting independently checks
raw-HSI RTC ownership, MCO3, HSIRDY and both I2C sides' source1/3. The I2C
source3 conflict remains unresolved. Held resets reject without release;
configuration gates restore independently and restoration errors take priority.

The supported handover must permit shared ADC1/ADC2 work when inspection opens
the ADC configuration-and-work gate. Conversion/trigger/downstream progress can
precede an enabled-ADC refusal; restoring the gate cannot undo it. Whole-GPIOC
sampling/filter/events, PB0 AF3 HSIOSC_OUT, PB11 AF4/PF3 AF2 LSI_OUT, retained
IWDT, timer/cascade/external observers and inaccessible UART3 also require
functional handover. Dormant timer/output/UART3 working gates are not opened to
infer inactivity. Native BTIM selector3 means RTC_OUT only on BTIM1, LPTIM_OV
on BTIM2 and VC4_OUT on BTIM3; SDK and ATIM mapping conflicts stay explicit.
The existing pre-Rust bus-master/memory-ownership boundary remains in force.

The final plan independently checks requested LSE, configured factory HSI at
final divisors, and full 4.08 MHz fallback without AHB/APB divider credit. This
is conservative headroom, not a guarantee of divider retention or CPU progress.
Initial WAIT3 precedes guarded HSI; final WAIT0/1/2/3 is installed and checked
while calibrated HSI and final divisors are still selected. The default HSI /12
needs WAIT0. Own FLASH.WAIT and SYSCTRL.FLASHWAIT must agree; mixed old/new
readbacks fail closed. FETCH/CACHE/CACHEINVALID and other controls are retained.
The final LSE selector write is followed by no CR0 or FLASH write, even on error.
Fault/brake/IRQ routes remain live; no rollback, flag clearing, output repair,
RTC migration, loss recovery or bounded fault-to-fallback time is promised.
Errors publish no clocks, may retain partial hardware state and require reset
before retry; ordinary reset may retain LSE and POR may be needed.

The held LSE calendar still uses SOURCE0 and actual divisors 1 and 16384 (total
32768). The raw-HSIOSC calendar remains SOURCE3 at 96 MHz, bounds
94.08–97.92 MHz, with actual divisors 120 and 400000. SYSCLK selection does not
reset or migrate RTC/AWT. The fixed 1 MHz time driver rejects this source before
singleton acquisition. See the [L012 system contract](../../docs/l012-lse-sysclk.md),
[native L011/L012 handover](../../docs/qualified-l011-l012-lse.md) and public
`init` contract before adapting these examples.

The Stage61 local script scope was twenty ordinary libraries and nineteen
crystal/bypass package pairs (38 system-clock binaries). Stage62 expanded that
historical scope to twenty-two libraries and twenty-one pairs (42 binaries).
The current local scope of `ci/check-lse-sysclk.sh` is twenty-three ordinary
libraries and twenty-two crystal/bypass pairs (44 system-clock binaries),
plus the existing four auxiliary/other-source regression commands. These are planned local command
counts, not executed results. Preserve older actual validation receipts and
record each new executed subset separately. No hosted CI, HAL test, hardware
run or new probe follows from adding these commands. The unchanged
`examples/l010-lse-clock` L011/L012 features remain available for auxiliary crystal,
bypass and HSIOSC-calendar preservation checks outside this local script.

The build script obtains exact memory limits from generated metadata and supplies
`-Tlink.x` explicitly. Each of the three L010 packages has 65536-byte Flash and
4096-byte RAM; both L011 and both L012 packages use their own generated memory
entries, with no guessed linker sizes. No crate or dependency upgrade is involved.
Compilation/link/ELF inspection do not run this firmware or
qualify physical startup, detection windows, board timing, fault recovery or RF.
