# Unblocked HAL extensions: software validation

Date: 2026-10-14. Baseline: main `35dc4d8131dcc84a61c6a5a431f5a0a7bf2faad8`.
This receipt covers wider-word async PIO SPI, coalesced timer input-capture IRQ
waits, LCD frame IRQ waits, and AUTOTRIM periodic-underflow IRQ waits only.
It does not claim UART TX ring buffering, I2C ten-bit/slave/bus recovery,
HALLTIM event waits, hardware qualification, or completion of the wider backlog.

## Environment and source restoration

- Official Rust `1.99.0 (b940084d7 2026-09-28)`, `thumbv6m-none-eabi`.
- Existing pinned Python requirements installed/verified.
- All twelve official SDK archives and their selected SVDs restored with the
  repository's exact SHA-256 checks. SDK members extracted offline with the
  existing acquisition tool. No vendor binary or example was executed.
- `./d gen-all` completed, including canonical source-lock/dependency metadata
  checks. This is not a replay of all PDF/manual source audits.
- Cargo's native Git backend retrieved the same pinned official Embassy/unitrait
  revisions; dependency sources, checksums and lock revisions were not substituted.

## ARM compilation and links

All 113 ARM library build commands passed: 54 existing chip selections with
runtime, the same 54 with runtime plus defmt, and five qualified GTIM1 time-driver
configurations (L083, L052, L010, L011 and L012). The matrix runner exited zero
and printed its production-matrix completion marker. The distinct example
command groups are:

| Example group | Exact chip selections / build commands | Result |
|---|---:|---|
| SPI async PIO, all three bins | 11 | pass |
| Classic SPI DMA, all bins | 12 | pass |
| L012 SPI DMA, all bins | 2 | pass |
| Timer overflow/capture, all bins | 13 | pass |
| Buffered timer input, all bins | 7 | pass |
| Classic timer input, all bins | 30 | pass |
| AUTOTRIM counter, blocking and async bins | 8 | pass |
| LCD, blocking and async bins | 8 | pass |

All 91 accepted example build commands exited zero. Each row counts one build invocation per listed chip selection, not the number
of emitted ELF files. Different groups reuse some selections; their sum is not
a count of distinct chips. All accepted example invocations use final relevant
source hashes; superseded SPI example attempts are excluded.

The library matrix command is `bash ci/check-hal.sh --matrix-only`.
Example invocations have the form:

```sh
cargo build --locked --release --bins --target thumbv6m-none-eabi \
  --manifest-path examples/GROUP/Cargo.toml --no-default-features --features CHIP
```

Commands use `--locked --release --target thumbv6m-none-eabi`.
Libraries additionally use `--no-default-features --features CHIP,rt` or
`CHIP,rt,defmt`. Example commands use `--bins --no-default-features --features
CHIP` and each example's own linker script/build script, without overriding
FLASH capacity. Building an ELF is not execution of that firmware.

## Executed focused checks

- SPI: six actual-source helper/dispatch tests passed with Rust 1.99 and
  `--cfg spi_dma`. They cover 4/8/9/16-bit payloads, mismatched/empty transfers,
  in-place TX preservation, nonbyte DMA rejection before backend entry, and
  byte DMA dispatch. The backend is a host stand-in, not actual DMA.
- Timer capture: five source-extracted wait/handler scenarios passed using
  mock registers. These exercise capture wake/completion, cancellation,
  forgotten-wait retirement and preserving unrelated channel state.
- LCD: nine source-extracted host tests passed, covering unpolled futures,
  cancellation, pending-event races, wake registration/replacement,
  forgotten-wait reconstruction/owner Drop and inactive-handler isolation.
  An unrelated-client sentinel does not model physical interrupt concurrency.
- AUTOTRIM: five source-extracted lifecycle scenarios passed with mock
  registers, including cancellation, pending events, forgotten wait cleanup
  and inactive-handler behavior.
- Actual production LCD API compiled for L052 with sequential awaited waits.
  Two simultaneously live wait futures were rejected with E0499; dropping
  the owner before later use of its future was rejected with E0505.
  Expected compile failures are ownership checks, not runtime tests.

Temporary harnesses and logs remain local under ignored validation output;
no permanent HAL test framework was added. Host stand-ins do not reproduce
silicon, NVIC scheduling, bus timing or clock-domain behavior. No Miri run is
claimed for this batch. Prior UART Miri results do not validate these changes.

## Issues caught and corrected during validation

1. L012 ATIM uses DIER, unlike the IER accessor on other timer versions. The
   new capture-request macro initially called IER unconditionally; versioned
   accessors corrected the E0599 build failure.
2. The first combined byte/wide SPI example exceeded F020 FLASH by 1696
   bytes. Separate byte and wide-word firmware binaries now preserve coverage
   without increasing the linker memory region or removing width validation.
3. Existing SPI2 DMA example integer literals inferred i32 after the wider
   generic interface; explicit u8 annotations restore intended byte calls.
4. The LCD example's new executor dependency required an updated Cargo.lock.
   Final checks use the resulting lock with `--locked`.

The failed attempts were retained locally and are not counted as successful
acceptance commands. A temporary SPI host invocation without its required
`spi_dma` cfg also failed in the dummy-backend path; the correctly configured
six-test run is the result reported above. A preliminary extra `-Tlink.x`
caused duplicate linker MEMORY definitions; final example commands rely only
on their existing build scripts.

## Source inventory and formatting

`python3 ci/update-functional-coverage.py --validate-only` passed for 13
families, 458 peripheral functions and 2280 rows. Its fingerprint is
`a5ab0ebadb83e468ad51151441a12027e57fb699f4640c6137aa272ebe7851d3`.
This validates source declarations, not compilation/runtime capabilities.
`git diff --check` passed. Existing generated `RCC_LSE_SYSCLK_*` unused-constant
warnings remain; no warning-free claim is made.

## Limits

No board was flashed. No electrical SPI exchange, capture timing, LCD waveform,
actual shared-vector simultaneity, low-power wake, clock failure, or sustained
interrupt stress was measured. Coalesced status flags are not lossless event
queues, and no timing/latency guarantee is added. Source-qualified unsupported
hardware features remain unsupported.

## Accepted source identity

The final source/example hash snapshot is checked for changes across acceptance
commands. Historical source-evidence receipts are not rewritten as new tests.

| File | SHA-256 |
|---|---|
| `embassy-cw32/src/spi.rs` | `466ac2406dfd725bc976ef0467beff9f2046afe670809c8cc622b16c9dba571a` |
| `embassy-cw32/src/lcd.rs` | `0a96df0d3d2634b7a3988d15e6ccf1db404be4cafd0494e830804373386edc8c` |
| `embassy-cw32/src/autotrim.rs` | `f03052e365a2a4cca20ef2654a44d247a4a08a37e6f76963caf7156fc518ffb6` |
| `embassy-cw32/src/timer/input_capture.rs` | `b2ccaa523242759107637a430a8a8aec9e348d7bcef97dc867df6464574a0ffc` |
| `embassy-cw32/src/timer/input.rs` | `628a99aaa574a65fbab84f15d0d96e84c5bdc9f600e3017b8c82b269f15be359` |
| `embassy-cw32/src/timer/classic_input.rs` | `6d10ea6a37236440b408f2c0972b35f272601efb7c3d3b40875f1cc6cd50d089` |
| `embassy-cw32/src/timer/low_level.rs` | `5894d7c4d58e8071821f4fcb93eb7134cc8f358fdbb624719fced5ac4dfade8f` |
| `embassy-cw32/src/timer/overflow.rs` | `62a61fa6a8ca08a636a508c1a1d834003e08115056a8a71c62b6516a3736e4b6` |
| `examples/shared/spi-async/src/bin/async_master.rs` | `f0cb8d150f2f7828e53d04053c5427b1b7be8580bc010cef8dbe899670bd3655` |
| `examples/shared/spi-async/src/bin/async_words.rs` | `ee5ef4a07f25f20df7911433acd94a11956fa325da004cf93e1f1fe7c3499a0e` |
| `examples/shared/spi-dma/src/bin/spi2_flash_id.rs` | `bd527628220a1d1804b00c78040b134ef6e2437b8f88b1a1f7733f3e8249e014` |
| `examples/shared/timer-overflow/src/bin/capture_event.rs` | `796409841a1623cb29f8a9ff71e99c2acf9aecfba873f4204de8a6e3ce58a325` |
| `examples/shared/timer-overflow/build.rs` | `fa96f6fa87ebb5a2d892912fe15f3dada65d997ed8523d531218692dec2e5ea5` |
| `examples/shared/lcd/src/bin/async.rs` | `b52960cabb4aa157fb4a0862f5560e08993f3c44a3e9e0e4c602fed4cd89a5d3` |
| `examples/shared/autotrim-counter/src/bin/async.rs` | `2ebe9e543121350f96acb688c4a5d07fa87cdff1c7ecad59288311e8b576978d` |
