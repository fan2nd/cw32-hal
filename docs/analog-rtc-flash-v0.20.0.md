> Historical fourth stage. Current follow-on work: [v0.21 timer/DMA](timer-dma-v0.21.0.md).

# Internal ADC sources, retained RTC, Flash and window watchdog

This fourth stage adds drivers with explicit capability limits for hardware already described by the
data/PAC. It continues the eight-gap comparison without treating every STM32
feature as a requirement for CW32.

| Area | This release | Boundary |
| --- | --- | --- |
| Internal ADC | Dedicated blocking/IRQ reads of reference and temperature on both chips, F030 VDDA/3, and borrowed L012 DAC outputs | Raw samples; no invented calibration constants or Celsius accuracy. Internal sources cannot enter ordinary channel erasure, scan, external-trigger or DMA APIs |
| WWDT | Owned shared-IP driver, explicit irreversible start, checked refresh window and retained running clock | Reset-only; no early-warning IRQ or stop API; counter pauses in DeepSleep |
| RTC | Owned calendar with validated DateTime, consistent reads, explicit initialization/set, real LSI/LSE startup proof and retained clock/pins | No alarm, tamper, backup-register facade, compensation or low-power time driver; RC timing is nominal |
| Flash | Blocking byte programming and page erase in an explicitly reserved data partition, with bounds/protection/errors/readback and restored controller state | Unsafe construction establishes whole-system memory exclusion; no running-firmware update, hardware abort or wall-clock timeout |

See [ADC internal sources](adc-internal.md), [WWDT](window-watchdog.md),
[RTC](rtc.md), and [Flash](flash.md) for actual constructors, ownership and
hardware-specific restrictions. `ReadNorFlash` is implemented; writable
`NorFlash` is withheld because the CW manuals do not establish that trait's
power-loss corruption-containment guarantee. Inherent write/erase operations
remain available with their stated hardware contract.

LSE routes are new data facts: SYSCTRL.LSE_IN=PC14 and LSE_OUT=PC15 on both
supported chips. They flow through persisted JSON and generated sealed pin
traits. Static pin tokens are permanently consumed for a retained crystal;
dropping RTC cannot restore GPIO authority over a running oscillator.

RCC startup also fixes an existing calibration problem. Before writing LSI
trim, it now checks software enable, stability and clock-monitor demands, and
audits actual automatic requesters. L012 GPIO filters, enabled VC/LVD filters
and IWDT can start LSI without setting LSIEN. F030 VC/LVD/IWDT use their distinct
RC150K source and are not mistaken for LSI clients. The audit temporarily enables
configuration gates without reset and restores their previous values. Active
or starting LSI keeps its existing trim; later RTC startup never rewrites it.

All six board examples and the motor API are unchanged from v0.19. Their
existing zero-direct-PAC application access, 96 MHz L012 motor clock, ADC/fault
logic and motor interrupt-executor priority remain the baseline. The new Flash
API is not called by these programs.

The Flash audit also corrects F030 `ICR.PROG` from readable/writable to
write-only, as explicitly stated in RM2.5 §7.9.6 and its revision history.
The persisted JSON and PAC metadata now record its write-only hardware access.
As with upstream metapac, field-value helpers inspect local words; an ICR word's
PROG helper is not a valid hardware-status observation. The driver observes
errors through `ISR.PROG`. Other ICR fields and defaults are preserved.

## Constructor correction included before release

The EAU review found a real architecture gap: its constructor and MMIO were
hardcoded to one singleton. EAU, CORDIC, IWDT/WWDT and DAC now accept generic
`Peri<T>` constrained by sealed Instance traits, with concrete implementations
generated from metadata. Register access and clock ownership use T, CORDIC
IRQ/event/results use the same identity, and DAC channel/source identity flows
through generated OPA/VC peripheral-and-channel connections. Existing numerical
algorithms, interrupt ordering and dependency lifetimes are retained. See
[EAU](eau-instance.md), [CORDIC](cordic-instance.md), and [DAC](dac-instance.md).

The pinned upstream CRC, RTC and Flash owners use concrete singleton tokens,
so they remain concrete here. BGR is CW32's single shared sticky power domain;
this does not grant a second independent bandgap or erase its lifetime witness.

## Progress against the original eight gaps

| Original gap | Implemented stages | Remaining work |
| --- | --- | --- |
| OPA output to ADC | v0.17 borrowed output source, no duplicated GPIO token | Electrical settling remains board-dependent |
| DAC channel ownership and live sources | v0.17 independent channels and updatable dependency guards; v0.20 internal ADC borrow | No calibrated voltage promise |
| Typed finite ADC DMA and writable copy source | v0.17 real requests, ownership/quarantine and `copy_mut` | Further bus/timer endpoints; safe circular DMA lacks drain evidence |
| RCC resource/frequency/configuration | v0.18 counted gates, all audited HSI/AHB/APB profiles; v0.20 retained LSI/LSE | HSE/PLL system clocks, runtime switching and low-power/time recovery |
| Independent PWM channels | v0.18 genuine static split with shared enable/clock state | Capture/encoder and generic complementary PWM |
| PAC semantic enums | v0.18 reserved-preserving selectors; v0.19 CRC modes | Further selectors are incremental data work, not raw data/count enums |
| Remaining repeated fields | v0.18 regular/irregular arrays, full candidate audit | Heterogeneous fields retain distinct accessors deliberately |
| Common missing HAL coverage | v0.19 buses/CRC/IWDT; v0.20 internal ADC/WWDT/RTC/Flash | Features listed above plus bus-specific buffering/flow-control/target modes |

The next bounded stages remain visible: timer capture/encoder/complementary
ownership, additional typed DMA endpoints, and external/system-clock and
low-power transitions. This release claims no physical programming, RTC drift
measurement, watchdog reset execution, temperature calibration or board timing
validation. Build and focused verification results are in
[v0.20 validation](validation-v0.20.0.md).
