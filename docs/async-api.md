> Motor/custom ISR wiring also uses these contracts; see [type-level interrupt migration](typelevel-interrupts.md).

# Typed owners and real interrupt-backed operations

Current APIs use one `Adc<'d, I, M>` and one `Comp<'d, I, M>` owner with sealed `Blocking`/`Async` modes. GPIO interrupt input has a real port-IRQ backend. These are not blocking loops wrapped in async syntax. ATIM event and CORDIC futures retain their existing specialized owners; not every driver needs the same mode shape.

All examples below are API sketches requiring HAL initialization, valid board wiring and a real `DelayNs` implementation. No silicon or electrical validation is claimed.

## Binding and shared interrupts

```rust
use embassy_cw32::{adc, analog, gpio, bind_interrupts, peripherals};
bind_interrupts!(struct Irqs {
    ADC1 => adc::InterruptHandler<peripherals::ADC1>;
    ADC2_DAC => adc::InterruptHandler<peripherals::ADC2>;
    VC13 => analog::InterruptHandler<peripherals::VC1>,
            analog::InterruptHandler<peripherals::VC3>;
    GPIOA => gpio::InterruptHandler<peripherals::PA0>,
             gpio::InterruptHandler<peripherals::PA1>;
});
```

This is the L012 vector naming. F030 uses its own generated interrupt names. Every enabled source sharing a physical vector must have its handler included. One source's completion, cancellation or drop does not disable/unpend the shared vector or clear another source's flags. Bindings prove the exact peripheral or GPIO pin handler; `()` and a binding for another pin are rejected.

## ADC owner, borrowed channels and sequences

```rust
let mut adc = adc::Adc::new_blocking(p.ADC1, adc::Config::default(), &mut delay)?;
let mut pin = p.PA0;
let sample = adc.blocking_read(&mut pin, adc::SampleTime::Cycles70, 10_000)?;
```

The async constructor is `Adc::new(instance, binding, config, delay)`. `read(channel, sample_time).await` borrows the channel for that operation. Verified typed `Peri` pins implement the sealed channel contract. `AdcChannel::reborrow_adc` creates an exclusive borrow; `degrade_adc` consumes the token into a non-cloneable erased channel. Channel tokens cannot be fabricated from integers. The owner derives its frequency from validated RCC state; no caller-selected `Clocks` parameter remains.

`configure_sequence([(channel_borrow, sample_time); N])` returns a `Sequence` which borrows both the ADC owner and its channels. It is the explicit fixed-N/FOC extension, not a second owner. It provides bounded `blocking_sample`, watchdog setup, ATIM arm/capture, and for Async mode `sample().await`/`sample_atim_update(&pwm).await`. L012 paired ADC sampling accepts two sequences. A live sequence/future prevents reuse of its owner or pins; cancellation completes source cleanup before those borrows end. Dropping channel tokens leaves the pin analog, matching the upstream borrowed-channel convention. Starting a new valid Sequence first stops any forgotten predecessor and clears its watchdog configuration/faults; invalid arguments still cause no MMIO.

CW-specific limits remain real:

- L012: up to8 slots, conservative ADC clock at most6MHz,32µs startup settling. Paired sampling is blocking; independent async reads are not claimed hardware-synchronous.
- F030: up to4 slots, one shared sample-time setting, conservative at most500kHz,40µs startup plus READY checking.
- F030 one-slot operations use actual MODE0/CHMUX/RESULT0/EOC. Longer scans use MODE4/SQR/EOS.
- F030 hardware watchdog operates only in single-channel mode (official RM2.5 §22.9). Multi-slot watchdog requests return `UnsupportedWatchdogMode` before hardware writes; N=1 selects the borrowed channel in WDTCH. No software watchdog is substituted. AUTOSTOP remains disabled, per §22.7/§22.13.8.

Handlers disable incoming triggers, clear the correct completion source before checking START, and wait for an already-started final conversion before publishing a coherent result. Late EOC/EOS races are handled for the actual mode. Triggered capture may return a later complete scan; it is not lossless streaming or a guaranteed first-edge capture. Cancellation disables the relevant completion enables, disarms/stops this ADC, clears its completion state/waker, and preserves shared BGR/reset/NVIC ownership. No implicit async timeout exists; timeout/select cancellation works by dropping the future.

Removed APIs: permanent `AnalogInput` arrays, `AsyncAdc`, and ADC `into_async/into_blocking` wrappers. Current types/functions are the migration authority.

## Comparator owner and explicit startup

`Comp::new_blocking(...)` constructs `Comp<I, Blocking>`; `Comp::new(..., binding, ...)` constructs `Comp<I, Async>`. Both configure the unit disabled. L012 `enable(&mut delay)` performs the required settling; F030 `enable() -> Result` checks readiness. `output_level` and async waits return errors when disabled/unready rather than silently enabling hardware.

L012 external/reference/DAC constructor variants retain their pin and Bandgap/RefDivider/DAC borrows. F030 preserves shared ADC/BGR ownership and its mode-generic comparator brake guard. Disabling/dropping a comparator first handles any armed F030 PWM brake safety path before removing protection routing.

Async mode provides rising/falling/any-edge and high/low waits. Waker registration, arming and recheck are synchronized; ISR latches under the same critical section and wakes outside it. Cancellation masks/clears only this comparator's event subscription. It does not turn off a comparator still owned by the caller or disturb a sibling's shared IRQ state. Drop performs complete owner shutdown. Single hardware flags coalesce transitions; these notifications are not an edge counter or lossless waveform record.

Removed APIs: separate `Comparator`/`AsyncComparator` owners and their conversion wrapper. `Comp<I,M>` is the actual mode-bearing owner.

## GPIO async Wait follows CW32 port IRQs

```rust
let mut key = gpio::InterruptInput::new(p.PA0, Irqs, gpio::Pull::Up);
key.wait_for_falling_edge().await;
```

`InterruptInput` owns a real `Input/Flex`, checked pin-specific Binding and private event state. It exposes synchronous level queries, five inherent async waits and `embedded_hal_async::digital::Wait`. The inherent waits return `()`; the embedded-hal adapter returns `Result<(), Infallible>`.

CW32 has one vector per GPIO port, not STM32 EXTI-line ownership. Distinct same-number pins on different ports do not compete for a fictitious EXTI token. Each generated pin handler serves only its enabled pending bit; include all active pin handlers on that shared vector. Per-pin local statics avoid one always-live full-chip state array.

First poll clears stale state, registers the waker, arms and rechecks under one critical section. F030 level waits use HIGHIE/LOWIE; L012 arms the relevant edge and rechecks the actual level. R1W0 acknowledgment uses the implemented-pin mask and preserves peer flags/enables. Wait cancellation and owner drop mask/clear only this pin, including a forgotten future, without disabling/unpending shared NVIC. The existing Input/Flex then handles pin disconnect. Filtering is preserved; debounce configuration is not invented.

Events before arming are discarded; multiple edges may coalesce, any-edge direction is not retained, and level may change again before the task resumes.

## Other event drivers and boundaries

`AsyncThreePhasePwm` still waits for actual ATIM update/break events while preserving the specialized six-output/break-protected owner. Cancelling a wait is not an emergency stop and does not disable the running counter or hardware break; fault recovery remains explicit. `AsyncCordic` waits for actual EOC and resets its dedicated accelerator on cancellation. These event waits do not stand in for generic timer or DMA transfers.

## DMA transfer cancellation is a different memory contract

The [DMA driver](dma.md) consumes the complete controller and partitions its actual
channels. Each async channel requires its own checked handler binding, even when
several channels share one physical IRQ. Its handler services only that channel.

Safe asynchronous memory copies own static source/destination buffers. Successful
normal transfer-complete returns them; cancellation, transfer error or a blocking
poll-budget timeout quarantines the buffers and poisons the channel. Forgetting a
transfer cannot make the DMA destination into dangling memory. `copy_mut` also
returns the exclusive mutable source on clean completion. Finite ADC DMA owns
static input-channel guards and a static destination, with persistent ADC/DMA
state across forgotten futures; it returns both only after normal TC. L012
supports 1–8 slots and F030 one MODE0 conversion. Ordinary borrowed buffers and
arbitrary peripheral MMIO endpoints require explicit unsafe transfer contracts
which remain in force after cancellation or forgetting the handle.

This conservatism follows an important CW documentation limit: clearing channel
EN is not documented as draining all outstanding AHB accesses. A CPU barrier is
not a substitute for another bus master's completion. See the [hardware
evidence](dma-hardware-evidence.md) before choosing an unsafe endpoint protocol.

No continuous ADC DMA, calibration/internal-channel ownership API, lossless GPIO/comparator event queue, STOP-clock handling, hardware-in-loop testing or complete upstream-driver feature parity is implied.
