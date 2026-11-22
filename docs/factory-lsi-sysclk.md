# Init-only factory LSI SYSCLK on F020/F030/A030

This page describes classic F020/F030/A030 cold-start and live-reuse admission.
The [exact L010 live-only contract](l010-live-factory-lsi-sysclk.md) separately
covers CW32L010F8P6/F8U6/Y8M6: original entry must already be software-requested,
stable and factory-matching. This page's stopped startup and trim preparation
are not imported. Other exact contracts remain in the
[qualification policy](../cw32-data/lsi-sysclk-qualified.yaml).

CW32F002F3P7/F3U7 use a separate [exact-two contract](f002-factory-lsi-sysclk.md),
with their own consumer roster and no RTC or dedicated LSI_OUT pad. The optional
`lsi_output_pin` metadata representation preserves the real classic output pin
and all classic consumer checks described here.

`rcc::Sysclk::LSI` selects the factory-qualified nominal 32,800 Hz oscillator at
one-time HAL initialization. HSI remains the default, and successful initialization
keeps factory-qualified HSI available. This capability is generated only from the
three families' own qualified metadata. The separate [exact-three-package LSE
SYSCLK contract](classic-lse-sysclk.md) reuses its factory preparation internally.
Neither capability adds runtime switching, arbitrary user trim/frequency,
sleep/resume support or clock-loss recovery. No silicon validation is claimed.

Set `config.rcc.sys = rcc::Sysclk::LSI` before `init(config)`. The existing
board supply/ambient declaration and AHB/APB prescalers apply. A configured HSE
or LSE can still be initialized under its existing independent contract, after
LSI is ready. A PLL configuration must still be selected as SYSCLK, so it cannot
be requested together with LSI SYSCLK.

## Rate qualification and compatibility

F030 and A030 have 31,816–33,784 Hz factory full-temperature rate bounds (±3%);
F020 has 31,160–34,440 Hz (±5%). These rows require 1.65–5.5 V and ambient
−40–105°C. Retained HSI must independently satisfy its own conditions. The
source's exact numerator and integer AHB/APB divisors are preserved; public
whole-hertz endpoints round outward.

These are rate bounds, not independently established absolute per-cycle or
jitter bounds. On these same three families, `LsiClock::bounds()`,
`CalendarClock::Lsi`, RTC source bounds and divided calendar-tick bounds now
consistently return `has_cycle_timing_bounds() == false`, even with HSI SYSCLK.
This is a public compatibility change: calling strict `minimum_duration_ns` or
`maximum_duration_ns` on those bounds now fails their existing qualification
assertion. Nominal/minimum/maximum rates, operating conditions and the exact
32800/32768 calendar ratio are retained. Other families' RTC qualification and native HSIOSC RTC are unchanged. The existing post-init `LsiClock::new`
remains compare-and-enable; it does not gain permission to trim a stopped or
running source.

Classic ADC and classic complementary PWM reject this rate-only source before
their strict timing use and peripheral startup, including zero requested dead
time. The fixed 1 MHz Embassy time driver rejects LSI with every supported bus
divisor during pure configuration validation, before peripheral singleton
acquisition or MMIO. Ordinary UART/SPI/I2C/timer rate limits still apply; common
high baud/default rates may be unrepresentable. Their existing constructor side
effects and error contracts are not changed by this feature. AWT retains its
independent HSIOSC source; IWDT uses separate RC10K.

## Entry and whole-bank behavior

The established platform entry model applies: hardware reset, or a low-level
handover that left bus masters quiescent before Rust uses application memory.
No debugger, DMA or exception handler may race HAL-owned register writes. A
critical section does not stop peripheral state machines or input edges.

Selecting LSI explicitly permits bounded opening of the entire GPIOA/B/C/F
working clocks to inspect retained FILTER selectors and PB11's LSI_OUT selector.
Sampling, filters and armed events may advance during each interval, even if
initialization later fails. Gate restoration cannot undo this progress. Every
GPIO setting and lock is preserved. Software does not clear flags, but flags
can change naturally. No pin-only isolation or absence of
functional effects is promised, and there is no extra unsafe constructor or
caller risk checkbox.

RTC, AWT and all three UARTs use their separately qualified configuration-only
gates. Each inspection saves the original gate, rejects a reset-held block,
opens only that gate if needed, and restores it with bounded readback. The
central RCC gate mechanism uses neighbor-preserving unkeyed RMW for these
classic gates. Failed enable still attempts restoration. No peripheral reset,
IRQ configuration or flag-clearing write is used to manufacture admission.

## Admission and ordering

An already-requested or currently selected factory-matching LSI is reused
without TRIM or WAIT writes. An enabled matching source still starting may
finish within the poll budget. Contradictory stable signals reject. A running
or in-flight nonmatching source is never stopped or retrimmed.

A genuinely stopped source, including one whose TRIM coincidentally matches,
requires two complete source/observer passes and a final use-edge pass:

- Current SYSCLK is legal HSI or HSI-fed PLL; LSIEN and both LSI stability signals
  are clear. HSEEN/LSEEN, HSE/LSE register and ISR stable signals, HSECCS/LSECCS
  are clear. An inherited enabled/stable external source cannot use this cold
  route because the mandatory CCS write could otherwise start its detector
  against an unready LSI reference.
- IER.LSIRDY, ISR.LSIRDY and the NVIC RCC pending bit are clear. None is cleared
  by initialization. A successful new start can set LSIRDY, and leaves it set.
- RTC SOURCE excludes LSI and reserved values regardless START; independent AWT
  SRC excludes LSI/reserved values regardless EN; UART1/2/3 SOURCE excludes LSI.
- MCO excludes LSI/reserved values regardless output pad use. Every GPIO bank's
  FLTCLK excludes LSI/reserved values regardless per-pin enables, and PB11 AF
  excludes LSI_OUT/reserved values regardless direction, lock or package bond.

Relevant selectors, gates, resets and global controls are checked again; a
changed snapshot or failed restoration rejects. These repeated checks rely on
the existing register-ownership entry model; they do not stop arbitrary writers.

Only after admission is the ten-bit factory TRIM loaded if needed. The aligned
public factory halfword at 0x00012602 is read before admission to classify
matching reuse. Raw 0xFFFF is rejected before masking; zero is accepted.
A typed TRIM-only RMW preserves WAIT and all other bits. Readback and the final
admission use the advanced expected factory TRIM, with original WAIT/reserved
parameters still intact. This finishes before the first CR1 write.

The first CR1 write requests HSI and permanently requests target LSI, with key
0x5A5A and all mandatory CCS controls set. LSI enable and both stable indications
are checked before a requested external source can start. Subsequent source
writes retain target LSIEN. The separately admitted existing HSI calibration
may briefly stop HSI while running on this qualified LSI; it retains the target
request. The retained AWT/LVD checks are not weakened.

An inherited PLL is left through confirmed unchanged HSI, never directly to
LSI. The LSI path does not stop a retained PLL. It rejects inconsistent PLL
request/readiness or an HSI-fed PLL whose required HSI trim/divider would change;
an HSE-fed retained PLL keeps its reference unchanged. Existing independent
source consumers still constrain explicit HSE/LSE configuration.

Conservative Flash/bus guards precede source changes. Final Flash waits also
cover retained HSI under final dividers rather than assuming WAIT0 merely from
the slow target. Selected source, dividers, factory LSI parameters, permanent
LSIEN, both LSI startup indicators, retained HSI and mandatory CCS are verified
before frozen clocks are published.

## Failure state

Pure configuration/time-driver failures take no singleton and touch no hardware.
Pre-trim owner admission failures write no LSI TRIM, source request or source selector;
Flash/bus guards may already be established and GPIO inspection may already
have advanced events. Use-edge admission or gate failures after calibration can
leave the attempted TRIM, without a source-enable request. A restore error can
leave a gate enabled. A trim-readback
failure can leave the attempted TRIM, without enabling LSI.

After the enable attempt, errors retain the permanent LSI request and publish no
clocks or new peripheral tokens. No cleanup stops or retrims a possibly started
source; it may become ready after timeout. HSI normally remains requested, but
failure during its separately admitted calibration may leave it stopped or
incompletely restarted while the CPU is on LSI. No automatic rollback is
promised. Reset is required before retrying initialization.

Poll budgets count register attempts, not microseconds; the same budget takes
longer on LSI. If the executing source physically stops, software cannot
necessarily make progress. STABLE is a startup indication that may remain set
after source loss, not a continuous health guarantee.

The authoritative facts and own-source page/hash bindings are in
`cw32-data/lsi-sysclk-qualified.yaml`; the accepted design and independent source
review are retained in `factory-lsi-cold-start-design.md` and
`factory-lsi-cold-start-review.md`. Static source evidence and compiler validation
are distinct from hardware startup, electrical, loss and peripheral testing.
