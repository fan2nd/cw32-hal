# L012 UART, I2C and IRQ extensions: software validation

Date: 2026-10-14. Baseline: main
`22efddf` (the preceding SPI/timer/LCD/AUTOTRIM batch).
This receipt covers L012 nine-bit UART PIO, ten-bit I2C master addressing,
HALLTIM capture waits, CORDIC completion, and L012 LVD/comparator edge waits.
It does not claim UART TX ring buffering, classic ten-bit I2C, I2C slave or bus
unwedge support, or hardware qualification.

## Environment and identity

The already restored official Rust 1.99.0 toolchain and thumbv6m-none-eabi target
were reused. SDK/SVD identity and generated PAC are the same pinned inputs as
the preceding receipt; no vendor code was executed. Each batch has separate
command results and source hashes. Earlier batch tests are not relabeled as
verification of this delta.

## ARM libraries and examples

All 113 ARM library build commands passed: 54 existing chip selections with
runtime, the same 54 with runtime plus defmt, and five qualified GTIM1
time-driver configurations (L083, L052, L010, L011 and L012).
`bash ci/check-hal.sh --matrix-only` exited zero and printed its completion
marker. These checks include the shared UART Config regression across all
existing selections; they are not hardware execution.

Example groups (one build invocation per chip selection, all bins):

| Group | Commands | Result |
|---|---:|---|
| UART nine-bit | 2 | pass |
| UART buffered | 8 | pass |
| UART DMA | 21 | pass |
| UART DMA TX | 7 | pass |
| UART flow control | 7 | pass |
| UART idle | 3 | pass |
| L012 I2C | 2 | pass |
| Classic I2C regression | 12 | pass |
| HALLTIM | 2 | pass |
| L012 math/CORDIC | 2 | pass |
| L012 analog/LVD/comparator | 2 | pass |

All 68 accepted example commands exited zero. These invocations are not 68 distinct chips or 68 ELF files. Some groups
reuse the same chip and link multiple bins. Build commands use
`cargo build --locked --release --bins --target thumbv6m-none-eabi
--manifest-path examples/GROUP/Cargo.toml --no-default-features --features CHIP`.
No FLASH limit or linker region was enlarged. Thirteen preliminary example
invocations were repeated after the complete source freeze; only final accepted
runs count in the table.

## Public API checks

Twenty-six actual production API checks yielded the expected outcomes:

- L012 combined/split UART u16 PIO methods compile. On each of the twelve
  non-L012 family representatives, the same method calls are rejected with
  E0599 because those methods are absent.
- L012 blocking and async `I2c<TenBitAddress>` calls compile. On each of the
  twelve other family representatives, those calls are rejected with E0277
  because the u16 address-mode trait implementations are absent.
- These probes contain no peripheral acquisition, UART3 symbol or pin routes.
  Every negative log contains only its intended diagnostic class. They prove
  API availability boundaries, not execution of constructor rejection logic.

The twelve non-L012 representative selections are A030C8T7, F002F3P7,
F003E4P7, F020C6U7, F030C8T7, L010F8P6, L011K8T6, L031C8T6,
L052C8T6, L083MCT6, R031C8U6 and W031R8U6. Positive API probes select
L012C8T6; example links additionally cover C8U6.

Twelve separate IRQ-owner borrow probes cover LVD, VC1, HALLTIM and CORDIC:
sequential awaits compile for each owner, two live mutable wait/calculation
futures are rejected with E0499, and moving/dropping an owner before later use
of its future is rejected with E0505. These are compile-time checks, not ISR
execution.

## Actual-source host checks

All temporary harnesses extract production code but replace the hardware and
surrounding environment with host stand-ins. No permanent HAL test framework
was added.

- UART: three native-family and two nonnative configuration tests passed,
  extracting `DataBits`, `validate_config` and `require_byte_config`.
  These exercise nine-bit/parity/family admission and byte-only backend
  restrictions separately from API compilation. Two receive tests passed,
  including 512 old/new byte-helper trace comparisons, nine-bit payload and
  error precedence. Payload-mask cases include values beyond 0x1ff.
- I2C: the extracted `Address`, preflight checks, blocking runner, async runner
  and cancellation guard passed exhaustive encoding of all 1024 legal ten-bit
  addresses, five transaction command patterns, cancellation/error insertion
  at each tested command boundary, and range/empty/256-byte checks. A mock
  command backend is not an I2C bus or a real slave.
- HALLTIM: five extracted lifecycle scenarios passed, including pending/event
  paths, request masking, forgotten waits and shared-state preservation.
- CORDIC: six extracted lifecycle scenarios passed, including completion before
  registration, pending completion, inactive dispatch, and actual owner Drop
  behavior with modeled busy/idle state. Numeric output accuracy is not tested
  by these lifecycle scenarios.
- LVD: six grouped scenarios passed using the actual edge-wait module and
  owner Drop body: inactive/no-MMIO, unpolled/no-effects, arm/cancel, IRQ
  wake/completion, event before handler, and forgotten wait/restart/Drop.
- Comparator: seven grouped scenarios passed using the actual edge-wait
  module and owner Drop body, covering the same lifecycle paths plus a live
  shared-vector sibling that remains unchanged while the other owner retires.
  Only chip-binding implementations and owner/register types are replaced by
  host definitions. The single-threaded model does not emulate NVIC races.

## Corrections and limits

The first temporary LVD/VC public-API harness used a nonexistent Edge::Both
variant. It was corrected to the actual Edge::Either variant and all six
LVD/VC outcomes were rerun; the invalid attempts are not accepted tests.

Incremental smoke checks caught HALLTIM sealed-type visibility warnings; the
source author corrected the declarations before final acceptance. The LVD
example threshold was changed to the supported 2600 mV setting before its final
link. These are not hardware observations.

No board was flashed. No real nine-bit UART, ten-bit bus transaction, analog
threshold, capture timing, CORDIC result accuracy, interrupt latency or
low-power wake was measured. No Miri run is claimed. Register/critical-section
stand-ins and source reviews cannot establish silicon timing or electrical
behavior. Coalesced events do not constitute lossless event histories.

## Source inventory and formatting

The source-declaration validator passed for 13 families,
458 peripheral functions and 2379 rows.
Current inventory fingerprint: `76bb437cd133a661781e350220c1340109166ceee057001125726619c6a718aa`.
This source-only check is not a runtime/compile test. `git diff --check` passed.
Only existing generated RCC_LSE_SYSCLK_* unused-constant warnings were observed
in final production builds; no warning-free claim is made.

## Final source identity

The following source/example SHA-256 values identify this acceptance run.
The full HAL and affected-example file inventories were checked for drift
throughout validation. Source hash drift requires relevant reruns, not carrying
over an old result. Canonical source-lock, historical evidence references and
dependency-pin metadata checks also passed after the source/docs updates.

| File | SHA-256 |
|---|---|
| `embassy-cw32/src/usart.rs` | `a205df9413daa9f4eff228e5ce4b7d0584b63b3bc12cd58d2cbd0705a76a4ced` |
| `embassy-cw32/src/i2c/lpi2c/mod.rs` | `96fd71fcff6158268c52d229b9e18b31bcd5e0423c9cdea90b5a3433aba103ee` |
| `embassy-cw32/src/halltim.rs` | `e3b2cfe798f7b37987d5f65dfbe203fe897859c4abfd1b6186a6f1369635f059` |
| `embassy-cw32/src/cordic.rs` | `a6ebf4c2df49286354f1257e1aaadbe23e7c936b1e8ce95c7a4560bba5869be6` |
| `embassy-cw32/src/lvd.rs` | `c45c563d837a41b73f0ade628072eab81357b479afae3f6c63aa2b93b9d0c130` |
| `embassy-cw32/src/comparator.rs` | `5fffdda2da437354759b5e4eb8df87ba8ed0396074627b586139fecc381947a8` |
| `examples/shared/uart-ninebit/src/bin/async_split.rs` | `c74d279b1d4c1cfe7d052c814f9bc49973dab91b95281a243b3cfa3ddc974730` |
| `examples/shared/uart-ninebit/src/bin/blocking.rs` | `8c5552845a6b97bc0e5e44be312010f7c113193e046f82a18816b4f51dd77f43` |
| `examples/l012/i2c/src/bin/async_master.rs` | `7365c1a45330f13158c31679287753d62a5befe988aab400530704b46244793b` |
| `examples/l012/i2c/src/bin/blocking_master.rs` | `9721bcc4d09f35f1cbda3903a033557c28bf184a022874f471ac801493481d75` |
| `examples/l012/i2c/src/bin/ten_bit_master.rs` | `3fec4ae0cec7166b93c8234f41e62cbe6bd7eb655324a611c581c09410981e40` |
| `examples/l012/halltim/src/bin/capture.rs` | `a54d89338d7e084935fe287c52f7c774c0189ac283ff294994653943282093ee` |
| `examples/l012/halltim/src/bin/async_capture.rs` | `92c7616435e322ed8f7e2d85ae40c113a910225eeb4ad4a194ac2104fd3ac58e` |
| `examples/l012/math/src/bin/cordic_async.rs` | `718eeda5d06aa9d8dfa78fa5227e17909cb19f43665a367f952ed3e8c8a48d26` |
| `examples/l012/math/src/bin/cordic_fixed_point.rs` | `2f073e0cc82c139422a8ee1c04b0824bb5984ea6087fbdab11a09dfaa6c333c5` |
| `examples/l012/math/src/bin/eau_integer.rs` | `163360da4cb7d6473f29a77d96e58c5986f034f16367caf1001202354b5a953a` |
| `examples/l012/analog/src/bin/comparator_edges.rs` | `c382411562d5a7044acb85d63b1a4907cf3ac34322d78f3ed1cfd051ee2d9b98` |
| `examples/l012/analog/src/bin/opa_buffer.rs` | `18b6bbde89bffc72b069fe1b26c6f71cc18e1945ce5e1681d060a303fc5f811d` |
| `examples/l012/analog/src/bin/comparator_divider_edges.rs` | `9d5cad26f392916fca0c766c3672b01ffa3e1fdfd97d5511d198064dbc6316f3` |
| `examples/l012/analog/src/bin/dac_steps.rs` | `31646fed86c975480a26cb8769d299ec9a978ed902c2b0038d058e8ec3bc4d4d` |
| `examples/l012/analog/src/bin/lvd_edges.rs` | `bcddd28f62f96300f9e4ad3027e7aeee9a19b8a8097144e8e8c78fea3e75766d` |
| `examples/l012/analog/src/bin/opa_pga.rs` | `9e908f7cea6e2795ceb28dd4b9e0714aa413dc082eb2bb221a14d956816ccbdc` |
| `examples/l012/analog/src/bin/opa_standalone.rs` | `a9d7155d28134cb1b4c93c220da830585271b0eee716ac973740c4fec63ecd05` |
