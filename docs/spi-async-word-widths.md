# Async SPI PIO word widths

This API delta extends the already qualified IRQ-assisted PIO master
backend from bytes to the existing sealed `Word` set: `u8`, `u16`, and `word::U4`
through `word::U7` / `word::U9` through `word::U15`. It does not add a family,
controller, pin route, slave mode, or DMA transfer width.

## Scope and upstream comparison

The unchanged `spi_async_pio` gate covers F020, F002, F003, L010, L011, L012,
L031, R031, W031 and L052. The per-instance and package constraints in
[the PIO qualification](spi-async-pio.md) still apply. F030/A030/L083 remain
DMA-only for async SPI, and cannot use wider async frames through this change.

The pinned Embassy implementation
[`embassy-stm32/src/spi/mod.rs`](https://github.com/embassy-rs/embassy/blob/f16efeffe37581092ec184718e6fdb1620393214/embassy-stm32/src/spi/mod.rs)
provides generic `read<W: Word>`, `write<W: Word>`, `transfer<W: Word>`,
`transfer_in_place<W: Word>` and `embedded_hal_async::spi::SpiBus<W>`.
CW32 now follows this public word-generic shape, not STM32's DMA implementation
or its IP-specific data-access sizes. The existing CW32 blocking word path and
[typed-register contract](spi-typed-registers.md) already use WIDTH = bits − 1
and 32-bit typed DR access with a 16-bit payload field. This delta reuses those
exact encoding/conversion rules; it does not introduce raw byte or halfword MMIO.

## Runtime contract

- PIO lengths count frames, not bytes. Separate transfers clock the larger
  length, pad TX with zero frames and discard excess RX. In-place transfers
  send each original frame before replacing it. Wrappers mask to their width.
- Every received frame is still drained, including write-only calls. A poll
  retains the existing budget of at most one DR read and one DR write.
- The owner first settles any canceled/forgotten preceding frame using its old
  width. Only after TXE and not BUSY, with error-first checks, may it disable the
  controller, set the new WIDTH, and re-enable it. Cancellation does not store
  pointers into caller buffers in an ISR. Error recovery, IRQ dispatch and
  shared SPI23 handling are unchanged.
- Supported-word empty calls settle earlier work but do not change width.
  PIO still requires a successful flush with the original CS selected after
  cancellation; this API is not a transaction-completion or timeout guarantee.
- A DMA-selected owner still accepts **only u8**. Any other word type returns
  `Error::UnsupportedWordSize` before touching registers, changing state, or
  settling earlier work, including for empty slices. It never silently falls
  back to PIO. If prior work is outstanding, call `flush()` explicitly while
  preserving CS; a rejected wider operation does not complete it.
- The sealed word trait returns byte slices only for u8. Other implementations
  return `None`; no representation cast, new DMA width or staging conversion
  is used. The byte DMA routines remain unchanged.

## Calling convention

Typed nonempty slices normally infer the word. Previously unconstrained
literals may require a suffix, e.g. `bus.write(&[0xa5u8]).await`, rather than
an inferred integer type. Empty inherent calls need a type when no other
argument supplies it: `bus.write::<u8>(&[]).await`.

For trait-qualified calls with no data argument, use
`SpiBus::<u8>::flush(&mut bus).await` (or another supported Word); the inherent
`bus.flush().await` is still word-independent. Existing generic callers bounded
by `impl SpiBus<u8>` require no change. This mirrors the already generic
blocking trait surface.

`examples/shared/spi-async/src/bin/async_master.rs` retains byte exchanges; the separate
`async_words.rs` binary adds 4-, 9- and 16-bit exchanges on each existing bus.
Splitting the binaries avoids combining all word monomorphizations in the
smallest FLASH configurations. This is loopback
firmware, not a protocol example for an arbitrary attached slave. A real slave
must accept the chosen width and framing; do not change its framing while
selected unless its protocol permits it.

## Verification boundary

On 2026-10-14, Rust 1.99 host tests extracted the actual `Word`, `PioBuffers`
and public DMA-dispatch method bodies from this source. Six tests passed:
4/8/9/16-bit payload handling, mismatched lengths, empty buffers, original
in-place TX preservation, rejection of nonbyte DMA operations (including empty)
without entering the backend, and preserved u8 backend dispatch. The dummy
backend in that narrow dispatch test records entry, not hardware behavior.

ARM build/link results are recorded separately after execution. No hardware
loopback, interrupt timing, electrical behavior, or cancellation-on-silicon
result is claimed by these tests. Historical byte-only qualification receipts
remain frozen and are not relabeled as verification of this delta.
