# Generic complementary PWM

`timer::complementary_pwm::ComplementaryPwm<'d, T>` owns an ATIM, independently
optional main/complementary pins for three channels, and an optional external
brake pin. It supports one pair, an N-only output, or any other verified subset;
it does not require the six-pin motor bundle. `atim::ThreePhasePwm` and its
motor/ADC update protocol remain separate and unchanged.

The shape follows the actual [pinned Embassy complementary PWM driver](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/timer/complementary_pwm.rs):
owned timer, typed main/N wrappers, optional pins and channel duty operations.
The CW interface deliberately bounds that model to audited hardware. It has
three externally routed pairs, edge-aligned upcounting, active-high output
polarity, low brake states and explicit master arming. There is no copied
STM32 four-channel advanced-timer contract, automatic start, raw MOE/AOE setter,
arbitrary comparator/break2 routing, timer DMA or ADC-trigger configuration.

## Ownership and startup

```rust
use embassy_cw32::timer::{Ch1, Ch2};
use embassy_cw32::timer::complementary_pwm::{
    BrakePin, BreakConfig, Channel, ComplementaryPwm, ComplementaryPwmPin,
    DeadTime, PwmPin,
};

// These routes are supported on both chips. Check the actual package and board.
let brake = BrakePin::new(p.PB12, BreakConfig { active_high: false, filter: 0 })?;
let mut pwm = ComplementaryPwm::new(
    p.ATIM,
    Some(PwmPin::<_, Ch1>::new(p.PA8)),
    Some(ComplementaryPwmPin::<_, Ch1>::new(p.PB13)),
    None, Some(ComplementaryPwmPin::<_, Ch2>::new(p.PB14)),
    None, None,
    Some(brake),
    20_000,
    DeadTime::symmetric(20),
)?;
pwm.set_duty_ticks(Channel::Ch1, pwm.max_duty_ticks() / 2)?;
pwm.set_duty_ticks(Channel::Ch2, pwm.max_duty_ticks() / 4)?;
pwm.enable(Channel::Ch1)?; // select this pair; pins and MOE remain disabled
pwm.enable(Channel::Ch2)?; // select the N-only output
pwm.start();               // counter only
pwm.enable_outputs()?;     // deliberate arming after board safety is established

{
    let mut channel = pwm.channel(Channel::Ch1)?;
    embedded_hal::pwm::SetDutyCycle::set_duty_cycle(&mut channel, 32_768)?;
}
pwm.disable_outputs();    // floating inputs, retaining channel selection
pwm.stop();
```

Each main route uses the existing generated `TimerPin<T, ChN>` capability.
Complementary and BK capabilities are independently generated from ATIM
metadata. L012 signals CH1N–CH3N and F030 signals CH1B–CH3B are used as real
complements; a main pin cannot substitute for them. Ch4, GTIM, unrelated GPIOs,
SWD PA13/PA14 and unverified routes are unconstructible through this API.
Three channels are an API boundary: L012 has additional timer hardware, but
that hardware is outside this audited generic complementary interface.
A runtime channel with neither pin returns `ChannelUnavailable` before writes.

Constructors leave the counter stopped, MOE/AOE clear and output pins
floating. Passing `None` for the brake explicitly disables external BK
protection; it does not imply a hidden comparator or software safety source.
The external brake wrapper validates the hardware filter before changing its
pin and remains owned until the PWM owner drops. Its input is connected only
after timer initialization, before external brake enable.

`enable(channel)` selects a pair for the next `enable_outputs` and requires
MOE clear. `is_enabled(channel)` reports that selection, not the instantaneous
master gate: consult `outputs_enabled()` and `fault_pending()` as well.
`disable(channel)` disconnects only that channel's owned pins. It does not
stop the counter or write MOE=1 for a peer. `disable_outputs()` disconnects all
owned output pins, retains channel selection and leaves the counter running.
`stop()` alone can retain an armed static output level.

L012 enables both internal CCE and CCNE for every selected pair, even when only
one pin is supplied. RM table 17-13 removes complement/dead-time behavior when
only one internal gate is enabled. Missing physical pins remain unmuxed. F030
has no equivalent per-pair gate: its pair is disconnected through the owned
GPIO mux while the shared MOE gate continues to protect all outputs.

The driver owns the timer's existing RCC `ClockGuard`; there is no caller-
fabricated clock. A borrowed channel handle excludes concurrent owner access
or a second mutable handle. Drop clears MOE, disconnects outputs, stops the
counter, disconnects BK and only then releases the timer clock. Forgetting an
owner retains its resources. There is no independently owned channel split,
because master arming, brake acknowledgment and dead time are shared resources.

## Duty endpoints and update boundaries

The native range is `0..=period_ticks`, where periods are `2..=65536` and
`ARR = period_ticks - 1`. The borrowed embedded-hal handle has a normalized
`u16` scale: `65535` always requests exact 100% main duty.

A duty of zero requests main low/complement high; full scale requests main
high/complement low, after dead-time settling. Neither endpoint means both
outputs off. Use `disable_outputs` to remove timer drive. Disconnected pins
are floating, so external pulls and gate-driver interlocks must establish the
required electrical state.

Both ATIM backends preload per-channel compares. A running `set_duty_ticks`
writes only that channel's compare; it takes effect at a hardware update.
Multiple calls can straddle an update and are not an atomic group commit.
A CPU critical section does not stop hardware update events. No F030 RCR trick
or unsupported shadow-load lock is claimed. These generic per-channel updates
do not relax the distinct F030 `ThreePhasePwm` live three-phase update contract.
When stopped, software UG loads the pending compare with timer/ADC trigger
routes gated. Arming a running timer may use an earlier active compare until
the next update transfers the latest requested value.

At a 65536-tick period, CCR cannot represent the full endpoint. The driver
uses forced-active reference mode for exactly 65536. Entering or leaving that
endpoint requires MOE clear and otherwise returns `OutputsEnabled` before
writes. It never silently disarms and rearms around a mode change. Ordinary
duty changes, including full scale at shorter periods, remain per-channel
preload writes.

`set_config` and `set_frequency` also require disarmed outputs. They validate
first, disconnect outputs, stop/load/reset phase and restore only the prior
counter-running state. Selected channels stay selected, but pins and MOE stay
disabled until another explicit `enable_outputs`. Duty fractions are preserved
with rounding down. No timing/dead-time change can automatically recover a
broken power output.

## Dead time is chip-specific

`DeadTime { rising_ticks, falling_ticks }` requests a minimum; hardware values
round upward. `dead_time()` and `config()` report the actual rounded values.
`dead_time_clock_ratio()` reports the exact tick frequency as a numerator and
denominator. `dead_time_clock_hz()` rounds that frequency down to whole hertz.

| Contract | CW32L012 | CW32F030 |
| --- | --- | --- |
| Tick clock | PCLK with CKD=/1 | TTCLK = PCLK / counter divider |
| Rising/falling | Independently selectable | Must be equal |
| Zero request | DTG/DTGF zero | DTEN off |
| Positive minimum | 1 tick | 2 ticks; request 1 rounds up |
| Maximum | 1008 ticks | 1010 ticks |
| Hardware encoding | Four-segment DTG | Same segment steps plus 2 TTCLK ticks |

L012's first segment is 0–127 ticks; later segments use 2, 8 and 16 tick steps.
F030's enabled code zero means 2 ticks and its maximum code means 1010 ticks.
Changing the F030 divider changes dead-time duration even if its tick count
stays constant. L012 dead-time duration is independent of PSC. The API rejects
unrepresentable values and asymmetric F030 requests before writes. L012 DTAE
is changed only with CEN clear, as required by the manual.

## Break and rearm policy

External BK uses the explicitly supplied polarity. L012 BKF accepts 0–15;
F030 FLTBK accepts 0 or 4–7 (three consecutive samples at PCLK /1, /4, /16 or
/64). Zero is unfiltered. Clocked filtering is not clock-loss protection.
Only the supplied external input is configured; L012 BK2 and internal
comparator routes, and F030 VCE/SAFEEN, stay disabled.

AOE stays zero. Arming first removes timer drive and checks latched faults,
then writes MOE=1 once while output pins remain disconnected. It checks the
fault latch/MOE, connects the selected pairs and pins, then checks again.
There is no later MOE=1 write that could override a break arriving during that
sequence. A hardware break can still arrive after a successful return.

`acknowledge_fault()` disables outputs first, clears only hardware break
flags with the IP's R1W0 semantics and reports a persistent fault. It never
rearms. L012's BIF/B2IF/SBIF are treated as faults; F030 exposes one combined
BIF, so this interface does not invent separate fault-source attribution.
The application must resolve the source, acknowledge, establish board safety
and deliberately call `enable_outputs` again.

The low brake states and pin ownership are peripheral behavior, not a power-
stage safety certification. Electrical timing, break propagation, external
interlocks and gate-driver polarity require measurement on the actual board.

## Evidence and verification

The implementation was checked against the exact upstream source above and:

- [CW32L012 RM1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf), §17.3.4.7–8, tables 17-12 and 17-13, and §17.10.25–26: dead-time encoding, CCE/CCNE interaction, break/idle states, AOE and DTAE restrictions.
- [CW32x030 RM2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf), §15.3.1.6, §15.3.3.5–6, table 15-9 and §15.7: three A/B pairs, COMP/single-point mode, buffered A compare, dead time, filter and fault controls.
- Existing chip AF metadata and the original RM GPIO AF tables. The real main/N/B/BK routes are generated from these facts; no new speculative route was added.

Verification uses external Rust probes and actual ARM builds; no probe harness
or tests are shipped in the source package. Register-memory probes exercise
the real HAL and typed PAC with synthetic MMIO, including endpoints, rejected
writes, independent channel selection, dead-time settings, fault acknowledgment,
Drop and RCC release. Fault-injection sequencing probes check breaks around
the single arm write. Compile probes verify valid typed construction and reject
wrong routes/timer/channel, duplicate ownership and overlapping borrows.
Synthetic MMIO does not emulate PWM edges, shadow-transfer timing or electrical
break latency. No board measurements were performed.
