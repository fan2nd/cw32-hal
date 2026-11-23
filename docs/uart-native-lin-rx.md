# Native LIN receive events

`LinReceiver` exclusively owns L010/L011 UART1 or L012 UART1/2 and a qualified
RX pin. Blocking, nonwaiting and IRQ-driven event reads use a fixed configured
baud and require 8N1. Construction validates before MMIO, resets the UART, selects
LIN and the 10/11-bit break threshold while RX is disabled, then enables RX only.
L012 UART3 remains excluded pending its clock-gate qualification.

`LinRxEvent` reports `Data(u8)` or `Break`. RXBRK requires one RDR read and joint
RXBRK/FE acknowledgment using the W0C no-op seed. A zero RDR with RXBRK permits
its expected FE; overrun, parity and noise still report errors. A nonzero RDR
reports framing error. FE without RXBRK also reports framing error, including
an early FE before the break detector finishes. Other sampled receive flags are
acknowledged; unrelated TX, idle and timer flags are preserved.

Status, RDR and acknowledgment are not atomic. Service gaps can overwrite data,
merge events or lose their order. `Break` is a hardware observation, not proof
of a valid 13-bit sender break or complete frame. On any error, discard partial
frame state and resynchronize. There is no lossless reception guarantee.

The ISR masks one-shot receive/error/break requests and wakes the reader without
reading RDR or clearing events. Cancellation masks requests without draining data.
A forgotten wait holds no caller-buffer pointer; a later read or owner Drop masks
it. Drop uses the ordinary RX lifecycle and disconnects the pin. Reads have no
timeout. The standalone constructor has no TX. [LinUart](uart-native-lin-duplex.md)
provides paired LIN owners and splitting. No ring, DMA, automatic baud detection, frame parser,
response scheduling or external transceiver control is exposed.

Own-manual evidence, hashes pinned in `sources/evidence-sources.json`:

| Manual | LIN receive / RDR-zero check and joint RXBRK/FE clear | CR3 threshold |
| --- | --- | --- |
| L010 CN1.2 | §16.3.4.2 p406/PDF407 | p422/PDF423 |
| L011 CN1.1 | §16.3.4.2 p408/PDF409 | p424/PDF425 |
| L012 CN1.4 | §21.3.4.2 p464/PDF490 | p484/PDF510 |

`examples/shared/uart-lin/src/bin/receive.rs` uses UART1 PA0 on L010F8P6 and PA3 on
L011K8T6/L012C8T6/L012C8U6, following canonical AF routes. It observes events at
19,200 baud through a board-configured external transceiver and never responds.
It feeds the table-driven assembler described in [LIN encoding](uart-lin-encoding.md),
resetting on every UART error. The example has no response deadline timer.
Source review only: compilation, tests, cancellation and board waveforms remain
unverified under the local execution restriction.
