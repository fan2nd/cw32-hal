# Linker-reserved FLASH storage example

This is destructive firmware, not a board-qualified demo. It has only been
compiled and linked; no device was programmed, emulated or accessed.

Before running, explicitly approve a disposable 4-KiB storage partition; review
all image sections/load segments, bootloader policy, live data/references and
DMA/debugger access. `build.rs` removes the selected exact part's upper 4 KiB of eligible storage
from executable FLASH and exports partition symbols. On L010/L011/L012 the 4-KiB region ends before the separately excluded final 512-byte SLIB descriptor page; an active SLIB overlap is rejected at runtime. That alone does not prove
exclusive application ownership. Generic profiles and legacy F030 aliases are
intentionally not selectable here. The exact part's RAM size remains unchanged.

Replace the illustrative 3.3-V / -20..70°C board envelope with sustained,
validated bounds. `OperatingConditions::from_rcc` uses the qualified factory-HSI
HCLK upper bound, including tolerance, rather than its nominal frequency. Keep
those conditions and clocks valid throughout driver use. Check watchdog and
interrupt latency: each program/erase can stall FLASH fetch, and BUSY has no
safe timeout/abort. Power interruption can corrupt data; no transaction or
`NorFlash`/`MultiwriteNorFlash` power-loss guarantee is made.

The program erases the first 512-byte page of the reservation, programs an
unaligned byte string, and reads it back with `ReadNorFlash`. It preserves the
other reserved pages. Readback failure panics; there is no automated retry.

Build only (do not flash/run):

```sh
RUSTFLAGS="-C link-arg=-Tlink.x" cargo build --locked --release \
  --target thumbv6m-none-eabi --no-default-features --features cw32f002f3p7
```

Select exactly one of the 37 exact part features in `Cargo.toml`. The default is
`cw32f030c8t7`. Main-array capacity comes from reviewed PAC memory metadata;
16-KiB F002, 20-KiB F003, 32-KiB F020/F030 64-KiB parts, and 128/256-KiB L083 parts therefore produce
different link-time boundaries. No controller security/option/bootloader area is
exposed. See `../../docs/flash-storage.md` for scope and failure behavior.
