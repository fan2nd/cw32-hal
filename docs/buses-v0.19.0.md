> Historical v0.19 stage. Current follow-on work: [v0.20](analog-rtc-flash-v0.20.0.md).

# UART, SPI, I2C, CRC and independent watchdog

This is the third bounded stage of the Embassy comparison. It adds the three
general controller buses on both chips, CRC, and reset-only non-window IWDT.
The implementation follows actual ownership, type-level IRQ and operation-mode
contracts; hardware-specific register sequences remain separate.

## Coverage

| Driver | L012 | F030 | Explicit boundary |
| --- | --- | --- | --- |
| UART | UART1–3 | UART1–3 | Unbuffered TX/RX, blocking and interrupt-driven async; no safe bus DMA or hardware flow control |
| SPI | SPI1–3, shared SPI23 IRQ | SPI1–2 | Controller full-duplex bus, 8/16-bit words, blocking and interrupt-driven async; external chip-select ownership belongs to a SpiDevice layer |
| I2C | I2C1–2, PCLK explicitly selected | I2C1–2, fixed PCLK | Seven-bit controller transactions, blocking and interrupt-driven async; no target or 10-bit addressing |
| CRC | Eight CRC16 algorithms, byte input | Eight CRC16 plus two CRC32 algorithms; 8/16/32-bit input writes | Hardware presets only, no arbitrary polynomial or seed |
| IWDT | Shared audited IWDT layout | Shared audited IWDT layout | Explicit start/feed/stop, reset-only and no early-feed window; no WWDT or IRQ mode |

The chip data gains 133 L012 and 116 F030 UART/SPI/I2C pin routes with independent
manual evidence. Existing route totals become 259 and 223. Generation uses the
actual peripheral kind, IP version, pin signal and AF value. PA13/PA14 remain
excluded from safe bus traits until an explicit SWD-release API exists. Signal
metadata includes CTS/RTS/CS as physical facts without claiming current driver
support for their hardware modes.

Clock configuration selects a documented internal source and derives its input
frequency from active RCC state. New owners retain clocks until their operation
is stopped and pins are disconnected. Constructors require only capabilities
supported by the selected IP. Interrupt constructors require the physical
vector's `Binding`; a shared vector must list every active source handler.

Async bus methods keep application buffer access in the polling future, rather
than retaining buffer pointers in an interrupt or DMA engine. Source masks,
pending status and event publication use the existing critical-section event
contract. Completion steps without a hardware IRQ use explicitly bounded
polling. Poll limits are work budgets, not elapsed-time deadlines. See each
driver's documentation for cancellation and partial on-wire effects.

## Application integration

Example 06 uses the UART driver for its existing polling UI protocol. Its
original UART2 software interrupt executor, synchronous motor ADC/commutation
handlers, one-byte-per-UI-step scheduling, baud and packet logic remain the
review baseline. The other board examples and motor algorithms are outside
the bus migration. Motor power remains an explicit opt-in for examples 05/06.

## Subsequent bounded stages

The approved remaining work stays visible:

1. Flash and RTC need their own program/erase interruption, reserved regions,
   clock/calendar and backup-domain audit. WWDT needs its one-way enable and
   early/late window reset lifecycle reviewed separately.
2. Safe internal ADC sources must model activation and settling. F030's internal
   sources require BUF=1 and single-channel single-shot operation; a channel
   number alone is insufficient.
3. Timer capture/encoder and generic complementary PWM require real channel,
   input-route, shared-counter and brake contracts. Existing specialized
   ThreePhasePwm remains available.
4. Additional typed peripheral DMA endpoints must retain the CW finite-transfer
   ownership and uncertain-abort quarantine rules. Safe circular DMA is still
   unsupported because hardware drain behavior is not established.
5. External/PLL clock selection, runtime switching and low-power recovery need
   oscillator pin ownership, stable transitions, peripheral kernel choices and
   time-driver continuity. Current checked HSI profiles remain the supported
   clock model.

These stages are distinct from unlimited STM32 feature parity. No new driver
claims board execution, electrical validation or measured interrupt timing.
