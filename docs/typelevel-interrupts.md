# Embassy-style type-level interrupt wiring

The comparison is pinned to Embassy
[`b12a6d9efcd2711037abca1b63a661a9ef726444`](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444):
[`embassy-hal-internal/src/interrupt.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-hal-internal/src/interrupt.rs),
[`embassy-stm32/src/lib.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/lib.rs), and
[`embassy-executor/src/platform/cortex_m.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-executor/src/platform/cortex_m.rs).

## What was already correct

The HAL uses the real `embassy-hal-internal` `interrupt_mod!`. Its metadata-generated
interrupt list produces sealed `typelevel::Interrupt` identities, including `IRQ`
and NVIC operations. `Handler<I>` is the synchronous ISR body contract;
`Binding<I,H>: Copy` proves that exact handler is called on that exact vector.

`bind_interrupts!` already emitted the actual strong IRQ function, synchronous
calls to each listed `Handler<I>::on_interrupt`, and corresponding unsafe Binding
implementations. It follows the pinned upstream macro, including shared lists
and conditional handlers. CW additionally rejects an unknown IRQ even for an
empty/fully-disabled handler list, and exposes the macro only with `rt`.

ADC, comparator, GPIO input, DMA, specialized ATIM and CORDIC interrupt-backed
constructors already required these real proofs. They have not been replaced by
a second interrupt framework.

## The v0.16.0 correction

Previously the ownership-bypass motor API selected ADC/basic timers at runtime
and enabled their source interrupts without requiring Binding. The board examples
also manually installed external IRQ functions. Now:

- `AdcScan<T: adc::Instance>` reuses the existing metadata-derived ADC identity,
  register pointer and associated IRQ. ADC2 therefore uses ADC2_DAC, not a guessed
  ADC2 vector.
- `BasicTimer<T: motor::BasicTimerInstance>` gets its register pointer and exact
  single GLOBAL IRQ association from metadata. BTIM3 uses BTIM3_HALLTIM.
- `enable_sequence_interrupt::<H>(irqs)` and
  `enable_update_interrupt::<H>(irqs)` require `Binding<T::Interrupt,H>` with
  `H: Handler<T::Interrupt>` before changing the hardware source enable.
- The source-enabling methods still do not clear flags or change NVIC state or
  priority. Polling and DMA-only setup do not require unrelated IRQ proofs.
- The former `AdcUnit`/`TimerUnit` selectors are replaced by type parameters.
  Acquiring a motor handle remains unsafe and does not consume or manufacture a
  peripheral singleton. Typed identity is not exclusive ownership.

For example, an application supplies its actual ISR implementation:

```rust,ignore
use embassy_cw32::{bind_interrupts, interrupt::typelevel, motor::AdcScan, peripherals};

struct AdcHandler;
impl typelevel::Handler<typelevel::ADC1> for AdcHandler {
    unsafe fn on_interrupt() {
        // Check/acknowledge ADC1 and process the sample synchronously here.
        // The board must uphold its motor resource/concurrency contract.
    }
}
bind_interrupts!(struct Irqs { ADC1 => AdcHandler; });

// After validated board setup, under the motor API's exclusion contract:
unsafe {
    AdcScan::<peripherals::ADC1>::acquire()
        .enable_sequence_interrupt::<AdcHandler>(Irqs);
}
```

The example sketch intentionally leaves application work unspecified. A Binding
proves that the handler runs, not that it acknowledges the right flag, obeys a
latency bound, or implements a correct motor algorithm.

## Board program migration

02–05 now declare real Handler implementations and use `bind_interrupts!`.
06 centralizes wiring in `main.rs`:

```rust,ignore
embassy_cw32::bind_interrupts!(struct Irqs {
    ADC1 => motor::AdcHandler;
    BTIM1 => motor::TickHandler;
    BTIM3_HALLTIM => motor::CommutationHandler;
    UART2 => motor::MotorExecutorHandler;
});
```

The ADC/timer Handler bodies retain the original source check/acknowledgment,
sample processing, commutation and state changes. These run in the hardware ISR
before the existing motor notification. There is no callback registry, queued
hardware event framework or deferred replacement for the hard ISR work.

The motor executor start function requires
`Binding<typelevel::UART2, MotorExecutorHandler>`. The adapter's handler only
calls the official executor's `on_interrupt`; startup passes
`typelevel::UART2::IRQ` to `InterruptExecutor::start`. This conversion is deliberate:
the actual upstream executor takes `InterruptNumber`, not Binding. It initializes
the executor before unmasking its IRQ. UART2 remains reserved as a software wake
vector with its peripheral unused, at P1 alongside the motor hardware ISRs. The
ordinary thread executor continues to run UI work.

Type-level NVIC operations keep the same priorities and startup ordering. The
shared BTIM3_HALLTIM pending bit is no longer indiscriminately unpended; the board
clears BTIM3's own source and leaves unused HALLTIM disabled. Adding a HALLTIM
user requires its own source service in the same vector and coordinated priority.

## Shared vectors and deliberate exceptions

Every handler listed on a shared vector is called synchronously and in list order
on every entry. A handler checks and services its own source. A Binding for the
same IRQ but a different handler is insufficient. A macro cannot discover an
unlisted active hardware source or automatically install its correct service.

Two kinds of entry are intentionally separate:

1. Cortex-M core exceptions such as HardFault/NMI retain the runtime's
   exception mechanism. The panic handler and DefaultHandler fallback also
   remain separate fatal entries; DefaultHandler is the fallback for otherwise
   unbound exceptions/device vectors, not a peripheral-specific Binding.
2. The optional GTIM1 time driver installs its private strong IRQ entry and removes
   GTIM1 from application `Peripherals`. This matches the pinned upstream's
   [build-generated time-driver IRQ](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/build.rs#L431-L440),
   rather than requiring the application to bind a resource it cannot acquire.
   Its NVIC operations use `typelevel::GTIM1`; counter, queue and wake algorithms
   are unchanged.

Binding is dispatch evidence, not a lock, a priority configuration, pin ownership
or permission to alias motor state. The unsafe motor concurrency contract and
DMA lifetime/cancellation contract remain fully applicable. Custom startup/vector
relocation must preserve the runtime dispatch contract, as with upstream Embassy.
