# Timer capture and quadrature encoder

The v0.21 capture and encoder drivers adapt the owned timer, pin, and interrupt
contracts from the actual Embassy STM32 sources at
[`b12a6d9efcd2711037abca1b63a661a9ef726444`](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/timer),
specifically `input_capture.rs` and `qei.rs`. Their register operations and
physical input identities are independently mapped to CW32. They do not claim
STM32 register compatibility or its full driver API.

## Audited scope

| Timer IP | Capture inputs | Encoder pair | Overcapture detection |
| --- | --- | --- | --- |
| L012 ATIM | CH1–CH4 | CH1 + CH2 | Yes, CCxOF |
| L012 GTIM1–4 | CH1–CH4 | CH1 + CH2 | Yes, CCxOF |
| F030 ATIM | CH1A/B, CH2A/B, CH3A/B | CH1A + CH1B | Yes, CxyE |
| F030 GTIM1–4 | CH1–CH4 | CH1 + CH2 | Unavailable |

F030 ATIM CH4 is internal and compare-only. Its B inputs are independent
capture inputs, not aliases for the next channel or L012 complementary-output
signals. L012 ATIM CH5/CH6 and internal peripheral input routing are outside
this bounded implementation. Generated capture pin implementations use audited
external input routes. No output-only, complementary-only, or unavailable route
can substitute for an input. The package and board must expose the chosen pins;
oscillator/debug ownership remains the application's responsibility.

Each `InputCapture<'d, T, C, M>` owns one whole timer and one capture pin. It does
not split the timer or perform simultaneous captures on multiple inputs. `Qei`
owns one whole timer and its two required input pins. Both use `Peri` lifetime
ownership and the timer's RCC clock guard. They cannot coexist with PWM or the
other capture/encoder owner of the same timer. GTIM1 remains unavailable through
safe singleton acquisition when the Embassy time driver reserves it.

## Input capture

```rust,ignore
use embassy_cw32::timer::input_capture::{
    CaptureInput, Ch1, Config, Edge, InputCapture,
};
use embassy_cw32::gpio::Pull;

// Example: verified GTIM2 CH1 route on PA0, on either supported chip.
let pin = CaptureInput::<_, Ch1>::from_pin(p.PA0, Pull::None);
let mut capture = InputCapture::new_blocking(p.GTIM2, pin, Config::default())?;
let sample = capture.blocking_capture(Edge::Rising, 100_000)?;
let timestamp = sample.count;
```

Construction initializes and starts a 16-bit PCLK upcounter with ARR=65535,
while capture remains gated. The prescaler is a physical divisor and is checked
against the selected IP: L012 1–65536, F030 ATIM 1/2/4/8/16/32/64/256, F030 GTIM
powers of two through 32768. `tick_frequency()` returns an exact rational tick
rate. A timestamp wraps at 65536; two timestamps' wrapping subtraction is useful
only when the elapsed interval is known to be less than one wrap. Software does
not extend the counter or count missed wraps.

`arm(edge)` begins a polling request, `try_capture()` checks without blocking,
and `cancel_capture()` discards it. `blocking_capture(edge, poll_limit)` has a
finite CPU-dependent polling allowance, not a time deadline. Timeout and
cancellation gate the capture input, mask its IRQ source, clear only its
capture/overcapture flags, and leave the counter running. A new request discards
earlier events and results.

`InputCapture::new` creates the asynchronous form and requires
`Binding<T::Interrupt, input_capture::InterruptHandler<T>>` for the actual timer.
The rising/falling/any-edge futures arm on their first poll. Cancellation performs
the same source-local cleanup. Registering the waker before observing the event
latch/hardware avoids losing an event across task/IRQ scheduling. Captures use
the hardware CCR value, not a software sample of the current CNT.

For L012 GTIM3 and GTIM4, both handlers belong in the single `GTIM34` binding if
both are active. Every ATIM capture event uses its global ATIM vector. Each
handler checks its peripheral's enabled pending capture bits. Constructors and
cleanup never unpend or disable a shared NVIC vector.

The service path gates capture before reading the CCR and status. If multiple
edges arrived before service, the last captured value may have replaced earlier
values. `Capture::overcapture` reports `Some(true)` when hardware detected loss,
`Some(false)` when hardware detected none, and `None` on F030 GTIM because that
IP exposes no capture-loss flag. `None` is not proof of loss-free measurement.
An any-edge capture does not preserve which edge occurred. These are one-shot
requests, not lossless pulse counters, queued events, or DMA waveform capture.

Only physical filter choices with audited meanings are exposed. `None` works
on all supported IPs. `PclkSamples2` and `PclkSamples4` work on L012 timers and
F030 GTIM. F030 ATIM instead offers `PclkSamples3` (encoding 4). Unsupported
choices return `UnsupportedFilter` before timer writes. Filtering and peripheral
sampling constrain the minimum input pulse width; software cannot remove those
electrical limits. GPIO weak pulls are selected when wrapping the input and are
subject to that GPIO's pull capabilities.

## Quadrature encoder

```rust,ignore
use embassy_cw32::timer::qei::{Config, Qei};

// GTIM2's first and second signals are CH1 and CH2, respectively.
let mut encoder = Qei::new(p.GTIM2, p.PA0, p.PA1, Config::default())?;
let position = encoder.count();
let direction = encoder.read_direction();
encoder.reset();
```

The first and second pin types are fixed by the generated timer instance, so two
CH1 pins or F030 ATIM CH1A+CH2A cannot accidentally construct an encoder. Mode1
counts both edges of the first input; Mode2 counts both edges of the second;
Mode3 counts both inputs (x4). Input inversion and each filter/pull are explicit.
SMS=1/2/3 on L012, ENCMODE=1/2/3 on F030 GTIM, and SMS=4/5/6 on F030 ATIM are
separate audited mappings.

The counter range is fixed to 0–65535, satisfying F030 GTIM's mandatory encoder
ARR value. Count and direction are separate observations, not an atomic pair.
Use wrapping arithmetic; deriving signed motion requires a known delta of less
than half the range between observations. `set_count`/`reset` briefly stop the
counter and preserve its previous running state; edges while stopped are lost.
No capture IRQ, index reset, DMA, ADC trigger, or timer output is enabled.

External input selectors are explicitly restored: L012 TISEL/TISEL1 TIySEL=0;
F030 GTIM's metadata-selected SYSCTRL.GTIMxCAP.CHy=0; F030 ATIM IA1S/IB1S=0.
L012 encoder index control is explicitly disabled. This matters when another
live peripheral sharing an RCC resource prevents a hardware reset. Drop stops
the timer and disconnects its owned pins before releasing the clock guard.

## Register and route evidence

- [CW32L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf):
  GTIM sections 16.3.2.8 (encoder, printed pp255–257), 16.3.3 (capture and loss,
  pp258–261), 16.8.6–7 (programming examples, pp280–282), 16.10.8/10/12
  (capture selection/filter/edge gate), and 16.10.21 (external input source).
  ATIM equivalents are sections 17.3.2.8, 17.3.3, 17.8.6–7 and 17.10.28.
  Table 9-2 establishes GPIO AF routes; L012 SDK 1.0.5 independently corroborates
  AF8/AF9 routes. Table 5-1 establishes shared GTIM34 and global ATIM interrupts.
- [CW32x030 RM2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf):
  GTIM sections 14.3.3 (capture and external selector, pp227–228), 14.8.1/2/4/5
  (encoder, filter, capture modes and mandatory encoder ARR, pp243–247), and
  14.8.11–13 (enabled flags and R1W0 acknowledgement, pp248–250).
  ATIM sections 15.3.1.6 (CH4 restriction, p264), 15.3.2.1 (capture remains
  possible with CNT stopped, p265), 15.3.4 (A1/B1 encoder, pp281–282), 15.5.1
  (capture programming), 15.7.4/5 (capture loss and R1W0 flags, pp298–300),
  15.7.7 (distinct filter, pp302–303), and 15.7.11–13 (capture gates and mixed
  software-command fields, pp306–308). Table 5-1 establishes interrupt mapping.
- [Timer route evidence](timer-pwm-route-evidence.md) and
  [F030 pin audit](cw32f030-pins.md) record the supporting GPIO evidence,
  including the resolved RM15-3 CH2A typo: PA4/AF7 is the supported route.

Software validation checks register writes, lifetime/type contracts, capture
cleanup, loss reporting, and interrupt dispatch. It does not establish real
waveform accuracy, noise immunity, interrupt latency, or board-level operation;
those require hardware measurements.
