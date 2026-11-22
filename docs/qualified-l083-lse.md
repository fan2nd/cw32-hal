# Bounded active LSE on CW32L083

This source-backed candidate admits init-only nominal 32768 Hz crystal or bypass on CW32L083RBT6, RCT6, RCS6, MCT6 and VCT6. Family aliases remain unqualified. It uses direct typed native PAC fields, generated exact-package facts and the existing central RCC initialization path. Candidate source review and build validation are separate from this source qualification; no silicon or board execution is claimed.

## Own source authority

Four originals are byte-verified in [the original receipts](lse-l083-source-receipt.json), with five directly recovered members in [the SDK receipts](lse-l083-sdk-member-receipt.json). The source authority lock is unchanged. All page citations below are 1-based PDF / printed pages.

- [CN UM V2.0](https://www.whxy.com/uploads/files/20240920/CW32L083_UserManual_CN_V2.0.pdf), SHA-256 9930bf1755f3bbf8933163c2d0da57fd9a4f3250a358a4c0bfc75ed4eda3a0a3. Cover 2024-09; revision history 2024-07-24.
- [CN datasheet V1.9](https://www.whxy.com/uploads/files/20251229/CW32L083_DataSheet_CN_V1.9.pdf), SHA-256 852f772e9174cb76bf0f475f31f1e275254f8fe176bd3e7ad60d00b41db9509e. Revision date 2025-12-29.
- [SDK V2.2](https://www.whxy.com/uploads/files/20240821/CW32L083_StandardPeripheralLib_V2.2.zip), SHA-256 2d58765568d8dd8a218b52e4650aa6e5bd4f2e4b5f386196bae637dfc0b3fe73, release note 2024-08-21.
- [EN UM V1.0](https://www.whxy.com/uploads/files/20240923/CW32L083_UserManual_EN_V1.0.pdf), SHA-256 720c37dfe4d67e70f74f08135bf712915631edd286667e64987ffbf7878583b7. Revision date 2022-10-10; corroboration only, not the URL-directory date.

The [native fact record](lse-active-l083.json) binds exact parts, page sets, gate semantics, factory LSI envelope and package routes. The [RTC reset record](lse-active-l083-rtc-admission.json) binds all 13 readable register observations. Authored qualification is in `cw32-data/lse-qualified.yaml`; generation checks each own-source hash against the original authority and each fact against the selected native IR.

## Exact packages and electrical contract

Datasheet 10/9 and 86/85 qualify RBT6/RCT6 as LQFP64 10×10 mm, RCS6 as LQFP64 7×7 mm, MCT6 as LQFP80 and VCT6 as LQFP100. The admitted source envelope is 1.65–5.5 V and −40–85 °C. Datasheet 27/26 bonds PC14/PC15 (OSC32_IN/OUT) to pins3/4 on the 64/80-pin packages and8/9 on the100-pin package. UM56/55 requires both pads analog in crystal mode; bypass uses PC14 digital input and leaves PC15 available as GPIO.

Datasheet53/52 table7-14 gives bypass typical32.768kHz and max1MHz, high0.7–1.0×VDDIOx, low0–0.3×VDDIOx, high/low pulses≥450ns and rise/fall≤50ns. UM57/56 requires45–55% duty. Electrical tables are design-guaranteed rather than production-tested. Datasheet54/53 table7-16 characterizes only the32.768kHz crystal; its1.5s startup is typical with no maximum. Manual prose allowing crystal output up to1MHz does not expand the qualified nominal frequency.

The board supplies per-cycle minimum/maximum bounds over load, aging, supply, temperature and short-term variation, and separately qualifies analog drive/load or bypass electrical behavior. `STABLE` does not establish accuracy or uninterrupted availability. Poll budgets count attempts, not milliseconds or a crystal-startup maximum.

## Native register facts and consumers

UM81/80 defines LSE reset0x2B, DRIVER1:0, AMP3:2, WAIT5:4 (256/1024/4096/16384 cycles), MODE6 and STABLE15; bits14:7 and31:16 remain reserved. There is one analog bank. PDRIVER/PAMP are absent. All real parameters must be configured before enable; no phase-switch or post-enable tuning is inferred. UM76/75 gives keyed CR1, LSEEN4, LSELOCK5, LSECCS6, HSECCS7 and CLKCCS8; LSE enable/configuration reset only on POR. LSELOCK prevents clearing enable.

UM63–65/62–64 and83–85/82–84 distinguish current STABLE from latched LSERDY. LSEFAIL and LSEFAULT are sticky; ICR is R1W0. Runtime LSE detection requires enabled LSI. CLKCCS is system-clock behavior, not RTC source migration or continuity. The implementation never changes CLKCCS, clears flags, masks ready/fault IRQ owners, or attempts fallback/recovery.

UM61/60,87/86 and89/88 independently establish these gates:

| Consumer | Native gate | Gate effect |
| --- | --- | --- |
| RTC | APBEN1+0x38 bit3 | Configuration only |
| UART1 | APBEN2+0x34 bit9 | Configuration only |
| UART2–5 | APBEN1 bits7–10 | Configuration only |
| UART6 | APBEN2 bit1 | Configuration only |
| AUTOTRIM | APBEN2 bit13 | Configuration only |
| LPTIM | APBEN1 bit15 | Configuration and work |
| LCD | APBEN1 bit13 | Configuration and work |

Configuration-only gate inspection preserves the original gate state and performs no reset/start/stop writes. An off LPTIM/LCD gate remains off: enabling it merely to inspect could resume retained work. An enabled working gate is read without local writes; gate changes fail admission.

- All six UARTs select LSE with native CR2.SOURCE=2 and LSI=3; they can work with configuration gates off (UM370/369,396–397/395–396). L083 uses `source()`. The existing L052 `sorce()` spelling remains unchanged.
- AUTOTRIM SRC3 selects LSE, SRC1 selects LSI, and active MD1 implicitly consumes/calibrates LSI independent of reference SRC. Automatic, active-calibration or reserved state prevents freezing the detector. MD3 is timer mode (UM178/177,180–181/179–180,187/186). PC14 AF1 can be ETR; pad AF admission covers that conflict.
- LPTIM ICLKSRC2/3 selects LSE/LSI. Its ICLK also supplies filters and encoder logic, including with external counter input (UM238/237,247–249/246–248).
- LCD CLKCS1/0 selects LSE/LSI with EN active (UM544/543,562–563/561–562).
- RTC source0 is LSE,2 is LSI,4–7 are divided HSE. Calendar START does not cover raw-clock wake operation. COMPEN.EN is independently LSE-dependent: UM197/196 table12-3 shows compensated LSE1Hz with SOURCE='-'. Admission demands the complete reset-like RTC roster twice, controls first, without KEY/ACCESS/ICR writes or WINDOW waits. Own ALARMA reset is0x00120000 at208/207; L052's0x04120000 is not borrowed.
- SYSCLK LSE/LSI and MCO LSE6/LSI4 are consumers. Reserved MCO selectors fail stopped-LSI admission. All native GPIO FILTER LSI selectors are checked. The other watchdog/filter clocks do not create an invented LSI dependency (UM353/352,355/354,363/362).

LSE direct outputs are PB12 AF3 and PF1 AF1 on every admitted part. LSI routes are package-specific and checked against the complete own bonded AF roster:

| Package | LSI routes (physical position) |
| --- | --- |
| LQFP64 | PC4 AF6 (24) |
| LQFP80 | PC4 AF6 (28), PF2 AF4 (14) |
| LQFP100 | PC4 AF6 (33), PD5 AF6 (86), PF2 AF4 (19) |

Datasheet27–32/26–31 establishes bonding;35–38/34–37 gives AF values. The100-pin pin set is not assumed to contain every smaller-package pad. GPIO has native LCKR at0x3C, no SPEED at0x08 (UM163–172/162–171); admission uses the real `lckr()` field and never unlocks owner pads.

## Factory LSI detector and conservative margin

UM58/57,62/61 and79/78 define nominal32800Hz LSI, factory trim halfword0x00100A02, TRIM9:0, WAIT11:10 and STABLE15. The ±10% adjustable region is not factory tolerance. Own datasheet55/54 table7-18 gives factory ±3% over−40–85°C, yielding31816–33784Hz over the admitted1.65–5.5V envelope. The existing generated L083 RTC source facts supply exactly this maximum, and new own-source validation checks the same min/max, temperature and supply tuple. SDK SystemInit loads factory trim but is not evidence that another runtime executed it.

UM65/64 requires at least128 LSE clocks during256 LSI clocks. For newly admitted L083 configurations, the software adds a conservative sufficient condition:

`256 × declared_LSE_min_hz > 129 × own_factory_LSI_max_hz`.

The extra edge is counting-phase margin. With max33784Hz the smallest accepted integer minimum is17024Hz; nominal remains32768Hz and bounds must contain that nominal. Both products widen u32 inputs to u64 before multiplication, and their maxima are below2^40, so arithmetic cannot overflow. This is checked by `Lse::bounds` during `Config::frequencies`, before `init` takes peripherals or performs RCC MMIO. A rejected loose declared envelope is insufficient qualification, not observation of a failing oscillator. Earlier eleven parts retain their prior bound checks unchanged.

Live factory-valid LSI and WAIT are preserved. Erased factory evidence or a live mismatch rejects. A stopped mismatch requires repeated unchanged disabled/nonstable snapshots, no detector, no IRQ/history owner and no selected/active native consumer before a masked TRIM-only write. This is conservative software admission, not a vendor STOP-status guarantee. The accepted source freezes factory trim, wait, enable and stable requirements for its lifetime.

## Implementation and retained state

The accepted `LseStartupConsumers` schema already represents L083 faithfully: `startup_analog=false`, native AUTOTRIM/LPTIM/LCD facts, all six UARTs and exact-package LSI routes. No wire/schema change or adapter is necessary. Two generated cfgs express separate facts:

- `rcc_lse_native_consumers`: the own-qualified AUTOTRIM/LPTIM/LCD consumer graph (L052/L083), including native LSERDY/history admission and preserving the LSE register image.
- `rcc_lse_startup_analog`: actual PDRIVER/PAMP fields, present only on L052. It controls only the public startup parameters and typed field access.

The old eight nonnative parts have both false. The old three L052 parts have both true, retaining the prior branches and writes. Their authored JSON facts and per-cycle bound semantics remain unchanged. Only L083's own register gets three typed enum associations; existing HSI/HSE/PLL layout, access, enum facts and reuse membership are untouched. One bounded reuse-ledger entry records that exact enum-only delta.

`Config.lse=None` does not call the new admission, monitor or start paths. A requested stopped source requires idle pads/native consumers and no retained enable/lock/current-stable/ready/fault history or IRQ owner before any source mutation. Existing enabled LSE must match exact real parameters/pads and a healthy enabled factory LSI detector; retained LSE with CCS off is rejected, not repaired. Failed admission/start publishes no capability. Once enabled, timeout/fault retains sources, reservations and diagnostic flags. No live RTC migration, automatic fallback, arbitrary source reconfiguration or low-power promise is introduced.

## Source conflicts and limits

SDK `RCC_LSE_Enable` configures LSECCS/CLKCCS but never provisions LSI despite the manual runtime-detection prerequisite. Its `PUR &= bv14` can alter unrelated pullups and it adds an input pulldown. Neither sequence is copied. The EN WAIT table loses a leading digit for1024; the selected CN manual and own SDK agree on encoding1. The older EN AUTOTRIM MD discrepancy does not authorize calibration; CN187/186 and the SDK agree on timerMD3. No stopped-source write is justified merely by SDK SystemInit convention.

Own page pixels were inspected for the LSE field table, compensation SOURCE dash, the two gate tables, ALARMA reset, package/pad tables and oscillator electrical tables. Source checks, generated metadata/PAC verification and ARM links can establish software correspondence; none establishes oscillator behavior, board compatibility, frequency accuracy or a maximum startup time on silicon.
