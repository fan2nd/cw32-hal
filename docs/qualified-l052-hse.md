# Direct HSE on CW32L052

The native shared `rcc/l052_l083.rs` backend supports one-time direct crystal or
bypass HSE initialization on CW32L052C8T6, CW32L052R8T6 and CW32L052R8S6.
`Hse` requires nominal, minimum and maximum frequency, board conditions, mode
and drive; `Config.sys` selects HSI or HSE. HSI remains the default and stays
running. Declaring HSE initializes and reserves it even when SYSCLK remains HSI.
The existing HseLimits schema, native typed PAC and central RCC gate inspection
are reused. No register adapter, raw shadow definition or PLL is introduced.

The exact source identities and claims are in
[the source receipt](qualified-l052-hse-source-receipt.json). All page citations
below are one-based PDF pages, with the printed page one lower. The authoritative
sources are L052 RM CN Rev 1.5, DS CN Rev 1.3 and SDK V1.4; SDK examples do not
override the own manual's configure-before-enable rule.

## Board and electrical contract

Every actual source endpoint must lie within 4–32 MHz, with
`0 < minimum <= nominal <= maximum`. The RM §4.3.3 PDF 52–53 specifies 4–32 MHz
in both modes; DS Table 7-13 PDF 48 permits bypass from 1 MHz. The initial policy
uses their intersection, leaving the discrepancy unresolved. Crystal agrees at
4–32 MHz (DS Table 7-15 PDF 50). Include supply, load, temperature, aging, tolerance
and short-term cycle variation in the declaration. External accuracy is never
inferred from HSI accuracy or measured by initialization.

Declare VDD 1.65–5.5 V, VDDA=VDD and ambient−40..85 °C. DS Table 7-4 PDF 43 limits
actual HCLK and PCLK to 24 MHz below 1.8 V, or 48 MHz at/above 1.8 V. The conditional
105 °C low-dissipation extension does not widen the retained factory-HSI accuracy
row (Table 7-17 PDF 51). Factory HSIOSC is 48 MHz ±2%, i.e.47.04–48.96 MHz before
its divider. AUTOTRIM bounds retain this raw envelope independently of SYSCLK.

Both selected-source and requested-HSI envelopes must fit final bus limits.
Additionally, L052 CLKCCS forces HSI 8 MHz, i.e./6, regardless of the requested
HSI divider (RM §4.4.3.3 PDF 62 and §4.7.2 PDF 71). Its actual 7.84–8.16 MHz
fallback is separately admitted even when CCS is currently disabled. L083 has
a distinct configured-HSI fallback; its policy is unchanged. Flash latency is
chosen from the maximum actual HCLK across selected source, requested HSI and
L052's fixed-/6 fallback. WAIT0/1/2 covers 24/48/72 MHz (RM §7.4 PDF 112).
WAIT2 does not authorize a 72 MHz bus. A 24 MHz nominal HSE with positive tolerance
needs WAIT1 at AHB/1. HSI/1 can reach 48.96 MHz, exceeding L052's 48 MHz bus limit;
HSI/2 can reach 24.48 MHz and cannot be used at AHB/1 below 1.8 V.

Bypass waveform requirements apply together: duty 40–60%, high and low pulses
each≥15 ns, rise/fall each≤20 ns, high 0.7–1.0×VDDIO and low 0–0.3×VDDIO,
plus the own TC I/O ratings (DS Tables 7-13/7-24 PDF 48/54). At 32 MHz,40% duty
alone violates the 15 ns pulse requirement. The source must be stable before
initialization and remain continuous. Crystal drive, load, ESR/resistance,
layout and startup are board obligations. DS 2 ms startup is typical, not a
universal maximum or software-timeout promise.

## Native pre-start fields and pins

L052 SYSCTRL_HSE includes PDRIVER[21:20] and PFREQRANGE[23:22], in addition to
run DRIVER[1:0] and FREQRANGE[3:2] (RM §4.7.6 PDF 75–76). Both phases use the
same board-selected drive 0..3 and nominal frequency bin. Bins 4–8/8–16/16–24/
24–32 MHz use the upper bin at 8/16/24 MHz;32 MHz uses the last bin. This is a
deterministic admission convention at overlapping manual boundaries.
WAITCYCLE=3 selects 262144 cycles; FLT remains off. All fields are programmed
and read back while HSE is disabled and STABLE clear. The retained-owner reuse
check includes both pre-start fields. Reserved bits 31:24 are preserved.

Own DS Table 5-2 PDF 26 bonds PF0=OSC_IN to physical pin 5 and PF1=OSC_OUT to
pin 6 on all three modeled packages: C8T6 LQFP48, R8T6 LQFP64 10 mm and R8S6
LQFP64 7 mm. PC15 HSE_OUT is an output AF and is not OSC_IN. The package's
existing oscillator aliases project these routes; no digital AF is invented.
GPIO uses the actual keyed LOCK/LCKR register, key 0x 5a 5a (RM §9.6.14 PDF 156).
Only owned pads are unlocked/configured: input direction first; clear pad-local
pulls, interrupts, open drain/filter; select AF0; crystal uses analog and bypass
digital input. Unrelated pins, shared filter clock and GPIO ICR are preserved.

Crystal reserves PF0/PF1 and bypass reserves PF0. The union of incoming enabled
HSE and requested source pads stays reserved for the boot. This intentionally
keeps inherited PF1 reserved after an unowned crystal is successfully stopped
and reconfigured as bypass, although PF1 is no longer physically used. Safe
GPIO/analog/peripheral construction checks reservation before touching pads.
The same conservative whole-boot policy correction applies to L083. Raw PAC
writes can invalidate the hardware clock contract.

## Retained consumers and initialization

Require stable, electrically legal incoming clocks, legal incoming Flash
latency, legal HSI divider, non-erased factory calibration at 0x 00100a 00, and
quiescent DMA/application interrupts/bus-dependent users. A critical section
does not mask NMI; a firmware jump does not establish reset state. SYSCLK codes
0/1/3/4 are HSI/HSE/LSI/LSE; code 2 and 5..7 are rejected. L052 has no PLL.

AUTOTRIM and RTC are inspected through their real central RCC configuration
gates with readback and gate restoration, without reset. Active AUTOTRIM must
be MD=TIMER,AUTO=0 with a documented source (RM §11.8.1 PDF 175). Active
calibration, automatic mode and reserved sources are excluded. Required HSI
retrim is rejected if active AUTOTRIM consumes raw HSIOSC or enabled LVD uses
HSIOSC filtering (RM §25.7 PDF 533). Divider-only changes preserve raw HSIOSC.

RTC SOURCE 4..7 owns HSE regardless of calendar START or wake controls; source
1/3 is rejected (RM §12.5.3 PDF 193). Active AUTOTRIM HSE also owns it. Such
owners admit only already enabled/stable exact source-parameter and pad reuse;
no HSE or pad write occurs. Active AUTOTRIM ETR rejects an HSE request because
PF0 is a documented ETR route. Unrelated sources/consumer configuration remain.

CR1 CLKCCS/HSECCS/LSECCS and LSELOCK are preserved and verified. Enabled HSE/LSE
detectors require unchanged legal LSI enabled and stable; CLKCCS alone is not
a detector. LSI must already satisfy the RM 32.8 kHz±10% envelope, whose maximum
is 36080Hz (PDF 59). No factory accuracy is assigned to arbitrary inherited LSI
trim. DETCNT uses ceil(8,000,000,000/minimum_HSE_Hz), requires 1..2047 and proves
minimum_HSE_Hz×count >131072×36080 (RM PDF 71/75). This is an admission bound,
not oscillator measurement or guaranteed recovery.

After pure validation and retained-owner inspection, enable Flash configuration,
raise WAIT2 and install at least AHB/4,APB/8 without weakening stronger incoming
dividers. Start unchanged HSI and switch/read back HSI before touching HSE.
If factory retrim is needed, use unchanged legal LSI as a bridge, stop HSI and
observe enable/STABLE clear, write/read trim, restart and return to HSI. Restore
LSI's previous software enable only when no detector requires retention.
L083-only PLL escape/stop code is compiled solely for L083.

An unowned requested HSE is stopped and observed disabled/not stable before
its pads or any parameter changes. An exactly retained HSE is never rewritten.
Enable/read HSE and poll startup STABLE, select/read the source under guarded
buses, then install/read final dividers. Final Flash latency accounts for every
admitted fallback. Before publication verify HSI divider/trim/STABLE, HSE
parameters/STABLE, enables, mux/dividers, unchanged CCS policy and relevant
sticky HSE/LSE FAIL/FAULT flags. No sticky flag is cleared to hide an event.

## Failures and limits

Every software wait has a finite iteration budget while the CPU continues
executing. STABLE is a startup latch and does not prove ongoing source liveness
(RM PDF 59–62). Hardware detector 65/130 ms windows are not a software duration
or universal crystal startup limit. Source loss can stop CPU progress when
fallback is disabled. After source loss or fallback, frozen HSE rates no longer
describe hardware. A failure returns no frozen RCC clocks and can leave a
partial guarded/bridge state; reset before retrying. No rollback is promised.

No runtime frequency changes, PLL, LSE initialization, calibration, sleep/
deep-sleep entry or resume, or clock-loss recovery is implemented. DeepSleep
stops HSE/HSIOSC and wakeup may select HSI 8 MHz (RM PDF 51/72); preserving a wake
register is not a resume protocol.

`examples/shared/hse-clock` builds genuine crystal and bypass firmware for all three
exact L052 packages using UART 1 PA8 and PB0 LED activity. The board declares
3.0–3.6 V,−20..70 °C and actual oscillator endpoints. UART timing uses frozen
PCLK; the busy delay is not calibrated wall time. Production ARM compilation
and linking are recorded separately from this qualification receipt. No HAL
harnesses, register mocks, synthetic API tests, firmware execution or measured
oscillator behavior are claimed.
