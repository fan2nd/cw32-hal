# Qualified ATIM polling counter and main-output PWM

This document scopes the counter and SimplePwm owners. The later buffered-ATIM
complementary/BK1 owner is documented separately in [atim-complementary-pwm.md](atim-complementary-pwm.md).

This source-only batch covers the eleven ATIM-bearing families. F002 and F020
have no ATIM and gain no fabricated instance. No firmware was flashed or run on
silicon. Source review and successful builds do not qualify a power stage.

## Public scope and ownership

- F030/A030/F003/L031/R031/W031/L052/L083: classic ATIM, CH1A–3A main outputs
  through `SimplePwm::new3`; `split3` borrows three channel handles.
  CH4 is expressly internal-only in every own manual. Four-channel construction,
  `ch4`, `channel(Channel)` and four-channel `split` require a distinct sealed
  four-channel capability and are unavailable for classic ATIM.
- L010/L011/L012: buffered ATIM, main CH1–4 through the usual four-channel
  `SimplePwm::new` / `split`. Hardware channels 5/6 and all N outputs
  remain disabled and have no HAL pin routes.
- `timer::low_level::Timer` owns ATIM for stopped construction, explicit start,
  stop, counter read/write, finite configuration and polling of the update flag.
  At RCR=0 every wrap updates. Flags coalesce: this is not a lossless event count.
- The ATIM token owns the whole timer, all compare channels, gate and reset.
  PWM wrappers retain their pins, and borrowed channels retain the controller.
  A channel borrow prevents frequency changes and dropping the controller.
- ATIM's independent gate/reset never resets or gates BTIM or GTIM. Buffered
  ATIM uses keyed APBEN1 bit 5 and unkeyed active-low APBRST1 bit 5; classic ATIM
  uses APBEN2/APBRST2 bit 7. Register adapters use the actual selected PAC.

The interface provides only internal PCLK, continuous edge-aligned up-counting,
main PWM, active-high/low, disabled inactive drive, and duty endpoints. It does
not provide complementary outputs, break protection, dead time, safety inputs,
capture, encoder, external counter/trigger/cascade, interrupts, DMA or an async
time driver. Pending L010 timer-trigger work is unchanged.

## Counter, prescaler and clock boundaries

Both paths expose periods of 2–65536 ticks and use ARR=period−1. Buffered hardware
explicitly stops at ARR=0. For classic counter-only use, two ticks is a deliberate
conservative API minimum; the own PWM initialization rule requires CNT<ARR.

Classic PRS encodes divisors 1,2,4,8,16,32,64,256. /128 is absent. The shared
`Prescaler` enum still contains values useful on other timers, but classic ATIM
rejects unsupported values before register access. Buffered ATIM uses its linear
PSC+1 register and the existing bounded power-of-two 1–32768 subset.

Requested ATIM frequency ceilings are selected using
`rcc::Clocks::pclk_bounds().maximum()`, rounded outward, under the declared RCC
`OperatingConditions`. Nominal `kernel_clock` / `get_frequency` are separate from
`kernel_clock_bounds` / `frequency_bounds`. The latter returns conservative
whole-hertz bounds. Actual pulse frequency therefore stays no higher than the
requested ceiling within the qualified clock envelope. This does not establish
electrical waveform quality at a pad. Load, supply/current conditions and routing
must satisfy the own datasheet. GTIM/BTIM's prior nominal selection policy is
preserved; these shared counter additions do not silently change their timing.

The widest divisor product is 32768×65536=2147483648, which fits u32. Selection
and duty fractions use widened arithmetic. The driver never truncates 65536 into
a 16-bit CCR: zero and full duty use documented forced-output modes.

## Register sequencing and output contract

Construction first stops ATIM, disables requests and main output, disables
break/dead-time/complementary/slave/master/ADC-trigger behavior, sets repetition
to zero, configures forced-low main outputs and their preloads, writes ARR/PSC
and CCR, then issues one software update. Only PWM construction enables main
outputs; pins connect after inactive mode is established. Counter-only
construction keeps MOE disabled.

Classic ATIM temporarily clears URS while issuing UG, avoiding the ambiguity in
its update-source prose. It then restores overflow-only updates. Buffered ATIM
uses the independently documented EGR.UG update. Every initialization and period
change clears UIF explicitly; no ISR read-modify-write is used. Classic ICR
reserved bit 1 remains at its reset-one value. All other event flags, including
break flags, are preserved by explicit overflow clearing.

Ordinary interior PWM duty changes write the preloaded compare and take effect
at the next update. Mode transitions, such as entering/leaving forced endpoints
or enabled output, restart the entire timer at zero and commit every pending
peer compare. An operation that leaves the output mode unchanged does not restart. Buffered mode transitions deliberately pass through Freeze to
force a fresh PWM comparison. Frequency changes hold all exposed outputs
inactive, rescale requested duties, commit once, and restart the period.
There is no glitch-free or phase-continuity promise.

Disabled channels actively drive their inactive level without clearing global
MOE. Drop forces inactive, stops, disconnects owned pads, disables main outputs
and requests, then gates ATIM. No electrical level is promised after disconnect
or when MOE=0. This subset deliberately disables hardware protection and must
not drive a safety-critical power stage without independently established
protection and board validation.

## Routing and provenance

`atim-pwm-route-evidence.json` and eleven `cw32-data/af/*-atim-pwm.json` sidecars
record 135 retained die routes. Qualification intersects own datasheet AF cells,
own reference-manual AF cells, actual SDK GPIO assignments and original package
pin grids. Exact package bonding controls projection; family aliases use the
common-package intersection. Complementary, CH5/6, break, debug/reset/boot,
input-only, oscillator-owned and radio-reserved routes are withheld.

The source pass excluded four SDK-only routes across R031/W031/L083 and four
unsafe pads across L010/L011. F/A030's timer-chapter CH2A PA05 typo is not used;
its own GPIO allocation grid, own package datasheet and SDK agree on PA04.
L052's vendor-derived PAC exposes MSCR.MSM although the own manual reserves bit 4;
this implementation writes zero and exposes no master/slave functionality.
The conflicting PAC field is not promoted as a capability or silently corrected.

`atim-evidence.json` records own source IDs, hashes, register sections, page
indices, semantic facts and these contradictions. Raw PDFs, SDKs and extracts
remain outside the release. Reproduce the source checks with:

```sh
python3 tests/verify_atim_evidence.py --sources "$CW32_SOURCES"
python3 tests/verify_atim_pwm_routes.py --sources "$CW32_SOURCES"
python3 cw32-data/tools/source_provenance.py --sources "$CW32_SOURCES"
```

ATIM sidecars are pinned in the data generator. Updating them requires deliberate
source requalification and updating the guarded pin. Regenerate chip metadata
and PAC through `./d gen-all`; no generated Rust was hand-edited.

## Verification boundary

Final production verification is recorded with before/after input hashes in
`verification-logs/atim/final-clean-builds/`. It compiles all 54 HAL selections
with runtime and runtime+defmt and links the genuine firmware examples, without
executing any firmware. Own-source evidence and generated-data parity are recorded
separately in `verification-logs/atim/final-clean-sources/`.

The latest explicit user direction removes HAL tests and test harnesses; this
batch adds none. Its source/provenance validators remain metadata audits. The
complete firmware examples in `examples/shared/atim` cover eleven exact packages and
document their actual output pads. Independent production/source review is
required before integration; its final acceptance is tied to frozen file hashes.
