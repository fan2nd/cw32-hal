# CORDIC instance ownership

`Cordic<'d, T: Instance>` and `AsyncCordic<'d, T: Instance>` retain the concrete
peripheral type supplied by `Peri<'d, T>`. Constructors infer `T` from that
token. Explicit type annotations now name the instance, for example
`Cordic<'static, peripherals::CORDIC>`.

This follows the ownership shape of Embassy STM32's
[`Cordic<'d, T: Instance>`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/cordic/mod.rs).
It does not imply compatible registers or mathematical conventions. CW32L012
keeps its separate X/Y/Z registers, final-input trigger, signed Q1.31 values and
angles in units of pi. CW32F030 has no CORDIC driver.

## Generated hardware binding

The sealed `Instance` contract combines Embassy's `PeripheralType`, the
metadata-generated peripheral clock resource, the actual PAC register block,
the physical type-level interrupt and a dedicated asynchronous state. HAL
`build.rs` emits implementations for independently owned CORDIC instances and
requires one audited `GLOBAL` interrupt association.

Every blocking and asynchronous register access uses the selected instance's
register block. Acquisition, reset, release and late-interrupt clock checks use
its clock resource. Each generated implementation supplies its own completion
event and result bank. The driver does not retain a fixed `pac::CORDIC` access,
global result bank or global completion event.

The only currently generated instance is `peripherals::CORDIC` on CW32L012.
Sealing prevents application code from declaring an arbitrary peripheral or
address to be a CORDIC. The generic shape does not claim additional physical
accelerators.

## Interrupt binding and lifetime

```rust
use embassy_cw32::{
    bind_interrupts,
    cordic::{AsyncCordic, Cordic, InterruptHandler},
    peripherals::CORDIC,
    Peri,
};

bind_interrupts!(struct Irqs {
    CORDIC => InterruptHandler<CORDIC>;
});

fn new_async<'d>(token: Peri<'d, CORDIC>) -> AsyncCordic<'d, CORDIC> {
    Cordic::new(token).into_async(Irqs)
}
```

`into_async` requires `Binding<T::Interrupt, InterruptHandler<T>>`; the handler
and vector must belong to the same instance. `into_blocking` preserves that
instance, and `release` returns `Peri<'d, T>`. A borrowed token remains borrowed
until its owner is released or dropped. An in-flight future retains its mutable
owner borrow.

## Preserved operation protocol

- Input validation and a zero polling budget reject before hardware access.
  BUSY rejects before writes. A blocking timeout leaves the operation running;
  a later operation can proceed once BUSY clears or after an explicit reset.
- Two-input operations write X before Y. The final input triggers computation.
  Completion reads only the selected result registers, once each, without
  waiting for EOC again after the first result read clears it.
- Async operations are lazy until first poll. Launch checks BUSY, resets the
  selected event, drains stale results, enables the peripheral completion
  source and writes inputs in the same order.
- The interrupt handler checks the selected clock before MMIO. It requires IE,
  EOC and idle status, masks IE, reads the result bank, then publishes results
  and the event in the same critical section. Waking remains outside that
  section.
- Completion cleanup, cancellation, conversion and Drop retain their existing
  peripheral IE masking and exclusive-reset rules. Forgotten clock owners can
  prevent a reset; the instance conversion does not weaken that protection.
  NVIC pending state is not cleared or disabled as part of conversion.

The refactor adds no operation-mode hierarchy, DMA path or new numerical
algorithm. Register-model and compilation checks do not establish physical
reset timing, interrupt latency, convergence accuracy or endpoint behavior.

## Verification

External probes compile the actual driver source against modeled registers,
clock resources and interrupts. All 16 shared protocol/lifecycle cases pass
against both the previous concrete driver and this implementation. Two added
cases pass with distinct modeled instances, including cancellation and release
of one owner while the other has a latched completion. The latter retains its
event, result, waker, clock and ability to start the next operation. An interrupt
after clock release performs no MMIO.

The public API passes Rust 1.99.0 release code generation for
`thumbv6m-none-eabi`, including generic construction, typed interrupt conversion,
all async operations and token release. Seven compile-fail cases reject a wrong
peripheral, wrong vector, missing binding, reused token, simultaneous reborrows,
moving an owner with a live future and an externally forged `Instance`.
Formatting and ARM rustdoc with warnings denied also pass. Probe code remains
outside the release source tree; no board execution is claimed.
