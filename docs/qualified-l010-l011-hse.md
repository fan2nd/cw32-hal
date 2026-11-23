# Direct HSE on CW32L010 and CW32L011

The existing `rcc/l010_l011/` backend supports one-time direct crystal or
bypass HSE initialization on CW32L010F8P6, CW32L010F8U6, CW32L010Y8M6,
CW32L011K8T6 and CW32L011K8U6. `Hse` requires nominal, minimum and maximum
frequency, board operating conditions, mode and drive. `Config.sys` selects
HSI or HSE; HSI remains the default and stays running. Declaring an HSE also
initializes and reserves it when the selected system source remains HSI.
No PLL or runtime clock-reconfiguration path is introduced. CW32L012 remains
a separate qualification and backend.

The later [exact L010 live-only LSI target](l010-live-factory-lsi-sysclk.md)
uses its own live-LSI guards and a target-only single-GPIOA HSE inspection
window. Existing HSI/HSE/LSE targets retain their original helper behavior,
ownership, electrical limits and error mapping. The addition creates no native
LSI RTC source or HSE source migration.

## Sources and scope

This qualification follows each family's own documentation:

- [L010 RM CN Rev 1.2](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf):
  cover May 2026, revision-history date 2025-09-19, upload path 2026-06-26.
- [L010 DS CN Rev 1.3](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf):
  revision-history date 2026-04-16.
- [L011 RM CN Rev 1.1](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf):
  cover June 2026, revision-history date 2025-09-19, upload path 2026-06-02.
  An older manual with the same revision number has different bytes; use this
  selected snapshot.
- [L011 DS CN Rev 1.1](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf):
  cover July 2025, revision-history date 2025-05-19.

Canonical source identities remain in `sources/evidence-sources.json` and
the source-qualified clock, electrical, register and pin YAML. SDK L010 1.0.9
and L011 1.0.3 corroborate implementation details but do not override manual
limits. Links below use one-based PDF pages; printed pages are distinguished
where useful. No vendor PDF, SDK, extracted text or rendered page is added to
the source deliverable. These are documentary and software contracts, not
measurements of a particular board.

## Board and electrical contract

Crystal mode and L011 bypass admit **actual 4–32 MHz**. L010 bypass admits
**3,908,159–32,000,000 Hz**, pending software acceptance. Ranges include all declared
endpoints, with `0 < minimum <= nominal <= maximum`. The entire board
operating envelope must be covered by the source envelope. Include tolerance,
supply, temperature, loading, aging and short-term cycle variation. A nominal
4 MHz or 32 MHz source with error extending outside the interval is not
admitted; long-term average accuracy alone does not prove cycle bounds.

L010's own RM and DS agree that bypass hardware can operate at 1–32 MHz;
The band below 3,908,159 Hz needs a different detector/startup policy. L011's RM says 4–32 MHz
while its DS permits bypass from 1 MHz; the initial interval is their safe
intersection and does not resolve that discrepancy. Both crystal intervals
are 4–32 MHz. Sources: L010 RM printed 48–49/PDF 49–50 and DS Tables 7-14,
7-16 [PDF 39–41](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf#page=39);
L011 RM [PDF 47–48](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=47)
and DS Tables 7-14, 7-16 [PDF 47–49](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf#page=47).

| Requirement | CW32L010 | CW32L011 |
| --- | --- | --- |
| VDD | 1.62–5.5 V | 1.7–5.5 V; VDDA=VDD |
| Ambient qualification with factory HSI retained | −40–85 °C | −40–85 °C |
| Actual HCLK/PCLK ceiling below 1.8 V | 24 MHz | 24 MHz |
| Actual HCLK/PCLK ceiling at or above 1.8 V | 48 MHz | 96 MHz |
| Raw factory HSIOSC, independent of HSE | 48 MHz ±2%: 47.04–48.96 MHz | 96 MHz ±2%: 94.08–97.92 MHz |
| Default HSI divider | /12 | /24 |
| Documented Flash WAIT values | 0/1 through 24/48 MHz | 0/1/2/3 through 24/48/72/96 MHz |

The minimum declared supply selects the bus ceiling. An interval crossing
1.8 V therefore uses 24 MHz. The conditional low-power 105 °C datasheet
extension does not extend the retained factory-HSI accuracy qualification.
Own general conditions and HSI accuracy: L010 DS Tables 7-4/7-18
[PDF 32/43](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf#page=32);
L011 DS Tables 7-4/7-18
[PDF 39/51](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf#page=39).
Flash: L010 RM [PDF 114](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=114),
L011 RM [PDF 113](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=113).

Validate selected-source, requested-HSI and forced-fallback bounds through
the final AHB/APB divisors. Flash latency uses the greatest admitted actual
HCLK maximum. Nominal 24 MHz with positive external error requires WAIT1 at
AHB/1, and is excluded below 1.8 V at AHB/1. Undivided factory HSI exceeds
each family's maximum bus frequency at its positive-error endpoint; a Flash
wait state does not authorize an overclocked bus.

Bypass must satisfy all waveform limits together: 40–60% duty; high and low
pulses each at least 15 ns; rise and fall each at most 20 ns; input high
0.7–1.0×VDDIO and low VSS–0.3×VDDIO, plus the actual pad's I/O ratings.
At 32 MHz, 40% duty does not meet the 15 ns minimum pulse width. The source
must already be stable and remain continuous. Crystal drive, load, ESR,
layout and startup need board qualification. The datasheets' 2 ms startup is
typical, not a maximum timeout. These obligations cannot be checked from a
frequency declaration or a successful firmware build.

## Native register fields and exact pins

Both HSE registers have DRIVER[3:0], WAITCYCLE[5:4], MODE6, HEXENPOL7,
DETCNT[18:8], STABLE19, PDRIVER[23:20] and DIGFLT24. There is **no frequency
range selector**. Only documented drive codes 0–7 are qualified for both
phases, despite the four-bit field width and broader SDK macros/comments.
The same board-selected `HseDrive` sets PDRIVER and DRIVER. WAITCYCLE=3
selects 262144 cycles; DIGFLT stays off. HEXENPOL and reserved bits are
preserved; automatic external-oscillator enable output is outside this API.
All parameters are configured and read back while the unowned HSE is stopped.
See L010 RM [PDF 73](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=73)
and L011 RM [PDF 71](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=71).

| Exact part | Package | OSC_IN | OSC_OUT |
| --- | --- | --- | --- |
| CW32L010F8P6 | TSSOP20 | PA0, pin 5 | PA1, pin 6 |
| CW32L010F8U6 | QFN20 | PA0, pin 2 | PA1, pin 3 |
| CW32L010Y8M6 | SOP16 | PA0, pin 4 | PA1, pin 5 |
| CW32L011K8T6 | LQFP32 | PC13, pin 31 | PB7, pin 30 |
| CW32L011K8U6 | QFN32 | PC13, pin 31 | PB7, pin 30 |

The own DS Table 5-2 establishes these routes: [L010 PDF 24](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf#page=24)
and [L011 PDF 31](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf#page=31).
L010 oscillator pads are TTa; L011 oscillator pads are TC. Crystal reserves
both pads; bypass reserves OSC_IN. Pads belonging to an incoming enabled HSE
remain reserved for the entire boot, including after a switch to bypass or
HSI. Safe GPIO and peripheral construction observes that reservation. Raw PAC
writes can invalidate it.

These GPIO variants have no pull-down, separate input-enable or lock register.
Owned-pad setup uses their actual DIR/PUR/AF/ANALOG, edge-interrupt, open-drain
and filter controls. Input direction precedes AF0 and analog crystal/digital
bypass selection. Unrelated pins, shared filter-clock controls and interrupt
flags remain unchanged. There is no write to a fabricated PDR or lock field.

## Security policy, LSI and fixed fallback

HSECCS/LSECCS select detectors; CLKCCS separately enables automatic system
fallback. Preserve the inherited policy instead of enabling all three.
Both families' fallback is nominal **4 MHz**, equivalent to L010 HSI/12 or
L011 HSI/24, independently of the requested HSI divider. Its factory envelope
is **3.92–4.08 MHz** and is admitted separately even if CLKCCS is disabled.
The manuals state the effective fallback rate but do not fully specify its
internal DIV/enable register mutation. Requested-divider readback alone
cannot prove clock rates after a fault. See L010 RM
[PDF 57–59](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=57)
and L011 RM [PDF 55–57](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=55).

LSI is automatically requested by external detectors, certain GPIO/VC/LVD
filters and IWDT without necessarily setting software LSIEN. Preserve its
trim, startup wait and incoming software request. Removing only a software
request does not imply STABLE should clear. Retained detector operation
requires unchanged, legally operating, stable LSI.

L010's inherited-LSI safety range is the RM's 32.8 kHz ±10%, maximum 36080 Hz;
the tighter DS factory accuracy cannot be assigned to arbitrary inherited
trim. L011's RM also says ±10%, but its DS factory row says −10/+25%, up to
41000 Hz. The detector calculation conservatively uses 41000 Hz for L011
while still requiring a legal incoming LSI. It does not resolve that source
conflict or assert all factory LSI behavior fits the narrower RM range.
The L011 conflict is visible in RM [PDF 54](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=54)
and DS [PDF 51](https://www.whxy.com/uploads/files/20250724/CW32L011_DataSheet_CN_V1.1.pdf#page=51).

DETCNT is `ceil(8_000_000_000 / actual_minimum_HSE_Hz)`, must fit 1–2047,
and must prove `HSE_min * DETCNT > 131072 * LSI_max`. L010's lower bound is
`ceil(8_000_000_000 / 2047) = 3_908_159 Hz`: the product is 8,000,001,473,
above `131072 * 36080`, and the count is 2047. One Hz lower requires 2048,
which cannot fit. L011 retains its 4 MHz source-policy lower bound.
The 3.95 MHz example declares 3,949,881–3,950,119 Hz and uses count 2026.
This extension has not been generated, compiled, tested or hardware-validated;
stale generated metadata still enforces the previous bound. Extending to 1 MHz
requires a separately qualified detector/startup policy.

## Retained consumers and entry conditions

The incoming source, HCLK/PCLK and Flash latency must already be legal and
stable. The initializer cannot repair past bootloader overclocking. Validate
source encoding and HSI divider; reject erased factory calibration at
0x001007C0. Keep DMA, other bus-dependent users and application interrupts
quiescent. A critical section does not mask NMI. Peripheral inspection uses
the actual central configuration gates, reads them back and restores them;
it does not reset retained peripherals.

- **RTC/AWT:** RTC SOURCE is LSE=0, HSE=1, LSI=2 or raw HSIOSC=3; 4–7 are
  reserved. Calendar START=0 does not establish that the source is unused,
  since AWT can use RTCCLKD. HSE ownership therefore admits only an already
  enabled/stable HSE with exact parameter and pad reuse, without stopping or
  rewriting it. Raw-HSIOSC ownership requires an already enabled, stable,
  factory-correct HSI and prohibits a necessary stop/retrim. Divider-only
  changes do not change raw HSIOSC. Preserve calendar/AWT registers and
  unrelated source selections. Own RTC/AWT registers: L010 RM
  [PDF 148–153](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=148),
  L011 RM [PDF 148–153](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=148).
- **ADC/BGR:** ADC uses PCLK/1, /2, /4 or /8. Require ADC.CR.EN=0 before
  clock changes; do not assume the absence of a pending hardware trigger.
  Preserve BGREN, TSEN and references. BGR is in ADC control and may support
  a retained comparator independently of ADC conversion, so do not reset ADC
  or stop BGR to inspect it. These families have no separate BGR clock mux.
  See L010 RM [PDF 504/513/517](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=517)
  and L011 RM [PDF 506/518](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=518).
- **LVD:** An enabled detector with nonzero filtering and FLTCLK=SYSCLK is
  excluded before a clock change. An unchanged LSI-filtered or unfiltered
  detector can remain enabled, with thresholds, actions and flags preserved.
  LVD shares its analog gate with VC; a group reset is not safe inspection.
  Own source is SYSCLK, not the raw-HSIOSC option of other families. See
  L010 RM [PDF 543/547–548](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=547)
  and L011 RM [PDF 545/549–550](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=549).
- **VC:** Retaining analog enable is insufficient when timing depends on
  PCLK. Enabled PCLK-filtered comparators with nonzero FLTTIME are excluded.
  Enabled timer-triggered blanking also uses PCLK for its duration and is
  excluded; selecting an LSI filter does not make blanking independent of
  the bus clock. Preserve independent analog/LSI operation, reference
  controls and flags. L010 PA0 is also VC1_CH1, so an enabled VC1 selecting
  it on either input blocks HSE restart/pad configuration. The check derives
  each positive/negative mux from the exact package's source-qualified routes;
  it does not substitute a logical source number. Already-running exact HSE
  reuse performs no oscillator or pad writes and may preserve this owner.
  L011's own oscillator pins have no such VC input overlap. L010 DS Table5-2
  [PDF24](https://www.whxy.com/uploads/files/20260416/CW32L010_DataSheet_CN_V1.3.pdf#page=24)
  and RM [PDF537](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=537)
  establish the pad and input mux. See L010 RM [PDF 538–539](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=538)
  and L011 RM [PDF 540–541](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=540).

The own complete memory maps and clock/register inventories do not document
an AUTOTRIM block for L010/L011; no guessed register probe is introduced.
Other retained peripherals still fall under the quiescent-entry contract.

## Bounded initialization and failures

After pure validation and retained-owner inspection, enable/read Flash
configuration access, raise WAIT to the own documented maximum (L010=1,
L011=3), and install at least AHB/8 and APB/8 without weakening greater
inherited divisors. These guards accommodate the own legal inherited HSIOSC
ranges during changes, including low-voltage operation.

Prefer retaining a running factory-correct HSI. Live divider changes preserve
TRIM and are explicitly allowed by both own manuals. If retrim is necessary
and no raw-HSI owner prevents it, use unchanged legal LSI as a temporary
bridge. Temporarily suppress automatic CLKCCS before stopping HSI, observe
enable/STABLE clear, program and read factory TRIM plus requested DIV, restart,
return to HSI and restore the original security policy. Do not reset or
retune LSI. No HSI WAITCYCLE field exists on these families.

For unowned HSE, select/read HSI before stopping HSE. Observe HSE disabled
and STABLE clear before configuring pads or oscillator parameters. An exact
retained HSE is reused without writes. Enable/read HSE, await startup STABLE,
select/read the requested source under guarded buses, install/read final
divisors, then lower Flash latency only to a value covering every admitted
clock envelope. Verify source enables, trim/divider, oscillator configuration,
STABLE, mux/dividers, security policy and relevant sticky faults before
publishing frozen bounds.

STABLE is a startup latch and does **not** clear on later oscillator failure.
HSERDY is a separate edge flag. L010/L011 even differ on whether clearing
HSEFAIL/HSEFAULT also clears HSERDY, so initialization does not clear any of
those flags. Relevant sticky HSE/LSE failures are rejected instead of hidden.
Own flags: L010 RM [PDF 76–78](https://www.whxy.com/uploads/files/20260626/CW32L010_UserManual_CN_V1.2.pdf#page=76),
L011 RM [PDF 74–76](https://www.whxy.com/uploads/files/20260602/CW32L011_UserManual_CN_V1.1.pdf#page=74).

Every software wait has a finite CPU-iteration budget while execution can
continue; it is not a guaranteed wall-clock deadline. The hardware 65/130 ms
detector windows are not software duration guarantees. External loss may stop
CPU progress when fallback is disabled. Later fallback invalidates frozen
HSE-derived rates even if execution continues. An RCC initialization failure
returns an error without publishing frozen clocks. A later
time-driver error can occur after RCC has already frozen clocks, as documented
by `init`. Hardware may remain in a partial guarded or bridge state. Reset before retrying; rollback to a failed external source is
not promised.

No runtime retuning, fault recovery, PLL, LSE initialization, sleep/deep-sleep
entry or resume is implemented. Preserving wake-control registers does not
establish a resume protocol.

## Exact firmware and verification boundary

[`examples/shared/hse-clock`](../examples/shared/hse-clock/README.md) supplies real Cortex-M
entry points and selected-part linker memory for all five exact features.
Both `crystal` (16 MHz) and `bypass` (24 MHz) use a board-declared ±30 ppm source,
VDD 3.0–3.6 V and ambient −20–70 °C. L010 uses UART1 PA6 and LED PA3; L011 uses
UART1 PA9 and LED PB0. These are bonded own-source routes and do not consume
HSE, LSE, reset or debug pads. UART baud calculation uses frozen PCLK. LED
busy delay is observable activity, not calibrated timekeeping.

The example README includes exact production ARM build commands. Source/PAC
checks and ARM compilation/linking results must be recorded separately for
the tested snapshot; this qualification text does not claim they ran. No
register mocks, HAL harnesses, synthetic tests, firmware execution, measured
waveforms or physical clock-loss recovery are implied by these examples.
