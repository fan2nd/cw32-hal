# Native LIN transmit primitives

`LinTransmitter` owns one qualified instance and its package-qualified TX pin.
The sealed `LinInstance` permits L010/L011 UART1 and L012 UART1/2. Both blocking and
interrupt-driven forms require 8N1 configuration, rejected before MMIO otherwise.
Ordinary clock/baud validation remains in force. Construction uses the existing
reset/configuration path (LSB first, SIGNAL/SYNC clear), selects CR3.LIN with TX
briefly disabled before any TDR write, then enables only TX. No receiver, DMA,
flow control, split or live mode conversion is exposed. Other UART instances are not admitted by this API. L012 UART3 remains excluded
pending its separate clock-gate qualification.

`LinBreakBits::new` accepts 13..511 dominant bit periods; default is 13.
`send_break`/`blocking_send_break` wait for TXE, clear only stale TC using the
existing W0C command seed, write BREAK and length together, then wait for TXBUSY
to clear. `send_idle` sends a complete all-ones hardware frame, with BREAK clear.
Byte writes clear both command bits and retain ordinary queue/flush semantics.
IDLE and BREAK are never asserted together. TC only wakes; TXBUSY proves the
UART buffer/shifter is empty. These waits have no timeout or fairness bound.

Cancellation before the command write emits no new command. Cancellation after
it may leave that command running; the next write obeys TXE and preserves queue
order without replay. Flush can settle pending work. Forgetting a future keeps
no caller buffer pointer in the ISR; subsequent exclusive operations use the
existing TX wait machinery. Owner Drop may truncate the signal and disconnects
TX through the existing UART owner. External transceiver bias must be qualified.

These are waveform primitives, not a complete LIN implementation. The caller
supplies the network's baud/tolerance, timing schedule, delimiter policy,
frame definition, transceiver control and bus ownership. Optional
[encoding helpers](uart-lin-encoding.md) prepare PID and payload/checksum bytes.
An explicit full idle frame is not a programmable one-bit delimiter. UART flush
does not prove external transceiver bus release. A separate [receive-only owner](uart-native-lin-rx.md) exposes break/data events.
[LinUart](uart-native-lin-duplex.md) can own both directions with a test responder
example. Automatic response scheduling and baud detection remain absent.

Own-manual evidence, hashes pinned in `sources/evidence-sources.json`:

| Part/manual | Hardware scope | LIN format and break count | TDR commands | TXE/TC/TXBUSY |
| --- | --- | --- | --- | --- |
| L010 CN1.2 | §16.1 p389/PDF390 | §16.3.4–16.3.4.1 p405/PDF406 | §16.8.8 p424/PDF425 | §16.3.3.3 p398/PDF399; §16.8.11 p426–427/PDF427–428 |
| L011 CN1.1 | §16.1 p391/PDF392 | §16.3.4–16.3.4.1 p407/PDF408 | §16.8.8 p426/PDF427 | §16.8.11 p428–429/PDF429–430 |
| L012 CN1.4 | §21.1–21.3.1 p445–447/PDF471–473 (UARTx=1,2,3) | §21.3.4–21.3.4.1 p463/PDF489 | §21.9.8 p485/PDF511 | §21.3.3.3 p455/PDF481; §21.9.11 p487–488/PDF513–514 |

All three own manuals require SIGNAL/SYNC/STOP/PARITY cleared for LIN, describe
minimum 13 dominant bits, and assign TDR[8:0] as the LIN break bit count. The
existing canonical native UART layout supplies the typed command fields.
The ordinary send-control chapter describes two high bits after a break; this
API does not use that prose as a complete LIN timing qualification.

`examples/shared/uart-lin` supports L010F8P6 UART1 PA1 and L011K8T6 UART1 PA2 using
canonical serial AF routes. L012C8T6/C8U6 use UART1 PA2 and UART2 PA7;
UART2 runs the blocking path on a second independently connected transceiver.
The L012 own manual uses UARTx=1,2,3 without the L010/L011 UART1-only note;
this extension still admits only UART1/2, whose existing gate path is reused.
No RX or DMA path is enabled by the transmitter. It emits one break, one full
idle frame, sync/PID and an encoded response, then parks. The test network must assign that publisher and payload layout; this
does not establish the frame timing limits. Local compilation,
tests and board waveform acceptance have not been performed.
