# Direct HSE on CW32L083

The shared direct `rcc/l052_l083.rs` backend adds L083-qualified
`Config.hse: Option<Hse>` and `Config.sys: Sysclk` while retaining distinct
native L052/L083 register layouts. Generated HSE limits select `rcc_hse`, and
HSE or HEX limits select `rcc_external_clock` for bounds; pad facts come from
the selected package metadata. HSI remains the default and retained fallback.
L052 has its own qualification in `qualified-l052-hse.md`. An HSE declaration
initializes and reserves
that source even when the selected SYSCLK is HSI. The own-source identities and
claim mapping are in `qualified-l083-hse-source-receipt.json`. PDF pages below
are one-based; the printed page is one lower.

## Board and frequency contract

Own RM V2.0 PDF53–54 specifies crystal and digital-input HSE at 4–32 MHz. Own DS
V1.9 PDF52 allows bypass down to 1 MHz; this implementation deliberately admits
their 4–32 MHz intersection. The discrepancy remains unresolved. Every declared
actual minimum and maximum must fit: 4 MHz with negative tolerance and 32 MHz
with positive tolerance are rejected. There is no inferred ppm or HSI-derived
external accuracy. Include board load, temperature, supply, tolerance, aging
and short-term cycle variation, not only a long-term-average frequency rating.

VDD is 1.65–5.5 V with VDDA=VDD (DS Table7-4 PDF47). Ambient −40..85°C is the
retained factory-HSI accuracy intersection (Table7-17 PDF55); conditional 105°C
low-power operation does not widen it. Actual HCLK/PCLK must stay at or below
24 MHz below 1.8 V, and 64 MHz at or above 1.8 V. Both selected HSE and retained
HSI fallback are checked against final bus/Flash limits, including when CCS is
disabled. Retained HSIOSC is 48 MHz ±2% before its divider. HSI/1 may reach
48.96 MHz; HSI/2 may reach 24.48 MHz and is not a legal AHB/1 fallback below
1.8 V. One-time HSI- or HSE-fed SYSCLK PLL, including crystal and bypass
references, has a separate [qualified contract](l083-hsi-pll.md). Independent
PLL outputs, runtime retuning and guaranteed reference-loss recovery are not
provided.

FLASH WAIT0/1/2 cover actual HCLK ≤24/48/72 MHz (RM PDF121/131). The maximum of
selected-source and retained-HSI HCLK bounds determines the wait state. WAIT2
does not authorize a 72 MHz bus. Reserved FLASH bits are preserved. A nominal
24 MHz source with positive tolerance needs WAIT1, while an admitted HSI/1
fallback needs WAIT2 even when the selected HSE is ≤32 MHz.

Bypass waveform constraints apply simultaneously: duty 40–60%, high and low
each at least 15 ns, rise and fall each at most 20 ns, high 0.7×VDDIOx..VDDIOx,
low VSS..0.3×VDDIOx, and own GPIO ratings (RM PDF54; DS PDF52/58). For example,
32 MHz at 40% duty fails the pulse-width minimum. These DS clock values are
design-guaranteed, not production-tested. The board must provide a stable,
continuous source before initialization and throughout use.

Crystal mode requires board-qualified drive, load, ESR/external resistance,
layout and startup. DRIVER 0/1/2/3 is weakest/weak/recommended/strongest (RM
PDF80). Nominal FREQRANGE bins are 4–8, 8–16, 16–24 and 24–32 MHz. Shared
nominal endpoints use the established upper-bin convention, independently of
actual-envelope admission; 32 MHz uses the last bin. The own SDK confirms that
convention, but its complete initialization sequence is not the HAL contract.
WAITCYCLE=3 selects 262144 cycles as recommended by the RM example PDF70;
FLT=0 bounds this implementation. The typical DS 2 ms startup at 8 MHz is not a
universal maximum or software timeout guarantee.

## Security, retained owners and pads

CR1.CLKCCS/HSECCS/LSECCS are configurable RW bits (RM PDF76), preserved and
verified with LSELOCK and unrelated enables. Enabled/requested HSE with HSECCS,
or inherited enabled LSE with LSECCS, retains explicitly enabled unchanged LSI.
CLKCCS alone establishes no detector. A temporary LSI software request is
restored only when no retained detector needs it. LSI must already be legal in
the own RM 32.8 kHz ±10% range, 29,520–36,080 Hz (PDF62); STABLE does not prove
trim accuracy and factory ±3% is not assigned to arbitrary inherited trim.

HSE DETCNT is nonzero 11-bit. The own RM counts 131072 HSE edges per DETCNT LSI
cycles and recommends 8000/fHSE(MHz) (PDF65/80). The generated policy uses
ceil(8,000,000,000 / actual_min_HSE_Hz), at most 2047, and proves
actual_min_HSE_Hz × count > 131072 × 36080. This is a conservative detector
admission policy, not a clock measurement.

Read RTC through APBEN1.RTC bit3 and AUTOTRIM through APBEN2.AUTOTRIM bit13,
using the central inspection helper and restoring each gate (RM PDF87/89).
Do not reset either peripheral, write RTC ACCESS/WINDOW/calendar/keys, timers,
flags or interrupts, or invent an LVD gate.

- RTC SOURCE=4/5/6/7 reserves HSE/128,/256,/512,/1024 regardless of calendar
  START, because CR2.AWTEN/AWTSRC wake timing can use RTCCLK independently
  (RM PDF205/206). SOURCE=0/2 means LSE/LSI; 1/3 are rejected even without an HSE request.
- L083's standalone wake timer is AUTOTRIM MD=3, not the L031 AWT block (RM
  PDF178/187). The admitted retained active form is EN=1, MD=TIMER, AUTO=0.
  Active other modes, automatic calibration and reserved sources are rejected.
  HSIOSC=0 blocks a required raw-HSI retrim; HSE=2 reserves HSE; LSI=1 and LSE=3
  are preserved. ETR=4 with an HSE request is conservatively rejected because
  PF0 is one documented ETR input (DS PDF27). No input route is guessed.
- LVD EN && FLTEN && FLTCLK blocks required raw-HSI retrim (RM PDF530/534/535).
  The reset-capable consumer is not disabled to make initialization succeed.
  HSI divider-only changes preserve raw HSIOSC users.

Retained HSE owners allow reuse only when HSE is already enabled and ready,
every requested HSE field matches and its actual pad state matches. Successful
reuse performs no source or pad writes. Active calibration/ETR exclusions are
bounded HAL policy, not claims that the hardware lacks those modes.

Own DS Table5-2 PDF27 bonds PF0=OSC_IN and PF1=OSC_OUT on all five modeled exact
packages: RBT6/RCS6/RCT6 use LQFP64 pins5/6; MCT6 uses LQFP80 pins7/8; VCT6 uses
LQFP100 pins12/13. Both LQFP64 body sizes and RB/RC memory variants share the
same column. Existing pinout aliases project HSE_IN/HSE_OUT without inventing a
digital AF. GPIO uses actual keyed LCKR at +0x3c, KEY=0x5A5A (RM PDF162/163/168).
Only owned pads are unlocked. Direction becomes input first; pad-local pulls,
edge/level IRQ enables, open drain and filter are cleared; AF0 is selected;
then crystal uses analog mode, bypass digital input. Shared FLTCLK, unrelated
pads and GPIO ICR are preserved.

Successful initialization reserves configured or inherited enabled HSE for the
whole boot. Safe `Flex::new` checks before its first gate/pad write: crystal owns
PF0/PF1 and bypass only PF0, leaving PF1 usable unless an incoming enabled crystal
already reserved it for the boot. The common guard covers GPIO,
analog and competing peripheral constructors. Raw PAC access can invalidate
this contract. Common RCC separately protects
[inherited LSE pad ownership](inherited-lse-pads.md); broad inherited ETR pad
ownership remains outside this contract.

## Transition and failure contract

Initialization requires stable, continuous and electrically legal entry clocks,
correct entry Flash latency and a documented HSI divider. Ordinary peripheral
users, DMA, application interrupts and clock-changing NMI activity must be
quiescent. A critical section does not mask NMI. A firmware jump is not reset.

After pure validation and retained-owner checks, raise WAIT2 and install
monotonic AHB≥/4/APB≥/8 guards, preserving stronger entry divisors. L083 permits
PLL transitions only with HSI/HSE (RM PDF67): escape an inherited PLL through
unchanged HSI before any LSI bridge. Stop PLL and observe !STABLE before
changing its source oscillator/divider; preserve PLL configuration. Use
unchanged legal LSI when retrim or the inherited-PLL sequencing requires it.
HSI trim comes from 0x00100A00; erased calibration is an early error. Stop HSI
only for retrim; its divider can change live (RM PDF69).

Before changing HSE or its pads, observe both HSEEN=0 and HSE.STABLE=0. Preserve
reserved bits31:20 with mixed-register RMW; STABLE19 is read-only and has no
generated setter. Configure owned pads/source, enable, read back and poll
STABLE. Relevant HSEFAIL/HSEFAULT and inherited LSEFAIL/LSEFAULT flags are
checked before mutation, selection and publication. ICR is never cleared to
hide an event. Final requested source/divider/trim/enables, CCS and faults must
be verified before publishing frozen clocks.

STABLE is a startup latch: it clears when disabled, but later source loss does
not clear it (RM PDF63). Frozen HSE rates cease to describe hardware after HSI
fallback or source loss. Iteration-bounded waits cannot guarantee wall-clock
progress after CPU source loss. A hardware failure may leave a partial clock
tree. Errors after singleton acquisition consume the tokens and require reset;
there is no rollback or safe retry. RCC publishes only after its sequence
succeeds; a later time-driver hardware failure may still return no peripheral
tokens after verified RCC clocks were published.

## Production coverage and limits

`examples/shared/hse-clock` declares both crystal and bypass on every modeled exact
L083 part using the existing PA8 UART1/PB0 LED paths. `ci/check-hal.sh` includes
all ten L083 firmware links, the normal alias/exact-part L052 library
matrix, existing HSE families, and L052/L083 GTIM1 time-driver + rt + defmt compilation.
Selected-source PCLK supplies UART's nominal baud calculation; its
divider-error tolerance excludes oscillator tolerance. SPI, I2C and ADC
consume qualified bounds for timing/rate constraints, and timers retain a
qualified kernel-clock envelope. The time driver requires exact nominal
1 MHz divisibility before singleton acquisition and exposes the corresponding
actual tick bounds; nominal ticks are not calibrated wall-clock time. HCLK
bounds govern Flash and peripheral stabilization delays.

The source receipt records qualification, not build or hardware success.
Verification belongs to the matching frozen-snapshot receipts. No HAL unit
tests, register mocks, firmware execution or measured oscillator behavior are
claimed. Excluded are PLL output configuration, LSE setup, runtime switching,
low-power entry/resume, source-loss recovery, active inherited AUTOTRIM
calibration and failed-entry recovery.
