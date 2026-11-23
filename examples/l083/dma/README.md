# L083 owned SRAM copies

Select one exact part feature (`cw32l083mct6`, `cw32l083rbt6`, `cw32l083rcs6`,
`cw32l083rct6`, or `cw32l083vct6`). The default is RCT6. Build with:

```sh
cargo build --release --target thumbv6m-none-eabi --no-default-features --features cw32l083rct6
```

The linker map comes from generated exact-part Flash/SRAM metadata. PB0 is a
completion indicator, not a claimed on-board LED. Confirm board wiring before
running. Channel 1 demonstrates a blocking u16 copy; channels 2/3 concurrently
copy exclusive u8/u32 singleton buffers through the shared DMACH23 interrupt.
Successful copies return and reuse the channel/buffer owners.

The example obtains `CopyChannel` capabilities from normal HAL initialization and
starts every copy through the safe `copy` method. Admission rejects unsupported
inherited DMA state; the runtime must enter from reset or a clean handover with
no outstanding, armed or gated transfers. Errors halt without recovering
quarantined resources. No safe abort, peripheral
request integration, firmware execution or silicon validation is implied by
successful compilation. See `../../../docs/dma-safe-owned-copy.md` and
`../../../docs/dma-owned-copy-evidence.md`.
