# Runtime clocks and low-power boundary

The bounded v0.22 HAL keeps its configured system clocks fixed after startup.
Ordinary CPU sleep is compatible with the existing drivers while their clocks
continue running. Runtime reclocking and managed DeepSleep entry/resume are not
implemented. This is a firmware integration boundary: **both chips document
clock switching and DeepSleep in hardware**.

This audit adds no placeholder `reconfigure`, `stop`, or sleep-wrapper API. A
function which only errors, or only executes an idle instruction, would not
provide clock, wakeup or elapsed-time integration. Startup clock choices are
described separately in [RCC resources](rcc-resources.md).

## What the manuals actually specify

The source revisions are CW32L012 User Manual V1.4 and CW32x030 User Manual
Rev 2.5. Page numbers below are the printed manual pages.

| Property | CW32L012 V1.4 | CW32F030 Rev 2.5 |
| --- | --- | --- |
| System-clock switching exists | HSE, LSE, HSI and LSI, §§4.5–4.5.2, pp.38–40 | HSE, LSE, PLL, HSI and LSI; PLL transitions only through HSE/HSI, §§4.5–4.5.2, pp.61–63 |
| Ordinary Sleep | CPU stops; peripheral clocks and I/O state remain unchanged, §§3.3–3.4, pp.16–20 | Same distinction, §§3.3–3.4, pp.35–39 |
| DeepSleep clock behavior | HSE/HSIOSC automatically stop; LSE/LSI/RC10K retain their previous state, §4.3.2, p.27 | HSE/HSIOSC/PLL automatically stop; LSE/LSI/RC10K/RC150K retain their previous state, §4.3.2, p.46 |
| Entry prerequisite | Flash erase/program must finish (`FLASH_ISR.BUSY=0`), `FLASH_CR1.MODE=0`, §3.3.1, p.17 | Flash erase/program must finish (`FLASH_CR1.BUSY=0`), `FLASH_CR1.MODE=0`; **HCLK must be at most 4 MHz before entry**; enabled VC must be READY, §3.3.1, p.36 |
| `WAKEUPCLK=0` | Wait for the previous system-clock source to become stable, §4.3.2, p.27 | Wait for the previous source; HSE/PLL restoration can be slower, §4.3.2, p.46 |
| `WAKEUPCLK=1` | Switch system-clock source to **HSI 4 MHz**, leaving the previous source enabled, §§4.3.2/4.7.3, pp.27/47 | Switch system-clock source to **HSI 8 MHz**, leaving the previous source enabled, §4.7.3, p.71 |

F030 §3.3.1 explicitly warns of possible core damage if the HCLK entry limit is
violated. Its §§3.3.2/4.3.2 prose abbreviates the fast wake source to “HSI”;
the register description specifies 8 MHz. L012's 4 MHz wake source must not be
copied onto F030. With a high-speed system-clock source, stopping that source
also interrupts its HCLK/PCLK consumers. The manuals say bus-clock availability
depends on the selected source; they do not say every possible low-speed
system-clock configuration stops every bus.

Both §3.3.1 descriptions use SCR.SLEEPDEEP with WFI; SLEEPONEXIT is a separate
entry mechanism. Both §3.3.2 descriptions require enabling the selected wake
interrupt before entry and warn that sleeping inside an ISR requires a
higher-priority interrupt to wake it. Clearing all pending events indiscriminately
is not a suitable async entry protocol: a wake arriving during preparation must
remain observable.

The wake-source tables are L012 table 3-3, pp.18–19 and F030 table 3-3,
pp.37–38. RTC, GPIO, IWDT, LVD, RCC, VC, UART and FAULT have DeepSleep wake
entries; L012 additionally has LPTIM, and F030 has AWT. DMA, GTIM, ATIM, BTIM,
SPI, I2C and ADC are not DeepSleep wake sources in these tables. A “yes” entry
still requires the peripheral's source and wake configuration. In particular,
the current UART HAL only selects PCLK, so the hardware table does not establish
a usable low-speed UART wake configuration for an existing UART driver.

The documented normal clock-switch sequence configures and enables the target,
uses Flash waits adequate for both old and new clocks, waits for target stability,
switches the source, adjusts waits for the new rate, then disables only unused
sources. HSI-divider changes preserve trim and do not require a new oscillator
stability delay (§4.5.2 on each chip). These register procedures do not by
themselves update peripheral timing or an Embassy monotonic clock.

## Why the current owners cannot be reclocked transparently

[`ClockResource`](../embassy-cw32/src/rcc/mod.rs) counts physical gate owners and
retains forgotten or quarantined resources. It does not classify stop modes,
record an operation's idle state, own every driver's configuration, or provide
a suspend/reconfigure/resume transaction. A critical section excludes maskable
CPU interrupt handlers; it neither stops peripheral activity nor drains a DMA bus
master.

Concrete affected state includes:

- [`Spi`](../embassy-cw32/src/spi/mod.rs) stores its input clock and divider;
  [`I2c`](../embassy-cw32/src/i2c/mod.rs) stores computed timing; UART programs
  a divisor at construction and its independently owned TX/RX halves cannot
  independently change shared timing. Updating only `rcc::clocks()` leaves
  these settings stale or changes a transfer already on the wire.
- ADC stores its divided clock and validates conversion/sample constraints
  against it. A faster clock can invalidate internal-channel sampling or
  throughput checks even if no conversion was in flight at the instant of a
  switch.
- [`Timer`](../embassy-cw32/src/timer/low_level.rs) stores input frequency and
  timing. [`SimplePwm::split`](../embassy-cw32/src/timer/simple_pwm.rs) transfers
  timer ownership into static shared state and gives each channel its saved
  `Frequency`; no central owner remains to retime it. Forgetting one channel
  intentionally preserves ownership. Complementary PWM's dead-time duration
  also depends on its clock. Pausing a motor waveform needs an explicit output
  policy and restoration protocol, not merely preserved register contents.
- Live or quarantined DMA cannot be treated as idle by writing EN=0. The
  documented normal completion boundary and unproven abort/drain boundary are
  covered in [DMA evidence](dma-hardware-evidence.md). Gate pins and static
  buffers preserve ownership; they are not permission to suspend or retime
  potentially outstanding endpoint activity.
- GPIO, RTC and unsafe motor takeover can pin clocks permanently for different
  reasons. Treating every pin as an active transfer would prevent useful low
  power operation; ignoring every pin would disregard retained DMA and motor
  hardware. Stop eligibility needs separate audited semantics.

A restricted transition with no affected live owners is possible in principle.
It would still need an exclusive transition authority, a complete audit of
affected resources and pinned states, coherent frequency publication and an
explicit rule for the time driver. No such runtime ownership contract is added
in this release. Re-running initialization is not that contract: global
initialization is one-shot and its clock setup expects the direct-reset state.

## Timekeeping and wakeup are separate requirements

With `time-driver-gtim1`, the HAL reserves GTIM1 and its IRQ for the entire boot,
pins its gate, and configures a nominal 1 MHz counter. L012 uses a linear
prescaler; F030 requires a power-of-two divider. Both extend a 16-bit hardware
counter in software. Every overflow must be acknowledged **strictly before the
next 65.536 ms wrap**, including critical sections, higher-priority handlers,
Flash stalls and waker execution. One sticky overflow bit cannot reconstruct
two missed wraps. See the [driver](../embassy-cw32/src/time_driver/driver.rs)
and variant-specific counter implementations.

Ordinary Sleep leaves this counter running. For the supported high-speed clock
trees, DeepSleep stops its input, and GTIM1 is not a DeepSleep wake source. The
current driver has neither a low-speed wake alarm nor a pause/snapshot/elapsed
time/resume interface. Restarting GTIM1 at zero, adding a guessed sleep duration,
or changing its prescaler without reconciling its counter epoch and queued
deadlines would violate the time contract. A 65.536 ms maximum interrupt delay
does not provide an elapsed-time measurement while the clock is stopped.

[`Rtc`](rtc.md) retains its low-speed source and calendar, but exposes whole
seconds only and requires alarms, automatic wakeup and RTC interrupts to be
disabled. A calendar write may change wall time and restart its second phase.
It is therefore not an existing monotonic sleep bridge. The silicon has real
additional capabilities: L012 §13.3.12, p.172 describes a 16-bit auto-reload
wakeup counter, low-speed clock selections and repeated equal AWTCNT reads;
F030 §12.3.11, p.183 describes its own wake counter and source encodings.
Implementing those facilities requires chip-specific elapsed-time and rollover
evidence, source ownership, an IRQ binding and wake-race handling. The presence
of an RTC token alone supplies none of those protocols.

## What the fixed Embassy comparison establishes

The following actual sources were inspected at commit
`b12a6d9efcd2711037abca1b63a661a9ef726444`:

- [`embassy-stm32/src/rcc/mod.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rcc/mod.rs)
  has `reinit(config, &mut Peri<RCC>)`, which reruns RCC and time-driver
  initialization. It explicitly does not reconfigure all other drivers. It
  separately stores an RCC configuration for stop restoration and maintains
  stop eligibility counters with `WakeGuard`/`MaybeWakeGuard`.
- [`low_power.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/low_power.rs)
  checks stop eligibility, pauses time, configures the chip's power mode,
  performs WFI, restores chip-specific state and resumes time. These are
  coupled operations, with ordinary WFI as its fallback.
- [`time_driver/mod.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/time_driver/mod.rs),
  [`tim.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/time_driver/tim.rs)
  and [`lptim.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/time_driver/lptim.rs)
  implement that contract using either RTC wake/elapsed-time support around a
  stopped general-purpose timer or a low-power timer that remains operative.
  STM32 register behavior is not evidence for CW32.
- [`embassy-executor/src/platform/cortex_m.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-executor/src/platform/cortex_m.rs)
  uses **WFE/SEV** for ordinary thread-executor idle, not WFI. Its interrupt
  executor pends an NVIC vector. The repository's pinned
  `embassy-executor =0.10.0` has the same relevant idle behavior. Neither automatically provides
  CW32 DeepSleep preparation or restoration.

## Supported application behavior

Keep SCR.SLEEPDEEP and SLEEPONEXIT clear for ordinary thread-executor idle.
Bare-metal examples already use WFI; the async application uses the ordinary
thread executor for UI and a P1 interrupt executor for the motor. Idle does not
authorize clearing pending events, masking interrupts indefinitely, disabling
PCLK or changing timing registers behind a live driver.

The default L012 4 MHz and F030 8 MHz profiles and the L012 motor's 96 MHz
profile remain unchanged. The motor's P1 serialization and thread UI remain
unchanged. With GTIM1 enabled, its periodic overflow interrupts continue even
without an imminent task deadline; ordinary sleep is not a long stop interval.

A managed DeepSleep implementation would need all of the owner eligibility,
per-chip entry checks, wake-source programming, clock restoration before driver
execution, elapsed-time reconciliation and deadline rearming described above.
This audit does not claim measured entry current, wake latency, oscillator
restart reliability, motor safety during suspension, or board validation.

Primary manuals:

- [CW32L012 V1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
  SHA-256 `a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`.
- [CW32x030 Rev 2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
  SHA-256 `1afd49261f0f0689af8cb8ebf1b0ac1c00e3209b20d3c722106707ff4a10bdd2`.

The cached official manuals were hash-verified, and fixed-revision upstream
sources were fetched and read for this audit. No hardware run was performed.
