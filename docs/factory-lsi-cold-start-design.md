# F020/F030/A030 factory-LSI SYSCLK with genuine cold-start admission

Read-only design audit, 2026-10-09. Revision 2 incorporates independent review of HSI calibration failure state and the narrow same-source RTC compatibility audit. It is complete and intended for independent review before implementation. Stage55 candidate is unchanged; no Cargo, HAL tests, adapter/harness, cleanup, commit or publication occurred. All page numbers below are one-based physical PDF pages (printed page is one less). `source-hashes.json` identifies the exact own-manual/datasheet/SDK files and freshly extracted texts; `inspected-code-hashes.json` identifies the inspected Stage55 code. No statement here is a silicon experiment.

## 1. Decision and minimal public contract

A useful cold-start route is supported by these own sources. Add a single initialization-only `Sysclk::LSI` selection for F020/F030/A030. It means factory-qualified nominal 32,800 Hz, with the selected family's rate interval and operating conditions. Keep the default `Config` on its existing HSI source. No reset-history token, caller-supplied trim, arbitrary-frequency declaration, risk-acceptance boolean, or unsafe factory-LSI constructor is needed.

The behavior is:

1. Reuse a factory-matching already-running LSI without changing TRIM or WAITCYCLE, regardless of existing direct LSI consumers. Keep the source alive and assert its permanent software request. A matching enabled source still starting may be allowed to finish within the existing poll budget without changing its parameters; contradictory hardware status is rejected.
2. If LSI is genuinely stopped, inspect every reviewed direct requester/root/ready observer before starting it. Factory TRIM may be loaded only after complete twice-checked admission. Apply the same new-start admission when the stopped TRIM already happens to match: equality does not authorize activating dormant consumers.
3. Never stop, retune, or change the startup delay of a running/in-flight nonmatching source to manufacture admission. Report an incompatible-calibration/in-use result, not a claim that nonfactory LSI is illegal.
4. Starting this mode can temporarily run all GPIOA/B/C/F banks to inspect their retained root selectors. This is an explicit functional handover of those whole banks for the bounded inspection intervals, including before an error. Sampling, filters and armed events can advance. Gate restoration cannot undo this progress. Every GPIO setting and flag is preserved. This is an observable behavior of selecting the mode, not a hidden Rust memory-safety precondition.
5. Existing pre-Rust bus-master quiescence and one-time HAL ownership remain the platform entry model. Normal critical sections do not sanitize arbitrary bootloader DMA, debugger writes or exception handlers that modify hardware. The new safe mode must not introduce a new memory-safety obligation disguised as a documentation note.

The narrow alternative that never opens closed GPIO workgates is valid only for an already-open/readable-bank handover. It cannot provide ordinary reset success, because GPIO AHBEN resets closed. A variant that leaves closed banks unprobed would need explicit handover of their unverified LSI selectors/outputs, which weakens automatic admission and adds another public policy. It is not the recommended first API. The whole-bank route above obtains the actual selector evidence and keeps the unprobed obligation limited to functional effects of opening a working bank and arbitrary external/software expectations.

## 2. Exact source identity

| Evidence | SHA256 |
|---|---|
| CW32x030_UserManual_CN_V2.5.pdf, joint F030/A030 own manual | `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2` |
| CW32F020_UserManual_CN_V1.4.pdf | `279521eaf7d32b241e614a9553044a4d99c1e7ca2566efa3b0fdd2b66c106eed` |
| CW32F030_DataSheet_CN_V1.9.pdf | `04ef91434320e5d05a3b6690fead0fedb31a7d9bad28c96b655a6e13a22e46b2` |
| CW32A030_DataSheet_CN_V1.1.pdf | `690433f36376352ee342ae3e2aa76786f2e7e59892a2519411f2754ac32c33a9` |
| current-datasheets/CW32F020_DataSheet_CN_V1.3.pdf | `1e330d800f10114c654b97cbd45b64d56a99c3cb39138d4ef20de3c12968dab0` |
| CW32F030_StandardPeripheralLib_V2.2.zip | `7c431df43d7075b817a51d818ea4c9aba55f83b7c9c77bf0065976758954b780` |
| CW32F020_StandardPeripheralLib_V1.2.zip | `1d77fece47a0c615c8ae51374ea946b17ab489042222f33e38d93e6969945b5d` |

The top-level F020 file named V1.3 actually contains Rev1.2 and is not used for current electrical bounds. F030/A030 share the explicitly joint manual, but their electrical datasheets remain separate. No independent A030 SDK has been asserted.

The SDK confirms the halfword factory read, not arbitrary live reconfiguration:

- F030 `Libraries/src/system_cw32f030.c`, SHA256 `62f27339a1dce8dc8fc67c37a0dc4f2b91ca3f0c8767a657533655aab46d8454`, lines41–42: volatile uint16_t reads load HSI and LSI TRIM. F020 counterpart SHA256 `8797aaaf4f0a695ebf43f51ebb00d0fe896a1c85e7e927e4f847218ad9b09f3e`, same lines. Neither SystemInit writes CR1. Their startup assembly calls SystemInit before C entry (member hashes in the receipt).
- F030 `cw32f030_rcc.c`, SHA256 `dbb3c80fa07af902f704c7e812d3b04d8819548736eb1163fe9feaae1d1b4370`, lines303–309, loads factory TRIM, replaces WAIT with 258 cycles, enables and polls. F020 `cw32f020_rcc.c`, SHA256 `8323dfddbc93401ef718ca39531059c327dcd0a4dbf3d193512163c34d5b10e5`, lines314–319, does the corresponding operation. These helpers do not prove owner absence and should not be copied wholesale.
- F030 `cw32f030_rcc.h`, SHA256 `de32996b8f852c456ada827bc5c13fe64a29fc12115489d98e2d3ddf6133d184`, gives public factory address0x00012602. F020 header SHA256 `b9e5f085b7789a9f29322a3371f8ae666076388bc0cfecf4eacd1999ea590749` also has a private-debug alternative0x001007B2. Only the own-manual public0x00012602 route belongs in this design.

## 3. Reset, stability and mandatory CCS are compatible

Both CR1 tables explicitly give reset `0x00000001`: x030 PDF71 §4.7.2; F020 PDF69 §4.7.2. HSECCS and LSECCS are RW bits7 and6; CLKCCS is RW bit8. Therefore their documented reset values are zero. The same tables prescribe writing one to these bits. This does not turn a reset read into one or authorize clearing an inherited one.

Both narratives independently say external HSE/LSE fault detection defaults disabled (x030 PDF59–60; F020 PDF57–58). Runtime detection additionally requires LSIEN=1. The proposal respects both statements by completing the stopped-source checks and the non-keyed LSI.TRIM write **before this initialization's first CR1 write**, then writing all three CCS bits as one on every CR1 write. It never writes CCS=0. Moving the new helper after the existing Stage55 mandatory-CCS write would make genuine reset fail; that ordering is incorrect.

The cold path must additionally reject inherited enabled or stable HSE/LSE, and HSE/LSE as active SYSCLK. Otherwise the first mandatory CCS=1 write could activate an external detector against a newly starting LSI reference. These manuals do not specify a holdoff until LSI becomes stable. Do not infer one. Require HSEEN=LSEEN=0, HSE.STABLE=LSE.STABLE=0, and the corresponding ISR stable mirrors low. New requested HSE/LSE sources may be started only after the factory LSI reference is stable. This extra restriction applies only to a new stopped-LSI start; unchanged running-LSI reuse does not reject existing external consumers merely for being present.

LSI's register reset is explicitly `0x---- ----` (x030 PDF74; F020 PDF72). Do not compare the full register against a fabricated reset constant. LSI is documented disabled by default, and STABLE clears when its source stops (x030 PDF53/58; F020 PDF51/56). ISR reset is `0x00000801` (x030 PDF79; F020 PDF77), which independently has LSISTABLE14=0 and LSIRDY3=0. Both stable indications must be read and checked, not assumed from a POR bit. Hardware similarity is sufficient only because the complete current predicates are checked; it is not a reconstruction of reset history.

Normal genuine reset can satisfy the admission: CR0.SYSCLK=HSI; CR1=1; external/LSI stable signals low; IER=0; ISR LSI-ready/stable clear; MCO source0; RTC source0=LSE; AWT source0=HSIOSC; all three UART sources0=PCLK; all four GPIO filters0 and PB11 AF0; NVIC pending reset0. The disabled peripheral configuration gates may be opened and restored under the protocols below. Unknown initial LSI TRIM/WAIT does not obstruct admission because all WAIT codes are defined and only TRIM is changed. An erased factory cell, inconsistent status, real retained owner or hardware readback failure still rejects. This is a checkable reset route, not a promise that every warm reset or firmware jump succeeds.

## 4. Exhaustive direct LSI root/requester/observer roster

SYSCTRL base is0x40010000. Register/selector facts below were independently checked in both manuals. Every defined selector is checked conservatively; invalid/reserved encodings reject rather than being interpreted as disconnected.

| Root | Concrete admission for a new stopped-source start | x030 own pages | F020 own pages |
|---|---|---|---|
| Explicit request and current system source | CR1.LSIEN3=0; CR0.SYSCLK2:0 is legal and not3. Also no HSE/LSE selected under the conservative external rule above. | 53,56,70,71 | 51,54,68,69 |
| Startup stability and ready observation | LSI(+0x20).STABLE15=0; ISR(+0x10).LSISTABLE14=0; IER(+0x0C).LSIRDY3=0; ISR.LSIRDY3=0. Neither flag nor IRQ enable is cleared. | 58,74,78,79,81 | 56,72,76,77,79 |
| Pending RCC handler | Read NVIC_ISPR at0xE000E200; IRQ4/RCC pending bit must be0. This rejects an already pending global RCC observer without clearing it, even when its peripheral cause was cleared earlier. No NVIC enable, priority or pending state is written. | 96–100 | 94–98 |
| RTC clock root | RTC0x40002800 CR1(+8).SOURCE10:8 must be one of0,4,5,6,7, never2=LSI or reserved1/3. Check regardless CR0.START. | 177–180,184,187–189 | 174–177,181,184–186 |
| Independent AWT | AWT0x40014C00 CR(+0).SRC10:8 must be0,2,3,4, never1=LSI or reserved5–7. Check regardless EN. | 166–173 | 163–170 |
| All UART transfer clocks | UART1 0x40013800, UART2 0x40004400, UART3 0x40004800: CR2(+4).SOURCE9:8 must be0/1=PCLK or2=LSE, never3=LSI. All three instances exist in both families. | 329–334,343,355–357 | 267–272,281,293–295 |
| HSE runtime fault detector | CR1.HSECCS7=0; conservative new-start route also requires HSEEN1=0 and HSE/ISR stable signals0. HSE detector interval is DETCNT/fLSI. | 59–60,71 | 57–58,69 |
| LSE runtime fault detector | CR1.LSECCS6=0; new-start route also requires LSEEN4=0 and LSE/ISR stable signals0. LSE detector uses256 LSI cycles and128 LSE edges. | 59–60,71 | 57–58,69 |
| MCO | SYSCTRL_MCO(+0x70).SOURCE3:0 must be a defined0–9 code other than4=LSI. Check selection even if no output pad is presently configured. | 61,94 | 59,92 |
| Direct LSI output | GPIOB0x48000400 AFRH(+0x14).AFR11[15:12] must be a defined0–7 code other than1=LSI_OUT. Check independently of DIR/ANALOG/lock and package pin use. | 147–148,154 | 144–145,151 |
| GPIO interrupt filters | GPIOA/B/C/F bases0x48000000/0400/0800/1400: FILTER(+0x40).FLTCLK[18:16] must be a defined0–6 code other than5=LSI. Check regardless the per-pin FILTER enables or IRQ enables. | 143–149,152,158 | 140–146,149,155 |

Reasons the root predicates are broader than a peripheral's obvious enable:

- RTC START only controls the calendar. RTC AWT can use RTCCLK/2,/4,/8,/16 without requiring calendar START; the START requirement is attached to the RTC1Hz branches. RTC_1Hz and RTC_OUT also have downstream pad/event users. SOURCE!=LSI cuts the direct LSI root for all of them without probing a running calendar's time/date, unlocking RTC or changing ACCESS/WINDOW. CR1 must be readable to perform the documented WINDOW poll itself; selector inspection does not enter the time/date access transaction.
- UART's source is used in synchronous half-duplex clock output as well as asynchronous RX/TX, single-wire, address recognition, RTS/CTS flow control, DMA and low-power receive. TXEN=RXEN=0 is not used to infer that every auxiliary or retained observer is absent. Checking SOURCE on all three is both simpler and more conservative. No RDR/TDR/ICR read/write is needed.
- GPIO digital filters can operate during sleep/deep sleep and produce edge/level IRQ effects. MCO and LSI_OUT can feed external devices, another GPIO, timers and their downstream trigger/capture/reset chains. Cutting the direct source selectors removes a need to open downstream timer workgates just to inspect their state.
- HCLK/PCLK-driven peripherals and outputs will intentionally change rate when SYSCLK changes. They belong to the general initialization/bus-clock functional handover, not a claim of uninterrupted inherited timing. No runtime switch beneath constructed HAL drivers is introduced.

Other roots were checked, not merely omitted:

- IWDT uses a **separate RC10K oscillator**, not shared LSI: x030 PDF312–313 §§16.1/16.3.1; F020 PDF250–251 §§15.1/15.3.1. Its watchdog deadline remains active and is not frozen/reconfigured by this work.
- RC10K/RC150K are independently identified in the clock tree/narrative (x030 PDF46–47; F020 PDF44–45). GPIO's choice of LSI is separately explicit in the GPIO chapter, even though the clock-tree shorthand emphasizes the RC clocks.
- PLL inputs are HSE or HSI, not LSI (x030 PDF54; F020 PDF52). The cold path can therefore admit a legal HSI-fed PLL if all other stopped predicates pass. An inherited HSE-fed PLL cannot pass the conservative external-source-off predicate, whereas it can use the running factory-LSI reuse route.
- These classic own manuals enumerate no AUTOTRIM, LCD or LPTIM peripheral. Their complete LSI mention/clock/mux sweep reveals no additional direct LSI engine. Do not import another family's AUTOTRIM/IWDT semantics.
- RTC expressly instructs software to enable/start LSI/LSE and wait stable first (x030 PDF178; F020 PDF175); external fault detection expressly needs software LSIEN. No documented UART/AWT/GPIO/MCO independent LSI auto-start entitlement was found. The system-clock-selected oscillator is protected against stopping, so LSIEN alone is still insufficient. The design uses complete source selection and both stability signals; it does not invent an undocumented BUSY register.

This is an exhaustive roster in the reviewed own-source model. It does not prove what arbitrary external software, a debugger or an attached external device intends to observe. Those are the explicit functional handover and low-level platform boundaries above, not an assertion of source-level hardware invisibility.

## 5. Gate protocol and the whole-bank interval

Own §4.3.9 says a configuration clock services CPU peripheral-register reads/writes and explicitly distinguishes RTC/UART/AWT/FLASH from other workgates (x030 PDF56; F020 PDF54). GPIO chapters say registers use AHB and configuration first opens GPIO AHBEN (x030 PDF143/151; F020 PDF140/148). These are positive reasons to enable a gated configuration path. No source guarantees valid GPIO register reads with its clock closed.

Gate facts:

- RTC APBEN1(+0x38).bit3; UART2 bit7; UART3 bit8. UART1 APBEN2(+0x34).bit9; AWT bit13. These are configuration-only gates (x030 PDF83–84; F020 PDF81–82).
- GPIOA/B/C/F AHBEN(+0x30).bits4/5/6/9 control configuration **and work** (x030 PDF82; F020 PDF80).
- AHBRST/APBRST1/APBRST2 are active-low module reset controls (x030 PDF85–87; F020 PDF83–85). Reject reset-held blocks for this inspector; do not pulse or release a reset to make values readable.
- Classic AHBEN/APBEN1/APBEN2 have **no KEY field**. Apply their own unkeyed neighbor-preserving RMW. CR0/CR1 require KEY0x5A5A; LSI has no key. A generic “keyed restore” helper must honor its per-register metadata, not inject a key into reserved high bits of an unkeyed classic gate.

For each qualified config-only block, save the incoming gate; open if needed using bounded readback; read only the required control; restore the exact incoming gate with bounded readback. Restore even when enable readback failed. If restore fails, report it distinctly enough to say a gate may remain active; no LSI TRIM write is allowed afterward. Never rewrite the whole saved gate word, because neighboring enabled peripherals must remain untouched.

For GPIO the mechanical steps are the same but their semantics differ. Open one bank at a time, inspect FILTER plus PB11 AFRH where applicable, then restore. Preserve every pin configuration, lock, ISR/ICR and filter field. The full bank can progress during the interval, whether the root selector subsequently passes or fails. Do not label it “read-only hardware state” or claim restoration rolls back events. No timer, DMA or other dormant workgate is opened to prove an observer idle. The public Sysclk::LSI and init documentation must state this exact whole-bank behavior; detailed RCC documentation should explain the source roster and failures.

## 6. Executable admission and sequencing

This is algorithm design, not proposed implementation code or a test harness.

### 6.1 Pure configuration stage

Validate own-family selected-LSI bounds, declared supply/temperature, AHB/APB divisors, timeout and any existing HSE/LSE/PLL combination rules before singleton acquisition. The existing 1 MHz Embassy time-driver preflight runs on the exact rational selected PCLK and rejects LSI before `Peripherals::take`. It is not delayed to a hardware error.

### 6.2 Classify without modifying CR1

Inside the existing whole-init critical section, capture current legal source, CR1, LSI raw word, stable mirror and required owner controls. Read one aligned volatile u16 at0x00012602. Reject raw0xFFFF before masking, then use bits9:0 as the factory trim. Do not use a u32 read crossing another calibration item; do not reject valid zero merely because it is zero; do not compare WAIT/reserved bits with a factory word.

- A mismatched source with LSIEN1, either stable indication1, or SYSCLK=LSI rejects immediately. It is not stopped to fix it.
- A matching source with LSIEN1 can be waited to matching stable indications1 without modifying TRIM/WAIT. A matching currently selected LSI with LSIEN0 is also effectively owned/running, because selected-source stop protection exists; it can be reused and its software request later asserted. This exception never qualifies a trim write.
- If LSIEN0 and neither stable indication is set and SYSCLK is not LSI, use full stopped admission. Disagreeing status signals, reserved selectors or a nonmatching active state reject. No “STABLE means calibrated” shortcut.

### 6.3 Complete stopped admission, twice

Define a global predicate containing: legal non-LSI/non-HSE/non-LSE SYSCLK; LSIEN0; both LSI stable signals0; unchanged original LSI TRIM/WAIT and raw preserved bits; HSECCS0 and LSECCS0; HSEEN0 and LSEEN0; HSE/LSE register and ISR stable signals0; IER.LSIRDY0; ISR.LSIRDY0; NVIC RCC pending0. Confirm the other controls relevant to an admitted HSI-fed PLL remain unchanged.

Perform two complete passes. Before and after each pass check the global predicate. Within each pass inspect all RTC/AWT/UART/MCO/GPIO roots using the gate protocol. Save/compare the relevant selectors and gate/reset state, or re-evaluate all predicates; changing state or a failed restoration rejects. No finite number of passes substitutes for excluding autonomous register writers. Double checks protect the check/use edges and catch changed status; their sufficiency relies on the own hardware model and platform ownership, not a belief that critical sections stop hardware.

A stopped matching TRIM still takes this same new-start path, but skips the unnecessary write. This prevents an accidental factory match at reset from admitting a pending ready observer or dormant LSI-selected consumer.

### 6.4 Program only the ten-bit trim field

After the last successful predicate, masked RMW of LSI.TRIM only. The LSI register has no key. Preserve original WAITCYCLE bits11:10 and all reserved bits; do not copy the SDK full-register assignment or force258 cycles. The four WAIT values6/18/66/258 are all documented (x030 PDF53/74; F020 PDF51/72).

Before enabling, bounded readback must prove factory TRIM, unchanged WAIT, unchanged preserved raw bits, and still-low stable signals. An unexpected write/readback rejects without enabling/pretending rollback. Do not restore the old TRIM after an enable attempt or otherwise retune a possibly started source. The successful trim write may remain even if a later phase fails.

### 6.5 Enable, bridge and finalize

Keep the stopped admission and trim stage ahead of the first CR1 write, including any existing HSI startup or LSE monitor setup that writes CR1. Existing Flash and bus guards can be established before the first source-enable write. They must cover the actual incoming source and HSI bridge/fallback envelopes. No requested external source starts before LSI is ready.

The first and every subsequent CR1 write includes key0x5A5A, mandatory CLKCCS/HSECCS/LSECCS=1, neighbor preservation and permanent target LSIEN. The first request also retains/asserts HSIEN. Subsequent writes keep HSI requested except for the existing separately admitted HSI factory-calibration stop while executing on the now-qualified LSI; do not forbid that existing required stop or weaken its admission. Before enabling the new source, recheck the admission-critical snapshot with expected TRIM advanced from the original value to the admitted factory code after a successful write. Original WAIT/reserved bits and all global/root/observer predicates stay unchanged; no preceding stage may have enabled a previously rejected owner. Rechecking against the old pre-write TRIM would incorrectly reject every actual calibration and is not the algorithm. Wait with the configured poll budget for LSIEN readback and both LSI.STABLE/ISR.LSISTABLE high, while verifying TRIM/WAIT unchanged.

The existing HSI factory-calibration admission is a separate obligation and must not be weakened. If it needs the temporary LSI bridge, it now uses the admitted factory LSI and must not withdraw the target's software request afterward. Preserve inherited AWT/RTC/clock owners and do not silently retune a shared HSI to make the new source work. Keep HSI available/calibrated after the final selection.

Leave any PLL through a confirmed unchanged HSI intermediate: PLL→HSI→LSI is permitted; direct PLL→LSI is prohibited (x030 PDF62/67; F020 PDF60/65). The stopped cold path's extra external guard restricts which inherited PLLs can enter that path; it does not change the hardware transition graph. Do not add an unconditional stop of an inherited shared source as part of this LSI feature. Existing claimed source consumers and output ownership must be respected.

Set SYSCLK=3, verify selector readback and barriers, then apply final AHB/APB divisors/Flash envelope according to the existing ordering. Final verification includes factory TRIM, unchanged WAIT and preserved parameters, LSIEN1, both ready indications, selectedSYSCLK3, HSI retained/readback-qualified, mandatory CCS1 and correct dividers. Publish `CLOCKS` only after all checks pass.

Flash may use WAIT0 for the actual slow target only when every reachable retained/transition/fallback envelope permits it. Retaining the conservative existing wait is safe; blindly lowering to WAIT0 because requested SYSCLK is32.8 kHz is not. Own Flash limits are x030 PDF112 and F020 PDF110; own switch procedure is x030 PDF63/65 and F020 PDF61/63. LSI has no documented automatic loss fallback. HSE/LSE monitoring/fallback and their retained configuration remain independent features, not a new LSI recovery guarantee.

## 7. Concurrency, pending state and failures

PRIMASK masks ordinary interrupts, including the external FAULT vector, but not all exceptions (x030 PDF96–98; F020 PDF94–96). It does not stop GPIO input edges, oscillator startup, detectors, peripheral state machines, DMA or debugger writes. NMI is marked unused by these own vector tables; that is not a license to rely on arbitrary low-level exception code or memory-corrupting masters being inert.

No reviewed peripheral auto-start path can defeat the stopped proof once the source selectors and detectors have been excluded and register writers are outside the supported entry model. The relevant source controls are software writable and are rechecked; the only source-selected stop protection is explicitly excluded. Hardware flags may still progress for unrelated devices during inspection. Preserve them, and acknowledge those effects in the functional handover. Do not add global flag clearing, interrupt masking, IRQ priority changes, resets, or DMA gating to make admission appear clean.

The NVIC RCC pending test is deliberately conservative: a global RCC pending bit does not identify which source caused it. Reject instead of clearing an unrelated owner's notification. IER.LSIRDY=0 is required on a new start even when NVIC RCC is disabled, because a ready flag is an independently visible observation. Starting LSI normally creates LSIRDY; leave the newly generated flag set. Arbitrary polling code's expectations are not introspectable, so the API must promise the stated start/flag behavior rather than pretend no software can observe it.

Failure state must be explicit:

- Pure configuration/time-driver error: no singleton taken and no hardware touched.
- Failed owner admission: no TRIM/source request/switch write; inspection gates may have briefly run, and a restoration error can leave one enabled. Events already produced remain.
- Factory-write readback failure: attempted TRIM may remain; no success/publication and no unsafe cleanup write.
- Startup/configuration/switch timeout after request: retain the permanent LSIEN request and preserve unrelated controls/flags, publish no clocks, return no new peripheral tokens, and retain the current one-time-init failure/reset requirement. HSI normally remains requested, but an error inside the separately admitted existing HSI calibration sequence may leave HSI stopped or incompletely restarted while the CPU is on LSI. Do not promise HSI is ready on every failure, or blindly re-enable/retune it to manufacture rollback. Successful finalization requires HSI enabled and qualified. An enable request can complete after the poll budget expires; cleanup must not stop that source.
- No wait is a microsecond deadline. At LSI-derived CPU speed the same poll budget takes longer; after loss of the currently executing source no software polling loop can guarantee progress.
- STABLE reports successful startup and is explicitly unchanged by later clock loss. Neither compare/readback nor frozen bounds certifies continuous physical oscillation.

## 8. Rate qualification and driver boundary

| Family | Own electrical source | Nominal | Factory full-temperature interval |
|---|---|---:|---:|
| F030 | own DS Rev1.9 PDF46 Table7-18 | 32,800 Hz | 31,816..33,784 Hz (±3%) |
| A030 | own DS Rev1.1 PDF43 Table7-17 | 32,800 Hz | 31,816..33,784 Hz (±3%) |
| F020 | actual own DS Rev1.3 PDF44 Table7-18 | 32,800 Hz | 31,160..34,440 Hz (±5%) |

These factory rows use the qualified supply1.65..5.5 V and ambient−40..105°C. Keep the retained-HSI operating qualification too; a slower selected source does not release retained HSI's conditions. Own bus electrical tables have no positive minimum: F030 PDF38, A030 PDF35, F020 PDF36. Separate narrower25°C accuracy must not silently replace the full declared range. The legal manually trimmed32.8kHz±10% operating range is not the factory accuracy envelope.

Recommendation: make this new selected-source capability **rate-only**. The own tables give frequency calibration accuracy, duty cycle and startup figures, but no independently established absolute per-cycle timing/jitter bound. Do not claim that low-frequency RC must be cycle-qualified just because existing HSI/RTC constructors currently expose such helpers. This is a conservative feature contract, not an assertion that its cycles are physically unbounded.

Use a source-specific LSI constructor/metadata; do not globally alias `ClockBounds::rtc_source()`, because other admitted families' RTC source is HSIOSC. On these same three families, align `LsiClock::bounds()`, `CalendarClock::Lsi` and RTC source/calendar-tick bounds with the same rate-only classification, even when SYSCLK happens to be HSI. The existing `rtc_source()` currently constructs `rate_only=false`; leaving that alias unchanged would expose contradictory strict-cycle qualification for the identical factory oscillator. This narrowly scoped consistency change is part of shipping the new policy, not an extension to the other ten families or to native HSIOSC RTC. Calendar rate/count conversion may remain supported, but must not relabel it an independent every-cycle guarantee. Preserve exact numerator/divisor values through AHB/APB division and outward public rounding. Extend the `rate_only` capability cfg consistently if it ceases to coincide with `rcc_pll`; the three current families have the PLL capability but that is not the reason LSI is rate-only. Change PLL-specific diagnostic wording where the generic failure now covers LSI too.

Consequences:

- Existing time-driver at1,000,000 Hz rejects selectedLSI before singleton acquisition for every AHB/APB divisor. Do not add a fractional divider, alter tick frequency or imply RTC-based time continuity.
- Classic ADC's `has_cycle_timing_bounds` guard and classic complementary-PWM/dead-time guard must reject this rate-only source before use. Existing per-driver rate/minimum/divisor/electrical limits remain. Do not relax these merely to make an example initialize.
- UART/SPI/I2C/timer ordinary rate representability remains each driver's responsibility. Default baud rates may fail at32.8kHz. Existing independent HSI-driven AWT and RC10K IWDT keep their own sources/bounds and state.
- No sleep/resume, arbitrary calibrated LSI, LSE SYSCLK, runtime source switch, transparent fault recovery, or RF/silicon qualification is included.

## 9. Review gates before implementation

No missing-BUSY blocker prevents this **bounded, conservative** cold path after the clarified whole-bank functional handover. The strict no-workgate-opening alternative lacks the GPIO selector evidence for normal reset and must be described accordingly. The actual requirements to settle in review are precise:

1. Accept the public whole-GPIO-bank inspection behavior, including events before failure, without converting it into an unsafe-Rust precondition or a risk checkbox.
2. Bind each family to the exact selector/gate/reset facts and source hashes above; validate all GPIO banks/all three UARTs in generated metadata instead of copying the other-family monitor roster.
3. Place stopped admission/TRIM before the first mandatory-CCS CR1 write, with inherited external enabled/stable states excluded; initialize requested external clocks only after LSI stable.
4. Keep rate-only classification and corresponding strict-driver rejection coherent; no independent per-cycle guarantee has been sourced here.
5. Retain detailed failure phases and permanent source requests, and do not confuse gate restoration with functional rollback.

The accepted `prepare_lse_monitor` code in `l031_r031_w031.rs` and `l052_l083.rs` was used only as an algorithm/design reference. Its own-family qualification cannot establish classic hardware facts, and this design deliberately adds the classic whole-GPIO roster, conservative external-start sequencing and pending RCC observer handling on their own evidence.


## 10. Narrow qualification compatibility audit (revision 2)

This policy intentionally changes the public timing qualification for the **same factory LSI on only F020/F030/A030**. It is a compatibility change and must be called out in the eventual change notes:

- `rcc/rtc.rs:125` `LsiClock::bounds()` currently returns `ClockBounds::rtc_source()`. That constructor in `rcc/bounds.rs:95` currently sets `rate_only=false`. The common `CalendarClock::bounds()` at `rcc/rtc.rs:175` forwards the same value, and `rtc/mod.rs:261` / `:266` expose source and divided calendar bounds. All these three-family paths must consistently report rate-only after the new policy. This change applies even when the system remains on HSI, because it is the same physical LSI source qualification.
- Their public `has_cycle_timing_bounds()` consequently changes from true to false. Calling strict `minimum_duration_ns` / `maximum_duration_ns` on those returned LSI bounds without checking qualification will now fail their existing assertion. This is an explicit withdrawal of an unsupported strict guarantee, not a silent API-preserving change. Nominal/minimum/maximum rate, supply/temperature intervals and exact32800/32768 calendar rate are retained.
- No other family's LSI, native HSIOSC RTC, HSI SYSCLK or board-qualified HSE/LSE qualification is silently changed by this three-family slice. Broader source policies need their own review. `LsiClock::new` remains its existing post-init compare-and-enable capability; stopped-source calibration belongs to the new init route, not a general live RTC constructor.

Actual existing RTC consumers were searched in the current candidate:

1. `rtc/mod.rs` uses `clock.bounds()` for the two public getters above, and for rate-ceiling checks in native initialization/attach (`:440` and `:488` branches). The latter are native L011/L012 or native HSIOSC/LSE code, outside this three-family change. It contains no call to the strict minimum/maximum-duration methods.
2. Classic RTC acquisition/attach/read/set operations check source identity, stability, trim, write/access ownership and compensation status. The access-poll delay at `rtc/mod.rs:361` uses the **selected HCLK** `delay_cycles_us`, which is already a rate-derived helper and does not require strict per-cycle bounds. It must remain described as such; no new real-time wait guarantee follows.
3. The three `examples/shared/rtc-calendar/src/bin/{preserve_calendar,initialize_calendar,set_calendar}.rs` programs pass `calendar_tick_bounds()` to `black_box`; they do not call strict duration methods. `examples/shared/lse-clock/src/lib.rs` and `examples/shared/l010-lse-clock/src/lib.rs` similarly inspect bounds, and their LSE source qualifications are unchanged. No in-tree RTC example was found that needs cycle-qualified LSI to compile or perform a calendar operation. This is a static source audit, not a build/run result, and does not cover downstream user code.
4. Existing prose in `rtc/mod.rs:264` and `docs/rtc-remaining-calendar.md:50–55` promises exact duration methods for classic LSI ticks. Amend it explicitly for the three newly rate-only families; do not leave that promise while changing the return capability. Calendar nominal-rate drift (32800/32768) is still meaningful, but is not an absolute per-cycle bound.

Actual strict-driver/cfg paths requiring coherent coverage:

- `rcc/bounds.rs`: the `rate_only` field, source constructors and `has_cycle_timing_bounds()` currently depend on `rcc_pll`. Every admitted new LSI profile must carry the qualification even if PLL cfg later differs; use an explicit selected-LSI capability in the cfg expression or another coherent common representation. The three present classic profiles sharing PLL is an incidental implementation fact, not proof of coverage.
- `adc/classic.rs:190` (`UnqualifiedCycleTiming`), `:229` (its display arm), and `:342` (the guard in `Config::timing`) are all currently `#[cfg(rcc_pll)]`. Extend the full trio consistently for the new LSI capability. `Config::timing` is reached by blocking construction `:425`, async construction `:703`, sequence/read timing `:525` / `:794`, and both `set_config` paths `:669` / `:1037`. The early guard must remain ahead of cycle-duration calculations/hardware changes; diagnostics must say rate-only source, not specifically PLL.
- `timer/complementary_pwm/classic.rs:169` already checks `has_cycle_timing_bounds()` without PLL cfg before selecting timing/dead time or enabling RCC. It therefore rejects LSI as intended, including zero requested dead time. Its public prose around `:150` currently says PLL-derived and must become source-generic. F020 has no ATIM capability to invent; the guard remains relevant where classic complementary PWM exists.
- `time_driver.rs` exact-divisor validation and `lib.rs:183–189` configuration-before-singleton order remain unchanged in meaning. SelectedLSI must fail the1MHz check even though its rates are valid for CPU/RCC operation.

No implementation or Cargo validation of these prospective changes has been performed. These are the exact paths to cover in the separately authorized implementation; the scope does not expand to a generic all-HAL timing rewrite.
