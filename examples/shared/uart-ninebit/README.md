# Native nine-bit ordinary UART examples

Select exactly one feature:

| Feature | UART | TXD | RXD |
| --- | --- | --- | --- |
| `cw32l010f8p6` | UART2 | PA3 | PA4 |
| `cw32l011k8t6` | UART3 | PA4 | PA5 |
| `cw32l012c8t6` / `cw32l012c8u6` | UART1 | PA2 | PA3 |

Connect TXD to RXD for loopback. The blocking and async split examples send
`0x000`, `0x100`, `0x1ff`, `0x055` with nine data bits, no parity and one stop bit.
An eight-bit USB-UART adapter is not a suitable peer. Async TX/RX are polled
concurrently; delayed service can overrun the single receive register.

L010/L011 changes have not been compiled or run. Board acceptance remains
pending. See [source and API scope](../../../docs/uart-native-word-half-duplex.md).
