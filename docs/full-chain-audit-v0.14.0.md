# v0.14.0: data → PAC → HAL, module-by-module review

This review starts from v0.13.2 and compares the existing implementation with
actual pinned upstream source and CW manuals. All 46 data IP definitions and
45 existing HAL `src/` Rust files plus its build script are covered. Compilation was one check, not evidence
that names, hardware semantics or the entire architecture match upstream.

## What changed

| Issue found | Correction | Evidence |
| --- | --- | --- |
| Common/IP Rust and metadata were repeated in chip PACs | Shared access/types and one PAC/metadata module per kind/version; small chip roots | JSON-only and isolated consumer builds, shared inventory and order/conflict probes |
| Same-value defaults could retain only the last source; metadata dropped field details | Deterministically merge reset provenance; retain descriptions, kinds and enum values | Cross-chip reset fixture, sparse enum fixture and metadata checks |
| Comparator reference pairing was inferred from instance names | Sourced family-instance connections, consumed by HAL associated types | Schema validation and renamed-reference JSON→metadata→actual build-script proof |
| New ADC sequence could inherit a forgotten predecessor's watchdog/conversion state | Validate first, then stop/clear predecessor and reset its watchdog scope | Both current-PAC sequence probes and lifecycle/ownership checks |
| Software preload UG could trigger ADC/downstream timers during initialization/commit | Gate applicable trigger outputs while stopped, then restore intended routing | Actual ATIM and timer-startup register traces, including protection races |
| Time calls before init and queue saturation could panic | Zero pre-init timestamp, retained early registrations, bounded eviction/retry | Actual Driver contract and both-IP deadline/queue probes |
| Waker clone/drop/wake could reenter borrowed driver state | Move callbacks outside state borrows and critical sections | Custom reentrant RawWaker and ordinary wake→now/schedule checks |
| Malformed names/topology/paths could escape early validation | Fail-closed schema and shared-output checks | Malformed JSON and collision/core/memory-bound probes |

Public driver names, typed register access and board examples remain usable.
The generated output directory/layout and normalized schema changed; regenerate
from source. [PAC migration details](audit-v0.14-pac.md) explain the standalone
CLI bundle requirement.

## Complete comparison maps

- [Data: every one of the 46 IP models](audit-v0.14-data.md), with source sections,
  layout/reset counts, family/chip facts and unresolved source conflicts
- [Generator and PAC: every pipeline/access/metadata/build area](audit-v0.14-pac.md)
- [GPIO, RCC, generic timer, specialized PWM and time driver](audit-v0.14-digital.md)
- [ADC, BGR, DAC, VCREF, OPA, comparator, CORDIC and EAU](audit-v0.14-analog-math.md)

The remaining HAL root/build/interrupt modules were separately checked:

| Module | Result |
| --- | --- |
| `src/lib.rs` | Official Peri identity/borrow model; once-only acquisition under the outer critical section; validated RCC before time init; reserved timer not returned. No new root-init defect found. |
| `src/interrupt.rs` | Official type-level IRQ/Binding implementation, real dispatch macro and source-local event latch. Waker callbacks avoid outstanding latch RefCell borrows. Unknown/incorrect binding proofs reject. No new defect found. |
| `build.rs` | Chip feature selects metadata; presence/IP/capability cfg, tokens and trait implementations derive from it. Shared reset and reset-effects policies remain, comparator pairing now consumes explicit data, and GPIO checks FieldKind::Bool. No family-name behavior branch. |
| Cargo/workspace/CI | One generator crate; ordinary PAC build reads only generated Rust. Both chip selections are individually verified. Six independent board binaries remain direct, with 05/06 motor outputs explicitly opt-in. |
| Board examples | Recompiled and linked against the changed chain. Existing C-derived sampling/control/fault ordering was not replaced with a new abstraction or demo wrapper. No board execution is claimed. |

## Differences are not all defects

CW port interrupts are not STM32 EXTI lines; IP prescalers, ADC completion modes,
reference topology, trigger paths, read/clear behavior and dead time differ.
These remain explicit. Hardware absent on F030 is not synthesized in software.

There are also genuine **software scope limits**, not excuses about nonexistent
hardware: no full UART/SPI/I2C/DMA HAL, timer capture/encoder/generic complementary
API, ADC internal-channel/calibration/DMA ownership, split DAC/PWM owners, full
clock-tree/low-power framework or all upstream configuration options. The matrix
labels them as omissions. Multi-bit PAC hardware fields currently remain raw;
enum infrastructure is not a fully enumerated hardware dataset.

No current evidence justified changing the audited register layout/routes or
manufacturing the remaining unknown/default values. Shared data reuse does not
mean that incompatible IP versions can be merged.

## Source baseline and verification

- [Embassy b12a6d9e](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444)
- [stm32-data e6a417fa](https://github.com/embassy-rs/stm32-data/tree/e6a417fa643efaf031abc79590ea3e76bc4edbf2)
- [chiptool bcf538a2](https://github.com/embassy-rs/chiptool/tree/bcf538a2e7b8584ae874ee9ab72efb1576fc6152), with the older data-lock IR pin recorded in the data comparison
- [generated metapac e463add8](https://github.com/embassy-rs/stm32-data-generated/tree/e463add8cc54375f61c6f5f83d6b589e7fc68be2)

The final [validation record](validation-v0.14.0.md) distinguishes source/data
checks, memory/model probes, compiler ownership checks and real ARM linking.
No silicon was flashed, no motor was powered and no timing, analog accuracy or
board safety was measured. Passing those software checks is not hardware proof
or a claim of complete Embassy feature parity.
