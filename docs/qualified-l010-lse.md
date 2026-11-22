# Native CW32L010 LSE and calendar

This qualification covers CW32L010F8P6, CW32L010F8U6 and CW32L010Y8M6 only, from their own User Manual CN Rev1.2 and Data Sheet CN Rev1.3. Exact source identities and page references are in `lse-l010-qualification.json` and `lse-l010-rtc-admission.json`. The implementation adds one-time board-qualified nominal 32768-Hz crystal/bypass startup and native RTC use. It does not add LSE SYSCLK, runtime retuning, low-power transitions, recovery or measured time continuity.

## Native declaration and monitoring

`rcc::Lse` has independent four-bit running and startup drive, wait count, board bounds, operating conditions, poll budget and a required `LseFaultDetection`. There is no amplitude setting. All source parameters are configured while disabled and retained afterward. Both drive levels must be selected for the actual crystal/load/board. The DS typical 1.50-second startup is not a guaranteed maximum; polling attempts are not a duration.

`StartupOnly` requires inherited LSECCS clear and leaves it clear. The source's own startup edge counter can set STABLE without CCS, but later loss may leave STABLE set indefinitely. No later LSEFAIL/LSEFAULT is promised in this mode. RTC acquisition and operations verify enabled/startup state and frozen source parameters; they do not prove that time is still progressing.

`MonitoredExistingRoutes` requires an already stable, legally operating LSI and freezes its unchanged trim/wait. LSIEN may be zero because native CCS requests LSI automatically. This path never starts or calibrates LSI. The legal inherited upper frequency is 36080 Hz from the own RM 32.8-kHz +/-10% range; STABLE alone does not establish the tighter factory +/-3% specification or prove legal trim. The existing legal-entry clock contract supplies that qualification. Admission requires the declared minimum LSE frequency times the 256-LSI-cycle window to exceed 129 times that upper LSI frequency: one full LSE edge beyond the documented 128-edge requirement provides phase margin. Individual-cycle bounds are required, not only a long-term frequency average.

Monitoring deliberately retains existing hardware fault routes. Runtime loss can set ATIM SBIF before BKE, request an enabled brake IRQ, asynchronously clear MOE through enabled brake logic, and enter configured ATIM/GTIM capture/trigger routes. AOE and the existing output configuration determine subsequent behavior. Closed timer gates, BKE=0, CEN=0 or interrupt masking are not treated as complete isolation proofs. The HAL does not clear CR2 protection, flags or IRQ masks, take timer handlers, suppress outputs or restore PWM. Effects can occur before an initialization error returns. Startup failure is a separate LSEFAIL path; the runtime brake response is not attributed to every startup failure.

## Public functional entry conditions

Public `init`, linked from `Config.lse`, carry the reviewed functional handover amendment. These conditions do not turn safe calls into prose-only Rust memory-safety obligations; the existing pre-Rust bus-master and exclusive initialization boundary remains intact.

When starting a disabled LSE with RTC SOURCE=0, the quiet RTC control image does not define the reserved RTC1HZ=0 output behavior. RTC_1Hz branches from the divider before the calendar, and START=0 is not a universal output gate. No dependent observer may remain on either RTC_OUT or RTC_1Hz:

- PB04/PB06 AF2 output pads and external users
- BTIM1/2/3 RTC trigger/gated/external-count and count-reset/update roots
- GTIM/ATIM RTC TI capture or trigger roots
- LPTIM RTC trigger roots and downstream timer/ADC-trigger/GPIO-filter cascades

Actual hardware-reset entry with no intervening owner setup supplies disabled/reset selectors and analog pads. RTC/LSE retention still receives explicit checks. Register resemblance, reset flags, missing HAL tokens and a closed gate do not establish reset history. Firmware jumps must establish these functional conditions independently. Dormant BTIM/GTIM/ATIM work gates are never opened to probe them.

GPIOB is an operational gate for the entire bank. Temporary enabling permits passive input sampling and filter/edge progress, including before a later rejection. Restoring the gate cannot undo sampling history or captured events. The handover cannot depend on the bank staying paused or on undisturbed retained filter/edge/AF transactions, including LSI/LPTIM-filtered and asynchronous participants. Interrupt masking is not capture isolation. No universal unrelated output-level transition is inferred from opening the gate. Unrelated pin controls, ODR, FLTCLK and flags are preserved.

## New-source admission and writes

Pure frequency/conditions/pin/poll validation precedes MMIO. Native central checks precede any configuration gate opening and reject SYSCLK=LSE, MCO=LSE, enable/pad locks, stale STABLE/ready/fault flags, enabled LSE IRQ requests, incompatible CCS, unready monitored LSI and held-reset inspection domains. All reset bits are observed with their own active-low semantics; none is pulsed to manufacture admission.

RTC, UART1, UART2 and LPTIM configuration-only gates are opened individually with bounded readback, inspected and restored. Restoration failure is an error and is not reported as rollback. No peripheral control, command or status is written during this phase:

- RTC source4..7 is rejected. Source0 requires CR0 excluding H24 zero, ACCESS/WAIT clear, CR2/COMPCFR1/IER/ISR zero. ISR=0 is mandatory. DATE/TIME/PSC/AWTARR are preserved and never used as a reset detector.
- UART source2 is rejected regardless of RXEN/TXEN, covering independent timing/autobaud owners.
- Enabled LPTIM with ICLKSRC=LSE is rejected, including external-count mode. With RTC source0, enabled RTC trigger sources1..5 and nonzero trigger detection are also rejected.
- The enclosing RCC backend preserves its ADC.EN=0, voltage, Flash, inherited-source and analog-consumer admission.

Only then does the requested pad window operate GPIOB. PB01 and (crystal only) PB00 must have input direction, analog mode, AF0, and no pull/open-drain/filter/edge enable. A new bypass request never changes PB00. In the same window and before PB01 modification or oscillator enable, PB04/PB06 digital-output AF2 is rejected for new source0 startup, without changing those pins. That veto does not prove the earlier gate opening was inert or replace uninspected timer handover conditions.

Crystal needs no pad mutation. Bypass clears only PB01.ANALOG and reads it back. GPIOB's incoming gate is restored with bounded readback. The LSE register is modified only for MODE, WAITCYCLE, DRIVER and PDRIVER; reserved fields and PINLOCK remain intact. A keyed CR1 RMW sets LSEEN and the selected CCS policy while preserving other sources and locks. Polling checks faults before readiness and verifies source/pads before publishing clocks. Errors publish no new clocks, clear no borrowed flags and retain reservations; partially applied oscillator or gate changes can persist.

Exact enabled-source reuse verifies unchanged mode, drive settings, wait, monitor policy, startup/fault state and pads. It rewrites no source or pad and does not reject unchanged RTC/UART/LPTIM ownership merely for consuming LSE. Inherited crystal/PINLOCK ownership is never shrunk on bypass, failure, Drop or forgetting a capability. The 16 previously qualified active-LSE packages continue using their unchanged implementation and monitor semantics. `lse=None` preserves the former L010/L011 inherited-state comparisons. The shared central inspection helper additionally attempts bounded restoration when its initial gate-enable readback fails; this explicitly changes that error path for every family using the helper, while leaving successful reads/readiness unchanged. Native L010 exposes enable and restoration failure separately; legacy callers retain their existing generic error classification.

## Held calendar source

The native `CalendarClock` variants are `HsiOsc` and `Lse`. `CalendarClock::new(sysctrl)` and `HsiOscClock` retain their existing HSI behavior. `LseClock::new` consumes SYSCTRL, PB01 and PB00; bypass consumes SYSCTRL and PB01. Both verify the frozen init record and expose `fault_detection()`. Neither acquisition nor Drop starts, stops or releases a source/pad.

RTC source selection, preset divider writes, readback admission and calendar tick bounds follow the held clock. LSE uses source0, PSC1=0 and PSC2=0x3fff, giving nominal two-Hz TICKCLK and one-Hz calendar transitions. Its complete board envelope remains attached to those bounds. The HSI source retains its previous qualification and safe exact-factor attach rule. A successful read is a point-in-time configuration/startup check, especially under StartupOnly, and never an uninterrupted-time guarantee.

The ordinary examples in `examples/l010-lse-clock` contain crystal, bypass and HSI calendar binaries. Linking and ELF checks are software evidence; oscillator loading/startup, bypass electrical conditions and silicon fault timing still require board validation.
