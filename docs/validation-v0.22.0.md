# v0.22.0 verification

The HSE/F030 PLL startup implementation uses Rust 1.99.0 and the real
`thumbv6m-none-eabi` target. No physical board was programmed or exercised.
Runtime reclocking and DeepSleep were audited, not implemented or hardware-tested.

## Integrated software checks

- Formatting, source regeneration and YAML → persisted JSON → PAC drift checks.
- Both PAC release builds with runtime/metadata, and both HAL minimum/full ARM
  release builds.
- Both minimum/full ARM rustdocs with `-D warnings`.
- Fourteen individually enabled optional-feature combinations.
- All six release board examples and 05/06 with motor outputs enabled.
- Linked ARM integration images include the real HSE/PLL initialization bodies
  alongside UART/SPI DMA, capture, QEI and complementary PWM: L012 HSE crystal,
  F030 PLL with HSE crystal, HSE bypass and HSI input. Device IRQ vectors,
  selected strong handlers, stack and memory bounds are checked.

All 29 motor API and example Rust files remain byte-identical to v0.21.
They retain HSI defaults and the 96 MHz L012 motor profile. The only existing
upstream build notice is `proc-macro-error2` future compatibility.

## Independent focused verification

| Area | Executed evidence |
| --- | --- |
| Clock configuration and sequencing | 528 L012 and 381 F030 valid source/divider profiles using extracted actual backend source and real PAC value types; includes every historical HSI/AHB/APB combination |
| Numerical rejection | Supply/clock/drive/wait/PLL/source limits and fractional external clock cases reject before any mocked IO; exact rational voltage bounds and integer timebase constraints |
| MMIO order and faults | Flash waits before faster transitions, bus divisors/readback, HSE/PLL enable and readiness limits, all injected control/parameter/switch failures, preserved reserved fields and F030 PLL debug nibble |
| Full backend preflight | Invalid configurations return with unmapped peripheral addresses; six L012 and eight F030 retained/reset-incompatible states reject without writes or reaching calibration memory |
| Global initialization and pads | 160 external cases using production wrapper/config/AnyPin operations and generated pad code: retryable pure rejection, failed/panicking hardware attempts consume acquisition, no token/clock publication on failure, crystal/bypass/HSI token shape and preservation of every unselected pad bit |
| ARM ownership/API | 18 expected compile outcomes: 7 positive and 11 intended rejections for optional pins, moved/unsafe ownership, reserved GTIM1 and absent L012 PLL |
| Metadata independence | Relocating HSE routes changes generated pad/token behavior; missing, duplicate and aliased routes reject generation instead of assuming PF0/PF1 |
| Physical evidence | Both PF0/PF1 routes corroborated by datasheets and chip-specific SDK; the L012 RM example and SDK bypass-port discrepancies are recorded explicitly |

The external initialization fixture models register access and backend outcomes;
it does not emulate analog oscillation or ARM electrical behavior. Backend
preflight checks separately run production initialization up to its first
calibration access. Independent reviewers checked final clock/ownership source
and evidence without editing the implementation.

The source-only candidate was rebuilt in a separate clean directory, starting
without generated artifacts or Cargo.lock. The complete both-chip release,
strict-doc and six-example/default-plus-motor matrix passed. All 48 normalized
JSON and 102 generated PAC/metadata/runtime files match the maintained tree.
Every final source entry is compared with the maintained tree, clean source and
archive. Targets, generated outputs, lockfiles, probes, logs and caches are
excluded. No verification scaffold or Python dependency is added.

## Preserved data and limits

Schema remains 11. Four dedicated HSE pin routes are added across both chips,
giving 263 L012 and 227 F030 routes. Register layouts/defaults/arrays/access
contracts are unchanged. PF0/PF1 become Option<Peri> in the generated HAL
collection to prevent active oscillator/GPIO aliases. Pure profile/time errors
precede MMIO; a hardware-attempt failure requires reset before another init.

Software checks do not prove actual supply voltage, external frequency/duty,
crystal loading, startup yield, PLL lock timing, automatic fault recovery or
DeepSleep restoration. Nominal frozen frequencies depend on a healthy source.
The [clock scope ledger](clock-roadmap-v0.22.0.md) and
[runtime/low-power audit](runtime-low-power.md) state the remaining limits.
