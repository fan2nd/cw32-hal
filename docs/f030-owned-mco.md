# Owned bus-clock output on four exact F030/A030 parts

`rcc::Mco` owns the real `SYSCTRL` peripheral and PA8 through `Flex`. Its exact
scope is CW32F030C8T7, CW32F030K8T7, CW32F030K8U7 and CW32A030C8T7, all PA8 AF4:

- CW32F030C8T7 and CW32A030C8T7: LQFP48, PA8 pin 29, PC14/PC15 present.
- CW32F030K8T7: LQFP32, PA8 pin 18, PC14/PC15 absent.
- CW32F030K8U7: QFN32, PA8 pin 18, PC14/PC15 absent; the existing exposed-VSS
  position 0 convention is retained.

Generic CW32F030/CW32F030C8/CW32F030K8/CW32A030 aliases and all other selections
remain unqualified. In particular, F030F8V7/QFN20 and F030F6P7/TSSOP20 lack PA8.
Absent qualification is not a claim that other silicon lacks MCO. The existing
C8T7 qualification remains Stage76 evidence. Independent Stage77 main runtime
and metadata reviews accept the extension; final-source clean completion is
separate, as described below.

The unchanged immutable constructor selects already initialized HCLK or PCLK
and one of /1, /2, /8, /64, /128, /256, /512 or /1024. It changes no oscillator,
calibration, trim, SYSCLK source, HSI divider, AHB divider or APB divider.
There is no raw oscillator/PLL-output selection, runtime source/divider setter,
synthetic token, checked shutdown, token-release API, low-power restoration or
clock-fault recovery. Raw PAC `source`/`div` getters and setters retain their
existing `u8` signatures.

## Admission and board declarations

`Mco::new(sysctrl, pa8, source, prescaler, board)` consumes `Peri` values and
returns `Result<Mco, McoError>`. The board argument is explicit:

```rust
McoBoardConditions {
    operating_conditions: OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    },
    max_load_pf: 50,
}
```

These are example declarations requiring actual board qualification. They are
not measured values, a qualified BOM or defaults. The voltage interval must cover
the actual PA8 I/O supply, and the total-load maximum includes routing, receiver
and attached measurement equipment. Both operating intervals must be ordered;
load must be nonzero. Zero is rejected as an incomplete declaration, not as a
silicon minimum capacitance.

The constructor reads published `try_clocks()` data. A prospective
`Config::frequencies()` result does not establish initialization. The selected
bus must have `has_cycle_timing_bounds() == true`; rate-only PLL/LSI-rooted buses
are rejected even when division makes their average rate small. Existing
HSI/HSE/LSE-rooted buses remain eligible only under their original qualified
source and initialized-clock contracts.

The supplied operating interval must fit the published source's voltage and
ambient envelope, and the MCO policy of 2.4–5.5 V and −40–105°C. Total load must
be at most 50 pF. The normal ambient policy excludes the low-power 125°C
extension. Source conditions can narrow this range, and VDDA, junction,
power/current and other datasheet limits still apply. At TA=105°C the own
Table 7-4 package power limits are 364 mW for F030/A030 LQFP48, 357 mW for
F030 LQFP32 and 541 mW for F030 QFN32. These are package-dependent whole-device
limits, not output-power allowances; the actual junction temperature must remain
within the own datasheet's limit. A common MCO frequency policy does not make
package thermal performance interchangeable.

Frozen `Clocks` does not retain the operating declaration used at initialization.
MCO therefore checks compatibility with the selected source envelope; it cannot
recover or verify equality with the earlier board declaration. The caller must
truthfully describe the same running board, while all earlier source/init
requirements continue to hold. The examples use the same constant for both
declarations. A narrower later declaration cannot prove that actual conditions
changed or retroactively justify initialization.

Admission divides the exact source bounds by the selected metadata divisor and
requires the maximum to be at most 20 MHz, including fractional remainders.
Equality is admitted; rounded nominal or whole-hertz rates are not the criterion.
`clock_bounds()` returns that retained exact logical-clock envelope and
`clock_bounds().nominal()` gives its nominal rate. It is conditional on continued
source/board validity, not a measurement, live health check or pad timing bound.
A valid sub-hertz output may round to zero in a whole-hertz accessor. Duration
getters explicitly panic if a rounded nanosecond result exceeds `u64`; even a
valid slow clock can exceed that representation for a sufficiently large cycle
count. No public duration signature changes.

## Electrical limits do not qualify the waveform

F030 DS CN1.9 Table 7-27/Figure 7-7 (PDF51/printed50) and A030's own DS
CN1.1 Table 7-26/Figure 7-7 (PDF48/printed47) label their values as design/simulation
data, not measured data. At SPEED1 and CL=50 pF, each table lists 20 MHz for
2.4≤VDDIO<2.7 V and 30 MHz for VDDIO≥2.7 V; a separate CL=30 pF row lists 50 MHz
above 2.7 V. This API uses the conservative 20 MHz policy across its voltage
range. Allowing a declared load up to 50 pF is a conservative board-design
interpretation of the CL=50 pF row, not additional measured characterization.

The vendor's fmax definition also requires loaded duty of 45–55% and
`tr + tf <= 2T/3`. The rate check, source cycle bounds and integer divisor do not
prove these conditions. The MCO divider documentation does not establish
separate high/low pulse widths for every divider. The board designer must qualify
the actual loaded waveform and receiver requirements, including duty, rise/fall,
supply tolerance, routing, VOH/VOL/current, total load and power/thermal limits.
Until then, observation with suitable equipment must not be described as a fully
qualified receiver clock.

AF attachment uses the existing push-pull, no-pull, SPEED1 GPIO behavior. It
preserves PA8's inherited DRIVER strength. Qualification includes that actual
setting; the driver does not regulate strength or shape the waveform. No 50%
duty guarantee, glitch-free first/last pulse, phase continuity or fault-tolerant
delivery is implied. Board and downstream handover are functional/electrical
requirements, not additional hidden Rust memory-safety conditions on this safe API.

## Acquisition, shared GPIOA effects and errors

All numerical/board admission precedes GPIO or MCO writes. MMIO admission and
acquisition occur in one ordinary critical section. Nested existing HAL critical
sections do not add another owner or halt physical clocks, DMA, NMI or debugger
activity; later raw interference is outside the stable-register contract.

The hardware sequence is bounded and has no polling loop or hidden retry:

1. Nonzero inherited MCO SOURCE gives `OutputAlreadyEnabled`, even if it already
   matches the request. Reserved MCO bits give `ReservedMcoState`, and GPIOA held
   in reset gives `GpioResetAsserted`. No GPIO/MCO write has occurred. An idle MCO
   need not have DIV zero.
2. `Flex` opens GPIOA's gate, clears only PA8's lock bit using the bank lock key,
   preserves other low lock bits and disconnects PA8. The inherited GPIO gate
   read barrier is not a condition-checked acknowledgement. MCO performs its own
   gate/reset, lock and disconnected-pin readback. Failure is `GpioReadback`.
3. A typed MCO read-modify-write changes only SOURCE and DIV. Their readback is
   checked before AF attachment. Failure is `McoReadback`.
4. AF attachment selects PA8 AF4, digital push-pull/no-pull/SPEED1 with output
   enabled last. The driver then checks the complete PA8 configuration plus
   gate/reset/lock and rereads MCO. A MCO mismatch has priority over a pin
   mismatch (`PinReadback`). Success retains the owners and exact divided bounds.

Disconnected state means PA8 analog/input with pulls, open-drain and all four
edge/level enables clear. Configured state means AF4, digital/output,
push-pull/no-pull/SPEED1 and all edge/level enables clear. ODR and DRIVER are
inherited; unrelated pins are not compared against a whole-bank snapshot.

Opening the whole GPIOA gate can resume other pins' sampling, filters and armed
edge/level events before PA8 is disconnected, including on a constructor failure.
Interrupts can be serviced later. GPIOA remains enabled after success, error and
Drop; closing or resetting a shared bank would not undo this progress. Only PA8
is unlocked. PA8 stays unlocked, its prior function is not restored, and existing
downstream users must tolerate disconnection, clock activity and stopping.
The driver does not clear GPIO pending flags, configure NVIC or reset filters.

Before `Flex` acquisition, an error consumes passed tokens while leaving
GPIO/MCO hardware unchanged. A failed reborrow can end and allow use of the
original tokens; moved singletons are not reconstructed or returned.
After `Flex` acquisition, error cleanup runs inside the same critical section.
`GpioReadback` has not programmed MCO: the local `Flex` is dropped before the
final `pin_disconnected_readback` observation. For MCO/pin errors, `drop(pin)`
first runs the local `Flex` destructor to disconnect PA8; only after that drop
does cleanup disable SOURCE while retaining DIV. It then reports
`source_disabled_readback` and `pin_disconnected_readback` separately. This is
distinct from normal `Mco::drop`, which explicitly disconnects the retained
`Flex`, disables SOURCE, and is followed by the `Flex` destructor.
Those booleans are observed register states. A false value is a real unacknowledged
cleanup; even true values do not establish electrical behavior or protection
against later interference. Bus faults, aborts, reset and broken internal
invariants are not recoverable constructor results with guaranteed cleanup.

## Drop, forgetting and RTC ownership

Drop attempts PA8 disconnection first, then disables only MCO SOURCE with a read
barrier. The ordinary `Flex` destructor repeats the idempotent disconnect. This
bounded best effort returns no result and cannot report an acknowledgement
failure, guarantee an output level or promise clean last pulses. Normal
post-Drop state retains DIV, AF4 and SPEED1, preserves ODR/DRIVER, leaves PA8
unlocked and GPIOA enabled, and disconnects the pad. Before AF attachment, an
error can instead retain the older AF/SPEED values. Other pins and pending
status remain inherited or naturally advanced.

`mem::forget` is permitted and can leave the output active indefinitely. Moved
owners are lost; with reborrowed tokens, Rust can later permit original-token
reuse after the forgotten borrow ends. New `Mco` construction still refuses an
active SOURCE, while ordinary later PA8 use can disconnect/reroute the pad and
leave the orphaned MCO selector active. Shutdown is not leak-proof.

Owning real SYSCTRL excludes a simultaneous live safe `CalendarClock`, `LsiClock`
or `LseClock`; `Rtc` retains its `CalendarClock`, so this is not an init-only
restriction. Non-overlapping reborrows are possible after normal Drop. A
forgotten hardware output can outlive its Rust owner and later coexist with
an RTC after reborrow recovery; type-level exclusion is not permanent hardware
exclusion. There is no SYSCTRL split. The existing RTC preservation executable
remains separate from the two MCO examples.

## Sources, projection and verification boundary

The [authored catalog](../cw32-data/mco-qualified.yaml) retains the
[Stage76 C8T7 qualification](f030-mco-qualification.json). Stage77 source records
for the additional exact selections are `docs/f030k8t7-mco-qualification.json`,
`docs/f030k8u7-mco-qualification.json` and `docs/a030c8t7-mco-qualification.json`.
The extension has separate Stage77 main implementation/build acceptance;
neither the record names nor the old C8T7 reviews supply that acceptance.

All four use the locked shared x030 RM CN2.5. F030 uses its own DS CN1.9;
A030 uses its own DS CN1.1. The F030 SDK RCC/GPIO sources retain their F030
provenance and serve only as supplementary evidence for A030. RM PDF61/printed60,
PDF94/printed93 and PDF147/printed146 document MCO, its register geometry and
PA8 AF4. The register is at offset 0x70 with SOURCE bits 3:0, DIV bits 6:4 and
reserved bits 31:7. Codes 0–7 select /1, /2, /8, /64, /128, /256, /512 and /1024;
disabled/HCLK/PCLK source codes are 0/1/2. No encoding is inferred from a public
enum ordinal.

F030 DS PDF24/printed23 and PDF26/printed25 distinguish K8 PC14/PC15 absence
and PA8 pin 18 from C8T7's PA8 pin 29. F030 AF4 is in PDF29/printed28. A030's
own DS PDF22/printed21, PDF23/printed22 and PDF26/printed25 establish PC14/PC15,
PA8 pin 29 and AF4 respectively. Ordinary operating and package power limits
are at F030 PDF38/printed37 and A030 PDF35/printed34; the I/O waveform tables
are distinct as identified above. See the [source index](../sources/SOURCES.md)
for the source/version and locator context.

The permanent source/data policy admits exactly four of all 54 selections and
excludes the other 50. Each admitted part requires one real SYSCTRL owner, its
complete PA8/MCO/AF4 route and exact package pin identity. Both C8T7 parts require
PC14/LSE_IN and PC15/LSE_OUT; both K8 parts require those routes and the two pin
identities to be absent everywhere. This does not remove GPIOC or die-level LSE
register facts, or grant K8 an LSE API. PF0/HSE_IN and PF1/HSE_OUT reservations
remain required on all four parts. Generic family AF discovery is not promoted
to qualification. The HAL validates family, exact identity, owner, route,
electrical/table facts and selected register geometry before generating the
existing `mco_f030` cfg and constants. PAC interfaces stay intact.

Stage77's independent one-time generated comparison passed across 905 files:
416 under `cw32-data/data` and 489 under `cw32-metapac`. Only the three added
exact-part chip JSONs gain the complete MCO fact and PA8/AF4 route; the other
51, including F030C8T7, remain byte-identical. All 54 metadata wrappers resolve
to 39 bodies, as before. Six wrapper include numbers change; resolving those
includes leaves 51 selected bodies byte-identical and only the three new
capability projections changed. In total, 15 generated paths change, with no
additions or deletions. The three schema models, generated metadata template
and runtime implementation apart from its first scope-comment line remain
unchanged. Historical generated hashes are not permanent source-checker gates.

[The examples](../examples/shared/mco/README.md) retain the same /8 programs and explicit
matching init/output board declarations. Each selected exact part uses its
64 KiB Flash / 8 KiB SRAM map and explicit `link.x`. The current `./d check-mco`
recipe has seven release `rt,defmt` libraries: F030K8T7, F030K8U7, A030C8T7,
F030C8T7, generic F030, generic A030 and no-PA8 F030F8V7. It has five release
ELFs: F030K8T7 HCLK, F030K8U7 PCLK, A030C8T7 HCLK, F030C8T7 PCLK and the
separate A030C8T7 RTC `preserve_calendar` executable. All library commands use
`--manifest-path firmware/Cargo.toml`; the repository-root workspace does not
own the HAL. Full commands are in the [local verification guide](../ci/README.md).

Stage77 main verification produced seven real ARM library rlibs and five linked
ELFs. All twelve commands exited zero with no warnings; the 1,330 authored-file
and 905 generated-file snapshots stayed unchanged during the matrix. Independent
main runtime and metadata reviews accept that frozen implementation. Eight finite
source/data gates passed, including all 22 existing metadata contracts and all
43 required hardware originals. The two discovery HTML records remain excluded;
this is hardware-source completeness, not acquisition of all 45 catalog entries.
Matrix tool versions were captured in a post-run supplement, not a preflight.

The first generation attempt failed because a new singleton-orderable check
rejected the retained F030C8T7 LQFP48 source row containing both C8T7 and C8T6.
The repair validates that exact two-name source roster while keeping the MCO
admission allowlist at four exact parts. The successful regeneration does not
erase the original failed attempt or its separate repair record.

At main acceptance final-source clean had not run; completion requires a
separate two-library/two-ELF and finite-gates receipt. Clean replay uses F030K8U7
and A030C8T7 release libraries plus their PCLK and HCLK ELFs respectively, from
a fresh extraction and fresh target directory. Main and clean receipts separately
bind exact source, commands, toolchain, exits, warnings and actual artifacts.
These acceptance-status documentation changes follow the main build snapshot;
they are not retroactively described as its input and must be bound by final-source
verification. No aggregate recipe, new HAL tests, compile probes, register models,
hosted CI, divisor-instantiation matrices or hardware execution supplied these
results. Board and loaded-waveform qualification remain unverified.

### Historical Stage76 evidence

The original [schema acceptance](f030-mco-schema-acceptance.json),
[runtime acceptance](f030-mco-runtime-acceptance.json),
[metadata acceptance](f030-mco-metadata-acceptance.json) and qualification record
retain their original identities and C8T7-only scope. Stage76 main verification
produced five real ARM library rlibs and three ordinary firmware ELFs, all eight
Cargo builds successful with no warnings; source bytes and modes stayed unchanged
during those builds. Generation and the 904-to-905-file comparison across all
54 selections passed, limited to the then-approved C8T7 fact/route and optional
schema/projection changes.

All 19 schema contracts passed after generation; the earlier missing-generated-
directory failure remains historical. Eight finite source/data gates passed in
separate receipts, including all 22 metadata contracts after correcting seven
existing L012/L083 route expectation omissions; the earlier failure also remains
historical. No aggregate recipe ran. At Stage76 main acceptance, clean replay
had not run; its separate final-source replay scope was one exact F030C8T7
release library and one HCLK ELF. That historical main 5/3 and separate clean
1/1 scope does not supply Stage77's 7/5 main or 2/2 clean results. Missing private
Stage76 archive/receipt material is not reconstructed by the public source commit.
