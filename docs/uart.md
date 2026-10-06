# UART ownership and interrupt I/O

`embassy_cw32::uart` implements asynchronous serial communication on both
CW32L012 and CW32F030. It follows the pinned Embassy USART ownership vocabulary:
`Uart`, `UartTx`, `UartRx`, `new_blocking`, checked-interrupt `new`, `split`,
`blocking_read`/`blocking_write`, and async `read`/`write`/`flush`. CW32 hardware
semantics are implemented independently; STM32 register layouts, DMA behavior,
and clock selection are not assumed.

## Construction and ownership

```rust,ignore
use embassy_cw32::{bind_interrupts, peripherals, uart};

bind_interrupts!(struct Irqs {
    UART1 => uart::InterruptHandler<peripherals::UART1>;
});

let mut config = uart::Config::default();
config.baudrate = 57_600;
let uart = uart::Uart::new(p.UART1, p.PB12, p.PB11, Irqs, config)?;
let (mut tx, mut rx) = uart.split();
```

The illustrated PB12/PB11 pair is an L012 UART1 route. Actual constructor pin
bounds are generated from each chip's audited AF metadata. A token proves the
silicon route; applications must still check package bonding and board wiring.
TX is initialized high and configured as push-pull alternate output. RX is an
alternate input with the requested pull resistor. Pins stay owned by their
halves and return to floating AF0 digital inputs when dropped.

A full driver consumes the UART token and both pins. `split` consumes it and
returns TX and RX halves that may move to separate tasks. `split_ref` borrows the
two halves. Each half retains a counted clock lease, so dropping one cannot gate
the other's clock. The last drop gates the UART after disabling its direction
and disconnecting its pin. A TX-only or RX-only constructor consumes the entire
UART token; creating the other direction later requires splitting a full owner
instead. Frame format and baud are fixed for the owner's lifetime.

Dropping TX disables transmission immediately and may truncate an in-flight
frame. Flush first when physical completion matters. No destructor waits for
an external device or reconfigures a shared interrupt vector.

## Supported frames and clocks

Both IPs support eight payload bits without, with even, or with odd parity;
one, one-and-a-half, or two stop bits; and nine payload bits without hardware
parity. Nine-bit payloads require the `u16` methods. Byte methods reject that
configuration with `Error::DataBitsMismatch` before transferring data. The
`u16` methods transmit the low configured eight or nine bits.

On L012, eight payload bits plus parity require `CHLEN=1` and `PARITYEN=1`.
The frame width includes the parity bit; `CHLEN=0` would clear `PARITYEN`.
On F030, `PARITY` selects the frame width; its custom-parity mode exposes bit 8
to software and implements the nine-payload-bit mode. Nine payload bits plus
hardware parity are rejected on both chips.

The driver selects fixed HAL PCLK using either documented source encoding,
`ClockSource::Pclk` or `PclkAlt`. The source field is in L012 CR1 and F030 CR2.
LSI/LSE selections are not exposed because the driver cannot acquire or prove
those clock dependencies. No generic RCC kernel-frequency implementation is
invented for a UART whose hardware source is configurable.

Baud generation uses 16x sampling and the nearest representable denominator:

`baud = PCLK / (16 * BRRI + BRRF)`

The constructor rejects zero baud and denominators outside 16 through
1,048,575. Frequency quantization still applies; the valid-register-range check
is not a guarantee of a particular peer's baud-error tolerance.

## Blocking and polling

Blocking read methods fill the complete supplied buffer or return the first
receive error. Blocking write methods queue the complete buffer; the last word
may still be transmitting on return. `blocking_flush` checks `TXBUSY`, so it
waits for both TDR and the shift register, including the stop bits, to become
empty. It also returns immediately before any transmission has occurred.

`is_write_ready` tests TXE. `try_write` queues at most one byte and returns false
without writing if TDR is occupied. `try_read` returns `Ok(None)` if no word or
error is present. These methods never wait or enable interrupts.

Reading RDR does not clear RC on either CW32 IP. The driver reads the status and
word, then acknowledges RC separately with ICR's write-zero-to-clear operation.
It clears only the observed receive/error flags, preserving unselected flags
and the audited reserved reset bits. An errored word is discarded and that read
returns an error. For simultaneous errors, L012 reports overrun, noise, framing,
then parity in that order; F030 reports framing before parity. All observed
errors are acknowledged so the next read can recover.

This is an unbuffered driver. The hardware retains one received word, which can
be overwritten when software is late or between calls. L012 reports hardware
overrun and noise. F030 exposes neither flag and can silently overwrite unread
data; `Error::Overrun` and `Error::Noise` are never synthesized for that IP.

## Interrupt-driven async behavior

Async constructors require `Binding<I::Interrupt, InterruptHandler<I>>`.
Their futures register a waker, inspect readiness, enable the needed source and
inspect readiness again under a critical section. Hardware can progress at any
point in this sequence without losing the wake-up. The handler examines only
its own UART's enabled pending sources, masks the signaled source and wakes the
appropriate TX/RX owner. It leaves receive data and error acknowledgement to the
read operation. Shared register RMW operations are serialized between split
halves. A shared-vector handler first checks the UART's clock resource before
accessing its registers.

Each async operation waits on actual UART IRQs. There is no DMA channel argument,
no borrowed-buffer DMA claim, no spin-based async loop and no application-memory
pointer retained by the ISR. The executor still has to resume promptly enough
to consume the hardware's single-word receive register. Applications requiring
continuous high-rate capture need a separately designed buffered or DMA driver.

Cancellation disables only the source armed by that future and removes its
waker. It leaves the UART clock, frame configuration, opposite direction and
NVIC vector intact. A cancelled write can leave a transmitted or queued prefix;
a cancelled flush leaves the transmitter running. A cancelled read leaves its
completed prefix in the caller's buffer and any unconsumed hardware word for
the next read. The inherent full-buffer read API does not return the prefix
length after error or cancellation; protocols must provide resynchronization.
An empty buffer performs no bus transfer.

The driver never disables, unpends, or changes the priority of a shared NVIC
vector. Applications using a shared vector must bind every active source. No
unbuffered full-buffer `embedded_io::Read` implementation is supplied: that
trait's partial-read behavior is different from these inherent methods, as the
pinned Embassy USART source also documents.

## Example 06 compatibility

`examples/l012-bldc/06-application/src/io.rs` now owns a blocking UART1 driver.
It keeps the original 96 MHz PCLK source encoding 1, BRRI=52, BRRF=1, requested
115,200 baud (actual approximately 115,246), 8N1, PB12 AF1 TX, PB11 AF1 RX pull-up,
and both TX/RX enables. UART1 interrupts remain disabled. The task still checks
TXE and pops at most one byte from the unchanged seven-byte frame queue on each
I/O tick. It still does not consume RX data, so no new command or receive-error
path affects the motor application. UART2's P1 software motor executor, tick
publishing, frame replacement rules and motor timing code are unchanged.

## Sources and boundaries

- [Pinned Embassy USART implementation](https://github.com/embassy-rs/embassy/blob/b12a6d9efcd2711037abca1b63a661a9ef726444/embassy-stm32/src/usart/mod.rs):
  ownership shapes, blocking/async method vocabulary, constructor pin ordering,
  split halves, buffer completion and unbuffered I/O trait caveats.
- [CW32L012 reference manual v1.4](https://www.whxy.com/uploads/files/20260603/CW32L012_UserManual_CN_V1.4.pdf),
  §§21.3.3.1–4, 21.5 and 21.9: frame format, baud generator, TXBUSY, explicit RC
  acknowledgement, overrun/noise behavior and register encodings.
- [CW32x030 reference manual Rev 2.5](https://www.whxy.com/uploads/files/20240920/CW32x030_UserManual_CN_V2.5.pdf),
  §§18.3.3.1–4, 18.5 and 18.9: custom ninth bit, clock source placement,
  receive overwrite limitation and write-zero-to-clear flags.

Initial support is asynchronous serial full duplex with polling or UART IRQ
transport. DMA, LIN, hardware flow control, half duplex, synchronous clocking,
low-frequency clock operation and runtime reconfiguration require further
hardware-specific API and ownership work.

The additional [finite typed DMA API](bus-dma.md) consumes complete static owners
and buffers. It has distinct completion and permanent-quarantine rules; these
CPU-driven operations retain their existing borrowed-buffer behavior.
