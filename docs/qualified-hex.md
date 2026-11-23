# Qualified F002/F003 digital HEX

This bounded extension uses the own F002 CN V1.4 and F003 CN V2.3 manuals,
F002 CN V1.2 and F003 CN V1.9 datasheets, and their selected source identities
in `sources/evidence-sources.json`. Exact source pages, hashes, register facts
and conflicts are authored in `cw32-data/hex-qualified.yaml`. The prior
qualification receipt is `docs/qualified-hex-source-receipt.json`.

## Admission and public configuration

The family-specific RCC API adds `Config.hex: Option<Hex>`, `Config.sys:
Sysclk::{HSI,HEX}`, and typed `HexInput::{Pb0,Pb1}`. Both pads are digital
inputs. `Hex` requires nominal/minimum/maximum Hertz and explicit operating
conditions. No oscillator mode, crystal drive, startup-count or detector
setting is exposed. HSI remains the default and stays enabled.

Both RMs require 4–32 MHz while both DS tables allow 1–32 MHz; admission uses
the intersection, including the complete actual bounds. Therefore nominal
4 MHz with negative tolerance and 32 MHz with positive tolerance fail.
The conservative conditions are 1.65–5.5 V and −40–105°C, intersected with the
retained factory HSI qualification. At VDD below 1.8 V the actual HCLK/PCLK
ceiling is 24 MHz; at 1.8 V and above it is 48 MHz.

The board must simultaneously satisfy 40–60% duty, at least 15 ns high/low,
at most 20 ns rise/fall, high 0.7×VDDIOx..VDDIOx and low VSS..0.3×VDDIOx, plus
own I/O ratings. At 32 MHz a 40% duty clock fails the 15 ns requirement.
These waveform constraints cannot be checked by the initializer. DS waveform
characteristics are design-guaranteed, not production-tested. Declared
frequency bounds include temperature, aging, loading, supply and short-term
cycle variation; there is no made-up default ppm.

`ClockBounds` stores the selected external envelope independently of HSI.
Its SYSCLK envelope is divided only by AHB/APB, while `hsi_bounds()` retains
the factory oscillator and HSI divider. Raw-HSI consumers remain independent.
Flash waits use actual upper HCLK: nominal 24 MHz with ±30 ppm needs WAIT1.
The WAIT2 bridge encoding does not authorize a 72 MHz bus.

## Retention and transition

Legal entry is a stable, electrically legal HSI/HEX/LSI source, documented HSI
divider, suitable incoming Flash wait and continuous sources throughout the
transition. Unknown incoming trim cannot be qualified by STABLE. A trim bridge
requires unchanged LSI inside its documented 32.8 kHz ±10% range. Interrupts,
DMA and ordinary clock-dependent users must be quiescent, including NMI code.
This is one-time initialization, not arbitrary live handover or clock recovery.

1. Pure electrical/source/package validation precedes peripheral acquisition.
2. Validate entry selector, readiness and divider; read the factory trim.
3. Inspect retained AWT through central `RCC_INFO.inspect_for_init`, restoring
   its APB configuration gate without resetting or rewriting AWT.
4. Reject a necessary HSI retune while active AWT selects HSIOSC or active LVD
   filtering uses HSIOSC. No RTC exists on this pair.
5. With active AWT using either external input, a requested HEX must exactly
   reuse enabled, ready, matching PINMUX/PINEN and matching pad settings. No
   HEX or pad writes occur on that path. Otherwise reject before clock/pad
   mutation. Active AWT ETR rejects new HEX requests because its actual pad
   conflict has not been proved absent.
6. Enable Flash through central RCC, raise WAIT2, install monotonic AHB >=/4
   and APB /8 guards without weakening stronger inherited divisors.
7. Start unchanged HSI and select/read it back with DSB/ISB. If trim differs,
   move to unchanged stable LSI, stop HSI and await !STABLE, install trim,
   restart/read HSI and select it, then restore the original LSI request.
   Program the documented HSI divider while preserving trim.
8. Reuse admitted HEX, or disable HEX and await !STABLE before configuring
   its digital input pad and PINMUX/PINEN. GPIO has no LOCK register. Disable
   output before changing pulls, edge/level IRQ enables, open drain, per-pin
   filtering, AF0 and digital input. Preserve other pins and filter-clock bits.
   Enable HEX and await readiness using bounded polls.
9. Select/read requested source while guarded, apply/read final buses, verify
   HSI, calculate final Flash wait from actual HCLK, then verify mux again.
10. Publish verified rates and a frozen whole-boot pad reservation, then return
    peripheral tokens through the established initialization boundary.

AWT selects PB0/PB1 independently of the system HEX mux and HEXEN, including
when DeepSleep stops system HEX automatically. Its active input is therefore
reserved independently even in HSI-only initialization. Configured or retained
system HEX plus AWT can reserve both pads. The other input receives no invented
frequency bounds. `Flex::new` checks reservation before any GPIO write.
HEXEN/PINEN software-write effects on the independent AWT feed are not explicit
in own sources, which is why this first batch admits exact reuse only.

Every register wait has a caller-selected finite iteration budget. Any RCC
failure after singleton acquisition returns no tokens or published RCC clocks;
reset before retrying. Invalid pure configuration is retryable before
acquisition. Existing global policy is unchanged: a later time-driver failure
may leave already-verified RCC clocks available, still without tokens.

## Source conflicts and excluded behavior

Own CR1 exposes HSIEN, HEXEN and LSIEN only; bit 2 and bits 15:4 are reserved.
Copied SDK CCS/LSELOCK helpers and generic automatic-switch prose do not
establish usable hardware. This implementation writes no CCS or PLL fields.
STABLE records startup and remains latched on later loss. HEXRDY is not a
loss interrupt. There is no documented automatic HSI fallback; no final-bus
fallback requirement is invented. If HEX disappears after selection the CPU
can stop, so bounded register polling is not a wall-clock progress guarantee.

No runtime changes, PLL configuration, DeepSleep entry/resume, RF behavior,
clock-loss recovery, silicon validation, flashing or firmware execution are
included. `examples/shared/hex-clock` contains real PB0/PB1 UART/LED programs for all
five exact packages. Production ARM builds and data/PAC/source checks are the
verification boundary; no HAL tests, mocks or register simulation are added.

The additive HexLimits metadata extension preserves absent/null omission and
existing HSE shapes. Downstream Rust struct literals require the new optional
field. It requires independent schema review and a new provenance-chain tip;
this implementation does not grant legal clearance to historical ancestry.
