# Init-only LSE system clock on three exact CW32L052 packages

`Config.sys = Sysclk::LSE` selects the existing `Config.lse = Some(...)` source
on **CW32L052C8T6, CW32L052R8S6 and CW32L052R8T6** only. The public enum gains
an exhaustive variant on these exact packages; downstream matches must handle
it. A missing LSE declaration is a configuration error. No family alias, other
package or L083 system target is added. Default HSI, existing HSI/HSE targets,
auxiliary LSE and the shared backend's L083/PLL paths retain their contracts.
L052 has no PLL; inherited SYSCLK selector 2 is rejected, and reserved CR1 bit 2
is preserved.

One board-qualified nominal 32768 Hz tuple supplies healthy SYSCLK, divided
HCLK/PCLK, `Clocks.lse`, `LseClock` and the RTC's /32768 calendar source. The
factory-LSI monitor and fixed fallback envelopes never replace or widen this
LSE tuple. This is an active, init-only healthy-source contract, without public
LSI SYSCLK, runtime clock switching, source-loss recovery, calendar continuity,
sleep/wakeup, RF-operation or silicon-validation guarantees.

## Board declaration and pins

Declare a positive per-cycle LSE minimum and a maximum that bracket 32768 Hz,
with maximum at most 1 MHz. These are board assertions across supply, load,
temperature, aging and short-term variation; average ppm alone is insufficient.
The 1 MHz ceiling does not qualify another nominal source. Board and source
intervals must fit 1.65–5.5 V, VDDA=VDD and ambient −40…85 °C. Independently
qualify the crystal, load capacitors, layout and analog drive, or bypass input
voltage, duty, pulse widths and edges. The source's 1.5 s typical crystal startup
has no maximum and is not a guaranteed completion deadline.

All three packages place **PC14/OSC32_IN at physical pin 3 and PC15/OSC32_OUT
at physical pin 4**, including both 64-pin R8 packages. Crystal mode reserves
PC14/PC15. Bypass reserves PC14 and permits PC15 only under the existing pad
policy and inherited reservations. Attempted enable, errors, drop and forget
do not release oscillator reservations. Own bypass limits are high
0.7×VDDIOx…VDDIOx, low VSS…0.3×VDDIOx, high/low pulse widths at least 450 ns,
rise/fall times at most 50 ns and 45–55% duty, together with full device I/O
requirements.

L052 has independent run `drive`/`amplitude` and
`startup_drive`/`startup_amplitude` fields. All four analog values, `wait` and
`mode` are installed while stopped, before enable. Neither analog bank is
rewritten at STABLE; the precise hardware phase-switch instant is unspecified.
No universal board-independent preset is qualified. The
[ordinary crystal/bypass examples](../examples/shared/lse-sysclk/README.md) deliberately
declare different startup and run values so that the independent fields remain
visible; each value still needs board qualification. Exact reuse compares both
banks, even in bypass.

## Exact bounds, configured HSI and fallback

For configured HSI divisor D, AHB divisor A and APB divisor P, SYSCLK keeps the
undivided LSE bounds, HCLK divides that same tuple by A and PCLK by A×P. Rational
division and exact electrical comparisons are retained; displayed integer
bounds round outward. The diagnostic HSI bounds remain factory HSIOSC divided
by D. Factory HSIOSC is nominal 48 MHz with 47.04–48.96 MHz bounds.

Final A/P are installed while configured HSI executes. Pure validation therefore
checks both LSE and **configured HSI at those final dividers**, even with no
requested HSE and CLKCCS=0. HCLK/PCLK ceilings are 24 MHz when minimum board
VDD is below 1.8 V, otherwise 48 MHz. D=1,A=1 is rejected because 48.96 MHz
exceeds 48 MHz; D=2,A=1 is rejected below 1.8 V because 24.48 MHz exceeds
24 MHz. Default D=6,A=1,P=1 is legal.

The own CLKCCS description names automatic HSI8MHz fallback. Its accepted
fixed-/6 rate model is nominal 8 MHz, bounded by 7.84–8.16 MHz independently
of D. The manual does not specify which HSI.DIV or bus-prescaler fields hardware
rewrites on fallback. Electrical coverage therefore separately uses the
**undivided 8.16 MHz upper bound for both buses and Flash**, independently of A
and P. It assumes neither retained dividers nor a particular post-fault register
image. This budget does not enable CLKCCS, certify fallback execution or keep
the published healthy LSE clocks valid after a fault.

Final Flash latency covers
`U = max(ceil(LSE_max/A), ceil(48960000/(D*A)), 8160000)` using
`WAIT = floor((U - 1)/24000000)`, after independent bus validation. WAIT2 remains
in force through transition guards, configured-HSI final-divisor installation
and acknowledgment of the LSE selection. It is reduced only after complete tree
verification. The 72 MHz Flash WAIT2 row does not raise the MCU's bus limit.

Every final A/P choice has nominal PCLK at most 32768 Hz. The fixed 1 MHz
Embassy time driver rejects this target before peripheral singleton acquisition
or hardware changes. No alternate timer rate or approximation is supplied.

## Native monitor and admitted source states

The monitor uses the own factory LSI halfword at 0x00100A02. Raw 0xFFFF is
rejected before masking to ten-bit TRIM; trim zero is valid. Its qualified rate
facts are nominal 32800 Hz, minimum 31816 Hz and maximum 33784 Hz. The target
alone requires strict `256 * LSE_min > 129 * 33784`: 17023 Hz fails, while
17024 Hz passes this count-margin condition. Hardware specifies 128 LSE edges
during 256 LSI cycles; the extra edge is software phase margin. These rate facts
do not prove LSI per-cycle jitter, every detector window or a loss deadline.
General auxiliary LSE does not acquire this target-only margin.

Native AUTOTRIM admission always precedes factory-LSI reading and any matching
shortcut. AUTO must be zero, mode must be defined (0, 1 or manual timer 3), SRC
must be 0…4, and an enabled module must be in manual timer mode. Disabled AUTO=1,
reserved mode/source, enabled calibration and reset-held state reject. An enabled
manual timer with SRC1 can be retained. Its configuration gate is restored;
an SRC-only test is insufficient.

LSI is classified before target-specific source mutation and rechecked after
preparation and before use:

- Factory match, disabled and both stable indications low: may request the
  unchanged source only with no selected-LSI state, retained detector or
  pending/enabled LSI-ready observer.
- Factory match, enabled and both stable indications low: may wait for an
  already requested startup only with no selected-LSI state or detector.
- Factory match, enabled and both stable indications high: may preserve the
  live source, including current LSI SYSCLK and retained detector consumers.
- Factory mismatch, fully stopped: only the existing two complete
  stopped/consumer/stopped passes authorize a trim-only write. WAIT and reserved
  bits stay unchanged. The conservative full cold-LSE consumer proof remains.
- Factory mismatch while enabled, stable, selected or serving a detector:
  rejects without stopping or retuning it.
- Disabled-but-stable/selected, selected-but-unready or disagreeing direct/ISR
  stable observations: rejects conservatively. Sequential reads can straddle
  startup; rejection does not diagnose defective silicon.

Either retained HSECCS or LSECCS requires already enabled, dual-stable,
factory-matching LSI, even if the corresponding external oscillator is disabled.
No ready/fault event or interrupt ownership is cleared to obtain admission.
LSI remains enabled for the detector after success, with frozen TRIM and WAIT.

Cold LSE requires stopped, unselected, unlocked and unchanged source state,
both stable indications low, no stale ready/fault/startup-fail state or relevant
enabled interrupt, complete native consumer admission and unused pads. Its full
13-register RTC reset proof includes SOURCE0, ALARMA 0x04120000, independent
compensation and wake/output ownership; START=0, SOURCE alone or a closed
configuration gate is insufficient. Enabled LSE is reused only when mode, WAIT,
both analog banks, pads, enable, readiness, faults and the already-ready factory
monitor match. Reuse writes neither oscillator parameters nor pads. An enabled
but unready LSE is rejected instead of being retuned or treated as cold.

## Functional handover and preserved ownership

Requesting factory-matching stopped LSI writes no TRIM/WAIT and is not a proof
that it has no consumers. It can resume parked UART1/2/3 SOURCE3 (the native PAC
field is `sorce`), admitted manual AUTOTRIM timer SRC1, GPIO FILTER.FLTCLK5,
MCO SOURCE4, and bonded PC4/AF6 on R8S6/R8T6 only. C8T6 has no bonded PC4.
Enabled and already work-ungated LPTIM ICLKSRC3 or LCD CLKCS0 can also resume.
Closed LCD/LPTIM work gates remain closed and are not opened for inspection.
There is no standalone AWT peripheral in this own-family roster.

RTC SOURCE2 is physically an LSI route, but **newly resuming it is not admitted
by the accepted LSI/LSE state combinations**. Cold LSE requires pristine RTC
SOURCE0, and reused LSE already requires a ready LSI monitor. A reused source
may retain an already running compatible RTC; initialization does not reset or
reconfigure the calendar to make it fit.

RTC/UART/AUTOTRIM gates control configuration access and do not establish
functional idleness. GPIO inspection can open entire GPIOA/B/C/D/F banks during
mismatching-LSI admission; cold-LSE direct-output and oscillator-pad admission
can run B/F and C respectively. Sampling, filters and armed events can advance,
including before an error. Unrelated pins, locks, selectors and flags are
preserved, but restoring gates cannot undo events. The functional handover must
permit this progress and changes to bus/output timing. Normal clock-sensitive
peripheral and interrupt work remains excluded during initialization.

The public safe entry model remains reset or a low-level handover that already
quiesced all bus masters before Rust application memory is used, including
outstanding accesses, armed/gated transfers and pending requests. These
functional limits neither replace that boundary nor impose a hidden additional
memory-safety obligation on ordinary safe Rust callers. Initialization cannot
retroactively sanitize an active-DMA jump and does not clear inherited owners,
reroute selectors or stop work gates to manufacture admission.

An unrequested enabled HSE stays enabled with its actual parameters and verified
pad modes. A requested enabled HSE requires full native parameter/pad match;
a live mismatch rejects on this target. Requested disabled HSE uses the existing
own cold admission with RTC/AUTOTRIM/ETR ownership checks. Retained PF0/PF1 HSE
ownership remains reserved; PF1's direct LSE route is also part of cold admission.
HSI is retained as the qualified escape. HSI trim changes require the existing
AUTOTRIM/LVD dependency checks and a verified temporary LSI bridge, with HSI
stopped before writing TRIM. L052 HSI has no WAIT field: trim-only updates
preserve DIV/reserved bits; permitted live divider-only updates preserve
TRIM/reserved bits. These rules do not promise continuous raw-HSIOSC outputs.

## Transition, publication and later RTC use

After entry admission, initialization enables the Flash configuration gate,
sets/readbacks WAIT2 and installs guards retaining at least AHB /4 and APB /8,
with stronger incoming division preserved. It verifies and selects unchanged
HSI under guard before any required HSI trim bridge. Ready factory LSI is
verified before dependence, configured HSI is established, optional/inherited
HSE is verified, and cold or exactly reused LSE is made ready while HSI executes.
CLKCCS, HSECCS and LSELOCK remain unchanged; cold LSE enables LSECCS/LSEEN
with a ready monitor. No fault is cleared to obtain success.

Final A/P are installed with HSI explicitly selected and acknowledged under
WAIT2. Complete source, monitor, policy and pad checks precede **one intentional
final LSE mux write**. There is no subsequent CR0 write, divider update, source
retry or cleanup that can replay a stale LSE selection after asynchronous
fallback. Fault-aware readback and barriers acknowledge the tree. Complete
checks surround Flash reduction and are repeated after gate-preserving pad
inspection, which can itself advance events.

Only complete success publishes the target monitor marker and frozen clocks in
the same initialization critical section. RCC failure publishes neither and
returns no peripheral set. Earlier internal monitor preparation is not success
publication. Errors can retain partial guards, gates, trim setup, source enables
and pad reservations; no rollback or retry without reset is promised. Ordinary
reset can retain LSE controls, so POR may be needed. All polling budgets count
iterations and require CPU progress; they do not bound elapsed time.

Later `LseClock` acquisition uses the same frozen source, verifies health/pads
and never reconfigures it. Common LSE health checks exact parameters, enable,
stable and faults. Only after this target succeeds does its additional marker
require software LSIEN, direct and ISR LSI stable, unchanged factory TRIM/WAIT,
preserved CLKCCS/HSECCS/LSELOCK and enabled LSECCS. An absent marker contributes
no additional requirement to existing auxiliary-LSE callers. Drop/forget does
not stop shared oscillators or release pads; no RTC compensation/calendar change
is part of system initialization.

These observations do not prove the next clock cycle or repair arbitrary loss
during earlier read-modify-write operations. CLKCCS=0 can leave the CPU unable
to progress after loss; preserved fallback also invalidates frozen LSE timing.
There is no transparent recovery, clock republication or continued-clock promise.
The incoming executing source, voltage and Flash state must already be legal.

Source binding: [own SYSCLK qualification](l052-lse-sysclk-qualification.json),
[native LSE facts](qualified-l052-lse.md) and
[native RTC reset roster](lse-active-l052-rtc-admission.json). Build/link results
and independent implementation review must be reported for their frozen inputs;
neither constitutes physical silicon qualification. Generated PAC, chip JSON,
reports and compiler outputs remain outside source deliverables.
