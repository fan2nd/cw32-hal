# Owned timers and single-ended PWM

This layer follows the owned `Timer`, typed `PwmPin`, borrowed channel and
consuming static `split` structure of the [pinned Embassy timer sources](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/timer).
It implements CW32 hardware contracts rather than STM32 register compatibility.

## Owners and channels

`timer::low_level::Timer<'d,T>` owns one ATIM or GTIM1–4 `Peri`. Construction
stops the counter and masks its IRQ/DMA/trigger sources. It exposes PCLK
edge-aligned upcounting, checked divider/period/frequency, counter access,
explicit start/stop and stop-on-Drop. When the Embassy time driver is enabled,
GTIM1 remains absent from the application's safe `Peripherals` fields.

`timer::simple_pwm::SimplePwm` owns the timer and up to four independently
optional `PwmPin` values. Each pin must have an exact metadata-generated
(timer, channel, AF) route; an erased arbitrary GPIO does not satisfy it.
An unsupported or absent channel is rejected. SWD PA13/PA14 routes are withheld.

```rust
use embassy_cw32::timer::{simple_pwm::{PwmPin, SimplePwm}, Ch1, Channel};

// Both chips have this audited ATIM.CH1/CH1A route. Check package/board wiring.
let ch1 = PwmPin::<_, Ch1>::new(p.PA8);
let mut pwm = SimplePwm::new(p.ATIM, Some(ch1), None, None, None, 20_000)?;
pwm.set_duty_ticks(Channel::Ch1, pwm.max_duty_ticks() / 2)?;
pwm.enable(Channel::Ch1)?; // connects this output, does not start the counter
pwm.start();
{
    let mut channel = pwm.ch1()?;
    embedded_hal::pwm::SetDutyCycle::set_duty_cycle(&mut channel, 32_768)?;
}
```

The channel handle exclusively borrows its owner. It cannot coexist with an
owner frequency change, a second mutable handle or owner Drop. Native duty is
`u32`, from zero through `period_ticks` inclusive. The embedded-hal `u16` duty
interface is normalized: `u16::MAX` means 100%, not a truncated native compare.

For independent drivers or tasks, `SimplePwm<'static, T>::split(self)` consumes
the owner and returns `SimplePwmChannels<T>`. Its `ch1` through `ch4` fields are
`Option<OwnedPwmChannel<T>>`: each present handle owns exactly one pin and can
move independently. Handles cannot be cloned. Missing pins stay `None`, so
this does not invent a fourth external F030 ATIM channel.

```rust
use embassy_cw32::timer::{low_level::Config, simple_pwm::{PwmPin, SimplePwm}, Ch1, Ch2};

let mut pwm = SimplePwm::new_with_config(
    p.ATIM,
    Some(PwmPin::<_, Ch1>::new(p.PA8)),
    Some(PwmPin::<_, Ch2>::new(p.PA9)),
    None, None,
    Config { prescaler_divisor: 1, period_ticks: 1000 },
)?;
pwm.start(); // Select the shared running state before consuming the owner.
let channels = pwm.split().map_err(|(error, _owner)| error)?;
let mut first = channels.ch1.unwrap();
let mut second = channels.ch2.unwrap();
first.set_duty_cycle(32_768)?;
first.enable();
second.set_duty_cycle(u16::MAX)?; // exact 100%; first channel keeps running
second.enable();
drop(first); // second retains its output, counter phase and clock ownership
// Move `second` into any driver/task accepting embedded_hal::pwm::SetDutyCycle.
```

Split freezes the frequency and counter running state. Start/stop and timing
changes therefore belong to the whole owner before the split; channel handles
offer only duty and individual output-enable control. A stopped split stays
stopped. Split requires `period_ticks <= 65535`: the full 0–100% range then fits
the compare register, with no forced-mode transition or peer-wide phase reset.
At 65536 it returns `(Error::SplitPeriodTooLong, original_owner)` without
changing registers, pins, duties, or running state. The returned owner can be
reconfigured or continue using the full native unsplit range.

All shared output-register updates, ATIM master-output enable changes and
channel lifetime counts are protected by one critical section per operation.
Dropping a channel disconnects only its pin; the final channel drop stops the
counter and releases its clock guard. Forgetting any channel retains that
timer/clock ownership, even if the forgotten channel was disabled. Splitting
an owner with an accepted period and no pins stops/releases it and returns
four `None` fields.
The implementation transfers the original guard into per-instance state; it
does not clone the whole timer owner.

`PwmPin::new` disconnects its pin. `SimplePwm::new` leaves all channels disabled
and the counter stopped; enabling and starting are separate operations.
Disabled pins are floating inputs, not guaranteed electrically low. `stop`
only stops counting; call `disable`/`disable_all` to disconnect outputs.
Drop gates outputs, stops counting and disconnects owned pins. This is not a
motor-protection interface: it configures no complementary output, dead time,
break input, ADC trigger, timer IRQ or DMA.

## Clocks, periods and update boundaries

The clock comes from the timer's metadata-selected kernel clock and validated
RCC state, not a caller-supplied snapshot. A timer keeps an RCC clock guard
through its final owner; a shared gate cannot be reset or disabled while a
peer still owns it. On both audited chips these timers use PCLK.
`Config { prescaler_divisor, period_ticks }` uses physical quantities; supported
periods are 2..=65536. This API range is not a claim that every other silicon
setting is forbidden. The legal dividers are:

- L012 ATIM and GTIM: every integer from 1 through 65536
- F030 ATIM: 1, 2, 4, 8, 16, 32, 64, 256, with no /128
- F030 GTIM: powers of two from 1 through 32768

The integer-Hz solver selects the smallest legal divider, then rounds period
upward, prioritizing resolution and never exceeding the requested rate. It is
not a nearest-frequency optimizer. `Frequency::ratio()` reports exact
numerator/denominator Hz; `hz()` floors that value. Explicit Config supports
fractional rates below 1 Hz without pretending they equal zero frequency.

Ordinary duty writes update only the selected CCR. L012 and F030 ATIM use
per-channel preload, taking effect at a hardware update; F030 GTIM writes are
unbuffered and can change the current cycle. Enabling a channel while running
may initially use the previous active compare until the next update. It does
not reset phase just to force a new preload into that channel.

Frequency/configuration changes gate outputs, stop, write/load and restore
prior enabled/running state. This deliberately resets phase and can create a
floating interval. At a 65536-count period, a 16-bit CCR cannot hold 65536;
100% uses the actual forced-active mode. On the unsplit owner, entering/leaving
that endpoint uses the same gated phase-reset transaction. Independent split
channels reject that period when splitting, so none can cause this transaction
on a peer. Other duty changes do not pause peer channels. Zero and full-scale
never wrap through a narrowing cast.

Software update events are issued while stopped with TRGO routes gated,
including L012 ATIM MMS2 and F030 ATIM MSCR. Generic timer initialization never
silently emits an ADC update trigger. It does not promise phase-continuous
frequency changes, simultaneous multi-channel atomic commits or glitch-free
live updates on unbuffered hardware.

## Audited routes and remaining features

The [route evidence](timer-pwm-route-evidence.md) contains 90 additions:
L012 41 GTIM routes plus three ATIM.CH4 routes; F030 46 GTIM routes. ATIM and
GTIM main outputs are therefore usable beyond the old six-pin motor subset.
Both chips have GTIM CH1–4; L012 ATIM exposes four audited main channels.
F030 ATIM.CH4 is internal-only and cannot construct a PwmPin. Pin identities
and routes describe chip capability, not a package's bonding or PCB connection.

From v0.21, separate [input capture/QEI](timer-capture-encoder.md) modules
provide actual input ownership and checked capture IRQ waits, and
[generic complementary PWM](complementary-pwm.md) supports optional ATIM pairs
and an external brake. Arbitrary slave/external-clock modes, timer DMA and
broader timer IRQ operations remain outside the implemented scope. Existing
`atim::ThreePhasePwm` retains its specialized complementary/break and three-phase
update contracts. Its constraints are not relaxed by these separate owners.

For both-chip builds, negative ownership/route checks and real-source synthetic
MMIO sequencing coverage, see [validation](validation-v0.18.0.md).
No device timing or electrical behavior was measured.
