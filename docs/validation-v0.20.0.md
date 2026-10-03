# v0.20.0 verification

This release adds internal ADC sources, WWDT, RTC/low-speed startup and a
reserved-partition Flash driver. Verification uses Rust 1.99.0 and the real
`thumbv6m-none-eabi` target. No physical board was programmed or exercised.

## Build and source checks

- Workspace formatting and YAML → persisted JSON → PAC regeneration/drift.
- Both PACs with runtime/metadata and both HALs with minimum/full features,
  using release ARM code generation.
- Both minimum/full ARM rustdocs with `-D warnings`.
- Fourteen individually enabled optional-feature combinations across the chips.
- All six release board examples; 05/06 additionally with motor output enabled.
- Both real linked ARM integration ELFs include internal ADC, low-speed/RTC,
  WWDT and Flash operations alongside the preceding bus/CRC/IWDT drivers.
  All 32 IRQ vectors match metadata, selected bus/ADC/GTIM handlers are strong,
  and stack/Flash/RAM bounds pass. Flash load images stay below the integration
  probe's explicitly reserved data page; the probe is never run on a board.

The existing `proc-macro-error2` future-compatibility notice is the only upstream
notice in the aggregate build. There is no new checked-in test project,
Python dependency, generated source or lockfile in the release archive.

All motor API and example Rust source files compare byte-for-byte with v0.19.
The six examples still contain no direct PAC accesses or unstable-PAC feature.
Their original sampling, commutation, fault policy, UART divisor and executor
priorities are preserved. IWDT/WWDT exports follow independent peripheral-presence cfgs. Their generic
Instance owners use generated registers/clocks while retaining the same key,
window and lifetime protocol. EAU/CORDIC and DAC instance corrections are also
included, as described in their focused architecture records.

## Focused external verification

| Area | Executed checks |
| --- | --- |
| ADC internal sources | 13 register-model groups: source mapping/activation at START, source delays, L012 sample timing including a fractional-divider boundary, async completion/cancel/forgotten reads, retained DAC1/2 mapping on both L012 ADCs, F030 MODE0/BUF and preserved BIAS/reserved fields, active/poisoned DMA rejection before MMIO |
| ADC API | 7 positive and 20 intended rejection probes: source borrowing and sealed methods, source/ADC reuse, unsupported channel erasure/scan/trigger/DMA paths |
| WWDT | All 65,536 reload/window pairs and all 128 live counter values; legal 0x40/WINR boundaries, no-write early/late rejection, CR1→SR→CR0 ordering, inherited EN/IE preservation, Drop clock retention and manual cycle formula |
| RTC calendar | All 36,525 days in 2000–2099 and 3,506,400 calendar/BCD round trips, weekdays and malformed input |
| RTC protocol | Both IPs: untouched attach, stopped-only initialization, divider/source policy, unlock/relock, WAIT/WINDOW timeout, ACCESS readback/cleanup, stop/write/readback/restart, simulated midnight retry and retained reattachment |
| Low-speed clocks | Both IPs: keyed enable, unchanged LSI trim/WAIT, LSE analog-before-enable, untouched retained adoption, settings/pad rejection, same-token readiness retry and no Drop shutdown |
| LSI startup audit | Generated function bodies: all documented L012 automatic requests, F030 GPIO requests, software/CCS/STABLE early exits without writes, final observations, exact gate restoration, keyed writes; F030 RC150K users do not suppress LSI trim |
| RTC/WWDT ownership | 2 positive and 20 intended rejection probes: wrong LSE pads, non-static crystal borrows, forged source proof, missing clock token, duplicated RTC/WWDT tokens, wrong watchdog instances and nonexistent WWDT stop |
| Flash protocol | Bounds/overflow/alignment/empty requests, inherited BUSY clock retention, protected-region rejection, keyed cache/WAIT/IER/PAGELOCK restoration, injected cache-disable/unlock/invalidation failures, variant status/error handling |
| EAU Instance | Inferred/generic constructors, 5 ownership/identity rejections plus absent-IP rejection; 10 actual production protocol/PAC groups and byte-identical operation/error/poll algorithms |
| CORDIC Instance | Real typed handler/Binding, 7 compile rejections, per-instance event/result/clock identity; 18 current-source lifecycle cases, 16 matching baseline cases and 2 cross-instance isolation cases |
| DAC Instance | Typed owner/channel/source and generated peripheral+channel pairing; 26 ownership/route API cases plus 4 composition regressions; 16 register/resource scenarios including a distinct synthetic DAC address/clock |
| Flash API | Both chips accept unsafe partition construction, inherent operations and ReadNorFlash; safe constructor calls and writable NorFlash bounds fail intentionally |

An independent read-only review authored and ran the WWDT, RTC, Flash and
generated LSI probes. It also rebuilt and reran selected ADC and low-speed
checks and the calendar probes. The Flash register model does not emulate the
actual volatile program/erase effects. Neither those models nor ARM compilation
prove power-loss containment, analog accuracy, crystal startup reliability or
physical reset/interrupt timing.

## Data and package boundary

Four sourced LSE pad routes are added, giving 261 L012 and 225 F030 chip routes.
F030 FLASH.ICR.PROG hardware access is corrected to write-only from RM2.5
§7.9.6. Register layouts, reset values, arrays and enum values otherwise remain
unchanged; schema10 and the actual persisted-JSON boundary are preserved.
Flash geometry is read from chip memory metadata and checked against the
audited main-array geometry. Generated value helpers remain local word access;
write-only status must not be inferred from them.

The source-only package was rebuilt in a separate clean source directory,
starting without generated artifacts or Cargo.lock. The full two-chip release,
strict-doc and six-example/default-plus-motor matrix passed again after the
Instance corrections. All 48 normalized JSON files and 102 generated
PAC/metadata/runtime files match the maintained tree byte-for-byte. Final source
entries are checked against both trees and the archive; build targets, generated
files, lockfiles, probes, logs, caches and Python/scaffolding are excluded.

## Remaining limits

Flash operations wait for hardware completion without an abort or wall-clock
timeout, and can violate real-time interrupt budgets. Writable NorFlash is not
implemented because its power-loss containment guarantee is unproven. RTC
supports calendar access and nominal low-speed clocks, without alarm/IRQ,
compensation or low-power time integration. ADC returns raw codes, not calibrated
temperature or supply voltage. WWDT refresh still needs timing margin.

The [fourth-stage scope](analog-rtc-flash-v0.20.0.md) tracks the original eight
gaps and subsequent timer/DMA/system-clock work. No STM32-wide parity or physical
validation is claimed.
