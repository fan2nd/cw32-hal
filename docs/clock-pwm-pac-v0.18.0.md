# Clock resources, independent PWM and semantic PAC fields

This records v0.18. Later bus/CRC/IWDT implementation and current remaining
stages are described in [v0.19](buses-v0.19.0.md).

This is the second implementation stage of the remaining Embassy comparison.
It adds real clock-resource lifetimes, broader HSI clock configuration,
independently owned PWM channels, and sourced semantic/indexed PAC fields.
It does not add the remaining bus/storage/RTC/watchdog drivers or every STM32
clock mode. See [validation](validation-v0.18.0.md).

## Clock resources and frequencies

`rcc::Config` now chooses an HSI divider, AHB divider and APB divider. All 16
documented L012 HSI divisors and nine documented F030 divisors are supported,
with eight AHB and four APB divisors. Defaults remain 4 MHz and 8 MHz. The old
L012 `hsi_frequency = HsiFrequency::Mhz96` spelling becomes
`hsi_divider = HsiDivider::Div1`; existing board programs keep their previous
APB divider and resulting frequency.

One counted resource represents each physical enable gate. Driver owners retain
guards; split owners retain the appropriate share. The final owner stops its
hardware before releasing the gate. Shared resets across independent peripherals
remain suppressed, and documented reset cross-effects are retained. The DMA
controller/channel ownership domain is handled explicitly. Forgotten guards and
quarantined ADC/DMA activity prevent unsafe clock shutdown/reset.

Schema10 records each audited bus clock separately from an optional fixed kernel
input. `rcc::bus_frequency::<T>()` and `rcc::frequency::<T>()` use those distinct
contracts. A kernel frequency cannot be requested for an unresolved muxed clock.
For example, L012 I2C has independent master/slave clock selections; F030 I2C's
serial generator uses PCLK. ADC/timer local prescalers still apply after the
reported input clock, and CW timers do not acquire an STM32-style APB multiplier.

Pending ADC/VC/CORDIC/ATIM IRQ dispatch first checks that its resource is still
clocked. The F030 comparator brake route retains ATIM's clock until route cleanup,
including a forgotten brake guard. Invalid fixed-1-MHz GTIM1 configurations are
rejected by `try_init` before clock writes, and the time driver retains GTIM1.
GPIO, the time driver and unsafe motor takeover deliberately retain clocks as
documented in [RCC resources](rcc-resources.md).

Only checked direct-reset HSI startup is implemented. External oscillators, PLL,
runtime switching and STOP/DeepSleep recovery remain open work. Frequencies are
nominal integer-Hz values; analog/peripheral electrical limits remain independent.

## Independently owned PWM channels

After choosing frequency and starting/stopping the counter, consume a static
`SimplePwm` with `split()`. Its optional `OwnedPwmChannel` values can move into
independent drivers/tasks and implement `embedded_hal::pwm::SetDutyCycle`.
They own their pins and coordinate shared enable/MOE state in critical sections.
Dropping one leaves its peers running; the final drop stops the counter and
releases the timer clock. Forgetting a channel retains that ownership.

Frequency and counter-running state are fixed after split. A 65536-tick period
returns `(Error::SplitPeriodTooLong, original_owner)` unchanged: exact full duty
at that period needs a shared forced-mode/phase transaction. Split periods up to
65535 support the complete 0–100% range independently. The whole-owner API still
supports the original full native range. See [timer/PWM](timer-pwm.md).

## Semantic values and indexed fields

The 46-IP audit adds 95 repeated field groups, for 156 arrays and 1,075 indexed
elements. Explicit bit-offset lists now handle irregular layouts, with original
names, access, widths and evidence retained in YAML and persisted JSON. DMA flags,
timer channel enables/events and trigger banks use indexed accessors. Heterogeneous
fields keep separate APIs; the [coverage table](pac-fields-v0.18.0.md) explains them.

The data now has 50 semantic enum fields, including SYSCLK/dividers, DMA
width/status/request modes, UART/SPI modes and selected timer/ADC/DAC/I2C controls.
Getters return a typed value directly. Reserved encodings round-trip through
total `from_bits`/`to_bits`, with exhaustive enums or sparse newtypes following
the pinned chiptool policy. Counts, addresses and payloads remain integers.
Enum representation does not authorize writing hardware-reserved values.

Consumers must migrate scalar repeated setters to indexed ones and use enum
variants for semantic fields. For example, `DMA.isr().read().tc(1)` reads channel 2;
L012 `ATIM.dier().read().ccie(4)` addresses CC5 at bit16, despite the irregular
gap after CC4. Register read/write permissions, side effects, bus widths and
reset values are preserved. [Schema10](schema-v10.md) documents validation and
the unchanged YAML → persisted JSON → PAC boundary.

## Remaining scope

General UART/SPI/I2C, CRC/Flash/RTC/watchdog drivers, safe ADC internal sources,
timer capture/encoder and additional typed DMA endpoints remain unimplemented.
Broader clock-source selection and low-power support also remain explicit gaps.
The safe finite ADC DMA/static quarantine contract and deliberate unsafe motor
API continue from v0.17; this stage does not change the board control algorithm
or claim measured timing, electrical behavior or complete STM32 parity.
