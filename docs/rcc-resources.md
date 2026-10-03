# RCC clocks and resource ownership

The clock tree is selected once during checked direct-reset startup. The HAL
loads HSI factory trim and accepts only reset HSI/source/bus state before
applying the requested HSI, AHB and APB dividers. Frequencies are nominal values,
rounded down to integer Hz when the oscillator/divider ratio is fractional.
They are not measured frequency or oscillator-accuracy guarantees.

`rcc::Config` selects `hsi_divider`, `hclk_divider`, `pclk_divider` and a bounded
HSI stability poll count. `Config::clocks()` calculates the nominal requested
frequencies without hardware access; `rcc::clocks()` reports the successfully
initialized tree. `rcc::frequency::<Peripheral>()` uses that peripheral's audited
kernel source, while `rcc::bus_frequency::<Peripheral>()` reports its register
interface clock. Unresolved or configurable kernel sources have no kernel-clock
implementation. ADC sampling and timer prescalers are applied by their drivers;
CW32 timer clocks are PCLK without an STM32-style APB multiplier.

## Supported startup configurations

- L012: factory-trimmed 96 MHz HSI oscillator divided by 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 12, 16, 20, 24, 28 or 32. Default remains /24, nominal 4 MHz.
- F030: factory-trimmed 48 MHz HSI oscillator divided by 1, 2, 4, 6, 8, 10,
  12, 14 or 16. Default remains /6, nominal 8 MHz. The vendor header's /3
  encoding is omitted because the maintained Rev 2.5 manual does not document it.
- Both: AHB /1, /2, /4, /8, /16, /32, /64, /128; APB /1, /2, /4, /8.

Bus dividers are applied before changing HSI. Flash wait states are set before
raising HSI, read back, and followed by a trim-preserving HSI write/readback.
An error after writes leaves any already-applied settings in place; it does not
attempt an unproven clock rollback or report an initialized clock tree.
L012 uses its documented SYSCTRL.CR2 FLASHWAIT alias. F030 enables its FLASH
configuration clock and uses keyed FLASH.CR2 WAIT writes. Reserved fields and
SWD/cache/prefetch configuration are preserved. RTC LSI/LSE startup is available
separately from v0.20. No HSE/PLL system-clock selection, runtime clock switching,
STOP/DeepSleep recovery or board-level validation is claimed. The application must satisfy the datasheet voltage/temperature limits;
L012 96 MHz requires VDD at least 1.8 V.

With `time-driver-gtim1`, `try_init` rejects incompatible 1 MHz timebase profiles
before any RCC MMIO with `InitError::UnsupportedTimeClock`. L012 needs a positive
integer PCLK/1 MHz ratio at most 65536. F030 needs a power-of-two ratio at most
32768. GTIM1 stays reserved and its clock is permanently retained.

The existing L012 motor examples keep HSI /1, AHB /1, APB /1: SYSCLK, HCLK
and PCLK remain 96 MHz. APB /2 remains available as a separate 48 MHz PCLK
profile. Default initialization remains unchanged on both
chips. Peripheral electrical clock limits still apply independently.

## Low-speed source retention

`RtcClock` starts and observes LSI/LSE without changing the system clock. LSE
requires the actual metadata-generated PC14/PC15 static pin tokens, permanently
reserved for this boot. A failed readiness wait retains its token for retries.
The oscillator and RTC gate remain enabled after owner Drop; this is not a
battery-backup or power-loss guarantee. See [RTC](rtc.md).

Global startup writes LSI factory trim only after an off-state audit. LSIEN alone
is insufficient on L012: GPIO/VC/LVD/IWDT and clock monitors can automatically
start it. The audit uses actual metadata instances, temporarily enables config
gates without reset, reads their demands and restores the previous gates. An
active or starting oscillator keeps its existing trim. F030 RC150K clients are
not treated as LSI consumers. Source startup never rewrites LSI trim or WAIT.

## Gate lifetimes and resets

One critical-section-protected resource record represents each physical gate.
Each live hardware owner stores a guard. Only the first owner enables the gate
and requests an eligible reset; the final guard disables that gate after the
driver's own shutdown. Retaining a guard for a split owner never resets hardware.
Counter overflow fails before touching the gate or changing the count. Pending
ADC/VC/CORDIC/ATIM handlers check resource state inside their MMIO critical
section and skip hardware access after the final driver gates the peripheral off.
DMA handlers already test persistent channel state before hardware access.

Metadata separately identifies bus/kernel sources, shared gates, reset lines,
ownership parents and reset side effects. Resets across independently owned
analog/timer siblings remain suppressed. A reset shared only by one complete DMA
controller and its descendant channel views is allowed for first whole-controller
acquisition. DMA channel construction never requests a controller reset.

DAC channel owners retain separate references, so either may outlive its sibling.
DMA channels also hold separate references. A forgotten owner leaks its reference
and therefore prevents subsequent reset/gate-off. ADC/DMA cancellation, error,
timeout or a forgotten transfer never treats EN=0 as proof of bus quiescence:
when a dropping owner remains quarantined, its resource is permanently pinned.
Re-splitting DMA preserves software poison and cannot reset a retained controller.

F030 comparator-to-ATIM brake sources retain the ATIM clock until the physical
route is removed. This also covers forgetting the borrow guard, dropping PWM,
then disabling/dropping the comparator: safety shutdown still has clocked MMIO.

Some gates are intentionally permanent. GPIO configuration persists through
copyable pin identities and analog/AF routes; GPIO setup pins the port clock.
The BGR analog reference is never disabled because ADC/VC/OPA hardware can enable
and share it automatically. Unsafe motor setup pins each taken-over clock without
resetting it because its deliberately configured hardware outlives the short
motor handle. Such takeover still requires the caller's existing exclusion and
DMA/interrupt coordination guarantees.

## Sources and architectural comparison

- CW32L012 User Manual v1.4, §§4.3.1, 4.5.2, 4.7.1, 4.7.3–4.7.4,
  printed pp.25–27, 40, 44–48; clock tree, dividers and Flash wait alias.
- CW32x030 User Manual Rev 2.5, §§4.3.1, 4.5.2, 4.7.1, 4.7.4,
  7.4 and 7.9.2, printed pp.44–46, 63, 68, 72, 111 and 121;
  clock tree, HSI encodings, Flash clock prerequisite and keyed wait writes.
- The exact metadata source strings identify the per-instance gate/reset and
  bus/kernel evidence; cross-effects are not inferred from field spelling.
- Embassy commit `b12a6d9efcd2711037abca1b63a661a9ef726444`,
  `embassy-stm32/src/rcc/mod.rs`: first-reference reset/enable and last-reference
  disable. CW32 uses owned guards, permanent quarantine pins, and conservative
  cross-effect suppression suited to its peripheral and DMA guarantees.

This is a bounded CW32 implementation, not a claim of STM32 RCC parity. External
host probes check all documented divider combinations and mocked MMIO ordering,
resource lifetime transitions, forgotten-owner and quarantine behavior. ARM builds
check the generated per-chip interfaces. Neither substitutes for board testing.
