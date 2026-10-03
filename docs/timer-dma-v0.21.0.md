# Timer capture, encoder, complementary PWM and typed bus DMA

This fifth bounded stage addresses timer input/output ownership and additional
finite DMA endpoints. It follows the fixed Embassy STM32 owner, pin-trait and
interrupt-binding architecture while retaining CW register semantics.

| Area | Implemented | Deliberate boundary |
| --- | --- | --- |
| Capture | One physical input per owned ATIM/GTIM, polling and IRQ waits, actual timestamp and available overcapture reporting | No simultaneous multi-input capture, continuous stream or timer DMA; F030 GTIM cannot report hardware overcapture |
| Encoder | Typed real two-input pair, x2/x4 quadrature modes, filters, inversion, 16-bit position and direction | No index reset or extended position counter; count/direction are separate observations |
| Complementary PWM | Optional main/N or A/B pairs on three routed ATIM channels, optional external brake, actual deadtime, explicit output enable and fault acknowledgement | Edge-aligned active-high outputs; no fourth complementary channel, automatic rearm, split owner or atomic cross-channel updates |
| UART DMA | Finite eight-bit TX and single-frame RX, real request/channel/IRQ mapping, TX wire drain | Static owned buffers and complete UART half ownership; nine-bit and multiword RX remain unsupported |
| SPI DMA | Finite two-channel full-duplex u8/u16, equal lengths, RX armed before TX, both DMA completions and bus drain | Explicit dummy buffers for one-way protocols; no in-place, unequal-length or circular transfer |

See [capture and encoder](timer-capture-encoder.md),
[complementary PWM](complementary-pwm.md), and [bus DMA](bus-dma.md) for APIs,
register evidence, cancellation, error and timing contracts.

## Safety and hardware distinctions

DMA operations consume complete static owners, channels and exclusive static
buffers. Pre-start validation failures return the resources. Only proven clean
DMA completion and peripheral drain release them after launch. Cancellation,
timeout or an error disables request generation, poisons the affected channels
and permanently retains all consumed resources. Forgetting an operation also
retains them. The undocumented CW DMA abort-drain guarantee is not inferred
from channel disable, peripheral flush, or upstream STM32 behavior.

UART RX is intentionally limited to one frame. The CW manuals say software
must acknowledge RC, while vendor multiword DMA examples omit an explanation of
DMA-specific acknowledgement. Those examples do not establish error-free
multiword RX. L012 overrun/error flags are checked; F030 cannot report a lost
receive word. This is a safe API bound, not a claim that the hardware lacks
multiword DMA.

L012 and F030 deadtime units differ. L012 uses PCLK and supports separate edge
values; F030 uses prescaled TTCLK and one shared value. Fault acknowledgement
never rearms outputs. Buffered compare writes can take effect at different
boundaries and are not advertised as an atomic multichannel update. Existing
ThreePhasePwm and all motor/example Rust sources remain unchanged.

## Metadata and comparison evidence

Schema 11 records each F030 GTIM instance's external capture selector explicitly.
It flows through actual persisted JSON and generated PAC metadata before HAL
trait generation. No timer-name parsing supplies a SYSCTRL index. Existing
pin routes remain unchanged: all 70 F030 and 72 L012 represented timer CH/BK
route tuples were checked against the original PDF alternate-function tables.
The same physical route can have distinct input and output contracts, which the
HAL exposes through separate sealed traits. See [schema 11](schema-v11.md).

Upstream comparison uses Embassy commit
[`b12a6d9efcd2711037abca1b63a661a9ef726444`](https://github.com/embassy-rs/embassy/tree/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src):
`timer/input_capture.rs`, `timer/qei.rs`, `timer/complementary_pwm.rs`,
`usart/mod.rs`, and `spi/mod.rs`. The driver records link the independent CW
manual sections; upstream source is an API/ownership reference, not CW hardware
proof.

## Progress against the original eight gaps

| Gap | Implemented stages | Remaining scope |
| --- | --- | --- |
| OPA output → ADC | v0.17 lifetime-protected output source | Board-dependent analog settling |
| DAC granularity/live sources | v0.17 split channels and source guards; v0.20 generic identity and internal ADC | Calibrated output accuracy |
| Typed DMA/reusable source | v0.17 ADC and copy_mut; v0.21 UART/SPI | Timer/I2C endpoints, multiword UART RX and safe circular DMA need further hardware proof |
| RCC | v0.18 counted gates and documented HSI/AHB/APB profiles; v0.20 retained LSI/LSE | HSE/PLL system clocks, runtime switching and low-power/time recovery |
| PWM ownership | v0.18 independent single-ended channels; v0.21 complementary pairs, capture and encoder | Features beyond the explicit bounded timer matrix |
| PAC semantic enums | v0.18 audited reserved-preserving selectors; v0.19 CRC modes | Incremental selectors, not enums for addresses/counts/raw data |
| Repeated fields | v0.18 regular/irregular arrays and candidate audit | Heterogeneous semantics remain separate deliberately |
| Common HAL coverage | v0.19 buses/CRC/IWDT; v0.20 ADC internals/WWDT/RTC/Flash; v0.21 timer and DMA extensions | Bus buffering/flow control/target modes, RTC alarms/compensation, watchdog IRQs, calibrated ADC units |

External/system-clock and low-power work remain the next distinct stage.
This release does not claim waveform, serial integrity, fault response, DMA
bus-drain timing or any other physical board validation. Verification scope is
recorded in [v0.21 validation](validation-v0.21.0.md).
