# Qualified direct HSE on F020/F030/A030

This batch adds direct HSE crystal/ceramic and digital bypass initialization for
CW32F020, CW32F030 and CW32A030 only. Default factory-HSI behavior and its public
Hsi/HsiDiv/AHBPrescaler/APBPrescaler APIs remain available. One-time HSI- or
HSE-fed SYSCLK PLL has a separate [qualified contract](f020-x030-hsi-pll.md),
including crystal and bypass HSE references. Independent PLL outputs, runtime
switching, low-power entry/recovery and runtime clock-loss recovery remain
unimplemented. Other families have their own external-source contracts.

## API and architecture

The reference is embassy-stm32 at
`f16efeffe37581092ec184718e6fdb1620393214`, specifically its `rcc/f013.rs`
Hse/HseMode, optional Config.hse, Config.sys and one-time frozen clocks. CW32
uses its own typed PAC/register rules, its existing central RccInfo and its
flat backend files. No STM32 clock limits or Stop-mode behavior are imported.

Intentional additions/divergences:

- Config.hse: Option<Hse> and Config.sys: Sysclk (direct HSI/HSE; PLL follows
  the separate contract above). HSE selected
  without a declaration fails before writes. A declared HSE is started and its
  pads reserved even when SysClk remains HSI, as an explicitly requested source.
- Hse carries required nominal/minimum/maximum Hertz, explicit board operating
  conditions, HseMode and typed PAC HseDrive. No external default accuracy is
  invented. No external bound comes from factory-HSI tolerance.
- A supplied HSE declaration covers source tolerance, temperature, aging, load,
  short-term cycle variation, component supply and board effects over its condition interval. That interval
  must cover the Config board interval. HSI remains alive, so the board interval
  must also remain inside its original factory-HSI qualification.
- `sys_bounds`, `hclk_bounds` and `pclk_bounds` follow the selected source with
  exact integer division and outward-rounded public endpoints. `hsi_bounds`
  and HSIOSC-dependent consumers still use the factory-HSI envelope.
- Safe GPIO construction panics before pad writes when a pad is reserved by the
  configured HSE, or by an inherited enabled HSE. Crystal reserves OSC_IN and
  OSC_OUT; bypass reserves only OSC_IN. The singleton token still exists, as in
  the existing init API; this is a runtime ownership check, not a new linear
  ownership API. Raw PAC/unsafe access can invalidate the contract.
- The five other RCC backends are outside this worker's file ownership.

## Own-source admission

All three own datasheets allow 1..32 MHz high-speed bypass; both applicable RMs
state 4..32 MHz. The unresolved discrepancy is preserved and admission uses the
intersection 4..32 MHz, including the actual endpoints. Crystal admission also
requires actual 4..32 MHz. A nominal endpoint with nonzero outward tolerance
can fail admission. The range-field table is explicitly for nominal crystal
frequency. Shared nominal bin endpoints select the upper bin deterministically;
32 MHz uses the final bin. This does not relax actual electrical admission.

All three own general condition tables give 1.65..5.5 V and −40..105°C as the
conservative ambient range. The conditional 125°C power/junction allowance is
not adopted. Bus ceilings remain their authored own-family values: F020 48 MHz,
F030/A030 64 MHz at VDD≥1.8 V; all three require actual HCLK/PCLK≤24 MHz below
1.8 V. Flash wait decisions use actual upper HCLK, not nominal Hertz. For
example 24 MHz ±30 ppm has an upper bound of 24,000,720 Hz and requires WAIT1
at AHB/1. Mandatory CCS can fall back to HSI, so final buses and Flash latency
must additionally be legal for the retained divided factory-HSI envelope.

Bypass is a board waveform obligation: 40..60% duty from the own RM, at least
15 ns each high and low, at most 20 ns each rise and fall, and own-datasheet
input high/low voltages and pin limits. These must hold together; 32 MHz at
40% duty does not satisfy 15 ns minimum pulse width. Crystal drive, loading,
placement and startup must be board-qualified. The typical 2 ms startup table
entry is not a universal maximum or a timeout guarantee.

The HSE startup setting is the longest documented 262144 source cycles.
DETCNT = ceil(8,000,000,000 / minimum_actual_HSE_Hz) applies the manual heuristic
to the slowest admitted source instead of rounded nominal MHz. It is checked
nonzero, fitting the 11-bit field, and large enough for 131072 source cycles at
the fastest documented legal retained LSI (32800×1.10 = 36080 Hz). At the lower
4 MHz bound the result is 2000, below 2047; the slowest source supplies at least
8 billion HSE-Hz×LSI-cycles versus 4,729,077,760 units required by the detector.
The inequality in source code uses exact integers. The manual's LSI safe range
is distinct from each own datasheet's factory-trim accuracy.

LSI trim and WAITCYCLE are retained. LSI is enabled and kept on whenever the new
HSE or an inherited HSE/LSE requires the mandatory CR1 CLKCCS/HSECCS/LSECCS
policy. The trim bridge restores the original software-enable bit only when
no such external-clock detector needs LSI. No claim is made that a ready flag
establishes the frequency of arbitrarily modified inherited oscillator trim.

## Transition and retained-state contract

Entry must already satisfy its own supply, source, bus and Flash limits. Every
incoming source required during the transition must remain stable. A configured
LSI must satisfy its documented legal safe range when used for the trim bridge
or CCS; this initializer does not retune a potentially shared low-speed source.
DMA, clock-derived bus consumers, ordinary interrupts and clock-changing NMI
handlers must be quiescent. This is one-time initialization, not a runtime
frequency change API or recovery from arbitrary asynchronous source loss.

Board/source/bus/timeout admission occurs before side effects. Retained AWT and
RTC source state is read by temporarily enabling only its APB configuration
gate, then restoring the original gate with bounded readback. No reset or
peripheral control write occurs. This avoids mistaking a disabled configuration
gate for an unused independent working oscillator. Active AWT HSIOSC or active
LVD HSIOSC filtering prevents a required HSI retune. RTC selecting HSE reserves
HSE even when the calendar is stopped, because its independent wakeup path can
still use it. An active AWT HSE source reserves it too. Such retained HSE is
reused only when enabled, ready, and its parameters and actual pad configuration
match. Conflicting requests fail; retained calendar/wakeup state is untouched.

The source-level order is:

1. Enable Flash's existing central RCC gate and raise WAIT before source changes.
2. Install AHB at least /4 and APB /8, preserving stronger inherited dividers.
   This monotonic policy is an intentional conservative delta. Acceleration by
   the previous fixed /4,/8 guard alone did not prove an electrical violation.
3. Enable unchanged HSI and required CCS bits; await HSI ready; switch to HSI;
   acknowledge the mux and issue DSB/ISB. Stop PLL and await disabled/unstable.
4. If trim differs and no retained owner conflicts, enable unchanged LSI, await
   ready, switch HSI→LSI, stop HSI, await unstable, set trim, restart HSI, await
   ready and switch back. Preserve required LSI. Update only the documented
   live HSI divider after trim is stable.
5. If HSE is unreserved, stop it and await disabled/unstable before changing its
   actual pins or parameters. Crystal uses analog on both; bypass uses digital
   input on OSC_IN. Disable pad output, pulls, edge/level interrupts, open-drain
   and input filtering; select GPIO AF before final analog/digital mode. Retained
   matching HSE is left alone. Enable HSE and use a bounded startup poll.
6. Select the requested source while guarded; verify it before relaxing the
   final AHB/APB divisors. Verify source/HSI calibration/dividers, then set final
   Flash WAIT covering both selected source and any mandatory HSI fallback.
   Recheck the mux after Flash acknowledgment before publishing frozen clocks.

Timeout budgets are iterations, not elapsed durations. Range-based loops accept
success on their final poll and cannot underflow. Hardware failures can leave a
bridge source or partial setup; no clocks are published. Reset before retrying.
STABLE is a startup latch: ongoing clock loss does not clear it. Mux readback
reduces stale publication opportunities but is not continuous liveness evidence.
After source loss, frozen rates are no longer valid and this HAL offers no
runtime recovery promise. HSI is kept on and the fallback electrical operating
point is admitted, but that does not preserve external-clock timing guarantees.

## Pin ownership and errors

On success RCC publishes its source reservation before `init` returns the
singleton set. `Flex::new` is the shared entry used by safe GPIO, alternate
function and analog driver constructors; it checks HSE ownership before its
first clock-gate or pad write. Crystal protects both actual pads, bypass only
OSC_IN, and an inherited enabled HSE is protected even if Config.hse is absent.
A matching retained HSE is checked using temporarily accessible GPIO registers;
its pad controls are not rewritten.

Pure board/configuration rejection happens before singleton acquisition or
MMIO, so a corrected configuration may be submitted. Once acquisition occurs,
any retained-owner rejection, stop/start/pin/readback timeout or later timer
initialization error returns no peripheral tokens. The actual pinned Embassy
singleton macro makes `Peripherals::take` crate-private; its `steal` escape is
unsafe. Failed initialization consumes the set and cannot supply safe GPIO
constructors with tokens for the partially configured pads. No guessed clocks
are published by RCC on its failures. A second hardware initialization attempt
without reset is unsupported and cannot recover the singleton through safe
code. Raw PAC or unsafe token theft always lies outside this ownership contract.
An initialized time-driver failure can occur after RCC has frozen clocks; it
still returns no singleton set and requires reset, as documented by init.

## Physical pads and coverage

Own package pin tables independently identify PF0=OSC_IN and PF1=OSC_OUT.
F020 QFN48 uses package pins 5/6, QFN32 2/3, QFN20 19/20. F030 LQFP48 uses 5/6,
LQFP32/TSSOP20/QFN32 2/3, QFN20 19/20. A030 LQFP48 uses 5/6. Exact selected
package metadata owns availability. PC15's digital HSE_OUT is not OSC_OUT.
F020 shares the GPIO register form but uses its real SYSCTRL register block and
its typed HSE.FREQ field; x030 uses HSE.FREQRANGE.

Genuine crystal and bypass UART/LED firmware lives in `examples/shared/hse-clock` for
F020C6U7, F030C8T7 and A030C8T7. It requires real qualified board sources and is
only compile/linked in verification. No firmware execution, HAL test, mock,
register simulator or synthetic compile harness is part of this batch.

The source receipt records exact official URLs, revisions, PDF pages and hashes.
Vendor originals, page renders/full-text derivations and historical predecessor
archives are excluded from the review deliverable. Schema ancestry remains at
its previous reviewed tip until the final combined schema receives independent
review.

## AWT source semantics and remaining PAC gap

The original awt_v1 register layout is shared by F002/F003 and F020/x030, but
its external source meanings differ. F020 RM PDF170 and x030 RM PDF173 define
SRC=2 as HSE. F002/F003 instead define SRC=2 as HEX_PB00 and SRC=3 as HEX_PB01.
The HSE families therefore use awt_cw32f030_v1 with independently cited own
source evidence and the same qualified HSIOSC command behavior. F002/F003
retain their original block byte-for-byte. Their real external SRC=2/3 values
still have incomplete reserved-placeholder names in the PAC enum. That is a
known PAC enum coverage gap, not absence of the documented HEX hardware, and
is withheld from this HSE-only implementation batch.
