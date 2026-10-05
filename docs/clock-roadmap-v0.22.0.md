# Clock startup and the eight-gap implementation ledger

The six bounded implementation stages have addressed the original eight-gap
comparison with real CW hardware and explicit ownership contracts. This release
adds HSE on both chips and PLL on F030. It does not claim full STM32 feature
parity or managed runtime/DeepSleep support.

## Current clock result

- HSI defaults remain 4 MHz L012 / 8 MHz F030; every previously supported HSI
  divider remains available, including the existing nominal floor reporting.
  The L012 motor's 96 MHz profile and all motor/example Rust source remain unchanged.
- Both chips support 4–32 MHz HSE crystal or external input. F030 additionally
  supports HSI/HSE-fed PLL with the intersection of documented configuration and
  datasheet limits: input 4–24 MHz, output 12–64 MHz, multiplier 2–12. L012 has no PLL.
- Requested voltage, rational bus/source limits and fixed 1 MHz Embassy timebase
  constraints are checked before MMIO. Flash waits precede faster clocks;
  readiness and configuration readback must succeed before publishing frequencies.
- PF0/PF1 are generated from actual HSE pad metadata. Their fields become
  Option<Peri>: crystal reserves both; bypass reserves PF0 and leaves PF1;
  HSI/PLL-HSI leave both available. No other singleton field changes shape.
  Pure invalid requests leave initialization available, while any failed hardware
  attempt returns no tokens and requires reset before retry.

The L012 manual's PB7/PC13 example conflicts with its datasheet and SDK.
The implemented PF0/PF1 routes use the independently corroborated physical pin
facts; the conflict and an additional vendor bypass-code port error are documented
in [external clocks](external-clocks.md).

## Original eight gaps

| Gap | Implemented result | Explicit remaining boundary |
| --- | --- | --- |
| OPA output to ADC | Borrowed OPA output channels preserve the original pin/OPA ownership and analog dependencies | Analog settling/accuracy requires board evidence |
| DAC channels and live sources | Independent channels, updatable source guards, generated instance and OPA/VC channel connections, borrowed internal ADC source | No calibrated physical-voltage promise |
| Typed DMA and mutable copy source | Finite ADC, UART and SPI endpoints; writable copy sources return after clean completion; real request/IRQ metadata | Static resources and permanent abort/error quarantine remain. Multiword UART RX, I2C/timer endpoints and safe rings are not established |
| RCC ownership/frequency/configuration | Counted shared gates, reset cross-effect protection, peripheral bus/kernel frequency, all audited HSI dividers, retained LSI/LSE, HSE and real F030 PLL | Runtime reclocking and managed DeepSleep require the missing transition protocol described below |
| Independent PWM channels | Static single-ended channel split; generic complementary pairs with brake/deadtime; capture and typed QEI | Single-input capture, no timer DMA or atomic cross-channel update claim; F030 GTIM has no overcapture indicator |
| PAC semantic enums | Audited selectors with total reserved-value representation; counts/addresses/data remain raw | Additional semantic selectors are incremental data work, not an API correctness gap |
| Repeated fields | Regular and irregular indexed field/register/subblock arrays, all-IP candidate review and validated source identities | Different access/default/side-effect semantics remain separate deliberately |
| Common HAL coverage | UART/SPI/I2C controller drivers, CRC, IWDT/WWDT, internal ADC, RTC/low-speed sources and reserved-partition Flash | Bus buffering/flow control/target modes, RTC alarms/compensation, watchdog IRQs and calibrated ADC units remain beyond the bounded implementations |

All these stages retain the one Rust generator and actual YAML → persisted JSON
→ PAC boundary, shared kind/version IP modules, metadata-generated driver/pin/IRQ
identities and typed access permissions. The separate unsafe motor API remains an
intentional user-requested ownership boundary; the original motor control and
fault policy are not replaced by generic drivers.

## Runtime and low-power decision

Both CW families support clock switching and DeepSleep in hardware. The current
HAL has no complete transaction for freezing every affected owner, handling
forgotten/static split owners and quarantined DMA, restoring peripheral rates,
or reconciling monotonic time and deadlines around a stopped GTIM1. Existing RTC
calendar retention is not an alarm/elapsed-time bridge. F030 also requires HCLK
at most 4 MHz before DeepSleep, Flash idle and ready comparators.

Consequently this release exposes ordinary CPU sleep through the existing
executor behavior, while retaining fixed clocks after startup. It adds no
always-failing placeholder or apparently safe clock switch that leaves live
drivers stale. [Runtime/low-power evidence](runtime-low-power.md) records the
actual wake matrix, 4/8 MHz fast-wake distinction, hardware prerequisites and
specific firmware contracts needed for a later managed implementation. This is
a known implementation limit, not a claim that those silicon features are absent.

The original comparison is therefore closed as a bounded, validated improvement
set with the limitations above still visible. Physical clock startup, waveform,
DMA timing, fault response and low-power behavior have not been tested on a board.
See [v0.22 verification](validation-v0.22.0.md) for executed software checks.
