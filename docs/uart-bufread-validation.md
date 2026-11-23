# Buffered UART BufRead software validation

Validation date: 2026-10-14. This receipt covers the follow-up zero-copy
`embedded_io_async::BufRead` implementation, not new hardware qualification.
The earlier `uart-spi-api-validation.md` remains a receipt for its earlier stage.

## Toolchains and scope

- Production builds and host tests: `rustc 1.99.0 (b940084d7 2026-09-28)`.
- Production target: `thumbv6m-none-eabi`, release profile.
- Miri only: official rustup nightly, `rustc 1.101.0-nightly
  (822182db4 2026-10-13)`, host `x86_64-unknown-linux-gnu`.
  Installing this toolchain did not change the production toolchain.
- Validated `embassy-cw32/src/usart.rs` SHA-256:
  `7b96baccdd2eed883a6f177dd3cb6d1b8206cf43374cc4907262ada4210b6b88`.
- Validated `examples/shared/uart-buffered/src/bin/split.rs` SHA-256:
  `5d287701cdf12123c5c679e0d247e81180109b40f285f20094ed0edd89739ec6`.

## Completed software checks

1. Five independent host tests passed against the actual `RxStorage` and
   `RxRing` helper source, extracted byte-for-byte from the validated file.
   Tests retained a shared RX prefix while calling the producer helper to
   append into free slots, including wrap, full-ring overflow and sticky errors.
   All small capacities 1–12, initial offsets and occupied lengths were covered.
   Additional cases checked clean-prefix/error ordering, partial consumption,
   empty reads, 64-byte copying limits and rejection of overconsumption.
2. The same five tests passed under both Miri Stacked Borrows and Tree Borrows.
   `-Zmiri-ignore-leaks` permits the intentionally leaked static ring allocations;
   aliasing checks remained enabled. These results cover tested executions,
   not a general formal proof of the driver.
3. Five additional source-extraction tests passed, including a reference-queue
   comparison across mixed producer/read/fill/consume/error operations.
4. ARM compile checks used the actual public `BufferedUartRx` API. Holding a
   returned slice across an await and consuming after its last use compiled.
   Negative cases failed with the intended lifetime/borrow errors:
   - consuming while the slice remains in use: E0499;
   - dropping or moving its owner while the slice remains in use: E0505;
   - returning the slice as `'static`: E0521;
   - passing a local stack buffer to the constructor: E0597.
5. All UART buffered example binaries linked for these eight selections:
   `cw32f030c8t7`, `cw32f002f3p7`, `cw32l031c8t6`, `cw32l052c8t6`,
   `cw32l083rct6`, `cw32l010f8p6`, `cw32l011k8t6`, `cw32l012c8t6`.
   The split example calls the real BufRead trait, retaining an RX slice across
   an independent TX await before consuming it.
6. `bash ci/check-hal.sh --matrix-only` passed: all 54 chip features with
   both `rt` and `rt,defmt` (108 release library builds), plus five
   time-driver selections with runtime and defmt (113 configurations total).
7. `git diff --check` passed.

Temporary test harnesses and logs are local validation artifacts, not a new
committed HAL test framework. Production build warnings observed concern the
pre-existing unused generated constants `RCC_LSE_SYSCLK_LSE_EDGES`,
`RCC_LSE_SYSCLK_LSI_CYCLES` and `RCC_LSE_SYSCLK_MARGIN_LSE_EDGES`.

## Not validated

No board was used. These tests do not execute the real ISR, MMIO, interrupt
masking, task wake scheduling or UART timing. Linking the async example does
not execute it. Hardware behavior and concurrent IRQ timing remain unverified.
