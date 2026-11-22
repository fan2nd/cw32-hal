# Init-only factory LSI SYSCLK on CW32F003F4P7/F4U7/E4P7

`rcc::Sysclk::LSI` is qualified for these exact F003 packages only:

| Part / Cargo feature | Package | Flash / SRAM |
|---|---|---|
| CW32F003F4P7 / `cw32f003f4p7` | TSSOP20 | 20 KiB / 3 KiB |
| CW32F003F4U7 / `cw32f003f4u7` | QFN20 | 20 KiB / 3 KiB |
| CW32F003E4P7 / `cw32f003e4p7` | TSSOP24 | 20 KiB / 3 KiB |

Flash begins at 0x00000000 and SRAM at 0x20000000. Generic `cw32f003` and all
unlisted parts remain excluded. HSI is still the default. Set
`config.rcc.sys = rcc::Sysclk::LSI` before one-time `init(config)`, using
the existing operating conditions, HSI/HEX configuration, AHB/APB prescalers
and nonzero timeout. No user trim, WAIT setting, arbitrary frequency, runtime
switching, sleep/wake restoration or clock-loss recovery API is added. F003
has no RTC or dedicated LSI_OUT; this adds no `LsiClock` or calendar API.

## Rate qualification

The own factory full-temperature envelope is nominal 32,800 Hz, bounded by
31,816–33,784 Hz (±3%) at VDD 1.65–5.5 V and ambient −40–105°C. The RM's ±10%
adjustment range, DS 25°C ±1% row and high-temperature exception do not expand
this qualification. Board declarations must cover real supply and ambient
conditions; initialization does not measure them.

SYSCLK/HCLK/PCLK are rate-only: `has_cycle_timing_bounds()` is false. Exact
source numerators and cumulative divisors remain intact; whole-Hz endpoints
round outward. AHB /128 plus APB /8 gives displayed PCLK nominal 32 Hz,
minimum 31 Hz and maximum 33 Hz, with exact divisor 1024 retained. No absolute
per-cycle or jitter bound is established.

ADC timing rejects this rate-only source before peripheral startup. The fixed
1 MHz `time-driver-gtim` cannot divide any supported LSI bus tree and rejects
selection before singleton acquisition or RCC MMIO. The LSI example enables
no time driver. Factory HSI stays enabled on success with independent bounds;
AWT retains its own HSIOSC timing. Existing peripheral rate, representability
and ownership limits still apply, including to UART, ATIM and IR.

## Whole-bank handover

The platform entry model requires quiescent bus masters before Rust uses
application memory, legal continuous executing clocks, and no concurrent RCC
or HAL-owner modification. Critical sections do not freeze peripheral events.
LSI selection permits inspection windows that run every GPIOA/B/C bank,
including unbonded pins. Sampling, filters and armed events can advance, even
before failure. Configuration and locks are preserved; software does not
clear flags, including LSIRDY, but flags can change naturally. Restoring gates
cannot undo progress. Bus guards and the system handover affect HCLK/PCLK
consumers and outputs as well.

Cold admission checks AWT, UART1/2, GPIOA/B/C, ungated MCO and RCC ready/NVIC
observers. Legal non-LSI selector sets are AWT [0,2,3,4], UART [0,1], GPIO
[0,1,2,3,4,6] and MCO [0,1,2,3,5,8,9]. Disabled functions, masked interrupts,
closed gates or unused pads do not waive these checks. Each gated inspection
verifies reset and readability, then restores the original gate with bounded
readback. Failed enable still attempts restore; restore failure takes priority
over a consumer rejection. No reset or selector/flag write creates admission.

F003's ATIM and IR have no independent LSI selector. Their documented UART
dependencies are covered by the UART roots; timer/system-clock dependencies
remain part of the functional handover. No ATIM/IR inspection gate opens, and
IRMOD, ATIMETR and TIMITR are not rewritten. Single-bit gate operations preserve
unrelated bits, including ATIM bit7 in APBEN2/APBRST2 and reserved neighbors.
This grants no additional ATIM/IR mode qualification or timing continuity.
IWDT uses independent RC10K; existing AWT/LVD HSI and AWT/HEX guards remain.

## Admission and transition

A requested or selected, factory-matching LSI with both stable observations
set is reused without TRIM/WAIT writes. An unselected requested matching
source can finish startup within the poll budget; a selected source must
already be stable. Contradictory status, unowned stable state and live/in-flight
nonfactory trim are rejected. The entry identity is retained throughout.

A cold source requires LSIEN and both stable observations clear, legal HSI or
HEX execution, and clear IER.LSIRDY, ISR.LSIRDY and NVIC RCC IRQ4 pending.
Two matching consumer/observer passes precede any TRIM write; even matching
cold TRIM must pass them. The aligned factory halfword at 0x001007BA supplies
the low ten TRIM bits. Raw 0xFFFF is conservatively rejected as an all-ones
factory read, without claiming silicon damage; zero is valid and high bits
are not separately rejected. All four inherited WAIT encodings and other LSI
bits are preserved. Readback and a third use-edge pass retain the original
identity advanced only by confirmed changes.

Conservative Flash/bus guards precede cold preparation. The first CR1 source
write requests HSI and permanent LSI together. Later HSI calibration and
independent HEX handling keep that LSI request and the existing owner checks.
HSI calibration may temporarily execute on LSI. `hex: None` preserves an
inherited HEX source. For independent HEX reconfiguration, stop/readback comes
before consumer/LSI checks and pad setup; successful pad setup advances only
the expected GPIOB gate, without resampling the other retained state.

LSI is selected and verified before final bus divisors relax. Final Flash WAIT
covers both target LSI and retained factory HSI at the final dividers; low LSI
rate alone does not imply WAIT0 or fallback capability. Final source, owner,
divider and Flash checks precede publishing frozen RCC clocks.

## Failure and verification limits

Public configuration and time-driver preflight reject before singleton
acquisition or MMIO. Later entry, factory-memory, owner and hardware checks
can consume singletons. Failures may leave TRIM, enabled gates, conservative
Flash/bus guards, GPIO progress, partial HEX setup or a permanent LSI request.
HSI-calibration failure may leave execution on LSI with HSI stopped or not yet
restarted. No cleanup promises rollback or ready HSI; reset before retrying.
RCC errors publish no new clocks and return no peripheral tokens. The existing
later time-driver initialization failure boundary remains separate.

Timeouts count polls, not microseconds. STABLE is a startup latch, not running
clock-loss detection; software may not return if its executing clock stops.
Ordinary compilation/linking and static source review establish no hardware
startup, electrical accuracy, failure-path behavior or low-speed Flash timing.
The example must fit the real 20 KiB / 3 KiB limits; inspect its ELF entry,
vectors, load regions and remaining stack space before claiming a fit.

## Sources and compatibility

Own bindings in `cw32-data/lsi-sysclk-qualified.yaml` use RM Rev2.3 PDF
46/48/59 for factory trim, DS Rev1.9 PDF 38 table 7-15 for the factory rate,
and DS PDF 5/8/27/62 for exact packages and memory. E4 is supported by DS,
without a matching DFP device. Hashes and additional consumer/ATIM/IR locators
are indexed in [SOURCES.md](../sources/SOURCES.md#f003-精确三封装-factory-lsi-sysclk).

The existing optional output metadata already expresses the independently
checked absence; no schema change is added. F003 emits seven selectors and
six gates. [F002](f002-factory-lsi-sysclk.md) keeps its own ±5% rate and 16 KiB /
2 KiB memory; [classic](factory-lsi-sysclk.md) keeps its RTC/output qualification
and eleven-selector/nine-gate roster. Shared runtime code grants no capability
to a generic alias or another part.
