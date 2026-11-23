# L012 single-wire UART

This document records the L012 qualification. Current L010/L011 support and
its pending acceptance are described in [native UART scope](uart-native-word-half-duplex.md).


## Qualified source and electrical scope

CW32L012 CN user manual V1.4, SHA-256
`a9e54694a26f03c1f3e3041f40844900168ca2b8e6142f3328671b92e6b7a340`:
printed p447 table 21-1, §21.3.3.5 p457 and §21.7.3 pp477–478 explicitly
specify bidirectional TXD, open-drain alternate function, external pull-up,
SIGNAL=1 and TXEN=RXEN=1. A TDR write begins transmission; hardware returns
the pin to reception after transmission. This is own-family qualification,
not inferred from another MCU's similarly named registers.

Only L012 exposes `HalfDuplexUart<'d, M>`. It consumes the UART and one typed
TXD pin, not RXD. `new_blocking` and typed-IRQ `new` perform ordinary frame,
clock, baud and RX-pull preflight before UART/GPIO changes. The pin is
configured directly as open-drain AF with its latch preloaded high; it never
passes through the ordinary push-pull TX constructor. Internal weak pull-up
is not a substitute for the required external pull-up. No glitch-free
initialization, rise-time or electrical bus-fault claim is made.

## API and ownership

- `write` / `blocking_write`, `read` / `blocking_read`, and `flush` /
  `blocking_flush` provide byte operations.
- Corresponding `write_u16`, `read_u16`, `blocking_write_u16`, and
  `blocking_read_u16` use the existing low-8/9-bit container policy.
- `actual_baudrate` returns divider-rounded baud, not measured accuracy.
- The wrapper owns private ordinary TX/RX lifecycle halves, with exactly one
  GPIO owner in TX. It exposes no split, dereference, DMA, ring-buffer,
  flow-control, live-conversion, idle-packet or embedded-io API.

Every read first flushes queued output, then performs ordinary exact PIO
reception. A nine-bit owner's byte read fails before that flush, including
empty calls. A supported empty read **does flush** and can wait indefinitely;
its receive phase retires an old receive wait without reading a data word.
Writes queue output and do not imply final-stop-bit completion. Flush waits
for TX buffer plus shifter to become empty; it does not arbitrate the bus or
wait for a peer. There is no software direction toggle.

Unpolled futures do nothing. Canceling a read while flushing leaves the
owner available for another read/flush; canceling after receive begins may
leave a filled prefix. Operation-buffer pointers are not stored in the ISR.
Normal Drop uses the existing bounded destructive TX/RX shutdown and may
truncate output. Neither public half can escape while depending on the shared
pin. Forgotten owners may leave hardware enabled; safe-token reconstruction
uses existing reset/lifecycle retirement, not a new reclamation shortcut.

Applications must coordinate turn-taking and peer response timing. The
ordinary one-word RX storage, errors and possible overwrite remain. No
collision detection, arbitration, guaranteed response window, throughput,
lossless reception or hardware validation is claimed.

## Examples and software checks

`examples/shared/uart-half-duplex` supplies blocking and async request/one-byte-response
initiators for C8T6 and C8U6. Wire UART1 PA2 to a compatible half-duplex peer,
add an external pull-up and common ground. RXD PA3 is unused. These examples
are not full-duplex TX-to-RX loopback tests and do not implement the peer.

Software acceptance results belong in the batch validation receipt; compiling
or running host traces does not validate physical direction switching,
pull-up sizing, peer timing, collisions or signal integrity.
