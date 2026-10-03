# RTC calendar and low-speed clocks

The v0.20 RTC driver supports CW32L012C8 and CW32F030C8 with an owned `Rtc`,
validated Gregorian `DateTime`, whole-second reads and explicit calendar writes.
It includes actual LSI/LSE startup and source readiness checks. It does not
implement alarms, RTC interrupts, automatic wakeup, tamper, clock-output pins,
subseconds, calibration, backup registers or an Embassy RTC time driver.

## Start a source, then explicitly initialize or attach

```rust,ignore
use embassy_cw32::{rcc::RtcClock, rtc::{Config, DateTime, Rtc}};

let p = embassy_cw32::init(Default::default());
let clock = RtcClock::start_lsi(100_000)?;
let mut rtc = Rtc::new(p.RTC, clock, Config::default())?;
if !rtc.is_running() {
    // This is an explicit application decision to replace a stopped calendar.
    rtc.initialize(DateTime::new(2026, 10, 3, 14, 0, 0)?)?;
}
let current = rtc.now()?;
```

`Rtc::new` enables the APB gate **without a peripheral reset**. Construction
never chooses a reset policy from ambiguous reset flags, changes the source
selector, writes a date or starts/stops the counter. A running retained RTC is
accepted only if it already has the selected source, the expected divider,
24-hour mode, and disabled compensation, alarms, tamper, wakeup, output and
interrupts. Incompatible retained state returns an error without rewriting it.
A stopped RTC can be explicitly initialized, even if its calendar was retained.
`initialize` refuses an already-running RTC and never clears system reset flags.

`release()` returns the RTC and source tokens for a later `Rtc::new` attachment.
Dropping or releasing an RTC leaves the APB clock, oscillator and calendar
running. The APB gate stays permanently pinned, including after a constructor
failure that enabled it. The driver does not expose a destructive reset or stop
operation.

For a board with the specified 32.768 kHz crystal, replace source startup with:

```rust,ignore
use embassy_cw32::rcc::{LseConfig, RtcClock};
let clock = RtcClock::start_lse(p.PC14, p.PC15, LseConfig::default())?;
clock.wait_ready(5_000_000)?;
```

The OSC32_IN/OUT pads are PC14/PC15 on both supported chips. The arguments must
be `Peri<'static, ...>` with metadata-generated input/output traits; wrong pins
and short-lived reborrows are rejected by the type system. Both tokens are
**consumed permanently for this boot, even when startup returns an error**.
No GPIO authority is returned while the oscillator may still run. A failed LSE
readiness wait leaves the oscillator enabled and its pins reserved. It borrows
the clock token, so `wait_ready` can be retried on that same token. Recovering the
GPIO pins is outside this API. Permanent reservation also applies to invalid
configuration arguments, so validate board settings before moving the tokens.

For a stopped oscillator, startup configures both pins as analog, applies the
board's LSE settings and sets the keyed LSE enable bit. It returns the source
token immediately; `wait_ready` polls stability without consuming the token.
For an already-enabled oscillator, it verifies crystal mode, the requested
settings and both analog pads; it preserves all of them. LSE defaults match
register reset settings, but drive strength and F030 amplitude must be selected
for the actual crystal, load capacitors and board. Only 32.768 kHz crystal mode
is supported; a ready bit does not measure the board crystal's frequency.
Neither hardware LSE pin locking nor oscillator locking is changed.

LSI startup sets its keyed enable bit and waits for stability. It never changes
LSI trim or wait-cycle configuration, including when a watchdog, filter or other
autonomous peripheral already uses LSI. A timeout leaves LSI enabled, and LSI
startup can be retried. Both source startup and RTC synchronization bounds count
poll iterations; they are not timeouts calibrated in milliseconds. Zero is
rejected. Crystal startup often needs a much larger bound than the internal RC.

## Real chip differences and accuracy

| Property | CW32L012 | CW32F030 |
| --- | --- | --- |
| Supported source selector | LSE = 0, LSI = 2 | LSE = 0, LSI = 2 |
| Calendar prescaler | PSC1 = 0; PSC2 = 16383 for LSE, 16399 for nominal LSI | Fixed /32768 |
| Calendar tick | 2 Hz internally, 1 calendar second per two ticks | Fixed hardware division |
| Running access synchronization | Wait until CR1.WAIT = 0 | Wait WINDOW = 1; set ACCESS; complete within 1 second; clear ACCESS |
| Write protection | Unlock CA/53; lock CA/00 | Unlock CA/53; lock CA/00 |

L012's first-stage clock is 32768 or nominal 32800 Hz, below the manual's 1 MHz
maximum. The second stage produces nominal **2 Hz**, not 1 Hz. Source frequency
and counter frequency must not be conflated.

Both manuals specify LSI as nominal 32.8 kHz, with RC error affected by supply,
temperature and trim. On F030 the fixed division also causes a nominal
`32800 / 32768 - 1 = 976.5625 ppm` rate error, approximately **84.375 seconds per
day fast**, before RC frequency error. L012's programmable divider removes that
nominal mismatch, but not the RC's actual error. LSI is useful for approximate
calendar operation; there is no precision-time guarantee. The driver does not
claim that a previously configured trim is correct or measure oscillator drift.
Use the specified crystal and characterize the board when accuracy matters.

## Calendar consistency and write protection

`DateTime::new` accepts 2000–2099, real Gregorian month/day combinations and
24-hour time, then calculates the weekday. `DateTime::from` additionally checks
an explicitly supplied weekday. The hardware stores only two year digits;
there is no timezone, leap-second or century support. Applications must handle
the end of 2099 explicitly. Malformed BCD, invalid dates, invalid weekday codes
and weekday/date disagreements in retained registers are reported, not silently
normalized.

`now` follows both manuals' permitted repeated-read alternative. It reads two
complete DATE/TIME pairs inside a short critical section and accepts them only
if both registers agree across the pair. Rollover retries are bounded. Reads do
not unlock registers or stop timekeeping. A stopped calendar reports
`NotRunning`.

`set_datetime` is a deliberate calendar correction with **no subsecond-phase
continuity guarantee**. After the variant's synchronization handshake, it stops
the counter, writes and verifies the whole date/time while stopped, then
restarts it. This avoids a midnight increment splitting the two writes. A
synchronization timeout leaves the old calendar running unchanged. A readback
failure after stopping leaves it stopped; the application can explicitly
initialize it again. Unlock/lock and F030 ACCESS cleanup use guards on all
ordinary success/error exits.

F030's ACCESS window encloses a fixed register sequence in a critical section,
with no blocking polling or user callback inside. At the HAL's supported clocks
this is below the manual's one-second limit. Debug halts, nonmaskable handlers,
and external raw-register interference are not covered by that bound. The
manual's 10 ms retry recipe is implemented as bounded polling of the same
WINDOW condition; no time driver or delay provider is required.

## Retention boundary

Calendar preservation means **the driver does not erase or shut down retained
state**. It is not a claim of battery-backed timekeeping, survival through power
loss/brownout, or uninterrupted counting through every reset. POR/BOR reset the
chip; RTC-specific non-POR language does not override the RCC reset rules.
L012's RTC chapter describes retention through other reset sources; F030 says
to preserve a running RTC and describes cases retaining date/time but requiring
control reconfiguration. Its prose and flowchart are not treated as one universal
reset recipe. The application chooses whether a stopped calendar should be
reinitialized.

The main HAL still accepts only its checked direct-reset HSI startup boundary.
Low-speed source startup does not add arbitrary bootloader clock support,
runtime system-clock switching, DeepSleep/STOP recovery, or a clock-loss
monitor. `now` and writes recheck source enable/stability. Reacquiring a source
after reset cannot reconstruct elapsed time while that source was unavailable.

## Evidence and verification

The structure was compared against the actual pinned Embassy
[`rtc/mod.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rtc/mod.rs)
and [`rtc/v2.rs`](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/rtc/v2.rs):
owned RTC, date/time value, fallible calendar reads and separate hardware
backends. STM32 backup registers, subsecond/shadow behavior, INIT protocol,
unbounded loops and low-power framework are not copied onto CW32.

Hardware evidence:

- [CW32L012 manual V1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
  §§4.3.5–6, 4.7.2, 4.7.7 and 13.3.2–6, 13.5.1–7: oscillator startup, mode,
  stability, keys, 2 Hz divider, calendar/weekday encoding and WAIT access.
- [CW32x030 manual Rev 2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
  §§4.3.5–6, 4.7.2, 4.7.7 and 12.3.2–5, 12.3.7, 12.5.1–3: oscillator
  startup, LSI frequency, fixed division, retention and WINDOW/ACCESS contract.
- L012 SDK V1.0.5 `Libraries/inc/cw32l012_sysctrl.h` defines LSE input/output
  PC14/PC15; `SYSCTRL_LSE_Enable` selects analog pins. F030 SDK V2.2
  `RCC_LSE_Enable` independently uses analog PC14/PC15. The SDK RTC lock macro
  and unrestricted calendar writes are not substituted for the newer manuals.

External source-based validation checks all 36,525 dates from 2000 through 2099,
3,506,400 calendar/BCD round trips, consecutive weekdays and invalid-register
rejection. Release ARM checks exercise both generated chip backends. Register
models and compilation do not establish crystal startup margins, frequency
accuracy or measured reset/low-power behavior; those need board validation.
