# UART hardware flow-control firmware

These are standalone Cortex-M0+ applications. They link against the real HAL,
PAC, Cortex-M runtime and pinned Embassy executor. They are not tests or mocks.
They have been built and linked, not flashed or executed on a board.
These historical results cover ordinary unbuffered owners. The separate
[`uart-buffered` applications](../uart-buffered/README.md) exercise the Stage83
extension described in the [buffered receive contract](../../../docs/uart-buffered-rx.md);
its accepted main results and final-source clean requirement are recorded separately.

Choose one exact package feature and one binary:

```sh
cargo build --offline --locked --release --target thumbv6m-none-eabi \
  --manifest-path examples/shared/uart-flow-control/Cargo.toml \
  --no-default-features --features cw32f030c8t7 --bin async_echo
```

- `blocking_echo` / `async_echo`: split a full RTS/CTS UART, receive one byte,
  echo it and wait for its final stop bit.
- `blocking_sender` / `async_sender`: send a repeated identifying line through
  a TX-only UART with CTS. Deasserted CTS prevents a new frame from starting; a frame already in progress finishes.
- `blocking_receiver` / `async_receiver`: receive individual bytes through an
  RX-only UART with RTS; the last byte is available at the `black_box` call for
  debugger inspection. No received data is transmitted.

All use 115200 baud, 8 data bits, no parity, one stop bit, PCLK and default
oversampling. CTS has a weak pull-up, so an unplugged peer pauses the sender.
The L083 firmware uses the complete UART1/UART4 shared handler; UART4 stays
unowned, and must be quiescent if bootloader or other code used it previously.

| Package feature | UART | TX | RX | RTS | CTS |
|---|---|---|---|---|---|
| cw32f002f3p7 | UART1 | PB1 | PB0 | PA7 | PA6 |
| cw32f030c8t7 | UART2 | PA2 | PA3 | PA1 | PA0 |
| cw32l010f8p6 | UART2 | PA3 | PA4 | PB1 | PB0 |
| cw32l012c8t6 | UART1 | PA2 | PA3 | PA1 | PA0 |
| cw32l031c8t6 | UART2 | PA2 | PA3 | PA1 | PA0 |
| cw32l052c8t6 | UART2 | PA2 | PA3 | PA1 | PA0 |
| cw32l083rct6 | UART1 | PA8 | PA9 | PA11 | PA10 |

Use a voltage-compatible peer and common ground. Cross TX to the peer's RX,
RX to TX, RTS to CTS and CTS to RTS. Enable active-low RTS/CTS flow control on
the peer. Sender-only needs TX/CTS; receiver-only needs RX/RTS. The ownership
macro returns unused pads without configuring them. Check the board schematic
for any other device connected to these pads before running.

Hardware RTS reflects the single receive register. It is not a software FIFO or
an application-level packet credit. The peer must obey its documented timing;
extra in-flight frames can still be lost. CTS can pause a write/flush forever.
Stopping a transmitter without flushing can truncate an in-flight frame.
Buffered owners also use that hardware RC signal; RTS is not a watermark for
their software ring.
