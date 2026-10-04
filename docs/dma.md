# CW DMA: channels, transfers and the cancellation boundary

The HAL implements actual channel programming, normal completion, transfer errors,
shared-vector dispatch and owned transfers on CW32L012C8 and CW32F030C8. It is not
merely a route table. Its software structure follows the pinned Embassy
[`Channel`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dma/mod.rs)
and [`Transfer`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/dma/dma_bdma/mod.rs)
implementation: singleton channel ownership, checked interrupt binding, a transfer
borrowing its channel, register-before-check wake registration, explicit memory
ordering and Drop cleanup. Hardware configuration and cancellation guarantees
come from CW documentation, not STM32 assumptions.

## Ownership and normal use

`Peripherals` directly owns `DMA_CH1` through `DMA_CH4` on L012,
and through `DMA_CH5` on F030. Each field is the corresponding exclusive
`Peri` token, generated from the audited channel metadata. Construct a channel
with `Channel::new(p.DMA_CH1, binding)` or `new_blocking(p.DMA_CH1)`.
The whole-controller `DMA` token and `dma::split`/`Channels` API are removed.

Global HAL initialization resets the shared controller once before any token
escapes. Channel construction only acquires a counted clock reference and never
resets hardware, including after all previous clean owners were dropped. A
channel token can be reborrowed with the normal `Peri` lifetime rules; persistent
software poison survives owner reconstruction. See [the ownership migration](dma-channel-tokens.md).

- L012: four channels; vectors DMACH12 and DMACH34.
- F030: five channels; vectors DMACH1, DMACH23 and DMACH45.
- `Channel::new_blocking(ch)` does not require or enable NVIC.
- `Channel::new(ch, binding)` creates Async mode. Every used channel on a shared
  vector must be listed in `bind_interrupts!`.
- Completion and error are per channel. A handler neither services a peer's flags
  nor disables, clears pending state or changes priority on the shared NVIC line.

For example, on L012:

```rust,ignore
use embassy_cw32::{bind_interrupts, dma, peripherals};

bind_interrupts!(struct Irqs {
    DMACH12 => dma::InterruptHandler<peripherals::DMA_CH1>,
               dma::InterruptHandler<peripherals::DMA_CH2>;
});

async fn copy_words(
    dma_token: embassy_cw32::Peri<'static, peripherals::DMA_CH1>,
    source: &'static [u32],
    destination: &'static mut [u32],
) -> Result<dma::CopyBuffers<u32>, dma::Error> {
    let mut channel = dma::Channel::new(dma_token, Irqs);
    let transfer = match channel.copy(source, destination) {
        Ok(transfer) => transfer,
        Err(rejected) => {
            // Validation never started DMA. rejected.buffers remain usable.
            return Err(rejected.error);
        }
    };
    transfer.await
}
```

For F030 channel 1, replace the binding with
`DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;`.

A blocking owner uses the same `copy` constructor and
`transfer.blocking_wait(poll_budget)`. The budget counts wait-loop checks, not
elapsed time. Cancellation performs a final completion check. A `StartError`
returns both untouched buffers; an error after starting does not return them.

`copy_mut(source, destination)` accepts two owned `&'static mut [W]` slices and
returns `MutableCopyBuffers { source, destination }` after clean TC. This allows
editing the source and copying it again. Neither reference is exposed while DMA
uses it. Length/address/channel validation returns both references in
`StartError`; cancellation, timeout, error and forgetting retain the same
quarantine rules as `copy`.

## Typed finite ADC reads

`Sequence::read_dma(channel, destination)` consumes an ADC sequence whose input
tokens are static and an owned `&'static mut [u32]` destination. The destination
must contain exactly one word per configured slot. This is a hardware-requested,
software-started finite operation with actual ADC/request identity and the DMA
channel's checked interrupt binding. An ADC IRQ binding cannot substitute for a
DMA binding. A blocking ADC owner can use an Async DMA channel because completion
comes from DMA TC, not the ADC interrupt.

The two supported endpoints intentionally differ:

- L012 ADC1/ADC2: one non-continuous 1–8-slot scan. DMAEOS selects the generated
  `ADC1_SEQUENCE` or `ADC2_SEQUENCE` request. One EOS triggers BULK over the now
  stable result bank, with both addresses incrementing by four bytes.
- F030 ADC: one MODE=0 conversion to RESULT0. DMAEN selects generated
  `ADC_CONVERSION`, and BLOCK transfers exactly one native 32-bit word. A
  multi-slot sequence is returned with `UnsupportedDmaSequence` before launch.
  F030 requests each conversion, not EOS; no request-queue or multi-slot scan
  guarantee is inferred from L012.

The low 12 bits of each destination word are the conversion code. The 32-bit
storage matches native registers and L012's four-byte result-slot stride; it is
not a packed `u16` stream. There is no ADC circular, continuous, external-trigger
or lossless streaming API here.

For a one-slot sequence on L012 (F030 uses `p.ADC` and its own sample-time enum):

```rust,ignore
use embassy_cw32::adc::AdcChannel;

// destination: &'static mut [u32] with exactly one element.
// dma_channel was constructed with its real bind_interrupts! DMA binding.
let sequence = adc.configure_sequence([
    (p.PA0.degrade_adc(), adc::SampleTime::Cycles70),
])?;
let transfer = match sequence.read_dma(&mut dma_channel, destination) {
    Ok(transfer) => transfer,
    Err(rejected) => {
        // rejected.sequence and rejected.destination remain usable.
        return;
    }
};
match transfer.await {
    Ok(completed) => {
        let code = completed.destination[0] & 0x0fff;
        // completed.sequence and completed.destination can be used again.
    }
    Err(error) => {
        // Input resources and destination remain quarantined until device reset.
    }
}
```

Use `blocking_wait(poll_budget)` with either channel mode. `DmaStartError` returns
the sequence and destination on rejected setup or launch; `DmaBuffers` returns
them only after clean DMA TC. The ADC is stopped and its DMA request disabled at
every terminal outcome. On cancellation/error/timeout without TC, both the ADC
and DMA channel remain poisoned, and the ADC stays enabled for any outstanding
peripheral read. The owned sequence is deliberately forgotten, preserving any
input-resource guards with destructors as well as its static pin lifetimes.
Per-call borrowed pins and ordinary borrowed OPA outputs remain available for
CPU/ADC-IRQ reads; DMA requires static input guards and static destination memory.

ADC lease state is stored with the generated instance, rather than in a guard
that `mem::forget` can bypass. Reconstruction, configuring a sequence and starting
an existing sequence check that lease. ADC owner Drop and explicit `stop` cancel
an outstanding DMA channel; neither makes an unproven stop reclaimable. DMA owns
the endpoint's terminal hook, removes it before publishing a reusable channel,
and records ADC completion/poison immediately. Thus a forgotten ADC guard cannot
later cancel a new user of the same DMA channel. Constructing another channel
never resets the controller or clears an active ADC endpoint's lease.

This composition follows the pinned Embassy
[`Adc::read_sequence`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/adc/mod.rs#L984-L1026)
pattern of typed ADC configuration, checked DMA binding, raw DMA beneath the safe
endpoint, and ADC cleanup. Its storage and cancellation contract is deliberately
derived from CW: upstream DMA Drop requests reset and waits for `is_running` to
clear, whereas the CW manuals do not give that bus-drain proof.

## Why safe buffers are static, and why cancellation is deliberately restrictive

Rust permits `core::mem::forget` on any future, even one that borrows a stack
buffer. A borrow plus Drop cancellation is not, by itself, a sound DMA lifetime
guarantee. A safe returned future must not leave hardware accessing deallocated
or reused memory when its destructor is skipped.

`copy` therefore takes an owned `&'static mut [u8/u16/u32]` destination and a
`&'static [u8/u16/u32]` source, with equal nonzero lengths up to 65535 words.
The destination is inaccessible while the transfer owns it. A clean TC, with no
TE, returns `CopyBuffers`. Forgetting an active future leaks its destination and
leaves the channel busy. Reuse requires completion to have been observed by IRQ
or polling. A forgotten Blocking transfer stays busy even if hardware has since
finished; dropping its owner performs a final completion check, after which a
reconstructed owner can reuse a clean completed channel. None of these paths
recovers the forgotten transfer's leaked destination.

The two CW manuals define EN only as enable/disable. They do not document that
EN=0/readback drains outstanding DMA bus accesses. STATUS is a terminal outcome
code, not BUSY. SOFTSRC's R0/R1 description concerns software transfer completion,
without a documented abort/error drain protocol. A memory fence orders accesses;
it cannot manufacture a missing peripheral handshake.

Consequently:

1. Clean normal TC is the only buffer-reclamation proof used by this HAL.
2. Drop, `cancel`, timeout and TE request channel disable and remove its trigger
   and IRQ enables, but make no bus-drain promise.
3. Unless normal TC was already observed, those exits permanently consume the
   static buffer references and poison the channel. Dropping a static reference
   does not deallocate or release its backing storage for safe reuse.
4. Poison lives in per-channel static state, not in the owner or transfer. Owner
   reconstruction through a reborrowed channel token cannot clear it. Sibling
   construction cannot reset the controller or clear another channel's state.
   There is no safe recovery API; restart the device.
5. A simultaneous TC+TE is an error. The destination is not returned as successful.

This is a conservative usable static-buffer API, not a claim of general safe
borrowed-buffer cancellation. A safe stack-buffer `blocking_copy` is deliberately
absent: timeout, fault, unwinding or forgotten async state cannot be allowed to
release memory on an unproven stop condition. Hardware validation or additional
vendor drain documentation would be needed to relax this restriction.

## Unsafe peripheral and raw transfers

`unsafe Channel::transfer_raw::<W>(source, destination, count, RawConfig)` supports
software or audited hardware requests, source/destination increment selection,
and BLOCK/BULK. It returns the same `Transfer` lifecycle, with `()` as its payload.
Async `.await` reports real errors; `blocking_wait` supports a polling owner.

This unsafe boundary is essential. A typed MMIO pointer, a route enum, or matching
width does not prove peripheral ownership, register side effects, request pacing,
memory validity or endpoint lifetime. The caller must guarantee all of these,
including that the source remains readable and destination writable across the
entire incremented range. DMA, FLASH-controller and RAM-controller registers
are forbidden endpoints; Flash/SRAM memory arrays are different resources.

The obligations survive Drop, cancellation, error and `mem::forget`: until clean
normal TC or independently established quiescence, keep storage and endpoint
ownership valid. Static raw buffers are a practical low-level choice. Source memory must remain immutable while DMA reads it. Destination memory
must have no live ordinary references or nonvolatile CPU access while DMA writes
it. A caller-audited, target-specific protocol may allow aligned native-width
raw volatile CPU observations without CPU writes or other DMA writers. A volatile
load alone does not establish that protocol, an atomic multiword snapshot or an
automatic waiver of Rust aliasing rules.
Unaudited endpoints remain unsafe. The finite ADC endpoint above and the
[typed UART/SPI endpoints](bus-dma.md) establish their additional contracts
inside their drivers; their stated width/length and quarantine bounds still apply.

`Request` is generated from the selected controller's audited selector bank,
for example L012 `ADC2_SINGLE` and F030 `ADC_CONVERSION`. It is a hardware event
selector, not proof that a peripheral's DMA mode has been configured. Selector
meanings differ substantially between the chips; no STM32 request mux is copied.

## Hardware facts encoded by the driver

| Property | L012 | F030 |
|---|---|---|
| Channel count | 4 | 5 |
| Width and increment | SIZE 0/1/2: 8/16/32 bits, increment 1/2/4 bytes | Same encodings |
| Peripheral width | SIZE explicitly applies to Flash/RAM; peripheral native access must be audited separately | Source/destination widths must match; no mixed-width packing API |
| Count | 1..65535 words, REPEAT fixed to 1 | Same; both count fields must be reprogrammed after completion |
| Software BLOCK | One start transfers all words, with arbitration gaps | Same |
| Hardware BLOCK | One word per request with REPEAT=1 | Same |
| BULK | One trigger transfers whole count, without arbitration gaps | Same |
| Priority | Fixed by channel number, channel 1 highest | Same |
| Completion/errors | Global ISR TC/TE at bits 4*n / 4*n+1 | Same |
| Clear | ICR R1W0; write zero only to own flags, preserve ones in peers/reserved bits | Same |
| Current addresses/count | Separate CCNT/CSRCADDR/CDSTADDR | Not present |
| Restart/address reload | Real RESTART/SRCLOAD/DSTLOAD fields | Not present |

All CSR configuration is constructed from known writable control bits. The driver
never RMW-replays STATUS, L012 TC/TE or TRIG.SOFTSRC. No count-zero-as-65536,
half-transfer IRQ, adjustable priority, width conversion, or remaining-count
promise is invented. Count is words, not bytes, and requests are never silently
substituted.

The L012-only unsafe `start_repeating_raw` returns a `RepeatingTransfer`, not a
finite completion future. It sets RESTART and keeps per-block SRCLOAD/DSTLOAD off;
normal full-transfer restart restores the initial configuration. It provides
`error()` and `request_stop()` only. It does not provide a ring-buffer index,
coherent ADC scan snapshot, overrun detection, lossless event counting, or a
safe concurrent CPU/DMA data-access abstraction. A TC from one iteration is
not proof that repeating DMA has stopped. F030 has no corresponding method.

## Synchronization and evidence

Per-channel phase and wake state use `critical_section::Mutex<Cell<_>>` and the
HAL's `EventState`, so Cortex-M0+ does not need compare-and-swap. Start, interrupt
service and cleanup/latch publication are serialized. Waker registration precedes
completion inspection; IRQ arrival before/during/after polling cannot lose normal
completion. MMIO data publication and successful memory hand-back use compiler
and CPU fences; no fence is represented as an outstanding-write drain.

Primary evidence:

- [CW32L012 User Manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
  sections 8.4–8.8, printed pages 109–123: transfer modes, width, count, repeat,
  errors, control and trigger encodings, endpoint exclusions and L012 restart.
- [CW32x030 User Manual Rev 2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
  chapter 8, especially sections 8.4–8.8: five-channel mapping, trigger bank,
  count/REPEAT requirements and interrupt/control semantics.
- The pinned CMSIS headers and SVDs in `vendor/` establish the generated register
  layout. Manual STATUS has an RW label but no useful write-state contract; the
  HAL does not attempt to write a status code.
- Pinned Embassy source above establishes software architecture, not CW hardware
  compatibility. Upstream STM32 stop waits are not transplanted without evidence.

ARM builds and external register/lifetime probes validate implementation and
compile-time boundaries; they do not establish silicon timing, bus arbitration,
actual abort draining or board-level DMA operation.

## Additional finite bus endpoints

From v0.21, [typed UART/SPI endpoints](bus-dma.md) consume full static bus
owners, channels and buffers. They use generated request/direction traits and
return resources only after clean DMA and peripheral completion. Abort/error
quarantine remains mandatory; UART RX is conservatively one frame.
