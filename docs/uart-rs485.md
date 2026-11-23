# Native UART RS485 driver enable

`Rs485Uart` supports blocking and IRQ PIO on L010/L011/L012. It owns TX, RX and
one existing typed RTS route used as DE. `DriverEnable` selects polarity and
0..31 sample periods of equal assertion/deassertion delay; out-of-range values
fail before MMIO. Only existing 4x/8x/16x oversampling is selectable. Nine-bit
word methods retain their ordinary PIO checks. No DMA or split API is exposed.

Each family's own manual defines RTS-to-DE selection, DEP polarity, five-bit
DETIME and the unsupported OVER=3 combination:

| Manual | Protocol, printed page | Register, printed page |
| --- | --- | --- |
| L010 CN1.2 | §16.3.3.8 p403 | §16.8.3 p422 |
| L011 CN1.1 | §16.3.3.8 p405 | §16.8.3 p424 |
| L012 CN1.4 | §21.3.3.8 p461 | §21.9.3 p484 |

Pinned identities are in `sources/evidence-sources.json`. PDF pages are printed
+1 for L010/L011 and +26 for L012. DE polarity/delay are programmed before
connecting its AF. Drop selects an inactive GPIO latch before disabling DEM,
then normal UART and pin teardown runs. This does not guarantee glitch-free
handover or a driven inactive level after pin disconnection; use external bias.

Flush observes UART transmission, not the end of the DE hold interval or the
external transceiver's propagation delay. Reads neither flush nor suppress echo.
Cancellation can transmit a prefix; automatic DE service remains active. Drop
can truncate a frame. Application turn-taking, receiver enable, transceiver
timing, termination and electrical limits require board-specific handling.

`examples/shared/uart-half-duplex` includes a one-shot `rs485` transmitter using UART2
PA3/PA4/PB1 on L010F8P6, UART3 PA4/PA5/PB4 on L011K8T6, or UART1 PA2/PA3/PA1
on L012C8T6/C8U6 (TX/RX/DE). L010 PB1 must not be reserved for LSE.
These additions are source-reviewed and release-built; see
[software acceptance](validation-scope.md). Physical DE timing remains unverified.
