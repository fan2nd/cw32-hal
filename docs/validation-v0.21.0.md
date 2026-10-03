# v0.21.0 verification

The timer capture/encoder/complementary and UART/SPI DMA additions are checked
with Rust 1.99.0 and real `thumbv6m-none-eabi` code generation. No physical board
was programmed or exercised.

## Integrated builds

- Formatting and YAML → persisted schema11 JSON → PAC regeneration/drift pass.
- Both PAC release builds with runtime/metadata and both HAL minimum/full release
  ARM builds pass.
- Both minimum/full ARM rustdocs pass with `-D warnings`.
- Fourteen individual optional-feature combinations pass across both chips.
- All six release board examples pass; 05/06 also pass with motor outputs enabled.
- Two linked ARM integration ELFs exercise the new API bodies: typed UART/SPI
  DMA, both shared DMA channel handlers, capture, QEI and complementary PWM.
  Every one of the 32 device IRQ vectors matches metadata; selected DMA,
  capture, UART/SPI and reserved GTIM1 handlers have real strong symbols. Stack,
  RAM and Flash bounds pass. The artifacts are never run on hardware.

Only the existing upstream `proc-macro-error2` future-compatibility notice remains.
All 29 motor API and example Rust files are byte-identical to v0.20, and the six
examples still contain no direct PAC access or unstable-PAC dependency.

## Focused external checks

| Area | Verification |
| --- | --- |
| Capture/QEI | Actual driver and typed PAC adapter models for all four IP combinations: physical input identity, filters, encoder modes/inversion, timestamps and loss flags, source-local W0C, timeout/cancel/rearm, both IRQ/poll orders, shared-peer isolation and clock-gated post-Drop dispatch |
| Capture/QEI API | 22 ARM compile probes: 5 positive and 17 intended rejections covering wrong input/pair/IRQ, F030 ATIM CH4, ownership/lifetime/mode and GTIM1 time-driver reservation |
| Complementary PWM | Eight actual-library/PAC MMIO cases across both chips; 29 manual deadtime anchors, optional/main/N-only routes, duty endpoints, buffered updates, disabled timing changes, brake acknowledgement/rearm, Drop and clock retention |
| Complementary API | 2 positive ARM constructions and 22 intended rejections for wrong timer/main/N/BK/channel, nonexistent fourth pair, duplicate ownership and overlapping channel borrows |
| Arm fault ordering | Six actual-source injected sequences cover a fault before/after MOE and during/after pin mux, hardware refusal of MOE, and successful arm |
| UART/SPI DMA | 42 whole-driver cases across both IPs: clean completion, wire drain, error/drop/forget, validation and native byte/halfword setup; 2 independently added pre-poll error IRQ/late-TC races retain both SPI channels, buffers, pins and clocks |
| Bus DMA API | 2 positive ARM compilations and 22 intended rejections for missing/wrong Binding, borrowed owners/channels/buffers, use after move, and invalid word type |
| Existing DMA regression | 18 actual-source ADC/copy_mut lifecycle cases, both unchanged DMA engine suites, and immutable-copy behavior included in the bus fixture |
| Schema11 | Valid JSON accepted; 14 malformed topology/schema inputs rejected. Additional independent generated-build probes reject wrong controller/layout/cardinality/width/encoding and preserve an explicit selector after renaming the timer instance |
| Physical routes | All 70 F030 and 72 L012 represented ATIM/GTIM CH/BK tuples match independently extracted official PDF AF tables |
| Data preservation | Regenerating v0.20 with the current schema11 generator yields identical 46 register blocks and pin routes; only the four sourced F030 capture-mux relationships are added to normalized hardware facts |

The independent reviewer authored the additional pre-poll DMA error/late-TC
ordering scenario and metadata relationship probes. Implementation fixtures use
actual production driver/PAC code with external MMIO models. No checked-in
probe or test scaffold is added.

## Clean package

The source-only candidate was rebuilt in a separate directory from no generated
artifacts or Cargo.lock. Both-chip minimum/full release and strict-doc checks,
six examples and 05/06 motor opt-in passed. The complete clean matrix passed again after the final UART wire-drain and
shared-register refinement, along with the final linked ELF and optional-feature
checks. All 48 normalized JSON and 102 generated PAC/metadata/runtime files
match between the maintained and clean trees, and every packaged source entry
is checked byte-for-byte. Build targets,
generated source, lockfiles, probes, logs and caches are excluded.

## Limits

Host models check software protocol and modeled MMIO outcomes, not hardware
synchronizers, sampling bandwidth, physical deadtime, fault latency or bus timing.
F030 GTIM's missing overcapture indication and UART RX's single-frame bound
remain visible in the API. DMA disable is not treated as an abort-drain proof:
post-launch errors/cancellation permanently retain static resources.

The bounded scope and remaining clock/low-power work are tracked in
[v0.21 scope](timer-dma-v0.21.0.md).
