# Direct HSE on CW32L031, CW32R031 and CW32W031

This extension uses the existing shared `rcc_cw32l031_v1` PAC backend. It adds
`Config.hse: Option<Hse>` and `Config.sys: Sysclk`, with HSI remaining the default.
A declared HSE is initialized and its pins reserved even when SYSCLK uses HSI.
The own-source receipt is `qualified-l031-hse-source-receipt.json`; all PDF page
numbers in that receipt and below are one-based (printed page is one lower).

## Electrical declaration

The complete actual crystal or bypass envelope must fit 4–32 MHz. Each own RM
specifies 4–32 MHz, whereas each own DS allows bypass down to 1 MHz. This batch
admits their intersection and does not resolve the discrepancy. A nominal 4 MHz
source with negative tolerance, or 32 MHz with positive tolerance, is rejected.
HSE bounds are supplied by the board; no default ppm or HSI-derived accuracy is
inferred. Include tolerance, temperature, aging, loading, short-term cycle
variation and oscillator supply effects, not just long-term average accuracy.

The admitted supply ranges are L031 1.65–5.5 V, R031 2.2–3.6 V and W031 2.0–3.6 V.
The W031 bound is the intersection of its LDO and DCDC rows and requires no RF
mode read/write. Existing HSI-only W031 operation at 1.8–2.0 V is unchanged.
All three retain factory HSI and therefore use ambient −40..85°C. Conditional
105°C device operation does not qualify the retained HSI accuracy there.
Analog/RF supply requirements, including equal supply rails where specified,
remain board obligations; the RCC extension configures no RF peripheral.

L031 actual HCLK and PCLK must be at most 24 MHz below 1.8 V and at most 48 MHz
otherwise. R031/W031 use the 48 MHz ceiling in their admitted ranges. HSIOSC is
48 MHz ±2%, before HSI.DIV. A nominal 48 MHz bus is consequently not admitted.
For every declared HSE the software conservatively checks the retained divided
HSI against the final bus limits too, even when inherited CCS is disabled.
That is an admission restriction, not a hardware rule to enable CCS.

Bypass requires 40–60% duty, high and low pulses each at least 15 ns, rise and
fall each at most 20 ns, low VSS..0.3×VDDIOx and high 0.7×VDDIOx..VDDIOx, plus
own pad I/O limits. These conditions apply together: 32 MHz at 40% duty fails
the pulse-width minimum. The DS numbers are design-guaranteed, not production
tested. Crystal drive/load/external resistance/placement/startup must be board
qualified. The typical 2 ms startup at 8 MHz is not a universal timeout bound.

The actual upper HCLK determines FLASH WAIT, including the conservative HSI
fallback: WAIT0/1/2 cover ≤24/48/72 MHz respectively. WAIT2 does not authorize a
72 MHz bus. A 24 MHz ±30 ppm HSE at AHB/1 needs WAIT1. Reserved FLASH_CR2 bits
are preserved; this block has no F030 FETCH/CACHE fields.

## Clock security and retained consumers

CR1.CLKCCS, HSECCS and LSECCS are configurable bits on these chips. Every write
preserves them and final readback verifies their original values. They are not
F020/x030's mandatory-one controls. HSECCS with a requested or inherited enabled
HSE, or LSECCS with inherited enabled LSE, causes unchanged LSI to be enabled,
read back and retained. CLKCCS alone does not establish a detector. Otherwise a
temporary calibration bridge restores the original LSI software request.

The unchanged LSI bridge/detector must satisfy the RM safe 32.8 kHz ±10% range
(29,520–36,080 Hz). STABLE cannot prove arbitrary inherited trim is legal, and
factory ±3% cannot be assumed for an inherited oscillator. HSE DETCNT is set
while stopped to ceil(8,000,000,000 / minimum_HSE_Hz), nonzero and ≤2047. The
initializer additionally proves minimum_HSE_Hz × count > 131072 × 36080 with
exact integer arithmetic. This conservative implementation of the RM heuristic
is not an oscillator accuracy measurement. The longest documented startup count
is used (262144 HSE cycles), with FLT=0 and explicit typed drive. Nominal range
bins use the established upper-bin choice at shared endpoints; actual envelope
admission remains separate.

RTC and AWT configuration registers are inspected through their real central
RCC gates, which are restored with bounded readback. There is no peripheral
reset, RTC WINDOW/calendar/counter/key write, or interrupt write. RTC's HSE
SOURCE reserves HSE even with calendar START=0 because wake timing is independent.
An enabled AWT using HSE reserves it as well. The retained SOURCE field
is named in authored YAML using L/R/W RM §12.5.3 (PDF180/182/181). Its exact
existing RTC-layout reuse group also includes L083, whose own RM V2.0 PDF205
independently confirms the same six values. No layout or reuse membership changes. Such consumers permit reuse only
when HSE is already enabled/ready and every requested source/pad parameter
matches. Reuse performs no HSE or pad writes. Active AWT ETR is conservatively
rejected with an HSE request because ETR may occupy PF0; no route selector is
invented. HSI-only initialization does not change that pad.

HSI is stopped/retrimmed only when actual trim differs from factory trim. An
enabled AWT on raw HSIOSC, or enabled LVD with FLTEN=1 and FLTCLK=1, blocks a
required retrim before any oscillator/pad mutation. No independent LVD gate is
invented. DIV-only changes are live and preserve raw HSIOSC. AWT timing continues
to use `ClockBounds::hsi(1)`; independent RTC LSI qualification is unchanged.

## Pads and transition

Own package OSC_IN/OSC_OUT aliases yield SYSCTRL HSE_IN/HSE_OUT metadata with no
digital AF invention. L031 QFN20 has neither PF0 nor PF1 and rejects HSE. Its
package-less alias spans QFN20 and also advertises no universal HSE route.
L031 LQFP48/QFN48 use pins 5/6; QFN32/TSSOP20 use 2/3. R031 QFN48 uses 4/5 and
W031 QFN64 uses 63/64. The L031-family GPIO block has no LOCK/HIGHIE/LOWIE;
generated operations are selected from real register metadata. Pad programming
sets input direction first, clears only pad-local pulls/edge IRQ/open-drain/
filter controls, sets AF0, then selects analog for crystal or digital for bypass.
Other pins and shared filter-clock selection are preserved.

Successful RCC initialization freezes configured or inherited enabled HSE pad
ownership for the boot. `Flex::new` checks this before its first gate or pad
write, protecting all safe GPIO/AF/analog construction paths: crystal owns both
pads, bypass only the input. Common RCC separately protects
[inherited LSE pad ownership](inherited-lse-pads.md); broad inherited ETR pad
ownership remains outside this contract. No LSE/ETR parameters or pads are
reconfigured by this HSE operation.

The entry clock must be stable, continuous and electrically legal, with correct
entry Flash latency and a documented HSI divider. Unchanged HSI/LSI bridge
sources must also be legal. Ordinary clock-sensitive peripheral activity, DMA,
application IRQs and clock-changing NMI code, including RF host traffic on R/W,
must be quiescent. A critical section does not mask NMI. ROM boot selection does
not establish the state of an arbitrary firmware jump.

After pure configuration/timebase checks, the backend inspects retained owners,
enables FLASH centrally and raises WAIT2. It installs AHB=max(entry,/4),
APB=max(entry,/8), preserving stronger entry divisors, enables unchanged HSI
and selects it with readback/barriers. If necessary it uses unchanged legal LSI
to stop/retrim/restart HSI, then installs the requested live HSI divider. It
reuses matched retained HSE or stops it, observes disabled and !STABLE, programs
pads/source, then starts and polls it. Source selection is acknowledged under
guarded buses before final divisors. Final source/divider/HSI readbacks precede
lowering Flash latency, followed by a final mux and fault check.

STABLE is only a startup latch; later source loss does not clear it. Relevant
pre-existing HSE/LSE startup-failure or loss flags are deliberately rejected,
and flags are checked again before selection and publication. No ICR event is
cleared. A detected fault or bounded poll failure returns an error without new
RCC clocks or peripheral tokens. Poll budgets count iterations, include success
on the final poll, and do not establish wall-clock progress after CPU source
loss. Frozen HSE rates cease to be valid after automatic HSI fallback.

Pure validation precedes singleton acquisition. After acquisition, errors consume
the set and require reset; there is no transaction rollback or safe retry.
The pinned Embassy singleton macro makes `take` crate-private and `steal` unsafe.
RCC publishes only after its complete sequence succeeds. A later time-driver
hardware failure can occur after verified RCC clocks were published, while
still returning no peripheral tokens. This is a distinct documented failure case.

## Scope and validation

Real crystal/bypass UART/LED examples add exact L031C8T6, R031C8U6 and W031R8U6
packages to `examples/shared/hse-clock`. The existing HSI-only QFN20 build remains
required. Normal ARM library/example builds validate compiled production paths;
no firmware execution, HAL unit test, register simulator or mock is claimed.
Selected-source PCLK supplies UART's nominal baud calculation; its
divider-error tolerance excludes oscillator tolerance. SPI, I2C and ADC
consume qualified bounds for timing/rate constraints, and timers retain a
qualified kernel-clock envelope. The time driver requires exact nominal
1 MHz divisibility before singleton acquisition and exposes the corresponding
actual tick bounds; nominal ticks are not calibrated wall-clock time. HCLK
bounds govern Flash and peripheral stabilization delays. Peripheral-local partial mux metadata
remains partial, without fabricated fixed kernels.

Excluded: PLL, LSE setup, runtime switching, low-power entry/resume, external
clock-loss recovery, arbitrary trim/failed-source recovery, W031 HSE below 2.0 V,
RF configuration, and broad inherited low-speed pad ownership. Board waveform
and startup characterization remain physical requirements.
